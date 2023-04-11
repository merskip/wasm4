use core::fmt::{Debug, Formatter};

use libm::fabsf;

use crate::color::Color;

#[derive(Copy, Clone, PartialEq)]
pub struct HSLColor {
    pub hue: f32,
    pub saturation: f32,
    pub lightness: f32,
}

impl HSLColor {
    pub fn new(hue: f32, saturation: f32, lightness: f32) -> Self {
        assert!(hue >= 0.0 && hue <= 360.0, "hue must be between 0.0 and 360.0");
        assert!(saturation >= 0.0 && saturation <= 1.0, "saturation must be between 0.0 and 1.0");
        assert!(lightness >= 0.0 && lightness <= 1.0, "lightness must be between 0.0 and 1.0");
        Self { hue, saturation, lightness }
    }

    pub fn from(color: Color) -> Self {
        let red = color.red as f32 / 255.0;
        let green = color.green as f32 / 255.0;
        let blue = color.blue as f32 / 255.0;

        let c_max = red.max(green).max(blue);
        let c_min = red.min(green).min(blue);
        let delta = c_max - c_min;

        let hue = if delta == 0.0 {
            0.0
        } else if c_max == red {
            60.0 * (((green - blue) / delta) % 6.0)
        } else if c_max == green {
            60.0 * ((blue - red) / delta + 2.0)
        } else {
            60.0 * ((red - green) / delta + 4.0)
        };

        let lightness = (c_max + c_min) / 2.0;

        let saturation = if delta == 0.0 {
            0.0
        } else {
            delta / (1.0 - fabsf(2.0 * lightness - 1.0))
        };

        Self { hue, saturation, lightness }
    }

    pub fn to_color(&self) -> Color {
        let (hue, saturation, lightness) = (self.hue, self.saturation, self.lightness);
        let chroma = fabsf(1.0 - (2.0 * lightness - 1.0)) * saturation;
        let hue_prime = hue / 60.0;
        let x = chroma * (1.0 - fabsf((hue_prime % 2.0) - 1.0));
        let (red, greed, blue) = if hue_prime < 1.0 {
            (chroma, x, 0.0)
        } else if hue_prime < 2.0 {
            (x, chroma, 0.0)
        } else if hue_prime < 3.0 {
            (0.0, chroma, x)
        } else if hue_prime < 4.0 {
            (0.0, x, chroma)
        } else if hue_prime < 5.0 {
            (x, 0.0, chroma)
        } else {
            (chroma, 0.0, x)
        };
        let m = lightness - 0.5 * chroma;
        Color::new(
            ((red + m) * 255.0) as u8,
            ((greed + m) * 255.0) as u8,
            ((blue + m) * 255.0) as u8,
        )
    }
}

impl From<Color> for HSLColor {
    fn from(value: Color) -> Self {
        Self::from(value)
    }
}

impl Into<Color> for HSLColor {
    fn into(self) -> Color {
        self.to_color()
    }
}

impl Debug for HSLColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "HSL({}°, {}%, {}%)",
               self.hue as u32,
               (self.saturation * 100.0) as u32,
               (self.lightness * 100.0) as u32)
    }
}