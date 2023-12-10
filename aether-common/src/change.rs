pub struct Location {
    pub line: u32,
    pub chr: u32,
}

// todo(eric): create methods to read / write Location to and from a Vec<u8>

pub enum Change {
    Insert { loc: Location, str: String },
    Delete { loc: Location, length: usize },
    Overwrite { loc: Location, length: usize, str: String },
}

// todo(eric): create methods to read / write Change to and from a Vec<u8>