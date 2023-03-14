
use petgraph::{algo::dijkstra, graph::NodeIndex, Graph};

fn parse(input: &str) -> (u32, u32) {
    let mut graph: Graph<u8, ()> = Graph::default();
    let mut start_node = NodeIndex::new(0);
    let mut end_node = NodeIndex::new(0);
    input
    .lines()
    .enumerate()
    .for_each(|(i, l)| {
        let width = l.len();
        l
        .chars()
        .enumerate()
        .for_each(|(j, c)| {
            let c_byte = c as u8;
            let elevation: u8 = match c_byte {
                83 => 0,
                69 => 25,
                _ => c_byte - 97 
            };
            let node = graph.add_node(elevation);
            match c_byte {
                83 => start_node = node,
                69 => end_node = node,
                _ => {}
            }
            if j > 0 {
                let left_adj = NodeIndex::new(node.index() - 1);
                if *graph.node_weight(left_adj).unwrap() <= elevation + 1 {
                    graph.add_edge(NodeIndex::new(node.index() - 1), node, ());
                }
                if elevation <= *graph.node_weight(left_adj).unwrap() + 1 {
                    graph.add_edge(node, NodeIndex::new(node.index() - 1), ());
                }
            }
            if i > 0 {
                let up_adj = NodeIndex::new(node.index() - width as usize);
                if *graph.node_weight(up_adj).unwrap() <= elevation + 1 {
                    graph.add_edge(up_adj, node, ());
                }
                if elevation <= *graph.node_weight(up_adj).unwrap() + 1 {
                    graph.add_edge(node, up_adj, ());
                }
     } })});
     let walk_graph = dijkstra(&graph, end_node, None, |_| 1);
     let part_1 = walk_graph[&start_node];
     let part_2 =  *graph
     .node_indices()
     .filter(|&i| *graph.node_weight(i).unwrap() == 0)
     .filter_map(|start_node| walk_graph.get(&start_node))
     .min()
     .unwrap();
     (part_1, part_2)
     

}

fn main() {
    let ans = parse(include_str!("../input.txt"));
    println!("part 1 answer: {}", ans.0);
    println!("part 2 answer: {}", ans.1);
}

#[cfg(test)]
mod tests {
    use super::*;
   
    #[test]
    fn test() {
        let test_ans = parse(include_str!("../example.txt"));
        assert_eq!(test_ans.0, 31);
        assert_eq!(test_ans.1, 29);
    }
}
