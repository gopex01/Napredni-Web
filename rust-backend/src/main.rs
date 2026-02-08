use axum::{routing::get, Router};
use sqlx::postgres::{PgPool, PgPoolOptions}; // Dodali smo PgPool
use dotenvy::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok(); 

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");

    // Kreiranje pool-a konekcija
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Ne mogu da se povežem na bazu podataka!");

    println!("✅ Uspešno povezan na Postgres bazu!");

    // OVDE JE PROMENA: Eksplicitno kažemo da Router koristi PgPool kao State
    let app: Router = Router::new()
        .route("/", get(|| async { "API radi i baza je povezana!" }))
        .with_state(pool);

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    
    println!("🚀 Server pokrenut na http://localhost:3000");
    
    axum::serve(listener, app).await.unwrap();
}