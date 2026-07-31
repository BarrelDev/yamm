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

async fn pull_mod(mod_: &Mod) -> Result<(), Box<dyn error::Error>> {
    if mod_.source != "modrinth" {
        return Result::Err(InvalidSourceError.into());
    }
    let resp = reqwest::get(format!("https://staging-api.modrinth.com/v2/project/{}", mod_.id))
        .await?
        .json::<serde_json::Value>()
        .await?;
    println!("{resp:#?}");
    Result::Ok(())
}

#[tokio::main]
async fn main() {
    println!("YAMM Test");
    let mut mods: Vec<Mod> = vec![];
    mods.push(Mod{
        id: String::from("fabric-api"),
        source: String::from("modrinth")
    });

    if let Err(e) = pull_mod(&mods[0]).await {
        eprintln!("failed to pull fabric-api: {e}");
    }
    
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
}
