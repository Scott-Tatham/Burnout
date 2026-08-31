/*!
 * Handles configuration of Burnout.
 */

pub mod base_configuration;
pub mod prompt;
pub mod transient;
pub mod right;
pub mod right_transient;
pub mod window_title;

use std::{fs, path};
use directories::ProjectDirs;

/**
 * Loads or creates the [BaseConfiguration] from the configuration file.
 */
pub fn load_or_create_configuration() -> base_configuration::BaseConfiguration
{
    let configuration_path = configuration_path();

    match fs::read_to_string(&configuration_path)
    {
        Ok(content) => toml::from_str(&content).expect("Failed to parse Burnout configuration file."),
        Err(_) =>
            {
                let base_configuration = base_configuration::BaseConfiguration::default();
                let toml_value = toml::to_string(&base_configuration).unwrap();

                if let Some(parent) = configuration_path.parent()
                {
                    fs::create_dir_all(parent).expect("Failed to create the configuration directory.");
                }
                
                fs::write(&configuration_path, toml_value).expect("Failed to write the default configuration file.");

                base_configuration
            }
    }
}

/**
 * Returns the path to the configuration file.
 */
fn configuration_path() -> path::PathBuf
{
    if let Some(project_directories) = ProjectDirs::from("dev", "chicken-lips", "burnout")
    {
        project_directories.config_dir().join("configuration.toml")
    }

    else
    {
        dirs::home_dir().unwrap().join(".burnout.toml")
    }
}
