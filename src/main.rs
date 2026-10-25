/*!
 * The entry point for Burnout.
 */
pub mod configuration;
pub mod initialisation;
pub mod render;

use std::env;
use initialisation::shell;
use render::renderer;

/// The entry point for Burnout.
fn main()
{
    let arguments: Vec<String> = env::args().collect();

    if arguments.len() == 1
    {
        renderer::render_display("prompt", "bash").expect("The rendering of the display failed.");

        return;
    }

    let command = arguments.get(1).map(|value| value.as_str()).unwrap_or("prompt");
    let shell = arguments.get(2).map(|value| value.as_str()).unwrap_or("bash");

    match command
    {
        "initialise" => shell::shell_initialisation(shell).expect("The initialisation of the shell failed."),
        _ => renderer::render_display(command, shell).expect("The rendering of the display failed.")
    }
}