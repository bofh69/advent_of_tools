// SPDX-FileCopyrightText: 2023 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

type Length = i32;

pub trait LengthType:
    Signed
    + Num
    + Ord
    + std::ops::AddAssign
    + std::ops::SubAssign
    + Copy
    + std::fmt::Debug
    + ToPrimitive
    + std::hash::Hash
{
}

impl LengthType for i16 {}
impl LengthType for i32 {}
impl LengthType for i64 {}
impl LengthType for i128 {}

mod dir;
mod point;
pub use dir::Dir;
use num::*;
pub use point::Point;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct CostAndPoint<T: LengthType, U: Num>(U, Point<T>);

impl<T: LengthType + PartialOrd + Eq + PartialEq, U: Num + Ord + PartialOrd + Eq + PartialEq> Ord
    for CostAndPoint<T, U>
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.0.cmp(&self.0)
    }
}

impl<T: LengthType + PartialOrd + Eq + PartialEq, U: Num + Ord + PartialOrd + Eq + PartialEq>
    PartialOrd for CostAndPoint<T, U>
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.0.cmp(&self.0))
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Map<T: LengthType = Length>
where
    usize: TryFrom<T>,
    <usize as TryFrom<T>>::Error: std::fmt::Debug,
{
    data: Vec<u8>,
    width: T,
    height: T,
    has_border: bool,
}

pub struct MapIterator<'a, T: LengthType>
where
    usize: TryFrom<T>,
    <usize as TryFrom<T>>::Error: std::fmt::Debug,
{
    map: &'a Map<T>,
    pos: Point<T>,
}

impl<'a, T: LengthType> MapIterator<'a, T>
where
    usize: TryFrom<T>,
    <usize as TryFrom<T>>::Error: std::fmt::Debug,
{
    pub fn new(map: &'a Map<T>) -> Self {
        Self {
            map,
            pos: Point::<T> {
                x: Zero::zero(),
                y: Zero::zero(),
            },
        }
    }
}

impl<'a, T: LengthType> Iterator for MapIterator<'a, T>
where
    usize: TryFrom<T>,
    <usize as TryFrom<T>>::Error: std::fmt::Debug,
{
    type Item = (Point<T>, u8);

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos.x >= self.map.get_width() {
            self.pos.x = Zero::zero();
            self.pos.y += One::one();
        }
        if self.pos.y >= self.map.get_height() {
            None
        } else {
            let pos = self.pos;
            self.pos.x += One::one();
            Some((pos, self.map.get_at_unchecked(pos)))
        }
    }
}

pub struct MapNeighborIterator<'a, T: LengthType>
where
    usize: TryFrom<T>,
    <usize as TryFrom<T>>::Error: std::fmt::Debug,
{
    map: &'a Map<T>,
    pos: Point<T>,
    dir: Dir,
}

impl<'a, T: LengthType> MapNeighborIterator<'a, T>
where
    usize: TryFrom<T>,
    <usize as TryFrom<T>>::Error: std::fmt::Debug,
{
    pub fn new(map: &'a Map<T>, pos: Point<T>) -> Self {
        Self {
            map,
            pos,
            dir: Dir::North,
        }
    }
}

impl<'a, T: LengthType> Iterator for MapNeighborIterator<'a, T>
where
    usize: TryFrom<T>,
    <usize as TryFrom<T>>::Error: std::fmt::Debug,
{
    type Item = (Point<T>, Dir, u8);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.dir == Dir::None {
                return None;
            } else {
                let dir = self.dir;
                self.dir = self.dir.turn_right();
                if self.dir == Dir::North {
                    self.dir = Dir::None;
                }

                let pos = self.pos.walk(dir);
                if self.map.is_inside_map(pos) {
                    return Some((pos, dir, self.map.get_at_unchecked(pos)));
                }
            }
        }
    }
}

impl<T: LengthType> Map<T>
where
    usize: TryFrom<T>,
    <usize as TryFrom<T>>::Error: std::fmt::Debug,
{
    pub fn get_width(&self) -> T {
        self.width
    }

    pub fn get_height(&self) -> T {
        self.height
    }

    fn get_index_for(&self, pos: Point<T>) -> usize {
        usize::try_from(pos.x + pos.y * self.width).expect("Positive index")
    }

    pub fn get_at_unchecked(&self, pos: Point<T>) -> u8 {
        self.data[self.get_index_for(pos)]
    }

    pub fn get_at(&self, pos: Point<T>) -> Option<u8> {
        if self.is_inside_map(pos) {
            Some(self.data[self.get_index_for(pos)])
        } else {
            None
        }
    }

    pub fn set_at(&mut self, pos: Point<T>, val: u8) {
        let index = self.get_index_for(pos);
        self.data[index] = val
    }

    pub fn new(width: T, height: T) -> Self {
        let mut data =
            Vec::with_capacity(usize::try_from(width * height).expect("Positive number"));
        data.resize_with(
            usize::try_from(width * height).expect("Positive size"),
            || b'.',
        );
        Self {
            data,
            width,
            height,
            has_border: false,
        }
    }

    pub fn add_boarder(&mut self, val: u8) {
        for y in range(Zero::zero(), self.get_height()) {
            self.set_at(Point { x: Zero::zero(), y }, val);
            self.set_at(
                Point {
                    x: self.get_width() - One::one(),
                    y,
                },
                val,
            );
        }
        for x in range(Zero::zero(), self.get_width()) {
            self.set_at(Point { x, y: Zero::zero() }, val);
            self.set_at(
                Point {
                    x,
                    y: self.get_height() - One::one(),
                },
                val,
            );
        }
    }

    pub fn from_string(s: &str) -> Self
    where
        T: TryFrom<usize>,
        <T as TryFrom<usize>>::Error: std::fmt::Debug,
    {
        let height = s.lines().count();
        let width = s.lines().next().expect("At least one line").len();
        let mut data = Vec::with_capacity(height * width);
        for c in s.chars() {
            if c != '\n' {
                data.push(u8::try_from(c).expect("Ascii char"));
            }
        }

        let width = T::try_from(width).expect("Positive width");
        let height = T::try_from(height).expect("Positive height");
        Self {
            data,
            width,
            height,
            has_border: false,
        }
    }

    pub fn from_string_with_border(s: &str) -> Self
    where
        T: TryFrom<usize>,
        <T as TryFrom<usize>>::Error: std::fmt::Debug,
    {
        let height = s.lines().count() + 2;
        let width = s.lines().next().expect("At least one line").len() + 2;
        let mut data = Vec::with_capacity(height * width);
        data.push(b'+');
        for _x in 0..width - 2 {
            data.push(b'-');
        }
        data.push(b'+');
        data.push(b'|');
        for c in s.chars() {
            if c != '\n' {
                data.push(u8::try_from(c).expect("Ascii char"));
            } else {
                data.push(b'|');
                data.push(b'|');
            }
        }
        data.push(b'|');
        data.push(b'+');
        for _x in 0..width - 2 {
            data.push(b'-');
        }
        data.push(b'+');

        let width = T::try_from(width).expect("Positive width");
        let height = T::try_from(height).expect("Positive height");
        Self {
            data,
            width,
            height,
            has_border: true,
        }
    }

    pub fn print_with_overlay<F>(&self, mut f: F)
    where
        F: FnMut(Point<T>, u8) -> Option<u8>,
    {
        for y in range(Zero::zero(), self.height) {
            for x in range(Zero::zero(), self.width) {
                let pos = Point { x, y };
                let mut c = self.get_at_unchecked(pos);
                if let Some(new_c) = f(pos, c) {
                    c = new_c;
                }
                print!("{}", char::from(c));
            }
            println!();
        }
    }

    pub fn print(&self) {
        self.print_with_overlay(|_, _| None);
    }

    pub fn iter(&self) -> MapIterator<T> {
        MapIterator::new(self)
    }

    pub fn neighbors(&self, pos: Point<T>) -> MapNeighborIterator<T> {
        MapNeighborIterator::new(self, pos)
    }

    pub fn transform_area<F>(&mut self, from: Point<T>, to: Point<T>, mut f: F) -> bool
    where
        F: FnMut(&Self, Point<T>, u8) -> u8,
    {
        let mut new_map = Map::new(self.width, self.height);
        let mut any_change = false;
        for (pos, c) in self.iter() {
            if pos.x >= from.x && pos.y >= from.y && pos.x < to.x && pos.y < to.y {
                let new_c = f(self, pos, c);
                if new_c != c {
                    any_change = true;
                }
                new_map.set_at(pos, new_c);
            }
        }
        for (pos, c) in new_map.iter() {
            if pos.x >= from.x && pos.y >= from.y && pos.x < to.x && pos.y < to.y {
                self.set_at(pos, c);
            }
        }
        any_change
    }

    pub fn transform<F>(&mut self, f: F) -> bool
    where
        F: FnMut(&Self, Point<T>, u8) -> u8,
    {
        if self.has_border {
            self.transform_area(
                Point::<T> {
                    x: One::one(),
                    y: One::one(),
                },
                Point::<T> {
                    x: self.width - One::one(),
                    y: self.height - One::one(),
                },
                f,
            )
        } else {
            self.transform_area(
                Point::<T> {
                    x: Zero::zero(),
                    y: Zero::zero(),
                },
                Point::<T> {
                    x: self.width,
                    y: self.height,
                },
                f,
            )
        }
    }

    pub fn is_inside_map(&self, pos: Point<T>) -> bool {
        pos.x >= Zero::zero()
            && pos.y >= Zero::zero()
            && pos.x < self.get_width()
            && pos.y < self.get_height()
    }

    /// moves pos in dir
    ///
    /// It stops when the next point in that direction is outside of the map or causes f to return
    /// false
    pub fn walk_until<F>(&self, pos: Point<T>, dir: Dir, mut f: F) -> Point<T>
    where
        F: FnMut(Point<T>, u8) -> bool,
    {
        let mut pos = pos;
        loop {
            let new_pos = pos.walk(dir);
            if !self.is_inside_map(new_pos) || f(new_pos, self.get_at_unchecked(new_pos)) {
                break;
            }
            pos = new_pos;
        }
        pos
    }

    /// flood fill the map from point pos with val
    /// Only fills north, south, east and west of each position
    pub fn flood_cardinal(&mut self, pos: Point<T>, empty: u8, val: u8) {
        if self.get_at_unchecked(pos) != empty {
            // Nothing to fill here
            return;
        }
        let min_pos = self.walk_until(pos, Dir::West, |_, c| c != empty);
        let max_pos = self.walk_until(pos, Dir::East, |_, c| c != empty);

        let mut pos = min_pos;
        while pos.x <= max_pos.x {
            self.set_at(pos, val);
            pos = pos.walk(Dir::East);
        }
        pos = min_pos;
        while pos.x <= max_pos.x {
            pos.y -= One::one();
            if pos.y > Zero::zero() {
                self.flood_cardinal(pos, empty, val);
            }
            pos.y = pos.y + One::one() + One::one();
            if pos.y < self.get_height() {
                self.flood_cardinal(pos, empty, val);
            }
            pos.y -= One::one();
            pos = pos.walk(Dir::East);
        }
    }

    pub fn flood_cardinal_with<F>(&mut self, pos: Point<T>, f: &mut F)
    where
        F: FnMut(Point<T>, u8) -> Option<u8>,
    {
        if f(pos, self.get_at_unchecked(pos)).is_none() {
            // Nothing to fill here
            return;
        }
        let min_pos = self.walk_until(pos, Dir::West, |pos, c| f(pos, c).is_none());
        let max_pos = self.walk_until(pos, Dir::East, |pos, c| f(pos, c).is_none());

        let mut pos = min_pos;
        while pos.x <= max_pos.x {
            let val = f(pos, self.get_at_unchecked(pos)).expect("value");
            self.set_at(pos, val);
            pos = pos.walk(Dir::East);
        }
        pos = min_pos;
        while pos.x <= max_pos.x {
            pos.y -= One::one();
            if pos.y >= Zero::zero() {
                self.flood_cardinal_with(pos, f);
            }
            pos.y = pos.y + One::one() + One::one();
            if pos.y < self.get_height() {
                self.flood_cardinal_with(pos, f);
            }
            pos.y -= One::one();
            pos = pos.walk(Dir::East);
        }
    }

    pub fn find(&self, needle: u8) -> Vec<Point<T>> {
        self.iter()
            .filter_map(|(p, c)| if c == needle { Some(p) } else { None })
            .collect()
    }

    pub fn bfs<F, U>(&self, from: Point<T>, to: Point<T>, f: &mut F) -> U
    where
        F: FnMut(&Self, Point<T>, Dir, u8) -> Option<U>,
        U: Num + Ord + Copy + std::fmt::Debug,
    {
        // TODO: Give path instead?
        let mut expanded = std::collections::HashMap::new();
        let mut to_expand = std::collections::BinaryHeap::new();
        to_expand.push(CostAndPoint(Zero::zero(), from));
        while let Some(CostAndPoint(cost, pos)) = to_expand.pop() {
            if to == pos {
                return cost;
            }
            if let Some(old_cost) = expanded.get_mut(&pos) {
                if *old_cost <= cost {
                    continue;
                } else {
                    *old_cost = cost;
                }
            } else {
                expanded.insert(pos, cost);
            }
            for (new_cost, pos) in self
                .neighbors(pos)
                .filter_map(|(pos, dir, c)| f(self, pos, dir, c).map(|step| (step + cost, pos)))
            {
                to_expand.push(CostAndPoint(new_cost, pos));
            }
        }
        Zero::zero()
    }
}
