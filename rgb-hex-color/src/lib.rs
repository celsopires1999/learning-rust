use std::{fmt::Display, str::FromStr};

#[derive(Debug, PartialEq)]
struct Rgb {
    r: u32,
    g: u32,
    b: u32,
}

trait RgbChannels {
    fn r(&self) -> u8;
    fn g(&self) -> u8;
    fn b(&self) -> u8;
}

impl RgbChannels for Rgb {
    fn r(&self) -> u8 {
        self.r as u8
    }

    fn g(&self) -> u8 {
        self.g as u8
    }

    fn b(&self) -> u8 {
        self.b as u8
    }
}

impl FromStr for Rgb {
    type Err = ParseColorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(hex) = s.strip_prefix("#") {
            let r = u32::from_str_radix(&hex[0..=1], 16);
            let g = u32::from_str_radix(&hex[2..=3], 16);
            let b = u32::from_str_radix(&hex[4..=5], 16);

            Ok(Rgb {
                r: r.or_else(|_| Err(ParseColorError::RedOutOfBounds))?,
                g: g.or_else(|_| Err(ParseColorError::GreenOutOfBounds))?,
                b: b.or_else(|_| Err(ParseColorError::BlueOutOfBounds))?,
            })
        } else {
            Err(ParseColorError::Invalid)
        }
    }
}

#[derive(Debug, PartialEq)]
enum ParseColorError {
    RedOutOfBounds,
    GreenOutOfBounds,
    BlueOutOfBounds,
    Invalid,
}

impl Display for Rgb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_create_rgb() {
        let rgb = Rgb::from_str("#AABBCC").unwrap();
        assert_eq!(rgb.r, 170);
        assert_eq!(rgb.g, 187);
        assert_eq!(rgb.b, 204);
        assert_eq!(format!("{}", rgb), "#aabbcc")
    }

    #[test]
    fn it_should_throw_red_out_bounds() {
        let e = Rgb::from_str("#AZBBCC").unwrap_err();
        assert_eq!(e, ParseColorError::RedOutOfBounds);
    }

    #[test]
    fn it_should_throw_green_out_bounds() {
        let e = Rgb::from_str("#AABZCC").unwrap_err();
        assert_eq!(e, ParseColorError::GreenOutOfBounds);
    }

    #[test]
    fn it_should_throw_blue_out_bounds() {
        let e = Rgb::from_str("#AABBXC").unwrap_err();
        assert_eq!(e, ParseColorError::BlueOutOfBounds);
    }

    #[test]
    fn it_should_throw_invalid() {
        let e = Rgb::from_str("AABBCC").unwrap_err();
        assert_eq!(e, ParseColorError::Invalid);
    }
}
