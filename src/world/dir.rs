// SPDX-FileCopyrightText: 2023 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

#![warn(missing_docs)]

/// Dir is the 8 primary directions, plus None.
#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
#[allow(missing_docs)]
pub enum Dir {
    /// No direction
    ///
    /// Used mainly to have an end for iterators
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
    /// Returns a new direction after turning 45 degrees to the right.
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

    /// Returns a new direction after turning 45 degrees to the left.
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

    /// Returns a new direction after turning 90 degrees left.
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

    /// Returns a new direction after turning 90 degrees right.
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
