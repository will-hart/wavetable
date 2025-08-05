use firewheel::{
    SilenceMask,
    channel_config::{ChannelConfig, ChannelCount},
    diff::{Diff, Patch},
    event::NodeEventList,
    node::{AudioNode, AudioNodeInfo, AudioNodeProcessor, ProcBuffers, ProcInfo, ProcessStatus},
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
        cx: firewheel::node::ConstructProcessorContext,
    ) -> impl AudioNodeProcessor {
        WaveTable::<64>::new(config.frequency, config.sample_rate)
    }
}

#[derive(Clone, Copy, PartialEq, Debug, Diff, Patch)]
pub struct WaveTable<const R: usize> {
    samples: [f32; R],
    sample_rate: u32,
    frequency: f32,
    index_delta: f32,
    index: f32,
}

impl<const R: usize> WaveTable<R> {
    /// Creates a new wave table with the given resolution `R`, frequency and
    /// sample rate.
    pub fn new(frequency: f32, sample_rate: u32) -> Self {
        let mut item = Self {
            samples: [0.0; R],
            sample_rate,
            frequency,
            index: 0.0,
            index_delta: 0.0,
        };

        item.build_table();
        item.update_index_delta();

        item
    }

    /// Updates the internal ring buffer increment speed based on the frequency
    /// and sample rate
    fn update_index_delta(&mut self) {
        self.index_delta = self.frequency * R as f32 / self.sample_rate as f32;
    }

    /// Builds the wave table samples
    fn build_table(&mut self) {
        let lf = R as f32;
        for i in 0..R {
            self.samples[i] = (2.0 * std::f32::consts::PI * i as f32 / lf).sin();
        }
    }

    /// Sets the frequency of the output, restarting the ring buffer
    pub fn set_frequency(&mut self, freq: f32) {
        self.index = 0.0;
        self.frequency = freq;
        self.update_index_delta();
    }

    pub fn set_sample_rate(&mut self, sample_rate: u32) {
        self.sample_rate = sample_rate;
        self.update_index_delta();
    }

    /// Gets a sample and increments the internal buffer to the next sample
    pub fn sample(&mut self) -> f32 {
        let current_index = self.index as usize;
        let next_index = (current_index + 1) % R;

        let next_weight = self.index - current_index as f32;
        let current_weight = 1.0 - next_weight;

        let sample =
            current_weight * self.samples[current_index] + next_weight * self.samples[next_index];

        self.index = (self.index + self.index_delta) % R as f32;

        sample
    }
}

impl<const R: usize> Default for WaveTable<R> {
    fn default() -> Self {
        Self::new(440.0, 44100)
    }
}

impl<const R: usize> AudioNodeProcessor for WaveTable<R> {
    fn process(
        &mut self,
        buffers: ProcBuffers,
        _proc_info: &ProcInfo,
        events: &mut NodeEventList,
        _logger: &mut firewheel::log::RealtimeLogger,
    ) -> ProcessStatus {
        for patch in events.drain_patches::<SineNode>() {
            match patch {
                SineNodePatch::Frequency(freq) => {
                    self.set_frequency(freq);
                }
                SineNodePatch::SampleRate(r) => self.set_sample_rate(r),
                SineNodePatch::Enabled(_e) => {
                    panic!("not supported");
                }
            };
        }

        for s in buffers.outputs[0].iter_mut() {
            *s = self.sample();
        }

        ProcessStatus::OutputsModified {
            out_silence_mask: SilenceMask::NONE_SILENT,
        }
    }
}
