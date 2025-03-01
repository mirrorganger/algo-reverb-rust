use nih_plug::prelude::*;
use std::sync::Arc;
use euterpe_rs::processor::AudioProcessor;
use crate::schroeder::Schroeder;

pub struct SchroederPlugin {
    params: Arc<SchroederParams>,
    processor: Schroeder,
    sample_rate: f32,
}

const DEFAULT_SAMPLE_RATE : f32 = 44100.0;

#[derive(Params)]
struct SchroederParams {
    #[id = "rt60"]
    pub rt60: FloatParam,

    #[id = "dampening"]
    pub dampening: FloatParam,

    #[id = "dryWetMix"]
    pub dry_wet_mix: FloatParam,

    #[id = "modFreq"]
    pub mod_freq: FloatParam,

    #[id = "modEnabled"]
    pub mod_enabled: BoolParam,
}

impl Default for SchroederPlugin {
    fn default() -> Self {
        Self {
            params: Arc::new(SchroederParams::default()),
            processor: Schroeder::new(DEFAULT_SAMPLE_RATE as f64),
            sample_rate: DEFAULT_SAMPLE_RATE as f32,
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
            mod_freq : FloatParam::new(
                "modFreq",
                0.5,
                FloatRange::Linear{min : 0.0, max : 2.0}                
                   ).with_smoother(SmoothingStyle::Exponential(10.0))
                   .with_unit(" Hz"),
            mod_enabled : BoolParam::new("modEnabled", false),
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

    fn initialize(
            &mut self,
            _audio_io_layout: &AudioIOLayout,
            buffer_config: &BufferConfig,
            _context: &mut impl InitContext<Self>,
        ) -> bool {
        self.sample_rate = buffer_config.sample_rate;
        self.processor.prepare(self.sample_rate as f64, (self.params.rt60.default_plain_value() * 1000.0) as f64);
        self.processor.set_dampening(0.5);
        self.processor.set_dry_wet_mix(0.5);
        self.processor.set_mod_enabled(false);
        self.processor.set_mod_lfo_freq(0.5);
        true
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux : &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        
        self.processor.update_reverb_time((self.params.rt60.smoothed.next() * 1000.0) as f64); 
        self.processor.set_dampening(self.params.dampening.smoothed.next() as f64);
        self.processor.set_dry_wet_mix(self.params.dry_wet_mix.smoothed.next() as f64);        
        self.processor.set_mod_enabled(self.params.mod_enabled.value());
        self.processor.set_mod_lfo_freq(self.params.mod_freq.smoothed.next() as f32);

        for mut channel_samples in buffer.iter_samples() {
            
            let mut mono_sample : f32 = 0.0;
            let num_channels = channel_samples.len();

            for sample in channel_samples.iter_mut(){
                mono_sample += *sample;
            }

            mono_sample /= num_channels as f32;

            let output = self.processor.process(mono_sample as f64) as f32;

            for sample in channel_samples.iter_mut(){
                *sample = output;
            }
        } 

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
