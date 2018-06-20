#[macro_use]
#[cfg(test)]
extern crate proptest;
extern crate rand;

pub mod bubble;
pub mod common;
pub mod heap;
pub mod insertion;
pub mod merge;
pub mod quick;
pub mod selection;

#[cfg(test)]
mod tests {

    use super::*;
    use rand;
    use rand::Rng;
    use std::time::{Duration, Instant};

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
            proptest! {
                #[test]
                fn $lib(a in 3..10000usize) {
                    assert_eq(&mut common::random_array(a), &$lib::sort);
                }
            }
        };
    }

    gen_test!(insertion, "Insertion");
    gen_test!(selection, "Selection");
    gen_test!(bubble, "Bubble\t");
    gen_test!(merge, "Merge\t");
    gen_test!(heap, "Heap\t");
    gen_test!(quick, "Quick\t");

    proptest! {
        #[test]
        fn partial_sort(a in 10..100usize) {
            let mut list = common::random_array(a);
            let mut rng = rand::thread_rng();
            let b = rng.gen_range(0, a);
            quick::partial(&mut list, b);
            let value = list[b];
            list.sort();
            assert_eq!(value, list[b]);
        }
    }

}
