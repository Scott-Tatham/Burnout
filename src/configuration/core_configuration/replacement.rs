/*!
 * Stores a replacement configuration.
 */
use serde::{Deserialize, Serialize};

/// Stores a replacement configuration.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ReplacementConfiguration
{
    /// The optional target text to replace.
    pub target: Option<String>,

    /// The optional value to replace the target text with.
    pub value: Option<String>
}

/// Implements the [Default] trait for the [ReplacementConfiguration].
impl Default for ReplacementConfiguration
{
    /// Initialises a new, default instance of the [ReplacementConfiguration] struct.
    /// # Returns
    /// `ReplacementConfiguration` - Returns the new, default instance of the [ReplacementConfiguration] struct.
    fn default() -> Self
    {
        Self
        {
            target: None,
            value: None
        }
    }
}

/// Tests for the [ReplacementConfiguration].
#[cfg(test)]
mod tests
{
    use super::ReplacementConfiguration;

    /// Tests the default values for the configuration are correct.
    #[test]
    fn test_default_values_are_correct()
    {
        let configuration = ReplacementConfiguration::default();

        assert!(configuration.target.is_none());
        assert!(configuration.value.is_none());
    }

    /// Tests the deserialisation of an empty configuration as YAML.
    #[test]
    fn test_yaml_deserialisation_empty_configuration()
    {
        let configuration: ReplacementConfiguration = yaml_serde::from_str(String::default().as_str()).expect("Failed to deserialise the empty configuration.");

        assert!(configuration.target.is_none());
        assert!(configuration.value.is_none());
    }

    /// Tests the deserialisation of a partial configuration as YAML.
    #[test]
    fn test_yaml_deserialisation_partial_configuration()
    {
        let configuration: ReplacementConfiguration = yaml_serde::from_str(r#"target: 'Test Target'"#).expect("Failed to deserialise the partial configuration.");

        assert_eq!(configuration.target, Some("Test Target".to_string()));
        assert!(configuration.value.is_none());
    }

    /// Tests the deserialisation of a full configuration as YAML.
    #[test]
    fn test_yaml_deserialisation_full_configuration()
    {
        let configuration: ReplacementConfiguration = yaml_serde::from_str(r#"target: 'Test Target'
value: 'Test Value'"#).expect("Failed to parse valid YAML.");

        assert_eq!(configuration.target, Some("Test Target".to_string()));
        assert_eq!(configuration.value, Some("Test Value".to_string()));
    }

    /// Tests the serialisation and deserialisation of an empty configuration as YAML.
    #[test]
    fn test_yaml_serialisation_and_deserialisation_empty_configuration()
    {
        let configuration = ReplacementConfiguration
        {
            target: None,
            value: None
        };

        let deserialised: ReplacementConfiguration = yaml_serde::from_str(&yaml_serde::to_string(&configuration).expect("Failed to serialise the empty configuration.")).expect("Failed to deserialise the empty configuration.");

        assert!(deserialised.target.is_none());
        assert!(deserialised.value.is_none());
    }

    /// Tests the serialisation and deserialisation of a partial configuration as YAML.
    #[test]
    fn test_yaml_serialisation_and_deserialisation_partial_configuration()
    {
        let configuration = ReplacementConfiguration
        {
            target: Some("Test Target".to_string()),
            value: None
        };

        let deserialised: ReplacementConfiguration = yaml_serde::from_str(&yaml_serde::to_string(&configuration).expect("Failed to serialise the partial configuration.")).expect("Failed to deserialise the partial configuration.");

        assert_eq!(deserialised.target, Some("Test Target".to_string()));
        assert!(deserialised.value.is_none());
    }

    /// Tests the serialisation and deserialisation of a default configuration as YAML.
    #[test]
    fn test_yaml_serialisation_and_deserialisation_default_configuration()
    {
        let configuration = ReplacementConfiguration::default();
        let deserialised: ReplacementConfiguration = yaml_serde::from_str(&yaml_serde::to_string(&configuration).expect("Failed to serialise the default configuration.")).expect("Failed to deserialise the default configuration.");

        assert_eq!(configuration.target, deserialised.target);
        assert_eq!(configuration.value, deserialised.value);
    }

    /// Tests the serialisation and deserialisation of a full configuration as YAML.
    #[test]
    fn test_yaml_serialisation_and_deserialisation_full_configuration()
    {
        let configuration = ReplacementConfiguration
        {
            target: Some("Test Target".to_string()),
            value: Some("Test Value".to_string())
        };

        let deserialised: ReplacementConfiguration = yaml_serde::from_str(&yaml_serde::to_string(&configuration).expect("Failed to serialise the full configuration.")).expect("Failed to deserialise the full configuration.");

        assert_eq!(configuration.target, deserialised.target);
        assert_eq!(configuration.value, deserialised.value);
    }
}