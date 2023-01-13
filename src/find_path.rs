use log::debug;
use std::collections::btree_map::BTreeMap;

use crate::construct_graph::*;
use crate::parse_input::get_route;

#[derive(Debug, Clone, PartialEq)]
struct Node {
    index: usize,
    parent_idx: usize,
    dist_to_node: usize,
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
    while idx != original_start_idx {
        idx = nodes_visited[idx].parent_idx;
        nodes_in_order.push(idx);
    }

    nodes_in_order.reverse();
    debug!("Nodes in order: {:?}", &nodes_in_order);

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

fn index_of_closest_node(nodes_can_visit: &BTreeMap<usize, Node>) -> usize {
    let mut min_weight = INFINITE_DIST;
    let mut index_to_remove = INFINITE_DIST;
    for (_, node) in nodes_can_visit {
        if node.dist_to_node < min_weight {
            min_weight = node.dist_to_node;
            index_to_remove = node.index;
        }
    }
    return index_to_remove;
}

fn add_to_frontier(
    nodes_can_visit: &mut BTreeMap<usize, Node>,
    nodes_visited: &Vec<Node>,
    edge_to_add: &Edge,
    start_idx: usize,
) {
    if nodes_can_visit.contains_key(&edge_to_add.index_second) {
        debug!(
            "we already have the ability to visit node {:?};{:?}",
            edge_to_add.index_second,
            nodes_can_visit.entry(edge_to_add.index_second)
        );
        debug!(
            "the proposed edge to add is {:?} from {}",
            edge_to_add, start_idx
        );
        nodes_can_visit
            .entry(edge_to_add.index_second)
            .and_modify(|curr_node| {
                debug! {"comparing the new edge dist ({}) with the old", (edge_to_add.weight)}
                if edge_to_add.weight < curr_node.dist_to_node {
                    curr_node.dist_to_node = edge_to_add.weight;
                    curr_node.parent_idx = edge_to_add.index_first;
                }
            });
    } else if (None
        == nodes_visited
            .iter()
            .find(|&x| x.index == edge_to_add.index_second))
        && edge_to_add.index_second != start_idx
    {
        // if not present, and we haven't visited the node
        nodes_can_visit.insert(
            edge_to_add.index_second,
            Node {
                index: edge_to_add.index_second,
                parent_idx: start_idx,
                dist_to_node: edge_to_add.weight,
            },
        );
    }
    debug!("nodes can visit: {:?}", nodes_can_visit);
}

fn dijkstra(
    mut start_idx: usize,
    end_idx: usize,
    graph: &Graph,
) -> Result<(usize, Vec<usize>), String> {
    let original_start_idx = start_idx;
    let mut parent_idx = start_idx;

    let number_of_nodes = graph.number_of_nodes;
    //todo: use a binary search tree here to avoid needing to allocate space for the whole vector.
    let mut nodes_visited: Vec<Node> = Vec::with_capacity(number_of_nodes);
    for _ in 0..number_of_nodes {
        nodes_visited.push(Node {
            index: INFINITE_DIST,
            parent_idx: INFINITE_DIST,
            dist_to_node: 0,
        });
    }
    nodes_visited[start_idx] = Node {
        index: start_idx,
        parent_idx,
        dist_to_node: 0,
    };

    let mut nodes_can_visit: BTreeMap<usize, Node> = BTreeMap::new();

    while start_idx != end_idx {
        for edge in &graph.edges[start_idx] {
            add_to_frontier(&mut nodes_can_visit, &nodes_visited, edge, start_idx);
            debug!("now adding edge: {:?}", edge);
        }
        if nodes_can_visit.is_empty() {
            return Err("Are the start and end disconnected? No path found".to_string());
        }
        debug!("nodes can visit: {:?}", nodes_can_visit);

        let index_to_remove = index_of_closest_node(&nodes_can_visit);
        let closest_node = nodes_can_visit
            .remove(&index_to_remove)
            .ok_or("Error in path finding".to_string())?;
        let can_go_to_closest_node = (closest_node.index != start_idx)
            && (nodes_visited
                .iter()
                .find(|&x| x.index == closest_node.index)
                == None);
        if can_go_to_closest_node {
            start_idx = closest_node.index;
            parent_idx = closest_node.parent_idx;
            nodes_visited[start_idx] = Node {
                index: start_idx,
                parent_idx,
                dist_to_node: nodes_visited[parent_idx].dist_to_node + closest_node.dist_to_node,
            };
        }
    }
    let nodes_in_order = get_route_travelled(original_start_idx, end_idx, &nodes_visited);

    return Ok((nodes_visited[end_idx].dist_to_node, nodes_in_order));
}

pub fn get_human_readable_solution(
    route: &str,
    graph_nodes: &Vec<GraphNode>,
    graph: &Graph,
) -> Result<String, String> {
    let route_names: Vec<&str> = route.split(" ").collect();
    let route_result = get_route(route_names, &graph_nodes)?;
    let (start_idx, end_idx) = route_result;
    debug!("finding route from {} to {}", start_idx, end_idx);

    let (dist, route) = dijkstra(start_idx, end_idx, &graph)?;
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
    use assert_cmd::Command;
    use predicates::prelude::*;
    #[test]
    fn test_dijkstra() {
        let start_idx = 0;
        let end_idx = 2;
        let edges_from_start = vec![create_new_edge(0, 1, 2)];
        let edges_from_middle = vec![create_new_edge(2, 0, 2), create_new_edge(1, 2, 3)];
        let edges_from_end = vec![create_new_edge(2, 1, 3)];

        let graph = Graph {
            number_of_nodes: 3,
            edges: vec![edges_from_start, edges_from_middle, edges_from_end],
        };

        let (dist, _) = dijkstra(start_idx, end_idx, &graph).unwrap();
        assert_eq!(dist, 5);
    }
    #[test]
    fn test_multiple_start_edges() {
        let start_idx = 0;
        let end_idx = 2;
        let edges_from_start = vec![create_new_edge(0, 1, 20), create_new_edge(0, 1, 2)];
        let edges_from_middle = vec![create_new_edge(0, 1, 2), create_new_edge(1, 2, 3)];
        let edges_from_end = vec![create_new_edge(2, 3, 1)];

        let graph = Graph {
            number_of_nodes: 3,
            edges: vec![edges_from_start, edges_from_middle, edges_from_end],
        };

        let (dist, _) = dijkstra(start_idx, end_idx, &graph).unwrap();
        assert_eq!(dist, 5);
    }
    #[test]
    fn test_shorter_initial_route_gets_updated() {
        // assuming bidirectionality, now the edge weight for middle->end should be updated from 3 to 2.

        let expected_graph = Graph {
            number_of_nodes: 3,
            edges: vec![
                vec![create_new_edge(0, 1, 2)],
                vec![create_new_edge(1, 0, 2), create_new_edge(1, 2, 2)],
                vec![create_new_edge(2, 1, 2)],
            ],
        };
        let (dist, _) = dijkstra(0, 2, &expected_graph).unwrap();
        assert_eq!(dist, 4);
    }
    #[test]
    fn test_edges_not_explicitly_in_both_directions() {
        let start_idx = 0;
        let end_idx = 2;
        let edges_from_start = vec![create_new_edge(2, 1, 2)];
        let edges_from_middle = vec![create_new_edge(1, 2, 3)];

        let graph = Graph {
            number_of_nodes: 3,
            edges: vec![edges_from_start, edges_from_middle],
        };

        let (dist, _) = dijkstra(start_idx, end_idx, &graph).unwrap();
        assert_eq!(dist, 5);
    }
    #[test]
    fn find_shortest_path_branches() {
        let start_idx = 0;
        let end_idx = 4;
        let graph = Graph {
            number_of_nodes: 5,
            edges: vec![
                vec![create_new_edge(0, 1, 10)],
                vec![
                    create_new_edge(1, 0, 10),
                    create_new_edge(1, 3, 9),
                    create_new_edge(1, 2, 6),
                ],
                vec![create_new_edge(2, 3, 1)],
                vec![
                    create_new_edge(3, 1, 9),
                    create_new_edge(3, 2, 1),
                    create_new_edge(3, 4, 1),
                ],
                vec![create_new_edge(4, 3, 1)],
            ],
        };
        let (dist, path) = dijkstra(start_idx, end_idx, &graph).unwrap();
        assert_eq!(path, vec![0, 1, 2, 3, 4]);
        assert_eq!(dist, 18);
    }
    #[test]
    fn add_to_frontier_test() {
        let mut nodes_can_visit: BTreeMap<usize, Node> = BTreeMap::new();
        let nodes_visited: Vec<Node> = Vec::new();
        let edge_to_add = create_new_edge(0, 1, 10);
        let second_edge_to_add = create_new_edge(2, 1, 3);

        let start_idx = 0;
        add_to_frontier(
            &mut nodes_can_visit,
            &nodes_visited,
            &edge_to_add,
            start_idx,
        );
        add_to_frontier(
            &mut nodes_can_visit,
            &nodes_visited,
            &second_edge_to_add,
            start_idx,
        );

        let mut expected_visitable_nodes = BTreeMap::new();
        expected_visitable_nodes.insert(
            1,
            Node {
                index: 1,
                parent_idx: 2,
                dist_to_node: 3,
            },
        );
        assert_eq!(nodes_can_visit, expected_visitable_nodes);
    }

    #[test]
    fn find_correct_route_in_file() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("rust_dijkstra")?;
        cmd.arg("src/test/uk.txt".to_string());
        cmd.assert().success().stdout(predicate::str::contains(
            "Route travelled: Glasgow->Edinburgh, with distance 45\n",
        ));

        Ok(())
        //todo test more complex routes than this.
        //test output when multiple paths have the same length.
    }
    #[test]
    fn find_self_referential_route_in_file() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("rust_dijkstra")?;
        cmd.arg("src/test/edge-cases.txt".to_string());
        cmd.assert().success().stdout(predicate::str::contains(
            "Route is self referential. Dist from SelfReferential to SelfReferential = 0",
        ));
        Ok(())
    }
    #[test]
    fn find_disconnected_route_in_file() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("rust_dijkstra")?;
        cmd.arg("src/test/edge-cases.txt".to_string());

        cmd.assert().success().stdout(predicate::str::contains(
            "Are the start and end disconnected? No path found",
        ));
        Ok(())
    }
}
