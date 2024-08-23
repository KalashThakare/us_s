use actix_cors::Cors;
use actix_web::{cookie::Cookie, web, App, HttpResponse, HttpServer, Responder };
use dotenv::dotenv;
use mongodb::{bson::doc, Client, Collection};
use std::env;
use serde::{Deserialize, Serialize};
use futures::TryStreamExt;
use actix_web::web::Data;

#[derive(Serialize, Deserialize)]
struct User {
    name: String,
    username: String,
    email: String,
    password: String,
    phone: String,
}

#[derive(Serialize, Deserialize)]
struct UserLogin {
    email: String,
    password: String,
}

#[derive(Serialize, Deserialize)]

struct FileUpload {
    title: String,
    description: String,
    email: String,
    url: String
}

async fn register(
    user: web::Json<User>,
    user_collection:  Data<Collection<User>>
) -> impl Responder {
    let result = user_collection.insert_one(user.into_inner(), None).await;

    match result {
        Ok(_) => HttpResponse::Ok().body("User Registered Successfully"),
        Err(_) => HttpResponse::InternalServerError().body("Error registering user"),
    }
}

async fn login(
    user: web::Json<UserLogin>,
    user_collection: Data<Collection<User>>
) -> impl Responder {
    let filter =doc! { "email": &user.email, "password": &user.password }; 
    let result = user_collection.find_one(filter, None).await;

    match result {
        Ok(Some(_)) => {
            let cookie = Cookie::build("user_email", user.email.clone())
                .path("/")
                .http_only(true)
                .finish();
            HttpResponse::Ok()
                .cookie(cookie)
                .body(user.email.clone())
        },
        Ok(None) => HttpResponse::Unauthorized().body("Invalid Credentials"),
        Err(_) => HttpResponse::InternalServerError().body("Login Error"),
    }
}

async fn upload_file(
    file: web::Json<FileUpload>,
    file_collection: Data<Collection<FileUpload>>,
) -> impl Responder {
    let result = file_collection.insert_one(file.into_inner(), None).await;

    match result {
        Ok(_) => HttpResponse::Ok().body("File Uploaded Successfully"),
        Err(_) => HttpResponse::InternalServerError().body("Error uploading file"),
    }
}

async fn get_files(
    file_collection: Data<Collection<FileUpload>>
) -> impl Responder{
    let mut cursor = file_collection.find(None, None).await.unwrap();
    let mut uploads:Vec<FileUpload> = Vec::new();

    while let Some(result) = cursor.try_next().await.unwrap() {
        uploads.push(result);
    }
    HttpResponse::Ok().json(uploads)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let mongo_uri = env::var("MONGO_URI").expect("MONGO_URI must be set in .env file");

    let client = Client::with_uri_str(&mongo_uri)
        .await
        .expect("Failed to initialize MongoDB client");

    let db = client.database("rust_backend");
    let user_collection: Collection<User> = db.collection("users");
    let upload_collection: Collection<FileUpload> = db.collection("upload_file");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_header()
            .allow_any_method()
            .allow_any_origin()
            .supports_credentials()
            .max_age(3600);

        App::new()
            .app_data(Data::new(user_collection.clone()))
            .app_data(Data::new(upload_collection.clone()))
            .wrap(cors)
            .route("/register", web::post().to(register))
            .route("/login", web::post().to(login))
            .route("/upload", web::get().to(get_files))
            .route("/upload", web::post().to(upload_file))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}