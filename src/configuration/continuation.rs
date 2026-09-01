/*!
 * Stores the continuation configuration.
 */
use serde::{Serialize, Deserialize};

/**
 * Stores the continuation configuration.
 */
#[derive(Serialize, Deserialize)]
pub struct ContinuationConfiguration
{
    pub content: Option<String>
}

/**
 * Implements the [Default] trait for the continuation prompt.
 */
impl Default for ContinuationConfiguration
{
    fn default() -> Self
    {
        Self
        {
            content: Some("Continuation".to_string())
        }
    }
}

/**
 * Unit tests for the continuation prompt configuration.
 */
#[cfg(test)]
mod tests
{
    use super::*;

    /**
     * Tests the default values for continuation prompt configuration are correct.
     */
    #[test]
    fn test_default_values_are_correct()
    {
        let configuration = ContinuationConfiguration::default();

        assert_eq!(configuration.content, Some("Continuation".to_string()));
    }

    /**
     * Tests the serialisation and deserialisation of the continuation prompt configuration as TOML.
     */
    #[test]
    fn test_toml_serialisation_and_deserialisation_full_configuration()
    {
        let configuration = ContinuationConfiguration
        {
            content: Some("Test Continuation".to_string())
        };

        let deserialised: ContinuationConfiguration = toml::from_str(&toml::to_string(&configuration).expect("Failed to serialise the full configuration.")).expect("Failed to deserialise the full configuration.");

        assert_eq!(configuration.content, deserialised.content);
    }

    /**
     * Tests the serialisation and deserialisation of an empty continuation prompt configuration as TOML.
     */
    #[test]
    fn test_toml_serialisation_and_deserialisation_empty_configuration()
    {
        let configuration = ContinuationConfiguration
        {
            content: None
        };

        let deserialised: ContinuationConfiguration = toml::from_str(&toml::to_string(&configuration).expect("Failed to serialise the empty configuration.")).expect("Failed to deserialise the empty configuration.");

        assert!(deserialised.content.is_none());
    }
}