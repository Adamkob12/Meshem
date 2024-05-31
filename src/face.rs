#[derive(Copy, Clone, Debug, PartialEq, Eq)]
/// This enum represents all the faces of a cubic voxel.
pub enum Face {
    Top,
    Bottom,
    Right,
    Left,
    Back,
    Forward,
}

pub(crate) const OFFSET_CONST: u32 = 0b0001_1111_1111_1111_1111_1111_1111_1111;
pub(crate) const REVERSE_OFFSET_CONST: u32 = 0b1110_0000_0000_0000_0000_0000_0000_0000;

/// Funtion converts a `Face` into an its encoded representation for opcode.
pub fn face_to_u32(f: Face) -> u32 {
    match f {
        // 101 -> 2^31 + 2^29
        Face::Top => 2_684_354_560,
        // 100 -> 2^31
        Face::Bottom => 2_147_483_648,
        // 011 -> 2^30 + 2^29
        Face::Right => 1_610_612_736,
        // 010 -> 2^30
        Face::Left => 1_073_741_824,
        // 001 -> 2^29
        Face::Back => 536_870_912,
        // 000 -> 0
        Face::Forward => 0,
    }
}

pub fn face_from_u32(u: u32) -> Face {
    match u {
        2_684_354_560 => Face::Top,
        // 100 -> 2^31
        2_147_483_648 => Face::Bottom,
        // 011 -> 2^30 + 2^29
        1_610_612_736 => Face::Right,
        // 010 -> 2^30
        1_073_741_824 => Face::Left,
        // 001 -> 2^29
        536_870_912 => Face::Back,
        // 000 -> 0
        0 => Face::Forward,

        _ => panic!("Can't convert u32 to Face"),
    }
}

impl Face {
    /// The opposite of the `Face` given.
    pub fn opposite(&self) -> Face {
        match *self {
            Face::Top => Face::Bottom,
            Face::Bottom => Face::Top,
            Face::Right => Face::Left,
            Face::Left => Face::Right,
            Face::Back => Face::Forward,
            Face::Forward => Face::Back,
        }
    }
}

impl Into<usize> for Face {
    fn into(self) -> usize {
        match self {
            Face::Top => 0,
            Face::Bottom => 1,
            Face::Right => 2,
            Face::Left => 3,
            Face::Back => 4,
            Face::Forward => 5,
        }
    }
}

impl From<usize> for Face {
    fn from(i: usize) -> Face {
        match i {
            0 => Face::Top,
            1 => Face::Bottom,
            2 => Face::Right,
            3 => Face::Left,
            4 => Face::Back,
            5 => Face::Forward,
            _ => panic!("Face can only be infered from 6 values, 0..5 (inclucive)"),
        }
    }
}
