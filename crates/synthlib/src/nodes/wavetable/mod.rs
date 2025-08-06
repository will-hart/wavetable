use firewheel::{
    channel_config::{ChannelConfig, ChannelCount},
    diff::{Diff, Patch},
    node::{AudioNode, AudioNodeInfo, AudioNodeProcessor},
};

use wavetable::{WaveTableSampler, WaveType};

pub mod processor;
use processor::WaveTableProcessor;

/// A node that produces different [WaveType]s from a wavetable.
#[derive(Diff, Patch, Debug, Clone, Copy, PartialEq, Default)]
pub struct WaveTableNode;

#[derive(Debug, Clone, Copy)]
pub struct WaveTableProcessorConfig {
    pub base_frequency: f32,
    pub enabled: bool,
}

impl Default for WaveTableProcessorConfig {
    fn default() -> Self {
        Self {
            base_frequency: 440.0,
            enabled: true,
        }
    }
}

impl AudioNode for WaveTableNode {
    type Configuration = WaveTableProcessorConfig;

    fn info(&self, _configuration: &Self::Configuration) -> firewheel::node::AudioNodeInfo {
        AudioNodeInfo::new()
            .debug_name("sine_node")
            .channel_config(ChannelConfig {
                num_inputs: ChannelCount::MONO,
                num_outputs: ChannelCount::MONO,
            })
    }

    fn construct_processor(
        &self,
        config: &Self::Configuration,
        cx: firewheel::node::ConstructProcessorContext,
    ) -> impl AudioNodeProcessor {
        let processor = WaveTableProcessor::new(
            config.enabled,
            config.base_frequency,
            [
                WaveTableSampler {
                    sample_rate: cx.stream_info.sample_rate.into(),
                    frequency_multiplier: 0.15,
                    wave_type: WaveType::Square,
                    ..Default::default()
                },
                WaveTableSampler {
                    sample_rate: cx.stream_info.sample_rate.into(),
                    frequency_multiplier: 1.2,
                    wave_type: WaveType::Sine,
                    ..Default::default()
                },
                WaveTableSampler {
                    sample_rate: cx.stream_info.sample_rate.into(),
                    frequency_multiplier: 0.9,
                    wave_type: WaveType::Triangle,
                    ..Default::default()
                },
            ],
        );

        processor
    }
}
