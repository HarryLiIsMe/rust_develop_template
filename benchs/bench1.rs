#![feature(test)]
extern crate test;

#[cfg(test)]
pub mod benches
{
  use test::Bencher;
  pub fn loop_add()
  {
    let mut count: u64 = 1u64;
    while count < 20000000000u64
    {
      count += 1;
    }
  }

  #[bench]
  fn bench_case1(bencher: &mut Bencher)
  {
    bencher.iter(|| loop_add());
  }
}
