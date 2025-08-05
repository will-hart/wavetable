use firewheel::{
    channel_config::{ChannelConfig, ChannelCount},
    diff::{Diff, Patch},
    node::{AudioNode, AudioNodeInfo, AudioNodeProcessor},
};

use crate::{
    processor::WaveTableProcessor,
    wavetable::{WaveTable, WaveTableSampler},
};

#[derive(Diff, Patch, Debug, Clone, Copy, PartialEq)]
pub struct SineNode {
    frequency: f32,
    sample_rate: u32,
    enabled: bool,
}

impl Default for SineNode {
    fn default() -> Self {
        Self {
            frequency: 440.0,
            sample_rate: 44_100,
            enabled: true,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SineNodeConfig {
    pub frequency: f32,
    pub sample_rate: u32,
}

impl Default for SineNodeConfig {
    fn default() -> Self {
        Self {
            frequency: 440.0,
            sample_rate: 44100,
        }
    }
}

impl AudioNode for SineNode {
    type Configuration = SineNodeConfig;

    fn info(&self, _configuration: &Self::Configuration) -> firewheel::node::AudioNodeInfo {
        AudioNodeInfo::new()
            .debug_name("sine_node")
            .channel_config(ChannelConfig {
                num_inputs: ChannelCount::ZERO,
                num_outputs: ChannelCount::MONO,
            })
    }

    fn construct_processor(
        &self,
        config: &Self::Configuration,
        _cx: firewheel::node::ConstructProcessorContext,
    ) -> impl AudioNodeProcessor {
        WaveTableProcessor::new([
            WaveTableSampler::new(170.0, 44_100),
            WaveTableSampler::new(220.0, 44_100),
            WaveTableSampler::new(1220.0, 44_100),
        ])
    }
}
