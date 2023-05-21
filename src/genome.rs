
use std::collections::HashMap;

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
}

pub struct Genome{
    connections: HashMap<(u32, u32), Connection>,
    node_genes: HashMap<u32, Gene>,
    node_number: u32,

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
            node_number: in_nodes+out_nodes,
        }
    }

    pub fn add_node(&mut self, in_gene_num: u32, out_gene_num: u32, weight: f32, innovation_num: u32){
        if !Genome::does_connection_exist(&self.connections, in_gene_num, out_gene_num){
            return;
        }

        self.node_genes.insert(self.node_number, Gene::new(self.node_number, GeneType::Hidden));
        self.node_number += 1;
    }

    pub fn add_connection(&mut self, in_gene_num: u32, out_gene_num: u32, weight: f32, innovation_num: u32) -> bool{

        if !self.node_genes.contains_key(&in_gene_num) ||
        !self.node_genes.contains_key(&out_gene_num){
            return false;
        }

        // if Genome::should_disable_connection(&self.connections, in_gene_num, out_gene_num){
        //     self.connections.entry((in_gene_num, out_gene_num)).and_modify(|k| k.is_enabled = false);
        // }

        self.connections.insert((in_gene_num, out_gene_num),
         Connection::new(in_gene_num, out_gene_num, weight, true, innovation_num));

        true
    }

    fn does_connection_exist(connections: &HashMap<(u32, u32), Connection>, in_gene_num: u32, out_gene_num: u32) -> bool{
        connections.contains_key(&(in_gene_num, out_gene_num))
    }

    fn disable_connection(connections: &mut HashMap<(u32, u32), Connection>, in_gene_num: u32, out_gene_num: u32){
        connections.entry((in_gene_num, out_gene_num)).and_modify(|k| k.is_enabled = false);
    }
}