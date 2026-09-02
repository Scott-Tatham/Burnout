/*!
 * Stores the base configuration.
 */
use super::bash::BashConfiguration;
use super::zsh::ZshConfiguration;
use super::git_bash::GitBashConfiguration;
use super::powershell::PowerShellConfiguration;
use super::cmd::CmdConfiguration;
use serde::{Deserialize, Serialize};

/**
 * Stores the base configuration.
 */
#[derive(Serialize, Deserialize)]
pub struct BaseConfiguration
{
    pub bash: Option<BashConfiguration>,
    pub zsh: Option<ZshConfiguration>,
    pub git_bash: Option<GitBashConfiguration>,
    pub powershell: Option<PowerShellConfiguration>,
    pub cmd: Option<CmdConfiguration>
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
            bash: Some(BashConfiguration::default()),
            zsh: Some(ZshConfiguration::default()),
            git_bash: Some(GitBashConfiguration::default()),
            powershell: Some(PowerShellConfiguration::default()),
            cmd: Some(CmdConfiguration::default())
        }
    }
}

/**
 * Unit tests for the base configuration.
 */
#[cfg(test)]
mod tests
{
    use super::*;

    /**
     * Tests the default values for base configuration are correct.
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
     * Tests the deserialisation of a full configuration as TOML.
     */
    #[test]
    fn test_toml_deserialisation_full_configuration()
    {
        let configuration: BaseConfiguration = toml::from_str(r#"
            [bash]
            setup = '''
BURNOUT=$(command -v burnout); \\
PS1="$($BURNOUT)"; \\
RPROMPT="$($BURNOUT right)"; \\
PS1_TRANSIENT="$($BURNOUT transient)"; \\
RPROMPT_TRANSIENT="$($BURNOUT right-transient)"; \\
PS2="$($BURNOUT continuation)" \\
PROMPT_COMMAND="echo -ne "\\033]0;$($BURNOUT window-title)\\007""
'''

            [zsh]
            setup = '''
autoload -Uz promptinit \\
promptinit \\
prompt transient \\
setopt transient_rprompt \\
BURNOUT=$(command -v burnout); \\
PROMPT="$($BURNOUT)"; \\
RPROMPT="$($BURNOUT right)"; \\
PROMPT_TRANSIENT="$($BURNOUT transient)"; \\
RPROMPT_TRANSIENT="$($BURNOUT right-transient)"; \\
PS2="$($BURNOUT continuation)" \\
precmd_functions+=(set_window_title); \\
function set_window_title(){{echo -ne "\\033]0;$($BURNOUT window-title)\\007"}}
'''

            [git_bash]
            setup = '''
BURNOUT=$(command -v burnout.exe); \\
PS1="$($BURNOUT)"; \\
RPROMPT="$($BURNOUT right)"; \\
PS1_TRANSIENT="$($BURNOUT transient)"; \\
RPROMPT_TRANSIENT="$($BURNOUT right-transient)"; \\
PS2="$($BURNOUT continuation)" \\
PROMPT_COMMAND="echo -ne "\\033]0;$($BURNOUT window-title)\\007""
'''

            [powershell]
            setup = '''[Console]::OutputEncoding = [System.Text.Encoding]::UTF8; $burnout = (Get-Command burnout).Source; $PSContinuationPrompt = { & $burnout continuation }; function global:prompt { $host.UI.RawUI.WindowTitle = (& $burnout window-title); return & $burnout;}'''

            [cmd]
            setup = '''
if (clink.version_encoded or 0) < 10020030 then
error("Burnout requires Clink v1.2.30 or later.")
end
local prompt = clink.promptfilter(1)
function prompt:filter(prompt)
set_title(prompt)
return io.popen("burnout"):read("*a")
end
function prompt:rightfilter(prompt)
return io.popen("burnout right"):read("*a")
end
function prompt:transientfilter(prompt)
return io.popen("burnout transient"):read("*a")
end
function prompt:transientrightfilter(prompt)
return io.popen("burnout right-transient"):read("*a")
end
function set_title(prompt)
local title = io.popen("burnout window-title"):read("*a")
if title ~= nil then
console.settitle(title)
end
end
'''
            "#).expect("Failed to parse valid TOML.");

        assert_eq!(configuration.bash.unwrap().setup, Some(r#"BURNOUT=$(command -v burnout); \\
PS1="$($BURNOUT)"; \\
RPROMPT="$($BURNOUT right)"; \\
PS1_TRANSIENT="$($BURNOUT transient)"; \\
RPROMPT_TRANSIENT="$($BURNOUT right-transient)"; \\
PS2="$($BURNOUT continuation)" \\
PROMPT_COMMAND="echo -ne "\\033]0;$($BURNOUT window-title)\\007""
"#.to_string()));
        assert_eq!(configuration.zsh.unwrap().setup, Some(r#"autoload -Uz promptinit \\
promptinit \\
prompt transient \\
setopt transient_rprompt \\
BURNOUT=$(command -v burnout); \\
PROMPT="$($BURNOUT)"; \\
RPROMPT="$($BURNOUT right)"; \\
PROMPT_TRANSIENT="$($BURNOUT transient)"; \\
RPROMPT_TRANSIENT="$($BURNOUT right-transient)"; \\
PS2="$($BURNOUT continuation)" \\
precmd_functions+=(set_window_title); \\
function set_window_title(){{echo -ne "\\033]0;$($BURNOUT window-title)\\007"}}
"#.to_string()));
        assert_eq!(configuration.git_bash.unwrap().setup, Some(r#"BURNOUT=$(command -v burnout.exe); \\
PS1="$($BURNOUT)"; \\
RPROMPT="$($BURNOUT right)"; \\
PS1_TRANSIENT="$($BURNOUT transient)"; \\
RPROMPT_TRANSIENT="$($BURNOUT right-transient)"; \\
PS2="$($BURNOUT continuation)" \\
PROMPT_COMMAND="echo -ne "\\033]0;$($BURNOUT window-title)\\007""
"#.to_string()));
        assert_eq!(configuration.powershell.unwrap().setup, Some(r#"[Console]::OutputEncoding = [System.Text.Encoding]::UTF8; $burnout = (Get-Command burnout).Source; $PSContinuationPrompt = { & $burnout continuation }; function global:prompt { $host.UI.RawUI.WindowTitle = (& $burnout window-title); return & $burnout;}"#.to_string()));
        assert_eq!(configuration.cmd.unwrap().setup, Some(r#"if (clink.version_encoded or 0) < 10020030 then
error("Burnout requires Clink v1.2.30 or later.")
end
local prompt = clink.promptfilter(1)
function prompt:filter(prompt)
set_title(prompt)
return io.popen("burnout"):read("*a")
end
function prompt:rightfilter(prompt)
return io.popen("burnout right"):read("*a")
end
function prompt:transientfilter(prompt)
return io.popen("burnout transient"):read("*a")
end
function prompt:transientrightfilter(prompt)
return io.popen("burnout right-transient"):read("*a")
end
function set_title(prompt)
local title = io.popen("burnout window-title"):read("*a")
if title ~= nil then
console.settitle(title)
end
end
"#.to_string()));
    }

    /**
     * Tests the deserialisation of a partial configuration as TOML.
     */
    #[test]
    fn test_toml_deserialisation_partial_configuration()
    {
        let toml_string = r#"
            [bash]
            setup = '''
BURNOUT=$(command -v burnout); \\
PS1="$($BURNOUT)"; \\
RPROMPT="$($BURNOUT right)"; \\
PS1_TRANSIENT="$($BURNOUT transient)"; \\
RPROMPT_TRANSIENT="$($BURNOUT right-transient)"; \\
PS2="$($BURNOUT continuation)" \\
PROMPT_COMMAND="echo -ne "\\033]0;$($BURNOUT window-title)\\007""
'''
            "#;

        let configuration: BaseConfiguration = toml::from_str(toml_string).expect("Failed to deserialise the partial configuration.");

        assert_eq!(configuration.bash.unwrap().setup, Some(r#"BURNOUT=$(command -v burnout); \\
PS1="$($BURNOUT)"; \\
RPROMPT="$($BURNOUT right)"; \\
PS1_TRANSIENT="$($BURNOUT transient)"; \\
RPROMPT_TRANSIENT="$($BURNOUT right-transient)"; \\
PS2="$($BURNOUT continuation)" \\
PROMPT_COMMAND="echo -ne "\\033]0;$($BURNOUT window-title)\\007""
"#.to_string()));
        assert!(configuration.zsh.is_none());
        assert!(configuration.git_bash.is_none());
        assert!(configuration.powershell.is_none());
        assert!(configuration.cmd.is_none());
    }

    /**
     * Tests the deserialisation of an empty configuration as TOML.
     */
    #[test]
    fn test_toml_deserialisation_empty_configuration()
    {
        let configuration: BaseConfiguration = toml::from_str("").expect("Failed to deserialise the empty configuration.");

        assert!(configuration.bash.is_none());
        assert!(configuration.zsh.is_none());
        assert!(configuration.git_bash.is_none());
        assert!(configuration.powershell.is_none());
        assert!(configuration.cmd.is_none());
    }

    /**
     * Tests the serialisation and deserialisation of the default configuration as TOML.
     */
    #[test]
    fn test_toml_serialisation_and_deserialisation_default_configuration()
    {
        let configuration = BaseConfiguration::default();
        let deserialised: BaseConfiguration = toml::from_str(&toml::to_string(&configuration).expect("Failed to serialise the base configuration.")).expect("Failed to deserialise the base configuration.");

        assert_eq!(configuration.bash.unwrap().setup, deserialised.bash.unwrap().setup);
        assert_eq!(configuration.zsh.unwrap().setup, deserialised.zsh.unwrap().setup);
        assert_eq!(configuration.git_bash.unwrap().setup, deserialised.git_bash.unwrap().setup);
        assert_eq!(configuration.powershell.unwrap().setup, deserialised.powershell.unwrap().setup);
        assert_eq!(configuration.cmd.unwrap().setup, deserialised.cmd.unwrap().setup);
    }
}