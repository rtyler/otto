/*
 * This test module will test everything in test_data/valid
 */
use otto_parser::*;
use std::fs::ReadDir;
use std::path::PathBuf;

fn parse_file(path: &PathBuf) -> Result<otto_models::Pipeline, pest::error::Error<Rule>> {
    use std::fs::File;
    use std::io::Read;

    let mut file = File::open(path).expect(&format!("Failed to open {:?}", path));
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read file into string");

    parse_pipeline_string(&contents)
}

fn test_in_dir(dir: &mut ReadDir, can_parse: bool) {
    for entry in dir {
        if let Ok(entry) = entry {
            let path = entry.path();
            match path.as_path().extension() {
                Some(ext) => {
                    if ext == "otto" {
                        let result = parse_file(&path);

                        assert_eq!(can_parse, result.is_ok());
                    }
                }
                _ => {}
            }
        }
    }
}

#[test]
fn test_valid_pipelines() {
    let mut dir = std::fs::read_dir("./test_data/valid").expect("Failed to read directory");
    test_in_dir(&mut dir, true);
}

#[test]
fn test_invalid_pipelines() {
    let mut dir = std::fs::read_dir("./test_data/invalid").expect("Failed to read directory");
    test_in_dir(&mut dir, false);
}
