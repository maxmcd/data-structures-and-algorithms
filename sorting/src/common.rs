use rand;
use rand::Rng;

pub fn random_array(size: usize) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    (0..size).map(|_| rng.gen_range(0, 255)).collect()
}
