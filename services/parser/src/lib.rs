extern crate pest;
#[macro_use]
extern crate pest_derive;

use log::*;
use otto_models::*;
use pest::error::Error as PestError;
use pest::iterators::{Pair, Pairs};
use pest::Parser;
use uuid::Uuid;

#[derive(Parser)]
#[grammar = "pipeline.pest"]
struct PipelineParser;

/**
 * This function will attempt to fully parse the buffer as if it were a complete
 * pipeline file.
 */
pub fn parse_pipeline_string(buffer: &str) -> Result<Pipeline, PestError<Rule>> {
    let mut parser = PipelineParser::parse(Rule::pipeline, buffer)?;
    let mut pipeline = Pipeline::default();

    while let Some(parsed) = parser.next() {
        match parsed.as_rule() {
            Rule::execBlocks => {
                let mut parsed = parsed.into_inner();
                while let Some(parsed) = parsed.next() {
                    match parsed.as_rule() {
                        Rule::steps => {
                            let mut ctx = Context::default();
                            ctx.steps
                                .extend(parse_steps(&mut parsed.into_inner(), pipeline.uuid));

                            pipeline.batches.push(Batch {
                                mode: BatchMode::Linear,
                                contexts: vec![ctx],
                            });
                        }
                        Rule::stage => {
                            let ctx = parse_stage(&mut parsed.into_inner());
                            pipeline.batches.push(Batch {
                                mode: BatchMode::Linear,
                                contexts: vec![ctx],
                            });
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    Ok(pipeline)
}

fn parse_str(parser: &mut pest::iterators::Pair<Rule>) -> String {
    // TODO: There's got to be a better way than cloning
    let mut parser = parser.clone().into_inner();
    while let Some(parsed) = parser.next() {
        match parsed.as_rule() {
            Rule::STRV => {
                return parsed.as_str().to_string();
            }
            _ => {}
        }
    }
    "".to_string()
}

/**
 * Parse the steps
 *
 * In the case of orphan steps, the uuid should be the pipeline's uuid
 */
fn parse_steps(parser: &mut Pairs<Rule>, uuid: Uuid) -> Vec<Step> {
    let mut steps = vec![];

    while let Some(parsed) = parser.next() {
        if Rule::step == parsed.as_rule() {
            // Grab the step components
            let mut parts: Vec<Pair<Rule>> = parsed.into_inner().collect();
            // We need at least two parts here!
            assert!(parts.len() > 1);

            let symbol = parts[0].as_str().to_string();
            let command = parse_str(&mut parts.pop().unwrap());

            let parameters = serde_json::Value::String(command);
            let parameters = StepParameters::Positional(vec![parameters]);
            let step = Step::new(uuid, symbol, parameters);
            steps.push(step);
        }
    }
    steps
}

fn parse_stage(parser: &mut Pairs<Rule>) -> Context {
    let mut stage = Context::default();

    debug!("stage: {:?}", parser);

    while let Some(parsed) = parser.next() {
        match parsed.as_rule() {
            Rule::property => {
                let mut inner = parsed.into_inner();

                while let Some(parsed) = inner.next() {
                    match parsed.as_rule() {
                        Rule::IDENT => {
                            let key = parsed.as_str().to_string();

                            // This pair should be a STR
                            if let Some(pair) = inner.next() {
                                let value = pair.into_inner().as_str().to_string();
                                debug!("Adding to context key: {}, value: {}", key, value);
                                stage.properties.insert(key, value);
                            }
                        }
                        _ => {}
                    }
                }
            }
            Rule::steps => {
                let mut inner = parsed.into_inner();
                stage.steps.extend(parse_steps(&mut inner, stage.uuid));
            }
            _ => {}
        }
    }
    stage
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_steps() {
        let _steps = PipelineParser::parse(
            Rule::steps,
            r#"steps {
                sh 'ls'
            }
            "#,
        )
        .unwrap()
        .next()
        .unwrap();
    }

    #[test]
    fn parse_steps_positional_args() {
        let _steps = PipelineParser::parse(
            Rule::steps,
            r#"steps {
                sh 'ls', 'utf-8', 'lolwut'
            }
            "#,
        )
        .unwrap()
        .next()
        .unwrap();
    }

    #[test]
    fn parse_steps_keyword_arg() {
        let _steps = PipelineParser::parse(
            Rule::steps,
            r#"steps {
                sh script: 'ls'
            }
            "#,
        )
        .unwrap()
        .next()
        .unwrap();
    }

    #[test]
    fn parse_steps_keyword_args() {
        let _steps = PipelineParser::parse(
            Rule::steps,
            r#"steps {
                sh script: 'ls', label: 'lolwut'
            }
            "#,
        )
        .unwrap()
        .next()
        .unwrap();
    }

    #[test]
    fn it_works() {
        let _pipeline = PipelineParser::parse(
            Rule::pipeline,
            r#"
            pipeline {
                stage {
                    name = 'Build'
                    steps {
                        sh 'ls'
                        sh 'env'
                    }
                }

                stage {
                    name = 'Deploy'
                    steps {
                        sh 'make deploy'
                    }
                }
            }
            "#,
        )
        .unwrap()
        .next()
        .unwrap();
    }

    #[test]
    fn parse_simple_pipeline() {
        let buf = r#"
            pipeline {
                stage {
                    name = 'Build'
                    steps {
                        sh 'ls'
                    }
                }
            }"#;

        let pipeline = parse_pipeline_string(&buf).expect("Failed to parse");
        assert!(!pipeline.uuid.is_nil());
        assert_eq!(pipeline.batches.len(), 1);
        let context = &pipeline.batches[0].contexts[0];
        assert!(context.properties.contains_key("name"));
        assert_eq!(context.steps.len(), 1);
    }

    #[test]
    fn parse_more_pipeline() {
        let buf = r#"
            pipeline {
                stage {
                    name = 'Build'
                    steps {
                        sh 'ls'
                    }
                }
                stage {
                    name = 'Deploy'
                    steps {
                        sh 'ls -lah && touch deploy.lock'
                        sh 'make depoy'
                    }
                }
            }"#;

        let pipeline = parse_pipeline_string(&buf).expect("Failed to parse");
        assert!(!pipeline.uuid.is_nil());
        assert_eq!(pipeline.batches.len(), 2);
    }

    #[test]
    fn parse_orphan_steps() {
        let buf = r#"
            pipeline {
                steps {
                    sh 'make all'
                }
            }"#;
        let pipeline = parse_pipeline_string(&buf).expect("Failed to parse");
        assert!(!pipeline.uuid.is_nil());
        assert_eq!(pipeline.batches.len(), 1);
    }
}
