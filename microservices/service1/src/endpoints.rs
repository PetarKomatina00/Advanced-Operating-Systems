use crate::repositories::LekRepository;
use rocket::serde::json::serde_json::json;
use rocket::{
    http::Status,
    post,
    serde::json::Json,
};

use crate::{
    DbConnection,
    models::{Lek, NoviLek},
};

#[get("/lek?<naziv>")]
pub async fn get_lek(
    connection: DbConnection,
    naziv: &str,
) -> Result<rocket::serde::json::Value, Status> {
    let naziv = naziv.to_string();
    connection.run(move |c| LekRepository::find_lek(c, naziv).map(|lek: Lek| json!(lek)))
        .await
        .map_err(|_| Status::NotFound)
}
#[post("/list", data = "<lek>")]
pub async fn create_lekovi(connection: DbConnection,lek: Json<NoviLek>) -> Result<rocket::serde::json::Value, Status> {
    let lek = lek.into_inner();
    let result = connection
        .run(move |c| LekRepository::create_lek(c, lek))
        .await
        .map_err(|_| Status::InternalServerError);

    let result_clone = result.clone();
    match result{
        Ok(lek) => {
            println!("Uspesno upisan lek u bazi!");
            tokio::spawn(async move {
                if let Err(e) = LekRepository::send_to_rabbit(&lek).await{
                    eprintln!("Could not send to rabbit :{:?}", e);
                }
            });
            
        }
        Err(e) => {
            eprintln!("Greska: {}", e);
        }
    }
    match result_clone{
        Ok(result) => {
            Ok(json!(result))
        }
        Err(e) => {
            Err(e)
        }
    }

}
