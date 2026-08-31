//! Handles PowerShell specific functionality.
use super::Shell;

pub struct PowerShell;

/**
 * Implements the [Shell] trait for PowerShell.
 */
impl Shell for PowerShell
{
    /**
     * Prints the prompt initialisation code for PowerShell.
     * By printing the initialisation, it sets the prompt for that session.
     */
    fn print_initialisation()
    {
        println!(r#"[Console]::OutputEncoding = [System.Text.Encoding]::UTF8; function global:prompt {{ $host.UI.RawUI.WindowTitle = (& '$HOME\Bin\burnout' 'window-title'); return & '$HOME\Bin\burnout';}}"#);
    }
}