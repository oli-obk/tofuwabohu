use std::{
    cell::Cell,
    cmp::Ordering,
    fmt::Display,
    ops::*,
    rc::{Rc, Weak},
};

use crate::save::Save;

pub struct Sensor<T> {
    input: Rc<Cell<T>>,
}

impl<T: Copy> Sensor<T> {
    /// Just create a sensor, leaving reader creation to later.
    pub fn raw(val: T) -> Self {
        let input = Rc::new(Cell::new(val));
        Self { input }
    }
    /// Create a sensor and a reader at the same time
    pub fn new(val: T) -> (Self, Reader<T>) {
        let this = Self::raw(val);
        let reader = this.make_reader();
        (this, reader)
    }
    pub fn make_reader(&self) -> Reader<T> {
        let output = Rc::downgrade(&self.input);
        Reader { output }
    }
    pub fn set(&self, val: T) {
        self.input.set(val);
    }
    pub fn update(&self, f: impl FnOnce(&mut T)) {
        let mut val = self.get();
        f(&mut val);
        self.set(val);
    }
    pub fn modify(&self, f: impl FnOnce(T) -> T) -> T {
        let old = self.get();
        let new = f(old);
        self.set(new);
        new
    }
    pub fn get(&self) -> T {
        self.input.get()
    }
}

pub struct Reader<T> {
    output: Weak<Cell<T>>,
}

impl<T: Copy> Reader<T> {
    pub fn get(&self) -> Option<T> {
        self.output.upgrade().map(|o| o.get())
    }
}

impl<T: Copy + AddAssign> AddAssign<T> for Sensor<T> {
    fn add_assign(&mut self, rhs: T) {
        self.update(|val| *val += rhs)
    }
}

impl<T: Copy + MulAssign> MulAssign<T> for Sensor<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.update(|val| *val *= rhs)
    }
}

impl<T: Copy + SubAssign> SubAssign<T> for Sensor<T> {
    fn sub_assign(&mut self, rhs: T) {
        self.update(|val| *val -= rhs)
    }
}

impl<T: Copy + RemAssign> RemAssign<T> for Sensor<T> {
    fn rem_assign(&mut self, rhs: T) {
        self.update(|val| *val %= rhs)
    }
}

impl<T: Copy + PartialEq> PartialEq<T> for Sensor<T> {
    fn eq(&self, other: &T) -> bool {
        self.get().eq(other)
    }
}

impl<T: Copy + PartialOrd> PartialOrd<T> for Sensor<T> {
    fn partial_cmp(&self, other: &T) -> Option<Ordering> {
        self.get().partial_cmp(other)
    }
}

impl<T: Copy + Save> Save for Sensor<T> {
    fn save(&self, key: impl Display) {
        self.get().save(key)
    }

    fn load(&mut self, key: impl Display) {
        self.update(|val| val.load(key));
    }
}
