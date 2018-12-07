#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries/create_store.graphql",
)]
pub struct CreateStoreMutation;

pub use self::create_store_mutation::*;

pub fn default_create_store_input() -> CreateStoreInput {
    CreateStoreInput {
        client_mutation_id: "".to_string(),
        user_id: 1,
        slug: "default-store".to_string(),
        cover: None,
        logo: None,
        phone: None,
        email: None,
        slogan: None,
        long_description: None,
        instagram_url: None,
        twitter_url: None,
        facebook_url: None,
        default_language: Language::EN,
        short_description: vec![TranslationInput {
            lang: Language::EN,
            text: "short_description".to_string(),
        }],
        name: vec![TranslationInput {
            lang: Language::EN,
            text: "name".to_string(),
        }],
        address_full: AddressInput {
            value: None,
            country: None,
            country_code: None,
            administrative_area_level1: None,
            administrative_area_level2: None,
            locality: None,
            political: None,
            postal_code: None,
            route: None,
            street_number: None,
            place_id: None,
        },
    }
}
