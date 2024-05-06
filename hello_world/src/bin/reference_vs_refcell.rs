use std::vec;

#[derive(Debug, Clone, Copy)]
struct RGB(i32, i32, i32);

#[derive(Debug, Clone)]
struct Node {
    val: RGB,
}

#[derive(Debug)]
struct NodeCollection {
    collection: Vec<Node>,
}

impl NodeCollection {
    fn new(collection: Vec<Node>) -> Self {
        NodeCollection { collection }
    }

    fn get_node_mut(&mut self, index: usize) -> &mut Node {
        &mut self.collection[index]
    }

    // fn get_node(&self, index: usize) -> &Node {
    //     &self.collection[index]
    // }
}

fn main() {
    let mut node_collection = NodeCollection::new(vec![
        Node {
            val: RGB(100, 100, 100),
        },
        Node {
            val: RGB(200, 80, 80),
        },
        Node {
            val: RGB(150, 300, 100),
        },
    ]);

    let node2 = node_collection.get_node_mut(1);
    let node2_val = node2.val.0;
    let node1 = node_collection.get_node_mut(0);
    node1.val.0 += node2_val;

    println!("{node_collection:?}");
    let mut node3 = Node {
        val: RGB(10, 20, 30),
    };
    println!("node3 is {node3:?}");
    let node4 = &mut node3;
    println!("node4 is {node4:?}");
    node4.val.0 = 20;
    println!("node4 is {node4:?}");
    println!("node3 is {node3:?}");
}
