use mongodb::{Client, Collection};
use dotenv::dotenv;
use std::env;
use std::sync::Arc; // Import Arc
use crate::models::user::User;

#[derive(Clone)] // Add Clone derive since Arc<DB> will handle cloning
pub struct DB {
    pub user_collection: Collection<User>,
}

impl DB {
    pub async fn init() -> mongodb::error::Result<Arc<Self>> { // Return Arc<DB>
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let client = Client::with_uri_str(&database_url).await?;
        let db = client.database("user_db");
        let user_collection = db.collection::<User>("users");
        
        Ok(Arc::new(DB {
            user_collection
        }))
    }
}