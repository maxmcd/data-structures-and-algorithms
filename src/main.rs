extern crate rand;
#[macro_use]
extern crate proptest;

mod bubble;
mod common;
mod heap;
mod insertion;
mod merge;
mod quick;
mod selection;

use std::time::{Duration, Instant};

fn main() {
    println!("hello");
}

fn assert_eq(list: &mut [u8], f: &Fn(&mut [u8])) -> Duration {
    let mut dst = vec![0; list.len()];
    dst.copy_from_slice(list);
    dst.sort();
    let now = Instant::now();
    f(list);
    let took = now.elapsed();
    assert_eq!(list[..], dst[..]);
    took
}

macro_rules! gen_test {
    ($lib:ident, $name:tt) => {
        #[test]
        fn $lib() {
            for i in 2..6 {
                let dur = assert_eq(&mut common::random_array(10usize.pow(i)), &$lib::sort);
                println!(
                    "{}\t {}\t took {}.{:#09} seconds",
                    $name,
                    10usize.pow(i),
                    dur.as_secs(),
                    dur.subsec_nanos()
                );
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    gen_test!(insertion, "Insertion");
    gen_test!(selection, "Selection");
    gen_test!(bubble, "Bubble\t");
    gen_test!(merge, "Merge\t");
    gen_test!(heap, "Heap\t");
    gen_test!(quick, "Quick\t");

}
