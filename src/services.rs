use async_trait::async_trait;
use crate::infrastructure::client::get_value_ticket_last_day;
use crate::domains::ticker::TickerMain;
use futures::{self, StreamExt};
use crate::helpers::load_env::{Environment, Actions as ActionsEnv};


pub struct Money;

#[async_trait]
pub trait Actions {
   async fn get_all_value_last_day_ticker()->Vec<TickerMain>;
   async fn get_value_ticker(ticker: String) -> Vec<TickerMain>;
}


#[async_trait]
impl Actions for Money {

    async fn get_all_value_last_day_ticker()-> Vec<TickerMain>{

        let env_value: Environment = ActionsEnv::new("TICKER") ;
        let list_ticker = env_value.to_list_string();
        let prices_ticket_last_day: Vec<TickerMain> = futures::stream::iter(list_ticker)
            .filter_map(|ticker|  async move {
                match  get_value_ticket_last_day(ticker).await {
                    Ok(x) => Some(x),
                    Err(err) => {
                        println!("log aqui {:?}", err);
                        None
                    }
                }
            })
            .collect::<Vec<TickerMain>>().await;
        prices_ticket_last_day
    }

    async fn get_value_ticker(ticker: String) -> Vec<TickerMain> {
        let mut list_ticker: Vec<TickerMain> = Vec::with_capacity(1);
        match get_value_ticket_last_day(ticker.as_str()).await {
            Ok(ticker) => {
                list_ticker.push(ticker);

            },
            Err(err) => {
                println!("log aqui {:?}", err);
            }
        };
        list_ticker

    }
}