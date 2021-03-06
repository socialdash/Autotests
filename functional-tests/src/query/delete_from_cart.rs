use failure::Error as FailureError;
use graphql_client::GraphQLQuery;
use graphql_client::Response;

use request::GraphqlRequest;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/delete_from_cart.graphql"
)]
pub struct DeleteFromCartMutation;

pub use self::delete_from_cart_mutation::*;

pub fn default_delete_from_cart_input() -> DeleteFromCartInput {
    DeleteFromCartInput {
        client_mutation_id: "".to_string(),
        product_id: 0,
    }
}

type GraphqlRequestOutput = RustDeleteFromCartDeleteFromCart;

impl GraphqlRequest for DeleteFromCartInput {
    type Output = GraphqlRequestOutput;

    fn response(body: serde_json::Value) -> Result<GraphqlRequestOutput, FailureError> {
        let response_body: Response<ResponseData> = serde_json::from_value(body)?;
        match (response_body.data, response_body.errors) {
            (Some(data), None) => Ok(data.delete_from_cart),
            (_, Some(errors)) => Err(::failure::format_err!("{:?}", errors)),
            _ => unreachable!(),
        }
    }
}

impl From<DeleteFromCartInput> for serde_json::Value {
    fn from(val: DeleteFromCartInput) -> serde_json::Value {
        let request_body = DeleteFromCartMutation::build_query(Variables { input: val });
        serde_json::to_value(request_body).expect("failed to serialize DeleteFromCartInput")
    }
}
