/*!
 * Stores the prompt configuration.
 */
use serde::{Serialize, Deserialize};

/**
 * Stores the prompt configuration.
 */
#[derive(Serialize, Deserialize)]
pub struct PromptConfiguration
{
    pub content: Option<String>
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
            content: Some("Prompt".to_string())
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
     * Tests the default values for prompt configuration are correct.
     */
    #[test]
    fn test_default_values_are_correct()
    {
        let configuration = PromptConfiguration::default();

        assert_eq!(configuration.content, Some("Prompt".to_string()));
    }

    /**
     * Tests the serialisation and deserialisation of the full prompt configuration as TOML.
     */
    #[test]
    fn test_toml_serialisation_and_deserialisation_full_configuration()
    {
        let configuration = PromptConfiguration
        {
            content: Some("Test Prompt".to_string())
        };

        let deserialised: PromptConfiguration = toml::from_str(&toml::to_string(&configuration).expect("Failed to serialise the full configuration.")).expect("Failed to deserialise the full configuration.");

        assert_eq!(configuration.content, deserialised.content);
    }

    /**
     * Tests the serialisation and deserialisation of an empty prompt configuration as TOML.
     */
    #[test]
    fn test_toml_serialisation_and_deserialisation_empty_configuration()
    {
        let configuration = PromptConfiguration
        {
            content: None
        };

        let deserialised: PromptConfiguration = toml::from_str(&toml::to_string(&configuration).expect("Failed to serialise the empty configuration.")).expect("Failed to deserialise the empty configuration.");

        assert!(deserialised.content.is_none());
    }
}