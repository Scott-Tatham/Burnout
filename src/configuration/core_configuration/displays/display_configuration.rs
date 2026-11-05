/*!
 * Stores the continuation prompt configuration.
 */
use serde::{Serialize, Deserialize};
use super::super::segment::SegmentConfiguration;

/// Stores the configuration for a display element.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DisplayConfiguration
{
    /// The collection of [SegmentConfiguration] instances.
    pub segments: Option<Vec<SegmentConfiguration>>
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

        assert!(configuration.segments.is_none());
    }

    /// Tests the serialisation and deserialisation of an empty configuration as YAML.
    #[test]
    fn test_yaml_serialisation_and_deserialisation_empty_configuration()
    {
        let configuration = DisplayConfiguration
        {
            segments: None
        };

        let serialised = &yaml_serde::to_string(&configuration).expect("Failed to serialise the empty configuration.");
        let deserialised: DisplayConfiguration = yaml_serde::from_str(serialised).expect("Failed to deserialise the empty configuration.");

        assert!(deserialised.segments.is_none());
    }
}