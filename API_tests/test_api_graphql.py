#!/usr/bin/env python3
# -*- coding: utf-8 -*-
import requests
import json
from datetime import datetime
import os

n = datetime.strftime(datetime.now(), "%m%d%H%M%S")
regmail = 'test' + n + '@test.test'

if os.getenv('GRAPHQL_URL'):
    url = os.environ['GRAPHQL_URL']
else: url = 'https://nightly.stq.cloud/graphql'
#url = 'https://nightly.stq.cloud/graphql'

def request(json_query, headers):
    r = requests.post(url, json=json_query, headers=headers)
    return r


#Проверка версии
version = {"query": "query {apiVersion}"}
request_version = request(version, '')
print (request_version.text)

#Получение токена админа
admin_token = {"query":
                   "mutation getJWTByEmail($input: CreateJWTEmailInput!) {getJWTByEmail (input: $input) {token}}",
    "variables": {"input": {"clientMutationId": "1",
                            "email": "admin@storiqa.com",
                            "password": "bqF5BkdsCS"
                            }} }
get_admin_token = request(admin_token, '')
print (get_admin_token.text)
token = get_admin_token.json()['data']['getJWTByEmail']['token']
token_admin = {'Authorization': 'Bearer '+token}
print ('Admin token is: %s' % token)

#Создание категории
category = {"query":
                "mutation createCategory($input: CreateCategoryInput!) {createCategory(input: $input) {id rawId name {lang text}}}",
    "variables": {"input": {"clientMutationId": "1",
                            "name": [{"lang": "DE", "text": "test"},
                                     {"lang": "RU", "text": "тест%s" % n}],
                            "level": 1
                            }} }
create_category = request(category, token_admin)
print (create_category.text)
cat_id = create_category.json()['data']['createCategory']['id']
cat_rawid = create_category.json()['data']['createCategory']['rawId']


#Редактирование категории
category = {"query":
                "mutation updateCategory($input: UpdateCategoryInput!) {updateCategory(input: $input) {id}}",
    "variables": {"input" : {"id": cat_id,
                             "clientMutationId": "1",
                             "name": [{"lang": "EN", "text": "test%s"% n}]
                             }} }
update_category = request(category, token_admin)
print (update_category.text)

#Создание атрибута
attribute = {"query":
                 "mutation createAttribute($input: CreateAttributeInput!) {createAttribute(input: $input) {id, rawId}}",
    "variables": {"input" : {"clientMutationId": "1",
                             "name": [{"text": "test", "lang": "EN"}],
                             "valueType": "STR",
                             "metaField": { "values": ["ebatb"], "uiElement": "COMBOBOX"}
                             }} }
get_create_attr = request(attribute, token_admin)
print (get_create_attr.text)
attr_id = get_create_attr.json()['data']['createAttribute']['id']
attr_rawid = get_create_attr.json()['data']['createAttribute']['rawId']



#Редактирование атрибута
attribute = {"query":
                 "mutation updateAttribute($input: UpdateAttributeInput!) {updateAttribute(input: $input) {id}}",
             "variables": {"input" : {"clientMutationId": "1",
                                      "id":  attr_id,
                                      "name": [{"text": "test%s"%n, "lang": "EN"}],
                                      }} }
get_update_attr = request(attribute, token_admin)
print (get_update_attr.text)

#Добавление атрибута к категории
add_attr = {"query":
                "mutation addAttributeToCategory($input: AddAttributeToCategoryInput!) {addAttributeToCategory(input: $input) {mock}}",
    "variables": {"input": {"clientMutationId": "1",
                            "catId": cat_rawid,
                            "attrId": attr_rawid
                            }} }
get_add_attr = request(add_attr, token_admin)
print (get_add_attr.text)

#Удаление атрибута у категории
del_attr = {"query":
                "mutation deleteAttributeFromCategory($input: DeleteAttributeFromCategory!) {deleteAttributeFromCategory(input: $input) {mock}}",
    "variables": {"input": {"clientMutationId": "1",
                            "catId": cat_rawid,
                            "attrId": attr_rawid
                            }} }
get_del_attr = request(del_attr, token_admin)
print (get_del_attr.text)

#Создание пользователя
user = {"query":
            "mutation createUser($input: CreateUserInput!) {createUser(input: $input) {id}}",
    "variables": {"input": {"clientMutationId": "1",
                            "email": regmail,
                            "password": "tester11" }},"operationName": "createUser"}
create_user = request(user, '')
print (create_user.text)

#Получение токена пользователя
user_token = {"query":
                  "mutation getJWTByEmail($input: CreateJWTEmailInput!) {getJWTByEmail (input: $input) {token}}",
    "variables": {"input": {"clientMutationId": "1",
                            "email": "tester@storiqa.com",
                            "password": "qwe123QWE"
                            }} }
get_user_token = request(user_token, '')
token = get_user_token.json()['data']['getJWTByEmail']['token']
print ('User token is: %s' % token)
token_headers = {'Authorization': 'Bearer '+token}

#Получаение ID пользователя
user_id = {"query":
               "query {me {id, rawId, isActive}}"}
get_user_id = request(user_id, token_headers)
print (get_user_id.text)
id = get_user_id.json()['data']['me']['id']
rawId = get_user_id.json()['data']['me']['rawId']

#Редактирование пользователя
update_user = {"query":
                   "mutation updateUser($input:  UpdateUserInput!) {updateUser(input: $input){id, isActive}}",
    "variables": {"input": {"clientMutationId": "1",
                            "id": id,
                            "phone": "89095754585",
                            "firstName": "Testoviy",
                            "lastName": "User"+n,
                            "middleName": "epta",
                            "gender": "MALE",
                            "birthdate": "1987-04-04"
                            }} }
get_update_user = request(update_user, token_headers)
print (get_update_user.text)

#Создание магазина
create_store = {"query":
                    "mutation createStore($input: CreateStoreInput!) {createStore(input: $input) {id, name{lang, text}, rawId}}",
    "variables": {"input": {"clientMutationId": "1",
                            "name": [{"lang": "EN", "text": "testshop"+n}],
                            "userId": int(rawId),
                            "defaultLanguage": "EN",
                            "shortDescription": [{"lang": "EN", "text": "test"}],
                            "slug": "test"+n,
                            "phone": "89091112233",
                            "email": "teststore@test.test",
                            "address": "test street 5"
                            }} }
get_create_store = request(create_store, token_headers)
print (get_create_store.text)
store_rawid = get_create_store.json()['data']['createStore']['rawId']
store_id = get_create_store.json()['data']['createStore']['id']

#Редактирование магазина
update_store = {"query":
                    "mutation updateStore($input: UpdateStoreInput!) {updateStore(input: $input) {id, isActive}}",
    "variables": {"input": {"clientMutationId": "1",
                            "id": store_id,
                            "name": [{"lang": "EN", "text": "qwerty"+n}],
                            "shortDescription": [{"lang": "EN", "text": "short"}],
                            "longDescription": [{"lang": "EN", "text": "long"}],
                            "slug": "xxx"+n,
                            "phone": "89093335522",
                            "email": "ssss@test.test",
                            "address": "example 3"
                            }} }
get_update_store = request(update_store, token_headers)
print (get_update_store.text)

#Создание базового товара
create_bproduct = {"query":
                       "mutation createBaseProduct($input: CreateBaseProductInput!) {createBaseProduct(input: $input) {id rawId name {lang text}}}",
    "variables": {"input" : {"clientMutationId": "1",
                             "name": [{"lang": "EN", "text": "testproduct"+n},
                                      {"lang": "RU", "text": "тестпродукт"+n}],
                             "storeId": int(store_rawid),
                             "currencyId": 1,
                             "categoryId": 9,
                             "slug": "bptest"+n,
                             "shortDescription": [{"lang": "EN", "text": "test"}] }} }
get_create_bproduct = request(create_bproduct, token_headers)
print (get_create_bproduct.text)
base_product_id = get_create_bproduct.json()['data']['createBaseProduct']['id']
base_product_rawid = get_create_bproduct.json()['data']['createBaseProduct']['rawId']

#Редактирование базового товара
update_base_product = {"query":
                           "mutation updateBaseProduct($input: UpdateBaseProductInput!) {updateBaseProduct(input: $input) {id, rawId}}",
    "variables": {"input" : {"clientMutationId": "1",
                             "id": base_product_id,
                             "currencyId": 2
                             }} }
get_update_base_product = request(update_base_product, token_headers)
print (get_update_base_product.text)

#Создание товара с атрибутами
create_product = {"query":
                      "mutation createProduct($input: CreateProductWithAttributesInput!) {createProduct(input: $input) {id, isActive, rawId}}",
    "variables": {"input" : {"clientMutationId": "1",
                             "product":  {"baseProductId": int(base_product_rawid),
                                          "price": 200.00,
                                          "vendorCode": "11"},
                             "attributes": [{"attrId": 1,
                                             "value": "1",
                                             "metaField": "dfasfas",}]
                             }} }
get_create_product = request(create_product, token_headers)
print (get_create_product.text)
product_id = get_create_product.json()['data']['createProduct']['id']
product_rawid = get_create_product.json()['data']['createProduct']['rawId']


#Редактирование товара с атрибутам
update_product = {"query":
                      "mutation updateProduct($input: UpdateProductWithAttributesInput!) {updateProduct(input: $input) {id}}",
    "variables": {"input": {"clientMutationId": "1",
                            "id": product_id,
                            "product": {"discount": 1.0},
                            "attributes": []
                            }} }
get_update_product = request(update_product, token_headers)
print (get_update_product.text)

#Получение базовых товаров
baseProducts = {"query":
                    "{me {baseProducts {edges {node {id rawId}}}}}"}
get_baseProducts = request(baseProducts, token_admin)
print (get_baseProducts.text)

#Выключение товара
deactivate_product = {"query":
                          "mutation deactivateProduct($input: DeactivateProductInput!) {deactivateProduct(input: $input) {id, isActive}}",
    "variables": {"input" :{"clientMutationId": "1",
                            "id": product_id
                            }} }
get_deactivate_product = request(deactivate_product, token_headers)
print (get_deactivate_product.text)

#Выключение магазина
deactivate_store = {"query":
                        "mutation deactivateStore($input:  DeactivateStoreInput!) {deactivateStore(input: $input) {id, isActive}}",
    "variables": {"input" :{"clientMutationId": "1",
                            "id": store_id
                            }} }
get_deactivate_store = request(deactivate_store, token_headers)
print (get_deactivate_store.text)

#Выключение пользователя
'''deactivate_user = {"query":
"mutation deactivateUser($input:  DeactivateUserInput!) {deactivateUser(input: $input){id, isActive}}",
    "variables": {"input": {"clientMutationId": "1",
                            "id": id}}  }
get_deactivate_user = request(deactivate_user, token_headers)
print (get_deactivate_user.text)'''