use std::collections::HashMap;

use derive_more::{Deref, DerefMut};
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Definition {
    pub tags: Vec<String>,
    pub definition: String,
}

pub struct DictionaryEntry(pub wa::Word, pub Vec<Definition>);

#[derive(Deref, DerefMut, Serialize, Deserialize)]
pub struct Dictionary {
    #[deref]
    entries: HashMap<wa::Word, Vec<Definition>>,
}

impl Dictionary {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }
}

impl Default for Dictionary {
    fn default() -> Self {
        Self::new()
    }
}
