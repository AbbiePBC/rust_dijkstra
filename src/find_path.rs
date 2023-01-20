use log::debug;
use std::collections::btree_map::BTreeMap;

use crate::construct_graph::*;
use crate::parse_input::*;
use std::fs;


fn get_route_travelled(
    original_start_idx: usize,
    end_idx: usize,
    nodes_visited: &Vec<Node>,
) -> (Vec<usize>, Vec<usize>) {
    //go backwards through the nodes to find the parent node.
    let mut idx = end_idx;
    let mut nodes_in_order: Vec<usize> = Vec::new();
    let mut weights_in_order: Vec<usize> = Vec::new();

    nodes_in_order.push(end_idx);
    while idx != original_start_idx {
        idx = nodes_visited[idx].parent_idx;
        nodes_in_order.push(idx);
        weights_in_order.push(nodes_visited[idx].dist_to_node);
    }

    nodes_in_order.reverse();

    return (nodes_in_order, weights_in_order);
}

pub fn get_human_readable_route(
    nodes_in_order: Vec<usize>,
    graph_nodes: &Vec<GraphNode>,
) -> Result<Vec<String>, String> {
    let mut path_travelled: Vec<String> = Vec::new();
    for node_idx in nodes_in_order {
        let node = &graph_nodes[node_idx];

        if node.index != node_idx {
            return Err("Error in the indexing for the route travelled.".to_string());
        } else {
            path_travelled.push(node.node_name.to_string());
        }
    }
    return Ok(path_travelled);
}

pub fn print_route(route: Vec<String>) -> String {
    let mut final_path: String = route[0].to_string();
    for i in 1..route.len() {
        final_path = format!("{}->{}", final_path, route[i]);
    }

    return final_path;
}

fn find_shortest_untraversed_path(
    edges_can_traverse: &mut Vec<Edge>,
) -> Edge {
    let mut min_weight = INFINITE_DIST;
    let mut idx_edge= 0;
    println!("edges can traverse - {:?}", edges_can_traverse);

    for idx in 0..edges_can_traverse.len() {
        if edges_can_traverse[idx].weight < min_weight && !edges_can_traverse[idx].is_traversed {
            min_weight = edges_can_traverse[idx].weight ;
            idx_edge = idx;
        }
    }
    let edge_to_travel = edges_can_traverse[idx_edge];
    edges_can_traverse.remove(idx_edge);

    return edge_to_travel;
}

fn update_existing_edges_from_node(nodes_visited: &mut Vec<Node>, closest_node: Node, original_start_idx: usize) -> usize {

    let node_in_current_path = nodes_visited[closest_node.index];

    let node_visited_already = nodes_visited
        .into_iter()
        .find(|x| x.index == closest_node.parent_idx);

    match node_visited_already {
        Some(node) => {
            debug!(" we have a path to {:?}", closest_node);
            debug!("comparing node.dist_to_node {} > closest_node.dist_to_node {}+ node_to_add_to_path.dist_to_node {}", node.dist_to_node, closest_node.dist_to_node, node_in_current_path.dist_to_node);
            if node_in_current_path.dist_to_node > node.dist_to_node + closest_node.dist_to_node {
                let decrease_in_dist = node_in_current_path.dist_to_node - (node.dist_to_node + closest_node.dist_to_node);
                nodes_visited[closest_node.index] = Node::new(
                    closest_node.index,
                    node.index,
                    closest_node.dist_to_node + node.dist_to_node,
                );
                let cp_nodes_visited = nodes_visited.clone();
                // todo the idea was to update the above rather than replace it.
                // but now think we want to keep the nodes_visited and not overwrite data
                return decrease_in_dist;
            }
        }
        None => {
                //return nodes_visited[closest_node.parent_idx].dist_to_node - closest_node.dist_to_node;
            //
        }
    }
    return 0;
}

fn update_existing_edges_to_node(mut nodes_visited: &mut Vec<Node>, closest_node: Node,  decrease_in_dist: usize) {
    let cp = nodes_visited.clone();
    for mut node in cp {
        // because node is not mutable (yet), overwrite Node
        if node.parent_idx == closest_node.index && node.dist_to_node != 0  {
            nodes_visited[node.index] = Node::new(node.index,  node.parent_idx, nodes_visited[node.index].dist_to_node - decrease_in_dist);
            let x = nodes_visited.clone(); // todo: this is to make the debugger work - can remove when done
            update_existing_edges_to_node(nodes_visited, nodes_visited[node.index],  decrease_in_dist);
            return;
        }
    }
    return;
}


fn node_to_add(
    edges_can_traverse: &mut Vec<Edge>,
    nodes_visited: &mut Vec<Node>,
    graph: &mut Graph,
) -> Node {
    // todo make this naming less confusing; we add node to node_visited, but remove it from nodes_can_visit

    //debug!("possible nodes to add are: {:?}", edges_can_traverse);
    let edge = find_shortest_untraversed_path(edges_can_traverse);
    let closest_node = Node::new(edge.index_second, edge.index_first, edge.weight);
    graph.mark_edge_as_traversed(closest_node);

    return closest_node;
}


pub fn dijkstra(
    mut original_start_idx: usize,
    end_idx: usize,
    graph: &mut Graph,
) -> Result<(usize, Vec<usize>), String> {
    let mut current_idx = original_start_idx;
    let mut parent_idx = current_idx;

    let number_of_nodes = graph.number_of_nodes;
    let mut nodes_visited: Vec<Node> = Vec::with_capacity(number_of_nodes);

    for _ in 0..number_of_nodes {
        nodes_visited.push(Node::new(INFINITE_DIST, INFINITE_DIST, 0));
    }
    nodes_visited[current_idx] = Node::new(current_idx, parent_idx, 0);

    let mut edges_can_traverse = Vec::new();
    let mut look_for_node = true;
    let mut still_look_for_node = true;
    while still_look_for_node {
        add_to_frontier_edges_from_node(graph, current_idx, &mut edges_can_traverse);
        if !look_for_node {
            still_look_for_node = false;
        }
        if edges_can_traverse.is_empty() {
            if nodes_visited.iter().find(|&x| x.index == end_idx) == None {
                return Err("Are the start and end disconnected? No path found".to_string());
            } else {
                debug!("stopped looking for node. edges_can_traverse.is_empty");
                look_for_node = false;
            }
        } else {
            let closest_node =
                node_to_add(&mut edges_can_traverse, &mut nodes_visited, graph);

            match nodes_visited
                .iter()
                .find(|&x| x.index == closest_node.index)
            {
                None => {
                    current_idx = closest_node.index;
                    nodes_visited[closest_node.index] = Node::new(
                        closest_node.index,
                        closest_node.parent_idx,
                        nodes_visited[closest_node.parent_idx].dist_to_node
                            + closest_node.dist_to_node,
                    );

                }
                Some(node) => {
                    if closest_node.dist_to_node != INFINITE_DIST {
                        let dist_dec = update_existing_edges_from_node(&mut nodes_visited, closest_node, original_start_idx);
                        if dist_dec!= 0 {
                            update_existing_edges_to_node(&mut nodes_visited, closest_node, dist_dec);
                        }

                    }
                }
            }
            //graph.mark_edge_as_traversed(closest_node);
        }
    }
    debug!(
        "so far, route: {:?}",
        get_route_travelled(original_start_idx, current_idx, &nodes_visited)
    );

    let (nodes_in_order,_) = get_route_travelled(original_start_idx, end_idx, &nodes_visited);

    return Ok((nodes_visited[end_idx].dist_to_node, nodes_in_order));
}

fn add_to_frontier_edges_from_node(
    graph: &mut Graph,
    start_idx: usize,
    edges_can_traverse: &mut Vec<Edge>,
) {
    for edge in &graph.edges[start_idx] {
        if !edge.is_traversed && !edges_can_traverse.contains(&edge) {
            edges_can_traverse.push(
                Edge::new(
                    edge.index_first,
                    edge.index_second,
                    edge.weight
                ),
            );
        }
    }

    debug!("edges can traverse {:?}", edges_can_traverse);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_input;

    #[test]
    fn test_dijkstra() {
        let start_idx = 0;
        let end_idx = 2;
        let mut graph = Graph::new(vec![GraphNode::new(0, "node0".to_string()),GraphNode::new(1, "node1".to_string()), GraphNode::new(2, "node2".to_string())], vec![Edge::new(0, 1, 2),Edge::new(1, 2, 3)]);

        let (dist, path) = dijkstra(start_idx, end_idx, &mut graph).unwrap();
        assert_eq!(dist, 5);
        assert_eq!(path, vec![0, 1, 2]);
    }
    #[test]
    fn test_multiple_start_edges() {
        let start_idx = 0;
        let end_idx = 2;
        // this test started failing bc the graph::new from edges doesnt take into account redundancies
        // todo: fix this^

        let mut graph = Graph::new_from_string("3\nA\nB\nC\n\n5\nA B 20\nA B 2\nB A 2\nB C 3\nC B 1\n\nA C").unwrap();

        let (dist, path) = dijkstra(start_idx, end_idx, &mut graph).unwrap();
        assert_eq!(dist, 3);
        assert_eq!(path, vec![0, 1, 2]);
    }
    #[test]
    fn test_shorter_initial_route_gets_updated() {
        // assuming bidirectionality, now the edge weight for middle->end should be updated from 3 to 2.

        let mut expected_graph = Graph::new(
            vec![GraphNode::new(0, "node0".to_string()), GraphNode::new(1, "node1".to_string()), GraphNode::new(2, "node2".to_string())],
            vec![Edge::new(0, 1, 2), Edge::new(1, 2, 2)],
        );
        let (dist, path) = dijkstra(0, 2, &mut expected_graph).unwrap();
        assert_eq!(dist, 4);
        assert_eq!(path, vec![0, 1, 2])
    }
    #[test]
    fn find_shortest_path_branches() {
        let start_idx = 0;
        let end_idx = 4;
        let mut graph = Graph::new(
            vec![GraphNode::new(0, "node0".to_string()), GraphNode::new(1, "node1".to_string()), GraphNode::new(2, "node2".to_string()), GraphNode::new(3, "node3".to_string()), GraphNode::new(4, "node4".to_string())],
            vec![Edge::new(0, 1, 10), Edge::new(1, 2, 6),
                Edge::new(2, 3, 1), Edge::new(3, 1, 9), Edge::new(3, 4, 1)],
        );
        let (dist, path) = dijkstra(start_idx, end_idx, &mut graph).unwrap();
        assert_eq!(path, vec![0, 1, 2, 3, 4]);
        assert_eq!(dist, 18);
    }

    #[test]
    fn find_correct_route_in_file()  {
        let start_idx = 0;
        let end_idx = 2;

        let mut graph = Graph::new_from_string("5\nCardiff\nBristol\nLondon\nYork\nBirmingham\n\n5\nYork London 194\nCardiff Bristol 44\nBristol Birmingham 88\nBristol London 114\nBirmingham London 111\n\nCardiff London").unwrap();
        let (dist, path) = dijkstra(start_idx, end_idx, &mut graph).unwrap();
        // the edge 2->1 is marked as traversed ever though it doesn't get selected as the closest node
        //assert_eq!(dist, 158);
        assert_eq!(path, vec![0, 1, 2]);
        // todo: remove graph_nodes  as a needed thing here
        // let route = get_human_readable_route(path, &graph_nodes).unwrap();
        // assert_eq!(print_route(route), "Cardiff->Bristol->London".to_string());

    }
    #[test]
    fn find_correct_route_in_file_when_shorter_early_edge_is_wrong_path_simple(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let start_idx = 3;
        let end_idx = 0;
        let mut graph =
            Graph::new_from_string("4\nA\nB\nC\nD\n\n4\nA B 1\nB D 10\nA C 2\nC D 5\n\nA D")
                .unwrap();

        let (dist, path) = dijkstra(start_idx, end_idx, &mut graph).unwrap();

        assert_eq!(dist, 7);
        assert_eq!(path, vec![3, 2, 0]);

        Ok(())
    }
    #[test]
    fn simplify_below_test() {
        let mut graph = Graph::new_from_string("8\nInverness\nGlasgow\nEdinburgh\nNewcastle\nManchester\nYork\nBirmingham\nLondon\n\n12\nInverness Glasgow 167\nInverness Edinburgh 158\nGlasgow Edinburgh 45\nGlasgow Newcastle 145\nGlasgow Manchester 214\nEdinburgh Newcastle 107\nNewcastle York 82\nManchester York 65\nManchester Birmingham 81\nYork Birmingham 129\nYork London 194\nBirmingham London 111\n\nLondon York").unwrap();

        // let (dist, path) = dijkstra(7, 3, &mut graph).unwrap();
        // assert_eq!(path, [7,5,3]);
        // assert_eq!(dist, 194 + 82);
        // let mut graph = Graph::new_from_string("8\nInverness\nGlasgow\nEdinburgh\nNewcastle\nManchester\nYork\nBirmingham\nLondon\n\n12\nInverness Glasgow 167\nInverness Edinburgh 158\nGlasgow Edinburgh 45\nGlasgow Newcastle 145\nGlasgow Manchester 214\nEdinburgh Newcastle 107\nNewcastle York 82\nManchester York 65\nManchester Birmingham 81\nYork Birmingham 129\nYork London 194\nBirmingham London 111\n\nLondon York").unwrap();
        //
        //
        // let (dist, path) = dijkstra(7, 5, &mut graph).unwrap();
        // assert_eq!(path, [7,5]);
        // assert_eq!(dist, 194);
        // let mut graph = Graph::new_from_string("8\nInverness\nGlasgow\nEdinburgh\nNewcastle\nManchester\nYork\nBirmingham\nLondon\n\n12\nInverness Glasgow 167\nInverness Edinburgh 158\nGlasgow Edinburgh 45\nGlasgow Newcastle 145\nGlasgow Manchester 214\nEdinburgh Newcastle 107\nNewcastle York 82\nManchester York 65\nManchester Birmingham 81\nYork Birmingham 129\nYork London 194\nBirmingham London 111\n\nLondon York").unwrap();
        //
        // let (dist, path) = dijkstra(5, 3, &mut graph).unwrap();
        // assert_eq!(path, [5,3]);
        // assert_eq!(dist, 82);

        // let mut graph = Graph::new_from_string("8\nInverness\nGlasgow\nEdinburgh\nNewcastle\nManchester\nYork\nBirmingham\nLondon\n\n12\nInverness Glasgow 167\nInverness Edinburgh 158\nGlasgow Edinburgh 45\nGlasgow Newcastle 145\nGlasgow Manchester 214\nEdinburgh Newcastle 107\nNewcastle York 82\nManchester York 65\nManchester Birmingham 81\nYork Birmingham 129\nYork London 194\nBirmingham London 111\n\nLondon York").unwrap();
        // let (dist, path) = dijkstra(5, 2, &mut graph).unwrap();
        // assert_eq!(path, [5,3,2]);
        // assert_eq!(dist, 82 + 107);
        //
        //
        // let mut graph = Graph::new_from_string("8\nInverness\nGlasgow\nEdinburgh\nNewcastle\nManchester\nYork\nBirmingham\nLondon\n\n12\nInverness Glasgow 167\nInverness Edinburgh 158\nGlasgow Edinburgh 45\nGlasgow Newcastle 145\nGlasgow Manchester 214\nEdinburgh Newcastle 107\nNewcastle York 82\nManchester York 65\nManchester Birmingham 81\nYork Birmingham 129\nYork London 194\nBirmingham London 111\n\nLondon York").unwrap();
        // let (dist, path) = dijkstra(5, 0, &mut graph).unwrap();
        // assert_eq!(path, [5,3,2,0]);
        // assert_eq!(dist, 82 + 107 + 158);

        let mut graph = Graph::new_from_string("8\nInverness\nGlasgow\nEdinburgh\nNewcastle\nManchester\nYork\nBirmingham\nLondon\n\n12\nInverness Glasgow 167\nInverness Edinburgh 158\nGlasgow Edinburgh 45\nGlasgow Newcastle 145\nGlasgow Manchester 214\nEdinburgh Newcastle 107\nNewcastle York 82\nManchester York 65\nManchester Birmingham 81\nYork Birmingham 129\nYork London 194\nBirmingham London 111\n\nLondon York").unwrap();
        let (dist, path) = dijkstra(7, 2, &mut graph).unwrap();
        assert_eq!(path, [7, 5,3,2]);
        assert_eq!(dist, 194 + 82 + 107);

        // let mut graph = Graph::new_from_string("8\nInverness\nGlasgow\nEdinburgh\nNewcastle\nManchester\nYork\nBirmingham\nLondon\n\n12\nInverness Glasgow 167\nInverness Edinburgh 158\nGlasgow Edinburgh 45\nGlasgow Newcastle 145\nGlasgow Manchester 214\nEdinburgh Newcastle 107\nNewcastle York 82\nManchester York 65\nManchester Birmingham 81\nYork Birmingham 129\nYork London 194\nBirmingham London 111\n\nLondon York").unwrap();
        // let (dist, path) = dijkstra(7, 0, &mut graph).unwrap();
        // assert_eq!(path, [7, 5,3,2,0]);
        // assert_eq!(dist, 194 + 82 + 107 + 158)

    }
    #[test]
    fn find_correct_route_in_file_when_shorter_early_edge_is_wrong_path() {
        let start_idx = 0;
        let end_idx = 7;
        let mut graph = Graph::new_from_string("8\nInverness\nGlasgow\nEdinburgh\nNewcastle\nManchester\nYork\nBirmingham\nLondon\n\n12\nInverness Glasgow 167\nInverness Edinburgh 158\nGlasgow Edinburgh 45\nGlasgow Newcastle 145\nGlasgow Manchester 214\nEdinburgh Newcastle 107\nNewcastle York 82\nManchester York 65\nManchester Birmingham 81\nYork Birmingham 129\nYork London 194\nBirmingham London 111\n\nLondon Inverness").unwrap();

        let (dist, path) = dijkstra( start_idx, end_idx, &mut graph).unwrap();

        assert_eq!(dist, 541);
        assert_eq!(path, vec![0, 2, 3, 5, 7]);

        // in opposite direction
        let mut graph = Graph::new_from_string("8\nInverness\nGlasgow\nEdinburgh\nNewcastle\nManchester\nYork\nBirmingham\nLondon\n\n12\nInverness Glasgow 167\nInverness Edinburgh 158\nGlasgow Edinburgh 45\nGlasgow Newcastle 145\nGlasgow Manchester 214\nEdinburgh Newcastle 107\nNewcastle York 82\nManchester York 65\nManchester Birmingham 81\nYork Birmingham 129\nYork London 194\nBirmingham London 111\n\nLondon Inverness").unwrap();

        let (dist, path) = dijkstra( end_idx, start_idx, &mut graph).unwrap();

        assert_eq!(path, vec![7, 5, 3, 2, 0]);
        assert_eq!(dist, 541);

    }
    #[test]
    fn find_self_referential_route_in_file() {
        assert_eq!(
            Graph::new_from_string("3\nA\nB\nC\n\n4\nA A 1\nA B 2\nB C 3\nA C 4\n\nA A"),
            Err("Route is self referential. Dist from A to A = 0".to_string())
        );
    }
    #[test]
    fn find_disconnected_route_in_file() {
        let mut graph =
            Graph::new_from_string("4\nA\nB\nC\nD\n\n4\nA B 1\nA B 2\nB C 3\nA C 4\n\nA D")
                .unwrap();
        assert_eq!(
            dijkstra(0, 3, &mut graph),
            Err("Are the start and end disconnected? No path found".to_string())
        );
    }
    #[test]
    fn test_updating_path_simple(){

        //
        // if node.dist_to_node > closest_node.dist_to_node + node_to_add_to_path.dist_to_node {
        //     nodes_visited[closest_node.parent_idx] = Node::new(1000, 1000, 1000);
        let original_start_idx = 0;
        let mut nodes_visited = vec![Node::new(0, 0, 0), Node::new(1, 0, 100), Node::new(2,0, 300)];
        let closest_node = Node::new(2,1, 20);
        update_existing_edges_from_node(&mut nodes_visited, closest_node, original_start_idx);
        assert_eq!(nodes_visited, vec![Node::new(0, 0, 0), Node::new(1, 0, 100), Node::new(2,1, 120)]);
    }
    #[test]
    fn test_updating_path_repeatedly(){

        let original_start_idx = 0;
        let mut nodes_visited = vec![Node::new(0, 0, 0), Node::new(1, 0, 300), Node::new(2,1, 400), Node::new(3,2, 500)];
        let closest_node = Node::new(1,0, 20);
        update_existing_edges_from_node(&mut nodes_visited, closest_node, original_start_idx);
        update_existing_edges_to_node(&mut nodes_visited, closest_node, 280);

        assert_eq!(nodes_visited, vec![Node::new(0, 0, 0), Node::new(1, 0, 20), Node::new(2,1, 120), Node::new(3,2, 220)]);
    }
    #[test]
    fn test_updating_old_path(){

        //
        // if node.dist_to_node > closest_node.dist_to_node + node_to_add_to_path.dist_to_node {
        //     nodes_visited[closest_node.parent_idx] = Node::new(1000, 1000, 1000);
        let original_start_idx = 0;
        let mut nodes_visited = vec![Node::new(0, 0, 0), Node::new(1, 0, 100), Node::new(2,0, 300), Node::new(3,2, 400)];
        let closest_node = Node::new(2,1, 20);
        update_existing_edges_from_node(&mut nodes_visited, closest_node, original_start_idx);
        update_existing_edges_to_node(&mut nodes_visited, closest_node, 300 - (100+20));
        assert_eq!(nodes_visited, vec![Node::new(0, 0, 0), Node::new(1, 0, 100), Node::new(2,1, 120), Node::new(3,2, 220)]);
        // let closest_node = Node::new(2,1, 10);
        // update_existing_edges_from_node(&mut nodes_visited, closest_node, original_start_idx);
        // update_existing_edges_to_node(&mut nodes_visited, closest_node, 120 - (100+10));
        //
        // assert_eq!(nodes_visited, vec![Node::new(0, 0, 0), Node::new(1, 0, 100), Node::new(2,1, 110), Node::new(3,2, 210)]);
    }

    #[test]
    fn test_update_path_to_start(){
        // taken from the london->inverness test failing, but relevant indexes here are -5 for indexing reasons.
        let mut nodes_visited = vec![Node::new(0,1,240), Node::new(1,2,111),  Node::new(2,2, 0)];
        let closest_node = Node::new(0,2, 194);
        let dec = update_existing_edges_from_node(&mut nodes_visited, closest_node, 2);
        update_existing_edges_to_node(&mut nodes_visited, closest_node, dec);
        assert_eq!(nodes_visited, vec![ Node::new(0,2, 194), Node::new(1,2,111), Node::new(2,2, 0)]);

    }
}
