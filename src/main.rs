/*!
 * The entry point for Burnout.
 */
use std::env;

pub mod configuration;
pub mod initialisation;
pub mod render;

fn main()
{
    let arguments: Vec<String> = env::args().collect();
    let argument = arguments.get(1).map(|value| value.as_str());

    match argument
    {
        Some("initialise") => initialisation::shell_initialisation(arguments.get(2).map(|value| value.as_str()).unwrap_or("bash")),
        _ => render::render_display(argument.unwrap_or("prompt"))
    }
}