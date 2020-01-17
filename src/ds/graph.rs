use std::collections::HashMap;

/// Node of the graph.
///
/// `id` - unique identifier
/// `data` (optional) - data of some type
pub struct Node<T> {
    id: usize,
    pub data: Option<T>,
}

impl<T> Node<T> {
    /// Gets an *id* of the node
    /// 
    pub fn id(&self) -> usize {
        self.id
    }
}

/// Edge of the graph.
///
/// `linked` - *id* of linked node
/// `weight` - an edge weight
pub struct Edge<W: Clone> {
    linked: usize,
    pub weight: Option<W>,
}

/// Graph data structure.
/// `T` - type of the data of graph nodes.
/// `W` - type of the *weight* of edges.
pub struct Graph<T, W: Clone> {
    oriented: bool,
    nodes: Vec<Option<Node<T>>>,
    links: HashMap<usize, Vec<Edge<W>>>,
}

impl<T, W> Graph<T, W>
where
    W: Clone,
{
    /// Creates new `Graph`.
    ///
    /// `oriented` - if `true`, then graph is oriented
    pub fn new(oriented: bool) -> Graph<T, W> {
        Graph {
            oriented,
            nodes: Vec::new(),
            links: HashMap::new(),
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
        weight: Option<W>,
    ) -> usize {
        let id = self.nodes.len();
        self.nodes.push(Some(Node { id, data }));
        self.links.insert(id, Vec::new());
        if let Some(linked) = linked_from {
            self.add_link(linked, id, weight);
        }
        id
    }

    /// Adds a new link between `from` and `to` nodes
    ///
    /// If graph is not oriented then also adds a mirror link: `from` <- `to`.
    /// `weight` (optional) - weight of the link
    pub fn add_link(&mut self, from: usize, to: usize, weight: Option<W>) {
        if let Some(list) = self.links.get_mut(&from) {
            list.push(Edge {
                linked: to,
                weight: weight.clone(),
            });
            if !self.oriented {
                if let Some(list) = self.links.get_mut(&to) {
                    list.push(Edge {
                        linked: from,
                        weight: weight.clone(),
                    });
                }
            }
        }
    }

    /// Check if graph is oriented.
    ///
    pub fn is_oriented(&self) -> bool {
        self.oriented
    }

    /// Check if graph is empty.
    ///
    pub fn is_empty(&self) -> bool {
        self.nodes_count() == 0
    }

    /// Returns count of nodes.
    ///
    pub fn nodes_count(&self) -> usize {
        let mut count = 0;
        for node in &self.nodes {
            if let Some(_) = node {
                count += 1;
            }
        }
        count
    }

    /// Returns count of edges (links).
    ///
    pub fn edges_count(&self) -> usize {
        let mut count = 0;
        for v in self.links.values() {
            count += v.len();
        }
        if self.oriented {
            count
        } else {
            count / 2
        }
    }

    // Removes node from graph by `id`.
    //
    // This removal occurs with constant complexity O(1)~
    // pub fn remove_node(&mut self, id: usize) {
    //     self.nodes[id] = None;
    //     self.links.remove(&id);
    // }
}

#[cfg(test)]
mod tests {
    use super::Graph;

    #[test]
    fn new_ok() {
        let g = Graph::<&str, usize>::new(true);
        assert_eq!(g.oriented, true);
        assert_eq!(g.nodes.len(), 0);
        assert_eq!(g.links.len(), 0);
    }

    #[test]
    fn is_oriented_ok() {
        let g = Graph::<&str, usize>::new(true);
        assert_eq!(g.oriented, true);
        assert_eq!(g.is_oriented(), true);
    }

    #[test]
    fn is_empty_ok() {
        let mut g = Graph::<&str, usize>::new(true);
        assert_eq!(g.is_empty(), true);
        g.add_node(None, None, None);
        assert_eq!(g.is_empty(), false);
    }

    #[test]
    fn add_node_ok() {
        let mut g = Graph::<&str, usize>::new(true);
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
        let mut g = Graph::<&str, usize>::new(true);
        let mut id = g.add_node(Some("First"), None, None);
        assert_eq!(id, 0);
        id = g.add_node(Some("Second"), Some(0), None);
        assert_eq!(id, 1);
        assert_eq!(g.nodes_count(), 2);
        g.add_link(1, 0, None);
        assert_eq!(g.edges_count(), 2);

        // not oriented
        let mut g = Graph::<&str, usize>::new(false);
        let mut id = g.add_node(Some("First"), None, None);
        assert_eq!(id, 0);
        id = g.add_node(Some("Second"), Some(0), None);
        assert_eq!(id, 1);
        assert_eq!(g.nodes_count(), 2);
        assert_eq!(g.edges_count(), 1);
    }

    // #[test]
    // fn remove_node_ok() {
    //     let mut g = Graph::<&str, usize>::new(true);
    //     let mut id = g.add_node(Some("First"), None, None);
    //     assert_eq!(id, 0);
    //     id = g.add_node(Some("Second"), Some(0), None);
    //     assert_eq!(id, 1);
    //     id = g.add_node(Some("Third"), Some(0), None);
    //     assert_eq!(id, 2);
    //     assert_eq!(g.nodes_count(), 3);
    //     assert_eq!(g.edges_count(), 2);
    //     g.remove_node(1);
    //     assert_eq!(g.nodes_count(), 2);
    //     //assert_eq!(g.edges_count(), 1);
    // }
}
