use std::sync::Arc;

use bimap::BiHashMap;
use petgraph::{
    stable_graph::{NodeIndex, StableGraph},
    Directed,
};

use crate::Idable;

use crate::models::types::{Attribute, Id};

pub type Ref<T> = Arc<T>;

pub struct DataGraph<Id, Data>
where
    Id: std::hash::Hash + Eq,
    Data: Idable<Id = Id>,
{
    pub nodes: BiHashMap<Id, NodeIndex<u32>>,
    pub graph: StableGraph<Data, (), Directed, u32>,
}

pub type AttributeDataGraph = DataGraph<Id, Ref<Attribute>>;

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use bimap::BiMap;
    use petgraph::{
        dot::{Config, Dot},
        stable_graph::StableDiGraph,
    };

    use crate::{
        collections::AttributeDataGraph,
        models::types::{Attribute, Id},
        Idable,
    };

    #[test]
    fn it_works() -> eyre::Result<()> {
        let a0 = Arc::new(Attribute {
            id: Id("0".into()),
            name: "Element".into(),
        });
        let a1 = Arc::new(Attribute {
            id: Id("1".into()),
            name: "Fire".into(),
        });

        let mut g = AttributeDataGraph {
            nodes: BiMap::new(),
            graph: StableDiGraph::new(),
        };

        let idx0 = g.graph.add_node(a0.clone());
        g.nodes.insert(a0.uid().to_owned(), idx0);

        let idx1 = g.graph.add_node(a1.clone());
        g.nodes.insert(a1.uid().to_owned(), idx1);
        g.graph.add_edge(idx1, idx0, ());

        println!("{:?}", Dot::with_config(&g.graph, &[Config::EdgeNoLabel]));
        println!("{:?}", g.nodes);

        Ok(())
    }
}
