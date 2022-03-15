// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use crate::{
    core::types::{ApplicationId, ApplicationIdOrUid, ApplicationUid},
    error::{HttpError, Result},
    v1::utils::{EmptyResponse, ListResponse, ModelIn, ModelOut, ValidatedJson, ValidatedQuery},
};
use axum::{
    extract::{Extension, Path},
    routing::{get, post},
    Json, Router,
};
use chrono::{DateTime, Utc};
use hyper::StatusCode;
use sea_orm::{entity::prelude::*, ActiveValue::Set, QueryOrder};
use sea_orm::{ActiveModelTrait, DatabaseConnection, QuerySelect};
use serde::{Deserialize, Serialize};
use svix_server_derive::{ModelIn, ModelOut};
use validator::Validate;

use crate::core::security::Permissions;
use crate::db::models::application;
use crate::v1::utils::Pagination;

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, Validate, ModelIn)]
#[serde(rename_all = "camelCase")]
struct ApplicationIn {
    #[validate(length(min = 1))]
    name: String,

    #[validate(range(min = 1))]
    #[serde(skip_serializing_if = "Option::is_none")]
    rate_limit: Option<u16>,
    /// Optional unique identifier for the application
    #[validate]
    #[serde(skip_serializing_if = "Option::is_none")]
    uid: Option<ApplicationUid>,
}

// FIXME: This can and should be a derive macro
impl ModelIn for ApplicationIn {
    type ActiveModel = application::ActiveModel;

    fn update_model(self, model: &mut Self::ActiveModel) {
        model.name = Set(self.name);
        model.rate_limit = Set(self.rate_limit.map(|x| x.into()));
        model.uid = Set(self.uid);
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ModelOut)]
#[serde(rename_all = "camelCase")]
struct ApplicationOut {
    // FIXME: Do we want to use serde(flatten) or just duplicate the keys?
    #[serde(skip_serializing_if = "Option::is_none")]
    uid: Option<ApplicationUid>,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    rate_limit: Option<u16>,

    id: ApplicationId,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

// FIXME: This can and should be a derive macro
impl From<application::Model> for ApplicationOut {
    fn from(model: application::Model) -> Self {
        Self {
            uid: model.uid,
            name: model.name,
            rate_limit: model.rate_limit.map(|x| x as u16),

            id: model.id,
            created_at: model.created_at.into(),
            updated_at: model.updated_at.into(),
        }
    }
}

async fn list_applications(
    Extension(ref db): Extension<DatabaseConnection>,
    pagination: ValidatedQuery<Pagination<ApplicationId>>,
    permissions: Permissions,
) -> Result<Json<ListResponse<ApplicationOut>>> {
    let limit = pagination.limit;
    let iterator = pagination.iterator.clone();

    let mut query = application::Entity::secure_find(permissions.org_id)
        .order_by_asc(application::Column::Id)
        .limit(limit + 1);

    if let Some(iterator) = iterator {
        query = query.filter(application::Column::Id.gt(iterator))
    }

    Ok(Json(ApplicationOut::list_response(
        query.all(db).await?.into_iter().map(|x| x.into()).collect(),
        limit as usize,
    )))
}

async fn create_application(
    Extension(ref db): Extension<DatabaseConnection>,
    ValidatedJson(data): ValidatedJson<ApplicationIn>,
    permissions: Permissions,
) -> Result<(StatusCode, Json<ApplicationOut>)> {
    let app = application::ActiveModel {
        org_id: Set(permissions.org_id.clone()),
        ..data.into()
    };
    let ret = app.insert(db).await?;
    Ok((StatusCode::CREATED, Json(ret.into())))
}

async fn get_application(
    Extension(ref db): Extension<DatabaseConnection>,
    Path(app_id): Path<ApplicationIdOrUid>,
    permissions: Permissions,
) -> Result<Json<ApplicationOut>> {
    let app = application::Entity::secure_find_by_id_or_uid(permissions.org_id, app_id)
        .one(db)
        .await?
        .ok_or_else(|| HttpError::not_found(None, None))?;
    Ok(Json(app.into()))
}

async fn update_application(
    Extension(ref db): Extension<DatabaseConnection>,
    Path(app_id): Path<ApplicationIdOrUid>,
    ValidatedJson(data): ValidatedJson<ApplicationIn>,
    permissions: Permissions,
) -> Result<Json<ApplicationOut>> {
    let app = application::Entity::secure_find_by_id_or_uid(permissions.org_id.clone(), app_id)
        .one(db)
        .await?
        .ok_or_else(|| HttpError::not_found(None, None))?;

    let mut app: application::ActiveModel = app.into();
    data.update_model(&mut app);

    let ret = app.update(db).await?;
    Ok(Json(ret.into()))
}

async fn delete_application(
    Extension(ref db): Extension<DatabaseConnection>,
    Path(app_id): Path<ApplicationIdOrUid>,
    permissions: Permissions,
) -> Result<(StatusCode, Json<EmptyResponse>)> {
    let app = application::Entity::secure_find_by_id_or_uid(permissions.org_id, app_id)
        .one(db)
        .await?
        .ok_or_else(|| HttpError::not_found(None, None))?;

    let mut app: application::ActiveModel = app.into();
    app.deleted = Set(true);
    app.uid = Set(None); // We don't want deleted UIDs to clash
    app.update(db).await?;
    Ok((StatusCode::NO_CONTENT, Json(EmptyResponse {})))
}

pub fn router() -> Router {
    Router::new()
        .route("/app/", get(list_applications))
        .route("/app/", post(create_application))
        .route(
            "/app/:app_id/",
            get(get_application)
                .put(update_application)
                .delete(delete_application),
        )
}

#[cfg(test)]
mod tests {
    use reqwest::StatusCode;

    use super::{ApplicationIn, ApplicationOut};
    use crate::{
        test_util::{start_svix_server, IgnoredResponse},
        v1::utils::ListResponse,
    };

    fn application_in(name: &str) -> ApplicationIn {
        ApplicationIn {
            name: name.to_owned(),
            ..Default::default()
        }
    }

    #[tokio::test]
    async fn test_crud() {
        let (client, _jh) = start_svix_server();

        const APP_NAME_1_1: &str = "v1ApplicationCrudTest11";
        const APP_NAME_1_2: &str = "v1ApplicationCrudTest12";
        const APP_NAME_2_1: &str = "v1ApplicationCrudTest21";
        const APP_NAME_2_2: &str = "v1ApplicationCrudTest22";

        // CREATE
        let app_1: ApplicationOut = client
            .post(
                "api/v1/app/",
                application_in(APP_NAME_1_1),
                StatusCode::CREATED,
            )
            .await
            .unwrap();
        assert_eq!(app_1.name, APP_NAME_1_1);

        let app_2: ApplicationOut = client
            .post(
                "api/v1/app/",
                application_in(APP_NAME_2_1),
                StatusCode::CREATED,
            )
            .await
            .unwrap();
        assert_eq!(app_2.name, APP_NAME_2_1);

        // READ
        assert_eq!(
            client
                .get::<ApplicationOut>(&format!("api/v1/app/{}/", app_1.id), StatusCode::OK)
                .await
                .unwrap(),
            app_1
        );

        assert_eq!(
            client
                .get::<ApplicationOut>(&format!("api/v1/app/{}/", app_2.id), StatusCode::OK,)
                .await
                .unwrap(),
            app_2
        );

        // UPDATE
        let app_1_id = app_1.id;
        let app_1: ApplicationOut = client
            .put(
                &format!("api/v1/app/{}", app_1_id),
                application_in(APP_NAME_1_2),
                StatusCode::OK,
            )
            .await
            .unwrap();

        let app_2_id = app_2.id;
        let app_2: ApplicationOut = client
            .put(
                &format!("api/v1/app/{}", app_2_id),
                application_in(APP_NAME_2_2),
                StatusCode::OK,
            )
            .await
            .unwrap();

        // CONFIRM UPDATE
        assert_eq!(
            client
                .get::<ApplicationOut>(&format!("api/v1/app/{}/", app_1_id), StatusCode::OK,)
                .await
                .unwrap(),
            app_1
        );

        assert_eq!(
            client
                .get::<ApplicationOut>(&format!("api/v1/app/{}/", app_2_id), StatusCode::OK,)
                .await
                .unwrap(),
            app_2
        );

        // DELETE
        let _: IgnoredResponse = client
            .delete(&format!("api/v1/app/{}/", app_1.id), StatusCode::NO_CONTENT)
            .await
            .unwrap();
        let _: IgnoredResponse = client
            .delete(&format!("api/v1/app/{}/", app_2.id), StatusCode::NO_CONTENT)
            .await
            .unwrap();

        // CONFIRM DELETION
        let _: IgnoredResponse = client
            .get(&format!("api/v1/app/{}/", app_1.id), StatusCode::NOT_FOUND)
            .await
            .unwrap();
        let _: IgnoredResponse = client
            .get(&format!("api/v1/app/{}/", app_2.id), StatusCode::NOT_FOUND)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_list() {
        let (client, _jh) = start_svix_server();

        const APP_NAME_1: &str = "v1ApplicationCrudTest1";
        const APP_NAME_2: &str = "v1ApplicationCrudTest2";

        // CREATE
        let app_1 = client
            .post(
                "api/v1/app/",
                application_in(APP_NAME_1),
                StatusCode::CREATED,
            )
            .await
            .unwrap();

        let app_2 = client
            .post(
                "api/v1/app/",
                application_in(APP_NAME_2),
                StatusCode::CREATED,
            )
            .await
            .unwrap();

        let list = client
            .get::<ListResponse<ApplicationOut>>("api/v1/app/", StatusCode::OK)
            .await
            .unwrap();

        assert_eq!(list.data.len(), 2);
        assert!(list.data.contains(&app_1));
        assert!(list.data.contains(&app_2));
    }
}
