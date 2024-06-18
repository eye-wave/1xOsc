use nih_plug::prelude::Enum;
use nih_plug_vizia::vizia::binding::Data;

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

impl Data for OscillatorType {
    fn same(&self, other: &Self) -> bool {
        self.eq(other)
    }
}
