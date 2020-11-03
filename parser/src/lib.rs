extern crate pest;
#[macro_use]
extern crate pest_derive;

use log::*;
use otto_models::*;
use pest::iterators::Pairs;
use pest::Parser;

#[derive(Parser)]
#[grammar = "pipeline.pest"]
struct PipelineParser;

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

fn parse_stage(parser: &mut Pairs<Rule>) -> (Context, Vec<Step>) {
    use pest::iterators::Pair;

    let stage = Context::new("Fake".to_string());
    let mut steps: Vec<Step> = vec![];

    debug!("stage: {:?}", parser);

    while let Some(parsed) = parser.next() {
        match parsed.as_rule() {
            Rule::steps => {
                let mut inner = parsed.into_inner();

                while let Some(parsed) = inner.next() {
                    if Rule::step == parsed.as_rule() {
                        // Grab the step components
                        let mut parts: Vec<Pair<Rule>> = parsed.into_inner().collect();
                        // We need at least two parts here!
                        assert!(parts.len() > 1);

                        let symbol = parts[0].as_str().to_string();
                        let command = parse_str(&mut parts.pop().unwrap());

                        let parameters = serde_yaml::Value::String(command);
                        let parameters = StepParameters::Positional(vec![parameters]);
                        let step = Step::new(stage.uuid, symbol, parameters);
                        steps.push(step);
                    }
                }
            }
            _ => {}
        }
    }
    (stage, steps)
}

fn parse_pipeline_string(buffer: &str) -> Result<Pipeline, pest::error::Error<Rule>> {
    let mut parser = PipelineParser::parse(Rule::pipeline, buffer)?;
    let mut pipeline = Pipeline::default();

    while let Some(parsed) = parser.next() {
        match parsed.as_rule() {
            Rule::stages => {
                let mut stages = parsed.into_inner();
                while let Some(parsed) = stages.next() {
                    match parsed.as_rule() {
                        Rule::stage => {
                            let (ctx, mut steps) = parse_stage(&mut parsed.into_inner());
                            pipeline.contexts.push(ctx);
                            pipeline.steps.append(&mut steps);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_steps() {
        let steps = PipelineParser::parse(
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
        let steps = PipelineParser::parse(
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
        let steps = PipelineParser::parse(
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
        let steps = PipelineParser::parse(
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
        let pipeline = PipelineParser::parse(
            Rule::pipeline,
            r#"
            pipeline {
                stages {
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
                stages {
                    stage {
                        name = 'Build'
                        steps {
                            sh 'ls'
                        }
                    }
                }
            }"#;

        let pipeline = parse_pipeline_string(&buf).expect("Failed to parse");
        assert!(!pipeline.uuid.is_nil());
        assert_eq!(pipeline.contexts.len(), 1);
        assert_eq!(pipeline.steps.len(), 1);
    }

    #[test]
    fn parse_more_pipeline() {
        use otto_models::*;
        let buf = r#"
            pipeline {
                stages {
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
                }
            }"#;

        let pipeline = parse_pipeline_string(&buf).expect("Failed to parse");
        assert!(!pipeline.uuid.is_nil());
        assert_eq!(pipeline.contexts.len(), 2);
        assert_eq!(pipeline.steps.len(), 3);
    }
}
