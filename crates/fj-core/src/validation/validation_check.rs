use std::fmt::Display;

use super::ValidationConfig;

/// Run a specific validation check on an object
///
/// This trait is implemented once per validation check and object it applies
/// to. `Self` is the object, while `T` identifies the validation check.
pub trait ValidationCheck<T>: Sized {
    /// Run the validation check on the implementing object
    fn check(
        object: &T,
        config: &ValidationConfig,
    ) -> impl Iterator<Item = Self>;

    /// Convenience method to run the check return the first error
    ///
    /// This method is designed for convenience over flexibility (it is intended
    /// for use in unit tests), and thus always uses the default configuration.
    fn check_and_return_first_error(object: &T) -> Result<(), Self> {
        let config = ValidationConfig::default();
        let mut errors = Self::check(object, &config);

        if let Some(err) = errors.next() {
            return Err(err);
        }

        Ok(())
    }

    /// Convenience method to run the check and expect one error
    ///
    /// This method is designed for convenience over flexibility (it is intended
    /// for use in unit tests), and thus always uses the default configuration.
    fn check_and_expect_one_error(object: &T) -> Self
    where
        Self: Display,
    {
        let config = ValidationConfig::default();
        let mut errors = Self::check(object, &config).peekable();

        let err = errors
            .next()
            .expect("Expected one validation error; none found");

        if errors.peek().is_some() {
            println!("Unexpected validation errors:");

            for err in errors {
                println!("{err}");
            }

            panic!("Expected only one validation error")
        }

        err
    }
}
