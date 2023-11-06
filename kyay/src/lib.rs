pub struct Entry {
    pub word: wa::Word,
    pub definitions: Vec<Definition>
}

pub struct Definition {
    pub tags: Vec<String>,
    pub definition: String,
}