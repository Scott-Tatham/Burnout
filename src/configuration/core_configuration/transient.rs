/*!
 * Stores the transient prompt configuration.
 */
use serde::{Serialize, Deserialize};

/**
 * Stores the transient prompt configuration.
 */
#[derive(Serialize, Deserialize)]
pub struct TransientConfiguration
{
    pub content: Option<String>
}

/**
 * Implements the [Default] trait for the transient prompt configuration.
 */
impl Default for TransientConfiguration
{
    fn default() -> Self
    {
        Self
        {
            content: Some("Transient".to_string())
        }
    }
}

/**
 * Unit tests for the transient prompt configuration.
 */
#[cfg(test)]
mod tests
{
    use super::*;

    /**
     * Tests the default values for transient prompt configuration are correct.
     */
    #[test]
    fn test_default_values_are_correct()
    {
        let configuration = TransientConfiguration::default();

        assert_eq!(configuration.content, Some("Transient".to_string()));
    }

    /**
     * Tests the serialisation and deserialisation of the full transient prompt configuration as TOML.
     */
    #[test]
    fn test_toml_serialisation_and_deserialisation_full_configuration()
    {
        let configuration = TransientConfiguration
        {
            content: Some("Test Transient".to_string())
        };

        let deserialised: TransientConfiguration = toml::from_str(&toml::to_string(&configuration).expect("Failed to serialise the full configuration.")).expect("Failed to deserialise the full configuration.");

        assert_eq!(configuration.content, deserialised.content);
    }

    /**
     * Tests the serialisation and deserialisation of an empty transient prompt configuration as TOML.
     */
    #[test]
    fn test_toml_serialisation_and_deserialisation_empty_configuration()
    {
        let configuration = TransientConfiguration
        {
            content: None
        };

        let deserialised: TransientConfiguration = toml::from_str(&toml::to_string(&configuration).expect("Failed to serialise the empty configuration.")).expect("Failed to deserialise the empty configuration.");

        assert!(deserialised.content.is_none());
    }
}