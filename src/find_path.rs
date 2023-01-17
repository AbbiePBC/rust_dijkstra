use log::debug;
use std::collections::btree_map::BTreeMap;

use crate::construct_graph::*;
use crate::parse_input::*;
use std::{env, fs};

#[derive(Debug, Clone, PartialEq, Copy)]
pub(crate) struct Node {
    pub index: usize,
    pub parent_idx: usize,
    dist_to_node: usize,
}

impl Node {
    pub(crate) fn new(index_: usize, parent_idx_: usize, dist_to_node_: usize) -> Node {
        return Node {
            index: index_,
            parent_idx: parent_idx_,
            dist_to_node: dist_to_node_,
        };
    }
}

fn get_route_travelled(
    original_start_idx: usize,
    end_idx: usize,
    nodes_visited: &Vec<Node>,
) -> Vec<usize> {
    //go backwards through the nodes to find the parent node.
    let mut idx = end_idx;
    let mut nodes_in_order: Vec<usize> = Vec::new();
    nodes_in_order.push(end_idx);
    //debug!("the nodes visited are 1 {:?}", nodes_visited);
    while idx != original_start_idx {
        idx = nodes_visited[idx].parent_idx;
        nodes_in_order.push(idx);
    }

    nodes_in_order.reverse();
    //debug!("Nodes in order: {:?}", &nodes_in_order);

    return nodes_in_order;
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

fn print_route(route: Vec<String>) -> String {
    let mut final_path: String = route[0].to_string();
    for i in 1..route.len() {
        final_path = format!("{}->{}", final_path, route[i]);
    }

    return final_path;
}

fn find_closest_node(
    edges_can_traverse: &mut BTreeMap<usize, Node>,
    nodes_visited: &Vec<Node>,
    graph: &mut Graph,
) -> Node {
    let mut min_weight = INFINITE_DIST;
    let mut node_to_remove = Node::new(INFINITE_DIST, INFINITE_DIST, INFINITE_DIST);

    //debug!("the starting edges are {:?}", edges_can_traverse);
    let mut key_to_remove = INFINITE_DIST;
    //todo same as the node idx?
    for (key, node) in &*edges_can_traverse {
        if node.dist_to_node + nodes_visited[node.parent_idx].dist_to_node <= min_weight {
            min_weight = node.dist_to_node; // + nodes_visited[node.parent_idx].dist_to_node
            node_to_remove = *node;
            key_to_remove = *key;
            //debug!("the key {} might be removed, which is node {:?}", key_to_remove, edges_can_traverse[&key_to_remove]);
            //.ok_or("Error in path finding".to_string())?;;
        }
    }
    graph.mark_edge_as_traversed(edges_can_traverse[&key_to_remove]);
    //debug!("the key {} was removed, which is node {:?}", key_to_remove, edges_can_traverse[&key_to_remove]);

    edges_can_traverse.remove(&key_to_remove);
    return node_to_remove;
}

fn update_existing_edges_to_node(nodes_visited: &mut Vec<Node>, closest_node: Node) {
    //debug!(" updating existing edges to {:?}", closest_node);
    let node_to_add_to_path = nodes_visited[closest_node.index];
    //debug!(" for the edge {:?} from {}", closest_node.dist_to_node, closest_node.parent_idx);

    //debug!(" updating the current val of {:?}", nodes_visited[closest_node.index]);

    let node_visited_already = nodes_visited
        .into_iter()
        .find(|x| x.index == closest_node.parent_idx);

    match node_visited_already {
        Some(node) => {
            debug!(" we have a path to {:?}", closest_node);
            debug!("comparing node.dist_to_node {} > closest_node.dist_to_node {}+ node_to_add_to_path.dist_to_node {}", node.dist_to_node, closest_node.dist_to_node, node_to_add_to_path.dist_to_node);
            if node.dist_to_node > closest_node.dist_to_node + node_to_add_to_path.dist_to_node {
                debug!(
                    " we're now updating the path' to {:?}",
                    closest_node.parent_idx
                );
                nodes_visited[closest_node.parent_idx] = Node::new(
                    node.index,
                    closest_node.index,
                    closest_node.dist_to_node + node_to_add_to_path.dist_to_node,
                );
                // todo the idea was to update the above rather than replace it.
                // but now think we want to keep the nodes_visited and not overwrite data
                // either way, revisit this

                update_existing_edges_to_node(nodes_visited, nodes_visited[closest_node.parent_idx])
            }
        }
        None => {
            println!("no edges to {:?} yet", closest_node);
        }
    }
}

fn index_of_node_to_add(
    edges_can_traverse: &mut BTreeMap<usize, Node>,
    nodes_visited: &mut Vec<Node>,
    graph: &mut Graph,
) -> Node {
    // todo make this naming less confusing; we add node to node_visited, but remove it from nodes_can_visit
    // if A and B are added, we can add either A->B or B->A here, and one of those will be in the wrong direction.

    //debug!("possible nodes to add are: {:?}", edges_can_traverse);
    let closest_node = find_closest_node(edges_can_traverse, nodes_visited, graph);
    //update_existing_edges_to_node(nodes_visited, closest_node);
    // if the node to add is too far away, return soemthing esle?

    return closest_node;
}

fn add_to_frontier(nodes_can_visit: &mut BTreeMap<usize, Node>, edge_to_add: &Edge) {
    nodes_can_visit.insert(
        edge_to_add.index_second,
        Node::new(
            edge_to_add.index_second,
            edge_to_add.index_first,
            edge_to_add.weight,
        ),
    );
    //debug!("nodes can visit: {:?}", nodes_can_visit);
}

fn dijkstra(
    mut start_idx: usize,
    end_idx: usize,
    graph: &mut Graph,
) -> Result<(usize, Vec<usize>), String> {
    let original_start_idx = start_idx;
    let mut parent_idx = start_idx;

    let number_of_nodes = graph.number_of_nodes;
    //todo: use a binary search tree here to avoid needing to allocate space for the whole vector.
    let mut nodes_visited: Vec<Node> = Vec::with_capacity(number_of_nodes);
    // nodes_visited should be turned into a Vec<Vec<Nodes>>, so that nodes_visited[node_index] = path that node.
    // then we can find the path with the min dist, by the dist to the end node following that path.
    for _ in 0..number_of_nodes {
        nodes_visited.push(Node::new(INFINITE_DIST, INFINITE_DIST, 0));
    }
    nodes_visited[start_idx] = Node::new(start_idx, parent_idx, 0);

    let mut edges_can_traverse: BTreeMap<usize, Node> = BTreeMap::new(); // can make this just contain edges probably
    let mut look_for_node = true;
    while look_for_node {
        add_to_frontier_edges_from_node(graph, start_idx, &mut edges_can_traverse);

        if edges_can_traverse.is_empty() {
            if nodes_visited.iter().find(|&x| x.index == end_idx) == None {
                return Err("Are the start and end disconnected? No path found".to_string());
            } else {
                debug!("stopped looking for node. edges_can_traverse.is_empty");
                look_for_node = false;
            }
        } else {
            let closest_node =
                index_of_node_to_add(&mut edges_can_traverse, &mut nodes_visited, graph);
            graph.mark_edge_as_traversed(closest_node);
            match nodes_visited
                .iter()
                .find(|&x| x.index == closest_node.index)
            {
                None => {
                    start_idx = closest_node.index;
                    nodes_visited[closest_node.index] = Node::new(
                        closest_node.index,
                        closest_node.parent_idx,
                        nodes_visited[closest_node.parent_idx].dist_to_node + closest_node.dist_to_node,
                    );
                }
                Some(node) => {
                    if closest_node.dist_to_node != INFINITE_DIST {
                        //debug!("updating existing edges to nde");
                        update_existing_edges_to_node(&mut nodes_visited, closest_node);
                    }
                }
            }
            debug!("nodes visited are: {:?}", nodes_visited);
            debug!(
                "so far, dist = {}; route: {:?}",
                nodes_visited[closest_node.index].dist_to_node,
                get_route_travelled(original_start_idx, closest_node.index, &nodes_visited)
            );
        }
    }
    debug!(
        "so far, route: {:?}",
        get_route_travelled(original_start_idx, start_idx, &nodes_visited)
    );

    let nodes_in_order = get_route_travelled(original_start_idx, end_idx, &nodes_visited);

    return Ok((nodes_visited[end_idx].dist_to_node, nodes_in_order));
}

fn add_to_frontier_edges_from_node(
    graph: &mut Graph,
    start_idx: usize,
    edges_can_traverse: &mut BTreeMap<usize, Node>,
) {
    for edge in &graph.edges[start_idx] {
        if !edge.is_traversed {
            add_to_frontier(edges_can_traverse, &edge);
        }
    }

    debug!("edges can traverse {:?}", edges_can_traverse);
}

pub fn get_human_readable_solution(
    route: &str,
    graph_nodes: &Vec<GraphNode>,
    graph: &mut Graph,
) -> Result<String, String> {
    let route_names: Vec<&str> = route.split(" ").collect();
    let route_result = get_route(route_names, &graph_nodes)?;
    let (start_idx, end_idx) = route_result;

    let (dist, route) = dijkstra(start_idx, end_idx, graph)?;
    let human_readable_route = get_human_readable_route(route, &graph_nodes)?;
    let result = print_route(human_readable_route);

    return Ok(format!(
        "Route travelled: {}, with distance {}",
        result, dist
    ));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_input;

    #[test]
    fn test_dijkstra() {
        let start_idx = 0;
        let end_idx = 2;
        let edges_from_start = vec![Edge::new(0, 1, 2)];
        let edges_from_middle = vec![Edge::new(1, 0, 2), Edge::new(1, 2, 3)];
        let edges_from_end = vec![Edge::new(2, 1, 3)];

        let mut graph = Graph::new(3, vec![edges_from_start, edges_from_middle, edges_from_end]);

        let (dist, path) = dijkstra(start_idx, end_idx, &mut graph).unwrap();
        assert_eq!(dist, 5);
        assert_eq!(path, vec![0, 1, 2]);
    }
    #[test]
    fn test_multiple_start_edges() {
        let start_idx = 0;
        let end_idx = 2;
        let edges_from_start = vec![Edge::new(0, 1, 20), Edge::new(0, 1, 2)];
        let edges_from_middle = vec![Edge::new(1, 0, 2), Edge::new(1, 2, 3)];
        let edges_from_end = vec![Edge::new(2, 1, 1)];

        let mut graph = Graph::new(3, vec![edges_from_start, edges_from_middle, edges_from_end]);

        let (dist, path) = dijkstra(start_idx, end_idx, &mut graph).unwrap();
        assert_eq!(dist, 3);
        assert_eq!(path, vec![0, 1, 2]);
    }
    #[test]
    fn test_shorter_initial_route_gets_updated() {
        // assuming bidirectionality, now the edge weight for middle->end should be updated from 3 to 2.

        let mut expected_graph = Graph::new(
            3,
            vec![
                vec![Edge::new(0, 1, 2)],
                vec![Edge::new(1, 0, 2), Edge::new(1, 2, 2)],
                vec![Edge::new(2, 1, 2)],
            ],
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
            5,
            vec![
                vec![Edge::new(0, 1, 10)],
                vec![Edge::new(1, 0, 10), Edge::new(1, 3, 9), Edge::new(1, 2, 6)],
                vec![Edge::new(2, 3, 1)],
                vec![Edge::new(3, 1, 9), Edge::new(3, 2, 1), Edge::new(3, 4, 1)],
                vec![Edge::new(4, 3, 1)],
            ],
        );
        let (dist, path) = dijkstra(start_idx, end_idx, &mut graph).unwrap();
        assert_eq!(path, vec![0, 1, 2, 3, 4]);
        assert_eq!(dist, 18);
    }
    #[test]
    fn add_to_frontier_test() {
        let mut nodes_can_visit: BTreeMap<usize, Node> = BTreeMap::new();
        let mut nodes_visited: Vec<Node> = Vec::new();
        let edge_to_add = Edge::new(0, 1, 10);
        let second_edge_to_add = Edge::new(2, 1, 3);

        let start_idx = 0;
        add_to_frontier(&mut nodes_can_visit, &edge_to_add);
        add_to_frontier(&mut nodes_can_visit, &second_edge_to_add);

        let mut expected_visitable_nodes = BTreeMap::new();
        expected_visitable_nodes.insert(1, Node::new(1, 2, 3));
        assert_eq!(nodes_can_visit, expected_visitable_nodes);
    }

    #[test]
    fn find_correct_route_in_file() -> Result<(), Box<dyn std::error::Error>> {
        let start_idx = 0;
        let end_idx = 2;

        let (node_data, edge_data, routes_to_find) = parse_input::read_input("5\nCardiff\nBristol\nLondon\nYork\nBirmingham\n\n5\nYork London 194\nCardiff Bristol 44\nBristol Birmingham 88\nBristol London 114\nBirmingham London 111\n\nCardiff London".to_string())?;
        let graph_nodes: Vec<GraphNode> = parse_input::get_nodes(&node_data)?;
        let mut graph = construct_graph_from_edges(&graph_nodes, &edge_data)?;

        let (dist, path) = dijkstra(start_idx, end_idx, &mut graph).unwrap();
        assert_eq!(dist, 158);
        assert_eq!(path, vec![0, 1, 2]);
        let route = get_human_readable_route(path, &graph_nodes).unwrap();
        assert_eq!(print_route(route), "Cardiff->Bristol->London".to_string());

        Ok(())
    }
    #[test]
    fn find_correct_route_in_file_when_shorter_early_edge_is_wrong_path_simple(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let start_idx = 3;
        let end_idx = 0;
        let (node_data, edge_data, routes_to_find) = parse_input::read_input(
            "4\nA\nB\nC\nD\n\n4\nA B 1\nB D 10\nA C 2\nC D 5\n\nA D".to_string(),
        )?;
        let graph_nodes: Vec<GraphNode> = parse_input::get_nodes(&node_data)?;
        let mut graph = construct_graph_from_edges(&graph_nodes, &edge_data)?;

        let (dist, path) = dijkstra(start_idx, end_idx, &mut graph).unwrap();

        assert_eq!(dist, 7);
        assert_eq!(path, vec![3, 2, 0]);

        Ok(())
    }
    #[test]
    fn find_correct_route_in_file_when_shorter_early_edge_is_wrong_path(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let start_idx = 7;
        let end_idx = 0;
        let (node_data, edge_data, routes_to_find) = parse_input::read_input("8\nInverness\nGlasgow\nEdinburgh\nNewcastle\nManchester\nYork\nBirmingham\nLondon\n\n12\nInverness Glasgow 167\nInverness Edinburgh 158\nGlasgow Edinburgh 45\nGlasgow Newcastle 145\nGlasgow Manchester 214\nEdinburgh Newcastle 107\nNewcastle York 82\nManchester York 65\nManchester Birmingham 81\nYork Birmingham 129\nYork London 194\nBirmingham London 111\n\nLondon Inverness".to_string())?;
        let graph_nodes: Vec<GraphNode> = parse_input::get_nodes(&node_data)?;
        let mut graph = construct_graph_from_edges(&graph_nodes, &edge_data)?;

        let (dist, path) = dijkstra(start_idx, end_idx, &mut graph).unwrap();

        // current behaviour: 111 + 81 + 214 + 167 = 573 (London->Birmingham->Manchester->Glasgow->Inverness)
        // expected behaviour: 194 + 82 + 107 + 158 = 541 (London->York->Newcastle->Edinburgh->Inverness)
        // = [7, 5, 3, 2, 0] ; we get as far as to Edinburgh :(
        // this path is found when we remove the other route. do we parse the graph incorrectly then?
        // seems as though ed-> in is incorrect?
        // i.e. we don't expect the Manchester Glasgow edge to be added yet
        // removing this edge gives the correct route but dist 587? this is 43 bigger than expected :/

        // => we expect the algorithm to consider the edges in the order:
        // London Birmingham 111 [7,6] = 111 y
        // Birmingham Manchester 81 [7,6,4] = 192 y
        // Manchester York 65 [7,6,4,5] = 257 y
        // York Newcastle 82 [7, 6, 4, 5, 3] = 339 y
        // Newcastle Edinburgh 107 ? this one is skipped
        // Edinburgh Glasgow 45
        // York Birmingham 129 (both directions) this one goes before N->E??
        // Glasgow Newcastle 145 (both directions)
        // Edinburgh Inverness 158
        // Inverness Glasgow 167 (both directions)
        assert_eq!(dist, 541);
        // assert_eq!(path, vec![0,1,2]);

        Ok(())
    }
    #[test]
    fn find_self_referential_route_in_file() {
        let contents = fs::read_to_string("src/test/edge-cases.txt".to_string());
        let (node_data, edge_data, routes_to_find) =
            parse_input::read_input(contents.unwrap()).unwrap();
        let graph_nodes: Vec<GraphNode> = parse_input::get_nodes(&node_data).unwrap();
        assert_eq!(
            get_route(vec!["SelfReferential", "SelfReferential"], &graph_nodes),
            Err(
                "Route is self referential. Dist from SelfReferential to SelfReferential = 0"
                    .to_string()
            )
        );
    }
    #[test]
    fn find_disconnected_route_in_file() {
        let contents = fs::read_to_string("src/test/edge-cases.txt".to_string());
        let (node_data, edge_data, routes_to_find) =
            parse_input::read_input(contents.unwrap()).unwrap();
        let graph_nodes: Vec<GraphNode> = parse_input::get_nodes(&node_data).unwrap();
        let mut graph = construct_graph_from_edges(&graph_nodes, &edge_data).unwrap();

        assert_eq!(
            dijkstra(0, 4, &mut graph),
            Err("Are the start and end disconnected? No path found".to_string())
        );
    }
}
