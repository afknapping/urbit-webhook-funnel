use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use urbit_http_api::ShipInterface;
use yaml_rust::{Yaml, YamlLoader};

static BAREBONES_FUNNEL_CONFIG_YAML: &str = r#"
# The @p of the Ship which is hosting the chat
chat_ship: "~zod"
# Name of the chat
chat_name: "..."
"#;

/// Attempts to create a new `funnel_config.yaml` with the barebones yaml inside.
/// Returns `None` if file already exists.
pub fn create_new_funnel_config_file() -> Option<()> {
    let file_path = Path::new("funnel_config.yaml");
    if file_path.exists() == false {
        let mut file = File::create(file_path).ok()?;
        file.write_all(&BAREBONES_FUNNEL_CONFIG_YAML.to_string().into_bytes())
            .ok()?;
        return Some(());
    }
    None
}

/// Based on the provided input config yaml, build the chat path `String`
fn chat_path_from_yaml(config: Yaml) -> Option<String> {
    let ship = config["chat_ship"].as_str()?;
    let name = config["chat_name"].as_str()?;
    Some(format!("{}/{}", ship, name))
}

/// Opens a local `ship_config.yaml` file and uses the
/// data inside to build the chat path `String`
pub fn chat_path_from_local_config() -> Option<String> {
    let yaml_str = std::fs::read_to_string("ship_config.yaml").ok()?;
    let yaml = YamlLoader::load_from_str(&yaml_str).unwrap()[0].clone();
    chat_path_from_yaml(yaml)
}