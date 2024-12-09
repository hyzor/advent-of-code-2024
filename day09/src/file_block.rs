pub  struct FileBlock {
    pub id: i16,
    pub value: char,
}

impl Clone for FileBlock {
    fn clone(&self) -> Self {
        FileBlock {
            id: self.id,
            value: self.value,
        }
    }
}

impl std::fmt::Debug for FileBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}