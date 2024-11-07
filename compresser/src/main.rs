use counter::Counter;
use std::{cmp::Reverse, collections::{BinaryHeap, HashMap}, fs};

#[derive(Debug)]
enum NodeType {
    Leaf {
        symbol: u8
    },
    Branch {
        left: Box<Node>,
        rigth: Box<Node>
    }
}

#[derive(Debug)]
struct Node {
    frequency: usize,
    kind: NodeType
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.frequency == other.frequency
    }
}

impl Eq for Node {}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.frequency.partial_cmp(&other.frequency)
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return self.frequency.cmp(&other.frequency);
    }
}

fn main() -> Result<(), &'static str> {
    let args = std::env::args();

    if args.len() != 2 {
        return Err("Wrong usage: expected file path argument");
    }

    let file = args.last().unwrap();

    if let Ok(data) = fs::read_to_string(file) {
        let frequencies = data
            .bytes()
            .collect::<Counter<_>>()
            .into_map();

        let root = build_huffman_tree(&frequencies);
        println!("{:#?}", root);
        let codes = generate_codes_from_huffman_tree(&root);
        println!("{:#?}", codes);

        return Ok(())
    }

    Err("Error reading file")
}

fn generate_codes_from_huffman_tree(root: &Node) -> HashMap<u8, String> {
    fn build_code(current_code: String, node: &Node, codes: &mut HashMap<u8, String>) {
        match &node.kind {
            NodeType::Leaf { symbol } => {
                codes.insert(*symbol, current_code);
            }
            NodeType::Branch { left, rigth } => {
                build_code(current_code.clone()+"0", &left, codes);
                build_code(current_code+"1", &rigth, codes);
            }
        }
    }
    let mut codes = HashMap::new();
    build_code(String::new(), root, &mut codes);
    codes
}

fn build_huffman_tree(frequencies: &HashMap<u8, usize>) -> Node {
    let mut queue = frequencies
        .iter()
        .map(|(symbol, frequency)| Reverse(Node{
            frequency: frequency.to_owned(),
            kind: NodeType::Leaf { 
                symbol: symbol.to_owned() 
            }
        }))
        .collect::<BinaryHeap<_>>();

    while queue.len() > 1 {
        let rigth = queue.pop().unwrap().0;
        let left = queue.pop().unwrap().0;

        queue.push(Reverse(Node { 
            frequency: left.frequency + rigth.frequency, 
            kind: NodeType::Branch { 
                left: Box::new(left), 
                rigth: Box::new(rigth) 
            }
        }));
    }

    queue.pop().unwrap().0
}