use serde::{Deserialize, Serialize};
use std::result::Result;
use std::fmt;
use std::error;

#[derive(Debug, Clone)]
struct InvalidSourceError;

impl fmt::Display for InvalidSourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid Mod Source")
    }
}

impl error::Error for InvalidSourceError {}

#[derive(Serialize, Deserialize)]
struct Mod {
    id: String,
    source: String
}

#[derive(Serialize, Deserialize)]
struct Config {
    mod_loader: String,
    version: String,
    mods: Vec<Mod>
}

#[derive(Debug, Deserialize)]
struct ModFile {
    url: String,
    filename: String,
    primary: bool
}

impl fmt::Display for ModFile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ {}, {}, {} }}", self.url, self.filename, self.primary)
    }
}

#[derive(Debug, Deserialize)]
struct ModVersion {
    id: String,
    name: String,
    version_number: String,
    game_versions: Vec<String>,
    files: Vec<ModFile>,
}

//https://cdn.modrinth.com/data/P7dR8mSH/versions/3gT0I5vt/fabric-api-0.156.0%2B26.2.jar?mr_download_reason=standalone&mr_game_version=26.2&mr_loader=fabric
async fn pull_mod(mod_: &Mod, target_version: &str) -> Result<Option<ModVersion>, Box<dyn error::Error>> {
    if mod_.source != "modrinth" {
        return Result::Err(InvalidSourceError.into());
    }
    let resp: Vec<ModVersion> = reqwest::get(format!("https://api.modrinth.com/v2/project/{}/version", mod_.id))
        .await?
        .json()
        .await?;
    let matching_version = resp.into_iter().find(|v| {
        v.game_versions.iter().any(|ver| ver == target_version)
    });
    Result::Ok(matching_version)
}

fn load_yaml(file_path: &str) -> Result<Config, String> {
    let content = std::fs::read_to_string(file_path)
        .map_err(|e| format!("Failed to read YAML: {}", e))?;

    let config: Config = yaml_serde::from_str(&content)
        .map_err(|e| format!("Invalid YAML format: {}", e))?;

    Ok(config)
}

#[tokio::main]
async fn main() {
    println!("YAMM Test");
    let mut mods: Vec<Mod> = vec![];
    mods.push(Mod{
        id: String::from("fabric-api"),
        source: String::from("modrinth")
    });

    let fabric_api_mod= match pull_mod(&mods[0], "26.2").await {
            Ok(res) => match res {
                Some(mod_version) => println!("retrieved mod {} with files {}", mod_version.name, mod_version.files[0]),
                _ => eprintln!("no such mod found")
            },
            Err(e) => eprintln!("failed to pull fabric-api: {e}")
        };
    
    mods.push(Mod{
        id: String::from("sodium"),
        source: String::from("modrinth")
    });
    let config: Config = Config {
        mod_loader: String::from("fabric"),
        version: String::from("26.2"),
        mods: mods
    };

    let config_str = yaml_serde::to_string(&config);
    match config_str {
        Result::Ok(s) => println!("{}", s),
        Result::Err(_) => println!("failed")
    };

    let new_config: Config = match load_yaml("test.yaml") {
        Ok(c) => c,
        Err(_) => Config {..config}
    };

    let config_str = yaml_serde::to_string(&new_config);
    match config_str {
        Result::Ok(s) => println!("{}", s),
        Result::Err(_) => println!("failed")
    };

}
