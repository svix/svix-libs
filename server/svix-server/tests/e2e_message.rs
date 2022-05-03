// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use chrono::{Duration, Utc};
use reqwest::StatusCode;
use sea_orm::{sea_query::Expr, ColumnTrait, EntityTrait, QueryFilter};

use svix_server::{
    db::models::message,
    expired_message_cleaner,
    v1::{endpoints::message::MessageOut, utils::ListResponse},
};

mod utils;

use utils::{
    common_calls::{create_test_app, create_test_endpoint, message_in},
    start_svix_server,
};

#[tokio::test]
async fn test_message_create_read_list() {
    let (client, _jh) = start_svix_server();

    let app_id = create_test_app(&client, "v1MessageCRTestApp")
        .await
        .unwrap()
        .id;

    let _endp_id = create_test_endpoint(&client, &app_id, "http://localhost:2/bad/url/")
        .await
        .unwrap()
        .id;

    // CREATE
    let message_1: MessageOut = client
        .post(
            &format!("api/v1/app/{}/msg/", &app_id),
            message_in(&app_id, serde_json::json!({"test": "value"})).unwrap(),
            StatusCode::ACCEPTED,
        )
        .await
        .unwrap();
    let message_2: MessageOut = client
        .post(
            &format!("api/v1/app/{}/msg/", &app_id),
            message_in(&app_id, serde_json::json!({"test": "value2"})).unwrap(),
            StatusCode::ACCEPTED,
        )
        .await
        .unwrap();

    assert_eq!(
        client
            .get::<MessageOut>(
                &format!("api/v1/app/{}/msg/{}/", &app_id, &message_1.id),
                StatusCode::OK
            )
            .await
            .unwrap(),
        message_1
    );
    assert_eq!(
        client
            .get::<MessageOut>(
                &format!("api/v1/app/{}/msg/{}/", &app_id, &message_2.id),
                StatusCode::OK
            )
            .await
            .unwrap(),
        message_2
    );

    let list: ListResponse<MessageOut> = client
        .get(&format!("api/v1/app/{}/msg/", &app_id), StatusCode::OK)
        .await
        .unwrap();
    assert_eq!(list.data.len(), 2);
    assert!(list.data.contains(&message_1));
    assert!(list.data.contains(&message_2));
}

#[tokio::test]
async fn test_message_create_read_list_with_content() {
    let (client, _jh) = start_svix_server();

    let app_id = create_test_app(&client, "v1MessageCRTestApp")
        .await
        .unwrap()
        .id;

    let _endp_id = create_test_endpoint(&client, &app_id, "http://localhost:2/bad/url/")
        .await
        .unwrap()
        .id;

    let msg_payload = serde_json::json!({"test": "value"});

    let msg_1_w_payload: MessageOut = client
        .post(
            &format!("api/v1/app/{}/msg", &app_id),
            message_in(&app_id, msg_payload.clone()).unwrap(),
            StatusCode::ACCEPTED,
        )
        .await
        .unwrap();

    assert_eq!(msg_1_w_payload.payload, msg_payload);

    let msg_2_wo_payload: MessageOut = client
        .post(
            &format!("api/v1/app/{}/msg/?with_content=false", &app_id),
            message_in(&app_id, msg_payload.clone()).unwrap(),
            StatusCode::ACCEPTED,
        )
        .await
        .unwrap();

    assert_eq!(msg_2_wo_payload.payload, serde_json::json!({}));

    let msg_1_wo_payload = MessageOut {
        payload: serde_json::json!({}),
        ..msg_1_w_payload.clone()
    };
    let msg_2_w_payload = MessageOut {
        payload: msg_payload,
        ..msg_2_wo_payload.clone()
    };

    for m in [&msg_1_w_payload, &msg_2_w_payload] {
        assert_eq!(
            client
                .get::<MessageOut>(
                    &format!("api/v1/app/{}/msg/{}/", &app_id, &m.id),
                    StatusCode::OK
                )
                .await
                .unwrap(),
            *m,
        );
    }

    for m in [&msg_1_wo_payload, &msg_2_wo_payload] {
        assert_eq!(
            client
                .get::<MessageOut>(
                    &format!("api/v1/app/{}/msg/{}/?with_content=false", &app_id, &m.id),
                    StatusCode::OK
                )
                .await
                .unwrap(),
            *m
        );
    }

    let list: ListResponse<MessageOut> = client
        .get(&format!("api/v1/app/{}/msg", &app_id), StatusCode::OK)
        .await
        .unwrap();
    assert_eq!(list.data.len(), 2);
    assert!(list.data.contains(&msg_1_w_payload));
    assert!(list.data.contains(&msg_2_w_payload));

    let list: ListResponse<MessageOut> = client
        .get(
            &format!("api/v1/app/{}/msg/?with_content=false", &app_id),
            StatusCode::OK,
        )
        .await
        .unwrap();
    assert_eq!(list.data.len(), 2);
    assert!(list.data.contains(&msg_1_wo_payload));
    assert!(list.data.contains(&msg_2_wo_payload));
}

#[tokio::test]
async fn test_payload_retention_period() {
    let (client, _jh) = start_svix_server();
    dotenv::dotenv().ok();
    let cfg = svix_server::cfg::load().expect("Error loading configuration");
    let pool = svix_server::db::init_db(&cfg).await;

    let app_id = create_test_app(&client, "v1MessageCRTestApp")
        .await
        .unwrap()
        .id;

    let msg_row: MessageOut = client
        .post(
            &format!("api/v1/app/{}/msg/", &app_id),
            message_in(&app_id, serde_json::json!({"test": "value"})).unwrap(),
            StatusCode::ACCEPTED,
        )
        .await
        .unwrap();
    let msg_row_2 = msg_row.clone();

    message::Entity::update_many()
        .col_expr(
            message::Column::Expiration,
            Expr::value(Utc::now() - Duration::days(1)),
        )
        .filter(message::Column::Id.eq(msg_row.id))
        .exec(&pool)
        .await
        .unwrap();

    expired_message_cleaner::clean_expired_messages(&pool)
        .await
        .unwrap();

    let message: Option<message::Model> = message::Entity::find_by_id(msg_row_2.id)
        .one(&pool)
        .await
        .unwrap();

    assert_eq!(message.unwrap().payload, None);
}
