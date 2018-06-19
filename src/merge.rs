pub fn sort(mut list: &mut [u8]) {
    let len = list.len();
    let mut copy = vec![0u8; len];
    copy.copy_from_slice(&list);
    let mut width = 1;
    while width < len {
        let mut i = 0;
        while i < len {
            merge(
                &mut list,
                &mut copy,
                i,
                min(i + width, len),
                min(i + 2 * width, len),
            );
            i = i + 2 * width;
        }
        width = 2 * width;
    }
}

fn merge(a: &mut [u8], b: &mut [u8], begin: usize, middle: usize, end: usize) {
    let mut bi = begin;
    let mut mi = middle;
    for i in bi..end {
        if mi == end || (bi < middle && a[bi] < a[mi]) {
            b[i] = a[bi];
            bi += 1;
        } else {
            b[i] = a[mi];
            mi += 1;
        }
    }
    a[begin..end].copy_from_slice(&b[begin..end]);
}

fn min(x: usize, y: usize) -> usize {
    if x > y {
        y
    } else {
        x
    }
}
