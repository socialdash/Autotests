extern crate failure;
extern crate functional_tests;

use failure::Error as FailureError;

use functional_tests::query::*;

use functional_tests::context::TestContext;

#[test]
pub fn verify_email() {
    //setup
    let mut context = TestContext::new();
    //given
    let user = context
        .request(create_user::default_create_user_input())
        .expect("createUser failed");
    let verification_token = context
        .get_email_verification_token(&user.email)
        .expect("context.get_email_verification_token");
    //when
    let verification = context
        .verify_email(verify_email::VerifyEmailApply {
            token: verification_token.clone(),
            ..verify_email::default_verify_email_input()
        })
        .expect("context.verify_email failed")
        .verify_email;
    //then
    assert_eq!(verification.success, true);
    assert_eq!(verification.email, user.email);
    context.set_bearer(verification.token);
    //only verified user can create store
    let store = context
        .request(create_store::CreateStoreInput {
            user_id: user.raw_id,
            ..create_store::default_create_store_input()
        })
        .expect("create_store failed");
    assert_eq!(store.user_id, user.raw_id);
}

#[test]
pub fn deactivate_user() {
    //setup
    let mut context = TestContext::new();
    //given
    let user = context
        .request(create_user::default_create_user_input())
        .expect("createUser failed");
    //when
    context.as_superadmin();
    let deactivated_user = context
        .request(deactivate_user::DeactivateUserInput {
            id: user.id,
            ..deactivate_user::default_deactivate_user_input()
        })
        .expect("deactivate_user failed");
    //then
    assert_eq!(deactivated_user.is_active, false);
}

#[test]
pub fn update_user() {
    //setup
    let mut context = TestContext::new();
    //given
    let user = context
        .request(create_user::default_create_user_input())
        .expect("createUser failed");
    context.verify_user_email(&user.email).unwrap();
    let token: String = context
        .get_jwt_by_email(get_jwt_by_email::default_create_jwt_email_input())
        .unwrap()
        .get_jwt_by_email
        .token;
    context.set_bearer(token);
    //when
    let updated_user = context
        .update_user(update_user::UpdateUserInput {
            id: user.id,
            is_active: Some(false),
            ..update_user::default_update_user_input()
        })
        .unwrap()
        .update_user;
    //then
    let expected_values = update_user::default_update_user_input();
    assert_eq!(updated_user.is_active, false);
    assert_eq!(
        updated_user.phone.expect("updated_user.phone is none"),
        expected_values.phone.unwrap()
    );
    assert_eq!(
        updated_user
            .first_name
            .expect("updated_user.first_name is none"),
        expected_values.first_name.unwrap()
    );
    assert_eq!(
        updated_user
            .last_name
            .expect("updated_user.last_name is none"),
        expected_values.last_name.unwrap()
    );
    assert_eq!(
        updated_user
            .middle_name
            .expect("updated_user.middle_name is none"),
        expected_values.middle_name.unwrap()
    );
    assert_eq!(
        updated_user
            .birthdate
            .expect("updated_user.birthdate is none"),
        expected_values.birthdate.unwrap()
    );
    assert_eq!(
        updated_user.avatar.expect("updated_user.avatar is none"),
        expected_values.avatar.unwrap()
    );
}

#[test]
pub fn deactivate_base_product() {
    //setup
    let mut context = TestContext::new();
    //given
    let (_user, token, _store, _category, base_product) =
        set_up_base_product(&mut context).expect("Cannot get data from set_up_base_product");
    context.set_bearer(token);
    //when
    let _ = context
        .request(deactivate_base_product::DeactivateBaseProductInput {
            id: base_product.id,
            ..deactivate_base_product::default_deactivate_base_product_input()
        })
        .expect("deactivate_base_product failed");
    //then
    let deactivated_base_product = context
        .get_base_product(base_product.raw_id)
        .unwrap()
        .base_product;
    assert!(deactivated_base_product.is_none());
}

#[test]
pub fn update_base_product_test() {
    //setup
    let mut context = TestContext::new();
    //given
    let (_user, token, _store, _category, base_product) =
        set_up_base_product(&mut context).expect("Cannot get data from set_up_base_product");
    context.set_bearer(token);
    //when
    let updated_base_product = context
        .update_base_product(update_base_product::UpdateBaseProductInput {
            id: base_product.id,
            length_cm: Some(20),
            width_cm: Some(30),
            height_cm: Some(40),
            weight_g: Some(2000),
            ..update_base_product::default_update_base_product_input()
        })
        .unwrap()
        .update_base_product;
    //then
    let updated_base_product = context
        .get_base_product(updated_base_product.raw_id)
        .unwrap()
        .base_product
        .unwrap();
    let expected_values = update_base_product::default_update_base_product_input();
    assert!((updated_base_product.rating - expected_values.rating.unwrap()).abs() < 0.001);
    assert_eq!(updated_base_product.slug, expected_values.slug.unwrap());
    assert_eq!(updated_base_product.length_cm, Some(20));
    assert_eq!(updated_base_product.width_cm, Some(30));
    assert_eq!(updated_base_product.height_cm, Some(40));
    assert_eq!(updated_base_product.weight_g, Some(2000));
    assert_eq!(
        updated_base_product.name[0].text,
        expected_values.name.unwrap()[0].text
    );
    assert_eq!(
        updated_base_product.short_description[0].text,
        expected_values.short_description.unwrap()[0].text
    );
    assert_eq!(
        updated_base_product
            .long_description
            .expect("updated_base_product.long_description is none")[0]
            .text,
        expected_values.long_description.unwrap()[0].text
    );
    assert_eq!(
        updated_base_product
            .seo_title
            .expect("updated_base_product.seo_title is none")[0]
            .text,
        expected_values.seo_title.unwrap()[0].text
    );
    assert_eq!(
        updated_base_product
            .seo_description
            .expect("updated_base_product.seo_description is none")[0]
            .text,
        expected_values.seo_description.unwrap()[0].text
    );
}

#[test]
#[ignore]
pub fn update_base_product_does_not_update_rating() {
    //setup
    let mut context = TestContext::new();
    //given
    let (_user, token, _store, _category, base_product) =
        set_up_base_product(&mut context).expect("Cannot get data from set_up_base_product");
    context.set_bearer(token);
    let initial_rating = base_product.rating;
    //when
    let updated_base_product = context
        .update_base_product(update_base_product::UpdateBaseProductInput {
            id: base_product.id,
            ..update_base_product::default_update_base_product_input()
        })
        .unwrap()
        .update_base_product;
    //then
    let updated_base_product = context
        .get_base_product(updated_base_product.raw_id)
        .unwrap()
        .base_product
        .unwrap();
    assert!((updated_base_product.rating - initial_rating).abs() < 0.001);
}

#[test]
pub fn create_base_product_with_variants() {
    //setup
    let mut context = TestContext::new();
    //given
    let (_user, token, store, category) = set_up_store(&mut context).unwrap();
    context.set_bearer(token);
    //when
    let base_product = context.request(create_base_product_with_variants::NewBaseProductWithVariantsInput{
        store_id: store.raw_id,
        category_id: category.raw_id,
        ..create_base_product_with_variants::default_create_base_product_with_variants_input()
    }).expect("create_base_product_with_variants failed");
    //then
    let base_product = context
        .get_base_product(base_product.raw_id)
        .unwrap()
        .base_product
        .unwrap();
    assert_eq!(
        base_product
            .products
            .as_ref()
            .expect("base_product.products is none")
            .edges
            .len(),
        1
    );
    let variant = base_product.products.unwrap().edges.pop().unwrap().node;
    assert_eq!(variant.discount, Some(30.0));
    assert_eq!(variant.photo_main, Some("photo".to_string()));
    assert_eq!(variant.vendor_code, "vendor_code".to_string());
    assert_eq!(variant.cashback, Some(10.0));
    assert_eq!(variant.price, 100.0);
    assert_eq!(variant.pre_order, false);
    assert_eq!(variant.pre_order_days, 100);
    assert_eq!(
        variant
            .additional_photos
            .as_ref()
            .expect("variant.additional_photos is none")
            .len(),
        2
    );
    assert_eq!(
        variant
            .additional_photos
            .as_ref()
            .expect("variant.additional_photos is none")[0],
        "additional_photo_1".to_string()
    );
    assert_eq!(
        variant
            .additional_photos
            .as_ref()
            .expect("variant.additional_photos is none")[1],
        "additional_photo_2".to_string()
    );
}

#[test]
pub fn update_store() {
    //setup
    let mut context = TestContext::new();
    //given
    let (_user, token, store, _) = set_up_store(&mut context).unwrap();
    context.set_bearer(token);
    //when
    let updated_store = context
        .update_store(update_store::UpdateStoreInput {
            id: store.id,
            ..update_store::default_update_store_input()
        })
        .unwrap()
        .update_store;
    //then
    let updated_store = context
        .get_store(updated_store.raw_id)
        .unwrap()
        .store
        .unwrap();
    let expected_values = update_store::default_update_store_input();
    verify_update_store_values(updated_store, expected_values);
}

#[test]
#[ignore]
pub fn update_store_does_not_update_rating() {
    //setup
    let mut context = TestContext::new();
    //given
    let (_user, token, store, _) = set_up_store(&mut context).unwrap();
    context.set_bearer(token);
    let initial_rating = store.rating;
    //when
    let updated_store = context
        .update_store(update_store::UpdateStoreInput {
            id: store.id,
            ..update_store::default_update_store_input()
        })
        .unwrap()
        .update_store;
    //then
    let updated_store = context
        .get_store(updated_store.raw_id)
        .unwrap()
        .store
        .unwrap();
    assert!((updated_store.rating - initial_rating).abs() < 0.001);
}

#[test]
pub fn delete_attribute() {
    //setup
    let mut context = TestContext::new();
    context.as_superadmin();
    //given
    let attribute = context
        .request(create_attribute::default_create_attribute_input())
        .expect("create_attribute failed");
    //when
    let _ = context
        .request(delete_attribute::DeleteAttributeInput {
            id: attribute.id,
            ..delete_attribute::default_delete_attribute_input()
        })
        .expect("delete_attribute failed");
    //then
    let all_attribute = context.get_attributes().unwrap().attributes.unwrap();
    assert!(all_attribute.is_empty());
}

#[test]
pub fn update_attribute() {
    //setup
    let mut context = TestContext::new();
    context.as_superadmin();
    //given
    let attribute = context
        .request(create_attribute::default_create_attribute_input())
        .expect("create_attribute failed");
    //when
    let updated_attribute = context
        .update_attribute(update_attribute::UpdateAttributeInput {
            id: attribute.id,
            ..update_attribute::default_update_attribute_input()
        })
        .unwrap()
        .update_attribute;
    //then
    assert_eq!(updated_attribute.name[0].text, "Update category");
}

#[test]
pub fn delete_attribute_from_category() {
    //setup
    let mut context = TestContext::new();
    //given
    context.as_superadmin();
    let category = context
        .request(create_category::default_create_category_input())
        .expect("create_category failed");
    let attribute = context
        .request(create_attribute::default_create_attribute_input())
        .expect("create_attribute failed");
    let _ = context
        .request(add_attribute_to_category::AddAttributeToCategoryInput {
            cat_id: category.raw_id,
            attr_id: attribute.raw_id,
            ..add_attribute_to_category::default_add_attribute_to_categoryinput()
        })
        .expect("add_attribute_to_category failed");
    //when
    let _ = context
        .request(
            delete_attribute_from_category::DeleteAttributeFromCategory {
                cat_id: category.raw_id,
                attr_id: attribute.raw_id,
                ..delete_attribute_from_category::default_delete_attribute_from_category_input()
            },
        )
        .expect("delete_attribute_from_category failed");
    //then
    let changed_category_attributes = context
        .get_categories()
        .unwrap()
        .categories
        .unwrap()
        .children
        .into_iter()
        .filter(|cat| cat.id == category.id)
        .next()
        .unwrap()
        .get_attributes;
    assert!(changed_category_attributes.is_empty());
}

#[test]
pub fn a_microservice_healthcheck() {
    //given
    let context = TestContext::new();
    //then
    let _ = context.microservice_healthcheck().unwrap();
}

#[test]
pub fn add_attribute_to_category() {
    //setup
    let mut context = TestContext::new();
    //given
    context.as_superadmin();
    let category = context
        .request(create_category::default_create_category_input())
        .expect("create_category failed");
    let attribute = context
        .request(create_attribute::default_create_attribute_input())
        .expect("create_attribute failed");
    //when
    let _ = context
        .request(add_attribute_to_category::AddAttributeToCategoryInput {
            cat_id: category.raw_id,
            attr_id: attribute.raw_id,
            ..add_attribute_to_category::default_add_attribute_to_categoryinput()
        })
        .expect("add_attribute_to_category failed");
    //then
    let changed_category_attributes = context
        .get_categories()
        .unwrap()
        .categories
        .unwrap()
        .children
        .into_iter()
        .filter(|cat| cat.id == category.id)
        .next()
        .unwrap()
        .get_attributes;
    assert_eq!(changed_category_attributes.len(), 1);
    assert!(changed_category_attributes
        .iter()
        .filter(|attr| attr.id == attribute.id)
        .next()
        .is_some());
}

#[test]
pub fn delete_category() {
    //setup
    let mut context = TestContext::new();
    context.as_superadmin();
    //given
    let category = context
        .request(create_category::default_create_category_input())
        .expect("create_category failed");
    //when
    let _ = context
        .request(delete_category::DeleteCategoryInput {
            cat_id: category.raw_id,
            ..delete_category::default_delete_category_input()
        })
        .expect("delete_category failed");
    //then
    let existing_categories = context
        .get_categories()
        .unwrap()
        .categories
        .unwrap()
        .children;
    assert!(existing_categories.is_empty());
}

#[test]
pub fn update_category() {
    //setup
    let mut context = TestContext::new();
    context.as_superadmin();
    //given
    let category = context
        .request(create_category::default_create_category_input())
        .expect("create_category failed");
    //when
    let updated_category = context
        .update_category(update_category::UpdateCategoryInput {
            id: category.id,
            ..update_category::default_update_category_input()
        })
        .unwrap()
        .update_category;
    //then
    let expected_values = update_category::default_update_category_input();
    assert_eq!(updated_category.slug, expected_values.slug.unwrap());
    assert_eq!(
        updated_category.meta_field.unwrap(),
        expected_values.meta_field.unwrap()
    );
    assert_eq!(
        updated_category.parent_id.unwrap(),
        expected_values.parent_id.unwrap()
    );
    assert_eq!(updated_category.level, expected_values.level.unwrap());
}

#[test]
pub fn create_base_product() {
    //setup
    let mut context = TestContext::new();
    //given
    let (_user, token, store, category) = set_up_store(&mut context).unwrap();
    context.set_bearer(token);
    let new_base_product = create_base_product::CreateBaseProductInput {
        store_id: store.raw_id,
        category_id: category.raw_id,
        ..create_base_product::default_create_base_product_input()
    };
    //when
    let base_product = context
        .request(new_base_product)
        .expect("create_base_product failed");
    //then
    assert_eq!(
        base_product.slug,
        create_base_product::default_create_base_product_input()
            .slug
            .unwrap()
    );
}

#[test]
pub fn delete_attribute_value() {
    //setup
    let mut context = TestContext::new();
    context.as_superadmin();
    //given
    let attribute = context
        .request(create_attribute::default_create_attribute_input())
        .expect("create_attribute failed");
    let new_value = context
        .request(create_attribute_value::CreateAttributeValueInput {
            raw_attribute_id: attribute.raw_id,
            ..create_attribute_value::default_create_attribute_value_input()
        })
        .expect("create_attribute_value failed");
    //when
    let _ = context
        .request(delete_attribute_value::DeleteAttributeValueInput {
            raw_id: new_value.raw_id,
            ..delete_attribute_value::default_delete_attribute_value_input()
        })
        .expect("delete_attribute_value failed");
    //then
    let changed_attribute = context
        .get_attributes()
        .unwrap()
        .attributes
        .into_iter()
        .flatten()
        .filter(|a| a.raw_id == attribute.raw_id)
        .next()
        .unwrap();
    assert!(changed_attribute.values.unwrap().is_empty());
}

#[test]
pub fn update_attribute_value() {
    //setup
    let mut context = TestContext::new();
    context.as_superadmin();
    //given
    let attribute = context
        .request(create_attribute::default_create_attribute_input())
        .expect("create_attribute failed");
    let new_value = context
        .request(create_attribute_value::CreateAttributeValueInput {
            raw_attribute_id: attribute.raw_id,
            ..create_attribute_value::default_create_attribute_value_input()
        })
        .expect("create_attribute_value failed");
    //when
    let updated = context
        .update_attribute_value(update_attribute_value::UpdateAttributeValueInput {
            raw_id: new_value.raw_id,
            raw_attribute_id: attribute.raw_id,
            ..update_attribute_value::default_create_attribute_value_input()
        })
        .unwrap()
        .update_attribute_value;
    //then
    assert_eq!(
        Some(updated.code),
        update_attribute_value::default_create_attribute_value_input().code
    );
}

#[test]
pub fn add_values_to_attribute() {
    //setup
    let mut context = TestContext::new();
    context.as_superadmin();
    //given
    let attribute = context
        .request(create_attribute::default_create_attribute_input())
        .expect("create_attribute failed");
    //when
    let new_value = context
        .request(create_attribute_value::CreateAttributeValueInput {
            raw_attribute_id: attribute.raw_id,
            ..create_attribute_value::default_create_attribute_value_input()
        })
        .expect("create_attribute_value failed");
    //then
    assert_eq!(new_value.attr_raw_id, attribute.raw_id);
    assert_eq!(new_value.attribute.unwrap().raw_id, attribute.raw_id);
}

#[test]
pub fn create_attribute_with_values() {
    //setup
    let mut context = TestContext::new();
    context.as_superadmin();
    //then
    let _attribute = context
        .request(create_attribute::CreateAttributeInput {
            values: Some(vec![
                create_attribute::CreateAttributeValueWithAttributeInput {
                    code: "attribute_code".to_string(),
                    translations: Some(vec![create_attribute::TranslationInput {
                        lang: create_attribute::Language::EN,
                        text: "attribute code".to_string(),
                    }]),
                },
            ]),
            ..create_attribute::default_create_attribute_input()
        })
        .expect("create_attribute failed");
}

#[test]
pub fn create_attribute() {
    //setup
    let mut context = TestContext::new();
    context.as_superadmin();
    //then
    let _attribute = context
        .request(create_attribute::default_create_attribute_input())
        .expect("create_attribute failed");
}

#[test]
pub fn create_subcategories() {
    //setup
    let mut context = TestContext::new();
    context.as_superadmin();
    //given
    let category_level_1 = context
        .request(create_category::default_create_category_input())
        .expect("create_category failed");
    let category_level_2 = context
        .request(create_category::CreateCategoryInput {
            parent_id: category_level_1.raw_id,
            slug: Some("category-slug-1".to_string()),
            ..create_category::default_create_category_input()
        })
        .expect("create_category failed");
    //when
    let category_level_3 = context
        .request(create_category::CreateCategoryInput {
            parent_id: category_level_2.raw_id,
            slug: Some("category-slug-2".to_string()),
            ..create_category::default_create_category_input()
        })
        .expect("create_category failed");
    //then
    assert_eq!(category_level_3.level, 3);
}

#[test]
pub fn create_category() {
    //setup
    let mut context = TestContext::new();
    context.as_superadmin();
    //given
    let new_category = create_category::default_create_category_input();
    //when
    let category = context
        .request(new_category)
        .expect("create_category failed");
    //then
    assert_eq!(
        Some(category.slug),
        create_category::default_create_category_input().slug
    );
    let existing_categories = context
        .get_categories()
        .unwrap()
        .categories
        .unwrap()
        .children;
    assert_eq!(existing_categories.len(), 1);
}

#[test]
pub fn create_store() {
    //setup
    let mut context = TestContext::new();
    //given
    let user = context
        .request(create_user::default_create_user_input())
        .expect("createUser failed");
    context.verify_user_email(&user.email).unwrap();
    let token: String = context
        .get_jwt_by_email(get_jwt_by_email::default_create_jwt_email_input())
        .unwrap()
        .get_jwt_by_email
        .token;
    context.set_bearer(token);
    //when
    let store = context
        .request(create_store::CreateStoreInput {
            user_id: user.raw_id,
            ..create_store::default_create_store_input()
        })
        .expect("create_store failed");
    //then
    assert_eq!(store.user_id, user.raw_id);
}

#[test]
pub fn create_user() {
    //setup
    let context = TestContext::new();
    //given
    let new_user = create_user::default_create_user_input();
    //when
    let user = context.request(new_user).expect("createUser failed");
    //then
    assert_eq!(user.email, create_user::default_create_user_input().email);
}

#[test]
pub fn delete_user() {
    //setup
    let mut context = TestContext::new();
    //given
    let new_user = create_user::default_create_user_input();
    let user = context.request(new_user).expect("createUser failed");
    //when
    context.as_superadmin();
    let delete_result = context.delete_user(user.raw_id);
    //then
    assert!(delete_result.is_ok())
}

#[test]
pub fn create_user_with_additional_data() {
    //setup
    let context = TestContext::new();
    //given
    let referal = context
        .request(create_user::CreateUserInput {
            email: "referral@email.net".to_string(),
            ..create_user::default_create_user_input()
        })
        .expect("createUser failed");

    let new_user = create_user::CreateUserInput {
        additional_data: Some(create_user::NewUserAdditionalDataInput {
            country: Some("MM".to_string()),
            referal: Some(referal.raw_id),
            referer: Some("localhost".to_string()),
            utm_marks: Some(vec![create_user::UtmMarkInput {
                key: "source".to_string(),
                value: "word_of_mouth".to_string(),
            }]),
        }),
        ..create_user::default_create_user_input()
    };
    //when
    let user = context.request(new_user).expect("createUser failed");
    //then
    assert_eq!(user.email, create_user::default_create_user_input().email);
    assert_eq!(user.referal.expect("user.referal is none"), referal.raw_id);
    assert_eq!(
        user.country.expect("user.country is none").alpha3,
        "MMR".to_string()
    );
    assert_eq!(
        user.referer.expect("user.referer is none"),
        "localhost".to_string()
    );
    assert_eq!(
        &user.utm_marks.as_ref().expect("user.utm_marks is none")[0].key,
        "source"
    );
    assert_eq!(
        &user.utm_marks.as_ref().expect("user.utm_marks is none")[0].value,
        "word_of_mouth"
    );
}

#[test]
#[ignore]
pub fn create_user_via_facebook() {
    //setup
    let context = TestContext::new();
    //given
    let facebook_jwt = get_jwt_by_provider::facebook_create_jwt_provider_input();
    //when
    let user = context.create_user_jwt(facebook_jwt);
    //then
    assert!(user.is_ok());
}

#[test]
#[ignore]
pub fn create_user_via_google() {
    //setup
    let context = TestContext::new();
    //given
    let google_jwt = get_jwt_by_provider::google_create_jwt_provider_input();
    //when
    let user = context.create_user_jwt(google_jwt);
    //then
    assert!(user.is_ok());
}

#[test]
#[ignore]
pub fn create_user_via_facebook_with_additional_data() {
    //setup
    let context = TestContext::new();
    //given
    let referal = context
        .request(create_user::CreateUserInput {
            email: "referral@email.net".to_string(),
            ..create_user::default_create_user_input()
        })
        .expect("createUser failed");

    let facebook_jwt = get_jwt_by_provider::CreateJWTProviderInput {
        additional_data: Some(get_jwt_by_provider::NewUserAdditionalDataInput {
            country: Some("MM".to_string()),
            referal: Some(referal.raw_id),
            referer: Some("localhost".to_string()),
            utm_marks: Some(vec![get_jwt_by_provider::UtmMarkInput {
                key: "source".to_string(),
                value: "word_of_mouth".to_string(),
            }]),
        }),
        ..get_jwt_by_provider::facebook_create_jwt_provider_input()
    };
    //when
    let user = context.create_user_jwt(facebook_jwt);
    //then
    assert!(user.is_ok());
    panic!("Finish test");
}

#[test]
#[ignore]
pub fn create_user_via_google_with_additional_data() {
    //setup
    let context = TestContext::new();
    //given
    let referal = context
        .request(create_user::CreateUserInput {
            email: "referral@email.net".to_string(),
            ..create_user::default_create_user_input()
        })
        .expect("createUser failed");

    let google_jwt = get_jwt_by_provider::CreateJWTProviderInput {
        additional_data: Some(get_jwt_by_provider::NewUserAdditionalDataInput {
            country: Some("MM".to_string()),
            referal: Some(referal.raw_id),
            referer: Some("localhost".to_string()),
            utm_marks: Some(vec![get_jwt_by_provider::UtmMarkInput {
                key: "source".to_string(),
                value: "word_of_mouth".to_string(),
            }]),
        }),
        ..get_jwt_by_provider::google_create_jwt_provider_input()
    };
    //when
    let user = context.create_user_jwt(google_jwt);
    //then
    assert!(user.is_ok());
    panic!("Finish test");
}

#[test]
pub fn delete_store() {
    //setup
    let mut context = TestContext::new();
    //given
    let (_user, token, store, _) = set_up_store(&mut context).unwrap();
    context.set_bearer(token);
    //when
    context.as_superadmin();
    let delete_result = context.delete_store(store.raw_id);
    //then
    assert!(delete_result.is_ok())
}

#[test]
pub fn update_store_in_status_draft() {
    //setup
    let mut context = TestContext::new();
    //given
    let (_user, token, store, _) =
        set_up_store(&mut context).expect("Cannot get data from set_up_store");
    context.set_bearer(token);

    //when
    let update_result = context.update_store(update_store::UpdateStoreInput {
        id: store.id.clone(),
        email: Some("example@example.com".to_string()),
        ..update_store::default_update_store_input()
    });

    let update_store = update_result.expect("Cannot get update store").update_store;

    //then
    assert_eq!(update_store.email, Some("example@example.com".to_string()));
    assert_eq!(update_store.id, store.id);
    assert_eq!(update_store.status, update_store::Status::DRAFT)
}

#[test]
pub fn update_base_product_in_status_draft() {
    //setup
    let mut context = TestContext::new();
    //given
    let (_user, token, _store, _category, base_product) =
        set_up_base_product(&mut context).expect("Cannot get data from set_up_base_product");
    context.set_bearer(token);
    //when
    let update_base_product_payload = update_base_product::UpdateBaseProductInput {
        id: base_product.id.clone(),
        slug: Some(format!("{}-{}", base_product.slug, "plus")),
        ..update_base_product::default_update_base_product_input()
    };

    let update_base_product = context
        .update_base_product(update_base_product_payload)
        .expect("Cannot get update base_product")
        .update_base_product;

    //then
    assert_eq!(update_base_product.id, base_product.id);
    assert_eq!(
        update_base_product.slug,
        format!("{}-{}", base_product.slug, "plus")
    );
    assert_eq!(
        update_base_product.status,
        update_base_product::Status::DRAFT
    );
}

#[test]
pub fn create_product_without_attributes() {
    //setup
    let mut context = TestContext::new();
    //given
    let (_user, token, _store, _category, base_product) =
        set_up_base_product(&mut context).expect("Cannot get data from set_up_base_product");
    context.set_bearer(token);
    //when
    let product_payload = create_product::CreateProductWithAttributesInput {
        product: create_product::NewProduct {
            base_product_id: Some(base_product.raw_id),
            ..create_product::default_new_product()
        },
        ..create_product::default_create_product_input()
    };

    let new_product = context
        .request(product_payload)
        .expect("Cannot get data from create_product");

    //then
    assert_eq!(base_product.status, create_base_product::Status::DRAFT);
    assert_eq!(new_product.base_product_id, base_product.raw_id);
}

#[test]
pub fn deactivate_product() {
    //setup
    let mut context = TestContext::new();
    //given
    let (_user, token, _store, _category, base_product) =
        set_up_base_product(&mut context).expect("Cannot get data from set_up_base_product");
    context.set_bearer(token);
    let product_payload = create_product::CreateProductWithAttributesInput {
        product: create_product::NewProduct {
            base_product_id: Some(base_product.raw_id),
            ..create_product::default_new_product()
        },
        ..create_product::default_create_product_input()
    };

    let new_product = context
        .request(product_payload)
        .expect("Cannot get data from create_product");

    let deactivate_product_payload = deactivate_product::DeactivateProductInput {
        id: new_product.id.clone(),
        ..deactivate_product::default_deactivate_product_input()
    };

    //when
    let product = context
        .request(deactivate_product_payload)
        .expect("Cannot get data from deactivate_product");
    //then
    assert_eq!(new_product.id, product.id);
    assert_eq!(product.is_active, false);
}

#[test]
pub fn create_product_without_base_product() {
    //setup
    let mut context = TestContext::new();
    //given
    let (_user, token, _store, _category, base_product) =
        set_up_base_product(&mut context).expect("Cannot get data from set_up_base_product");
    context.set_bearer(token);
    //when
    let product_payload = create_product::CreateProductWithAttributesInput {
        product: create_product::default_new_product(),
        ..create_product::default_create_product_input()
    };

    let new_product = context.request(product_payload);

    //then
    assert_eq!(base_product.status, create_base_product::Status::DRAFT);
    assert!(new_product.is_err());
}

#[test]
#[ignore]
pub fn update_product_without_attributes() {
    //setup
    let mut context = TestContext::new();
    //given
    let (_user, token, _store, _category, base_product) =
        set_up_base_product(&mut context).expect("Cannot get data from set_up_base_product");
    context.set_bearer(token);
    let product_payload = create_product::CreateProductWithAttributesInput {
        product: create_product::NewProduct {
            base_product_id: Some(base_product.raw_id),
            ..create_product::default_new_product()
        },
        ..create_product::default_create_product_input()
    };

    let new_product = context
        .request(product_payload)
        .expect("Cannot get data from create_product");

    //when
    let update_product_payload = update_product::UpdateProductWithAttributesInput {
        id: new_product.id.clone(),
        product: Some(update_product::UpdateProduct {
            price: Some(15f64),
            pre_order: Some(true),
            pre_order_days: Some(15),
            ..update_product::default_update_product_input()
        }),
        ..update_product::default_update_product_with_attributes_input()
    };

    let update_product = context
        .update_product(update_product_payload)
        .expect("Cannot get update product")
        .update_product;
    //then
    assert_eq!(base_product.status, create_base_product::Status::DRAFT);
    assert_eq!(update_product.id, new_product.id);
    assert_eq!(update_product.pre_order, true);
    assert_eq!(update_product.pre_order_days, 15);
}

#[test]
pub fn create_delivery_company() {
    //setup
    let mut context = TestContext::new();
    context.as_superadmin();
    //given
    let company_payload = create_delivery_company::NewCompanyInput {
        name: "Test company".to_string(),
        label: "TEST".to_string(),
        description: Some("Test description".to_string()),
        deliveries_from: vec!["RUS".to_string()],
        logo: "test loge URL".to_string(),
        ..create_delivery_company::default_create_company_input()
    };
    //when
    let create_company = context
        .request(company_payload.clone())
        .expect("Cannot get data from create_delivery_company");

    let rus_country = create_company
        .deliveries_from
        .iter()
        .flat_map(|root| {
            root.children
                .iter()
                .flat_map(|region| region.children.iter())
        })
        .find(|c| c.alpha3 == "RUS".to_string());
    //then
    assert_eq!(
        rus_country.map(|c| c.alpha3.clone()),
        Some("RUS".to_string())
    );
    assert_eq!(create_company.label, company_payload.label);
    assert_eq!(create_company.name, company_payload.name);
    assert_eq!(create_company.description, company_payload.description);
    assert_eq!(create_company.logo, company_payload.logo);
}

#[test]
pub fn update_delivery_company() {
    //setup
    let mut context = TestContext::new();
    context.as_superadmin();
    //given
    let company_payload = create_delivery_company::default_create_company_input();
    let create_company = context
        .request(company_payload)
        .expect("Cannot get data from create_delivery_company");
    //when
    let update_company_payload = update_delivery_company::UpdateCompanyInput {
        id: create_company.id.clone(),
        name: Some("Test company plus update".to_string()),
        ..update_delivery_company::default_update_company_input()
    };
    let update_company = context
        .update_delivery_company(update_company_payload)
        .expect("Cannot get data from update_delivery_company")
        .update_company;
    //then
    assert_eq!(update_company.id, create_company.id);
    assert_eq!(update_company.name, "Test company plus update".to_string());
}

#[test]
pub fn delete_delivery_company() {
    //setup
    let mut context = TestContext::new();
    context.as_superadmin();
    //given
    let company_payload = create_delivery_company::default_create_company_input();
    let create_company = context
        .request(company_payload)
        .expect("Cannot get data from create_delivery_company");
    //when
    let delete_company = context
        .delete_delivery_company(create_company.raw_id)
        .expect("Cannot get data from delete_delivery_company")
        .delete_company;
    //then
    assert_eq!(create_company.raw_id, delete_company.raw_id);
}

fn set_up_store(
    context: &mut TestContext,
) -> Result<
    (
        create_user::RustCreateUserCreateUser,
        String,
        create_store::RustCreateStoreCreateStore,
        create_category::RustCreateCategoryCreateCategory,
    ),
    FailureError,
> {
    let user = context.request(create_user::default_create_user_input())?;
    context.verify_user_email(&user.email).unwrap();
    let token: String = context
        .get_jwt_by_email(get_jwt_by_email::default_create_jwt_email_input())?
        .get_jwt_by_email
        .token;
    context.set_bearer(token.clone());
    let store = context.request(create_store::CreateStoreInput {
        user_id: user.raw_id,
        ..create_store::default_create_store_input()
    })?;
    context.as_superadmin();
    let category_level_1 = context.request(create_category::default_create_category_input())?;
    let category_level_2 = context.request(create_category::CreateCategoryInput {
        parent_id: category_level_1.raw_id,
        slug: Some("category-slug-1".to_string()),
        ..create_category::default_create_category_input()
    })?;
    let category_level_3 = context.request(create_category::CreateCategoryInput {
        parent_id: category_level_2.raw_id,
        slug: Some("category-slug-2".to_string()),
        ..create_category::default_create_category_input()
    })?;
    context.clear_bearer();
    Ok((user, token, store, category_level_3))
}

fn verify_update_store_values(
    updated_store: get_store::RustGetStoreStore,
    expected_values: update_store::UpdateStoreInput,
) {
    assert_eq!(
        updated_store.name[0].text,
        expected_values.name.unwrap()[0].text
    );
    assert_eq!(
        updated_store.short_description[0].text,
        expected_values.short_description.unwrap()[0].text
    );
    assert_eq!(
        updated_store
            .long_description
            .expect("update_store.long_description is none")[0]
            .text,
        expected_values.long_description.unwrap()[0].text
    );
    assert_eq!(updated_store.slug, expected_values.slug.unwrap());
    assert_eq!(
        updated_store.cover.expect("update_store.cover is none"),
        expected_values.cover.unwrap()
    );
    assert_eq!(
        updated_store.logo.expect("update_store.logo is none"),
        expected_values.logo.unwrap()
    );
    assert_eq!(
        updated_store.phone.expect("update_store.phone is none"),
        expected_values.phone.unwrap()
    );
    assert_eq!(
        updated_store.email.expect("update_store.email is none"),
        expected_values.email.unwrap()
    );
    assert_eq!(
        updated_store
            .instagram_url
            .expect("update_store.instagram_url is none"),
        expected_values.instagram_url.unwrap()
    );
    assert_eq!(
        updated_store
            .twitter_url
            .expect("update_store.twitter_url is none"),
        expected_values.twitter_url.unwrap()
    );
    assert_eq!(
        updated_store
            .facebook_url
            .expect("update_store.facebook_url is none"),
        expected_values.facebook_url.unwrap()
    );
    assert_eq!(
        updated_store.slogan.expect("update_store.slogan is none"),
        expected_values.slogan.unwrap()
    );
    assert!((updated_store.rating - expected_values.rating.unwrap()).abs() < 0.001);

    assert_eq!(
        updated_store
            .address_full
            .value
            .expect("update_store.address_full.value is none"),
        expected_values.address_full.value.unwrap()
    );
    assert_eq!(
        updated_store
            .address_full
            .country
            .expect("update_store.address_full.country is none"),
        expected_values.address_full.country.unwrap()
    );
    assert_eq!(
        updated_store
            .address_full
            .country_code
            .expect("update_store.address_full.country_code is none"),
        expected_values.address_full.country_code.unwrap()
    );
    assert_eq!(
        updated_store
            .address_full
            .administrative_area_level1
            .expect("update_store.address_full.administrative_area_level1 is none"),
        expected_values
            .address_full
            .administrative_area_level1
            .unwrap()
    );
    assert_eq!(
        updated_store
            .address_full
            .administrative_area_level2
            .expect("update_store.address_full.administrative_area_level2 is none"),
        expected_values
            .address_full
            .administrative_area_level2
            .unwrap()
    );
    assert_eq!(
        updated_store
            .address_full
            .locality
            .expect("update_store.address_full.locality is none"),
        expected_values.address_full.locality.unwrap()
    );
    assert_eq!(
        updated_store
            .address_full
            .political
            .expect("update_store.address_full.political is none"),
        expected_values.address_full.political.unwrap()
    );
    assert_eq!(
        updated_store
            .address_full
            .postal_code
            .expect("update_store.address_full.postal_code is none"),
        expected_values.address_full.postal_code.unwrap()
    );
    assert_eq!(
        updated_store
            .address_full
            .route
            .expect("update_store.address_full.route is none"),
        expected_values.address_full.route.unwrap()
    );
    assert_eq!(
        updated_store
            .address_full
            .street_number
            .expect("update_store.address_full.street_number is none"),
        expected_values.address_full.street_number.unwrap()
    );
    assert_eq!(
        updated_store
            .address_full
            .place_id
            .expect("update_store.address_full.place_id is none"),
        expected_values.address_full.place_id.unwrap()
    );
}

fn set_up_base_product(
    context: &mut TestContext,
) -> Result<
    (
        create_user::RustCreateUserCreateUser,
        String,
        create_store::RustCreateStoreCreateStore,
        create_category::RustCreateCategoryCreateCategory,
        create_base_product::RustCreateBaseProductCreateBaseProduct,
    ),
    FailureError,
> {
    let (user, token, store, category) =
        set_up_store(context).expect("Cannot get data from set_up_store");
    context.set_bearer(token.clone());

    let new_base_product = create_base_product::CreateBaseProductInput {
        store_id: store.raw_id,
        category_id: category.raw_id,
        ..create_base_product::default_create_base_product_input()
    };
    let base_product = context.request(new_base_product)?;
    context.clear_bearer();

    Ok((user, token, store, category, base_product))
}
