/*!
 * Stores the shell configuration.
 */
use serde::{Serialize, Deserialize};

/// Stores the shell configuration.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ShellConfiguration
{
    /// The optional setup script.
    pub setup: Option<String>
}

/// Tests for the [ShellConfiguration].
#[cfg(test)]
mod tests
{
    use super::ShellConfiguration;

    /// Tests the deserialisation of an empty configuration as YAML.
    #[test]
    fn test_yaml_deserialisation_empty_configuration()
    {
        let configuration: ShellConfiguration = yaml_serde::from_str(String::default().as_str()).expect("Failed to deserialise the empty configuration.");

        assert!(configuration.setup.is_none());
    }

    /// Tests the deserialisation of a full configuration as YAML.
    #[test]
    fn test_yaml_deserialisation_full_configuration()
    {
        let setup_line = r#"BURNOUT=$(command -v burnout); \
PS1="$($BURNOUT)"; \
RPROMPT="$($BURNOUT right)"; \
PS1_TRANSIENT="$($BURNOUT transient)"; \
RPROMPT_TRANSIENT="$($BURNOUT right-transient)"; \
PS2="$($BURNOUT continuation)" \
PROMPT_COMMAND='echo -ne "\033]0;$($BURNOUT window-title)\007"'"#;

        let setup = r#"setup: |-
  BURNOUT=$(command -v burnout); \
  PS1="$($BURNOUT)"; \
  RPROMPT="$($BURNOUT right)"; \
  PS1_TRANSIENT="$($BURNOUT transient)"; \
  RPROMPT_TRANSIENT="$($BURNOUT right-transient)"; \
  PS2="$($BURNOUT continuation)" \
  PROMPT_COMMAND='echo -ne "\033]0;$($BURNOUT window-title)\007"'"#;

        let configuration: ShellConfiguration = yaml_serde::from_str(setup).expect("Failed to deserialise the full configuration.");

        assert_eq!(configuration.setup, Some(setup_line.to_string()));
    }

    /// Tests the serialisation and deserialisation of an empty configuration as YAML.
    #[test]
    fn test_yaml_serialisation_and_deserialisation_empty_configuration()
    {
        let configuration = ShellConfiguration
        {
            setup: None
        };

        let serialised= &yaml_serde::to_string(&configuration).expect("Failed to serialise the empty configuration.");
        let deserialised: ShellConfiguration = yaml_serde::from_str(serialised).expect("Failed to deserialise the empty configuration.");

        assert!(deserialised.setup.is_none());
    }

    /// Tests the serialisation and deserialisation of a full configuration as YAML.
    #[test]
    fn test_yaml_serialisation_and_deserialisation_full_configuration()
    {
        let setup = r#"BURNOUT=$(command -v burnout); \
PS1="$($BURNOUT)"; \
RPROMPT="$($BURNOUT right)"; \
PS1_TRANSIENT="$($BURNOUT transient)"; \
RPROMPT_TRANSIENT="$($BURNOUT right-transient)"; \
PS2="$($BURNOUT continuation)" \
PROMPT_COMMAND='echo -ne "\033]0;$($BURNOUT window-title)\007"'"#;

        let configuration = ShellConfiguration
        {
            setup: Some(setup.to_string())
        };

        let serialised = &yaml_serde::to_string(&configuration).expect("Failed to serialise the full configuration.");
        let deserialised: ShellConfiguration = yaml_serde::from_str(serialised).expect("Failed to deserialise the full configuration.");

        assert_eq!(configuration.setup, deserialised.setup);
    }
}