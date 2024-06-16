use nih_plug::params::{EnumParam, FloatParam, Params};
use nih_plug_vizia::ViziaState;
use rand_pcg::Pcg32;
use std::sync::Arc;

use crate::{model::osc::OscillatorType, NUM_VOICES};

use super::Voice;

/// A simple polyphonic synthesizer with support for CLAP's polyphonic modulation. See
/// `NoteEvent::PolyModulation` for another source of information on how to use this.
pub struct OneXOsc {
    pub params: Arc<OneXOscParams>,

    /// A pseudo-random number generator. This will always be reseeded with the same seed when the
    /// synth is reset. That way the output is deterministic when rendering multiple times.
    pub prng: Pcg32,
    /// The synth's voices. Inactive voices will be set to `None` values.
    pub voices: [Option<Voice>; NUM_VOICES as usize],
    /// The next internal voice ID, used only to figure out the oldest voice for voice stealing.
    /// This is incremented by one each time a voice is created.
    pub next_internal_voice_id: u64,
}

#[derive(Params)]
pub struct OneXOscParams {
    /// A voice's gain. This can be polyphonically modulated.
    #[id = "gain"]
    pub gain: FloatParam,
    /// The amplitude envelope attack time. This is the same for every voice.
    #[id = "amp_atk"]
    pub amp_attack_ms: FloatParam,
    /// The amplitude envelope release time. This is the same for every voice.
    #[id = "amp_rel"]
    pub amp_release_ms: FloatParam,

    /// The editor state, saved together with the parameter state so the custom scaling can be
    /// restored.
    #[persist = "editor-state"]
    pub editor_state: Arc<ViziaState>,

    #[id = "osc_type"]
    pub osc_type: EnumParam<OscillatorType>,
}
