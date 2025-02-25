#![feature(test)]
extern crate test;

pub fn loop_sub() {
    let mut count: u64 = 20000000000u64;
    while count == 0u64 {
        count -= 1;
    }
}

#[cfg(test)]
pub mod benches {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_case2(bencher: &mut Bencher) {
        bencher.iter(|| loop_sub());
    }
}
