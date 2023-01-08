use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct Isbn {
    pub raw: String,
    pub digits: Vec<u8>,
}

#[derive(Debug, PartialEq)]
pub enum InvalidIsbn {
    TooLong,
    TooShort,
    FailedChecksum,
    InvalidCharacter(usize, char),
}
impl FromStr for Isbn {
    type Err = InvalidIsbn;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut digits = vec![];
        for (i, c) in s.char_indices() {
            match c {
                '-' => continue,
                '0'..='9' => digits.push(c.to_digit(10).unwrap() as u8),
                _ => {
                    return Err(InvalidIsbn::InvalidCharacter(i, c));
                }
            }
        }

        let n_digits = digits.len();
        if n_digits > 13 {
            return Err(InvalidIsbn::TooLong);
        } else if n_digits < 13 {
            return Err(InvalidIsbn::TooShort);
        }

        if digits[12] != calculate_check_digit(&digits) {
            return Err(InvalidIsbn::FailedChecksum);
        }

        Ok(Isbn {
            raw: s.to_string(),
            digits,
        })
    }
}

impl Display for Isbn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}-{}-{}{}-{}{}{}{}{}{}-{}",
            self.digits[0],
            self.digits[1],
            self.digits[2],
            self.digits[3],
            self.digits[4],
            self.digits[5],
            self.digits[6],
            self.digits[7],
            self.digits[8],
            self.digits[9],
            self.digits[10],
            self.digits[11],
            self.digits[12]
        )
    }
}

fn calculate_check_digit(digits: &[u8]) -> u8 {
    const WEIGHTS: [u8; 12] = [1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3];

    /************* Essa foi a minha versão, sem iterator *****
      let mut weight_applied = 0;

      for i in 0..digits.len() {
          weight_applied += digits[i] * WEIGHTS[i];
      }
    */

    let weight_applied: u32 = digits
        .iter()
        .zip(WEIGHTS.iter())
        .map(|(x, y)| x * y)
        .map(|value| value as u32)
        .sum();

    let check_digit = 10 - (weight_applied % 10);

    if check_digit == 10 {
        0_u8
    } else {
        check_digit as u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_check_digit() {
        let digits: [u8; 12] = [9, 7, 8, 3, 1, 6, 1, 4, 8, 4, 1, 0];
        let result = calculate_check_digit(&digits);
        assert_eq!(result, 0u8);
    }

    #[test]
    fn test_from_str_ok() {
        let isbn = Isbn::from_str("978-3-16-148410-0");
        let expected_isbn = Isbn {
            digits: vec![9, 7, 8, 3, 1, 6, 1, 4, 8, 4, 1, 0, 0],
            raw: "978-3-16-148410-0".to_string(),
        };
        assert_eq!(isbn.unwrap(), expected_isbn);
    }

    #[test]
    fn it_should_throw_invalid_character() {
        let e = Isbn::from_str("97A-3-16-148410-0").unwrap_err();
        assert_eq!(e, InvalidIsbn::InvalidCharacter(2, 'A'));
    }

    #[test]
    fn it_should_throw_too_long() {
        let e = Isbn::from_str("978-3-16-148410-0-9").unwrap_err();
        assert_eq!(e, InvalidIsbn::TooLong);
    }

    #[test]
    fn it_should_throw_too_short() {
        let e = Isbn::from_str("978-3-16-148410").unwrap_err();
        assert_eq!(e, InvalidIsbn::TooShort);
    }

    #[test]
    fn test_display_isbn() {
        let isbn = Isbn::from_str("978-3-16-148410-0").unwrap();
        assert_eq!(
            format!("The ISBN is: {isbn}"),
            "The ISBN is: 978-3-16-148410-0"
        );
    }
}
