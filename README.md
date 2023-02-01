## Dijkstra

This program is a simple implementation of Dijkstra's algorithm, for the purposes of learning rust.
The 

### Required input
See the test examples (src/test) for examples.
The expected format of the input is:

```
number of nodes
node
node
node

number of edges
node node distance
node node distance
node node distance

start-node end-node
start-node end-node

```

### Running the program
1. use `$ cargo run <path/to/data>`.
2. [DEBUG MODE] set the rust environment, i.e. `$ RUST_LOG=debug cargo run <path/to/data>`


### Design 

- PathFinder: this contains all the information about the graph, the routes to find, which route is being searched for, and the nodes visited.
- Graph: this stores the connections between the nodes, and the mapping between node index and the node name
- Nodes: once nodes have been found, they are stored inside the PathFinder with information including the parent node, and the distance needed to get to that node from the start-point.

The idea in keeping the nodes separate to the graph creation was that they stored path-only information, whereas the graph information should be constant. 
In writing the code, this design changed, and the edges stored by the graph are now updated. Therefore, it makes sense now to either:
a) keep the edges stored in the graph a constant, and duplicate the information in the path finder
b) move the edges to the PathFinder, since this is already changing anyway.
//todo: I think the second version in this case makes more sense.
