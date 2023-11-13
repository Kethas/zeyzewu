use std::collections::HashMap;

use derive_more::{Deref, DerefMut};
use serde_derive::{Deserialize, Serialize};
use serde_with::serde_as;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Definition {
    pub tags: Vec<String>,
    pub definition: String,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DictionaryEntry(pub wa::Syllable, pub Vec<Definition>);

#[serde_as]
#[derive(Deref, DerefMut, Serialize, Deserialize)]
pub struct Dictionary {
    #[deref]
    #[serde_as(as = "Vec<(_, _)>")]
    entries: HashMap<wa::Syllable, Vec<Definition>>,
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
