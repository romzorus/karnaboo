use config::{self, Config, File, FileFormat};
use serde::Deserialize;




// This function takes an os (md5 hash taken from repo-sources.yml)
// and a role and returns the appropriate script as a String
// to enforce this role to this os
pub fn get_script_from_source_file(role: &str, os: &str) -> Result<Script, String> {
    // Opening the script bank
    let config_builder = Config::builder()
        .add_source(File::new("../script_bank.yml", FileFormat::Yaml))
        .build()
        .unwrap();
    let script_bank = config_builder.try_deserialize::<ScriptBank>().unwrap();

    for script in script_bank.list.into_iter() {
        if (script.role == role) && script.compatible_with.contains(&os.to_string()) {
            return Ok(script);
        }
    }

    Err(String::from("No compatible script found !"))
}

#[derive(Deserialize, Debug)]
pub struct ScriptBank {
    pub list: Vec<Script>,
}

#[derive(Deserialize, Debug)]
pub struct Script {
    pub _key: String,
    pub role: String,
    pub content: String,
    pub compatible_with: Vec<String>,
}

