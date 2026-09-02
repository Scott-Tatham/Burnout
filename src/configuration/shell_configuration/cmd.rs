/*!
 * Stores the Cmd shell configuration.
 */
use serde::{Serialize, Deserialize};

/**
 * Stores the Cmd shell configuration.
 */
#[derive(Serialize, Deserialize)]
pub struct CmdConfiguration
{
    pub setup: Option<String>
}

/**
 * Implements the [Default] trait for the Cmd shell configuration.
 */
impl Default for CmdConfiguration
{
    fn default() -> Self
    {
        Self
        {
            setup: Some(r#"if (clink.version_encoded or 0) < 10020030 then
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
        end"#.to_string())
        }
    }
}

/**
 * Unit tests for the Cmd shell configuration.
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
        let configuration = CmdConfiguration::default();

        assert_eq!(configuration.setup, Some(r#"if (clink.version_encoded or 0) < 10020030 then
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
        end"#.to_string()));
    }

    /**
     * Tests the serialisation and deserialisation of the full Cmd shell configuration as TOML.
     */
    #[test]
    fn test_toml_serialisation_and_deserialisation_full_configuration()
    {
        let configuration = CmdConfiguration
        {
            setup: Some(r#"
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
        end"#.to_string())
        };

        let deserialised: CmdConfiguration = toml::from_str(&toml::to_string(&configuration).expect("Failed to serialise the full configuration.")).expect("Failed to deserialise the full configuration.");

        assert_eq!(configuration.setup, deserialised.setup);
    }

    /**
     * Tests the serialisation and deserialisation of an empty Cmd shell configuration as TOML.
     */
    #[test]
    fn test_toml_serialisation_and_deserialisation_empty_configuration()
    {
        let configuration = CmdConfiguration
        {
            setup: None
        };

        let deserialised: CmdConfiguration = toml::from_str(&toml::to_string(&configuration).expect("Failed to serialise the empty configuration.")).expect("Failed to deserialise the empty configuration.");

        assert!(deserialised.setup.is_none());
    }
}