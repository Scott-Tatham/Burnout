/*!
 * Stores a replacement configuration.
 */
use serde::{Deserialize, Serialize};

/**
 * Stores a replacement configuration.
 */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ReplacementConfiguration
{
    pub target: Option<String>,
    pub value: Option<String>
}

/**
 * Implements the [Default] trait for the replacement configuration.
 */
impl Default for ReplacementConfiguration
{
    fn default() -> Self
    {
        Self
        {
            target: Some(String::default()),
            value: Some(String::default())
        }
    }
}

/**
 * Unit tests for the replacement configuration.
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
        let configuration = ReplacementConfiguration::default();

        assert!(configuration.target.is_some());
        assert!(configuration.value.is_some());
    }

    /**
     * Tests the deserialisation of an empty configuration as YAML.
     */
    #[test]
    fn test_yaml_deserialisation_empty_configuration()
    {
        let configuration: ReplacementConfiguration = yaml_serde::from_str(String::default().as_str()).expect("Failed to deserialise the empty configuration.");

        assert!(configuration.target.is_none());
        assert!(configuration.value.is_none());
    }

    /**
     * Tests the deserialisation of a partial configuration as YAML.
     */
    #[test]
    fn test_yaml_deserialisation_partial_configuration()
    {
        let configuration: ReplacementConfiguration = yaml_serde::from_str(r#"replacement:
  target: 'Test Target'"#).expect("Failed to deserialise the partial configuration.");

        assert_eq!(configuration.target, Some("Test Target".to_string()));
        assert!(configuration.value.is_none());
    }

    /**
     * Tests the deserialisation of a full configuration as YAML.
     */
    #[test]
    fn test_yaml_deserialisation_full_configuration()
    {
        let configuration: ReplacementConfiguration = yaml_serde::from_str(r#"replacement:
  target: 'Test Target'
  value: 'Test Value'"#).expect("Failed to parse valid YAML.");

        assert_eq!(configuration.target, Some("Test Target".to_string()));
        assert_eq!(configuration.value, Some("Test Value".to_string()));
    }

    /**
     * Tests the serialisation and deserialisation of an empty configuration as YAML.
     */
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

    /**
     * Tests the serialisation and deserialisation of a partial configuration as YAML.
     */
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

    /**
     * Tests the serialisation and deserialisation of a default configuration as YAML.
     */
    #[test]
    fn test_yaml_serialisation_and_deserialisation_default_configuration()
    {
        let configuration = ReplacementConfiguration::default();
        let deserialised: ReplacementConfiguration = yaml_serde::from_str(&yaml_serde::to_string(&configuration).expect("Failed to serialise the default configuration.")).expect("Failed to deserialise the default configuration.");

        assert_eq!(configuration.target, deserialised.target);
        assert_eq!(configuration.value, deserialised.value);
    }

    /**
     * Tests the serialisation and deserialisation of a full configuration as YAML.
     */
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