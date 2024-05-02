use std::collections::HashMap;
use std::error::Error;
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
struct Edge {
    source: usize, 
    target: usize 
}

#[derive(Debug)]
struct Graph {
    nodes: Vec<NameData>, 
    edges: Vec<Edge>
}

impl Graph {
    fn new() -> Self {
        Graph { nodes: Vec::new(), edges: Vec::new() }
    }

    fn add_node(& mut self, node: NameData) {
        self.nodes.push(node);
    }

    fn add_edge(&mut self, source: usize, target: usize) {
        self.edges.push(Edge { source, target });
    }

    fn node_count(&self) -> usize { 
        self.nodes.len()
    }

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

        let mut gender_ethnicity_count: HashMap<(&String, &String), (u32, Vec<String>)> = HashMap::new();
        for node in &self.nodes {
            let entry = gender_ethnicity_count.entry((&node.gender, &node.ethnicity)).or_insert((0, Vec::new()));
            entry.0 += node.count;
            entry.1.push(node.name.clone());
        }

        println!("Total count of names that share both gender and ethnicity:");
        for ((gender, ethnicity), (count, names)) in &gender_ethnicity_count {
            println!("Gender: {}, Ethnicity: {}: {}", gender, ethnicity, count);
            println!("Names: {:?}", names);
        }
    }

    fn count_names_by_starting_letter(&self) -> HashMap<(&String, &String), HashMap<char, u32>> {
        let mut starting_letter_count: HashMap<(&String, &String), HashMap<char, u32>> = HashMap::new();

        for node in &self.nodes {
            let starting_letter_map = starting_letter_count
                .entry((&node.gender, &node.ethnicity))
                .or_insert(HashMap::new());

            if let Some(first_char) = node.name.chars().next() {
                *starting_letter_map.entry(first_char).or_insert(0) += 1;
            }
        }

        starting_letter_count
    }
}
fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "Popular_Baby_Names.csv";
    let mut rdr = ReaderBuilder::new().has_headers(true).from_path(file_path)?;

    let mut graph = Graph::new();
    let mut node_count = 0;
    for result in rdr.records() {
        if node_count >= 1000 {
            break; 
        }
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
                node_count += 1; 
            }
        }
    }

    let start_node = 0; 
    let mut visited = vec![false; graph.node_count()];
    graph.dfs(start_node, &mut visited);
    graph.analyze_graph();

    let mut starting_letter_counts = graph.count_names_by_starting_letter();
    for letter_count in starting_letter_counts.values_mut() {
        let mut sorted_entries: Vec<_> = letter_count.iter().collect();
        sorted_entries.sort_by_key(|(letter, _count)| **letter);
        *letter_count = sorted_entries.into_iter().map(|(letter, count)| (*letter, *count)).collect();
    }

    println!("Counts of names by starting letter:");
    for ((gender, ethnicity), letter_count) in &starting_letter_counts {
        println!("Gender: {}, Ethnicity: {}", gender, ethnicity);
        let mut sorted_letters: Vec<_> = letter_count.keys().collect();
        sorted_letters.sort();
        for letter in sorted_letters {
            if let Some(count) = letter_count.get(letter) {
                println!("{}: {}", letter, count);
            }
        }
    }

    Ok(())
}