extern crate rand;
mod insertion;
use rand::Rng;

fn main() {
    println!("hello");
}

fn assert_eq(list: &mut [u8], f: &Fn(&mut [u8])) {
    let mut dst = vec![0; list.len()];
    dst.copy_from_slice(list);
    dst.sort();
    f(list);
    assert_eq!(list[..], dst[..]);
}

fn random_array(size: usize) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    (0..size).map(|_| rng.gen_range(0, 255)).collect()
}

macro_rules! gen_tests {
    ($lib:ident) => {
        #[test]
        fn $lib() {
            assert_eq(&mut random_array(1000), &$lib::sort);
            assert_eq(&mut random_array(10), &$lib::sort);
            assert_eq(&mut random_array(10000), &$lib::sort);
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    gen_tests!(insertion);
}
