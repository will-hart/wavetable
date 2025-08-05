use firewheel::{
    SilenceMask,
    diff::{Diff, Patch},
    event::NodeEventList,
    node::{AudioNodeProcessor, ProcBuffers, ProcInfo, ProcessStatus},
};

use crate::{
    node::{SineNode, SineNodePatch},
    wavetable::{WaveTable, WaveTableSampler},
};

/// A processer with `N` samplers
#[derive(Clone, Copy, PartialEq, Debug, Diff, Patch)]
pub struct WaveTableProcessor<const N: usize> {
    wave_table: WaveTable<64>,
    samplers: [WaveTableSampler; N],
}

impl<const N: usize> WaveTableProcessor<N> {
    pub fn new(samplers: [WaveTableSampler; N]) -> Self {
        Self {
            wave_table: WaveTable::<64>::default(),
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
        // for patch in events.drain_patches::<SineNode>() {
        //     match patch {
        //         SineNodePatch::Frequency(freq) => {
        //             self.set_frequency(freq);
        //         }
        //         SineNodePatch::SampleRate(r) => self.set_sample_rate(r),
        //         SineNodePatch::Enabled(_e) => {
        //             panic!("not supported");
        //         }
        //     };
        // }

        for s in buffers.outputs[0].iter_mut() {
            *s = self
                .samplers
                .iter_mut()
                .map(|s| s.sample(&self.wave_table))
                .sum::<f32>()
                / N as f32;
        }

        ProcessStatus::OutputsModified {
            out_silence_mask: SilenceMask::NONE_SILENT,
        }
    }
}
