/*!
 * Stores the continuation prompt configuration.
 */
use serde::{Serialize, Deserialize};
use super::super::module::ModuleConfiguration;

/// Stores the configuration for a display element.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DisplayConfiguration
{
    /// The optional content for the display prompt.
    pub content: Option<String>,

    /// The optional collection of [ModuleConfiguration] instances.
    pub modules: Option<Vec<ModuleConfiguration>>
}

/// Tests for the [DisplayConfiguration].
#[cfg(test)]
mod tests
{
    use super::DisplayConfiguration;

    /// Tests the deserialisation of an empty configuration as YAML.
    #[test]
    fn test_yaml_deserialisation_empty_configuration()
    {
        let configuration: DisplayConfiguration = yaml_serde::from_str(String::default().as_str()).expect("Failed to deserialise the empty configuration.");

        assert!(configuration.content.is_none());
        assert!(configuration.modules.is_none());
    }

    /// Tests the deserialisation of a partial configuration as YAML.
    #[test]
    fn test_yaml_deserialisation_partial_configuration()
    {
        let configuration: DisplayConfiguration = yaml_serde::from_str(r#"content: Test Display Element"#).expect("Failed to parse valid YAML.");

        assert_eq!(configuration.content, Some("Test Display Element".to_string()));
        assert!(configuration.modules.is_none());
    }

    /// Tests the serialisation and deserialisation of an empty configuration as YAML.
    #[test]
    fn test_yaml_serialisation_and_deserialisation_empty_configuration()
    {
        let configuration = DisplayConfiguration
        {
            content: None,
            modules: None
        };

        let serialised = &yaml_serde::to_string(&configuration).expect("Failed to serialise the empty configuration.");
        let deserialised: DisplayConfiguration = yaml_serde::from_str(serialised).expect("Failed to deserialise the empty configuration.");

        assert!(deserialised.content.is_none());
        assert!(deserialised.modules.is_none());
    }

    /// Tests the serialisation and deserialisation of a full configuration as YAML.
    #[test]
    fn test_yaml_serialisation_and_deserialisation_partial_configuration()
    {
        let configuration = DisplayConfiguration
        {
            content: Some("Test Display Element".to_string()),
            modules: None
        };

        let serialised = &yaml_serde::to_string(&configuration).expect("Failed to serialise the partial configuration.");
        let deserialised: DisplayConfiguration = yaml_serde::from_str(serialised).expect("Failed to deserialise the partial configuration.");

        assert_eq!(configuration.content, deserialised.content);
        assert!(deserialised.modules.is_none());
    }
}