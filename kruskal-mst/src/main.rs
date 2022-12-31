#[derive(Clone)]
struct Edge {
    from: usize,
    to: usize,
    weight: i32,
}

impl Edge {
    fn new(from: usize, to: usize, weight: i32) -> Self {
        Self { from, to, weight }
    }
}

fn sort_edges(edges: &mut Vec<Edge>) {
    edges.sort_by(|a, b| a.weight.cmp(&b.weight));
}

fn find(parent: &mut [usize], node: usize) -> usize {
    if parent[node] == node {
        node
    } else {
        parent[node] = find(parent, parent[node]);
        parent[node]
    }
}

fn add_edge(parent: &mut [usize], rank: &mut [usize], edge: &Edge) -> bool {
    let root1 = find(parent, edge.from);
    let root2 = find(parent, edge.to);
    if root1 == root2 {
        return false;
    }
    if rank[root1] < rank[root2] {
        parent[root1] = root2;
    } else if rank[root1] > rank[root2] {
        parent[root2] = root1;
    } else {
        parent[root1] = root2;
        rank[root2] += 1;
    }
    true
}

fn kruskal(n: usize, edges: &[Edge]) -> Vec<Edge> {
    let mut result = Vec::new();
    let mut parent = (0..n).collect::<Vec<_>>();
    let mut rank = vec![0; n];
    let mut edges = edges.to_vec();
    sort_edges(&mut edges);
    for edge in edges {
        if add_edge(&mut parent, &mut rank, &edge) {
            result.push(edge.clone());
        }
    }
    result
}

fn main() {
    let edges = [
        Edge::new(0, 1, 10),
        Edge::new(0, 2, 6),
        Edge::new(0, 3, 5),
        Edge::new(1, 3, 15),
        Edge::new(2, 3, 4),
    ];
    let minimum_spanning_tree = kruskal(4, &edges);
    for edge in minimum_spanning_tree {
        println!("{} -> {}: {}", edge.from, edge.to, edge.weight);
    }
}
