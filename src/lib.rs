pub mod tda_broker;
pub mod tda_client;

#[cfg(test)]
mod tests {
    use traders::{broker::Broker, Rational64};

    use crate::{
        tda_broker::TdaBroker,
        tda_client::{self, Client},
    };

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[tokio::test]
    async fn test_auth() {
        let mut client = tda_client::Client::new();
        client.auth();
        client.update_access_token().await;
        assert!(client.refresh_token.len() > 5);
    }

    #[test]
    fn test_broker() {
        let broker = TdaBroker::new();
        assert!(broker.get_cash() > Rational64::new(100, 1));
    }

    #[tokio::test]
    async fn test_client() {
        let mut client = Client::new();
        let _ = client.update_access_token();
        assert!(client.get_cash().await > Rational64::new(100, 1));
    }
}
