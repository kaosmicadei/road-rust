fn sieve(limit: usize) -> Vec<usize> {
  let mut sieve = vec![true; limit / 2]; // Only store odd numbers
  let cross_limit = (limit as f64).sqrt() as usize;

  for i in 1..cross_limit / 2 { // Skip even indices
    if sieve[i] {
      for idx in ((2*i*(i+1))..limit / 2).step_by(2*i+1) { 
        sieve[idx] = false;
      }
    }
  }

  let mut primes = vec![2]; // Include 2 separately
  primes
    .extend((1..limit / 2)
    .filter(|&i| sieve[i])
    .map(|i| 2 * i + 1));
  primes
}

fn segmented_sieve(limit: usize, segment_size: usize) -> Vec<usize> {
  let sqrt_limit = (limit as f64).sqrt() as usize;
  let small_primes = sieve(sqrt_limit);
  let mut primes = small_primes.clone();
  
  let mut low = sqrt_limit + 1;
  let mut high = low + segment_size;

  while low < limit {
    if high > limit { high = limit; }

    let mut is_prime = vec![true; (high - low) / 2 + 1]; // Store only odds

    for &p in &small_primes {
      let mut start = (low / p) * p;
      if start < low { start += p; }
      if start % 2 == 0 { start += p; }
      
      for multiple in (start..=high).step_by(2 * p) { 
        is_prime[(multiple - low) / 2] = false;
      }
    }

    for (i, &prime) in is_prime.iter().enumerate() {
      if prime {
        primes.push(low + 2 * i);
      }
    }

    low += segment_size;
    high += segment_size;
  }

  primes
}

fn nth_prime(n: usize, limit: usize, segment_size: usize) -> usize {
  let primes = segmented_sieve(limit, segment_size);
  primes.len()
}

fn main() {
  let n = 10_001;
  let limit = 200_000;
  let segment_size = 10_000;

  println!("The {}th prime is {}", n, nth_prime(n, limit, segment_size));
}
