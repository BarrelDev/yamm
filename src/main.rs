use serde::{Deserialize, Serialize};

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

fn main() {
    println!("YAMM Test");
    let mut mods: Vec<Mod> = vec![];
    mods.push(Mod{
        id: String::from("fabric-api"),
        source: String::from("modrinth")
    });
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
