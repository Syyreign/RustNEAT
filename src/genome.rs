
use std::{collections::HashMap, f32::consts::E};

enum GeneType{
    Input,
    Output,
    Hidden,
}

struct Gene{
    node_num: u32,
    gene_type: GeneType,
}

impl Gene{
    fn new(node_num: u32, gene_type: GeneType) -> Gene{
        Gene{
            node_num: node_num,
            gene_type: gene_type,
        }
    }
}

struct Connection{
    in_gene_num: u32,
    out_gene_num: u32,
    weight: f32,
    is_enabled: bool,
    innovation_number: u32,
}

impl Connection{
    fn new(in_gene_num: u32, out_gene_num: u32, weight: f32, is_enabled: bool, innovation_num: u32) -> Connection{
        Connection{ 
            in_gene_num: in_gene_num, 
            out_gene_num: out_gene_num, 
            weight: weight, 
            is_enabled: is_enabled, 
            innovation_number: innovation_num, 
        }
    }

    pub fn is_enabled(&self) -> bool {
        self.is_enabled
    }
}

pub struct Genome{
    connections: HashMap<(u32, u32), Connection>,
    node_genes: HashMap<u32, Gene>,
    current_node_number: u32,
    current_innovation_number: u32,

}

impl Genome{
    pub fn new(in_nodes: u32, out_nodes: u32) -> Genome{

        let mut h_genes: HashMap<u32, Gene> = HashMap::new();

        for i in 0..in_nodes{
            h_genes.insert(i, Gene::new(i, GeneType::Input));
        }
        for i in in_nodes..in_nodes + out_nodes{
            h_genes.insert(i, Gene::new(i, GeneType::Output));
        }

        Genome{
            connections: HashMap::new(),
            node_genes: h_genes,
            current_node_number: in_nodes+out_nodes,
            current_innovation_number: 0,
        }
    }

    pub fn add_node(&mut self, in_gene_num: u32, out_gene_num: u32, weight: f32){
        if !Genome::does_connection_exist(&self.connections, in_gene_num, out_gene_num){
            return;
        }

        self.node_genes.insert(self.current_node_number, Gene::new(self.current_node_number, GeneType::Hidden));
        let current_node_number = self.current_node_number;
        self.current_node_number += 1;

        // Disable the older connection
        Genome::disable_connection(&mut self.connections, in_gene_num, out_gene_num);

        // Add two new connections
        let mut innov_num = self.get_next_innovation_number();
        self.add_connection(in_gene_num, current_node_number, weight);
        innov_num = self.get_next_innovation_number();
        self.add_connection(current_node_number, out_gene_num, weight);
    }

    pub fn add_connection(&mut self, in_gene_num: u32, out_gene_num: u32, weight: f32) -> bool{

        if !self.node_genes.contains_key(&in_gene_num) ||
        !self.node_genes.contains_key(&out_gene_num){
            return false;
        }

        let innov_num = self.get_next_innovation_number();
        self.connections.insert((in_gene_num, out_gene_num),
         Connection::new(in_gene_num, out_gene_num, weight, true, innov_num));

        true
    }

    fn get_next_innovation_number(&mut self) -> u32{
        let num = self.current_innovation_number;
        self.current_innovation_number += 1;
        num 
    }

    pub fn get_connection(&self, in_gene_num: u32, out_gene_num: u32) -> Option<&Connection>{
        if !Genome::does_connection_exist(&self.connections, in_gene_num, out_gene_num) {
            return None;
        }

        self.connections.get(&(in_gene_num, out_gene_num))
    }

    pub fn is_connection_enabled(&self, in_gene_num: u32, out_gene_num: u32) -> bool{
        if !Genome::does_connection_exist(&self.connections, in_gene_num, out_gene_num) {
            return false;
        }

        self.connections.get(&(in_gene_num, out_gene_num)).unwrap().is_enabled()
    }

    fn does_connection_exist(connections: &HashMap<(u32, u32), Connection>, in_gene_num: u32, out_gene_num: u32) -> bool{
        connections.contains_key(&(in_gene_num, out_gene_num))
    }

    fn disable_connection(connections: &mut HashMap<(u32, u32), Connection>, in_gene_num: u32, out_gene_num: u32){
        connections.entry((in_gene_num, out_gene_num)).and_modify(|k| k.is_enabled = false);
    }
}

pub fn sigmoid(input: f32) -> f32{
    1.0 / (1.0 + E.powf( -4.9 * input))
}