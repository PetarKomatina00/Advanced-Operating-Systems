use std::time::Duration;
use tokio::time::sleep;

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use rocket_sync_db_pools::database;

#[macro_use]
extern crate rocket;

pub mod models;
pub mod schema;
pub mod endpoints;
pub mod repositories;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

#[database("postgres")]
pub struct DbConnection(diesel::PgConnection);

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let rocket = rocket::build()
        .attach(DbConnection::fairing())
        .mount(
            "/",
            routes![
                endpoints::create_lekovi,
                endpoints::get_lek
            ],
        )
        .ignite()
        .await?;

    let mut connected = false;
    for i in 1..=100 {
        if let Some(conn) = DbConnection::get_one(&rocket).await {
            println!(" DB konekcija uspešno postavljena (pokušaj #{})", i);
            conn.run(|c| {
                if let Err(e) = c.run_pending_migrations(MIGRATIONS) {
                    eprintln!("Greška pri migracijama: {:?}", e);
                } else {
                    println!("Migracije uspešno postavljene!");
                }
            })
            .await;
            connected = true;
            break;
        } else {
            eprintln!("Baza još nije spremna (pokušaj #{})", i);
            sleep(Duration::from_secs(5)).await;
        }
    }

    if !connected {
        panic!("Nije moguće dobiti konekciju ni posle 100 pokušaja!");
    }

    rocket.launch().await?;
    Ok(())
}
