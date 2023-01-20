
use actix_web::{web, Responder};
use serde::{Serialize, Deserialize};
use actix_web::http::StatusCode;
use crate::services::{Actions, Money};



#[derive(Deserialize, Serialize)]
struct Info {
    money: Option<String>
}

async fn get_price_crypt_money(query: web::Query<Info>) ->impl Responder {


    let list_ticker_day = match query.0.money {
        Some(query)=> {
             Money::get_value_ticker(query).await
        }, None => {
            Money::get_all_value_last_day_ticker().await
        }
    };

    web::Json(
        list_ticker_day
    ).with_status(StatusCode::OK)

}



   pub fn config(cfg: &mut web::ServiceConfig) {
       cfg.service(
           web::scope("/v1")
               .service(
               web::resource("/money")
                   .route(web::get().to(get_price_crypt_money))
           )

       );
   }