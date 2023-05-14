use gtk::glib;
use std::collections::HashMap;

/// A [Weekday] is a day of the week.
#[derive(
    Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, strum::EnumString, strum::AsRefStr,
)]
#[strum(serialize_all = "snake_case")]
pub enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl glib::ToValue for Weekday {
    fn to_value(&self) -> glib::Value {
        self.as_ref().to_value()
    }

    fn value_type(&self) -> glib::Type {
        <String as glib::StaticType>::static_type()
    }
}

impl glib::StaticType for Weekday {
    fn static_type() -> glib::Type {
        <String as glib::StaticType>::static_type()
    }
}

// Extract the Weekday from glib::DateTime
impl From<glib::DateTime> for Weekday {
    fn from(date_time: glib::DateTime) -> Self {
        match date_time.day_of_week() {
            1 => Weekday::Monday,
            2 => Weekday::Tuesday,
            3 => Weekday::Wednesday,
            4 => Weekday::Thursday,
            5 => Weekday::Friday,
            6 => Weekday::Saturday,
            7 => Weekday::Sunday,
            _ => panic!("Invalid day of week"),
        }
    }
}

// Implement Next and Previous for Weekday
impl Weekday {
    pub fn as_str(&self) -> &str {
        self.as_ref()
    }

    pub fn as_short_str(&self) -> &str {
        match self {
            Weekday::Monday => "Mon",
            Weekday::Tuesday => "Tue",
            Weekday::Wednesday => "Wed",
            Weekday::Thursday => "Thu",
            Weekday::Friday => "Fri",
            Weekday::Saturday => "Sat",
            Weekday::Sunday => "Sun",
        }
    }

    pub fn as_short_str_uppercase(&self) -> &str {
        match self {
            Weekday::Monday => "MON",
            Weekday::Tuesday => "TUE",
            Weekday::Wednesday => "WED",
            Weekday::Thursday => "THU",
            Weekday::Friday => "FRI",
            Weekday::Saturday => "SAT",
            Weekday::Sunday => "SUN",
        }
    }

    pub fn from_short_str(short_str: &str) -> Option<Self> {
        match short_str {
            "Mon" => Some(Weekday::Monday),
            "Tue" => Some(Weekday::Tuesday),
            "Wed" => Some(Weekday::Wednesday),
            "Thu" => Some(Weekday::Thursday),
            "Fri" => Some(Weekday::Friday),
            "Sat" => Some(Weekday::Saturday),
            "Sun" => Some(Weekday::Sunday),
            _ => None,
        }
    }

    pub fn from_short_str_uppercase(short_str: &str) -> Option<Self> {
        match short_str {
            "MON" => Some(Weekday::Monday),
            "TUE" => Some(Weekday::Tuesday),
            "WED" => Some(Weekday::Wednesday),
            "THU" => Some(Weekday::Thursday),
            "FRI" => Some(Weekday::Friday),
            "SAT" => Some(Weekday::Saturday),
            "SUN" => Some(Weekday::Sunday),
            _ => None,
        }
    }

    pub fn next(&self) -> Self {
        match self {
            Weekday::Monday => Weekday::Tuesday,
            Weekday::Tuesday => Weekday::Wednesday,
            Weekday::Wednesday => Weekday::Thursday,
            Weekday::Thursday => Weekday::Friday,
            Weekday::Friday => Weekday::Saturday,
            Weekday::Saturday => Weekday::Sunday,
            Weekday::Sunday => Weekday::Monday,
        }
    }

    pub fn previous(&self) -> Self {
        match self {
            Weekday::Monday => Weekday::Sunday,
            Weekday::Tuesday => Weekday::Monday,
            Weekday::Wednesday => Weekday::Tuesday,
            Weekday::Thursday => Weekday::Wednesday,
            Weekday::Friday => Weekday::Thursday,
            Weekday::Saturday => Weekday::Friday,
            Weekday::Sunday => Weekday::Saturday,
        }
    }

    pub fn next_n(&self, n: u32) -> Self {
        let mut next = *self;
        for _ in 0..n {
            next = next.next();
        }
        next
    }

    pub fn previous_n(&self, n: u32) -> Self {
        let mut previous = *self;
        for _ in 0..n {
            previous = previous.previous();
        }
        previous
    }

    pub fn range(start: Weekday, end: Weekday) -> Vec<Weekday> {
        let mut range = Vec::new();
        let mut current = start;
        while current != end {
            range.push(current);
            current = current.next();
        }
        range.push(end);
        range
    }

    // Return a HashMap of a range of Glib::DateTime to Weekday for a given range of gli::DateTime
    pub fn range_datetime(
        start: &glib::DateTime,
        end: &glib::DateTime,
    ) -> HashMap<glib::DateTime, Self> {
        let mut range = HashMap::new();
        let mut current = start.clone();
        while current < end.clone() {
            range.insert(current.clone(), Weekday::from(current.clone()));
            current = current.add_days(1).unwrap();
        }
        range.insert(end.clone(), Weekday::from(end.clone()));
        range
    }
}

#[cfg(test)]
mod tests {
    use gtk::prelude::ToValue;

    use super::*;
    use std::str::FromStr;

    #[test]
    fn deserialize_weekday() {
        assert_eq!(Weekday::from_str("monday"), Ok(Weekday::Monday));
        assert_eq!(Weekday::from_str("tuesday"), Ok(Weekday::Tuesday));
        assert_eq!(Weekday::from_str("wednesday"), Ok(Weekday::Wednesday));
        assert_eq!(Weekday::from_str("thursday"), Ok(Weekday::Thursday));
        assert_eq!(Weekday::from_str("friday"), Ok(Weekday::Friday));
        assert_eq!(Weekday::from_str("saturday"), Ok(Weekday::Saturday));
        assert_eq!(Weekday::from_str("sunday"), Ok(Weekday::Sunday));
        assert!(Weekday::from_str("invalid").is_err());
    }

    #[test]
    fn serialize_weekday() {
        assert_eq!(Weekday::Monday.as_str(), "monday");
        assert_eq!(Weekday::Tuesday.as_str(), "tuesday");
        assert_eq!(Weekday::Wednesday.as_str(), "wednesday");
        assert_eq!(Weekday::Thursday.as_str(), "thursday");
        assert_eq!(Weekday::Friday.as_str(), "friday");
        assert_eq!(Weekday::Saturday.as_str(), "saturday");
        assert_eq!(Weekday::Sunday.as_str(), "sunday");
    }

    #[test]
    fn weekday_to_value() {
        assert_eq!(
            Weekday::Monday.to_value().get::<String>(),
            Ok("monday".to_string())
        );
        assert_eq!(
            Weekday::Tuesday.to_value().get::<String>(),
            Ok("tuesday".to_string())
        );
    }

    #[test]
    fn weekday_from_datetime() {
        let datetime =
            glib::DateTime::from_iso8601("2021-03-28T20:39:08.315749637+00:00", None).unwrap();
        assert_eq!(Weekday::from(datetime), Weekday::Sunday);
        let datetime =
            glib::DateTime::from_iso8601("2021-03-29T20:39:08.315749637+00:00", None).unwrap();
        assert_eq!(Weekday::from(datetime), Weekday::Monday);
        let datetime =
            glib::DateTime::from_iso8601("2020-03-30T20:39:08.315749637+00:00", None).unwrap();
        assert_eq!(Weekday::from(datetime), Weekday::Monday);
    }

    #[test]
    fn weekday_as_str() {
        assert_eq!(Weekday::Monday.as_str(), "monday");
        assert_eq!(Weekday::Tuesday.as_str(), "tuesday");
    }

    #[test]
    fn weekday_as_short_str() {
        assert_eq!(Weekday::Monday.as_short_str(), "Mon");
        assert_eq!(Weekday::Tuesday.as_short_str(), "Tue");
        assert_eq!(Weekday::Wednesday.as_short_str(), "Wed");
    }

    #[test]
    fn weekday_as_short_str_uppercase() {
        assert_eq!(Weekday::Monday.as_short_str_uppercase(), "MON");
        assert_eq!(Weekday::Tuesday.as_short_str_uppercase(), "TUE");
        assert_eq!(Weekday::Wednesday.as_short_str_uppercase(), "WED");
    }

    #[test]
    fn weekday_from_short_str() {
        assert_eq!(Weekday::from_short_str("Mon"), Some(Weekday::Monday));
        assert_eq!(Weekday::from_short_str("Tue"), Some(Weekday::Tuesday));
        assert_eq!(Weekday::from_short_str("Wed"), Some(Weekday::Wednesday));
        assert_eq!(Weekday::from_short_str("Weds"), None);
    }

    #[test]
    fn weekday_from_short_str_uppercase() {
        assert_eq!(
            Weekday::from_short_str_uppercase("MON"),
            Some(Weekday::Monday)
        );
        assert_eq!(
            Weekday::from_short_str_uppercase("TUE"),
            Some(Weekday::Tuesday)
        );
        assert_eq!(
            Weekday::from_short_str_uppercase("WED"),
            Some(Weekday::Wednesday)
        );
        assert_eq!(Weekday::from_short_str_uppercase("WEDS"), None);
    }

    #[test]
    fn weekday_next() {
        assert_eq!(Weekday::Monday.next(), Weekday::Tuesday);
        assert_eq!(Weekday::Tuesday.next(), Weekday::Wednesday);
        assert_eq!(Weekday::Sunday.next(), Weekday::Monday);
    }

    #[test]
    fn weekday_previous() {
        assert_eq!(Weekday::Monday.previous(), Weekday::Sunday);
        assert_eq!(Weekday::Tuesday.previous(), Weekday::Monday);
        assert_eq!(Weekday::Sunday.previous(), Weekday::Saturday);
    }

    #[test]
    fn weekday_next_n() {
        assert_eq!(Weekday::Monday.next_n(0), Weekday::Monday);
        assert_eq!(Weekday::Monday.next_n(1), Weekday::Tuesday);
        assert_eq!(Weekday::Monday.next_n(2), Weekday::Wednesday);
        assert_eq!(Weekday::Monday.next_n(3), Weekday::Thursday);
        assert_eq!(Weekday::Monday.next_n(4), Weekday::Friday);
        assert_eq!(Weekday::Monday.next_n(5), Weekday::Saturday);
        assert_eq!(Weekday::Monday.next_n(6), Weekday::Sunday);
        assert_eq!(Weekday::Monday.next_n(7), Weekday::Monday);
        assert_eq!(Weekday::Monday.next_n(8), Weekday::Tuesday);
    }

    #[test]
    fn weekday_previous_n() {
        assert_eq!(Weekday::Monday.previous_n(0), Weekday::Monday);
        assert_eq!(Weekday::Monday.previous_n(1), Weekday::Sunday);
        assert_eq!(Weekday::Monday.previous_n(2), Weekday::Saturday);
        assert_eq!(Weekday::Monday.previous_n(3), Weekday::Friday);
        assert_eq!(Weekday::Monday.previous_n(4), Weekday::Thursday);
        assert_eq!(Weekday::Monday.previous_n(5), Weekday::Wednesday);
        assert_eq!(Weekday::Monday.previous_n(6), Weekday::Tuesday);
        assert_eq!(Weekday::Monday.previous_n(7), Weekday::Monday);
        assert_eq!(Weekday::Monday.previous_n(8), Weekday::Sunday);
    }

    #[test]
    fn weekday_range() {
        assert_eq!(
            Weekday::range(Weekday::Monday, Weekday::Monday),
            vec![Weekday::Monday]
        );
        assert_eq!(
            Weekday::range(Weekday::Monday, Weekday::Tuesday),
            vec![Weekday::Monday, Weekday::Tuesday]
        );
        assert_eq!(
            Weekday::range(Weekday::Monday, Weekday::Wednesday),
            vec![Weekday::Monday, Weekday::Tuesday, Weekday::Wednesday,]
        );
        assert_eq!(
            Weekday::range(Weekday::Monday, Weekday::Thursday),
            vec![
                Weekday::Monday,
                Weekday::Tuesday,
                Weekday::Wednesday,
                Weekday::Thursday,
            ]
        );
        assert_eq!(
            Weekday::range(Weekday::Monday, Weekday::Friday),
            vec![
                Weekday::Monday,
                Weekday::Tuesday,
                Weekday::Wednesday,
                Weekday::Thursday,
                Weekday::Friday,
            ]
        );
        assert_eq!(
            Weekday::range(Weekday::Monday, Weekday::Saturday),
            vec![
                Weekday::Monday,
                Weekday::Tuesday,
                Weekday::Wednesday,
                Weekday::Thursday,
                Weekday::Friday,
                Weekday::Saturday,
            ]
        );
        assert_eq!(
            Weekday::range(Weekday::Monday, Weekday::Sunday),
            vec![
                Weekday::Monday,
                Weekday::Tuesday,
                Weekday::Wednesday,
                Weekday::Thursday,
                Weekday::Friday,
                Weekday::Saturday,
                Weekday::Sunday,
            ]
        );
        assert_eq!(
            Weekday::range(Weekday::Monday, Weekday::Monday),
            vec![Weekday::Monday]
        );
        assert_eq!(
            Weekday::range(Weekday::Tuesday, Weekday::Monday),
            vec![
                Weekday::Tuesday,
                Weekday::Wednesday,
                Weekday::Thursday,
                Weekday::Friday,
                Weekday::Saturday,
                Weekday::Sunday,
                Weekday::Monday,
            ]
        );
        assert_eq!(
            Weekday::range(Weekday::Wednesday, Weekday::Monday),
            vec![
                Weekday::Wednesday,
                Weekday::Thursday,
                Weekday::Friday,
                Weekday::Saturday,
                Weekday::Sunday,
                Weekday::Monday,
            ]
        );
        assert_eq!(
            Weekday::range(Weekday::Thursday, Weekday::Monday),
            vec![
                Weekday::Thursday,
                Weekday::Friday,
                Weekday::Saturday,
                Weekday::Sunday,
                Weekday::Monday,
            ]
        );
    }

    #[test]
    fn weekday_range_datetime() {
        let start =
            glib::DateTime::from_iso8601("2021-03-28T20:39:08.315749637+00:00", None).unwrap();
        let end =
            glib::DateTime::from_iso8601("2021-04-03T20:39:08.315749637+00:00", None).unwrap();
        let range = Weekday::range_datetime(&start, &end);
        assert_eq!(range.len(), 7);
        assert_eq!(range.get(&start).unwrap(), &Weekday::Sunday);
        assert_eq!(range.get(&end).unwrap(), &Weekday::Saturday);
    }
}
