/*!
 * Stores the base configuration.
 */
use serde::{Serialize, Deserialize};
use crate::
{
    configuration::
    {
        prompt::PromptConfiguration,
        right::RightConfiguration,
        transient::TransientConfiguration,
        right_transient::RightTransientConfiguration,
        continuation::ContinuationConfiguration,
        window_title::WindowTitleConfiguration
    }
};

/**
 * Stores the base configuration.
 */
#[derive(Serialize, Deserialize)]
pub struct BaseConfiguration
{
    pub prompt: Option<PromptConfiguration>,
    pub right: Option<RightConfiguration>,
    pub transient: Option<TransientConfiguration>,
    pub right_transient: Option<RightTransientConfiguration>,
    pub continuation: Option<ContinuationConfiguration>,
    pub window_title: Option<WindowTitleConfiguration>
}

/**
 * Implements the [Default] trait for the prompt.
 */
impl Default for BaseConfiguration
{
    fn default() -> Self
    {
        Self
        {
            prompt: Some(PromptConfiguration::default()),
            right: Some(RightConfiguration::default()),
            transient: Some(TransientConfiguration::default()),
            right_transient: Some(RightTransientConfiguration::default()),
            continuation: Some(ContinuationConfiguration::default()),
            window_title: Some(WindowTitleConfiguration::default())
        }
    }
}

/**
 * Unit tests for the base configuration.
 */
#[cfg(test)]
mod tests
{
    use super::*;

    /**
     * Tests the default values for base configuration are correct.
     */
    #[test]
    fn test_default_values_are_correct()
    {
        let configuration = BaseConfiguration::default();

        assert!(configuration.prompt.is_some());
        assert!(configuration.right.is_some());
        assert!(configuration.transient.is_some());
        assert!(configuration.right_transient.is_some());
        assert!(configuration.continuation.is_some());
        assert!(configuration.window_title.is_some());
    }

    /**
     * Tests the deserialisation of a full configuration as TOML.
     */
    #[test]
    fn test_toml_deserialisation_full_configuration()
    {
        let configuration: BaseConfiguration = toml::from_str(r#"
            [prompt]
            content = "Test Prompt"

            [right]
            content = "Test Right"

            [transient]
            content = "Test Transient"

            [right_transient]
            content = "Test Right Transient"
            
            [continuation]
            content = "Test Continuation"

            [window_title]
            content = "Test Window Title"
        "#).expect("Failed to parse valid TOML.");

        assert_eq!(configuration.prompt.unwrap().content, Some("Test Prompt".to_string()));
        assert_eq!(configuration.right.unwrap().content, Some("Test Right".to_string()));
        assert_eq!(configuration.transient.unwrap().content, Some("Test Transient".to_string()));
        assert_eq!(configuration.right_transient.unwrap().content, Some("Test Right Transient".to_string()));
        assert_eq!(configuration.continuation.unwrap().content, Some("Test Continuation".to_string()));
        assert_eq!(configuration.window_title.unwrap().content, Some("Test Window Title".to_string()));
    }

    /**
     * Tests the deserialisation of a partial configuration as TOML.
     */
    #[test]
    fn test_toml_deserialisation_partial_configuration()
    {
        let toml_string = r#"
            [prompt]
            content = "Test Prompt"
        "#;

        let configuration: BaseConfiguration = toml::from_str(toml_string).expect("Failed to deserialise the partial configuration.");

        assert_eq!(configuration.prompt.unwrap().content, Some("Test Prompt".to_string()));
        assert!(configuration.right.is_none());
        assert!(configuration.transient.is_none());
        assert!(configuration.right_transient.is_none());
        assert!(configuration.continuation.is_none());
        assert!(configuration.window_title.is_none());
    }

    /**
     * Tests the deserialisation of an empty configuration as TOML.
     */
    #[test]
    fn test_toml_deserialisation_empty_configuration()
    {
        let configuration: BaseConfiguration = toml::from_str("").expect("Failed to deserialise the empty configuration.");

        assert!(configuration.prompt.is_none());
        assert!(configuration.right.is_none());
        assert!(configuration.transient.is_none());
        assert!(configuration.right_transient.is_none());
        assert!(configuration.continuation.is_none());
        assert!(configuration.window_title.is_none());
    }

    /**
     * Tests the serialisation and deserialisation of the default configuration as TOML.
     */
    #[test]
    fn test_toml_serialisation_and_deserialisation_default_configuration()
    {
        let configuration = BaseConfiguration::default();
        let deserialised: BaseConfiguration = toml::from_str(&toml::to_string(&configuration).expect("Failed to serialise the base configuration.")).expect("Failed to deserialise the base configuration.");

        assert_eq!(configuration.prompt.unwrap().content, deserialised.prompt.unwrap().content);
        assert_eq!(configuration.right.unwrap().content, deserialised.right.unwrap().content);
        assert_eq!(configuration.transient.unwrap().content, deserialised.transient.unwrap().content);
        assert_eq!(configuration.right_transient.unwrap().content, deserialised.right_transient.unwrap().content);
        assert_eq!(configuration.continuation.unwrap().content, deserialised.continuation.unwrap().content);
        assert_eq!(configuration.window_title.unwrap().content, deserialised.window_title.unwrap().content);
    }
}