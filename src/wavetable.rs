use firewheel::diff::{Diff, Patch};

/// A table that contains a sine wave and allows for interpolation between points
/// using a WaveTableSampler
#[derive(Clone, Copy, PartialEq, Debug, Diff, Patch)]
pub struct WaveTable<const R: usize> {
    samples: [f32; R],
}

impl<const R: usize> Default for WaveTable<R> {
    fn default() -> Self {
        Self {
            samples: Self::build_table(),
        }
    }
}

impl<const R: usize> WaveTable<R> {
    /// Builds the wave table samples
    fn build_table() -> [f32; R] {
        let mut samples = [0.0; R];
        let lf = R as f32;
        for i in 0..R {
            samples[i] = (2.0 * std::f32::consts::PI * i as f32 / lf).sin();
        }

        samples
    }
}

#[derive(Clone, Copy, PartialEq, Debug, Diff, Patch)]
/// Samples a wave table at a given frequency and sample rate
pub struct WaveTableSampler {
    sample_rate: u32,
    frequency: f32,
    index_delta: f32,
    index: f32,
}

impl Default for WaveTableSampler {
    fn default() -> Self {
        let mut item = Self {
            sample_rate: 44_100,
            frequency: 220.0,
            index_delta: 0.0,
            index: 0.0,
        };

        item.update_index_delta();
        item
    }
}

impl WaveTableSampler {
    /// Creates a new wave table with the given resolution `R`, frequency and
    /// sample rate.
    pub fn new(frequency: f32, sample_rate: u32) -> Self {
        let mut item = Self {
            sample_rate,
            frequency,
            index: 0.0,
            index_delta: 0.0,
        };

        item.update_index_delta();

        item
    }

    /// Updates the internal ring buffer increment speed based on the frequency
    /// and sample rate. This must be multiplied by the table size when applied
    fn update_index_delta(&mut self) {
        self.index_delta = self.frequency as f32 / self.sample_rate as f32;
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
    pub fn sample<const R: usize>(&mut self, table: &WaveTable<R>) -> f32 {
        let current_index = self.index as usize;
        let next_index = (current_index + 1) % R;

        let next_weight = self.index - current_index as f32;
        let current_weight = 1.0 - next_weight;

        let sample =
            current_weight * table.samples[current_index] + next_weight * table.samples[next_index];

        let table_size = R as f32;
        self.index = (self.index + self.index_delta * table_size) % table_size;

        sample
    }
}
