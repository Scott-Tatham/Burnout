/*!
 * Stores the base configuration.
 */
use super::shells::bash::BashConfiguration;
use super::shells::cmd::CmdConfiguration;
use super::shells::git_bash::GitBashConfiguration;
use super::shells::powershell::PowerShellConfiguration;
use super::shells::zsh::ZshConfiguration;
use super::module::ModuleConfiguration;
use serde::{Deserialize, Serialize};

/**
 * Stores the base configuration.
 */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct BaseConfiguration
{
    pub bash: Option<BashConfiguration>,
    pub zsh: Option<ZshConfiguration>,
    pub git_bash: Option<GitBashConfiguration>,
    pub powershell: Option<PowerShellConfiguration>,
    pub cmd: Option<CmdConfiguration>,
    pub modules: Option<Vec<ModuleConfiguration>>
}

/**
 * Implements the [Default] trait for the prompt.
 */
impl Default for BaseConfiguration
{
    fn default() -> Self
    {
        Self
        {
            bash: Some(BashConfiguration::default()),
            zsh: Some(ZshConfiguration::default()),
            git_bash: Some(GitBashConfiguration::default()),
            powershell: Some(PowerShellConfiguration::default()),
            cmd: Some(CmdConfiguration::default()),
            modules: Some(Vec::default())
        }
    }
}

/**
 * Unit tests for the base configuration.
 */
#[cfg(test)]
mod tests
{
    use super::*;

    /**
     * Tests the default values for the configuration are correct.
     */
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

    /**
     * Tests the deserialisation of an empty configuration as YAML.
     */
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

    /**
     * Tests the serialisation and deserialisation of an empty configuration as YAML.
     */
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