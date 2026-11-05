/*!
 * Stores the shell configuration.
 */
use serde::{Deserialize, Serialize};
use super::super::displays::display_configuration::DisplayConfiguration;

/// Stores the shell configuration.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ShellConfiguration
{
    /// The [DisplayConfiguration] instance for the prompt configuration.
    pub prompt: Option<DisplayConfiguration>,

    /// The [DisplayConfiguration] instance for the right prompt configuration.
    pub right: Option<DisplayConfiguration>,

    /// The [DisplayConfiguration] instance for the transient prompt configuration.
    pub transient: Option<DisplayConfiguration>,

    /// The [DisplayConfiguration] instance for the right transient prompt configuration.
    pub right_transient: Option<DisplayConfiguration>,

    /// The [DisplayConfiguration] instance for the continuation prompt configuration.
    pub continuation: Option<DisplayConfiguration>,

    /// The [DisplayConfiguration] instance for the window title configuration.
    pub window_title: Option<DisplayConfiguration>
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
                    segments: Some(Vec::default())
                }
            ),
            right: Some(
                DisplayConfiguration
                {
                    segments: Some(Vec::default())
                }
            ),
            transient: Some(
                DisplayConfiguration
                {
                    segments: Some(Vec::default())
                }
            ),
            right_transient: Some(
                DisplayConfiguration
                {
                    segments: Some(Vec::default())
                }
            ),
            continuation: Some(
                DisplayConfiguration
                {
                    segments: Some(Vec::default())
                }
            ),
            window_title: Some(
                DisplayConfiguration
                {
                    segments: Some(Vec::default())
                }
            )
        }
    }
}

/// Tests for the [ShellConfiguration].
#[cfg(test)]
mod tests
{
    use super::ShellConfiguration;

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
            window_title: None
        };

        let deserialised: ShellConfiguration = yaml_serde::from_str(&yaml_serde::to_string(&configuration).expect("Failed to serialise the empty configuration.")).expect("Failed to deserialise the empty configuration.");

        assert!(deserialised.prompt.is_none());
        assert!(deserialised.right.is_none());
        assert!(deserialised.transient.is_none());
        assert!(deserialised.right_transient.is_none());
        assert!(deserialised.continuation.is_none());
        assert!(deserialised.window_title.is_none());
    }
}