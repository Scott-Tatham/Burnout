/*!
 * Stores the PowerShell shell configuration.
 */
use serde::{Serialize, Deserialize};

/**
 * Stores the PowerShell shell configuration.
 */
#[derive(Serialize, Deserialize)]
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
     * Tests the default values for PowerShell shell configuration are correct.
     */
    #[test]
    fn test_default_values_are_correct()
    {
        let configuration = PowerShellConfiguration::default();

        assert_eq!(configuration.setup, Some(r#"[Console]::OutputEncoding = [System.Text.Encoding]::UTF8; $burnout = (Get-Command burnout).Source; $PSContinuationPrompt = { & $burnout continuation }; function global:prompt { $host.UI.RawUI.WindowTitle = (& $burnout window-title); return & $burnout;}"#.to_string()));
    }

    /**
     * Tests the serialisation and deserialisation of the full PowerShell shell configuration as TOML.
     */
    #[test]
    fn test_toml_serialisation_and_deserialisation_full_configuration()
    {
        let configuration = PowerShellConfiguration
        {
            setup: Some(r#"[Console]::OutputEncoding = [System.Text.Encoding]::UTF8; $burnout = (Get-Command burnout).Source; $PSContinuationPrompt = { & $burnout continuation }; function global:prompt { $host.UI.RawUI.WindowTitle = (& $burnout window-title); return & $burnout;}"#.to_string())
        };

        let deserialised: PowerShellConfiguration = toml::from_str(&toml::to_string(&configuration).expect("Failed to serialise the full configuration.")).expect("Failed to deserialise the full configuration.");

        assert_eq!(configuration.setup, deserialised.setup);
    }

    /**
     * Tests the serialisation and deserialisation of an empty PowerShell shell configuration as TOML.
     */
    #[test]
    fn test_toml_serialisation_and_deserialisation_empty_configuration()
    {
        let configuration = PowerShellConfiguration
        {
            setup: None
        };

        let deserialised: PowerShellConfiguration = toml::from_str(&toml::to_string(&configuration).expect("Failed to serialise the empty configuration.")).expect("Failed to deserialise the empty configuration.");

        assert!(deserialised.setup.is_none());
    }
}