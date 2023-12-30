// SPDX-FileCopyrightText: 2023 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub enum Dir {
    None,
    North,
    South,
    East,
    West,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

impl std::fmt::Display for Dir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Dir::*;
        write!(
            f,
            "{}",
            match self {
                None => "No direction",
                North => "North",
                South => "South",
                East => "East",
                West => "West",
                NorthEast => "North-East",
                NorthWest => "North-West",
                SouthEast => "South-East",
                SouthWest => "South-West",
            }
        )
    }
}

impl Dir {
    pub fn turn_right(self) -> Self {
        use Dir::*;
        match self {
            None => self,
            North => NorthEast,
            South => SouthWest,
            East => SouthEast,
            West => NorthWest,
            NorthEast => East,
            NorthWest => North,
            SouthEast => South,
            SouthWest => West,
        }
    }

    pub fn turn_left(self) -> Self {
        use Dir::*;
        match self {
            None => self,
            North => NorthWest,
            South => SouthEast,
            East => NorthEast,
            West => SouthWest,
            NorthEast => North,
            NorthWest => West,
            SouthEast => East,
            SouthWest => South,
        }
    }

    pub fn turn_cardinal_left(self) -> Self {
        use Dir::*;
        match self {
            None => self,
            North => West,
            South => East,
            East => North,
            West => South,
            _ => panic!("Direction {} is invalid", self),
        }
    }

    pub fn turn_cardinal_right(self) -> Self {
        use Dir::*;
        match self {
            None => self,
            North => East,
            South => West,
            East => South,
            West => North,
            _ => panic!("Direction {} is invalid", self),
        }
    }
}
