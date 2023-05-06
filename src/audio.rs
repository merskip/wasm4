use crate::system;

pub struct Audio;

impl Audio {
    pub const fn shared() -> Self {
        Self
    }

    pub fn tone(&self,
                frequency: Frequency,
                duration: ADSRDuration,
                volume: Volume,
                flags: Flags) {
        unsafe {
            system::tone(frequency.into(),
                         duration.into(),
                         volume.into(),
                         flags.into());
        }
    }
}

/// Wave frequency
pub struct Frequency(u32);

impl Frequency {
    /// * `hertz` - Wave frequency in hertz
    pub const fn constant(hertz: u16) -> Self {
        Self(hertz as u32)
    }

    /// * `start_hertz` - Start wave frequency in hertz
    /// * `end_hertz` - End wave frequency in hertz
    pub const fn linear(start: u16, end: u16) -> Self {
        Self((start as u32) | (end as u32) << 16)
    }
}

impl Into<u32> for Frequency {
    fn into(self) -> u32 {
        self.0
    }
}

/// Duration of the tone in frames
pub type ToneDuration = u8;

/// Duration of the tone with describe changes over time
pub struct ADSRDuration(u32);

impl ADSRDuration {
    /// * `duration` - Constant duration of the tone
    pub const fn constant(duration: ToneDuration) -> Self {
        Self(duration as u32)
    }

    /// * `attack` - The time it takes to initially ramp up from 0 volume to peak volume.
    /// * `decay` - The time taken to ramp down from peak volume to the sustain volume.
    /// * `sustain` - The time to hold the tone steady at the sustain volume.
    /// * `release` - The time to ramp back down to 0 volume.
    pub const fn new(attack: ToneDuration,
                     decay: ToneDuration,
                     sustain: ToneDuration,
                     release: ToneDuration) -> Self {
        Self((sustain as u32) |
            ((release as u32) << 8) |
            ((decay as u32) << 16) |
            ((attack as u32) << 24))
    }
}

impl Into<u32> for ADSRDuration {
    fn into(self) -> u32 {
        self.0
    }
}

/// Volume of sound
pub struct Volume(u32);

impl Volume {
    /// `value` - Volume between 0 and 100
    pub const fn constant(volume: u8) -> Self {
        assert!(volume <= 100, "volume must be between 0 and 100");
        Self(volume as u32)
    }

    /// `peak` - The peak volume reached by the attack duration
    /// `sustain` - The main volume used for the sustain duration
    pub const fn new(peak: u8, sustain: u8) -> Self {
        assert!(peak <= 100, "peak must be between 0 and 100");
        assert!(sustain <= 100, "sustain must be between 0 and 100");
        Self((sustain as u32) | ((peak as u32) << 8))
    }
}

impl Into<u32> for Volume {
    fn into(self) -> u32 {
        self.0
    }
}

impl Default for Volume {
    fn default() -> Self {
        Self::constant(100)
    }
}

pub struct Flags(u32);

/// WASM-4's sound system has 4 independent channels.
/// Each channel is dedicated to a different type of audio waveform.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Channel {
    /// Classic chiptune sound, square wave
    Pulse1 = 0b00,
    /// Classic chiptune sound, square wave
    Pulse2 = 0b01,
    /// A softer sound, great for bass
    Triangle = 0b10,
    /// A harsh sound, for percussion and effects
    Noise = 0b11,
}

/// For pulse channels, the pulse wave duty cycle.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum DutyCycle {
    /// 1/8, 12.5%
    OneEighth = 0b00,
    /// 1/4, 25%
    OneQuarter = 0b01,
    /// 1/2, 50%
    OneHalf = 0b10,
    /// 3/4, 75%
    ThreeQuarters = 0b11,
}

/// The pan means the direction of the sound,
/// it could be from left speaker or from right speaker or both
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Pan {
    Center = 0b00,
    Left = 0b01,
    Right = 0b10,
}

impl Default for Pan {
    fn default() -> Self {
        Self::Center
    }
}

impl Flags {
    pub const fn new(channel: Channel, mode: DutyCycle, pan: Pan) -> Self {
        Self((channel as u32) |
            ((mode as u32) << 2) |
            ((pan as u32) << 4))
    }
}

impl Into<u32> for Flags {
    fn into(self) -> u32 {
        self.0
    }
}
