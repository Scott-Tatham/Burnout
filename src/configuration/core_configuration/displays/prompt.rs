/*!
 * Stores the prompt configuration.
 */
use super::super::module::ModuleConfiguration;
use serde::{Serialize, Deserialize};

/**
 * Stores the prompt configuration.
 */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PromptConfiguration
{
    pub content: Option<String>,
    pub modules: Option<Vec<ModuleConfiguration>>
}

/**
 * Implements the [Default] trait for the prompt configuration.
 */
impl Default for PromptConfiguration
{
    fn default() -> Self
    {
        Self
        {
            content: Some("Prompt".to_string()),
            modules: Some(Vec::default())
        }
    }
}

/**
 * Unit tests for the prompt configuration.
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
        assert_eq!(PromptConfiguration::default().content, Some("Prompt".to_string()));
        assert!(PromptConfiguration::default().modules.is_some());
    }

    /**
     * Tests the deserialisation of an empty configuration as YAML.
     */
    #[test]
    fn test_yaml_deserialisation_empty_configuration()
    {
        let configuration: PromptConfiguration = yaml_serde::from_str(String::default().as_str()).expect("Failed to deserialise the empty configuration.");

        assert!(configuration.content.is_none());
        assert!(configuration.modules.is_none());
    }

    /**
     * Tests the deserialisation of a partial configuration as YAML.
     */
    #[test]
    fn test_yaml_deserialisation_partial_configuration()
    {
        let configuration: PromptConfiguration = yaml_serde::from_str(r#"prompt:
  content: Prompt"#).expect("Failed to parse valid YAML.");

        assert_eq!(configuration.content, Some("Prompt".to_string()));
        assert!(configuration.modules.is_none());
    }

    /**
     * Tests the serialisation and deserialisation of an empty configuration as YAML.
     */
    #[test]
    fn test_yaml_serialisation_and_deserialisation_empty_configuration()
    {
        let configuration = PromptConfiguration
        {
            content: None,
            modules: None
        };

        let deserialised: PromptConfiguration = yaml_serde::from_str(&yaml_serde::to_string(&configuration).expect("Failed to serialise the empty configuration.")).expect("Failed to deserialise the empty configuration.");

        assert!(deserialised.content.is_none());
        assert!(deserialised.modules.is_none());
    }

    /**
     * Tests the serialisation and deserialisation of a partial configuration as YAML.
     */
    #[test]
    fn test_yaml_serialisation_and_deserialisation_partial_configuration()
    {
        let configuration = PromptConfiguration
        {
            content: Some("Test Prompt".to_string()),
            modules: None
        };

        let deserialised: PromptConfiguration = yaml_serde::from_str(&yaml_serde::to_string(&configuration).expect("Failed to serialise the partial configuration.")).expect("Failed to deserialise the partial configuration.");

        assert_eq!(configuration.content, deserialised.content);
        assert!(deserialised.modules.is_none());
    }

    /**
     * Tests the serialisation and deserialisation of a default configuration as YAML.
     */
    #[test]
    fn test_yaml_serialisation_and_deserialisation_default_configuration()
    {
        let configuration = PromptConfiguration::default();
        let deserialised: PromptConfiguration = yaml_serde::from_str(&yaml_serde::to_string(&configuration).expect("Failed to serialise the default configuration.")).expect("Failed to deserialise the default configuration.");

        assert_eq!(configuration.content, deserialised.content);
        assert_eq!(configuration.modules, deserialised.modules);
    }
}