/// Edge of the graph.
///
/// `item.0` - a first node
/// `item.1` - a second node
/// Direction of link: first <- second
/// `item.2` - a node weight (optional)
struct Edge(usize, usize, Option<f64>);

/// Node of the graph.
///
/// `id` - unique identifier
/// `data` (optional) - some type of data
pub struct Node<T> {
    id: usize,
    pub data: Option<T>,
}

pub struct Graph<T> {
    oriented: bool,
    nodes: Vec<Node<T>>,
    edges: Vec<Edge>,
    next_id: usize,
    modified: bool,
}

impl<T> Graph<T> {
    /// Creates new `Graph`.
    ///
    /// `oriented` - if `true`, then graph is oriented
    pub fn new(oriented: bool) -> Graph<T> {
        Graph {
            oriented,
            nodes: Vec::new(),
            edges: Vec::new(),
            next_id: 0,
            modified: false,
        }
    }

    /// Adds a new node into graph.
    ///
    /// `data` (optional) - a nodes data
    /// Optional adds a link: `linked_from` -> new node.
    /// If graph is not oriented then also adds a mirror link: `linked_from` <- new node.
    /// `weight` (optional) - weight of the link
    /// Return ID of the new node.
    pub fn add_node(
        &mut self,
        data: Option<T>,
        linked_from: Option<usize>,
        weight: Option<f64>,
    ) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        self.nodes.push(Node { id, data });
        if let Some(linked) = linked_from {
            self.add_link(linked, id, weight);
        }
        self.modified = true;
        id
    }

    /// Adds a new link between `from` and `to` nodes
    ///
    /// If graph is not oriented then also adds a mirror link: `from` <- `to`.
    /// `weight` (optional) - weight of the link
    pub fn add_link(&mut self, from: usize, to: usize, weight: Option<f64>) {
        self.edges.push(Edge(to, from, weight));
        if !self.oriented {
            self.edges.push(Edge(from, to, weight));
        }
        self.modified = true;
    }

    /// Check if graph is oriented.
    ///
    pub fn is_oriented(&self) -> bool {
        self.oriented
    }

    /// Check if graph is empty.
    ///
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// Returns count of nodes.
    ///
    pub fn nodes_count(&self) -> usize {
        self.nodes.len()
    }

    /// Returns count of edges (links).
    ///
    pub fn edges_count(&self) -> usize {
        if self.oriented {
            self.edges.len()
        } else {
            self.edges.len() / 2
        }
    }

    /// Removes node from graph by `id`.
    ///
    /// This removal occurs with logarithmic complexity.
    pub fn remove_node(&mut self, id: usize) {
        if let Some(idx) = self.index_of(id) {
            self.nodes.remove(idx);
        }
        self.modified = true;
    }

    /// Gets index of the node by `id`.
    ///
    /// Search occurs with logarithmic complexity.
    fn index_of(&self, id: usize) -> Option<usize> {
        self.nodes.binary_search_by(|node| node.id.cmp(&id)).ok()
    }
}

#[cfg(test)]
mod tests {
    use super::Graph;

    #[test]
    fn new_ok() {
        let g = Graph::<&str>::new(true);
        assert_eq!(g.oriented, true);
        assert_eq!(g.modified, false);
        assert_eq!(g.next_id, 0);
        assert_eq!(g.nodes.len(), 0);
        assert_eq!(g.edges.len(), 0);
    }

    #[test]
    fn is_oriented_ok() {
        let g = Graph::<&str>::new(true);
        assert_eq!(g.oriented, true);
        assert_eq!(g.is_oriented(), true);
    }

    #[test]
    fn is_empty_ok() {
        let mut g = Graph::<&str>::new(true);
        assert_eq!(g.is_empty(), true);
        g.add_node(None, None, None);
        assert_eq!(g.is_empty(), false);
    }

    #[test]
    fn add_node_ok() {
        let mut g = Graph::<&str>::new(true);
        let mut id = g.add_node(Some("First"), None, None);
        assert_eq!(id, 0);
        id = g.add_node(Some("Second"), Some(0), None);
        assert_eq!(id, 1);
        assert_eq!(g.nodes_count(), 2);
        assert_eq!(g.edges_count(), 1);
    }

    #[test]
    fn add_link_ok() {
        // oriented
        let mut g = Graph::<&str>::new(true);
        let mut id = g.add_node(Some("First"), None, None);
        assert_eq!(id, 0);
        id = g.add_node(Some("Second"), Some(0), None);
        assert_eq!(id, 1);
        assert_eq!(g.nodes_count(), 2);
        g.add_link(1, 0, None);
        assert_eq!(g.edges_count(), 2);

        // not oriented
        let mut g = Graph::<&str>::new(false);
        let mut id = g.add_node(Some("First"), None, None);
        assert_eq!(id, 0);
        id = g.add_node(Some("Second"), Some(0), None);
        assert_eq!(id, 1);
        assert_eq!(g.nodes_count(), 2);
        assert_eq!(g.edges_count(), 1);
    }

    #[test]
    fn remove_node_ok() {
        let mut g = Graph::<&str>::new(true);
        let mut id = g.add_node(Some("First"), None, None);
        assert_eq!(id, 0);
        id = g.add_node(Some("Second"), Some(0), None);
        assert_eq!(id, 1);
        id = g.add_node(Some("Third"), Some(0), None);
        assert_eq!(id, 2);
        assert_eq!(g.nodes_count(), 3);
        g.remove_node(1);
        assert_eq!(g.nodes_count(), 2);
    }
}
