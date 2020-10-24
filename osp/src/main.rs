use osp::Manifest;
use std::fs::File;
use std::path::Path;

fn main() -> std::io::Result<()> {
    // TODO use gumdrop for real argument parsing
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        panic!("osp can only accept a single argument: the directory containing a manifest.yml");
    }

    let dir = Path::new(&args[1]);
    if !dir.is_dir() {
        panic!("The argument must be a directory");
    }
    let manifest = dir.join(Path::new("manifest.yml"));
    let manifest = serde_yaml::from_reader::<File, Manifest>(File::open(manifest)?)
        .expect("Failed to parse manifest.yml");

    let step_name = dir
        .file_name()
        .expect("Failed to unwrap the directory filename")
        .to_str()
        .unwrap();
    println!("default out: {:#?}", step_name);

    manifest.create_artifact(&dir, Path::new(&format!("{}.tar.gz", step_name)))?;
    Ok(())
}
