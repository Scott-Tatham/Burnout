/*!
 * Stores the right transient prompt configuration.
 */
use serde::{Serialize, Deserialize};

/**
 * Stores the right transient prompt configuration.
 */
#[derive(Serialize, Deserialize)]
pub struct RightTransientConfiguration
{
    pub content: Option<String>
}

/**
 * Implements the [Default] trait for the right transient prompt.
 */
impl Default for RightTransientConfiguration
{
    fn default() -> Self
    {
        Self
        {
            content: Some("Right Transient".to_string())
        }
    }
}

/**
 * Unit tests for the right transient prompt configuration.
 */
#[cfg(test)]
mod tests
{
    use super::*;

    /**
     * Tests the default values for right transient prompt configuration are correct.
     */
    #[test]
    fn test_default_values_are_correct()
    {
        let configuration = RightTransientConfiguration::default();

        assert_eq!(configuration.content, Some("Right Transient".to_string()));
    }

    /**
     * Tests the serialisation and deserialisation of the right transient prompt configuration as TOML.
     */
    #[test]
    fn test_toml_serialisation_and_deserialisation_full_configuration()
    {
        let configuration = RightTransientConfiguration
        {
            content: Some("Test Right Transient".to_string())
        };

        let deserialised: RightTransientConfiguration = toml::from_str(&toml::to_string(&configuration).expect("Failed to serialise the full configuration.")).expect("Failed to deserialise the full configuration.");

        assert_eq!(configuration.content, deserialised.content);
    }

    /**
     * Tests the serialisation and deserialisation of an empty right transient prompt configuration as TOML.
     */
    #[test]
    fn test_toml_serialisation_and_deserialisation_empty_configuration()
    {
        let configuration = RightTransientConfiguration
        {
            content: None
        };
        
        let deserialised: RightTransientConfiguration = toml::from_str(&toml::to_string(&configuration).expect("Failed to serialise the empty configuration.")).expect("Failed to deserialise the empty configuration.");

        assert!(deserialised.content.is_none());
    }
}