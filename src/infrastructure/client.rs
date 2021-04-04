use reqwest::{Url, Error};
use crate::domains::ticker::TickerMain;




pub async fn get_value_ticket_last_day(ticket: &str) -> Result<TickerMain, Error>{
    let url : Url =  format!("https://www.mercadobitcoin.net/api/{}/ticker/", ticket).parse().unwrap();
    reqwest::get(url).await.unwrap().json::<TickerMain>().await
}

