/*!
 * Stores the base configuration.
 */
use serde::{Deserialize, Serialize};
use super::shells::shell_configuration::ShellConfiguration;
use super::module::ModuleConfiguration;

/// Stores the base configuration.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct BaseConfiguration
{
    /// The optional [ShellConfiguration] instance for Bash.
    pub bash: Option<ShellConfiguration>,

    /// The optional [ShellConfiguration] instance for Zsh.
    pub zsh: Option<ShellConfiguration>,

    /// The optional [ShellConfiguration] instance for Git Bash.
    pub git_bash: Option<ShellConfiguration>,

    /// The optional [ShellConfiguration] instance for PowerShell.
    pub powershell: Option<ShellConfiguration>,

    /// The optional [ShellConfiguration] instance for Cmd.
    pub cmd: Option<ShellConfiguration>,

    /// The optional collection of [ModuleConfiguration] instances.
    pub modules: Option<Vec<ModuleConfiguration>>
}

/// The implementation of the [Default] trait for the [BaseConfiguration].
impl Default for BaseConfiguration
{
    /// Initialises a new, default instance of the [BaseConfiguration] struct.
    /// # Returns
    /// `BaseConfiguration` - Returns the new, default instance of the [BaseConfiguration] struct.
    fn default() -> Self
    {
        Self
        {
            bash: Some(ShellConfiguration::default()),
            zsh: Some(ShellConfiguration::default()),
            git_bash: Some(ShellConfiguration::default()),
            powershell: Some(ShellConfiguration::default()),
            cmd: Some(ShellConfiguration::default()),
            modules: Some(Vec::default())
        }
    }
}

/// Tests for the [BaseConfiguration].
#[cfg(test)]
mod tests
{
    use super::BaseConfiguration;

    /// Tests the default values for the configuration are correct.
    #[test]
    fn test_default_values_are_correct()
    {
        let configuration = BaseConfiguration::default();

        assert!(configuration.bash.is_some());
        assert!(configuration.zsh.is_some());
        assert!(configuration.git_bash.is_some());
        assert!(configuration.powershell.is_some());
        assert!(configuration.cmd.is_some());
        assert!(configuration.modules.is_some());
    }

    /// Tests the deserialisation of an empty configuration as YAML.
    #[test]
    fn test_yaml_deserialisation_empty_configuration()
    {
        let configuration: BaseConfiguration = yaml_serde::from_str(String::default().as_str()).expect("Failed to deserialise the empty configuration.");

        assert!(configuration.bash.is_none());
        assert!(configuration.zsh.is_none());
        assert!(configuration.git_bash.is_none());
        assert!(configuration.powershell.is_none());
        assert!(configuration.cmd.is_none());
        assert!(configuration.modules.is_none());
    }

    /// Tests the serialisation and deserialisation of an empty configuration as YAML.
    #[test]
    fn test_yaml_serialisation_and_deserialisation_empty_configuration()
    {
        let configuration = BaseConfiguration
        {
            bash: None,
            zsh: None,
            git_bash: None,
            powershell: None,
            cmd: None,
            modules: None
        };

        let deserialised: BaseConfiguration = yaml_serde::from_str(&yaml_serde::to_string(&configuration).expect("Failed to serialise the empty configuration.")).expect("Failed to deserialise the empty configuration.");

        assert!(deserialised.bash.is_none());
        assert!(deserialised.zsh.is_none());
        assert!(deserialised.git_bash.is_none());
        assert!(deserialised.powershell.is_none());
        assert!(deserialised.cmd.is_none());
        assert!(deserialised.modules.is_none());
    }
}