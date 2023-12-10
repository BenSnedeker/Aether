pub struct Location {
    pub line: u32,
    pub chr: u32,
}

impl Location {

    pub fn to_le_bytes(&self) -> Vec<u8> {
        [self.line.to_le_bytes(), self.chr.to_le_bytes()].concat()
    }

    pub fn from_le_bytes(bytes: Vec<u8>) -> Self {
        let (line_bytes, chr_bytes) = bytes.split_at(std::mem::size_of::<u32>());
        Location {
            line: u32::from_le_bytes(line_bytes.try_into().unwrap()),
            chr: u32::from_le_bytes(chr_bytes.try_into().unwrap()),
        }
    }

}

pub enum Change {
    Insert { loc: Location, str: String },
    Delete { loc: Location, length: usize },
    Overwrite { loc: Location, length: usize, str: String },
}

// todo(eric): create methods to read / write Change to and from a Vec<u8>