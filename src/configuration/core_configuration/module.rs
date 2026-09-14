/*!
 * Stores a module configuration.
 */
use super::segment::SegmentConfiguration;
use super::style::StyleConfiguration;
use super::condition::ConditionConfiguration;
use serde::{Deserialize, Serialize};
use yaml_serde::Value;

/**
 * Stores a module configuration.
 */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ModuleConfiguration
{
    pub name: Option<String>,
    pub arguments: Option<Vec<Value>>,
    pub segments: Option<Vec<SegmentConfiguration>>,
    pub style: Option<StyleConfiguration>,
    pub condition: Option<ConditionConfiguration>
}

/**
 * Implements the [Default] trait for the module configuration.
 */
impl Default for ModuleConfiguration
{
    fn default() -> Self
    {
        Self
        {
            name: Some(String::default()),
            arguments: Some(Vec::default()),
            segments: Some(Vec::default()),
            style: Some(StyleConfiguration::default()),
            condition: Some(ConditionConfiguration::default())
        }
    }
}

/**
 * Unit tests for the module configuration.
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
        let configuration = ModuleConfiguration::default();

        assert!(configuration.name.is_some());
        assert!(configuration.arguments.is_some());
        assert!(configuration.segments.is_some());
        assert!(configuration.style.is_some());
        assert!(configuration.condition.is_some());
    }

    /**
     * Tests the deserialisation of an empty configuration as YAML.
     */
    #[test]
    fn test_yaml_deserialisation_empty_configuration()
    {
        let configuration: ModuleConfiguration = yaml_serde::from_str(String::default().as_str()).expect("Failed to deserialise the empty configuration.");

        assert!(configuration.name.is_none());
        assert!(configuration.arguments.is_none());
        assert!(configuration.segments.is_none());
        assert!(configuration.style.is_none());
        assert!(configuration.condition.is_none());
    }

    /**
     * Tests the deserialisation of a partial configuration as YAML.
     */
    #[test]
    fn test_yaml_deserialisation_partial_configuration()
    {
        let configuration: ModuleConfiguration = yaml_serde::from_str(r#"module:
  name: 'Test Name'"#).expect("Failed to deserialise the partial configuration.");

        assert_eq!(configuration.name, Some("Test Name".to_string()));
        assert!(configuration.arguments.is_none());
        assert!(configuration.segments.is_none());
        assert!(configuration.style.is_none());
        assert!(configuration.condition.is_none());
    }

    /**
     * Tests the serialisation and deserialisation of an empty configuration as YAML.
     */
    #[test]
    fn test_yaml_serialisation_and_deserialisation_empty_configuration()
    {
        let configuration = ModuleConfiguration
        {
            name: None,
            arguments: None,
            segments: None,
            style: None,
            condition: None
        };

        let deserialised: ModuleConfiguration = yaml_serde::from_str(&yaml_serde::to_string(&configuration).expect("Failed to serialise the empty configuration.")).expect("Failed to deserialise the empty configuration.");

        assert!(deserialised.name.is_none());
        assert!(deserialised.arguments.is_none());
        assert!(deserialised.segments.is_none());
        assert!(deserialised.style.is_none());
        assert!(deserialised.condition.is_none());
    }

    /**
     * Tests the serialisation and deserialisation of a partial configuration as YAML.
     */
    #[test]
    fn test_yaml_serialisation_and_deserialisation_partial_configuration()
    {
        let configuration = ModuleConfiguration
        {
            name: Some("Test Name".to_string()),
            arguments: None,
            segments: None,
            style: None,
            condition: None
        };

        let deserialised: ModuleConfiguration = yaml_serde::from_str(&yaml_serde::to_string(&configuration).expect("Failed to serialise the partial configuration.")).expect("Failed to deserialise the partial configuration.");

        assert_eq!(deserialised.name, Some("Test Name".to_string()));
        assert!(deserialised.arguments.is_none());
        assert!(deserialised.segments.is_none());
        assert!(deserialised.style.is_none());
        assert!(deserialised.condition.is_none());
    }

    /**
     * Tests the serialisation and deserialisation of a default configuration as YAML.
     */
    #[test]
    fn test_yaml_serialisation_and_deserialisation_default_configuration()
    {
        let configuration = ModuleConfiguration::default();
        let deserialised: ModuleConfiguration = yaml_serde::from_str(&yaml_serde::to_string(&configuration).expect("Failed to serialise the default configuration.")).expect("Failed to deserialise the default configuration.");

        assert_eq!(configuration.name, deserialised.name);
        assert_eq!(configuration.arguments, deserialised.arguments);
        assert_eq!(configuration.segments, deserialised.segments);
        assert_eq!(configuration.style, deserialised.style);
        assert_eq!(configuration.condition, deserialised.condition);
    }
}