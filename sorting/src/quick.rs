pub fn sort(mut list: &mut [u8]) {
    if list.len() <= 1 {
        return;
    }
    let pivot = partition(&mut list);
    sort(&mut list[0..pivot + 1]);
    sort(&mut list[pivot + 1..]);
}

pub fn partial(mut list: &mut [u8], k: usize) {
    if list.len() <= 1 {
        return;
    }
    let pivot = partition(&mut list);
    partial(&mut list[0..pivot + 1], k);
    if pivot < k {
        partial(&mut list[pivot + 1..], k);
    }
}

// "media-of-three" pivot strategy
fn find_pivot(list: &mut [u8]) -> u8 {
    let hi = list.len() - 1;
    let mid = (0 + hi) / 2;
    if list[hi] < list[0] {
        list.swap(0, hi);
    }
    if list[mid] < list[0] {
        list.swap(0, mid);
    }
    if list[hi] < list[mid] {
        list.swap(mid, hi);
    }
    list[mid]
}

fn partition(mut list: &mut [u8]) -> usize {
    let pivot = find_pivot(&mut list);
    let mut low = 0;
    let mut hi = list.len() - 1;
    while hi >= low {
        while list[low] < pivot {
            low += 1;
        }
        while list[hi] > pivot {
            hi = hi.saturating_sub(1);
        }
        if hi >= low {
            list.swap(hi, low);
            low += 1;
            hi = hi.saturating_sub(1);
        }
    }
    return hi;
}

// mod tests {
//     use super::*;
//     use common;

//     // #[test]
//     // fn quicksort_large() {
//     //     sort(&mut common::random_array(100_000_000));
//     // }

//     // #[test]
//     // fn part() {
//     //     partition(&mut [9, 9, 9, 1, 1, 1, 1]);
//     //     partition(&mut [5, 8, 7, 5, 4, 4, 3, 2, 9]);
//     //     partition(&mut [209, 97, 224, 26, 91, 5, 195, 251, 238, 30]);
//     // }

//     // proptest! {
//     //     #[test]
//     //     fn doesnt_crash(a in 10..11usize) {
//     //         assert_accurate_partition(a);
//     //     }
//     // }

// }
