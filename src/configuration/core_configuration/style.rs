/*!
 * Stores a style configuration.
 */
use serde::{Deserialize, Serialize};

/// Stores a style configuration.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct StyleConfiguration
{
    /// The optional collection of colours.
    pub colours: Option<Vec<String>>,

    /// The optional pattern for the colours.
    pub pattern: Option<String>,

    /// The optional flag for whether to set the text bold.
    pub bold: Option<bool>,

    /// The optional flag for whether to set the text italicised.
    pub italics: Option<bool>,

    /// The optional flag for whether to set the text underlined.
    pub underline: Option<bool>,

    /// The optional flag for whether to set the text struck through.
    pub strikethrough: Option<bool>
}

/// Implements the [Default] trait for the [StyleConfiguration].
impl Default for StyleConfiguration
{
    /// Initialises a new, default instance of the [StyleConfiguration] struct.
    /// # Returns
    /// `StyleConfiguration` - Returns the new, default instance of the [StyleConfiguration] struct.
    fn default() -> Self
    {
        Self
        {
            colours: Some(Vec::default()),
            pattern: Some(String::default()),
            bold: Some(bool::default()),
            italics: Some(bool::default()),
            underline: Some(bool::default()),
            strikethrough: Some(bool::default())
        }
    }
}

/// Tests for the [StyleConfiguration].
#[cfg(test)]
mod tests
{
    use super::StyleConfiguration;

    /// Tests the default values for the configuration are correct.
    #[test]
    fn test_default_values_are_correct()
    {
        let configuration = StyleConfiguration::default();

        assert!(configuration.colours.is_some());
        assert!(configuration.pattern.is_some());
        assert!(configuration.bold.is_some());
        assert!(configuration.italics.is_some());
        assert!(configuration.underline.is_some());
        assert!(configuration.strikethrough.is_some());
    }

    /// Tests the deserialisation of an empty configuration as YAML.
    #[test]
    fn test_yaml_deserialisation_empty_configuration()
    {
        let configuration: StyleConfiguration = yaml_serde::from_str(String::default().as_str()).expect("Failed to deserialise the empty configuration.");

        assert!(configuration.colours.is_none());
        assert!(configuration.pattern.is_none());
        assert!(configuration.bold.is_none());
        assert!(configuration.italics.is_none());
        assert!(configuration.underline.is_none());
        assert!(configuration.strikethrough.is_none());
    }

    /// Tests the deserialisation of a partial configuration as YAML.
    #[test]
    fn test_yaml_deserialisation_partial_configuration()
    {
        let configuration: StyleConfiguration = yaml_serde::from_str(r#"colours: ['Test Colour 1', 'Test Colour 2', 'Test Colour 3']"#).expect("Failed to deserialise the partial configuration.");
        let colours = configuration.colours.expect("Failed to deserialise the colours.");

        assert_eq!(colours.get(0).expect("Failed to deserialise the Test Colour 1."), "Test Colour 1");
        assert_eq!(colours.get(1).expect("Failed to deserialise the Test Colour 2."), "Test Colour 2");
        assert_eq!(colours.get(2).expect("Failed to deserialise the Test Colour 3."), "Test Colour 3");
        assert!(configuration.pattern.is_none());
        assert!(configuration.bold.is_none());
        assert!(configuration.italics.is_none());
        assert!(configuration.underline.is_none());
        assert!(configuration.strikethrough.is_none());
    }

    /// Tests the deserialisation of a full configuration as YAML.
    #[test]
    fn test_yaml_deserialisation_full_configuration()
    {
        let configuration: StyleConfiguration = yaml_serde::from_str(r#"colours: ['Test Colour 1', 'Test Colour 2', 'Test Colour 3']
pattern: 'Test Pattern'
bold: true
italics: true
underline: true
strikethrough: true"#).expect("Failed to deserialise the full configuration.");
        let colours = configuration.colours.expect("Failed to deserialise the colours.");

        assert_eq!(colours.get(0).expect("Failed to deserialise the Test Colour 1."), "Test Colour 1");
        assert_eq!(colours.get(1).expect("Failed to deserialise the Test Colour 2."), "Test Colour 2");
        assert_eq!(colours.get(2).expect("Failed to deserialise the Test Colour 3."), "Test Colour 3");
        assert_eq!(configuration.pattern, Some("Test Pattern".to_string()));
        assert_eq!(configuration.bold, Some(true));
        assert_eq!(configuration.italics, Some(true));
        assert_eq!(configuration.underline, Some(true));
        assert_eq!(configuration.strikethrough, Some(true));
    }

    /// Tests the serialisation and deserialisation of an empty configuration as YAML.
    #[test]
    fn test_yaml_serialisation_and_deserialisation_empty_configuration()
    {
        let configuration = StyleConfiguration
        {
            colours: None,
            pattern: None,
            bold: None,
            italics: None,
            underline: None,
            strikethrough: None
        };

        let deserialised: StyleConfiguration = yaml_serde::from_str(&yaml_serde::to_string(&configuration).expect("Failed to serialise the empty configuration.")).expect("Failed to deserialise the empty configuration.");

        assert!(deserialised.colours.is_none());
        assert!(deserialised.pattern.is_none());
        assert!(deserialised.bold.is_none());
        assert!(deserialised.italics.is_none());
        assert!(deserialised.underline.is_none());
        assert!(deserialised.strikethrough.is_none());
    }

    /// Tests the serialisation and deserialisation of a partial configuration as YAML.
    #[test]
    fn test_yaml_serialisation_and_deserialisation_partial_configuration()
    {
        let configuration = StyleConfiguration
        {
            colours: Some(vec!["Test Colour 1".to_string(), "Test Colour 2".to_string(), "Test Colour 3".to_string()]),
            pattern: None,
            bold: None,
            italics: None,
            underline: None,
            strikethrough: None
        };

        let deserialised: StyleConfiguration = yaml_serde::from_str(&yaml_serde::to_string(&configuration).expect("Failed to serialise the partial configuration.")).expect("Failed to deserialise the partial configuration.");

        assert_eq!(deserialised.colours.clone().unwrap().get(0).unwrap().as_str(), "Test Colour 1".to_string());
        assert_eq!(deserialised.colours.clone().unwrap().get(1).unwrap().as_str(), "Test Colour 2".to_string());
        assert_eq!(deserialised.colours.clone().unwrap().get(2).unwrap().as_str(), "Test Colour 3".to_string());
        assert!(deserialised.pattern.is_none());
        assert!(deserialised.bold.is_none());
        assert!(deserialised.italics.is_none());
        assert!(deserialised.underline.is_none());
        assert!(deserialised.strikethrough.is_none());
    }

    /// Tests the serialisation and deserialisation of a default configuration as YAML.
    #[test]
    fn test_yaml_serialisation_and_deserialisation_default_configuration()
    {
        let configuration = StyleConfiguration::default();
        let deserialised: StyleConfiguration = yaml_serde::from_str(&yaml_serde::to_string(&configuration).expect("Failed to serialise the default configuration.")).expect("Failed to deserialise the default configuration.");

        assert_eq!(configuration.colours, deserialised.colours);
        assert_eq!(configuration.pattern, deserialised.pattern);
        assert_eq!(configuration.bold, deserialised.bold);
        assert_eq!(configuration.italics, deserialised.italics);
        assert_eq!(configuration.underline, deserialised.underline);
        assert_eq!(configuration.strikethrough, deserialised.strikethrough);
    }

    /// Tests the serialisation and deserialisation of a full configuration as YAML.
    #[test]
    fn test_yaml_serialisation_and_deserialisation_full_configuration()
    {
        let configuration = StyleConfiguration
        {
            colours: Some(vec!["Test Colour 1".to_string(), "Test Colour 2".to_string(), "Test Colour 3".to_string()]),
            pattern: Some("Test Pattern".to_string()),
            bold: Some(true),
            italics: Some(true),
            underline: Some(true),
            strikethrough: Some(true)
        };

        let deserialised: StyleConfiguration = yaml_serde::from_str(&yaml_serde::to_string(&configuration).expect("Failed to serialise the full configuration.")).expect("Failed to deserialise the full configuration.");

        assert_eq!(configuration.colours, deserialised.colours);
        assert_eq!(configuration.pattern, deserialised.pattern);
        assert_eq!(configuration.bold, deserialised.bold);
        assert_eq!(configuration.italics, deserialised.italics);
        assert_eq!(configuration.underline, deserialised.underline);
        assert_eq!(configuration.strikethrough, deserialised.strikethrough);
    }
}