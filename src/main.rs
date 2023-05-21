mod genome;

fn main() {
    println!("Hello, world!");

    let mut genome = genome::Genome::new(2,2);
    genome.add_connection(1, 2, 0.5, 1);
    let x = 5;
}
