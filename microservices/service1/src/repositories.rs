use crate::schema::lekovi::dsl::*;
use crate::schema::lekovi::name;
use crate::{
    models::{Lek, NoviLek},
    schema::lekovi,
};
use diesel::prelude::*;
use diesel::{PgConnection, QueryResult, result::Error};
use rabbitmq_stream_client::error::{ProducerPublishError, StreamCreateError};
use rabbitmq_stream_client::types::{ByteCapacity, Message, ResponseCode};
use rabbitmq_stream_client::{ConfirmationStatus, Environment};
use serde_json::json;
pub struct LekRepository;

impl LekRepository {
    pub fn create_lek(connection: &mut PgConnection, novi_lek: NoviLek) -> QueryResult<Lek> {
        let result: Result<Lek, Error> = diesel::insert_into(lekovi::table)
            .values(novi_lek)
            .get_result(connection);

        match result {
            Ok(res) => {
                println!("Uspesno upisan lek u bazi!");
                return Ok(res);
            }
            Err(e) => return Err(e),
        }
    }

    pub fn find_lek(connection: &mut PgConnection, naziv_leka: String) -> QueryResult<Lek> {
        diesel::QueryDsl::filter(lekovi, name.eq(naziv_leka)).first::<Lek>(connection)
    }

    pub async fn send_to_rabbit(lek: &Lek) -> Result<ConfirmationStatus, ProducerPublishError> {
        let environment = Environment::builder().host("rabbitmq").port(5552).build().await.expect("Could not connect to Rabbit");

        let stream_name = "Petar-Komatina-Stream";
        let create_response = environment.stream_creator().max_length(ByteCapacity::GB(5)).create(stream_name).await;

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

        let body = json!({
            "event" : "Novi lek",
            "data" : lek.name.clone()
        });
        let body = serde_json::to_vec(&body).unwrap();
        let producer = environment
            .producer()
            .build(stream_name)
            .await
            .expect("Could not create producer");
        let status = producer
            .send_with_confirm(Message::builder().body(body).build())
            .await?;
        let _ = producer.close().await;
        drop(environment);
        Ok(status)
    }
}
