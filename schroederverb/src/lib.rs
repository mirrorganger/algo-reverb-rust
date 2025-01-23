use nih_plug::prelude::*;
use std::sync::Arc;

pub struct Schroeder {
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

impl Default for Schroeder {
    fn default() -> Self {
        Self {
            params: Arc::new(SchroederParams::default()),
        }
    }
}

impl Default for SchroederParams{
    fn deault() -> Self {
        Self {
            rt60 : FloatParam::new(
                "rt60",
                2.0,
                FloatRange::Linear{1.0,20.0}                
                   ),
            dampening : FloatParam::new(
                "dampening",
                0.5,
                FloatRange::Linear{0.0,1.0}                
                   ),
            dry_wet_mix : FloatParam::new(
                "dryWetMix",
                0.5,
                FloatRange::Linear{0.0,1.0}                
                   ),
        }
    }
}

impl Plugin for Schoreder {
    const NAME: &'static str = "Schroeder";
    const VENDOR: &'static str = "MirrorAudio";
    const URL : &'static str = "mirroraudio.com";
    const EMAIL : &'static str = "mirroraudio@gmail.com";
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");
}
