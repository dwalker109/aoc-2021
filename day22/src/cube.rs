use std::{
    cmp::{max, min},
    ops::RangeInclusive,
};

use super::Xyz;

pub trait Cuboid {
    fn new(a: &Xyz, b: &Xyz) -> Self;
    fn x(&self) -> &RangeInclusive<i32>;
    fn y(&self) -> &RangeInclusive<i32>;
    fn z(&self) -> &RangeInclusive<i32>;

    fn l(&self) -> usize {
        (self.x().start() - self.x().end()).abs() as usize + 1
    }

    fn w(&self) -> usize {
        (self.y().start() - self.y().end()).abs() as usize + 1
    }

    fn h(&self) -> usize {
        (self.z().start() - self.z().end()).abs() as usize + 1
    }

    fn area(&self) -> usize {
        self.l() * self.w() * self.h()
    }

    fn intersects(&self, other: &impl Cuboid) -> bool {
        self.x().start() <= other.x().end()
            && self.x().end() >= other.x().start()
            && self.y().start() <= other.y().end()
            && self.y().end() >= other.y().start()
            && self.z().start() <= other.z().end()
            && self.z().end() >= other.z().start()
    }

    fn overlap_bounds(&self, other: &impl Cuboid) -> Option<(Xyz, Xyz)> {
        if !self.intersects(other) {
            return None;
        }

        let (x, y, z) = (
            (max(*self.x().start(), *other.x().start())..=min(*self.x().end(), *other.x().end())),
            (max(*self.y().start(), *other.y().start())..=min(*self.y().end(), *other.y().end())),
            (max(*self.z().start(), *other.z().start())..=min(*self.z().end(), *other.z().end())),
        );

        Some((
            Xyz::new(*x.start(), *y.start(), *z.start()),
            Xyz::new(*x.end(), *y.end(), *z.end()),
        ))
    }

    fn make_adjustment_cuboid<T: Cuboid>(&self, other: &impl Cuboid) -> Option<T> {
        let (from, to) = self.overlap_bounds(other)?;

        Some(T::new(&from, &to))
    }
}

pub struct PosiCuboid(
    RangeInclusive<i32>,
    RangeInclusive<i32>,
    RangeInclusive<i32>,
);

impl Cuboid for PosiCuboid {
    fn new(a: &Xyz, b: &Xyz) -> Self {
        let x = a.x()..=b.x();
        let y = a.y()..=b.y();
        let z = a.z()..=b.z();

        Self(x, y, z)
    }

    fn x(&self) -> &RangeInclusive<i32> {
        &self.0
    }

    fn y(&self) -> &RangeInclusive<i32> {
        &self.1
    }

    fn z(&self) -> &RangeInclusive<i32> {
        &self.2
    }
}

pub struct AntiCuboid(
    RangeInclusive<i32>,
    RangeInclusive<i32>,
    RangeInclusive<i32>,
);

impl Cuboid for AntiCuboid {
    fn new(a: &Xyz, b: &Xyz) -> Self {
        let x = a.x()..=b.x();
        let y = a.y()..=b.y();
        let z = a.z()..=b.z();

        Self(x, y, z)
    }

    fn x(&self) -> &RangeInclusive<i32> {
        &self.0
    }

    fn y(&self) -> &RangeInclusive<i32> {
        &self.1
    }

    fn z(&self) -> &RangeInclusive<i32> {
        &self.2
    }
}
