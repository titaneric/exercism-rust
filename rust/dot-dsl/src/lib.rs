pub mod graph {
    pub mod graph_items {
        pub mod edge {
            #[derive(Debug, PartialEq, Clone)]
            pub struct Edge<'a> {
                head: &'a str,
                tail: &'a str,
                attrs: &'a [(&'a str, &'a str)],
            }
            impl<'a> Edge<'a> {
                pub fn new(head: &'a str, tail: &'a str) -> Self {
                    Self {
                        head,
                        tail,
                        attrs: &[],
                    }
                }
                pub fn with_attrs(mut self, attrs: &'a [(&'a str, &'a str)]) -> Self {
                    self.attrs = attrs;
                    self
                }
            }
        }
        pub mod node {
            #[derive(Debug, PartialEq, Clone)]
            pub struct Node<'a> {
                name: &'a str,
                attrs: &'a [(&'a str, &'a str)],
            }
            impl<'a> Node<'a> {
                pub fn new(name: &'a str) -> Self {
                    Self { name, attrs: &[] }
                }
                pub fn with_attrs(mut self, attrs: &'a [(&'a str, &'a str)]) -> Self {
                    self.attrs = attrs;
                    self
                }
                pub fn get_attr(&self, attr: &str) -> Option<&str> {
                    self.attrs
                        .iter()
                        .find(|&(k, _)| k == &attr)
                        .and_then(|x| Some(x.1))
                }
            }
        }
    }
    use graph_items::edge::Edge;
    use graph_items::node::Node;
    use std::collections::HashMap;
    pub struct Graph<'a> {
        pub nodes: Vec<Node<'a>>,
        pub edges: Vec<Edge<'a>>,
        pub attrs: HashMap<String, String>,
    }

    impl<'a> Graph<'a> {
        pub fn new() -> Self {
            Self {
                nodes: Vec::new(),
                edges: Vec::new(),
                attrs: HashMap::new(),
            }
        }
        pub fn with_nodes(mut self, nodes: &'a [Node]) -> Self {
            self.nodes = nodes.to_vec();
            self
        }
        pub fn with_edges(mut self, edges: &'a [Edge]) -> Self {
            self.edges = edges.to_vec();
            self
        }
        pub fn with_attrs(mut self, attrs: &'a [(&str, &str)]) -> Self {
            for (k, v) in attrs {
                self.attrs.insert(k.to_string(), v.to_string());
            }
            self
        }
        pub fn get_node(&self, name: &str) -> Option<&Node> {
            self.nodes.iter().find(|&node| node == &Node::new(name))
        }
    }
}
