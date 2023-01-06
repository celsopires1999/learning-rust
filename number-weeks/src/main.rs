use chrono::prelude::*;

fn main() {
    let dt = Utc.with_ymd_and_hms(2014, 11, 28, 12, 0, 9).unwrap();
    let fixed_dt = dt.with_timezone(&FixedOffset::east_opt(9 * 3600).unwrap());

    // method 1
    assert_eq!(
        "2014-11-28T12:00:09Z".parse::<DateTime<Utc>>(),
        Ok(dt.clone())
    );

    let date = "2023-01-06T19:56:00+03:00".parse::<DateTime<FixedOffset>>();
    println!("{:?}", date.unwrap());
    dbg!(date);

    assert_eq!(
        "2014-11-28T21:00:09+09:00".parse::<DateTime<Utc>>(),
        Ok(dt.clone())
    );
    assert_eq!(
        "2014-11-28T21:00:09+09:00".parse::<DateTime<FixedOffset>>(),
        Ok(fixed_dt.clone())
    );

    // method 2
    assert_eq!(
        DateTime::parse_from_str("2014-11-28 21:00:09 +09:00", "%Y-%m-%d %H:%M:%S %z"),
        Ok(fixed_dt.clone())
    );
    assert_eq!(
        DateTime::parse_from_rfc2822("Fri, 28 Nov 2014 21:00:09 +0900"),
        Ok(fixed_dt.clone())
    );
    assert_eq!(
        DateTime::parse_from_rfc3339("2014-11-28T21:00:09+09:00"),
        Ok(fixed_dt.clone())
    );

    // method 3
    assert_eq!(
        Utc.datetime_from_str("2014-11-28 12:00:09", "%Y-%m-%d %H:%M:%S"),
        Ok(dt.clone())
    );
    assert_eq!(
        Utc.datetime_from_str("Fri Nov 28 12:00:09 2014", "%a %b %e %T %Y"),
        Ok(dt.clone())
    );

    // oops, the year is missing!
    assert!(Utc
        .datetime_from_str("Fri Nov 28 12:00:09", "%a %b %e %T %Y")
        .is_err());
    // oops, the format string does not include the year at all!
    assert!(Utc
        .datetime_from_str("Fri Nov 28 12:00:09", "%a %b %e %T")
        .is_err());
    // oops, the weekday is incorrect!
    assert!(Utc
        .datetime_from_str("Sat Nov 28 12:00:09 2014", "%a %b %e %T %Y")
        .is_err());
}
