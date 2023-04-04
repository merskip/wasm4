use crate::system;

pub struct Audio;

impl Audio {
    pub fn shared() -> Self {
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

/// Duration of the tone
pub struct Duration(u8);

impl Duration {
    /// * `value` - Duration of the tone in frames, one frame is 1/60 of a second
    pub const fn from_frames(value: u8) -> Self {
        Self(value)
    }

    /// * `value` - Duration of the tone in milliseconds
    pub const fn from_millis(value: u8) -> Self {
        Self(value / 16)
    }
}

/// Duration of the tone with describe changes over time
pub struct ADSRDuration(u32);

impl ADSRDuration {
    /// * `duration` - Constant duration of the tone
    pub const fn constant(duration: Duration) -> Self {
        Self(duration.0 as u32)
    }

    /// * `attack` - The time it takes to initially ramp up from 0 volume to peak volume.
    /// * `decay` - The time taken to ramp down from peak volume to the sustain volume.
    /// * `sustain` - The time to hold the tone steady at the sustain volume.
    /// * `release` - The time to ramp back down to 0 volume.
    pub const fn new(attack: Duration,
                     decay: Duration,
                     sustain: Duration,
                     release: Duration) -> Self {
        Self((sustain.0 as u32) |
            ((release.0 as u32) << 8) |
            ((decay.0 as u32) << 16) |
            ((attack.0 as u32) << 24))
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

pub struct Flags(u32);

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Channel {
    Pulse1 = 0b0000,
    Pulse2 = 0b0001,
    Triangle = 0b0010,
    Noise = 0b0011,
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

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Pan {
    Center = 0b00,
    Left = 0b01,
    Right = 0b10,
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
