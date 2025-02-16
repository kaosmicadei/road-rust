use std::time::Instant;

fn sieve_original(limit: usize) -> Vec<bool> {
  let mut sieve = vec![true; limit / 2]; // Only store odd numbers
  sieve[0] = false;
  let cross_limit = (limit as f64).sqrt() as usize;

  for i in 1..cross_limit / 2 {
    if sieve[i] {
      let p = 2 * i + 1;
      for multiple in ((p * p)..=limit / 2).step_by(2 * p) {
        sieve[multiple / 2] = false;
      }
    }
  }
  sieve
}

/*
  Here use that p(i) = 2i+1 implies that
      p(i)^2 = 4i^2 + 4i + 1 = 2(2*i*(i+1)) + 1 = p(2*i*(i+1))
  and, considering only integer divisions,
      p(2*i*(i+1)) / 2 = 2*i*(i+1)
  This version runs considerable fast than the original.
*/
fn sieve_optimized(limit: usize) -> Vec<bool> {
  let mut sieve = vec![true; limit / 2]; // Only store odd numbers
  sieve[0] = false;
  let cross_limit = (limit as f64).sqrt() as usize;

  for i in 1..cross_limit / 2 {
    if sieve[i] {
      for half_multiple in ((2 * i * (i + 1))..limit / 2).step_by(2 * i + 1) {
        sieve[half_multiple] = false;
      }
    }
  }
  sieve
}

fn benchmark(f: fn(usize) -> Vec<bool>, limit: usize) -> f64 {
  let start = Instant::now();
  f(limit);
  start.elapsed().as_secs_f64()
}

fn main() {
  let limit = 2_000_000;
  let runs = 10;

  let original_time: f64 = (0..runs).map(|_| benchmark(sieve_original, limit)).sum::<f64>() / runs as f64;
  let optimized_time: f64 = (0..runs).map(|_| benchmark(sieve_optimized, limit)).sum::<f64>() / runs as f64;

  println!("Original sieve avg time: {:.6} s", original_time);
  println!("Optimized sieve avg time: {:.6} s", optimized_time);
}
