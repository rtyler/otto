use flate2::write::GzEncoder;
use flate2::Compression;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Manifest {
    pub symbol: String,
    pub description: String,
    pub includes: Vec<Include>,
    pub entrypoint: Entrypoint,
    pub parameters: Vec<Parameter>,
}

impl Manifest {
    /**
     * Create an artifact from the given manifest
     */
    pub fn create_artifact(&self, dir: &Path, output: &Path) -> Result<(), std::io::Error> {
        let tar_gz = File::create(output)?;
        let enc = GzEncoder::new(tar_gz, Compression::default());
        let mut tar = tar::Builder::new(enc);
        let mut manifest = File::open(dir.join(Path::new("manifest.yml")))?;
        tar.append_file(format!("{}/manifest.yml", self.symbol), &mut manifest)?;

        for include in self.includes.iter() {
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
                self.symbol,
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
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Include {
    name: String,
    #[serde(default = "default_false")]
    flatten: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Entrypoint {
    pub path: PathBuf,
    #[serde(default = "default_false")]
    multiarch: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Parameter {
    name: String,
    required: bool,
    #[serde(rename = "type")]
    p_type: ParameterType,
    description: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ParameterType {
    #[serde(rename = "string")]
    StringParameter,
    #[serde(rename = "boolean")]
    BoolParameter,
    #[serde(rename = "block")]
    BlockParameter,
}

/** Simple function for serde defaults */
fn default_false() -> bool {
    false
}

#[cfg(test)]
mod tests {}
