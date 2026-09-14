/*!
 * Stores the Zsh shell configuration.
 */
use serde::{Serialize, Deserialize};

/**
 * Stores the Zsh shell configuration.
 */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
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
     * Tests the default values for the configuration are correct.
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
     * Tests the deserialisation of an empty configuration as YAML.
     */
    #[test]
    fn test_yaml_deserialisation_empty_configuration()
    {
        let configuration: ZshConfiguration = yaml_serde::from_str(String::default().as_str()).expect("Failed to deserialise the empty configuration.");

        assert!(configuration.setup.is_none());
    }

    /**
     * Tests the deserialisation of a full configuration as YAML.
     */
    #[test]
    fn test_yaml_deserialisation_full_configuration()
    {
        let configuration: ZshConfiguration = yaml_serde::from_str(r#"autoload -Uz promptinit \
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
function set_window_title(){{echo -ne "\033]0;$($BURNOUT window-title)\007"}}"#).expect("Failed to deserialise the full configuration.");

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
     * Tests the serialisation and deserialisation of an empty configuration as YAML.
     */
    #[test]
    fn test_toml_serialisation_and_deserialisation_empty_configuration()
    {
        let configuration = ZshConfiguration
        {
            setup: None
        };

        let deserialised: ZshConfiguration = yaml_serde::from_str(&yaml_serde::to_string(&configuration).expect("Failed to serialise the empty configuration.")).expect("Failed to deserialise the empty configuration.");

        assert!(deserialised.setup.is_none());
    }

    /**
     * Tests the serialisation and deserialisation of a default configuration as YAML.
     */
    #[test]
    fn test_yaml_serialisation_and_deserialisation_default_configuration()
    {
        let configuration = ZshConfiguration::default();
        let deserialised: ZshConfiguration = yaml_serde::from_str(&yaml_serde::to_string(&configuration).expect("Failed to serialise the default configuration.")).expect("Failed to deserialise the default configuration.");

        assert_eq!(configuration.setup, deserialised.setup);
    }

    /**
     * Tests the serialisation and deserialisation of a full configuration as YAML.
     */
    #[test]
    fn test_yaml_serialisation_and_deserialisation_full_configuration()
    {
        let configuration = ZshConfiguration
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
        };

        let deserialised: ZshConfiguration = yaml_serde::from_str(&yaml_serde::to_string(&configuration).expect("Failed to serialise the full configuration.")).expect("Failed to deserialise the full configuration.");

        assert_eq!(configuration.setup, deserialised.setup);
    }
}