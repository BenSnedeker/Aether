pub struct Location {
    pub line: u32,
    pub char: u32,
}

// todo: create methods to read / write Location to and from a Vec<u8>

pub enum Change {
    // TODO: These are placeholder Change types. Replace these with better implementations and more types.
    Insert { chr: char, loc: Location },
    Delete { chr: char, loc: Location },
}

// todo: create methods to read / write Change to and from a Vec<u8>