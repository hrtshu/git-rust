
use crate::api::common::datetime::Timestamp;
use crate::api::common::user::User;

use super::base::ObjectBase;
use super::io::{Hash, STR_HASH_LEN};

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
