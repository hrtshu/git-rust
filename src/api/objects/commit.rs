
use std::fmt::Display;

use chrono::{DateTime, FixedOffset, Local, Offset, TimeZone};

use super::base::ObjectBase;
use super::io::{Hash, STR_HASH_LEN};

pub struct User {
  pub name: String,
  pub email: String,
}

impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} <{}>", self.name, self.email)
    }
}

pub struct Timestamp {
  epoch: i64,
  tz_sec: i32,
}

impl Timestamp {
  pub fn now() -> Self {
    Self::from_datetime(Local::now())
  }

  pub fn from_datetime<Tz>(datetime: DateTime<Tz>) -> Self where Tz: TimeZone {
    Self {
      epoch: datetime.timestamp(),
      tz_sec: datetime.offset().fix().local_minus_utc(),
    }
  }
}

impl Display for Timestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      let offset =
        if self.tz_sec >= 0 {
          FixedOffset::east(self.tz_sec)
        } else {
          FixedOffset::west(-self.tz_sec)
        };
      let offset_str = offset.to_string();
      let splitted_offset: Vec<&str> = offset_str.split(':').collect();
      let offset_str = splitted_offset.join("");
      let offset_str = &offset_str[..5]; // TODO: 本当に秒を削ぎ落とす必要があるかを検討
      write!(f, "{} {}", self.epoch, offset_str)
    }
}

pub struct CommitObject {
  tree_hash: Hash,
  parent_hash: Hash,
  author: User,
  author_timestamp: Timestamp,
  committer: User,
  commit_timestamp: Timestamp,
  message: String,
}

impl ObjectBase for CommitObject {
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
