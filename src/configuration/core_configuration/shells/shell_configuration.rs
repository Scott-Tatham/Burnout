/*!
 * Stores the shell configuration.
 */
use serde::{Deserialize, Serialize};
use super::super::displays::display_configuration::DisplayConfiguration;
use super::super::module::ModuleConfiguration;

/// Stores the shell configuration.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ShellConfiguration
{
    /// The optional [DisplayConfiguration] instance for the prompt configuration.
    pub prompt: Option<DisplayConfiguration>,

    /// The optional [DisplayConfiguration] instance for the right prompt configuration.
    pub right: Option<DisplayConfiguration>,

    /// The optional [DisplayConfiguration] instance for the transient prompt configuration.
    pub transient: Option<DisplayConfiguration>,

    /// The optional [DisplayConfiguration] instance for the right transient prompt configuration.
    pub right_transient: Option<DisplayConfiguration>,

    /// The optional [DisplayConfiguration] instance for the continuation prompt configuration.
    pub continuation: Option<DisplayConfiguration>,

    /// The optional [DisplayConfiguration] instance for the window title configuration.
    pub window_title: Option<DisplayConfiguration>,

    /// The optional collection of [ModuleConfiguration] instances.
    pub modules: Option<Vec<ModuleConfiguration>>
}

/// Implements the [Default] trait for the [ShellConfiguration].
impl Default for ShellConfiguration
{
    /// Initialises a new, default instance of the [ShellConfiguration] struct.
    /// # Returns
    /// `ShellConfiguration` - Returns the new, default instance of the [ShellConfiguration] struct.
    fn default() -> Self
    {
        Self
        {
            prompt: Some(
                DisplayConfiguration
                {
                    content: Some("Prompt".to_string()),
                    modules: Some(Vec::default())
                }
            ),
            right: Some(
                DisplayConfiguration
                {
                    content: Some("Right".to_string()),
                    modules: Some(Vec::default())
                }
            ),
            transient: Some(
                DisplayConfiguration
                {
                    content: Some("Transient".to_string()),
                    modules: Some(Vec::default())
                }
            ),
            right_transient: Some(
                DisplayConfiguration
                {
                    content: Some("Right Transient".to_string()),
                    modules: Some(Vec::default())
                }
            ),
            continuation: Some(
                DisplayConfiguration
                {
                    content: Some("Continuation".to_string()),
                    modules: Some(Vec::default())
                }
            ),
            window_title: Some(
                DisplayConfiguration
                {
                    content: Some("Window Title".to_string()),
                    modules: Some(Vec::default())
                }
            ),
            modules: Some(Vec::default())
        }
    }
}

/// Tests for the [ShellConfiguration].
#[cfg(test)]
mod tests
{
    use super::{ShellConfiguration, DisplayConfiguration};

    /// Tests the default values for the configuration are correct.
    #[test]
    fn test_default_values_are_correct()
    {
        let configuration = ShellConfiguration::default();

        assert!(configuration.prompt.is_some());
        assert!(configuration.right.is_some());
        assert!(configuration.transient.is_some());
        assert!(configuration.right_transient.is_some());
        assert!(configuration.continuation.is_some());
        assert!(configuration.window_title.is_some());
        assert!(configuration.modules.is_some());
    }

    /// Tests the deserialisation of an empty configuration as YAML.
    #[test]
    fn test_yaml_deserialisation_empty_configuration()
    {
        let configuration: ShellConfiguration = yaml_serde::from_str(String::default().as_str()).expect("Failed to deserialise the empty configuration.");

        assert!(configuration.prompt.is_none());
        assert!(configuration.right.is_none());
        assert!(configuration.transient.is_none());
        assert!(configuration.right_transient.is_none());
        assert!(configuration.continuation.is_none());
        assert!(configuration.window_title.is_none());
        assert!(configuration.modules.is_none());
    }

    /// Tests the deserialisation of a partial configuration as YAML.
    #[test]
    fn test_yaml_deserialisation_partial_configuration()
    {
        let configuration: ShellConfiguration = yaml_serde::from_str(r#"prompt:
  content: Display Element"#).expect("Failed to deserialise the partial configuration.");

        assert_eq!(configuration.prompt.unwrap().content, Some("Display Element".to_string()));
        assert!(configuration.right.is_none());
        assert!(configuration.transient.is_none());
        assert!(configuration.right_transient.is_none());
        assert!(configuration.continuation.is_none());
        assert!(configuration.window_title.is_none());
        assert!(configuration.modules.is_none());
    }

    /// Tests the serialisation and deserialisation of an empty configuration as YAML.
    #[test]
    fn test_yaml_serialisation_and_deserialisation_empty_configuration()
    {
        let configuration = ShellConfiguration
        {
            prompt: None,
            right: None,
            transient: None,
            right_transient: None,
            continuation: None,
            window_title: None,
            modules: None
        };

        let deserialised: ShellConfiguration = yaml_serde::from_str(&yaml_serde::to_string(&configuration).expect("Failed to serialise the empty configuration.")).expect("Failed to deserialise the empty configuration.");

        assert!(deserialised.prompt.is_none());
        assert!(deserialised.right.is_none());
        assert!(deserialised.transient.is_none());
        assert!(deserialised.right_transient.is_none());
        assert!(deserialised.continuation.is_none());
        assert!(deserialised.window_title.is_none());
        assert!(deserialised.modules.is_none());
    }

    /// Tests the serialisation and deserialisation of a partial configuration as YAML.
    #[test]
    fn test_yaml_serialisation_and_deserialisation_partial_configuration()
    {
        let configuration = ShellConfiguration
        {
            prompt: Some(DisplayConfiguration
            {
                content: Some("Test Display Element".to_string()),
                modules: None
            }),
            right: None,
            transient: None,
            right_transient: None,
            continuation: None,
            window_title: None,
            modules: None
        };

        let deserialised: ShellConfiguration = yaml_serde::from_str(&yaml_serde::to_string(&configuration).expect("Failed to serialise the empty configuration.")).expect("Failed to deserialise the empty configuration.");

        assert_eq!(configuration.prompt.unwrap().content, deserialised.prompt.unwrap().content);
        assert!(deserialised.right.is_none());
        assert!(deserialised.transient.is_none());
        assert!(deserialised.right_transient.is_none());
        assert!(deserialised.continuation.is_none());
        assert!(deserialised.window_title.is_none());
        assert!(deserialised.modules.is_none());
    }

    /// Tests the serialisation and deserialisation of a default configuration as YAML.
    #[test]
    fn test_yaml_serialisation_and_deserialisation_default_configuration()
    {
        let configuration = ShellConfiguration::default();
        let deserialised: ShellConfiguration = yaml_serde::from_str(&yaml_serde::to_string(&configuration).expect("Failed to serialise the base configuration.")).expect("Failed to deserialise the base configuration.");

        assert_eq!(configuration.prompt.unwrap().content, deserialised.prompt.unwrap().content);
        assert_eq!(configuration.right.unwrap().content, deserialised.right.unwrap().content);
        assert_eq!(configuration.transient.unwrap().content, deserialised.transient.unwrap().content);
        assert_eq!(configuration.right_transient.unwrap().content, deserialised.right_transient.unwrap().content);
        assert_eq!(configuration.continuation.unwrap().content, deserialised.continuation.unwrap().content);
        assert_eq!(configuration.window_title.unwrap().content, deserialised.window_title.unwrap().content);
        assert_eq!(configuration.modules.unwrap(), deserialised.modules.unwrap());
    }
}