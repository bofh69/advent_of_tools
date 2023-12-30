// SPDX-FileCopyrightText: 2023 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

type Length = i32;

use super::dir::Dir;
use num::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Point<T = Length> {
    pub x: T,
    pub y: T,
}

impl<T: Signed + Copy> Point<T> {
    pub fn walk(self, dir: Dir) -> Self {
        match dir {
            Dir::None => Self {
                x: self.x,
                y: self.y,
            },
            Dir::North => Self {
                x: self.x,
                y: self.y - One::one(),
            },
            Dir::South => Self {
                x: self.x,
                y: self.y + One::one(),
            },
            Dir::East => Self {
                x: self.x + One::one(),
                y: self.y,
            },
            Dir::West => Self {
                x: self.x - One::one(),
                y: self.y,
            },
            Dir::NorthEast => Self {
                x: self.x + One::one(),
                y: self.y - One::one(),
            },
            Dir::NorthWest => Self {
                x: self.x - One::one(),
                y: self.y - One::one(),
            },
            Dir::SouthEast => Self {
                x: self.x + One::one(),
                y: self.y + One::one(),
            },
            Dir::SouthWest => Self {
                x: self.x - One::one(),
                y: self.y + One::one(),
            },
        }
    }

    pub fn manhattan_distance(&self, other: Self) -> T {
        T::abs(&(self.x - other.x)) + T::abs(&(self.y - other.y))
    }
}

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
