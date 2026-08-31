//! Handles Cmd specific functionality.
use super::Shell;

pub struct Cmd;

/**
 * Implements the [Shell] trait for Cmd.
 */
impl Shell for Cmd
{
    /**
     * Prints the prompt initialisation code for Cmd.
     * By printing the initialisation, it sets the prompt for that session.
     */
    fn print_initialisation()
    {
        println!(r#"if (clink.version_encoded or 0) < 10020030 then
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
        end"#);
    }
}