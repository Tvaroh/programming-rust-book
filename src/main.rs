use crate::gcd::Mode;

mod gcd;

#[tokio::main]
async fn main() {
  gcd::run(Mode::Console).await;
  gcd::run(Mode::Server).await;
}
