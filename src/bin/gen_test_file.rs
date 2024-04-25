use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    for _ in 0..(1 << 20) {
        println!("{}", rng.gen::<usize>())
    }
}
