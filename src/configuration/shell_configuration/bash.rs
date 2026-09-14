/*!
 * Stores the Bash shell configuration.
 */
use serde::{Serialize, Deserialize};

/**
 * Stores the Bash shell configuration.
 */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct BashConfiguration
{
    pub setup: Option<String>
}

/**
 * Implements the [Default] trait for the Bash shell configuration.
 */
impl Default for BashConfiguration
{
    fn default() -> Self
    {
        Self
        {
            setup: Some(r#"BURNOUT=$(command -v burnout); \
PS1="$($BURNOUT)"; \
RPROMPT="$($BURNOUT right)"; \
PS1_TRANSIENT="$($BURNOUT transient)"; \
RPROMPT_TRANSIENT="$($BURNOUT right-transient)"; \
PS2="$($BURNOUT continuation)" \
PROMPT_COMMAND='echo -ne "\033]0;$($BURNOUT window-title)\007"'"#.to_string())
        }
    }
}

/**
 * Unit tests for the Bash shell configuration.
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
        let configuration = BashConfiguration::default();

        assert_eq!(configuration.setup, Some(r#"BURNOUT=$(command -v burnout); \
PS1="$($BURNOUT)"; \
RPROMPT="$($BURNOUT right)"; \
PS1_TRANSIENT="$($BURNOUT transient)"; \
RPROMPT_TRANSIENT="$($BURNOUT right-transient)"; \
PS2="$($BURNOUT continuation)" \
PROMPT_COMMAND='echo -ne "\033]0;$($BURNOUT window-title)\007"'"#.to_string()));
    }

    /**
     * Tests the deserialisation of an empty configuration as YAML.
     */
    #[test]
    fn test_yaml_deserialisation_empty_configuration()
    {
        let configuration: BashConfiguration = yaml_serde::from_str(String::default().as_str()).expect("Failed to deserialise the empty configuration.");

        assert!(configuration.setup.is_none());
    }

    /**
     * Tests the deserialisation of a full configuration as YAML.
     */
    #[test]
    fn test_yaml_deserialisation_full_configuration()
    {
        let configuration: BashConfiguration = yaml_serde::from_str(r#"BURNOUT=$(command -v burnout); \
PS1="$($BURNOUT)"; \
RPROMPT="$($BURNOUT right)"; \
PS1_TRANSIENT="$($BURNOUT transient)"; \
RPROMPT_TRANSIENT="$($BURNOUT right-transient)"; \
PS2="$($BURNOUT continuation)" \
PROMPT_COMMAND='echo -ne "\033]0;$($BURNOUT window-title)\007"'"#).expect("Failed to deserialise the full configuration.");

        assert_eq!(configuration.setup, Some(r#"BURNOUT=$(command -v burnout); \
PS1="$($BURNOUT)"; \
RPROMPT="$($BURNOUT right)"; \
PS1_TRANSIENT="$($BURNOUT transient)"; \
RPROMPT_TRANSIENT="$($BURNOUT right-transient)"; \
PS2="$($BURNOUT continuation)" \
PROMPT_COMMAND='echo -ne "\033]0;$($BURNOUT window-title)\007"'"#.to_string()));
    }

    /**
     * Tests the serialisation and deserialisation of an empty configuration as YAML.
     */
    #[test]
    fn test_yaml_serialisation_and_deserialisation_empty_configuration()
    {
        let configuration = BashConfiguration
        {
            setup: None
        };

        let deserialised: BashConfiguration = yaml_serde::from_str(&yaml_serde::to_string(&configuration).expect("Failed to serialise the empty configuration.")).expect("Failed to deserialise the empty configuration.");

        assert!(deserialised.setup.is_none());
    }

    /**
     * Tests the serialisation and deserialisation of a default configuration as YAML.
     */
    #[test]
    fn test_yaml_serialisation_and_deserialisation_default_configuration()
    {
        let configuration = BashConfiguration::default();
        let deserialised: BashConfiguration = yaml_serde::from_str(&yaml_serde::to_string(&configuration).expect("Failed to serialise the default configuration.")).expect("Failed to deserialise the default configuration.");

        assert_eq!(configuration.setup, deserialised.setup);
    }

    /**
     * Tests the serialisation and deserialisation of a full configuration as YAML.
     */
    #[test]
    fn test_yaml_serialisation_and_deserialisation_full_configuration()
    {
        let configuration = BashConfiguration
        {
            setup: Some(r#"BURNOUT=$(command -v burnout); \
PS1="$($BURNOUT)"; \
RPROMPT="$($BURNOUT right)"; \
PS1_TRANSIENT="$($BURNOUT transient)"; \
RPROMPT_TRANSIENT="$($BURNOUT right-transient)"; \
PS2="$($BURNOUT continuation)" \
PROMPT_COMMAND='echo -ne "\033]0;$($BURNOUT window-title)\007"'"#.to_string())
        };

        let deserialised: BashConfiguration = yaml_serde::from_str(&yaml_serde::to_string(&configuration).expect("Failed to serialise the full configuration.")).expect("Failed to deserialise the full configuration.");

        assert_eq!(configuration.setup, deserialised.setup);
    }
}