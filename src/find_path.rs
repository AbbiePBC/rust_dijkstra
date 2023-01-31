use log::debug;

use crate::construct_graph::*;
use crate::parse_input::*;

pub(crate) struct PathFinder {
    pub(crate) graph: Graph,
    pub(crate) routes_to_find: Vec<(usize, usize)>,
    pub(crate) current_route_finding: usize,
    pub(crate) nodes_visited: Vec<Node>,
    pub(crate) edges_can_traverse: Vec<Edge>,
    pub(crate) solutions: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub(crate) struct Node {
    pub index: usize,
    pub parent_idx: usize,
    pub dist_to_node: usize
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


impl PathFinder {
    pub(crate) fn new(graph: Graph, routes_to_find: Vec<(usize, usize)>) -> PathFinder {
        let current_route_finding = 0;
        let solutions = Vec::with_capacity(routes_to_find.len());
        return PathFinder {
            graph,
            routes_to_find,
            current_route_finding,
            nodes_visited: vec![],
            edges_can_traverse: vec![],
            solutions,
        };
    }

    pub(crate) fn new_from_string(contents: &str) -> Result<PathFinder, String> {
        let graph = Graph::new_from_string(contents)?;
        let (_,  _ , routes_str) = split_contents_into_nodes_edges_routes(contents.to_string())?;
        let routes_to_find = parse_routes_from_string(&routes_str, &graph.graph_nodes)?;
        return Ok(PathFinder::new(graph, routes_to_find));
    }

    pub(crate) fn dijkstra_multiple_routes(&mut self) -> Result<(), String> {
        while self.current_route_finding < self.routes_to_find.len() {
            let (dist, nodes_in_order) = self.dijkstra()?;
            self.solutions.push(format!(
                "{}, dist {}",
                self.human_readable_route(nodes_in_order)?,
                dist
            ));
            self.current_route_finding += 1;
        }
        return Ok(());
    }
    pub fn reset_nodes_visited(&mut self) {
        let number_of_nodes = self.graph.graph_nodes.len();
        self.nodes_visited = Vec::with_capacity(number_of_nodes);
        for _ in 0..number_of_nodes {
            self.nodes_visited
                .push(Node::new(INFINITE_DIST, INFINITE_DIST, 0));
        }
    }

    pub fn traverse_shortest_connected_edge(&mut self) -> Option<usize> {
        // select shortest edge that is connected to the graph.
        let closest_edge = self.edges_can_traverse.next_edge_to_traverse();
        self.graph.mark_edge_as_traversed(closest_edge);
        // if we haven't visited the node that the edge takes us to already,
        // add the path to the graph, update the path (if needed)
        match self
            .nodes_visited
            .iter()
            .find(|&x| x.index == closest_edge.index_second)
        {
            None => {
                self.nodes_visited[closest_edge.index_second] = Node::new(
                    closest_edge.index_second,
                    closest_edge.index_first,
                    self.nodes_visited[closest_edge.index_first].dist_to_node + closest_edge.weight,
                );
                return Some(closest_edge.index_second);
            }
            Some(_) => {
                let dist_dec = self.nodes_visited.update_path_with_new_edge(closest_edge);
                if dist_dec != 0 {
                    self.nodes_visited.update_paths_through_node(
                        Node::new(
                            closest_edge.index_second,
                            closest_edge.index_first,
                            closest_edge.weight,
                        ),
                        dist_dec,
                    );
                }
                return None;
            }
        }
    }
    pub fn dijkstra(&mut self) -> Result<(usize, Vec<usize>), String> {
        // reset the graph inside path finder
        self.graph.mark_all_edges_as_not_traversed();
        self.reset_nodes_visited();

        let (original_start_idx, end_idx) = self.routes_to_find[self.current_route_finding];
        let mut current_idx = original_start_idx;

        // current node has no starting point, so parent = itself.
        self.nodes_visited[current_idx] = Node::new(current_idx, current_idx, 0);

        let mut look_for_node = true;
        while look_for_node {
            self.add_to_frontier_edges_from_node(current_idx);

            if self.edges_can_traverse.is_empty() {
                if self.nodes_visited.iter().find(|&x| x.index == end_idx) == None {
                    return Err("Are the start and end disconnected? No path found".to_string());
                } else {
                    debug!("stopped looking for node. edges_can_traverse.is_empty");
                    look_for_node = false;
                }
            } else {
                let new_edge_connected = self.traverse_shortest_connected_edge();
                if new_edge_connected != None {
                    current_idx = new_edge_connected
                        .expect("Expected an edge index to be returned from edge traversal");
                }
            }
        }

        let nodes_in_order = self.get_route_travelled();

        return Ok((self.nodes_visited[end_idx].dist_to_node, nodes_in_order));
    }

    fn add_to_frontier_edges_from_node(&mut self, edge_start_idx: usize) {
        for edge in &self.graph.edges[edge_start_idx] {
            if !edge.is_traversed && !self.edges_can_traverse.contains(&edge) {
                self.edges_can_traverse.push(*edge);
            }
        }
    }

    fn get_route_travelled(&self) -> Vec<usize> {
        //go backwards through the nodes to find the parent node.
        let original_start_idx = self.routes_to_find[self.current_route_finding].0;
        let end_idx = self.routes_to_find[self.current_route_finding].1;
        let mut idx = end_idx;
        let mut nodes_in_order: Vec<usize> = Vec::new();

        nodes_in_order.push(end_idx);
        while idx != original_start_idx {
            idx = self.nodes_visited[idx].parent_idx;
            nodes_in_order.push(idx);
        }

        nodes_in_order.reverse();

        return nodes_in_order;
    }

    pub fn human_readable_route(&self, nodes_in_order: Vec<usize>) -> Result<String, String> {
        let mut path_travelled: Vec<String> = Vec::new();
        for node_idx in nodes_in_order {
            let node = &self.graph.graph_nodes[node_idx];
            if node.index != node_idx {
                return Err("Error in the indexing for the route travelled.".to_string());
            } else {
                path_travelled.push(node.node_name.to_string());
            }
        }
        let mut final_path: String = path_travelled[0].to_string();
        for i in 1..path_travelled.len() {
            final_path = format!("{}->{}", final_path, path_travelled[i]);
        }

        return Ok(final_path);
    }
}

trait UpdatePath {
    fn update_path_with_new_edge(&mut self, closest_edge: Edge) -> usize;
    fn update_paths_through_node(&mut self, closest_node: Node, decrease_in_dist: usize);
}

impl UpdatePath for Vec<Node> {
    fn update_path_with_new_edge(&mut self, closest_edge: Edge) -> usize {
        let node_in_current_path = self[closest_edge.index_second];

        let node_visited_already = self
            .into_iter()
            .find(|x| x.index == closest_edge.index_first);

        match node_visited_already {
            Some(node) => {
                debug!(" we have a path to {:?}", closest_edge.index_second);
                debug!("comparing node.dist_to_node {} > closest_node.dist_to_node {}+ node_to_add_to_path.dist_to_node {}", node.dist_to_node, closest_edge.weight, node_in_current_path.dist_to_node);
                if node_in_current_path.dist_to_node > node.dist_to_node + closest_edge.weight {
                    let decrease_in_dist = node_in_current_path.dist_to_node
                        - (node.dist_to_node + closest_edge.weight);
                    self[closest_edge.index_second] = Node::new(
                        closest_edge.index_second,
                        node.index,
                        closest_edge.weight + node.dist_to_node,
                    );
                    return decrease_in_dist;
                }
            }
            None => {}
        }
        return 0;
    }

    fn update_paths_through_node(&mut self, closest_node: Node, decrease_in_dist: usize) {
        let cp = self.clone();
        for node in cp {
            // because node is not mutable (yet), overwrite Node
            if node.parent_idx == closest_node.index && node.dist_to_node != 0 {
                self[node.index] = Node::new(
                    node.index,
                    node.parent_idx,
                    self[node.index].dist_to_node - decrease_in_dist,
                );
                self.update_paths_through_node(self[node.index], decrease_in_dist);
            }
        }
        return;
    }
}

trait UpdateEdge {
    fn next_edge_to_traverse(&mut self) -> Edge;
}

impl UpdateEdge for Vec<Edge> {
    fn next_edge_to_traverse(&mut self) -> Edge {
        let mut min_weight = INFINITE_DIST;
        let mut idx_edge = 0;

        // todo: keep this in a sorted struct to minimise comparisons
        for idx in 0..self.len() {
            if self[idx].weight < min_weight {
                min_weight = self[idx].weight;
                idx_edge = idx;
            }
        }
        let edge_to_travel = self[idx_edge];
        self.remove(idx_edge);

        return edge_to_travel;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dijkstra() {
        let graph = Graph::new(
            vec![
                GraphNode::new(0, "node0".to_string()),
                GraphNode::new(1, "node1".to_string()),
                GraphNode::new(2, "node2".to_string()),
            ],
            vec![Edge::new(0, 1, 2), Edge::new(1, 2, 3)],
        );
        let mut pf = PathFinder::new(graph, vec![(0, 2)]);
        let (dist, path) = pf.dijkstra().unwrap();
        assert_eq!(dist, 5);
        assert_eq!(path, vec![0, 1, 2]);
    }
    #[test]
    fn test_multiple_start_edges() {
        let graph =
            Graph::new_from_string("3\nA\nB\nC\n\n5\nA B 20\nA B 2\nB A 2\nB C 3\nC B 1\n\nA C")
                .unwrap();

        let mut pf = PathFinder::new(graph, vec![(0, 2)]);
        let (dist, path) = pf.dijkstra().unwrap();
        assert_eq!(dist, 3);
        assert_eq!(path, vec![0, 1, 2]);
    }
    #[test]
    fn test_shorter_initial_route_gets_updated() {
        // assuming bidirectionality, now the edge weight for middle->end should be updated from 3 to 2.

        let graph = Graph::new(
            vec![
                GraphNode::new(0, "node0".to_string()),
                GraphNode::new(1, "node1".to_string()),
                GraphNode::new(2, "node2".to_string()),
            ],
            vec![Edge::new(0, 1, 2), Edge::new(1, 2, 2)],
        );

        let mut pf = PathFinder::new(graph, vec![(0, 2)]);
        let (dist, path) = pf.dijkstra().unwrap();
        assert_eq!(dist, 4);
        assert_eq!(path, vec![0, 1, 2])
    }
    #[test]
    fn find_shortest_path_branches() {
        let graph = Graph::new(
            vec![
                GraphNode::new(0, "node0".to_string()),
                GraphNode::new(1, "node1".to_string()),
                GraphNode::new(2, "node2".to_string()),
                GraphNode::new(3, "node3".to_string()),
                GraphNode::new(4, "node4".to_string()),
            ],
            vec![
                Edge::new(0, 1, 10),
                Edge::new(1, 2, 6),
                Edge::new(2, 3, 1),
                Edge::new(3, 1, 9),
                Edge::new(3, 4, 1),
            ],
        );
        let mut pf = PathFinder::new(graph, vec![(0, 4)]);
        let (dist, path) = pf.dijkstra().unwrap();
        assert_eq!(path, vec![0, 1, 2, 3, 4]);
        assert_eq!(dist, 18);
    }

    #[test]
    fn find_correct_route_in_file() {
        let graph = Graph::new_from_string("5\nCardiff\nBristol\nLondon\nYork\nBirmingham\n\n5\nYork London 194\nCardiff Bristol 44\nBristol Birmingham 88\nBristol London 114\nBirmingham London 111\n\nCardiff London").unwrap();

        let mut pf = PathFinder::new(graph, vec![(0, 2)]);
        let (dist, path) = pf.dijkstra().unwrap();
        assert_eq!(dist, 158);
        assert_eq!(path, vec![0, 1, 2]);
    }
    #[test]
    fn find_correct_route_in_file_when_shorter_early_edge_is_wrong_path_simple() {
        let graph =
            Graph::new_from_string("4\nA\nB\nC\nD\n\n4\nA B 1\nB D 10\nA C 2\nC D 5\n\nD A")
                .unwrap();

        let mut pf = PathFinder::new(graph, vec![(3, 0)]);
        let (dist, path) = pf.dijkstra().unwrap();

        assert_eq!(dist, 7);
        assert_eq!(path, vec![3, 2, 0]);
    }
    #[test]
    fn test_shortcuts_updated_correctly() {
        // if we update L->B->M->Y->N (322) to L->Y->N (276), test that we decrease the values properly
        let mut pf = PathFinder::new_from_string(
            "6\nL\nB\nM\nN\nY\nE\n\n6\nL B 111\nB M 81\nM Y 65\nY N 82\nL Y 194\nE N 107\n\nL E",
        )
        .unwrap();
        let (dist, path) = pf.dijkstra().unwrap();
        assert_eq!(path, [0, 4, 3, 5]);
        assert_eq!(dist, 276 + 107)
    }
    #[test]
    fn simplify_below_test() {
        let mut pf = PathFinder::new_from_string("8\nInverness\nGlasgow\nEdinburgh\nNewcastle\nManchester\nYork\nBirmingham\nLondon\n\n12\nInverness Glasgow 167\nInverness Edinburgh 158\nGlasgow Edinburgh 45\nGlasgow Newcastle 145\nGlasgow Manchester 214\nEdinburgh Newcastle 107\nNewcastle York 82\nManchester York 65\nManchester Birmingham 81\nYork Birmingham 129\nYork London 194\nBirmingham London 111\n\nLondon Edinburgh").unwrap();
        let (dist, path) = pf.dijkstra().unwrap();
        assert_eq!(path, [7, 5, 3, 2]);
        assert_eq!(pf.nodes_visited[7].dist_to_node, 0);
        assert_eq!(pf.nodes_visited[5].dist_to_node, 194);
        assert_eq!(pf.nodes_visited[3].index, 3);
        assert_eq!(pf.nodes_visited[3].parent_idx, 5);
        assert_eq!(pf.nodes_visited[3].dist_to_node, 194 + 82);
        assert_eq!(pf.nodes_visited[2].parent_idx, 3);
        assert_eq!(pf.nodes_visited[2].dist_to_node, 194 + 82 + 107);

        assert_eq!(dist, 194 + 82 + 107);

        let (_, path) = pf.dijkstra().unwrap();
        assert_eq!(path, [7, 5, 3, 2]);
        assert_eq!(pf.nodes_visited[7].dist_to_node, 0);
        assert_eq!(pf.nodes_visited[5].dist_to_node, 194);
        assert_eq!(pf.nodes_visited[3].index, 3);
        assert_eq!(pf.nodes_visited[3].parent_idx, 5);
        assert_eq!(pf.nodes_visited[3].dist_to_node, 194 + 82);
    }
    #[test]
    fn find_correct_route_in_file_when_shorter_early_edge_is_wrong_path() {
        let mut pf = PathFinder::new_from_string("8\nInverness\nGlasgow\nEdinburgh\nNewcastle\nManchester\nYork\nBirmingham\nLondon\n\n12\nInverness Glasgow 167\nInverness Edinburgh 158\nGlasgow Edinburgh 45\nGlasgow Newcastle 145\nGlasgow Manchester 214\nEdinburgh Newcastle 107\nNewcastle York 82\nManchester York 65\nManchester Birmingham 81\nYork Birmingham 129\nYork London 194\nBirmingham London 111\n\nLondon Inverness\nInverness London").unwrap();

        let (dist, path) = pf.dijkstra().unwrap();
        assert_eq!(path, vec![7, 5, 3, 2, 0]);
        assert_eq!(dist, 541);

        pf.current_route_finding += 1;
        let (dist, path) = pf.dijkstra().unwrap();
        assert_eq!(dist, 541);
        assert_eq!(path, vec![0, 2, 3, 5, 7]);
    }
    #[test]
    fn find_self_referential_route_in_file() {
        let info_string = "3\nA\nB\nC\n\n4\nA A 1\nA B 2\nB C 3\nA C 4\n\nA A";
        let graph = Graph::new_from_string(info_string).unwrap();
        let nodes_edges_routes: Vec<&str> = info_string.split("\n\n").collect();
        let route = parse_routes_from_string(nodes_edges_routes[2], &graph.graph_nodes);
        assert_eq!(
            route,
            Err("Route is self referential. Dist from A to A = 0".to_string())
        );
    }
    #[test]
    fn find_disconnected_route_in_file() {
        let graph = Graph::new_from_string("4\nA\nB\nC\nD\n\n4\nA B 1\nA B 2\nB C 3\nA C 4\n\nA D")
            .unwrap();

        let mut pf = PathFinder::new(graph, vec![(0, 3)]);
        assert_eq!(
            pf.dijkstra(),
            Err("Are the start and end disconnected? No path found".to_string())
        );
    }
    #[test]
    fn test_updating_path_simple() {
        let mut nodes_visited = vec![
            Node::new(0, 0, 0),
            Node::new(1, 0, 100),
            Node::new(2, 0, 300),
        ];
        let closest_edge = Edge::new(1, 2, 20);
        nodes_visited.update_path_with_new_edge(closest_edge);
        assert_eq!(
            nodes_visited,
            vec![
                Node::new(0, 0, 0),
                Node::new(1, 0, 100),
                Node::new(2, 1, 120)
            ]
        );
    }
    #[test]
    fn test_updating_path_repeatedly() {
        let mut nodes_visited = vec![
            Node::new(0, 0, 0),
            Node::new(1, 0, 300),
            Node::new(2, 1, 400),
            Node::new(3, 2, 500),
        ];
        let closest_edge = Edge::new(0, 1, 20);
        nodes_visited.update_path_with_new_edge(closest_edge);
        nodes_visited.update_paths_through_node(Node::new(1, 0, 20), 280);

        assert_eq!(
            nodes_visited,
            vec![
                Node::new(0, 0, 0),
                Node::new(1, 0, 20),
                Node::new(2, 1, 120),
                Node::new(3, 2, 220)
            ]
        );
    }
    #[test]
    fn test_updating_old_path() {
        let mut nodes_visited = vec![
            Node::new(0, 0, 0),
            Node::new(1, 0, 100),
            Node::new(2, 0, 300),
            Node::new(3, 2, 400),
        ];
        let closest_edge = Edge::new(1, 2, 20);
        nodes_visited.update_path_with_new_edge(closest_edge);
        nodes_visited.update_paths_through_node(Node::new(2, 1, 20), 300 - (100 + 20));
        assert_eq!(
            nodes_visited,
            vec![
                Node::new(0, 0, 0),
                Node::new(1, 0, 100),
                Node::new(2, 1, 120),
                Node::new(3, 2, 220)
            ]
        );
        let closest_edge = Edge::new(1, 2, 10);
        nodes_visited.update_path_with_new_edge(closest_edge);
        nodes_visited.update_paths_through_node(Node::new(2, 1, 10), 120 - (100 + 10));

        assert_eq!(
            nodes_visited,
            vec![
                Node::new(0, 0, 0),
                Node::new(1, 0, 100),
                Node::new(2, 1, 110),
                Node::new(3, 2, 210)
            ]
        );
    }

    #[test]
    fn test_update_path_to_start() {
        // taken from the london->inverness test failing, but relevant indexes here are -5 for indexing reasons.
        let mut nodes_visited = vec![
            Node::new(0, 1, 240),
            Node::new(1, 2, 111),
            Node::new(2, 2, 0),
        ];
        let closest_edge = Edge::new(2, 0, 194);
        let dec = nodes_visited.update_path_with_new_edge(closest_edge);
        nodes_visited.update_paths_through_node(Node::new(0, 2, 194), dec);
        assert_eq!(
            nodes_visited,
            vec![
                Node::new(0, 2, 194),
                Node::new(1, 2, 111),
                Node::new(2, 2, 0)
            ]
        );
    }
    #[test]
    fn ensure_last_node_is_updated_correctly() {
        let mut nodes_visited = vec![
            Node::new(0, 2, 587),
            Node::new(1, 3, 421),
            Node::new(2, 3, 429),
            Node::new(3, 5, 322),
            Node::new(4, 6, 192),
            Node::new(5, 7, 240),
            Node::new(6, 7, 111),
            Node::new(7, 7, 0),
        ];
        let closest_edge = Edge::new(7, 5, 194);
        let dec = nodes_visited.update_path_with_new_edge(closest_edge);
        nodes_visited.update_paths_through_node(Node::new(5, 7, 194), dec);

        assert_eq!(Node::new(5, 7, 194), nodes_visited[5]);
        assert_eq!(Node::new(2, 3, 383), nodes_visited[2]);
        assert_eq!(Node::new(0, 2, 541), nodes_visited[0]);
    }

    #[test]
    fn find_multiple_paths() {
        let mut pf = PathFinder::new_from_string("8\nInverness\nGlasgow\nEdinburgh\nNewcastle\nManchester\nYork\nBirmingham\nLondon\n\n12\nInverness Glasgow 167\nInverness Edinburgh 158\nGlasgow Edinburgh 45\nGlasgow Newcastle 145\nGlasgow Manchester 214\nEdinburgh Newcastle 107\nNewcastle York 82\nManchester York 65\nManchester Birmingham 81\nYork Birmingham 129\nYork London 194\nBirmingham London 111\n\nLondon Inverness\nInverness London").unwrap();

        pf.dijkstra_multiple_routes().unwrap();
        assert_eq!(
            pf.solutions[0],
            "London->York->Newcastle->Edinburgh->Inverness, dist 541"
        );
        assert_eq!(
            pf.solutions[1],
            "Inverness->Edinburgh->Newcastle->York->London, dist 541"
        );
    }
}
