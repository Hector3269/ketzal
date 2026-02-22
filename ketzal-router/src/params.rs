//! Params module
//! 
//! Provides the [`Params`] struct for extracting and storing route parameters.

use indexmap::IndexMap;

/// A collection of route parameters extracted from the URL path.
///
/// # Example
///
/// ```ignore
/// use ketzal_router::Params;
///
/// let mut params = Params::new();
/// params.insert("id", "42");
/// params.insert("name", "John");
///
/// // Get a specific parameter
/// let id = params.get("id"); // Some("42")
/// 
/// // Get all parameters
/// for (key, value) in params.all() {
///     println!("{}: {}", key, value);
/// }
/// ```
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Params {
    inner: IndexMap<String, String>,
}

impl Params {
    /// Creates a new empty Params collection.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let params = Params::new();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts a parameter into the collection.
    ///
    /// # Arguments
    ///
    /// * `key` - The parameter name
    /// * `value` - The parameter value
    ///
    /// # Example
    ///
    /// ```ignore
    /// let mut params = Params::new();
    /// params.insert("id", "42");
    /// ```
    pub fn insert(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.inner.insert(key.into(), value.into());
    }

    /// Gets a parameter value by name.
    ///
    /// # Arguments
    ///
    /// * `key` - The parameter name to look up
    ///
    /// # Returns
    ///
    /// Returns `Some(&String)` if the parameter exists, or `None` otherwise.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let id = params.get("id");
    /// ```
    pub fn get(&self, key: &str) -> Option<&String> {
        self.inner.get(key)
    }

    /// Returns all parameters as an ordered map.
    ///
    /// # Returns
    ///
    /// A reference to the underlying [`IndexMap<String, String>`].
    ///
    /// # Example
    ///
    /// ```ignore
    /// for (key, value) in params.all() {
    ///     println!("{}: {}", key, value);
    /// }
    /// ```
    pub fn all(&self) -> &IndexMap<String, String> {
        &self.inner
    }
}

/// Matches a path pattern against an actual path and extracts parameters.
///
/// Path patterns use the `:param_name` syntax for dynamic segments.
///
/// # Arguments
///
/// * `pattern` - The path pattern (e.g., "/users/:id/posts/:post_id")
/// * `actual` - The actual request path (e.g., "/users/42/posts/100")
///
/// # Returns
///
/// Returns `Some(Params)` with extracted parameters if the path matches,
/// or `None` if the path doesn't match the pattern.
///
/// # Example
///
/// ```ignore
/// // Extract single parameter
/// let params = match_path("/users/:id", "/users/42");
/// // params = Some(Params with "id" -> "42")
///
/// // Extract multiple parameters
/// let params = match_path("/users/:user_id/posts/:post_id", "/users/1/posts/2");
/// // params = Some(Params with "user_id" -> "1", "post_id" -> "2")
///
/// // No match (wrong number of segments)
/// let params = match_path("/users/:id", "/users/42/posts");
/// // params = None
///
/// // No match (static segment doesn't match)
/// let params = match_path("/users/:id", "/posts/42");
/// // params = None
/// ```
pub fn match_path(pattern: &str, actual: &str) -> Option<Params> {
    let pattern_parts: Vec<&str> = pattern.trim_matches('/').split('/').collect();
    let actual_parts: Vec<&str> = actual.trim_matches('/').split('/').collect();

    if pattern_parts.len() != actual_parts.len() {
        return None;
    }

    let mut params = Params::new();

    for (p, a) in pattern_parts.iter().zip(actual_parts.iter()) {
        if let Some(key) = p.strip_prefix(':') {
            if key.is_empty() {
                return None;
            }
            params.insert(key, *a);
        } else if p != a {
            return None;
        }
    }

    Some(params)
}
