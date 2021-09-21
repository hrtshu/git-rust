use std::fmt::Display;

use chrono::{DateTime, FixedOffset, Local, Offset, TimeZone};

use crate::api::common::user::User;

use super::base::ObjectBase;
use super::io::{Hash, STR_HASH_LEN};

pub struct Timezone {
    tz_sec: i32
}

impl Timezone {
    pub fn from_sec(sec: i32) -> Self {
        Self { tz_sec: sec }
    }

    pub fn from_chrono_offset(offset: FixedOffset) -> Self {
        Self::from_sec(offset.fix().local_minus_utc())
    }

    pub fn to_chrono_offset(self) -> FixedOffset {
        if self.tz_sec >= 0 {
            FixedOffset::east(self.tz_sec)
        } else {
            FixedOffset::west(-self.tz_sec)
        }
    }
}

impl Display for Timezone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let offset = self.to_chrono_offset();
        let offset_str = offset.to_string();
        let splitted_offset: Vec<&str> = offset_str.split(':').collect();
        let offset_str = splitted_offset.join("");
        let offset_str = &offset_str[..5]; // TODO: 本当に秒を削ぎ落とす必要があるかを検討
        write!(f, "{}", offset_str)
    }
}

pub struct Timestamp {
    epoch: i64,
    timezone: Timezone,
}

impl Timestamp {
    pub fn now() -> Self {
        Self::from_datetime(Local::now())
    }

    pub fn from_datetime<Tz>(datetime: DateTime<Tz>) -> Self where Tz: TimeZone {
        Self {
            epoch: datetime.timestamp(),
            timezone: Timezone::from_chrono_offset(datetime.offset().fix()),
        }
  }
}

impl Display for Timestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.epoch, self.timezone)
    }
}

pub struct CommitObject<'a> {
    pub tree_hash: Hash,
    pub parent_hash: Hash,
    pub author: &'a User,
    pub author_timestamp: &'a Timestamp,
    pub committer: &'a User,
    pub commit_timestamp: &'a Timestamp,
    pub message: String,
}

impl <'a>ObjectBase for CommitObject<'a> {
    fn obj_type(&self) -> &str {
        "commit"
    }

    fn body_size(&self) -> usize {
        4 + 1 + STR_HASH_LEN + 1 +
        6 + 1 + STR_HASH_LEN + 1 +
        6 + 1 + self.author.to_string().len() + 1 + self.author_timestamp.to_string().len() + 1 +
        9 + 1 + self.committer.to_string().len() + 1 + self.commit_timestamp.to_string().len() + 1 +
        1 +
        self.message.len() + 1
    }

    fn write_body_to<W>(&self, writer: &mut W) -> std::io::Result<()> where W: std::io::Write {
        writeln!(writer, "tree {}", &self.tree_hash)?;
        writeln!(writer, "parent {}", &self.parent_hash)?;
        writeln!(writer, "author {} {}", &self.author, &self.author_timestamp)?;
        writeln!(writer, "committer {} {}", &self.committer, &self.commit_timestamp)?;
        writeln!(writer)?;
        writeln!(writer, "{}", self.message)?;
        Ok(())
    }
}
