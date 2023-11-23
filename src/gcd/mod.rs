mod server;
mod console;

pub enum Mode {
  Console(Vec<String>),
  Server,
}

pub async fn run(mode: Mode) {
  match mode {
    Mode::Console(args) => async { console::run(args) }.await,
    Mode::Server => server::run().await
  }
}

fn gcd(m: u64, n: u64) -> u64 {
  assert!(n != 0 && m != 0);

  fn f(m: u64, n: u64) -> u64 {
    if m == 0 { n } else {
      if m < n { f(n % m, m) } else { f(m % n, n) }
    }
  }

  f(m, n)
}

#[test]
fn test_gcd() {
  assert_eq!(gcd(14, 15), 1);

  assert_eq!(
    gcd(
      2 * 3 * 5 * 11 * 17,
      3 * 7 * 11 * 13 * 19,
    ),
    3 * 11
  )
}
