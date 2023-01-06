use chrono::*;

pub fn weeks_between(a: &str, b: &str) -> i32 {
    let dt1 = a.parse::<DateTime<FixedOffset>>().unwrap();
    let dt2 = b.parse::<DateTime<FixedOffset>>().unwrap();

    let diff = dt2.signed_duration_since(dt1);
    diff.num_weeks() as i32
}

pub fn weeks_between_naive(a: &str, b: &str) -> i32 {
    let dt1 = NaiveDate::parse_from_str(a, "%Y-%m-%d").unwrap();
    let dt2 = NaiveDate::parse_from_str(b, "%Y-%m-%d").unwrap();

    let diff = dt2 - dt1;
    diff.num_weeks() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weeks_between() {
        let weeks = weeks_between("2023-01-06T19:56:00+03:00", "2023-01-13T19:56:00+03:00");
        assert_eq!(weeks, 1);
    }

    #[test]
    fn test_weeks_between_naive() {
        let weeks = weeks_between_naive("2023-01-06", "2023-01-13");
        assert_eq!(weeks, 1);
        let weeks = weeks_between_naive("2023-01-06", "2023-01-26");
        assert_eq!(weeks, 2);
    }
}
