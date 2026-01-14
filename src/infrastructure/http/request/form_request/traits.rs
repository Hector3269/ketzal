use super::errors::ValidationErrors;
use serde_json::Value;
use std::collections::HashMap;

pub trait FormRequest: Sized {
    fn rules(&self) -> HashMap<&'static str, &'static str>;

    fn messages(&self) -> HashMap<&'static str, &'static str> {
        HashMap::new()
    }

    fn attributes(&self) -> HashMap<&'static str, &'static str> {
        HashMap::new()
    }

    fn prepare_for_validation(&self, _data: &mut HashMap<String, Value>) {}

    fn with_validator(&self, _validator: &mut super::validator::Validator) {}

    fn validate_data(
        data: HashMap<String, Value>,
    ) -> Result<HashMap<String, Value>, ValidationErrors>
    where
        Self: Default,
    {
        let instance = Self::default();

        let mut mutable_data = data;
        instance.prepare_for_validation(&mut mutable_data);

        let mut validator =
            super::validator::Validator::make(mutable_data.clone(), instance.rules());
        validator.set_custom_messages(instance.messages());
        validator.set_custom_attributes(instance.attributes());

        instance.with_validator(&mut validator);

        validator.validate()?;

        Ok(validator.validated_data())
    }
}
