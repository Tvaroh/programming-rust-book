use std::env::args;
use std::process::exit;

use crate::gcd::Mode;

mod gcd;
mod mandelbrot;

#[tokio::main]
async fn main() {
  let args: Vec<String> = args().collect();
  let module_args: Vec<String> = args.iter().skip(2).cloned().collect();

  match args[1].to_lowercase().as_str() {
    "gcd_console" => gcd::run(Mode::Console(module_args)).await,
    "gcd_server" => gcd::run(Mode::Server).await,

    "mandelbrot" => mandelbrot::run(module_args, false),
    "mandelbrot_par" => mandelbrot::run(module_args, true),

    _ => {
      eprintln!("Usage: {} MODULE ARGS...", args[0]);

      exit(0)
    }
  }
}
