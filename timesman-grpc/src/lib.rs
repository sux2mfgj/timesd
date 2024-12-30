pub mod grpc {
    tonic::include_proto!("timesman");
}

impl Into<timesman_bstore::Times> for grpc::Times {
    fn into(self) -> timesman_bstore::Times {
        let c = self.created_at.unwrap();
        let ctime = chrono::DateTime::from_timestamp(c.seconds, c.nanos as u32)
            .unwrap();

        let utime = if let Some(u) = self.updated_at {
            Some(
                chrono::DateTime::from_timestamp(u.seconds, u.nanos as u32)
                    .unwrap()
                    .naive_local(),
            )
        } else {
            None
        };

        timesman_bstore::Times {
            id: self.id as i64,
            title: self.title,
            created_at: ctime.naive_local(),
            updated_at: utime,
        }
    }
}

use chrono::{Datelike, Timelike};

impl From<timesman_bstore::Times> for grpc::Times {
    fn from(value: timesman_bstore::Times) -> Self {
        let ctime = value.created_at;
        let ctime = {
            let c = value.created_at;
            prost_types::Timestamp::date_time(
                c.year() as i64,
                c.month() as u8,
                c.day() as u8,
                c.hour() as u8,
                c.minute() as u8,
                c.second() as u8,
            )
            .unwrap()
        };

        let utime = if let Some(u) = value.updated_at {
            let t = prost_types::Timestamp::date_time(
                u.year() as i64,
                u.month() as u8,
                u.day() as u8,
                u.hour() as u8,
                u.minute() as u8,
                u.second() as u8,
            )
            .unwrap();
            Some(t)
        } else {
            None
        };

        Self {
            id: value.id as u64,
            title: value.title,
            created_at: Some(ctime),
            updated_at: utime,
        }
    }
}