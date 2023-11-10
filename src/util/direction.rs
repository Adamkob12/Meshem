use crate::prelude::{Face, Face::*};
use std::fmt;

#[derive(Clone, Copy, PartialEq)]
pub enum Direction {
    North,
    South,
    East,
    West,
    NoEast,
    NoWest,
    SoEast,
    SoWest,
}
use Direction::*;

impl Into<&'static str> for Direction {
    fn into(self) -> &'static str {
        match self {
            North => "North",
            South => "South",
            East => "East",
            West => "West",
            NoEast => "NoEast",
            NoWest => "NoWest",
            SoEast => "SoEast",
            SoWest => "SoWest",
        }
    }
}

impl fmt::Debug for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Direction::{}", Into::<&'static str>::into(*self))
    }
}

impl Direction {
    pub fn add_direction(dir1: Option<Direction>, dir2: Option<Direction>) -> Option<Direction> {
        if dir1.is_none() {
            return dir2;
        }
        if dir2.is_none() {
            return dir1;
        }

        let d1 = to_cords(dir1);
        let d2 = to_cords(dir2);
        from_cords_change([d1[0] + d2[0], d1[1] + d2[1]])
    }

    pub fn opposite(&self) -> Direction {
        match self {
            North => South,
            South => North,
            East => West,
            West => East,
            SoEast => NoWest,
            NoWest => SoEast,
            SoWest => NoEast,
            NoEast => SoWest,
        }
    }

    pub fn decompose(&self) -> (Option<Direction>, Option<Direction>) {
        match self {
            North => (Some(North), None),
            South => (Some(South), None),
            East => (None, Some(East)),
            West => (None, Some(West)),
            SoEast => (Some(South), Some(East)),
            NoWest => (Some(North), Some(West)),
            SoWest => (Some(South), Some(West)),
            NoEast => (Some(North), Some(East)),
        }
    }
}

pub fn to_cords(dir: Option<Direction>) -> [i32; 2] {
    if dir.is_none() {
        return [0, 0];
    }
    let dir = dir.unwrap();
    match dir {
        North => [0, 1],
        South => [0, -1],
        East => [1, 0],
        West => [-1, 0],
        NoEast => [1, 1],
        NoWest => [-1, 1],
        SoEast => [1, -1],
        SoWest => [-1, -1],
    }
}

pub fn from_cords_change(change: [i32; 2]) -> Option<Direction> {
    if change[1] < 0 {
        if change[0] > 0 {
            return Some(Direction::SoEast);
        }
        if change[0] == 0 {
            return Some(Direction::South);
        }
        if change[0] < 0 {
            return Some(Direction::SoWest);
        }
    }
    if change[1] == 0 {
        if change[0] > 0 {
            return Some(Direction::East);
        }
        if change[0] == 0 {
            return None;
        }
        if change[0] < 0 {
            return Some(Direction::West);
        }
    }
    if change[1] > 0 {
        if change[0] > 0 {
            return Some(Direction::NoEast);
        }
        if change[0] == 0 {
            return Some(Direction::North);
        }
        if change[0] < 0 {
            return Some(Direction::NoWest);
        }
    }
    unreachable!();
}

impl From<Face> for Direction {
    fn from(value: Face) -> Self {
        match value {
            Back => Self::North,
            Forward => Self::South,
            Right => Self::East,
            Left => Self::West,
            Top => {
                assert!(false, "Face::Top cannot be cast as Direction");
                Self::North
            }
            Bottom => {
                assert!(false, "Face::Bottom cannot be cast as Direction");
                Self::South
            }
        }
    }
}

impl Into<usize> for Direction {
    fn into(self) -> usize {
        match self {
            Direction::North => 0,
            Direction::South => 1,
            Direction::East => 2,
            Direction::West => 3,
            Direction::NoEast => 4,
            Direction::NoWest => 5,
            Direction::SoEast => 6,
            Direction::SoWest => 7,
        }
    }
}

impl Into<Face> for Direction {
    fn into(self) -> Face {
        match self {
            Self::NoWest | Self::NoEast | Direction::North => Back,
            Self::SoWest | Self::SoEast | Direction::South => Forward,
            Self::West => Left,
            Self::East => Right,
        }
    }
}

impl From<u8> for Direction {
    fn from(value: u8) -> Self {
        match value {
            1 => Direction::North,
            2 => Direction::South,
            3 => Self::East,
            4 => Self::West,
            5 => Self::NoEast,
            6 => Self::NoWest,
            7 => Direction::SoEast,
            8 => Direction::SoWest,
            _ => unreachable!(),
        }
    }
}
