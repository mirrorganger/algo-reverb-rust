use nih_plug::prelude::*;
use std::sync::Arc;

pub struct SchroederPlugin {
    params: Arc<SchroederParams>,
}

#[derive(Params)]
struct SchroederParams {
    #[id = "rt60"]
    pub rt60: FloatParam,

    #[id = "dampening"]
    pub dampening: FloatParam,

    #[id = "dryWetMix"]
    pub dry_wet_mix: FloatParam,
}

impl Default for SchroederPlugin {
    fn default() -> Self {
        Self {
            params: Arc::new(SchroederParams::default()),
        }
    }
}

impl Default for SchroederParams{
    fn default() -> Self {
        Self {
            rt60 : FloatParam::new(
                "rt60",
                2.0,
                FloatRange::Linear{min : 1.0, max : 20.0})
                .with_smoother(SmoothingStyle::Linear(3.0))
                .with_unit(" s"),
            dampening : FloatParam::new(
                "dampening",
                0.5,
                FloatRange::Linear{min : 0.0, max : 1.0}                
                   ).with_smoother(SmoothingStyle::Linear(3.0)),
            dry_wet_mix : FloatParam::new(
                "dryWetMix",
                0.5,
                FloatRange::Linear{min : 0.0, max :1.0}                
                   ).with_smoother(SmoothingStyle::Linear(3.0)),
        }
    }
}

impl Plugin for SchroederPlugin {

    const NAME: &'static str = "Schroeder";
    const VENDOR: &'static str = "MirrorAudio";
    const URL : &'static str = "mirroraudio.com";
    const EMAIL : &'static str = "mirroraudio@gmail.com";
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
        main_input_channels: NonZeroU32::new(2),
        main_output_channels: NonZeroU32::new(2),
        ..AudioIOLayout::const_default()
    }];

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn process(
        &mut self,
        _buffer: &mut Buffer,
        _aux : &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        // Processing code goes here
        ProcessStatus::Normal
    }

}

impl Vst3Plugin for SchroederPlugin {
    const VST3_CLASS_ID: [u8; 16] = *b"SchroederPlugin\n";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[
        Vst3SubCategory::Reverb,
        Vst3SubCategory::Fx,
        Vst3SubCategory::Stereo
    ];
}

nih_export_vst3!(SchroederPlugin);
