pub fn sort(list: &mut [u8]) {
    let mut size = list.len();
    while size > 0 {
        let mut largest_i = 0;
        for i in 1..size {
            if list[i] > list[largest_i] {
                largest_i = i;
            }
        }
        list.swap(largest_i, size - 1);
        size -= 1;
    }
}
