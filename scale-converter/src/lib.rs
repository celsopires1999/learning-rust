#[derive(Debug, PartialEq)]
enum Scale {
    Celsius,
    Fahrenheit,
}

struct Temperature {
    degrees: f32,
    scale: Scale,
}

impl Temperature {
    fn new(scale: Scale, degrees: f32) -> Temperature {
        Temperature { degrees, scale }
    }

    fn to_celsius(&self) -> f32 {
        match self.scale {
            Scale::Celsius => self.degrees,
            Scale::Fahrenheit => (self.degrees - 32.0) * (5.0 / 9.0),
        }
    }

    fn to_fahrenheit(&self) -> f32 {
        match self.scale {
            Scale::Celsius => self.degrees * (9.0 / 5.0) + 32.0,
            Scale::Fahrenheit => self.degrees,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_create_celsius() {
        let celsius = Temperature::new(Scale::Celsius, 25.0);
        assert_eq!(celsius.degrees, 25.0);
        assert_eq!(celsius.scale, Scale::Celsius);
    }

    #[test]
    fn it_should_create_fahrenheit() {
        let fahrenheit = Temperature::new(Scale::Fahrenheit, 48.0);
        assert_eq!(fahrenheit.degrees, 48.0);
        assert_eq!(fahrenheit.scale, Scale::Fahrenheit);
    }

    #[test]
    fn it_should_convert_from_fahrenheit_to_celsius() {
        let fahrenheit = Temperature::new(Scale::Fahrenheit, 77.0);
        assert_eq!(fahrenheit.to_celsius(), 25.000002);
    }

    #[test]
    fn it_should_convert_from_celsius_to_fahrenheit() {
        let celsius = Temperature::new(Scale::Celsius, 25.0);
        assert_eq!(celsius.to_fahrenheit(), 77.0);
    }
}
