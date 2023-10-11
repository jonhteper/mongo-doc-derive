use proc_macro::TokenStream;

mod mongo_doc;

/// Generate a new struct with public fields to use like DTO for MongoDB.
/// The new struct transform field `id` to `_id` for Serde operations, and 
/// its name uses the pattern `___Doc`
/// # Example
/// ```
/// use uuid::Uuid;
/// use serde_json::json;
/// use serde_derive;
/// use std::str::FromStr;
/// use mongo_doc_derive::MongoDoc;
///
/// #[derive(Debug, MongoDoc)]
/// pub struct User {
///     id: Uuid,
///     name: String,
///     email: String,
/// }
///
/// let json_value = json! ({
///     "_id": "67e55044-10b1-426f-9247-bb680e5fe0c8",
///     "name": "John Doe",
///     "email": "john@doe.com",
/// });
///
/// let value = UserDoc {
///     id: Uuid::from_str("67e55044-10b1-426f-9247-bb680e5fe0c8").unwrap(),
///     name: "John Doe".to_string(),
///     email: "john@doe.com".to_string(),
/// };
///
/// let json_str = serde_json::to_string(&json_value).unwrap();
/// let value_from_json: UserDoc = serde_json::from_str(&json_str).unwrap();
///
///
/// assert_eq!(value_from_json, value);
///
/// ```
#[proc_macro_derive(MongoDoc)]
pub fn derive_mongo_doc(input: TokenStream) -> TokenStream {
    mongo_doc::mongo_doc_macro(input)
}
