/*!
 * Stores the right prompt configuration.
 */
use serde::{Serialize, Deserialize};

/**
 * Stores the right prompt configuration.
 */
#[derive(Serialize, Deserialize)]
pub struct RightConfiguration
{
    pub content: Option<String>
}

/**
 * Implements the [Default] trait for the right prompt.
 */
impl Default for RightConfiguration
{
    fn default() -> Self
    {
        Self
        {
            content: Some("Right".to_string())
        }
    }
}

/**
 * Unit tests for the right prompt configuration.
 */
#[cfg(test)]
mod tests
{
    use super::*;

    /**
     * Tests the default values for right prompt configuration are correct.
     */
    #[test]
    fn test_default_values_are_correct()
    {
        let configuration = RightConfiguration::default();

        assert_eq!(configuration.content, Some("Right".to_string()));
    }

    /**
     * Tests the serialisation and deserialisation of the right prompt configuration as TOML.
     */
    #[test]
    fn test_toml_serialisation_and_deserialisation_full_configuration()
    {
        let configuration = RightConfiguration
        {
            content: Some("Test Right".to_string())
        };

        let deserialised: RightConfiguration = toml::from_str(&toml::to_string(&configuration).expect("Failed to serialise the full configuration.")).expect("Failed to deserialise the full configuration.");

        assert_eq!(configuration.content, deserialised.content);
    }

    /**
     * Tests the serialisation and deserialisation of an empty right prompt configuration as TOML.
     */
    #[test]
    fn test_toml_serialisation_and_deserialisation_empty_configuration()
    {
        let configuration = RightConfiguration
        {
            content: None
        };

        let deserialised: RightConfiguration = toml::from_str(&toml::to_string(&configuration).expect("Failed to serialise the empty configuration.")).expect("Failed to deserialise the empty configuration.");

        assert!(deserialised.content.is_none());
    }
}