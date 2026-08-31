/*!
 * Handles Zsh specific functionality.
 */
use super::Shell;

pub struct Zsh;

/**
 * Implements the [Shell] trait for Zsh.
 */
impl Shell for Zsh
{
    /**
     * Prints the prompt initialisation code for Zsh.
     * By printing the initialisation, it sets the prompt for that session.
     */
    fn print_initialisation()
    {
        println!(r#"PROMPT='$(burnout)'; RPROMPT='$(burnout right)'; PROMPT_TRANSIENT='$(burnout transient)'; RPROMPT_TRANSIENT='$(burnout right-transient)'; precmd() {{print -Pn '\e]0;$(burnout window-title)\a';}}"#);
    }
}