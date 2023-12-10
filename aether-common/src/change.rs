use std::fmt::Display;

pub struct Location {
    pub line: usize,
    pub chr: usize,
}

impl Location {

    pub fn to_le_bytes(&self) -> Vec<u8> {
        [self.line.to_le_bytes(), self.chr.to_le_bytes()].concat()
    }

    pub fn from_le_bytes(bytes: Vec<u8>) -> Self {
        let (line_bytes, chr_bytes) = bytes.split_at(std::mem::size_of::<usize>());
        Location {
            line: usize::from_le_bytes(line_bytes.try_into().unwrap()),
            chr: usize::from_le_bytes(chr_bytes.try_into().unwrap()),
        }
    }

}

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "line: {}, char: {}", self.line, self.chr)
    }
}

pub struct ChangeFile {
    pub change_type: ChangeType,
    pub file_path: String,
    pub loc: Location,
}

impl ChangeFile {

    pub fn to_le_bytes(&self) -> Vec<u8> {
        let mut result = Vec::new();

        // get the variables as bytes
        let change_type_bytes = self.change_type.to_le_bytes();
        let file_path_bytes = self.file_path.as_bytes();
        let loc_bytes = self.loc.to_le_bytes();

        // store their varint lengths to make reading easier
        let total_length = [
            change_type_bytes.len() as u8,
            file_path_bytes.len() as u8,
            // don't need to write the length of location as it takes up the remaining space
        ];

        // push everything into a single vector
        result.extend(total_length);
        result.extend(change_type_bytes);
        result.extend(file_path_bytes);
        result.extend(loc_bytes);

        result
    }

    pub fn from_le_bytes(bytes: Vec<u8>) -> Self {
        let mut byte_slice = bytes.as_slice();

        // Read varint lengths
        let change_type_len = byte_slice[0] as usize;
        let file_path_len = byte_slice[1] as usize;
        byte_slice = &byte_slice[2..];

        // Extract data
        let change_type_bytes = &byte_slice[0..change_type_len];
        let file_path_bytes = &byte_slice[change_type_len..change_type_len + file_path_len];
        let loc_bytes = &byte_slice[change_type_len + file_path_len..];

        // read from bytes
        let change_type = ChangeType::from_le_bytes(change_type_bytes.to_vec());
        let file_path = String::from_utf8_lossy(file_path_bytes).to_string();
        let loc = Location::from_le_bytes(loc_bytes.to_vec());

        // return the data
        ChangeFile {
            change_type,
            file_path,
            loc,
        }
    }

}

impl Display for ChangeFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Change to {}: {} at {}", self.file_path, self.change_type, self.loc)
    }
}

pub enum ChangeType {
    Insert { new: String },
    Delete { len: usize },
    Overwrite { len: usize, new: String },
}

impl ChangeType {

    pub fn get_ident(&self) -> u8 {
        match self {
            Self::Insert { .. } => 0,
            Self::Delete { .. } => 1,
            Self::Overwrite { .. } => 2,
        }
    }

    pub fn to_le_bytes(&self) -> Vec<u8> {
        let mut result = Vec::new();
        result.push(self.get_ident());

        match self {
            Self::Insert { new } => {
                result.extend(new.as_bytes());
            }
            Self::Delete { len } => {
                result.extend(len.to_le_bytes());
            }
            Self::Overwrite { len, new } => {
                result.extend(len.to_le_bytes());
                result.extend(new.as_bytes());
            }
        }

        result
    }

    pub fn from_le_bytes(bytes: Vec<u8>) -> Self {
        let mut byte_slice = bytes.as_slice();
        let ident = byte_slice[0];
        byte_slice = &byte_slice[1..]; // Move past the identifier

        match ident {
            0 => {
                let new = String::from_utf8_lossy(&byte_slice[0..])
                    .to_string();
                Self::Insert { new }
            }
            1 => {
                let len_bytes = &byte_slice[0..];
                let len = usize::from_le_bytes(len_bytes.try_into().unwrap());
                Self::Delete { len }
            }
            2 => {
                let len_bytes = &byte_slice[0..];
                let len = usize::from_le_bytes(len_bytes.try_into().unwrap());
                let new = String::from_utf8_lossy(&byte_slice[std::mem::size_of::<usize>()..])
                    .to_string();
                Self::Overwrite { len, new }
            }
            _ => panic!("Invalid identifier"),
        }
    }

}

impl Display for ChangeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChangeType::Insert { new } => write!(f, "Insert {}", new),
            ChangeType::Delete { len } => write!(f, "Delete {} characters", len),
            ChangeType::Overwrite { len, new } => write!(f, "Overwrite {} with {}", len, new),
        }
    }
}