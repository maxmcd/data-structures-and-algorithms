pub fn sort(list: &mut [u8]) {
    let mut pairs = vec![vec![0u8; 0]; 0];
    for i in 0..list.len() {
        pairs.push(vec![list[i]]);
    }
    while pairs.len() != 1 {
        let left = pairs.pop().unwrap();
        let right = pairs.pop().unwrap();
        pairs.insert(0, merge(left, right));
    }
    list.copy_from_slice(&pairs[0])
}

fn merge(left: Vec<u8>, right: Vec<u8>) -> Vec<u8> {
    let mut out = vec![0; 0];
    let (mut li, mut ri) = (0, 0);
    while out.len() < left.len() + right.len() {
        if ri == right.len() || (li < left.len() && left[li] < right[ri]) {
            out.push(left[li]);
            li += 1;
        } else {
            out.push(right[ri]);
            ri += 1;
        }
    }
    out
}
