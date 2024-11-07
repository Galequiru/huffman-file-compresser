use counter::Counter;
use std::{cmp::Reverse, collections::{BinaryHeap, HashMap}, fs::File, io::{Error, ErrorKind, Read}, path::Path};

const COMPRESSED_FILE_EXTENTION: &str = "foo";

#[derive(Debug, PartialEq, Eq)]
enum NodeType {
    Leaf {
        symbol: u8
    },
    Branch {
        left: Box<TreeNode>,
        rigth: Box<TreeNode>
    }
}

#[derive(Debug, PartialEq, Eq)]
struct TreeNode {
    frequency: usize,
    node_type: NodeType
}

impl PartialOrd for TreeNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.frequency.partial_cmp(&other.frequency)
    }
}

impl Ord for TreeNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return self.frequency.cmp(&other.frequency);
    }
}

fn main() -> Result<(), Error> {
    let args = std::env::args();

    if args.len() != 2 {
        return Err(Error::new(ErrorKind::InvalidInput, 
            "Wrong usage\nExpected usage: compresser filePath"));
    }
    let arg = args.last().unwrap();
    let path = Path::new(&arg);

    let mut file = File::open(path)?;

    let mut data = String::new();
    file.read_to_string(&mut data)?;

    if let Some(extension) = path.extension() {
        match extension.to_str() {
            Some(COMPRESSED_FILE_EXTENTION) => {
                decompress_file();
            },
            _ => {
                compress_file(path, &data);
            }
        }
    }

    Ok(())
}

fn compress_file(path: &Path, data: &str) {
    let frequencies = data
        .bytes()
        .collect::<Counter<_>>()
        .into_map();

    let root = build_huffman_tree(&frequencies).unwrap();

    let codes = generate_codes_from_huffman_tree(&root);
}

fn decompress_file() {

}

fn generate_codes_from_huffman_tree(root: &TreeNode) -> HashMap<u8, String> {
    fn build_code(current_code: String, node: &TreeNode, codes: &mut HashMap<u8, String>) {
        match &node.node_type {
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

fn build_huffman_tree(frequencies: &HashMap<u8, usize>) -> Option<TreeNode> {
    let mut queue = frequencies
        .iter()
        .map(|(symbol, frequency)| Reverse(TreeNode{
            frequency: frequency.to_owned(),
            node_type: NodeType::Leaf { 
                symbol: symbol.to_owned() 
            }
        }))
        .collect::<BinaryHeap<_>>();

    while queue.len() > 1 {
        let rigth = queue.pop().unwrap().0;
        let left = queue.pop().unwrap().0;

        queue.push(Reverse(TreeNode { 
            frequency: left.frequency + rigth.frequency, 
            node_type: NodeType::Branch { 
                left: Box::new(left), 
                rigth: Box::new(rigth) 
            }
        }));
    }

    Some(queue.pop()?.0)
}