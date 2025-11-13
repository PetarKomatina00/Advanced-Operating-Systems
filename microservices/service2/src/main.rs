use std::time::Duration;

use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use rocket_sync_db_pools::database;
use tokio::time::sleep;

use crate::repositories::EventRepository;

#[macro_use]
extern crate rocket;

pub mod endpoints;
pub mod models;
pub mod repositories;
pub mod schema;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

#[database("postgres")]
pub struct DbConnection(diesel::PgConnection);

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    println!("Service2 is running...");
    tokio::spawn(async {
        EventRepository::receive_from_rabbit().await;
    });

    let rocket = rocket::build()
        .attach(DbConnection::fairing())
        .mount("/", routes![
            endpoints::get_lekovi_analytics
        ])
        .ignite()
        .await?;

    let mut connected = false;
    for i in 1..=100 {
        if let Some(conn) = DbConnection::get_one(&rocket).await {
            println!("Service2: DB konekcija uspešno postavljena (pokušaj #{})", i);
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
