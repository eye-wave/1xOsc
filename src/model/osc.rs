use nih_plug::prelude::Enum;

#[derive(Debug, Enum, Clone, Copy, PartialEq, Eq)]
pub enum OscillatorType {
    Sine,
    Triangle,
    Square,
    Sawtooth,
    Noise,
}

impl Default for OscillatorType {
    fn default() -> Self {
        Self::Sine
    }
}
