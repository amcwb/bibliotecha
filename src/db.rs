use mongodb::{bson::doc, sync::Client};

pub fn get_client_sync() -> mongodb::error::Result<Client> {
    dotenv::dotenv().ok();

    let database_url = &std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        panic!("No MongoDB url defined in DATABASE_URL")
    });

    let client = Client::with_uri_str(database_url)
        .unwrap_or_else(|_| {
            panic!("MongoDB must be available, from environment variable DATABASE_URL");
        });

    client.database("admin")
        .run_command(doc! {"ping": 1}, None)
        .unwrap_or_else(|_| {
            panic!("MongoDB {} is not accessible", database_url)
        });

    Ok(client)
}