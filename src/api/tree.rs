use std::collections::HashMap;
use std::fmt::Debug;

use super::objects::blob::BlobObject;
use super::objects::io::{Hash, ObjectWriter};
use super::objects::tree::{Mode, TreeObject};

struct Blob {
    path: String
}

pub struct Tree {
    entries: HashMap<TreeEntryName, TreeEntry>,
}

impl Tree {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    pub fn add_path(&mut self, path: &String, is_file: bool) {
        let mut split_path: Vec<String> = path.split("/").map(|s| s.to_string()).collect();
        self._add_path(&mut split_path, is_file, path)
    }

    fn _add_path(&mut self, path: &mut Vec<TreeEntryName>, is_file: bool, fullpath: &String) {
        if path.len() <= 0 {
            return;
        }
        let entry_name = path.remove(0);
        let entry_object =
            if is_file && path.len() == 0 {
                TreeEntryObject::Blob(Blob { path: fullpath.to_owned() })
            } else {
                TreeEntryObject::Tree(Tree::new())
            };
        let mut entry =
            match self.entries.remove(&entry_name) {
                Some(e) => e,
                None => 
                    TreeEntry {
                        // mode: String::from(""),
                        object: entry_object,
                    },
            };
        entry.object = match entry.object {
            TreeEntryObject::Blob(blob) => TreeEntryObject::Blob(blob),
            TreeEntryObject::Tree(mut tree) => {
                tree._add_path(path, is_file, fullpath);
                TreeEntryObject::Tree(tree)
            }
        };
        self.entries.insert(entry_name, entry);
    }

    pub fn write_recursively(self) -> std::io::Result<Hash> {
        let mut tree_object = TreeObject::new();
        for (name, entry) in self.entries {
            let mut mode = Mode(0o40000);
            let hash = match entry.object {
                TreeEntryObject::Blob(blob) => {
                    mode = Mode(0o100644);
                    let blob_object = BlobObject::from_path(&blob.path)?;
                    ObjectWriter::write(blob_object)?
                },
                TreeEntryObject::Tree(tree) => {
                    tree.write_recursively()?
                },
            };
            tree_object.add(super::objects::tree::TreeEntry {
                mode,
                name,
                hash
            })
        }
        ObjectWriter::write(tree_object)
    }
}

impl Debug for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Tree(")?;
        for (idx, (name, entry)) in self.entries.iter().enumerate() {
            write!(f, "{:?}: {:?}", name, entry)?;
            if idx < self.entries.len() - 1 {
                write!(f, ", ")?;
            }
        };
        write!(f, ")")?;
        Ok(())
    }
}

type TreeEntryName = String;

struct TreeEntry {
    // mode: String,
    object: TreeEntryObject,
}

impl Debug for TreeEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.object {
            TreeEntryObject::Blob(blob) => {
                write!(f, "<blob: {:?}>", blob.path)
            },
            TreeEntryObject::Tree(tree) => {
                write!(f, "{:?}", tree)
            }
        }
    }
}

enum TreeEntryObject {
    Blob(Blob),
    Tree(Tree),
}
