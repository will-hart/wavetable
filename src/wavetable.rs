use firewheel::diff::{Diff, Patch};

pub struct WaveTableGenerator;

impl WaveTableGenerator {
    pub fn sin<const R: usize>() -> [f32; R] {
        let mut samples = [0.0; R];
        let lf = R as f32;
        for i in 0..R {
            samples[i] = (2.0 * std::f32::consts::PI * i as f32 / lf).sin();
        }

        samples
    }

    pub fn triangle<const R: usize>() -> [f32; R] {
        let halfway_idx = R / 2;
        let mut samples = [0.0; R];

        let gradient = 2.0 / halfway_idx as f32;

        // make the first half of the graph which goes from -1 to 1 at halfway_idx
        for x in 0..halfway_idx {
            samples[x] = gradient * x as f32 - 1.0;
        }

        // now mirror to the end
        for x in halfway_idx..R {
            samples[x] = samples[x - halfway_idx];
        }

        samples
    }

    pub fn square<const R: usize>() -> [f32; R] {
        let halfway_idx = R / 2;
        let mut samples = [1.0; R];
        for x in 0..halfway_idx {
            samples[x] = 0.0;
        }

        samples
    }

    pub fn saw<const R: usize>() -> [f32; R] {
        let mut samples = [0.0; R];
        let gradient = 2.0 / R as f32;

        for x in 0..R {
            samples[x] = gradient * x as f32 - 1.0;
        }

        samples
    }
}

#[derive(Clone, Copy, PartialEq, Debug, Diff, Patch)]
pub enum WaveType {
    Sine,
    Square,
    Triangle,
    Saw,
}

#[derive(Clone, Copy, PartialEq, Debug, Diff, Patch)]
/// Samples a wave table at a given frequency and sample rate
pub struct WaveTableSampler {
    pub sample_rate: u32,
    pub base_frequency: f32,
    pub frequency_multiplier: f32,
    pub index: f32,
    pub wave_type: WaveType,
}

impl Default for WaveTableSampler {
    fn default() -> Self {
        Self {
            sample_rate: 44_100,
            base_frequency: 220.0,
            frequency_multiplier: 1.0,
            index: 0.0,
            wave_type: WaveType::Sine,
        }
    }
}

impl WaveTableSampler {
    /// Gets a sample and increments the internal buffer to the next sample
    pub fn sample<const R: usize>(&mut self, table: &[f32; R]) -> f32 {
        let current_index = self.index as usize;
        let next_index = (current_index + 1) % R;

        let next_weight = self.index - current_index as f32;
        let current_weight = 1.0 - next_weight;

        let sample = current_weight * table[current_index] + next_weight * table[next_index];

        let table_size = R as f32;
        self.index = (self.index
            + table_size * self.base_frequency * self.frequency_multiplier
                / self.sample_rate as f32)
            % table_size;

        sample
    }
}
