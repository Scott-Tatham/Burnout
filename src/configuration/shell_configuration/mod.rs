/*!
 * Handles the shell configuration of Burnout.
 */

pub mod base_configuration;
pub mod bash;
pub mod cmd;
pub mod git_bash;
pub mod powershell;
pub mod zsh;

use directories::ProjectDirs;
use std::{fs, path};

/**
 * Loads or creates the [BaseConfiguration] from the shell configuration file.
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
                    fs::create_dir_all(parent).expect("Failed to create the shell configuration directory.");
                }

                fs::write(&configuration_path, toml_value).expect("Failed to write the default shell configuration file.");

                base_configuration
            }
    }
}

/**
 * Returns the path to the shell configuration file.
 */
fn configuration_path() -> path::PathBuf
{
    if let Some(project_directories) = ProjectDirs::from("dev", "chicken-lips", "burnout")
    {
        project_directories.config_dir().join("shell_configuration.toml")
    }

    else
    {
        dirs::home_dir().unwrap().join("shell_configuration.toml")
    }
}

/**
 * Unit tests for the shell configuration module.
 */
#[cfg(test)]
mod tests
{
    use super::*;

    /**
     * Tests the path of the shell configuration file is not empty.
     */
    #[test]
    fn test_configuration_path_is_not_empty()
    {
        let path = configuration_path();
        assert!(!path.as_os_str().is_empty());
    }
}