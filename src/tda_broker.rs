use chrono::{DateTime, Duration, Utc};
use tokio::runtime::Runtime;
use traders::{broker::Broker, Rational64};

use crate::tda_client::Client;

#[derive(Debug)]
pub struct TdaBroker {
    client: Client,
    last_updated: Option<DateTime<Utc>>,
}

impl TdaBroker {
    async fn async_update(&mut self) {
        self.client.update_access_token().await;
        self.last_updated = Some(Utc::now());
    }

    fn sync_update(&mut self) {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            self.async_update().await;
        })
    }

    fn maybe_update(&mut self) {
        if self.last_updated.is_none()
            || self.last_updated.unwrap() + Duration::minutes(30) < Utc::now()
        {
            self.sync_update();
        }
    }
}

impl Broker for TdaBroker {
    fn new() -> Self {
        let mut broker = TdaBroker {
            client: Client::new(),
            last_updated: None,
        };
        broker.client.auth();
        broker.maybe_update();
        broker
    }

    fn get_cash(&self) -> Rational64 {
        //let rt = tokio::runtime::Runtime::new().unwrap();
        //rt.block_on(async {self.client.get_cash().await})
        Rational64::new(100, 100)
    }

    fn get_positions(&self) -> &std::collections::HashSet<traders::broker::Position> {
        todo!()
    }

    fn get_orders(&self) -> &[traders::broker::Order] {
        todo!()
    }

    fn new_order(&mut self, o: traders::broker::Order) {
        todo!()
    }
}
