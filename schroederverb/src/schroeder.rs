use euterpe_rs::all_pass::AllPass;
use euterpe_rs::comb::Comb;
use euterpe_rs::processor::AudioProcessor;
use euterpe_rs::mod_all_pass::ModAllPass;
use euterpe_rs::lfo::WaveformType;

const NUM_COMBS: usize = 4;
const NUM_APF: usize = 2;
const NUM_PRE_APF: usize = 2;
const COMB_DELAYS_MS: [f64; NUM_COMBS] = [29.7, 32.2, 38.1, 45.6];
const APF_DELAYS_MS: [f64; NUM_APF] = [2.3, 3.7];
const PRE_APF_DELAYS_MS : [f64; NUM_PRE_APF] = [1.0, 2.0];
const COMB_MAX_DELAY_MS: f64 = 50.0;
const APF_MAX_DELAY_MS: f64 = 20.0;
const MOD_DELAY_DELAY_MS : f32 = 30.0;
const MOD_DELAY_LFO_FREQ_HZ : f32 = 10.0;


fn get_length_in_samples(length_ms: f64, sample_rate_hz: f64) -> f64 {
    sample_rate_hz * length_ms / 1000.0
}

fn get_gain_from_rt60(delay_ms: f64, rt60_ms: f64) -> f64 {
    let base: f64 = 10.0;
    base.powf(-3.0 * delay_ms / rt60_ms)
}

pub struct Schroeder {
    combs: [(Comb, f64); NUM_COMBS],
    all_passes: [(AllPass, f64); NUM_APF],
    pre_all_passes: [(AllPass, f64); NUM_PRE_APF],
    mod_all_pass : ModAllPass,
    mod_enabled : bool,
    dry_wet_mix: f64,
}

impl Schroeder {
    pub fn new(sample_rate: f64) -> Self {
        let comb_delay_length = get_length_in_samples(COMB_MAX_DELAY_MS, sample_rate) as usize;
        let apf_delay_length = get_length_in_samples(APF_MAX_DELAY_MS, sample_rate) as usize;

        Schroeder {
            combs: [
                (Comb::new(comb_delay_length, true), COMB_DELAYS_MS[0]),
                (Comb::new(comb_delay_length, true), COMB_DELAYS_MS[1]),
                (Comb::new(comb_delay_length, true), COMB_DELAYS_MS[2]),
                (Comb::new(comb_delay_length, true), COMB_DELAYS_MS[3]),
            ],
            all_passes: [
                (AllPass::new(apf_delay_length), APF_DELAYS_MS[0]),
                (AllPass::new(apf_delay_length), APF_DELAYS_MS[1]),
            ],
            pre_all_passes: [
                (AllPass::new(apf_delay_length), PRE_APF_DELAYS_MS[0]),
                (AllPass::new(apf_delay_length), PRE_APF_DELAYS_MS[1]),
            ],
            mod_all_pass: ModAllPass::new(MOD_DELAY_DELAY_MS,MOD_DELAY_LFO_FREQ_HZ,WaveformType::Sawtooth,sample_rate as f32),
            mod_enabled: false,
            dry_wet_mix: 0.5,
        }
    }

    pub fn prepare(&mut self, sample_rate: f64, rt60_ms: f64) {
        for (comb, delay_ms) in self.combs.iter_mut() {
            let delay_samples = get_length_in_samples(*delay_ms, sample_rate);
            let gain = get_gain_from_rt60(*delay_ms, rt60_ms);
            comb.prepare(delay_samples, gain);
        }

        self.pre_all_passes.iter_mut().for_each(|(apf, delay)| {
            (*apf).prepare(get_length_in_samples(*delay, sample_rate),0.5 * (2f64).sqrt());
        });

        for (all_pass, delay_ms) in self.all_passes.iter_mut() {
            let delay_samples = get_length_in_samples(*delay_ms, sample_rate);
            all_pass.prepare(delay_samples, 0.5 * (2f64).sqrt());
        }
    }

    pub fn set_dampening(&mut self, dampening: f64) {
        for (comb, _) in self.combs.iter_mut() {
            comb.set_dampening(dampening);
        }
    }

    pub fn set_dry_wet_mix(&mut self, dry_wet_mix: f64) {
        self.dry_wet_mix = dry_wet_mix;
    }

    pub fn set_mod_enabled(&mut self, enabled: bool) {
        self.mod_enabled = enabled;
    }

    pub fn set_mod_lfo_freq(&mut self, freq: f32) {
        self.mod_all_pass.set_lfo_freq(freq);
    }

    pub fn update_reverb_time(&mut self, rt60_ms: f64) {
        for (comb, delay_ms) in self.combs.iter_mut() {
            comb.set_gain(get_gain_from_rt60(*delay_ms, rt60_ms))
        }
    }
}

impl AudioProcessor<f64> for Schroeder {
    fn process(&mut self, input: f64) -> f64 {
        let mut out: f64 = 0.0;
        let mut pre_apf_out: f64 = 0.0;
        
        for (all_pass, _) in self.pre_all_passes.iter_mut() {
            pre_apf_out = all_pass.process(input);
        }

        for (index, (combs, _)) in self.combs.iter_mut().enumerate() {
            let mut comb_out = combs.process(pre_apf_out);
            if index % 2 == 0 {
                comb_out *= -1.0;
            };
            out += comb_out;
        }
        out /= NUM_COMBS as f64;

        for (all_pass, _) in self.all_passes.iter_mut() {
            out = all_pass.process(out);
        }

        if self.mod_enabled {
            out = self.mod_all_pass.process(out);
        }

        out * self.dry_wet_mix + input * (1.0 - self.dry_wet_mix)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_creation() {
        use super::*;
        let sample_rate = 44100.0;
        let rt60_ms = 20.0;
        let mut uut = Schroeder::new(sample_rate);
        uut.prepare(sample_rate, rt60_ms);
        uut.set_dampening(0.5);
        uut.set_dry_wet_mix(0.6);
        let _out = uut.process(0.3);
    }
}
