/*!
 * Defines the implementing struct as a display element.
 */
use super::super::configuration::core_configuration::base_configuration::BaseConfiguration;

/// Defines the implementing struct as a display element.
pub trait Display
{
    /// Renders the value for the display.
    /// # Arguments
    /// * `configuration` - The configuration with the display values.
    /// * `shell` - The target shell to render to.
    /// # Returns
    /// * `Ok(())` - Returns that the rendering was successful.
    /// * `Err(String)` - Returns that the rendering failed.
    fn render_display(configuration: BaseConfiguration, shell: &str) -> Result<(), String>;
}