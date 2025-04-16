pub mod network;

#[cfg(test)]
mod tests {
    use crate::network::init;
    use crate::network::client;
    use crate::network::server;

    #[test]
    fn it_works() {
        init();
        client::connect();
    }

    #[test]
    fn test_init() {
        init();
    }

    #[test]
    fn test_client_connect() {
        client::connect();
    }

    #[test]
    fn test_server_connect() {
        server::connect();
    }
}
