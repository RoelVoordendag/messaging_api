use crate::AppState;
use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use entity::users::Entity as UserLoader;
use entity::users::{self, Model};
use entity::{rooms, rooms_users};
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveValue, JsonValue, QueryFilter,
    QuerySelect, RelationTrait,
};
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct User {
    name: String,
}

pub async fn create_user(
    app_state: web::Data<AppState>,
    request_data: web::Json<User>,
) -> impl Responder {
    let database_connection = &app_state.database_connection;

    let existing_user: Option<users::Model> = UserLoader::find()
        .filter(users::Column::Name.contains(request_data.name.to_owned()))
        .one(database_connection)
        .await
        .expect("Something went wrong collecting data");

    if existing_user != None {
        return HttpResponse::NotAcceptable().body("User already exists");
    }

    let user = users::ActiveModel {
        name: Set(request_data.name.to_owned()),
        created_at: Set(Utc::now().naive_utc()),
        id: Uuid::new_v4().into_active_value(),
        ..Default::default()
    };

    user.insert(database_connection)
        .await
        .expect("Something went wrong with creation user");

    return HttpResponse::Ok().body("Created new user");
}

pub async fn get_or_create_user(
    app_state: web::Data<AppState>,
    request_data: web::Json<User>,
) -> impl Responder {
    let db_connection = &app_state.database_connection;

    // @todo How do we handle joins that result in no results? do we filter later or not
    // with this
    // We need to add a todo a rooms object to make filtering for the FE easier
    let exisiting_user: Option<(JsonValue, std::option::Option<JsonValue>)> = UserLoader::find()
        .filter(users::Column::Name.eq(request_data.name.to_owned()))
        .join(
            sea_orm::JoinType::LeftJoin,
            users::Relation::RoomsUsers.def(),
        )
        .join(
            sea_orm::JoinType::LeftJoin,
            rooms_users::Relation::Rooms.def(),
        )
        .select_also(rooms::Entity)
        .into_json()
        .one(db_connection)
        .await
        .expect("Something went wrong querying the database.");

    if exisiting_user != None {
        // @todo we need to add rooms if users has them
        return HttpResponse::Ok().json(exisiting_user);
    }

    let user = users::ActiveModel {
        name: Set(request_data.name.to_owned()),
        created_at: Set(Utc::now().naive_utc()),
        id: Uuid::new_v4().into_active_value(),
        ..Default::default()
    };

    let inserted_user: Model = user
        .insert(db_connection)
        .await
        .expect("Something went wrong with creation user");

    return HttpResponse::Ok().json(json!(inserted_user));
}
