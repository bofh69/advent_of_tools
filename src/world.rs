// SPDX-FileCopyrightText: 2024 Sebastian Andersson <sebastian@bittr.nu>
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

impl LengthType for i8 {}
impl LengthType for i16 {}
impl LengthType for i32 {}
impl LengthType for i64 {}
impl LengthType for i128 {}

mod dir;
mod point;
pub use dir::*;
use num::*;
pub use point::Point;

/// A struct to keep a Point together with a number.
///
/// Used to keep track of Points and a cost, useful when storing them in BinaryHeap etc
/// for search algorithms.
///
/// `Ord` and `PartialOrd` are implemented for it. Only the cost field is compared,
/// and the lowest cost is the greatest.
///
/// # Example:
/// ```
/// # use advent_of_tools::*;
/// let point = Point {x: 1, y: 3};
/// let pc1 = PointAndCost{cost: 3, point: point * 2};
/// let pc2 = PointAndCost{cost: 7, point: point};
/// assert!(pc2.cmp(&pc1) == std::cmp::Ordering::Less);
/// ```
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct PointAndCost<T: LengthType, U: Num> {
    pub cost: U,
    pub point: Point<T>,
}

impl<T: LengthType + PartialOrd + Eq + PartialEq, U: Num + Ord + PartialOrd + Eq + PartialEq> Ord
    for PointAndCost<T, U>
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl<T: LengthType + PartialOrd + Eq + PartialEq, U: Num + Ord + PartialOrd + Eq + PartialEq>
    PartialOrd for PointAndCost<T, U>
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.cost.cmp(&self.cost))
    }
}

/// Map stores ASCII text maps.
///
/// # Example
///
/// ```
/// # use advent_of_tools::*;
///
/// let map = "####\n@.#.\n.#..\n";
/// let map : Map<i32> = Map::from_string(map);
/// assert_eq!(map.get_width(), 4);
/// assert_eq!(map.get_height(), 3);
/// assert_eq!(map.find(b'@').len(), 1);
/// ```
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

impl<T: LengthType> Iterator for MapIterator<'_, T>
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

impl<T: LengthType> Iterator for MapNeighborIterator<'_, T>
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
    /// Get the width of Map.
    ///
    /// The width includes the border.
    ///
    /// # Example:
    /// ```
    /// # use advent_of_tools::*;
    /// let map : Map<i16> = Map::from_string_with_border("aa");
    ///
    /// assert_eq!(map.get_width(), 4);
    /// ```
    pub fn get_width(&self) -> T {
        self.width
    }

    /// Get the height of Map.
    ///
    /// The height includes the border.
    ///
    /// # Example:
    /// ```
    /// # use advent_of_tools::*;
    /// let map = Map::<i32>::from_string_with_border("aa");
    ///
    /// assert_eq!(map.get_height(), 3);
    /// ```
    pub fn get_height(&self) -> T {
        self.height
    }

    fn get_index_for(&self, pos: Point<T>) -> usize {
        usize::try_from(pos.x + pos.y * self.width).expect("Positive index")
    }

    /// Get the tile at a valid position.
    ///
    /// # Example
    /// ```
    /// # use advent_of_tools::*;
    /// let map = Map::from_string("ab\ncd\n");
    ///
    /// assert_eq!(map.get_at_unchecked(Point{x: 0, y: 1}), b'c');
    /// ```
    pub fn get_at_unchecked(&self, pos: Point<T>) -> u8 {
        self.data[self.get_index_for(pos)]
    }

    /// Get the tile at a position.
    ///
    /// Returns None if the position isn't valid.
    ///
    /// # Example
    /// ```
    /// # use advent_of_tools::*;
    /// let map = Map::from_string("ab\ncd\n");
    ///
    /// assert_eq!(map.get_at(Point{x: 0, y: 2}), None);
    /// ```
    pub fn get_at(&self, pos: Point<T>) -> Option<u8> {
        if self.is_inside_map(pos) {
            Some(self.data[self.get_index_for(pos)])
        } else {
            None
        }
    }

    /// Set the tile at a position.
    ///
    /// The position is assumed to be valid.
    /// It can set tiles in the border as well.
    ///
    /// # Example
    /// ```
    /// # use advent_of_tools::*;
    /// let mut map = Map::from_string_with_border("ab\ncd\n");
    ///
    /// map.set_at(Point{x: 0, y: 2}, b'!');
    /// assert_eq!(map.get_at(Point{x: 0, y: 2}), Some(b'!'));
    /// ```
    pub fn set_at(&mut self, pos: Point<T>, val: u8) {
        let index = self.get_index_for(pos);
        self.data[index] = val
    }

    /// Create a new map of given dimensions.
    ///
    /// It is filled with b'.' tiles.
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

    /// Add a border around the map.
    ///
    /// The map will grow in size and all tiles within the map will have a new position,
    /// one higher in x and y.
    ///
    /// The border will be filled with the given tile.
    ///
    /// # Example
    /// ```
    /// # use advent_of_tools::*;
    /// let mut map = Map::new(10, 20);
    ///
    /// assert_eq!(map.get_at(Point{x: 0, y: 2}), Some(b'.'));
    /// map.add_border(b'*');
    /// assert_eq!(map.get_at(Point{x: 0, y: 2}), Some(b'*'));
    /// ```
    pub fn add_border(&mut self, tile: u8) {
        for y in range(Zero::zero(), self.get_height()) {
            self.set_at(Point { x: Zero::zero(), y }, tile);
            self.set_at(
                Point {
                    x: self.get_width() - One::one(),
                    y,
                },
                tile,
            );
        }
        for x in range(Zero::zero(), self.get_width()) {
            self.set_at(Point { x, y: Zero::zero() }, tile);
            self.set_at(
                Point {
                    x,
                    y: self.get_height() - One::one(),
                },
                tile,
            );
        }
    }

    /// Create a Map from a string.
    ///
    /// The string is assumed to end each line with a single linefeed.
    /// Each line becomes a row in the map.
    /// All lines are assumed to be of equal width.
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

    /// Create a Map from a string with a border.
    ///
    /// The string is assumed to end each line with a single linefeed.
    /// Each line becomes a row in the map.
    /// All lines are assumed to be of equal width.
    ///
    /// The map gets a border around it with b'+' in the corners,
    /// b'-' along the top and bottom edges and b'|' along the sides.
    ///
    /// # Example
    /// ```
    /// # use advent_of_tools::*;
    /// let mut map = Map::from_string_with_border("abc\ndef");
    ///
    /// assert_eq!(map.get_width(), 5);
    /// assert_eq!(map.get_height(), 4);
    ///
    /// assert_eq!(map.get_at_unchecked(Point{x: 0, y: 2}), b'|');
    /// assert_eq!(map.get_at(Point{x: 0, y: 0}), Some(b'+'));
    /// assert_eq!(map.get_at(Point{x: 2, y: 0}), Some(b'-'));
    /// ```
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

    /// Print the map to stdout with an overlay provided by f.
    ///
    /// For every tile in the map, the f function will be called
    /// with the position and the tile at that position.
    ///
    /// The returned u8 will be the tile that is printed.
    ///
    /// # Example:
    /// ```
    /// # use advent_of_tools::*;
    /// let mut map = Map::<i32>::from_string_with_border("abc\ndef");
    ///
    /// map.print_with_overlay(|pos, tile| {
    ///   if pos.y == 1 {
    ///      b'='
    ///   } else {
    ///      tile
    ///   }
    /// })
    /// ```
    pub fn print_with_overlay<F>(&self, mut f: F)
    where
        F: FnMut(Point<T>, u8) -> u8,
    {
        for y in range(Zero::zero(), self.height) {
            for x in range(Zero::zero(), self.width) {
                let pos = Point { x, y };
                let mut c = self.get_at_unchecked(pos);
                c = f(pos, c);
                print!("{}", char::from(c));
            }
            println!();
        }
    }

    /// Print the map to stdout.
    ///
    /// # Example:
    /// ```
    /// # use advent_of_tools::*;
    /// let mut map = Map::<i32>::from_string_with_border("abc\ndef");
    ///
    /// map.print();
    /// ```
    pub fn print(&self) {
        self.print_with_overlay(|_, tile| tile);
    }

    /// Iterate over all positions in the map.
    ///
    /// The iterator returns a tuple of the position's Point and the tile.
    ///
    /// # Example:
    /// ```
    /// # use advent_of_tools::*;
    /// let mut map = Map::<i32>::from_string("abc\ndef");
    ///
    /// assert_eq!(map.iter().filter(|&(_pos, tile)| tile == b'a').count(), 1);
    /// ```
    pub fn iter(&self) -> MapIterator<T> {
        MapIterator::new(self)
    }

    /// Iterate over all neigbors to a position in the map.
    ///
    /// The iterator returns a tuple of the neighbor's Point,
    /// the direction to it and the tile.
    /// All valid of the 8 neighbors are given.
    ///
    /// # Example:
    /// ```
    /// # use advent_of_tools::*;
    /// let mut map = Map::<i32>::from_string("abc\ndef");
    ///
    /// let point = Point {x: 0, y: 0};
    /// assert_eq!(map.neighbors(point).count(), 3);
    /// for (pos, dir, tile) in map.neighbors(point) {
    ///   print!("{pos:?}, {dir}, {tile}");
    /// }
    /// ```
    /// This outputs:
    ///
    /// ```text
    /// Point { x: 1, y: 0}, East, b
    /// Point { x: 0, y: 1}, South, d
    /// Point { x: 1, y: 1}, SouthEast, e
    /// ```
    pub fn neighbors(&self, pos: Point<T>) -> MapNeighborIterator<T> {
        MapNeighborIterator::new(self, pos)
    }

    /// Update all tiles with a given area.
    ///
    /// `from` is the top left corner of the area,
    /// `to` is the bottom right corner of the area.
    ///
    /// `f` is a function that gets called with the map, the position and the tile
    /// and its returned tile will set the new value.
    ///
    /// The map is double buffered while being transformed so old values
    /// are returned if `f` asks map about other tiles.
    ///
    /// The function returns true if any tiles were changed.
    ///
    /// # Example of filling an area with transform:
    /// ```
    /// # use advent_of_tools::*;
    /// let mut map = Map::<i32>::new(10, 10);
    ///
    /// let retval = map.transform_area(Point{x: 2, y: 2}, Point{x: 7, y: 6}, |_map, _pos, _tile| b'*');
    /// assert!(retval);
    /// assert_eq!(map.find(b'*').len(), 5*4);
    /// ```
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

    /// Update all tiles.
    ///
    /// The function works like `transform_area`, except the whole map
    /// will be affected.
    ///
    /// `f` is a function that gets called with the map, the position and the tile
    /// and its returned tile will set the new value.
    ///
    /// The map is double buffered while being transformed so old values
    /// are returned if `f` asks map about other tiles.
    ///
    /// The function returns true if any tiles were changed.
    ///
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

    /// Check if given position is within the Map's valid area.
    ///
    /// If the map has a border, its positions are also valid.
    pub fn is_inside_map(&self, pos: Point<T>) -> bool {
        pos.x >= Zero::zero()
            && pos.y >= Zero::zero()
            && pos.x < self.get_width()
            && pos.y < self.get_height()
    }

    /// moves pos in the given direction
    ///
    /// It stops when the next point in that direction is outside of the map or causes `f` to return
    /// false.
    ///
    /// # Example:
    /// ```
    /// # use advent_of_tools::*;
    /// let mut map = Map::<i32>::new(10, 10);
    ///
    /// let position = map.walk_until(Point{x: 5, y: 5}, Dir::West, |_pos, tile| tile != b'.');
    /// assert_eq!(position, Point {x: 0, y: 5});
    ///
    /// map.add_border(b'#');
    /// let position = map.walk_until(Point{x: 5, y: 5}, Dir::West, |_pos, tile| tile != b'.');
    /// assert_eq!(position, Point {x: 1, y: 5});
    /// ```
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

    /// flood fill the map from point `pos` with `tile`.
    ///
    /// Only fills via the cardinal directions from each position.
    ///
    /// # Example:
    /// ```
    /// # use advent_of_tools::*;
    ///
    /// // The map is:
    /// // ###.#
    /// // #.#..
    /// // #.###
    /// let mut map = Map::<i32>::from_string("###.#\n#.#..\n#.###\n");
    ///
    /// map.flood_cardinal(Point{x: 0, y: 2}, b'#', b'!');
    ///
    /// assert_eq!(map.get_at_unchecked(Point{x: 4, y: 2}), b'!');
    /// assert_eq!(map.get_at_unchecked(Point{x: 4, y: 0}), b'#');
    /// ```
    pub fn flood_cardinal(&mut self, pos: Point<T>, empty: u8, tile: u8) {
        if self.get_at_unchecked(pos) != empty {
            // Nothing to fill here
            return;
        }
        let min_pos = self.walk_until(pos, Dir::West, |_, c| c != empty);
        let max_pos = self.walk_until(pos, Dir::East, |_, c| c != empty);

        let mut pos = min_pos;
        while pos.x <= max_pos.x {
            self.set_at(pos, tile);
            pos = pos.walk(Dir::East);
        }
        pos = min_pos;
        while pos.x <= max_pos.x {
            pos.y -= One::one();
            if pos.y >= Zero::zero() {
                self.flood_cardinal(pos, empty, tile);
            }
            pos.y = pos.y + One::one() + One::one();
            if pos.y < self.get_height() {
                self.flood_cardinal(pos, empty, tile);
            }
            pos.y -= One::one();
            pos = pos.walk(Dir::East);
        }
    }

    /// flood fill the map from point `pos`.
    ///
    /// `is_ok_f` says if it is ok to fill the position,
    /// `tile_f` says what it should be filled with.
    ///
    /// Only fills via the cardinal directions from each position.
    ///
    /// # Example:
    /// ```
    /// # use advent_of_tools::*;
    ///
    /// // The map is:
    /// // ###.#
    /// // #.#..
    /// // #.###
    /// let mut map = Map::<i32>::from_string("###.#\n#.#..\n#.###\n");
    ///
    /// map.flood_cardinal_with(Point{x: 0, y: 2}, &mut |_pos, t| t == b'#', &mut |_pos, _tile| b'!');
    ///
    /// assert_eq!(map.get_at_unchecked(Point{x: 4, y: 2}), b'!');
    /// assert_eq!(map.get_at_unchecked(Point{x: 4, y: 0}), b'#');
    /// ```
    pub fn flood_cardinal_with<O, F>(&mut self, pos: Point<T>, is_ok_f: &mut O, tile_f: &mut F)
    where
        O: FnMut(Point<T>, u8) -> bool,
        F: FnMut(Point<T>, u8) -> u8,
    {
        if !is_ok_f(pos, self.get_at_unchecked(pos)) {
            // Nothing to fill here
            return;
        }
        let min_pos = self.walk_until(pos, Dir::West, |pos, c| !is_ok_f(pos, c));
        let max_pos = self.walk_until(pos, Dir::East, |pos, c| !is_ok_f(pos, c));

        let mut pos = min_pos;
        while pos.x <= max_pos.x {
            let val = tile_f(pos, self.get_at_unchecked(pos));
            self.set_at(pos, val);
            pos = pos.walk(Dir::East);
        }
        pos = min_pos;
        while pos.x <= max_pos.x {
            pos.y -= One::one();
            if pos.y >= Zero::zero() {
                self.flood_cardinal_with(pos, is_ok_f, tile_f);
            }
            pos.y = pos.y + One::one() + One::one();
            if pos.y < self.get_height() {
                self.flood_cardinal_with(pos, is_ok_f, tile_f);
            }
            pos.y -= One::one();
            pos = pos.walk(Dir::East);
        }
    }

    /// Finds all tiles matching `needle`.
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
        to_expand.push(PointAndCost {
            cost: Zero::zero(),
            point: from,
        });
        while let Some(PointAndCost { cost, point: pos }) = to_expand.pop() {
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
                to_expand.push(PointAndCost {
                    cost: new_cost,
                    point: pos,
                });
            }
        }
        Zero::zero()
    }
}

impl<'a, T: LengthType> IntoIterator for &'a Map<T>
where
    usize: TryFrom<T>,
    <usize as TryFrom<T>>::Error: std::fmt::Debug,
{
    type Item = (Point<T>, u8);
    type IntoIter = MapIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_into_iter() {
        let map = super::Map::new(2, 3);
        let mut count = 0;
        for (_p, _c) in &map {
            count += 1;
        }
        assert_eq!(count, 6);
    }
}
