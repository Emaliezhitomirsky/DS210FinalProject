use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::path::Path;
use csv::ReaderBuilder;

#[derive(Debug, Clone)]
struct NameData {
    year: u32,
    gender: String,
    ethnicity: String, 
    name: String, 
    count: u32,
    rank: u32,
}

impl NameData {
    fn from_record(record: csv::StringRecord) -> Option<Self> {
        if record.len() == 6 {
            Some(NameData {
                year: record[0].parse().ok()?,
                gender: record[1].to_string(),
                ethnicity: record[2].to_string(),
                name: record[3].to_string(), 
                count: record[4].parse().ok()?,
                rank: record[5].parse().ok()?,
            })
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Edge { source: usize, target: usize }

#[derive(Debug)]
struct Graph { nodes: Vec<NameData>, edges: Vec<Edge> }

impl Graph {
    fn new() -> Self {
        Graph { nodes: Vec::new(), edges: Vec::new() }
    }

    fn add_node(& mut self, node: NameData) { self.nodes.push(node); }

    fn add_edge(&mut self, souce: usize, target: usize) { self.edges.push(Edge { source, target }); }

    fn node_count(&self) -> usize { self.nodes.len() }

    fn get_node(&self, index: usize) -> Option<&NameData> { self.nodes.get(index) }

    fn dfs(&self, node: usize, visited: &mut Vec<bool>) {
        visited[node] = true;
        for edge in &self.edges {
            if edge.source == node && !visited[edge.target] {
                self.dfs(edge.target, visited);
            }
        }
    }

    fn analyze_graph(&self) {
        println!("Number of nodes in the graph: {}", self.node_count());
        println!("Number of edges in the graph: {}", self.edges.len());

        let mut gender_ethnicity_count: HashMap<(&String, &String), u32> = HashMap::new();
        for node in &self.nodes {
            *gender_ethnicity_count.entry((&node.gender, &node.ethnicity)).or_insert(0) += node.count;
        }

        println!("Total count of names that share both gender and ethnicity:");
        for ((gender, ethnicity), count) in &gender_ethnicity_count {
            println!("Gender: {}, Ethnicity: {}: {}", gender, ethnicity, count);
        }
    }
}
fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "Popular_Baby_Names.csv";
    let mut rdr = ReaderBuilder::new().has_headers(true).from_path(file_path)?;

    let mut graph = Graph::new();
    for result in rdr.records() {
        if let Ok(record) = result {
            if let Some(name_data) = NameData::from_record(record) {
                let node_index = graph.node_count();
                graph.add_node(name_data.clone());
                let similar_nodes: Vec<usize> = graph.nodes.iter().enumerate().filter_map(|(idx, node)| {
                    if node.gender == name_data.gender && node.ethnicity == name_data.ethnicity {
                        Some(idx)
                    } else { None }
                }).collect();
                for &similar_node in &similar_nodes {
                    if similar_node != node_index { graph.add_edge(similar_node, node_index); }
                }
            }
        }
    }

    let start_node = 0; 
    let mut visited = vec![false; graph.node_count()];
    graph.dfs(start_node, &mut visited);
    graph.analyze_graph();
    Ok(())
}