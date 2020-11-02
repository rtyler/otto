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
