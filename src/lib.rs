pub mod tda_broker;
pub mod tda_client;

#[cfg(test)]
mod tests {
    use crate::tda_client;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[tokio::test]
    async fn test_server() {
        tda_client::Client::auth_server().await;
    }
}
