/*!
 * Stores a segment configuration.
 */
use super::style::StyleConfiguration;
use super::condition::ConditionConfiguration;
use serde::{Deserialize, Serialize};

/**
 * Stores a segment configuration.
 */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SegmentConfiguration
{
    pub prefix: Option<String>,
    pub suffix: Option<String>,
    pub variable: Option<String>,
    pub style: Option<StyleConfiguration>,
    pub condition: Option<ConditionConfiguration>
}

/**
 * Implements the [Default] trait for the segment configuration.
 */
impl Default for SegmentConfiguration
{
    fn default() -> Self
    {
        Self
        {
            prefix: Some(String::default()),
            suffix: Some(String::default()),
            variable: Some(String::default()),
            style: Some(StyleConfiguration::default()),
            condition: Some(ConditionConfiguration::default())
        }
    }
}

/**
 * Unit tests for the segment configuration.
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
        let configuration = SegmentConfiguration::default();

        assert!(configuration.prefix.is_some());
        assert!(configuration.suffix.is_some());
        assert!(configuration.variable.is_some());
        assert!(configuration.style.is_some());
        assert!(configuration.condition.is_some());
    }

    /**
     * Tests the deserialisation of an empty configuration as YAML.
     */
    #[test]
    fn test_yaml_deserialisation_empty_configuration()
    {
        let configuration: SegmentConfiguration = yaml_serde::from_str(String::default().as_str()).expect("Failed to deserialise the empty configuration.");

        assert!(configuration.prefix.is_none());
        assert!(configuration.suffix.is_none());
        assert!(configuration.variable.is_none());
        assert!(configuration.style.is_none());
        assert!(configuration.condition.is_none());
    }

    /**
     * Tests the deserialisation of a partial configuration as YAML.
     */
    #[test]
    fn test_yaml_deserialisation_partial_configuration()
    {
        let configuration: SegmentConfiguration = yaml_serde::from_str(r#"segment:
  prefix: 'Test Prefix'"#).expect("Failed to deserialise the partial configuration.");

        assert_eq!(configuration.prefix, Some("Test Prefix".to_string()));
        assert!(configuration.suffix.is_none());
        assert!(configuration.variable.is_none());
        assert!(configuration.style.is_none());
        assert!(configuration.condition.is_none());
    }

    /**
     * Tests the serialisation and deserialisation of an empty configuration as YAML.
     */
    #[test]
    fn test_yaml_serialisation_and_deserialisation_empty_configuration()
    {
        let configuration = SegmentConfiguration
        {
            prefix: None,
            suffix: None,
            variable: None,
            style: None,
            condition: None
        };

        let deserialised: SegmentConfiguration = yaml_serde::from_str(&yaml_serde::to_string(&configuration).expect("Failed to serialise the empty configuration.")).expect("Failed to deserialise the empty configuration.");

        assert!(deserialised.prefix.is_none());
        assert!(deserialised.suffix.is_none());
        assert!(deserialised.variable.is_none());
        assert!(deserialised.style.is_none());
        assert!(deserialised.condition.is_none());
    }

    /**
     * Tests the serialisation and deserialisation of a partial configuration as YAML.
     */
    #[test]
    fn test_yaml_serialisation_and_deserialisation_partial_configuration()
    {
        let configuration = SegmentConfiguration
        {
            prefix: Some("Test Prefix".to_string()),
            suffix: None,
            variable: None,
            style: None,
            condition: None
        };

        let deserialised: SegmentConfiguration = yaml_serde::from_str(&yaml_serde::to_string(&configuration).expect("Failed to serialise the partial configuration.")).expect("Failed to deserialise the partial configuration.");

        assert_eq!(deserialised.prefix, Some("Test Prefix".to_string()));
        assert!(deserialised.suffix.is_none());
        assert!(deserialised.variable.is_none());
        assert!(deserialised.style.is_none());
        assert!(deserialised.condition.is_none());
    }

    /**
     * Tests the serialisation and deserialisation of a default configuration as YAML.
     */
    #[test]
    fn test_yaml_serialisation_and_deserialisation_default_configuration()
    {
        let configuration = SegmentConfiguration::default();
        let deserialised: SegmentConfiguration = yaml_serde::from_str(&yaml_serde::to_string(&configuration).expect("Failed to serialise the default configuration.")).expect("Failed to deserialise the default configuration.");

        assert_eq!(configuration.prefix, deserialised.prefix);
        assert_eq!(configuration.suffix, deserialised.suffix);
        assert_eq!(configuration.variable, deserialised.variable);
        assert_eq!(configuration.style, deserialised.style);
        assert_eq!(configuration.condition, deserialised.condition);
    }
}