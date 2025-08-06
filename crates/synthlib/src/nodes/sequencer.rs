use firewheel::{
    SilenceMask,
    channel_config::{ChannelConfig, ChannelCount},
    diff::{Diff, Patch},
    node::{AudioNode, AudioNodeInfo, AudioNodeProcessor, ProcessStatus},
};

/// A Sequencer is a node that has no inputs and plays a sequence of "notes" and
/// "pauses" on the output. It can be routed into another node such as a [WaveTableNode]
/// to control the output frequency.
///
/// The frequency is converted into a -1 to 1 signal with -1 equal to 20 Hz
/// and +1 being 20kHz.
#[derive(Diff, Patch, Debug, Clone, Copy, PartialEq, Default)]
pub struct SequencerNode;

#[inline]
pub fn frequency_to_voltage(frequency: f32) -> f32 {
    (2.0 * (frequency - 20.0) / 19980.0 - 1.0).clamp(-1.0, 1.0)
}

#[derive(Diff, Patch, Debug, Clone, Copy, PartialEq)]
pub struct SequenceStep {
    frequency: Option<f32>,
    duration_ms: u32,
}

impl Default for SequenceStep {
    fn default() -> Self {
        Self {
            frequency: None,
            duration_ms: 1000,
        }
    }
}

impl SequenceStep {
    pub fn pause(duration_ms: u32) -> Self {
        Self {
            frequency: None,
            duration_ms,
        }
    }
    pub fn note(frequency: f32, duration_ms: u32) -> Self {
        Self {
            frequency: Some(frequency),
            duration_ms,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SequencerConfig {
    pub sequences: Vec<SequenceStep>,
}

impl Default for SequencerConfig {
    fn default() -> Self {
        Self {
            sequences: vec![
                SequenceStep::note(440.0, 1000),
                SequenceStep::pause(500),
                SequenceStep::note(220.0, 500),
                SequenceStep::pause(250),
                SequenceStep::note(330.0, 500),
                SequenceStep::pause(250),
            ],
        }
    }
}

impl AudioNode for SequencerNode {
    type Configuration = SequencerConfig;

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
        configuration: &Self::Configuration,
        _cx: firewheel::node::ConstructProcessorContext,
    ) -> impl firewheel::node::AudioNodeProcessor {
        SequencerProcessor::new(configuration.sequences.clone())
    }
}

#[derive(Debug)]
pub struct SequencerProcessor {
    steps: Vec<SequenceStep>,
    current_time: f32,
    current_index: usize,
}

impl SequencerProcessor {
    pub fn new(steps: Vec<SequenceStep>) -> Self {
        Self {
            steps: if steps.is_empty() {
                vec![SequenceStep {
                    frequency: None,
                    duration_ms: 1000,
                }]
            } else {
                steps
            },
            current_time: 0.0,
            current_index: 0,
        }
    }

    /// Returns the next sequence to play, with the given number of frames
    /// at the given output level. Stops at "sequence" boundaries, meaning that
    /// this function should be called until the number of samples returned is 0
    fn get_samples(&mut self, sample_rate: u32, samples: usize) -> (usize, f32) {
        if samples == 0 {
            return (0, 0.0);
        }

        let step_time = self.steps[self.current_index].duration_ms as f32;

        // the number of samples we return is the smaller of the number of samples
        // requested and the number of samples left in the step
        //  no need to divide by 1000 here (step time in ms) because sample rate is in kHz
        let num_samples = (step_time - self.current_time) * sample_rate as f32;
        let num_samples = num_samples.min(samples as f32);

        let step_value = self.steps[self.current_index]
            .frequency
            // .map(frequency_to_voltage)
            .unwrap_or_default();

        // lets work out how long to increment the current time by
        // and if we've reached the end of the step we increment.
        // We have to divide by 1000 here as step_time is in ms and
        // current time is in sec
        self.current_time += num_samples / sample_rate as f32;
        if self.current_time >= step_time / 1000.0 {
            // should be ok to set this to 0.0 as we've limited the smaples to the remaining time
            self.current_time = 0.0;
            self.current_index = (self.current_index + 1) % self.steps.len();
        }

        (num_samples as usize, step_value)
    }
}

impl AudioNodeProcessor for SequencerProcessor {
    fn process(
        &mut self,
        buffers: firewheel::node::ProcBuffers,
        proc_info: &firewheel::node::ProcInfo,
        _events: &mut firewheel::event::NodeEventList,
        _logger: &mut firewheel::log::RealtimeLogger,
    ) -> firewheel::node::ProcessStatus {
        let mut sample_count = buffers.outputs[0].len();
        let mut current_sample_idx = 0;
        let sample_rate: u32 = proc_info.sample_rate.into();

        while sample_count > 0 {
            let (samples, value) = self.get_samples(sample_rate, sample_count);

            for idx in current_sample_idx..(current_sample_idx + samples) {
                buffers.outputs[0][idx] = value;
            }

            current_sample_idx += samples;
            sample_count -= samples;
        }

        ProcessStatus::OutputsModified {
            out_silence_mask: SilenceMask::NONE_SILENT,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_freq_to_v() {
        // limits
        assert_eq!(frequency_to_voltage(0.0), -1.0);
        assert_eq!(frequency_to_voltage(20.0), -1.0);
        assert_eq!(frequency_to_voltage(20_000.0), 1.0);
        assert_eq!(frequency_to_voltage(30_000.0), 1.0);
    }
}
