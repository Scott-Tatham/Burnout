/*!
 * Stores a condition configuration.
 */
use serde::{Deserialize, Serialize};
use yaml_serde::Value;

/**
 * Stores a condition configuration.
 */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ConditionConfiguration
{
    pub variable: Option<String>,
    pub operation: Option<String>,
    pub value: Option<Value>
}

/**
 * Implements the [Default] trait for the condition configuration.
 */
impl Default for ConditionConfiguration
{
    fn default() -> Self
    {
        Self
        {
            variable: Some(String::default()),
            operation: Some(String::default()),
            value: Some(Value::default())
        }
    }
}

/**
 * Unit tests for the condition configuration.
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
        let configuration = ConditionConfiguration::default();

        assert!(configuration.variable.is_some());
        assert!(configuration.operation.is_some());
        assert!(configuration.value.is_some());
    }

    /**
     * Tests the deserialisation of an empty configuration as YAML.
     */
    #[test]
    fn test_yaml_deserialisation_empty_configuration()
    {
        let configuration: ConditionConfiguration = yaml_serde::from_str(String::default().as_str()).expect("Failed to deserialise the empty configuration.");

        assert!(configuration.variable.is_none());
        assert!(configuration.operation.is_none());
        assert!(configuration.value.is_none());
    }

    /**
     * Tests the deserialisation of a partial configuration as YAML.
     */
    #[test]
    fn test_yaml_deserialisation_partial_configuration()
    {
        let configuration: ConditionConfiguration = yaml_serde::from_str(r#"condition:
  variable: 'Test Variable'"#).expect("Failed to deserialise the partial configuration.");

        assert_eq!(configuration.variable, Some("Test Variable".to_string()));
        assert!(configuration.operation.is_none());
        assert!(configuration.value.is_none());
    }

    /**
     * Tests the deserialisation of a full configuration as YAML.
     */
    #[test]
    fn test_yaml_deserialisation_full_configuration()
    {
        let configuration: ConditionConfiguration = yaml_serde::from_str(r#"condition:
  variable: 'Test Variable'
  operation: 'Test Operation'
  value: 0"#).expect("Failed to parse valid YAML.");

        assert_eq!(configuration.variable, Some("Test Variable".to_string()));
        assert_eq!(configuration.operation, Some("Test Operation".to_string()));
        assert_eq!(configuration.value.unwrap().as_i64(), Some(0));
    }

    /**
     * Tests the serialisation and deserialisation of an empty configuration as YAML.
     */
    #[test]
    fn test_yaml_serialisation_and_deserialisation_empty_configuration()
    {
        let configuration = ConditionConfiguration
        {
            variable: None,
            operation: None,
            value: None
        };

        let deserialised: ConditionConfiguration = yaml_serde::from_str(&yaml_serde::to_string(&configuration).expect("Failed to serialise the empty configuration.")).expect("Failed to deserialise the empty configuration.");

        assert!(deserialised.variable.is_none());
        assert!(deserialised.operation.is_none());
        assert!(deserialised.value.is_none());
    }

    /**
     * Tests the serialisation and deserialisation of a partial configuration as YAML.
     */
    #[test]
    fn test_yaml_serialisation_and_deserialisation_partial_configuration()
    {
        let configuration = ConditionConfiguration
        {
            variable: Some("Test Variable".to_string()),
            operation: None,
            value: None
        };

        let deserialised: ConditionConfiguration = yaml_serde::from_str(&yaml_serde::to_string(&configuration).expect("Failed to serialise the partial configuration.")).expect("Failed to deserialise the partial configuration.");

        assert_eq!(deserialised.variable, Some("Test Variable".to_string()));
        assert!(deserialised.operation.is_none());
        assert!(deserialised.value.is_none());
    }

    /**
     * Tests the serialisation and deserialisation of a default configuration as YAML.
     */
    #[test]
    fn test_yaml_serialisation_and_deserialisation_default_configuration()
    {
        let configuration = ConditionConfiguration::default();
        let deserialised: ConditionConfiguration = yaml_serde::from_str(&yaml_serde::to_string(&configuration).expect("Failed to serialise the default configuration.")).expect("Failed to deserialise the default configuration.");

        assert_eq!(configuration.variable, deserialised.variable);
        assert_eq!(configuration.operation, deserialised.operation);
        assert_eq!(configuration.value, deserialised.value);
    }

    /**
     * Tests the serialisation and deserialisation of a full configuration as YAML.
     */
    #[test]
    fn test_yaml_serialisation_and_deserialisation_full_configuration()
    {
        let configuration = ConditionConfiguration
        {
            variable: Some("Test Variable".to_string()),
            operation: Some("Test Operation".to_string()),
            value: Some(Value::from(0))
        };

        let deserialised: ConditionConfiguration = yaml_serde::from_str(&yaml_serde::to_string(&configuration).expect("Failed to serialise the full configuration.")).expect("Failed to deserialise the full configuration.");

        assert_eq!(configuration.variable, deserialised.variable);
        assert_eq!(configuration.operation, deserialised.operation);
        assert_eq!(configuration.value, deserialised.value);
    }
}