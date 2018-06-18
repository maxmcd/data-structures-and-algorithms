pub fn sort(list: &mut [u8]) {
    for mut i in 1..list.len() {
        while i != 0 && list[i] < list[i - 1] {
            list.swap(i, i - 1);
            i -= 1;
        }
    }
}
