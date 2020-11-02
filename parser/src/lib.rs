extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "pipeline.pest"]
struct Pipeline;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_steps() {
        let steps = Pipeline::parse(Rule::steps,
            r#"steps {
                sh 'ls'
            }
            "#)
            .unwrap().next().unwrap();
    }

    #[test]
    fn parse_steps_positional_args() {
        let steps = Pipeline::parse(Rule::steps,
            r#"steps {
                sh 'ls', 'utf-8', 'lolwut'
            }
            "#)
            .unwrap().next().unwrap();
    }

    #[test]
    fn parse_steps_keyword_arg() {
        let steps = Pipeline::parse(Rule::steps,
            r#"steps {
                sh script: 'ls'
            }
            "#)
            .unwrap().next().unwrap();
    }

    #[test]
    fn parse_steps_keyword_args() {
        let steps = Pipeline::parse(Rule::steps,
            r#"steps {
                sh script: 'ls', label: 'lolwut'
            }
            "#)
            .unwrap().next().unwrap();
    }

    #[test]
    fn it_works() {
        let pipeline = Pipeline::parse(Rule::pipeline,
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
            "#).unwrap().next().unwrap();
    }
}
