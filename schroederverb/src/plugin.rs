use nih_plug::prelude::*;
use nih_plug_egui::{create_egui_editor, egui, widgets,EguiState};
use nih_plug_egui::egui::Color32;
use std::sync::{Arc, Mutex};
use euterpe_rs::processor::AudioProcessor;
use crate::schroeder::Schroeder;

use crate::ui_knob::{self, KnobLayout};


pub struct SchroederPlugin {
    params: Arc<SchroederParams>,
    processor: Arc<Mutex<Schroeder>>,
    sample_rate: f32,
}

const DEFAULT_SAMPLE_RATE : f32 = 44100.0;
pub const TEAL_GREEN: Color32 = Color32::from_rgb(61, 178, 166);
pub const DARKEST_BOTTOM_UI_COLOR: Color32 = Color32::from_rgb(27, 27, 27);
pub const DARKER_GREY_UI_COLOR: Color32 = Color32::from_rgb(34, 34, 34);
pub const DARK_GREY_UI_COLOR: Color32 = Color32::from_rgb(42, 42, 42);
pub const MEDIUM_GREY_UI_COLOR: Color32 = Color32::from_rgb(52, 52, 52);
pub const LIGHTER_GREY_UI_COLOR: Color32 = Color32::from_rgb(69, 69, 69);
pub const A_BACKGROUND_COLOR_TOP: Color32 = Color32::from_rgb(38, 38, 38);
pub const YELLOW_MUSTARD: Color32 = Color32::from_rgb(172, 131, 25);
pub const FONT_COLOR: Color32 = Color32::from_rgb(248, 248, 248);
const TEXT_SIZE: f32 = 10.0;


#[derive(Params)]
struct SchroederParams {
    #[persist = "editor-state"]
    editor_state: Arc<EguiState>,

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
            processor: Arc::new(Mutex::new(Schroeder::new(DEFAULT_SAMPLE_RATE as f64))),
            sample_rate: DEFAULT_SAMPLE_RATE as f32,
        }
    }
}

impl Default for SchroederParams{
    fn default() -> Self {
        Self {
            editor_state: EguiState::from_size(300,300),
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

    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        let params = self.params.clone();
        // let processor = self.processor.clone();
        create_egui_editor(
            self.params.editor_state.clone(),
            (),
            |_, _| {},
            move |_egui_ctx, setter, _state| {
                egui::TopBottomPanel::top("top").show(_egui_ctx, |ui| {
                    ui.heading("Schroeder Reverb");
                });
                egui::CentralPanel::default().show(_egui_ctx, |ui| {
                    const KNOB_SIZE: f32 = 28.0;
                    
                    ui.horizontal(|ui|{
                    
                    let dampening_knob = ui_knob::ArcKnob::for_param(
                        &params.dampening,
                        setter,
                        KNOB_SIZE,
                        KnobLayout::Vertical)
                        .preset_style(ui_knob::KnobStyle::Preset1)
                        .set_fill_color(DARK_GREY_UI_COLOR)
                        .set_line_color(TEAL_GREEN)
                        .set_text_size(TEXT_SIZE).set_hover_text("Dampening of the reverb".to_string())
                        .use_outline(true);
                    let rt60_knob = ui_knob::ArcKnob::for_param(
                        &params.rt60,
                        setter,
                        KNOB_SIZE,
                        KnobLayout::Vertical)
                        .preset_style(ui_knob::KnobStyle::Preset1)
                        .set_fill_color(DARK_GREY_UI_COLOR)
                        .set_line_color(TEAL_GREEN)
                        .set_text_size(TEXT_SIZE).set_hover_text("Reverb Time".to_string())
                        .use_outline(true);
                    let lfo_freq_knob = ui_knob::ArcKnob::for_param(
                        &params.mod_freq,
                        setter,
                        KNOB_SIZE,
                        KnobLayout::Vertical)
                        .preset_style(ui_knob::KnobStyle::Preset1)
                        .set_fill_color(DARK_GREY_UI_COLOR)
                        .set_line_color(TEAL_GREEN)
                        .set_text_size(TEXT_SIZE).set_hover_text("LFO Modulation Frequency".to_string())
                        .use_outline(true);
                    let dry_wet_knob = ui_knob::ArcKnob::for_param(
                        &params.dry_wet_mix,
                        setter,
                        KNOB_SIZE,
                        KnobLayout::Vertical)
                        .preset_style(ui_knob::KnobStyle::Preset1)
                        .set_fill_color(DARK_GREY_UI_COLOR)
                        .set_line_color(TEAL_GREEN)
                        .set_text_size(TEXT_SIZE).set_hover_text("Dry / Wet Mix".to_string())
                        .use_outline(true);


                    ui.add(dampening_knob);    
                    ui.add(rt60_knob);
                    ui.add(lfo_freq_knob);
                    ui.add(dry_wet_knob);
                    });
                });
            }
        )
    }

    fn initialize(
            &mut self,
            _audio_io_layout: &AudioIOLayout,
            buffer_config: &BufferConfig,
            _context: &mut impl InitContext<Self>,
        ) -> bool {
        let mut processor = self.processor.lock().unwrap();
        self.sample_rate = buffer_config.sample_rate;
        processor.prepare(self.sample_rate as f64, (self.params.rt60.default_plain_value() * 1000.0) as f64);
        processor.set_dampening(0.5);
        processor.set_dry_wet_mix(0.5);
        processor.set_mod_enabled(false);
        processor.set_mod_lfo_freq(0.5);
        true
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux : &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {

        let mut processor = self.processor.lock().unwrap();
        processor.update_reverb_time((self.params.rt60.smoothed.next() * 1000.0) as f64); 
        processor.set_dampening(self.params.dampening.smoothed.next() as f64);
        processor.set_dry_wet_mix(self.params.dry_wet_mix.smoothed.next() as f64);        
        processor.set_mod_enabled(self.params.mod_enabled.value());
        processor.set_mod_lfo_freq(self.params.mod_freq.smoothed.next() as f32);

        for mut channel_samples in buffer.iter_samples() {
            
            let mut mono_sample : f32 = 0.0;
            let num_channels = channel_samples.len();

            for sample in channel_samples.iter_mut(){
                mono_sample += *sample;
            }

            mono_sample /= num_channels as f32;

            let output = processor.process(mono_sample as f64) as f32;

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
