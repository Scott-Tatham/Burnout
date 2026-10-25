/*!
 * Handles the shell configuration file of Burnout.
 */
use std::fs;
use std::path::PathBuf;
use directories::ProjectDirs;
use super::base_configuration::BaseConfiguration;

/// Gets the path to the shell configuration file.
/// # Returns
/// * `PathBuf` - Returns the path to the shell configuration file.
fn configuration_path() -> PathBuf
{
    if let Some(project_directories) = ProjectDirs::from("dev", "chicken-lips", "burnout")
    {
        project_directories.config_dir().join("shell_configuration.yaml")
    }

    else
    {
        dirs::home_dir().expect("Failed to get the home directory.").join("shell_configuration.yaml")
    }
}

/// Loads or creates the [BaseConfiguration] from the shell configuration file.
/// # Returns
/// * `BaseConfiguration` - Returns the loaded or created configuration.
pub fn load_or_create_configuration() -> BaseConfiguration
{
    let configuration_path = configuration_path();
    let read_result = fs::read_to_string(&configuration_path);

    match read_result
    {
        Ok(content) => yaml_serde::from_str(&content).expect("Failed to parse shell configuration file."),
        Err(_) =>
            {
                let base_configuration = BaseConfiguration::default();
                let yaml_value = yaml_serde::to_string(&base_configuration).expect("Failed to serialise shell configuration.");

                if let Some(parent) = configuration_path.parent()
                {
                    fs::create_dir_all(parent).expect("Failed to create the configuration directory.");
                }

                fs::write(&configuration_path, yaml_value).expect("Failed to write the default shell configuration file.");

                base_configuration
            }
    }
}

/// Tests for the shell configuration file.
#[cfg(test)]
mod tests
{
    use super::configuration_path;

    /// Tests the path of the configuration file is not empty.
    #[test]
    fn test_configuration_path_is_not_empty()
    {
        assert!(!configuration_path().as_os_str().is_empty());
    }
}