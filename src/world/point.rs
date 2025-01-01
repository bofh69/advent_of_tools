// SPDX-FileCopyrightText: 2023 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

#![warn(missing_docs)]

type Length = i32;

use super::dir::Dir;
use num::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
/// Point is a 2D point in space.
pub struct Point<T = Length> {
    /// x is the position along the x-axis.
    pub x: T,
    /// y is the position along the y-axis.
    pub y: T,
}

impl<T: Signed + Copy> Point<T> {
    /// Walks one step in the given direction and returns the new Point.
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

    /// Calculates the manhattan distance (|x| + |y|) between this and another point.
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

impl std::ops::Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T> std::ops::Mul<T> for Point<T>
where
    T: std::ops::Mul<T, Output = T>,
    T: Copy,
{
    type Output = Self;

    /// Multiply the point with a number.
    ///
    /// # Example:
    /// ```
    /// # use advent_of_tools::*;
    /// let p = Point {x: -2, y: 3};
    /// assert_eq!(p * -2, Point {x: 4, y: -6});
    /// ```
    fn mul(self, other: T) -> Self::Output {
        Self {
            x: self.x.mul(other),
            y: self.y.mul(other),
        }
    }
}
