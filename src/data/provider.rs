/*!
 * Defines the implementing struct as a source of data.
 */
use super::context::Context;

/// Defines the implementing struct as a source of data.
pub trait Provider
{
    /// Get the [Context] related to the [Provider].
    /// # Returns
    /// * `&dyn Context` - Returns the [Context] related to the [Provider].
    fn context(&self) -> &dyn Context;
    
    /// Get whether the [Context] is populated.
    /// # Returns
    /// * `bool` - Returns whether the [Context] is populated.
    fn is_populated(&self) -> bool;

    /// Populate the [Context] with data.
    fn populate(&self);
}