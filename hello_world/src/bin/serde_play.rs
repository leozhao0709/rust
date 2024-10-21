use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

mod date_time_utc_format {
    use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &str = "%Y-%m-%d %H:%M:%S"; // Adjust format as needed

    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        NaiveDateTime::parse_from_str(&s, FORMAT)
            .map_err(|err| serde::de::Error::custom(format!("Parse error: {}, input: {}", err, s)))
            .map(|native_dt| Utc.from_utc_datetime(&native_dt))
    }
}

#[derive(Serialize, Debug, Deserialize)]
struct Dog {
    name: String,
    #[serde(with = "date_time_utc_format")]
    birthday: DateTime<Utc>,
    age: Option<u8>,
}

fn main() {
    let dog = Dog {
        name: "d1".to_owned(),
        birthday: Utc::now(),
        age: Some(5),
    };

    let json = serde_json::to_string(&dog).unwrap_or_default();
    println!("{json:#?}");
    let json_pretty = serde_json::to_string_pretty(&dog).unwrap_or_default();
    println!("{json_pretty:#?}");

    let json_str = r#"{"name": "d1", "birthday": "2024-10-21 05:18:27"}"#;
    let dog = serde_json::from_str::<Dog>(json_str); // only accept &str
    println!("{dog:#?}");
}
