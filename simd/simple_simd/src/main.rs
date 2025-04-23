#![feature(portable_simd)]
#![feature(slice_as_chunks)]
use std::simd::f32x4;

/// Add two arrays with the same length and put the result in the `out` variable.
fn add_simd(a: &[f32], b: &[f32], out: &mut [f32]) {
  assert_eq!(a.len(), b.len());
  assert_eq!(a.len(), out.len());

  const LANES: usize = f32x4::LEN;
  
  let (a_chunks, a_reminder) = a.as_chunks::<4>();
  let (b_chunks, b_reminder) = a.as_chunks::<4>();
  
  a_chunks.iter()
  .zip(b_chunks.iter())
  .map(|(xs, ys)| {
    let vx = f32x4::from_slice(xs);
    let vy = f32x4::from_slice(ys);
    vx + vy
  })
  .enumerate()
  .for_each(|(i, res)| {
    let j = i * LANES;
    res.copy_to_slice(&mut out[j..j+LANES]);
  });
  
  let simd_len: usize = a.len() / LANES * LANES;
  
  a_reminder.iter()
    .zip(b_reminder.iter())
    .map(|(x, y)| x + y)
    .enumerate()
    .for_each(|(i, res)|
      out[simd_len + i] = res
    );
}


fn main() {
  let a = f32x4::splat(0.5);
  let b = f32x4::from_array([1.0, 2.0, 3.0, 4.0]);

  let c = a + b;
  println!("{:?}", c);
  
  let a: [f32; 18] = core::array::from_fn(|i| i as f32);
  let b: [f32; 18] = core::array::from_fn(|i| i as f32);
  let mut d = [0.0; 18];
  add_simd(&a, &b, &mut d);
  println!("{:?}", d);
}
