
use std::fmt::Display;

use super::base::ObjectBase;
use super::io::Hash;

pub struct User {
  pub name: String,
  pub email: String,
}

impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} <{}>", self.name, self.email)
    }
}

pub struct CommitObject {
  tree_hash: Hash,
  parent_hash: Hash,
  author: User,
  author_date: String,
  committer: User,
  commit_date: String,
  message: String,
}

impl ObjectBase for CommitObject {
    fn obj_type(&self) -> &str {
        "commit"
    }

    fn body_size(&self) -> usize {
        todo!()
    }

    fn write_body_to<W>(&self, writer: &mut W) -> std::io::Result<()> where W: std::io::Write {
        writeln!(writer, "tree {}", &self.tree_hash)?;
        writeln!(writer, "parent {}", &self.parent_hash)?;
        writeln!(writer, "author {} {}", &self.author, &self.author_date)?;
        writeln!(writer, "committer {} {}", &self.committer, &self.commit_date)?;
        writeln!(writer)?;
        writeln!(writer, "{}", self.message)?;
        Ok(())
    }
}
