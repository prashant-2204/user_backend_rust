use actix_web::{web, HttpResponse, Responder};
use mongodb::{bson::doc, options::FindOptions};
use crate::models::user::User;
use crate::db::DB;
use futures_util::stream::StreamExt;
use std::sync::Arc; // Import Arc

pub async fn create_user(
    user: web::Json<User>,
    db: web::Data<Arc<DB>>, // Update to Arc<DB>
) -> impl Responder {
    let user = User {
        id: None,
        name: user.name.clone(),
        email: user.email.clone(),
        age: user.age,
    };
    
    match db.user_collection.insert_one(user.clone(), None).await {
        Ok(_) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn get_users(db: web::Data<Arc<DB>>) -> impl Responder { // Update to Arc<DB>
    let options = FindOptions::builder().limit(10).build();
    match db.user_collection.find(None, options).await {
        Ok(mut cursor) => {
            let users: Vec<User> = cursor
                .collect::<Vec<_>>()
                .await
                .into_iter()
                .filter_map(|res| res.ok())
                .collect();
            HttpResponse::Ok().json(users)
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn get_user(
    path: web::Path<String>,
    db: web::Data<Arc<DB>>, // Update to Arc<DB>
) -> impl Responder {
    let id = match mongodb::bson::oid::ObjectId::parse_str(&path.into_inner()) {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().body("Invalid ID"),
    };

    match db.user_collection.find_one(doc! {"_id": id}, None).await {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().body("User not found"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn update_user(
    path: web::Path<String>,
    user: web::Json<User>,
    db: web::Data<Arc<DB>>, // Update to Arc<DB>
) -> impl Responder {
    let id = match mongodb::bson::oid::ObjectId::parse_str(&path.into_inner()) {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().body("Invalid ID"),
    };

    let update = doc! {
        "$set": {
            "name": user.name.clone(),
            "email": user.email.clone(),
            "age": user.age
        }
    };

    match db.user_collection.update_one(doc! {"_id": id}, update, None).await {
        Ok(result) if result.matched_count > 0 => HttpResponse::Ok().json(user),
        Ok(_) => HttpResponse::NotFound().body("User not found"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn delete_user(
    path: web::Path<String>,
    db: web::Data<Arc<DB>>, // Update to Arc<DB>
) -> impl Responder {
    let id = match mongodb::bson::oid::ObjectId::parse_str(&path.into_inner()) {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().body("Invalid ID"),
    };

    match db.user_collection.delete_one(doc! {"_id": id}, None).await {
        Ok(result) if result.deleted_count > 0 => HttpResponse::Ok().body("User deleted"),
        Ok(_) => HttpResponse::NotFound().body("User not found"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}