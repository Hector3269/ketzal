use serde_json::Value;
use std::collections::HashMap;

use crate::errors::ValidationErrors;
use crate::validator::Validator;

/// Trait for form request validation.
/// HTTP-agnostic - can be used in any context (CLI, tests, HTTP, etc.)
pub trait FormRequest: Sized {
    /// Returns the validation rules for this request.
    fn rules(&self) -> HashMap<&'static str, &'static str>;

    /// Returns custom validation messages.
    fn messages(&self) -> HashMap<&'static str, &'static str> {
        HashMap::new()
    }

    /// Returns custom attribute names for better error messages.
    fn attributes(&self) -> HashMap<&'static str, &'static str> {
        HashMap::new()
    }

    fn prepare_for_validation(&self, _data: &mut HashMap<String, Value>) {}

    fn with_validator(&self, _validator: &mut Validator) {}

    fn validate_data(
        data: HashMap<String, Value>,
    ) -> Result<HashMap<String, Value>, ValidationErrors>
    where
        Self: Default,
    {
        let instance = Self::default();

        let mut mutable_data = data;
        instance.prepare_for_validation(&mut mutable_data);

        let mut validator = Validator::make(mutable_data.clone(), instance.rules());
        validator.set_custom_messages(instance.messages());
        validator.set_custom_attributes(instance.attributes());

        instance.with_validator(&mut validator);

        validator.validate()?;

        Ok(validator.validated_data())
    }
}
