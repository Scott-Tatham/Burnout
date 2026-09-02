/*!
 * Stores the Zsh shell configuration.
 */
use serde::{Serialize, Deserialize};

/**
 * Stores the Zsh shell configuration.
 */
#[derive(Serialize, Deserialize)]
pub struct ZshConfiguration
{
    pub setup: Option<String>
}

/**
 * Implements the [Default] trait for the Zsh shell configuration.
 */
impl Default for ZshConfiguration
{
    fn default() -> Self
    {
        Self
        {
            setup: Some(r#"autoload -Uz promptinit \
            promptinit \
            prompt transient \
            setopt transient_rprompt \
            BURNOUT=$(command -v burnout); \
            PROMPT="$($BURNOUT)"; \
            RPROMPT="$($BURNOUT right)"; \
            PROMPT_TRANSIENT="$($BURNOUT transient)"; \
            RPROMPT_TRANSIENT="$($BURNOUT right-transient)"; \
            PS2="$($BURNOUT continuation)" \
            precmd_functions+=(set_window_title); \
            function set_window_title(){{echo -ne "\033]0;$($BURNOUT window-title)\007"}}"#.to_string())
        }
    }
}

/**
 * Unit tests for the Zsh shell configuration.
 */
#[cfg(test)]
mod tests
{
    use super::*;

    /**
     * Tests the default values for Zsh shell configuration are correct.
     */
    #[test]
    fn test_default_values_are_correct()
    {
        let configuration = ZshConfiguration::default();

        assert_eq!(configuration.setup, Some(r#"autoload -Uz promptinit \
            promptinit \
            prompt transient \
            setopt transient_rprompt \
            BURNOUT=$(command -v burnout); \
            PROMPT="$($BURNOUT)"; \
            RPROMPT="$($BURNOUT right)"; \
            PROMPT_TRANSIENT="$($BURNOUT transient)"; \
            RPROMPT_TRANSIENT="$($BURNOUT right-transient)"; \
            PS2="$($BURNOUT continuation)" \
            precmd_functions+=(set_window_title); \
            function set_window_title(){{echo -ne "\033]0;$($BURNOUT window-title)\007"}}"#.to_string()));
    }

    /**
     * Tests the serialisation and deserialisation of the full Zsh shell configuration as TOML.
     */
    #[test]
    fn test_toml_serialisation_and_deserialisation_full_configuration()
    {
        let configuration = ZshConfiguration
        {
            setup: Some(r#"
            autoload -Uz promptinit \
            promptinit \
            prompt transient \
            setopt transient_rprompt \
            BURNOUT=$(command -v burnout); \
            PROMPT="$($BURNOUT)"; \
            RPROMPT="$($BURNOUT right)"; \
            PROMPT_TRANSIENT="$($BURNOUT transient)"; \
            RPROMPT_TRANSIENT="$($BURNOUT right-transient)"; \
            PS2="$($BURNOUT continuation)" \
            precmd_functions+=(set_window_title); \
            function set_window_title(){{echo -ne "\033]0;$($BURNOUT window-title)\007"}}"#.to_string())
        };

        let deserialised: ZshConfiguration = toml::from_str(&toml::to_string(&configuration).expect("Failed to serialise the full configuration.")).expect("Failed to deserialise the full configuration.");

        assert_eq!(configuration.setup, deserialised.setup);
    }

    /**
     * Tests the serialisation and deserialisation of an empty Zsh shell configuration as TOML.
     */
    #[test]
    fn test_toml_serialisation_and_deserialisation_empty_configuration()
    {
        let configuration = ZshConfiguration
        {
            setup: None
        };

        let deserialised: ZshConfiguration = toml::from_str(&toml::to_string(&configuration).expect("Failed to serialise the empty configuration.")).expect("Failed to deserialise the empty configuration.");

        assert!(deserialised.setup.is_none());
    }
}