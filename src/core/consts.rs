/// The number of simultaneous voices for this synth.
pub const NUM_VOICES: u32 = 16;
/// The maximum size of an audio block. We'll split up the audio in blocks and render smoothed
/// values to buffers since these values may need to be reused for multiple voices.
pub const MAX_BLOCK_SIZE: usize = 64;

// Polyphonic modulation works by assigning integer IDs to parameters. Pattern matching on these in
// `PolyModulation` and `MonoAutomation` events makes it possible to easily link these events to the
// correct parameter.
pub const GAIN_POLY_MOD_ID: u32 = 0;
