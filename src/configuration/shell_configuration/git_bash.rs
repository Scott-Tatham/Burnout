/*!
 * Stores the Git Bash shell configuration.
 */
use serde::{Serialize, Deserialize};

/**
 * Stores the Git Bash shell configuration.
 */
#[derive(Serialize, Deserialize)]
pub struct GitBashConfiguration
{
    pub setup: Option<String>
}

/**
 * Implements the [Default] trait for the Git Bash shell configuration.
 */
impl Default for GitBashConfiguration
{
    fn default() -> Self
    {
        Self
        {
            setup: Some(r#"BURNOUT=$(command -v burnout.exe); \
        PS1=$($BURNOUT); \
        RPROMPT="$($BURNOUT right)"; \
        PS1_TRANSIENT="$($BURNOUT transient)"; \
        RPROMPT_TRANSIENT="$($BURNOUT right-transient)"; \
        PS2="$($BURNOUT continuation)" \
        PROMPT_COMMAND='echo -ne "\033]0;$($BURNOUT window-title)\007"'"#.to_string())
        }
    }
}

/**
 * Unit tests for the Git Bash shell configuration.
 */
#[cfg(test)]
mod tests
{
    use super::*;

    /**
     * Tests the default values for Cmd shell configuration are correct.
     */
    #[test]
    fn test_default_values_are_correct()
    {
        let configuration = GitBashConfiguration::default();

        assert_eq!(configuration.setup, Some(r#"BURNOUT=$(command -v burnout.exe); \
        PS1=$($BURNOUT); \
        RPROMPT="$($BURNOUT right)"; \
        PS1_TRANSIENT="$($BURNOUT transient)"; \
        RPROMPT_TRANSIENT="$($BURNOUT right-transient)"; \
        PS2="$($BURNOUT continuation)" \
        PROMPT_COMMAND='echo -ne "\033]0;$($BURNOUT window-title)\007"'"#.to_string()));
    }

    /**
     * Tests the serialisation and deserialisation of the full Git Bash shell configuration as TOML.
     */
    #[test]
    fn test_toml_serialisation_and_deserialisation_full_configuration()
    {
        let configuration = GitBashConfiguration
        {
            setup: Some(r#"BURNOUT=$(command -v burnout.exe); \
        PS1="$($BURNOUT)"; \
        RPROMPT="$($BURNOUT right)"; \
        PS1_TRANSIENT="$($BURNOUT transient)"; \
        RPROMPT_TRANSIENT="$($BURNOUT right-transient)"; \
        PS2="$($BURNOUT continuation)" \
        PROMPT_COMMAND='echo -ne "\033]0;$($BURNOUT window-title)\007"'"#.to_string())
        };

        let deserialised: GitBashConfiguration = toml::from_str(&toml::to_string(&configuration).expect("Failed to serialise the full configuration.")).expect("Failed to deserialise the full configuration.");

        assert_eq!(configuration.setup, deserialised.setup);
    }

    /**
     * Tests the serialisation and deserialisation of an empty Git Bash shell configuration as TOML.
     */
    #[test]
    fn test_toml_serialisation_and_deserialisation_empty_configuration()
    {
        let configuration = GitBashConfiguration
        {
            setup: None
        };

        let deserialised: GitBashConfiguration = toml::from_str(&toml::to_string(&configuration).expect("Failed to serialise the empty configuration.")).expect("Failed to deserialise the empty configuration.");

        assert!(deserialised.setup.is_none());
    }
}