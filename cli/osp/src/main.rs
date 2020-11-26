use otto_models::osp::Manifest;
use std::fs::File;
use std::path::Path;

/**
 * Create an artifact from the given manifest
 */
fn create_artifact(manifest: &Manifest, dir: &Path, output: &Path) -> Result<(), std::io::Error> {
    use flate2::write::GzEncoder;
    use flate2::Compression;

    let tar_gz = File::create(output)?;
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = tar::Builder::new(enc);
    let mut file = File::open(dir.join(Path::new("manifest.yml")))?;
    tar.append_file(format!("{}/manifest.yml", manifest.symbol), &mut file)?;

    for include in manifest.includes.iter() {
        let mut f = File::open(match include.name.starts_with("./") {
            true => {
                // Relative to dir
                dir.join(&include.name)
            }
            false => {
                // Relative to $PWD
                Path::new(&include.name).to_path_buf()
            }
        })?;

        let archive_path = format!(
            "{}/{}",
            manifest.symbol,
            match include.flatten {
                true => {
                    let p = Path::new(&include.name);
                    p.file_name().unwrap().to_str().unwrap()
                }
                false => &include.name,
            }
        );
        tar.append_file(&archive_path, &mut f)
            .expect(&format!("Failed to append file: {}", &archive_path));
    }
    Ok(())
}

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

    create_artifact(&manifest, &dir, Path::new(&format!("{}.tar.gz", step_name)))?;
    Ok(())
}
