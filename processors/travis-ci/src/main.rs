extern crate clap;
extern crate pretty_env_logger;
extern crate serde;
extern crate serde_yaml;
extern crate uuid;

use clap::{App, Arg};
use log::*;
use serde::{Deserialize, Serialize};
use serde_yaml::Value;
use uuid::Uuid;

use std::collections::HashMap;
use std::fs;

use otto_eventbus::client::*;
use otto_eventbus::*;

fn main() {
    pretty_env_logger::init();

    let matches = App::new("travis processor")
        .arg(
            Arg::with_name("filename")
                .short("f")
                .long("filename")
                .value_name("FILE")
                .required(true)
                .help("File")
                .takes_value(true),
        )
        .get_matches();
    let filename = matches.value_of("filename").unwrap_or(".travis-ci.yml");
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let pipeline = serde_yaml::from_str::<TravisConfig>(&contents)
        .expect("Failed to deserialize the yaml file into a TravisConfig");

    let mut output = PipelineManifest { tasks: vec![] };

    let mut caps = HashMap::new();

    if pipeline.sudo {
        caps.insert("docker_run".to_string(), Value::Bool(false));
    } else {
        caps.insert("docker_run".to_string(), Value::Bool(true));
    }

    let mut task = Task {
        id: Uuid::new_v4().to_string(),
        capabilities: caps,
        ops: vec![],
    };

    task.ops.push(Op {
        id: Uuid::new_v4().to_string(),
        op_type: OpType::BeginContext,
        // Cheap quick hack to get a simple hashmap here
        data: serde_yaml::from_str(r#"{ name: "Travis" }"#).unwrap(),
    });

    for script in pipeline.script.iter() {
        let mut data = HashMap::new();
        data.insert("script".to_string(), Value::String(script.to_string()));
        data.insert(
            "timeout_s".to_string(),
            Value::Number(serde_yaml::Number::from(300)),
        );
        data.insert("env".to_string(), Value::Null);
        task.ops.push(Op {
            id: Uuid::new_v4().to_string(),
            op_type: OpType::RunProcess,
            data,
        });
    }

    task.ops.push(Op {
        id: Uuid::new_v4().to_string(),
        op_type: OpType::EndContext,
        // Cheap quick hack to get a simple hashmap here
        data: serde_yaml::from_str(r#"{ name: "Travis" }"#).unwrap(),
    });

    output.tasks.push(task);

    info!(
        "{}",
        serde_yaml::to_string(&output).expect("Failed to serialize manifest")
    );

    /*
    let sys = System::new("name");
    Arbiter::spawn(async {
        let client = connect("http://127.0.0.1:8000/ws/", "processor-travis-ci").await;
        info!("Client created: {:?}", client);

        let input = InputMessage {
            meta: Meta::new("tasks.for_auction".to_string()),
            msg: Input::Publish {
                payload: serde_json::to_value(output).unwrap(),
            },
        };
        client.do_send(input);

        /*
         * Disconnecting as soon as we send Input doesn't work well, because the client seems to
         * terminate before it can actually send messages over.
         */
        //client.do_send(Disconnect {});
        //info!("Disconnected");
        //System::current().stop();
    });
    sys.run().unwrap();
    */
}

#[derive(Deserialize, Debug, Serialize)]
struct TravisConfig {
    sudo: bool,
    script: Vec<String>,
}

#[derive(Deserialize, Debug, PartialEq, Serialize)]
enum OpType {
    #[serde(rename = "BEGINCTX")]
    BeginContext,
    #[serde(rename = "RUNPROC")]
    RunProcess,
    #[serde(rename = "ENDCTX")]
    EndContext,
}

#[derive(Deserialize, Debug, Serialize)]
struct Op {
    id: String,
    #[serde(rename = "type")]
    op_type: OpType,
    data: HashMap<String, Value>,
}

#[derive(Deserialize, Debug, Serialize)]
struct Task {
    id: String,
    capabilities: HashMap<String, Value>,
    ops: Vec<Op>,
}
#[derive(Deserialize, Debug, Serialize)]
struct PipelineManifest {
    // meta: Meta,
    tasks: Vec<Task>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deser_simple() {
        let yaml = r#"
---
sudo: false
script:
  - echo "Hello World"
  - env
"#;
        let c = serde_yaml::from_str::<TravisConfig>(yaml).unwrap();
        assert!(!c.sudo);
        assert_eq!(c.script.len(), 2);
    }

    #[test]
    fn deser_yaml() {
        let yaml = r#"
---
meta:
tasks:
  - id: 0x1
    capabilities:
      docker_run: true
    ops:
      - id: 1
        type: BEGINCTX
        data:
          name: 'Travis'
      - id: 2
        type: RUNPROC
        data:
          script: 'echo "Hello World"'
          env:
          timeout_s: 300
      - id: 3
        type: ENDCTX
        data:
          name: 'Travis'
"#;
        let c = serde_yaml::from_str::<PipelineManifest>(yaml).unwrap();
        assert_eq!(c.tasks.len(), 1);
        assert_eq!(c.tasks[0].ops.len(), 3);
        assert_eq!(c.tasks[0].ops[0].op_type, OpType::BeginContext);
    }
}
