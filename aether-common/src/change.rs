use std::fmt::Display;

use send_it::Segment;

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

    pub fn to_segments(&self) -> Vec<Segment> {
        let change_type_seg = Segment::from(self.change_type.to_le_bytes());
        let file_path_seg = Segment::from(self.file_path.clone());
        let loc_seg = Segment::from(self.loc.to_le_bytes());

        vec![change_type_seg, file_path_seg, loc_seg]
    }

    pub fn from_segments(segs: Vec<Segment>) -> Result<ChangeFile, String> {
        if segs.len() != 3 {
            return Err("Invalid segment amount while trying to get ChangeFile!".to_string());
        }

        let change_type_seg = segs.first().unwrap();
        let file_path_seg = segs.get(1).unwrap();
        let loc_seg = segs.last().unwrap();

        Ok(ChangeFile {
            change_type: ChangeType::from_le_bytes(change_type_seg.to_raw()),
            file_path: file_path_seg.to_string(),
            loc: Location::from_le_bytes(loc_seg.to_raw())
        })
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