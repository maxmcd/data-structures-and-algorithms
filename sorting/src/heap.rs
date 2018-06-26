pub fn sort(mut list: &mut [u8]) {
    heapify(&mut list);

    let mut end = list.len() - 1;
    while end > 0 {
        list.swap(end, 0);
        end -= 1;
        sift_down(&mut list, 0, end);
    }
}

fn i_parent(i: usize) -> usize {
    (i - 1) / 2
}
fn i_left_child(i: usize) -> usize {
    (2 * i) + 1
}

fn heapify(mut list: &mut [u8]) {
    let mut start = i_parent(list.len() - 1);
    let len = list.len();
    loop {
        sift_down(&mut list, start, len - 1);
        if start == 0 {
            return;
        }
        start -= 1;
    }
}

fn sift_down(list: &mut [u8], start: usize, end: usize) {
    let mut root = start;
    while i_left_child(root) <= end {
        let child = i_left_child(root);

        let mut swap = root;
        if list[swap] < list[child] {
            swap = child;
        }
        if child + 1 <= end && list[swap] < list[child + 1] {
            swap = child + 1;
        }
        if swap == root {
            return;
        } else {
            list.swap(root, swap);
            root = swap;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_it() {
        println!("{:?}", sort(&mut [6, 5, 3, 1, 8, 7, 2, 4]));
    }
}
