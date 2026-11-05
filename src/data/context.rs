/*!
 * Defines the implementing struct as a container of data.
 */

/// Defines the implementing struct as a container of data.
pub trait Context
{
    /// Get the value of a variable with the specified name.
    /// # Arguments
    /// * `variable` - The name of the variable to retrieve.
    /// # Returns
    /// * `Option<String>` - Returns the value of the variable.
    fn get(&self, variable: &str) -> Option<String>;
}