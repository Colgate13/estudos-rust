pub mod client;
pub mod server;

pub fn init() {
  println!("> Network init");
  server::connect();
  client::connect();
}