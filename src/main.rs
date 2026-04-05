use actix_web::{web, App, HttpServer, Responder};


async fn hello() -> impl Responder {
    "Hello, World!"
}
#[warn(dead_code)]
struct Users{
    id :i32,
    name :String,
    email :String,
    password :String,
}

#[derive(serde::Deserialize)]
struct SignupData {
    id :i32,
    name:String,
    email:String,
    password:String,
    created_at:String,
    updated_at:String,
}
async fn signup(info: SignupData) -> impl Responder {
    // Here you would typically save the user data to a database
    format!("User {} signed up with email {}", info.name, info.email)
  
}




#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(hello))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}