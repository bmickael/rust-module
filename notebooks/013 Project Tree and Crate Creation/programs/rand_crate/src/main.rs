use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let random_number: f64 = rng.gen(); // generates a float between 0 and 1
    println!("Le nombre aléatoire est : {}", random_number);
}
