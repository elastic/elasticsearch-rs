extern crate url;

use url::Url;

pub struct Node {
    pub url: Url,
}

pub trait NodePool {
    fn next(&self) -> &Node;
}

pub struct SingleNodePool {
    node: Node,
}

impl SingleNodePool {
    pub fn new(url: String) -> SingleNodePool {
        let u = Url::parse(&url).expect("Not a valid URL");
        let node = Node { url: u };
        SingleNodePool { node }
    }
}

impl NodePool for SingleNodePool {
    fn next(&self) -> &Node {
        &self.node
    }
}
