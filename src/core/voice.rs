use nih_plug::prelude::Smoother;

/// Data for a single synth voice. In a real synth where performance matter, you may want to use a
/// struct of arrays instead of having a struct for each voice.
#[derive(Debug, Clone)]
pub struct Voice {
    /// The identifier for this voice. Polyphonic modulation events are linked to a voice based on
    /// these IDs. If the host doesn't provide these IDs, then this is computed through
    /// `compute_fallback_voice_id()`. In that case polyphonic modulation will not work, but the
    /// basic note events will still have an effect.
    pub voice_id: i32,
    /// The note's channel, in `0..16`. Only used for the voice terminated event.
    pub channel: u8,
    /// The note's key/note, in `0..128`. Only used for the voice terminated event.
    pub note: u8,
    /// The voices internal ID. Each voice has an internal voice ID one higher than the previous
    /// voice. This is used to steal the last voice in case all 16 voices are in use.
    pub internal_voice_id: u64,
    /// The square root of the note's velocity. This is used as a gain multiplier.
    pub velocity_sqrt: f32,

    /// The voice's current phase. This is randomized at the start of the voice
    pub phase: f32,
    /// The phase increment. This is based on the voice's frequency, derived from the note index.
    /// Since we don't support pitch expressions or pitch bend, this value stays constant for the
    /// duration of the voice.
    pub phase_delta: f32,
    /// Whether the key has been released and the voice is in its release stage. The voice will be
    /// terminated when the amplitude envelope hits 0 while the note is releasing.
    pub releasing: bool,
    /// Fades between 0 and 1 with timings based on the global attack and release settings.
    pub amp_envelope: Smoother<f32>,

    /// If this voice has polyphonic gain modulation applied, then this contains the normalized
    /// offset and a smoother.
    pub voice_gain: Option<(f32, Smoother<f32>)>,
}

/// Compute a voice ID in case the host doesn't provide them. Polyphonic modulation will not work in
/// this case, but playing notes will.
pub fn compute_fallback_voice_id(note: u8, channel: u8) -> i32 {
    note as i32 | ((channel as i32) << 16)
}
