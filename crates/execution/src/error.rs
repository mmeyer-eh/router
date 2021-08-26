use crate::GraphQLResponse;
use apollo_json_ext::prelude::*;
use displaydoc::Display;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Error types for execution. Note that these are not actually returned to the client, but are
/// instead converted to Json for GraphQLError
#[derive(Error, Display, Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[ignore_extra_doc_attributes]
pub enum FetchError {
    /// Query references unknown service '{service}'.
    ValidationUnknownServiceError {
        /// The service that was unknown.
        service: String,
    },

    /// Query requires variable '{name}', but it was not provided.
    ValidationMissingVariable {
        /// Name of the variable.
        name: String,
    },

    /// Query could not be planned: {reason}
    ValidationPlanningError {
        /// The failure reason.
        reason: String,
    },

    /// Response was malformed: {reason}
    MalformedResponse {
        /// The reason the serialization failed.
        reason: String,
    },

    /// Service '{service}' returned no response.
    SubrequestNoResponse {
        /// The service that returned no response.
        service: String,
    },

    /// Service '{service}' response was malformed: {reason}
    SubrequestMalformedResponse {
        /// The service that responded with the malformed response.
        service: String,

        /// The reason the serialization failed.
        reason: String,
    },

    /// Service '{service}' returned a PATCH response which was not expected.
    SubrequestUnexpectedPatchResponse {
        /// The service that returned the PATCH response.
        service: String,
    },

    /// HTTP fetch failed from '{service}': {reason}
    ///
    /// Note that this relates to a transport error and not a GraphQL error.
    SubrequestHttpError {
        /// The service failed.
        service: String,

        /// The reason the fetch failed.
        reason: String,
    },

    /// Subquery requires field '{field}' but it was not found in the current response.
    ExecutionFieldNotFound {
        /// The field that is not found.
        field: String,
    },

    /// Invalid content: {reason}
    ExecutionInvalidContent { reason: String },

    /// Could find path: {reason}
    ExecutionPathNotFound { reason: String },
}

impl FetchError {
    /// Convert the fetch error to a GraphQL error.
    pub fn to_graphql_error(&self, path: Option<Path>) -> GraphQLError {
        GraphQLError {
            message: self.to_string(),
            locations: Default::default(),
            path: path.unwrap_or_default(),
            extensions: serde_json::to_value(self)
                .unwrap()
                .as_object()
                .unwrap()
                .to_owned(),
        }
    }

    /// Convert the error to an appropriate response.
    pub fn to_response(&self, primary: bool) -> GraphQLResponse {
        GraphQLResponse {
            label: Default::default(),
            data: Default::default(),
            path: Default::default(),
            has_next: primary.then(|| false),
            errors: vec![self.to_graphql_error(None)],
            extensions: Default::default(),
        }
    }
}

/// {message}
#[derive(Error, Display, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphQLError {
    /// The error message.
    pub message: String,

    /// The locations of the error from the originating request.
    pub locations: Vec<Location>,

    /// The path of the error.
    pub path: Path,

    /// The optional graphql extensions.
    #[serde(default, skip_serializing_if = "Object::is_empty")]
    pub extensions: Object,
}

/// A location in the request that triggered a graphql error.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    /// The line number.
    pub line: i32,

    /// The column number.
    pub column: i32,
}
