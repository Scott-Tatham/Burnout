/*!
 * Stores the PowerShell shell configuration.
 */
use serde::{Serialize, Deserialize};

/**
 * Stores the PowerShell shell configuration.
 */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PowerShellConfiguration
{
    pub setup: Option<String>
}

/**
 * Implements the [Default] trait for the PowerShell shell configuration.
 */
impl Default for PowerShellConfiguration
{
    fn default() -> Self
    {
        Self
        {
            setup: Some(r#"[Console]::OutputEncoding = [System.Text.Encoding]::UTF8; $burnout = (Get-Command burnout).Source; $PSContinuationPrompt = { & $burnout continuation }; function global:prompt { $host.UI.RawUI.WindowTitle = (& $burnout window-title); return & $burnout;}"#.to_string())
        }
    }
}

/**
 * Unit tests for the PowerShell shell configuration.
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
        let configuration = PowerShellConfiguration::default();

        assert_eq!(configuration.setup, Some(r#"[Console]::OutputEncoding = [System.Text.Encoding]::UTF8; $burnout = (Get-Command burnout).Source; $PSContinuationPrompt = { & $burnout continuation }; function global:prompt { $host.UI.RawUI.WindowTitle = (& $burnout window-title); return & $burnout;}"#.to_string()));
    }

    /**
     * Tests the deserialisation of an empty configuration as YAML.
     */
    #[test]
    fn test_yaml_deserialisation_empty_configuration()
    {
        let configuration: PowerShellConfiguration = yaml_serde::from_str(String::default().as_str()).expect("Failed to deserialise the empty configuration.");

        assert!(configuration.setup.is_none());
    }

    /**
     * Tests the deserialisation of a full configuration as YAML.
     */
    #[test]
    fn test_yaml_deserialisation_full_configuration()
    {
        let configuration: PowerShellConfiguration = yaml_serde::from_str(r#"[Console]::OutputEncoding = [System.Text.Encoding]::UTF8; $burnout = (Get-Command burnout).Source; $PSContinuationPrompt = { & $burnout continuation }; function global:prompt { $host.UI.RawUI.WindowTitle = (& $burnout window-title); return & $burnout;}"#).expect("Failed to deserialise the full configuration.");

        assert_eq!(configuration.setup, Some(r#"[Console]::OutputEncoding = [System.Text.Encoding]::UTF8; $burnout = (Get-Command burnout).Source; $PSContinuationPrompt = { & $burnout continuation }; function global:prompt { $host.UI.RawUI.WindowTitle = (& $burnout window-title); return & $burnout;}"#.to_string()));
    }

    /**
     * Tests the serialisation and deserialisation of an empty configuration as YAML.
     */
    #[test]
    fn test_yaml_serialisation_and_deserialisation_empty_configuration()
    {
        let configuration = PowerShellConfiguration
        {
            setup: None
        };

        let deserialised: PowerShellConfiguration = yaml_serde::from_str(&yaml_serde::to_string(&configuration).expect("Failed to serialise the empty configuration.")).expect("Failed to deserialise the empty configuration.");

        assert!(deserialised.setup.is_none());
    }

    /**
     * Tests the serialisation and deserialisation of a default configuration as YAML.
     */
    #[test]
    fn test_yaml_serialisation_and_deserialisation_default_configuration()
    {
        let configuration = PowerShellConfiguration::default();
        let deserialised: PowerShellConfiguration = yaml_serde::from_str(&yaml_serde::to_string(&configuration).expect("Failed to serialise the default configuration.")).expect("Failed to deserialise the default configuration.");

        assert_eq!(configuration.setup, deserialised.setup);
    }

    /**
     * Tests the serialisation and deserialisation of a full configuration as YAML.
     */
    #[test]
    fn test_yaml_serialisation_and_deserialisation_full_configuration()
    {
        let configuration = PowerShellConfiguration
        {
            setup: Some(r#"[Console]::OutputEncoding = [System.Text.Encoding]::UTF8; $burnout = (Get-Command burnout).Source; $PSContinuationPrompt = { & $burnout continuation }; function global:prompt { $host.UI.RawUI.WindowTitle = (& $burnout window-title); return & $burnout;}"#.to_string())
        };

        let deserialised: PowerShellConfiguration = yaml_serde::from_str(&yaml_serde::to_string(&configuration).expect("Failed to serialise the full configuration.")).expect("Failed to deserialise the full configuration.");

        assert_eq!(configuration.setup, deserialised.setup);
    }
}