use std::collections::HashMap;
use std::fmt::Debug;

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

    pub fn add_path(&mut self, path: &mut Vec<TreeEntryName>, is_file: bool) {
        if path.len() <= 0 {
            return;
        }
        let entry_name = path.remove(0);
        let entry_object =
            if is_file && path.len() == 0 {
                TreeEntryObject::Blob(Blob { path: entry_name.clone() })
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
                tree.add_path(path, is_file);
                TreeEntryObject::Tree(tree)
            }
        };
        self.entries.insert(entry_name, entry);
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