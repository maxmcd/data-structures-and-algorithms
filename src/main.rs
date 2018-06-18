extern crate rand;

mod bubble;
mod insertion;
mod selection;

use rand::Rng;
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

fn random_array(size: usize) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    (0..size).map(|_| rng.gen_range(0, 255)).collect()
}

macro_rules! gen_tests {
    ($lib:ident, $name:tt) => {
        #[test]
        fn $lib() {
            for i in 2..5 {
                let dur = assert_eq(&mut random_array(10usize.pow(i)), &$lib::sort);
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

    gen_tests!(insertion, "Insertion");
    gen_tests!(selection, "Selection");
    gen_tests!(bubble, "Bubble\t");

}
