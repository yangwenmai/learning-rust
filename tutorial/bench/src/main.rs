//#![feature(test)]
//
//extern crate test;
//
//pub fn add_two(a: i32) -> i32 {
//    a + 2
//}
//
//#[cfg(test)]
//mod tests {
//    use super::*;
//    use test::Bencher;
//
//    #[test]
//    fn it_works() {
//        assert_eq!(4, add_two(2));
//    }
//
//    #[bench]
//    fn bench_add_two(b: &mut Bencher) {
//        b.iter(|| add_two(2));
//    }
//}

#![feature(test)]

extern crate test;
use test::Bencher;

#[bench]
fn bench_xor_1000_ints(b: &mut Bencher) {
    b.iter(|| {
        // use `test::black_box` to prevent compiler optimizations from disregarding
        // unused values
        test::black_box(range(0u, 1000).fold(0, |old, new| old ^ new));
    });
}