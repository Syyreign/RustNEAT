mod genome;

fn main() {
    println!("Hello, world!");

    let mut genome = genome::Genome::new(2,2);
    genome.add_connection(1, 2, 0.5);
    genome.add_node(1, 2, 0.5);
    genome.add_node(1, 4, 0.5);
    let x = 5;
}

#[cfg(test)]
mod tests {
    use crate::genome::Genome;  

    #[test]
    fn test_add_first_connection() {
        let mut genome = Genome::new(2,2);
        genome.add_connection(1, 2, 0.5);
        let is_enabled = genome.is_connection_enabled(1,2);
        assert_eq!(is_enabled, true);
    }

    #[test]
    fn test_add_hidden_node() {
        let mut genome = Genome::new(2,2);
        genome.add_connection(1, 2, 0.5);
        genome.add_node(1, 2, 0.5);
        let should_be_disabled = genome.is_connection_enabled(1,2);
        assert_eq!(should_be_disabled, false);
        let should_be_enabled = genome.is_connection_enabled(1, 4);
        assert_eq!(should_be_enabled, true);
    }

    #[test]
    fn test_add_two_hidden_nodes() {
        let mut genome = Genome::new(2,2);
        genome.add_connection(1, 2, 0.5);
        genome.add_node(1, 2, 0.5);
        genome.add_node(1, 4, 0.5);

        let should_be_disabled_1 = genome.is_connection_enabled(1,2);
        assert_eq!(should_be_disabled_1, false);

        let should_be_disabled_2 = genome.is_connection_enabled(1, 4);
        assert_eq!(should_be_disabled_2, false);

        let should_be_enabled = genome.is_connection_enabled(1, 5);
        assert_eq!(should_be_enabled, true);
    }
}
