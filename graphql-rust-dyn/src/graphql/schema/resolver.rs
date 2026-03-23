use async_graphql::dynamic::{FieldFuture, FieldValue, ResolverContext};
use serde_json::Value;

pub fn default_resolver(ctx: ResolverContext<'_>) -> FieldFuture<'_> {
    FieldFuture::new(async move {
        let field_name = ctx.field().name().to_string();

        // Use downcast_ref directly on the FieldValue reference
        let parent_json = ctx
            .parent_value
            .downcast_ref::<Value>() // This looks for data stored via owned_any/borrowed_any
            .ok_or_else(|| {
                format!(
                    "Field '{}' expected a JSON parent but found none",
                    field_name
                )
            })?;

        // Extract the value from the JSON
        let val = parent_json.get(&field_name).cloned().unwrap_or(Value::Null);

        match val {
            Value::Object(_) => Ok(Some(FieldValue::owned_any(val))),
            Value::Array(arr) => Ok(Some(FieldValue::list(
                arr.into_iter().map(FieldValue::owned_any),
            ))),
            Value::Null => Ok(None),
            _ => {
                // Use the value! macro to convert JSON scalar to GQL scalar
                Ok(Some(FieldValue::value(async_graphql::value!(val))))
            }
        }
    })
}
