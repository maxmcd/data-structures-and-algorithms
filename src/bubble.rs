pub fn sort(list: &mut [u8]) {
    let mut swapped = true;
    while swapped {
        swapped = false;
        for i in 1..list.len() {
            if list[i] < list[i - 1] {
                list.swap(i, i - 1);
                swapped = true;
            }
        }
    }
}
