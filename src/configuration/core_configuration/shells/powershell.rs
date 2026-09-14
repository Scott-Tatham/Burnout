/*!
 * Stores the PowerShell configuration.
 */
use super::super::displays::continuation::ContinuationConfiguration;
use super::super::displays::prompt::PromptConfiguration;
use super::super::displays::right::RightConfiguration;
use super::super::displays::right_transient::RightTransientConfiguration;
use super::super::displays::transient::TransientConfiguration;
use super::super::displays::window_title::WindowTitleConfiguration;
use super::super::module::ModuleConfiguration;
use serde::{Deserialize, Serialize};

/**
 * Stores the PowerShell configuration.
 */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PowerShellConfiguration
{
    pub prompt: Option<PromptConfiguration>,
    pub right: Option<RightConfiguration>,
    pub transient: Option<TransientConfiguration>,
    pub right_transient: Option<RightTransientConfiguration>,
    pub continuation: Option<ContinuationConfiguration>,
    pub window_title: Option<WindowTitleConfiguration>,
    pub modules: Option<Vec<ModuleConfiguration>>,
}

/**
 * Implements the [Default] trait for the PowerShell configuration.
 */
impl Default for PowerShellConfiguration
{
    fn default() -> Self
    {
        Self
        {
            prompt: Some(PromptConfiguration::default()),
            right: Some(RightConfiguration::default()),
            transient: Some(TransientConfiguration::default()),
            right_transient: Some(RightTransientConfiguration::default()),
            continuation: Some(ContinuationConfiguration::default()),
            window_title: Some(WindowTitleConfiguration::default()),
            modules: Some(Vec::default())
        }
    }
}

/**
 * Unit tests for the PowerShell configuration.
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
        let configuration = PowerShellConfiguration::default();

        assert!(configuration.prompt.is_some());
        assert!(configuration.right.is_some());
        assert!(configuration.transient.is_some());
        assert!(configuration.right_transient.is_some());
        assert!(configuration.continuation.is_some());
        assert!(configuration.window_title.is_some());
        assert!(configuration.modules.is_some());
    }

    /**
     * Tests the deserialisation of an empty configuration as YAML.
     */
    #[test]
    fn test_yaml_deserialisation_empty_configuration()
    {
        let configuration: PowerShellConfiguration = yaml_serde::from_str(String::default().as_str()).expect("Failed to deserialise the empty configuration.");

        assert!(configuration.prompt.is_none());
        assert!(configuration.right.is_none());
        assert!(configuration.transient.is_none());
        assert!(configuration.right_transient.is_none());
        assert!(configuration.continuation.is_none());
        assert!(configuration.window_title.is_none());
        assert!(configuration.modules.is_none());
    }

    /**
     * Tests the deserialisation of a partial configuration as YAML.
     */
    #[test]
    fn test_yaml_deserialisation_partial_configuration()
    {
        let configuration: PowerShellConfiguration = yaml_serde::from_str(r#"prompt:
  content: Test Prompt"#).expect("Failed to deserialise the partial configuration.");

        assert_eq!(configuration.prompt.unwrap().content, Some("Test Prompt".to_string()));
        assert!(configuration.right.is_none());
        assert!(configuration.transient.is_none());
        assert!(configuration.right_transient.is_none());
        assert!(configuration.continuation.is_none());
        assert!(configuration.window_title.is_none());
        assert!(configuration.modules.is_none());
    }

    /**
     * Tests the serialisation and deserialisation of an empty configuration as YAML.
     */
    #[test]
    fn test_yaml_serialisation_and_deserialisation_empty_configuration()
    {
        let configuration = PowerShellConfiguration
        {
            prompt: None,
            right: None,
            transient: None,
            right_transient: None,
            continuation: None,
            window_title: None,
            modules: None
        };

        let deserialised: PowerShellConfiguration = yaml_serde::from_str(&yaml_serde::to_string(&configuration).expect("Failed to serialise the empty configuration.")).expect("Failed to deserialise the empty configuration.");

        assert!(deserialised.prompt.is_none());
        assert!(deserialised.right.is_none());
        assert!(deserialised.transient.is_none());
        assert!(deserialised.right_transient.is_none());
        assert!(deserialised.continuation.is_none());
        assert!(deserialised.window_title.is_none());
        assert!(deserialised.modules.is_none());
    }

    /**
     * Tests the serialisation and deserialisation of a partial configuration as YAML.
     */
    #[test]
    fn test_yaml_serialisation_and_deserialisation_partial_configuration()
    {
        let configuration = PowerShellConfiguration
        {
            prompt: Some(PromptConfiguration
            {
                content: Some("Test Prompt".to_string()),
                modules: None
            }),
            right: None,
            transient: None,
            right_transient: None,
            continuation: None,
            window_title: None,
            modules: None
        };

        let deserialised: PowerShellConfiguration = yaml_serde::from_str(&yaml_serde::to_string(&configuration).expect("Failed to serialise the empty configuration.")).expect("Failed to deserialise the empty configuration.");

        assert_eq!(configuration.prompt.unwrap().content, deserialised.prompt.unwrap().content);
        assert!(deserialised.right.is_none());
        assert!(deserialised.transient.is_none());
        assert!(deserialised.right_transient.is_none());
        assert!(deserialised.continuation.is_none());
        assert!(deserialised.window_title.is_none());
        assert!(deserialised.modules.is_none());
    }

    /**
     * Tests the serialisation and deserialisation of a default configuration as YAML.
     */
    #[test]
    fn test_yaml_serialisation_and_deserialisation_default_configuration()
    {
        let configuration = PowerShellConfiguration::default();
        let deserialised: PowerShellConfiguration = yaml_serde::from_str(&yaml_serde::to_string(&configuration).expect("Failed to serialise the base configuration.")).expect("Failed to deserialise the base configuration.");

        assert_eq!(configuration.prompt.unwrap().content, deserialised.prompt.unwrap().content);
        assert_eq!(configuration.right.unwrap().content, deserialised.right.unwrap().content);
        assert_eq!(configuration.transient.unwrap().content, deserialised.transient.unwrap().content);
        assert_eq!(configuration.right_transient.unwrap().content, deserialised.right_transient.unwrap().content);
        assert_eq!(configuration.continuation.unwrap().content, deserialised.continuation.unwrap().content);
        assert_eq!(configuration.window_title.unwrap().content, deserialised.window_title.unwrap().content);
        assert_eq!(configuration.modules.unwrap(), deserialised.modules.unwrap());
    }
}