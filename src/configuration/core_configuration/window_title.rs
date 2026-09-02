/*!
 * Stores the window title configuration.
 */
use serde::{Serialize, Deserialize};

/**
 * Stores the window title configuration.
 */
#[derive(Serialize, Deserialize)]
pub struct WindowTitleConfiguration
{
    pub content: Option<String>
}

/**
 * Implements the [Default] trait for the window title configuration.
 */
impl Default for WindowTitleConfiguration
{
    fn default() -> Self
    {
        Self
        {
            content: Some(" Burnout ".to_string())
        }
    }
}

/**
 * Unit tests for the window title configuration.
 */
#[cfg(test)]
mod tests
{
    use super::*;

    /**
     * Tests the default values for window title configuration are correct.
     */
    #[test]
    fn test_default_values_are_correct()
    {
        let configuration = WindowTitleConfiguration::default();

        assert_eq!(configuration.content, Some(" Burnout ".to_string()));
    }

    /**
     * Tests the serialisation and deserialisation of the full window title configuration as TOML.
     */
    #[test]
    fn test_toml_serialisation_and_deserialisation_full_configuration()
    {
        let configuration = WindowTitleConfiguration
        {
            content: Some("Test Window Title".to_string())
        };

        let deserialised: WindowTitleConfiguration = toml::from_str(&toml::to_string(&configuration).expect("Failed to serialise the full configuration.")).expect("Failed to deserialise the full configuration.");

        assert_eq!(configuration.content, deserialised.content);
    }

    /**
     * Tests the serialisation and deserialisation of an empty window title configuration as TOML.
     */
    #[test]
    fn test_toml_serialisation_and_deserialisation_empty_configuration()
    {
        let configuration = WindowTitleConfiguration
        {
            content: None
        };

        let deserialised: WindowTitleConfiguration = toml::from_str(&toml::to_string(&configuration).expect("Failed to serialise the empty configuration.")).expect("Failed to deserialise the empty configuration.");

        assert!(deserialised.content.is_none());
    }
}