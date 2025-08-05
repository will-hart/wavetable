use firewheel::{FirewheelContext, error::UpdateError, node::NodeID};

use crate::node::SineNode;

pub struct AudioSystem {
    pub cx: FirewheelContext,
    sine_node: SineNode,
    sine_node_id: NodeID,
}

impl AudioSystem {
    pub fn new() -> Self {
        let mut cx = FirewheelContext::new(Default::default());
        cx.start_stream(Default::default()).unwrap();

        let sine_node = SineNode::default();
        let sine_node_id = cx.add_node(sine_node, None);

        let graph_out_node_id = cx.graph_out_node_id();
        cx.connect(sine_node_id, graph_out_node_id, &[(0, 0), (0, 1)], false)
            .expect("connect sine node to graph");

        Self {
            cx,
            sine_node,
            sine_node_id,
        }
    }

    pub fn update(&mut self) {
        if let Err(e) = self.cx.update() {
            println!("{:?}", e);
            if let UpdateError::StreamStoppedUnexpectedly(_) = e {
                panic!("Stream stopped unexpectedly");
            }
        }
    }
}
