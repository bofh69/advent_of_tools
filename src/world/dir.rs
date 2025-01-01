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

/// An array of the cardinal directions
pub const CARDINALS: [Dir; 4] = [Dir::North, Dir::East, Dir::South, Dir::West];

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
    ///
    /// # Example
    /// ```
    /// # use advent_of_tools::Dir;
    /// assert_eq!(Dir::NorthWest.turn_right(), Dir::North);
    /// assert_eq!(Dir::North.turn_right(), Dir::NorthEast);
    /// ```
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
    ///
    /// # Example
    /// ```
    /// # use advent_of_tools::Dir;
    /// assert_eq!(Dir::NorthWest.turn_left(), Dir::West);
    /// assert_eq!(Dir::North.turn_left(), Dir::NorthWest);
    /// ```
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
    ///
    /// # Example
    /// ```
    /// # use advent_of_tools::Dir;
    /// assert_eq!(Dir::North.turn_cardinal_left(), Dir::West);
    /// assert_eq!(Dir::East.turn_cardinal_left(), Dir::North);
    /// ```
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
    ///
    /// # Example
    /// ```
    /// # use advent_of_tools::Dir;
    /// assert_eq!(Dir::South.turn_cardinal_right(), Dir::West);
    /// assert_eq!(Dir::West.turn_cardinal_right(), Dir::North);
    /// ```
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

    /// Returns true for N, S, E and W.
    ///
    /// # Example
    /// ```
    /// # use advent_of_tools::{Dir, CARDINALS};
    /// for dir in CARDINALS {
    ///     assert!(dir.is_cardinal());
    /// }
    /// assert!(! Dir::NorthWest.is_cardinal());
    /// ```
    pub fn is_cardinal(&self) -> bool {
        use Dir::*;
        matches!(*self, North | South | East | West)
    }
}
