use firewheel::{
    SilenceMask,
    diff::{Diff, Patch},
    event::NodeEventList,
    node::{AudioNodeProcessor, ProcBuffers, ProcInfo, ProcessStatus},
};
use wavetable::{WaveTableGenerator, WaveTableSampler, WaveType};

/// A processer with `N` samplers
#[derive(Clone, Copy, PartialEq, Debug, Diff, Patch)]
pub struct WaveTableProcessor<const N: usize> {
    sine_wave: [f32; 64],
    triangle_wave: [f32; 64],
    saw_wave: [f32; 64],
    square_wave: [f32; 64],
    base_frequency: f32,
    samplers: [WaveTableSampler; N],
    enabled: bool,
}

impl<const N: usize> WaveTableProcessor<N> {
    pub fn new(enabled: bool, base_frequency: f32, samplers: [WaveTableSampler; N]) -> Self {
        Self {
            enabled,
            sine_wave: WaveTableGenerator::sin::<64>(),
            triangle_wave: WaveTableGenerator::triangle::<64>(),
            saw_wave: WaveTableGenerator::saw::<64>(),
            square_wave: WaveTableGenerator::square::<64>(),
            base_frequency,
            samplers,
        }
    }
}

impl<const N: usize> AudioNodeProcessor for WaveTableProcessor<N> {
    fn process(
        &mut self,
        buffers: ProcBuffers,
        _proc_info: &ProcInfo,
        _events: &mut NodeEventList,
        _logger: &mut firewheel::log::RealtimeLogger,
    ) -> ProcessStatus {
        // for patch in events.drain_patches::<WaveTableNode>() {
        //     match patch {
        //         WaveTableNodePatch::BaseFrequency(f) => self.base_frequency = f,
        //     };
        // }

        for s in buffers.outputs[0].iter_mut() {
            let mut val = 0.0;
            for sampler in self.samplers.iter_mut() {
                let wave_table = match sampler.wave_type {
                    WaveType::Sine => &self.sine_wave,
                    WaveType::Square => &self.square_wave,
                    WaveType::Triangle => &self.triangle_wave,
                    WaveType::Saw => &self.saw_wave,
                };
                val += sampler.sample(wave_table);
            }

            *s = val / N as f32;
        }

        ProcessStatus::OutputsModified {
            out_silence_mask: SilenceMask::NONE_SILENT,
        }
    }
}
