/*!
 * Defines the implementing structure as a registry.
 */

/// Defines the implementing structure as a registry.
pub trait Registry<T>
{
    /// Add a new entry to the registry.
    /// # Arguments
    /// * `name` - The name of the entry.
    /// * `item` - The item to add to the registry.
    fn register(&mut self, name: String, item: T);

    /// Get an entry from the registry.
    /// # Arguments
    /// * `name` - The name of the entry.
    /// # Returns
    /// * `Option<&T>` - Returns the item from the registry.
    fn get(&self, name: &str) -> Option<&T>;
}