use std::time::Duration;

use crate::{
    models::{Events, NewEvent},
    schema::events::{self},
};
use diesel::prelude::*;
use diesel::{PgConnection, result::Error};
use rabbitmq_stream_client::{
    Environment,
    error::StreamCreateError,
    types::{ByteCapacity, OffsetSpecification, ResponseCode},
};
use rocket::futures::StreamExt;
use serde_json::Value;
use tokio::{task, time::sleep};
pub struct EventRepository;

impl EventRepository {

    async fn connect_to_rabbit() -> Result<Environment, String>{
        for i in 1..=10{
            match Environment::builder().host("rabbitmq").port(5552).build().await {
                Ok(env) => {
                    println!("You can now listen to Rabbitmq. Attempt: {}", i);
                    return Ok(env);
                }
                Err(e) => {
                    eprintln!("Attempt {}. Error:{:?}", i, e);
                    sleep(Duration::from_secs(3)).await;
                }
            }
        } 
        return Err(String::from("Could not connect"));
    }
    pub async fn receive_from_rabbit() {
        let environment = EventRepository::connect_to_rabbit().await;
        if !environment.is_ok(){
            return;
        }
        let environment = environment.unwrap();
        let stream_name = "Petar-Komatina-Stream";
        let create_response = environment
            .stream_creator()
            .max_length(ByteCapacity::GB(5))
            .create(stream_name)
            .await;

        if let Err(e) = create_response {
            if let StreamCreateError::Create { stream, status } = e {
                match status {
                    ResponseCode::StreamAlreadyExists => {}
                    err => {
                        println!("Error creating stream {:?} {:?}", stream, err);
                    }
                }
            }
        }
        let mut consumer: rabbitmq_stream_client::Consumer = environment
            .consumer()
            .offset(OffsetSpecification::First)
            .build(stream_name)
            .await
            .expect("Could not create consumer");

        task::spawn(async move {
            while let Some(poruka) = consumer.next().await {
                let p: rabbitmq_stream_client::types::Delivery = poruka.unwrap();
                println!(
                    "Dobili smo novu poruku!: {:?} sa offsetom {:?}",
                    p.message()
                        .data()
                        .map(|data| String::from_utf8(data.to_vec()).unwrap()),
                    p.offset(),
                );
                if let Some(bytes) = p.message().data() {
                    let str = String::from_utf8(bytes.to_vec())
                        .expect("Greska prilikom konvertovanja u String");
                    let json_str = serde_json::from_str::<Value>(&str);
                    match json_str {
                        Ok(value) => {
                            let event = value["event"].as_str().unwrap_or("Greska");
                            let name = value["data"].as_str().unwrap_or("Greska");

                            let newevent = NewEvent {
                                event_type: event.to_string(),
                                name: name.to_string(),
                            };

                            let database_url =
                                std::env::var("DATABASE_URL").expect("DatabaseURL nije postavljen");
                            let mut connection = PgConnection::establish(&database_url)
                                .expect("Neuspesno povezivanje sa bazom");

                            let result = EventRepository::create_event(&mut connection, newevent);
                            match result{
                                Ok(_event) => {
                                    println!("Uspesno kreiran event!");
                                },
                                Err(e) =>{
                                    println!("Greska kod kreiranja event-a: {:?}", e);
                                }
                            }
                        }
                        Err(e) => {
                            println!("Nesto nije proslo po redu prilikom parsiranja: {:?}", e);
                        }
                    }
                }
            }
            drop(consumer);
        });
        drop(environment);
        
    }

    pub fn get_events_from_db(
        connection: &mut PgConnection,
        limit: i64,
    ) -> QueryResult<Vec<Events>> {
        events::table
            .limit(limit)
            .order(events::id.desc())
            .load::<Events>(connection)
    }

    pub fn create_event(connection: &mut PgConnection, event: NewEvent) -> Result<Events, Error>{
        let result: Result<Events, Error> = diesel::insert_into(events::table)
            .values(event)
            .get_result(connection);

        match result {
            Ok(event) => {
                println!("Uspesno upisan event u bazi!");
                return Ok(event);
            }
            Err(e) => {
                return Err(e);
            }
        }
    }
}
