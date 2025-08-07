use firewheel::{FirewheelContext, diff::Memo, error::UpdateError, node::NodeID};

use crate::nodes::{filter::FilterNode, sequencer::SequencerNode, wavetable::WaveTableNode};

pub struct AudioSystem {
    pub cx: FirewheelContext,
    #[expect(dead_code)]
    sequencer_node: Memo<SequencerNode>,
    #[expect(dead_code)]
    sequencer_node_id: NodeID,
    #[expect(dead_code)]
    wave_node: Memo<WaveTableNode>,
    #[expect(dead_code)]
    wave_node_id: NodeID,
    filter_node: Memo<FilterNode>,
    filter_node_id: NodeID,
}

impl AudioSystem {
    pub fn new() -> Self {
        let mut cx = FirewheelContext::new(Default::default());
        cx.start_stream(Default::default()).unwrap();

        let sequencer_node = SequencerNode::default();
        let wave_node = WaveTableNode::default();

        let sequencer_node_id = cx.add_node(sequencer_node, None);
        let wave_node_id = cx.add_node(wave_node, None);

        let filter_node = FilterNode::default();
        let filter_node_id = cx.add_node(filter_node, None);

        let graph_out_node_id = cx.graph_out_node_id();

        cx.connect(sequencer_node_id, wave_node_id, &[(0, 0)], false)
            .expect("connect sequencer node to graph");
        cx.connect(wave_node_id, filter_node_id, &[(0, 0)], false)
            .expect("connect sine node to graph");
        cx.connect(filter_node_id, graph_out_node_id, &[(0, 0), (0, 1)], false)
            .expect("connect filter to graph");

        Self {
            cx,
            wave_node: Memo::new(wave_node),
            sequencer_node: Memo::new(sequencer_node),
            sequencer_node_id,
            wave_node_id,
            filter_node: Memo::new(filter_node),
            filter_node_id,
        }
    }

    pub fn update(&mut self) {
        self.filter_node.cutoff_hz = (self.filter_node.cutoff_hz + 0.1) % 1500.0;
        self.filter_node
            .update_memo(&mut self.cx.event_queue(self.filter_node_id));

        if let Err(e) = self.cx.update() {
            println!("{:?}", e);
            if let UpdateError::StreamStoppedUnexpectedly(_) = e {
                panic!("Stream stopped unexpectedly");
            }
        }
    }
}
