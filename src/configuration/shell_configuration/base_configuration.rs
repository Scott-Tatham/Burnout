/*!
 * Stores the base configuration.
 */
use serde::{Deserialize, Serialize};
use super::shell_configuration::ShellConfiguration;

/// Stores the base configuration.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct BaseConfiguration
{
    /// The [ShellConfiguration] instance for Bash.
    pub bash: Option<ShellConfiguration>,
    
    /// The [ShellConfiguration] instance for Zsh.
    pub zsh: Option<ShellConfiguration>,

    /// The [ShellConfiguration] instance for Git Bash.
    pub git_bash: Option<ShellConfiguration>,

    /// The [ShellConfiguration] instance for PowerShell.
    pub powershell: Option<ShellConfiguration>,

    /// The [ShellConfiguration] instance for Cmd.
    pub cmd: Option<ShellConfiguration>
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
            bash: Some(ShellConfiguration
            {
                setup: Some(r#"BURNOUT=$(command -v burnout bash); \
PS1="$($BURNOUT)"; \
RPROMPT="$($BURNOUT right)"; \
PS1_TRANSIENT="$($BURNOUT transient)"; \
RPROMPT_TRANSIENT="$($BURNOUT right_transient)"; \
PS2="$($BURNOUT continuation)" \
PROMPT_COMMAND='echo -ne "\033]0;$($BURNOUT window_title)\007"'"#.to_string())
            }),
            zsh: Some(ShellConfiguration
            {
                setup: Some(r#"autoload -Uz promptinit \
promptinit \
prompt transient \
setopt transient_rprompt \
BURNOUT=$(command -v burnout zsh); \
PROMPT="$($BURNOUT)"; \
RPROMPT="$($BURNOUT right)"; \
PROMPT_TRANSIENT="$($BURNOUT transient)"; \
RPROMPT_TRANSIENT="$($BURNOUT right_transient)"; \
PS2="$($BURNOUT continuation)" \
precmd_functions+=(set_window_title); \
function set_window_title(){{echo -ne "\033]0;$($BURNOUT window_title)\007"}}"#.to_string())
            }),
            git_bash: Some(ShellConfiguration
            {
                setup: Some(r#"BURNOUT=$(command -v burnout.exe git_bash); \
PS1=$($BURNOUT); \
RPROMPT="$($BURNOUT right)"; \
PS1_TRANSIENT="$($BURNOUT transient)"; \
RPROMPT_TRANSIENT="$($BURNOUT right_transient)"; \
PS2="$($BURNOUT continuation)" \
PROMPT_COMMAND='echo -ne "\033]0;$($BURNOUT window_title)\007"'"#.to_string())
            }),
            powershell: Some(ShellConfiguration
            {
                setup: Some(r#"[Console]::OutputEncoding = [System.Text.Encoding]::UTF8; $burnout = (Get-Command burnout).Source; $PSContinuationPrompt = { & $burnout powershell continuation }; function global:prompt { $host.UI.RawUI.WindowTitle = (& $burnout powershell window_title); return & $burnout powershell;}"#.to_string())
            }),
            cmd: Some(ShellConfiguration
            {
                setup: Some(r#"if (clink.version_encoded or 0) < 10020030 then
error("Burnout requires Clink v1.2.30 or later.")
end
local prompt = clink.promptfilter(1)
function prompt:filter(prompt)
set_title(prompt)
return io.popen("burnout cmd"):read("*a")
end
function prompt:rightfilter(prompt)
return io.popen("burnout cmd right"):read("*a")
end
function prompt:transientfilter(prompt)
return io.popen("burnout cmd transient"):read("*a")
end
function prompt:transientrightfilter(prompt)
return io.popen("burnout cmd right_transient"):read("*a")
end
function set_title(prompt)
local title = io.popen("burnout cmd window_title"):read("*a")
if title ~= nil then
console.settitle(title)
end
end"#.to_string())
            })
        }
    }
}

/**
 * Unit tests for the base configuration.
 */
#[cfg(test)]
mod tests
{
    use super::BaseConfiguration;

    /**
     * Tests the default values for the configuration are correct.
     */
    #[test]
    fn test_default_values_are_correct()
    {
        let configuration = BaseConfiguration::default();

        assert!(configuration.bash.is_some());
        assert!(configuration.zsh.is_some());
        assert!(configuration.git_bash.is_some());
        assert!(configuration.powershell.is_some());
        assert!(configuration.cmd.is_some());
    }

    /**
     * Tests the deserialisation of an empty configuration as YAML.
     */
    #[test]
    fn test_yaml_deserialisation_empty_configuration()
    {
        let configuration: BaseConfiguration = yaml_serde::from_str(String::default().as_str()).expect("Failed to deserialise the empty configuration.");

        assert!(configuration.bash.is_none());
        assert!(configuration.zsh.is_none());
        assert!(configuration.git_bash.is_none());
        assert!(configuration.powershell.is_none());
        assert!(configuration.cmd.is_none());
    }

    /**
     * Tests the deserialisation of a partial configuration as YAML.
     */
    #[test]
    fn test_yaml_deserialisation_partial_configuration()
    {
        let configuration: BaseConfiguration = yaml_serde::from_str(r#"bash:
  setup: |-
    BURNOUT=$(command -v burnout bash); \
    PS1="$($BURNOUT)"; \
    RPROMPT="$($BURNOUT right)"; \
    PS1_TRANSIENT="$($BURNOUT transient)"; \
    RPROMPT_TRANSIENT="$($BURNOUT right_transient)"; \
    PS2="$($BURNOUT continuation)" \
    PROMPT_COMMAND='echo -ne "\033]0;$($BURNOUT window_title)\007"'"#).expect("Failed to deserialise the partial configuration.");

        assert_eq!(configuration.bash.unwrap().setup, Some(r#"BURNOUT=$(command -v burnout bash); \
PS1="$($BURNOUT)"; \
RPROMPT="$($BURNOUT right)"; \
PS1_TRANSIENT="$($BURNOUT transient)"; \
RPROMPT_TRANSIENT="$($BURNOUT right_transient)"; \
PS2="$($BURNOUT continuation)" \
PROMPT_COMMAND='echo -ne "\033]0;$($BURNOUT window_title)\007"'"#.to_string()));
        assert!(configuration.zsh.is_none());
        assert!(configuration.git_bash.is_none());
        assert!(configuration.powershell.is_none());
        assert!(configuration.cmd.is_none());
    }

    /**
     * Tests the deserialisation of a full configuration as YAML.
     */
    #[test]
    fn test_yaml_deserialisation_full_configuration()
    {
        let configuration: BaseConfiguration = yaml_serde::from_str(r#"bash:
  setup: |-
    BURNOUT=$(command -v burnout bash); \
    PS1="$($BURNOUT)"; \
    RPROMPT="$($BURNOUT right)"; \
    PS1_TRANSIENT="$($BURNOUT transient)"; \
    RPROMPT_TRANSIENT="$($BURNOUT right_transient)"; \
    PS2="$($BURNOUT continuation)" \
    PROMPT_COMMAND='echo -ne "\033]0;$($BURNOUT window_title)\007"'
zsh:
  setup: |-
    autoload -Uz promptinit \
    promptinit \
    prompt transient \
    setopt transient_rprompt \
    BURNOUT=$(command -v burnout zsh); \
    PROMPT="$($BURNOUT)"; \
    RPROMPT="$($BURNOUT right)"; \
    PROMPT_TRANSIENT="$($BURNOUT transient)"; \
    RPROMPT_TRANSIENT="$($BURNOUT right_transient)"; \
    PS2="$($BURNOUT continuation)" \
    precmd_functions+=(set_window_title); \
    function set_window_title(){{echo -ne "\033]0;$($BURNOUT window_title)\007"}}
git_bash:
  setup: |-
    BURNOUT=$(command -v burnout.exe git_bash); \
    PS1=$($BURNOUT); \
    RPROMPT="$($BURNOUT right)"; \
    PS1_TRANSIENT="$($BURNOUT transient)"; \
    RPROMPT_TRANSIENT="$($BURNOUT right_transient)"; \
    PS2="$($BURNOUT continuation)" \
    PROMPT_COMMAND='echo -ne "\033]0;$($BURNOUT window_title)\007"'
powershell:
  setup: '[Console]::OutputEncoding = [System.Text.Encoding]::UTF8; $burnout = (Get-Command burnout).Source; $PSContinuationPrompt = { & $burnout powershell continuation }; function global:prompt { $host.UI.RawUI.WindowTitle = (& $burnout powershell window_title); return & $burnout powershell;}'
cmd:
  setup: |-
    if (clink.version_encoded or 0) < 10020030 then
    error("Burnout requires Clink v1.2.30 or later.")
    end
    local prompt = clink.promptfilter(1)
    function prompt:filter(prompt)
    set_title(prompt)
    return io.popen("burnout cmd"):read("*a")
    end
    function prompt:rightfilter(prompt)
    return io.popen("burnout cmd right"):read("*a")
    end
    function prompt:transientfilter(prompt)
    return io.popen("burnout cmd transient"):read("*a")
    end
    function prompt:transientrightfilter(prompt)
    return io.popen("burnout cmd right_transient"):read("*a")
    end
    function set_title(prompt)
    local title = io.popen("burnout cmd window_title"):read("*a")
    if title ~= nil then
    console.settitle(title)
    end
    end
"#).expect("Failed to parse valid YAML.");

        assert_eq!(configuration.bash.unwrap().setup, Some(r#"BURNOUT=$(command -v burnout bash); \
PS1="$($BURNOUT)"; \
RPROMPT="$($BURNOUT right)"; \
PS1_TRANSIENT="$($BURNOUT transient)"; \
RPROMPT_TRANSIENT="$($BURNOUT right_transient)"; \
PS2="$($BURNOUT continuation)" \
PROMPT_COMMAND='echo -ne "\033]0;$($BURNOUT window_title)\007"'"#.to_string()));
        assert_eq!(configuration.zsh.unwrap().setup, Some(r#"autoload -Uz promptinit \
promptinit \
prompt transient \
setopt transient_rprompt \
BURNOUT=$(command -v burnout zsh); \
PROMPT="$($BURNOUT)"; \
RPROMPT="$($BURNOUT right)"; \
PROMPT_TRANSIENT="$($BURNOUT transient)"; \
RPROMPT_TRANSIENT="$($BURNOUT right_transient)"; \
PS2="$($BURNOUT continuation)" \
precmd_functions+=(set_window_title); \
function set_window_title(){{echo -ne "\033]0;$($BURNOUT window_title)\007"}}"#.to_string()));
        assert_eq!(configuration.git_bash.unwrap().setup, Some(r#"BURNOUT=$(command -v burnout.exe git_bash); \
PS1=$($BURNOUT); \
RPROMPT="$($BURNOUT right)"; \
PS1_TRANSIENT="$($BURNOUT transient)"; \
RPROMPT_TRANSIENT="$($BURNOUT right_transient)"; \
PS2="$($BURNOUT continuation)" \
PROMPT_COMMAND='echo -ne "\033]0;$($BURNOUT window_title)\007"'"#.to_string()));
        assert_eq!(configuration.powershell.unwrap().setup, Some(r#"[Console]::OutputEncoding = [System.Text.Encoding]::UTF8; $burnout = (Get-Command burnout).Source; $PSContinuationPrompt = { & $burnout powershell continuation }; function global:prompt { $host.UI.RawUI.WindowTitle = (& $burnout powershell window_title); return & $burnout powershell;}"#.to_string()));
        assert_eq!(configuration.cmd.unwrap().setup, Some(r#"if (clink.version_encoded or 0) < 10020030 then
error("Burnout requires Clink v1.2.30 or later.")
end
local prompt = clink.promptfilter(1)
function prompt:filter(prompt)
set_title(prompt)
return io.popen("burnout cmd"):read("*a")
end
function prompt:rightfilter(prompt)
return io.popen("burnout cmd right"):read("*a")
end
function prompt:transientfilter(prompt)
return io.popen("burnout cmd transient"):read("*a")
end
function prompt:transientrightfilter(prompt)
return io.popen("burnout cmd right_transient"):read("*a")
end
function set_title(prompt)
local title = io.popen("burnout cmd window_title"):read("*a")
if title ~= nil then
console.settitle(title)
end
end"#.to_string()));
    }

    /**
     * Tests the serialisation and deserialisation of the default configuration as YAML.
     */
    #[test]
    fn test_yaml_serialisation_and_deserialisation_default_configuration()
    {
        let configuration = BaseConfiguration::default();
        let deserialised: BaseConfiguration = yaml_serde::from_str(&yaml_serde::to_string(&configuration).expect("Failed to serialise the base configuration.")).expect("Failed to deserialise the base configuration.");

        assert_eq!(configuration.bash.unwrap().setup, deserialised.bash.unwrap().setup);
        assert_eq!(configuration.zsh.unwrap().setup, deserialised.zsh.unwrap().setup);
        assert_eq!(configuration.git_bash.unwrap().setup, deserialised.git_bash.unwrap().setup);
        assert_eq!(configuration.powershell.unwrap().setup, deserialised.powershell.unwrap().setup);
        assert_eq!(configuration.cmd.unwrap().setup, deserialised.cmd.unwrap().setup);
    }
}