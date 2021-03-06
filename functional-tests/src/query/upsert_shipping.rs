use failure::Error as FailureError;
use graphql_client::GraphQLQuery;
use graphql_client::Response;

use request::GraphqlRequest;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/upsert_shipping.graphql",
    response_derives = "Debug, Clone"
)]
pub struct UpsertShippingMutation;

pub use self::upsert_shipping_mutation::*;

pub type GraphqlRequestOutput = RustUpsertShippingUpsertShipping;

pub fn default_upsert_shipping_input() -> NewShippingInput {
    NewShippingInput {
        client_mutation_id: "".to_string(),
        base_product_id: 0,
        store_id: 0,
        pickup: Some(default_new_pickups_input()),
        local: vec![],
        international: vec![],
    }
}

pub fn default_new_pickups_input() -> NewPickupsInput {
    NewPickupsInput {
        pickup: false,
        price: None,
    }
}

pub fn default_new_local_shipping_products_input() -> NewLocalShippingProductsInput {
    NewLocalShippingProductsInput {
        company_package_id: 0,
        price: None,
    }
}

pub fn default_new_international_shipping_products_input() -> NewInternationalShippingProductsInput
{
    NewInternationalShippingProductsInput {
        company_package_id: 0,
        price: None,
        deliveries_to: vec![],
    }
}

impl GraphqlRequest for NewShippingInput {
    type Output = GraphqlRequestOutput;

    fn response(body: serde_json::Value) -> Result<GraphqlRequestOutput, FailureError> {
        let response_body: Response<ResponseData> = serde_json::from_value(body)?;
        match (response_body.data, response_body.errors) {
            (Some(data), None) => Ok(data.upsert_shipping),
            (None, Some(errors)) => Err(::failure::format_err!("{:?}", errors)),
            _ => unreachable!(),
        }
    }
}

impl From<NewShippingInput> for serde_json::Value {
    fn from(val: NewShippingInput) -> serde_json::Value {
        let request_body = UpsertShippingMutation::build_query(Variables { input: val });
        serde_json::to_value(request_body).expect("failed to serialize UpsertShippingInput")
    }
}
