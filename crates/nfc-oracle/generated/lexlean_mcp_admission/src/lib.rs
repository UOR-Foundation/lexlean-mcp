#![forbid(unsafe_code)]
use core::cmp::Ordering;
use core::sync::atomic::{AtomicU64, Ordering as Memory};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Overflow;
pub type R<T> = Result<T, Overflow>;

static WORK: AtomicU64 = AtomicU64::new(0);
pub fn tick(units: u64) { WORK.fetch_add(units, Memory::Relaxed); }
pub fn work() -> u64 { WORK.load(Memory::Relaxed) }


pub trait Same { fn same(&self, other: &Self) -> bool; }
pub trait Key { fn key(&self, other: &Self) -> Ordering; }
macro_rules! scalar {
    ($($t:ty),*) => { $(
        impl Same for $t { fn same(&self, other: &Self) -> bool { self == other } }
        impl Key for $t { fn key(&self, other: &Self) -> Ordering { self.cmp(other) } }
    )* };
}
scalar!(bool, u8, u16, u32, u64, i8, i16, i32, i64);
impl Same for Ordering { fn same(&self, other: &Self) -> bool { self == other } }
impl<A: Key, B: Key> Key for (A, B) {
    fn key(&self, other: &Self) -> Ordering {
        match self.0.key(&other.0) { Ordering::Equal => self.1.key(&other.1), decided => decided }
    }
}
pub fn equal<T: Same>(a: T, b: T) -> bool { a.same(&b) }
pub fn compare<T: Key>(a: T, b: T) -> Ordering { a.key(&b) }


pub fn nat_add(p0: u64, p1: u64) -> R<u64> {
    match u64::checked_add(p0, p1) {
        Some(v0) => {
            Ok(v0)
        }
        None => {
            Err::<u64, Overflow>(Overflow)
        }
    }
}

pub fn nat_sub(p0: u64, p1: u64) -> u64 {
    u64::saturating_sub(p0, p1)
}

pub fn nat_mul(p0: u64, p1: u64) -> R<u64> {
    match u64::checked_mul(p0, p1) {
        Some(v0) => {
            Ok(v0)
        }
        None => {
            Err::<u64, Overflow>(Overflow)
        }
    }
}

pub fn nat_quot(p0: u64, p1: u64, p2: u64) -> u64 {
    u64::checked_div(p0, p1).unwrap_or(p2)
}

pub fn nat_rem(p0: u64, p1: u64, p2: u64) -> u64 {
    u64::checked_rem(p0, p1).unwrap_or(p2)
}

pub fn nat_eq(p0: u64, p1: u64) -> bool {
    core::cmp::PartialEq::eq(&p0, &p1)
}

pub fn nat_le(p0: u64, p1: u64) -> bool {
    core::cmp::PartialOrd::le(&p0, &p1)
}

pub fn nat_lt(p0: u64, p1: u64) -> bool {
    core::cmp::PartialOrd::lt(&p0, &p1)
}

pub fn nat_succ(p0: u64) -> R<u64> {
    match u64::checked_add(p0, 1u64) {
        Some(v0) => {
            Ok(v0)
        }
        None => {
            Err::<u64, Overflow>(Overflow)
        }
    }
}

pub fn int_add(p0: i64, p1: i64) -> R<i64> {
    match i64::checked_add(p0, p1) {
        Some(v0) => {
            Ok(v0)
        }
        None => {
            Err::<i64, Overflow>(Overflow)
        }
    }
}

pub fn int_sub(p0: i64, p1: i64) -> R<i64> {
    match i64::checked_sub(p0, p1) {
        Some(v0) => {
            Ok(v0)
        }
        None => {
            Err::<i64, Overflow>(Overflow)
        }
    }
}

pub fn int_mul(p0: i64, p1: i64) -> R<i64> {
    match i64::checked_mul(p0, p1) {
        Some(v0) => {
            Ok(v0)
        }
        None => {
            Err::<i64, Overflow>(Overflow)
        }
    }
}

pub fn int_neg(p0: i64) -> R<i64> {
    match i64::checked_neg(p0) {
        Some(v0) => {
            Ok(v0)
        }
        None => {
            Err::<i64, Overflow>(Overflow)
        }
    }
}

pub fn int_quot(p0: i64, p1: i64, p2: i64) -> R<i64> {
    { let value = { let (a, b, z): (i64, i64, i64) = (p0, p1, p2); if b == 0 { Some(z) } else { a.checked_div(b) } }; match value {
        Some(v0) => {
            Ok(v0)
        }
        None => {
            Err::<i64, Overflow>(Overflow)
        }
    } }
}

pub fn int_rem(p0: i64, p1: i64, p2: i64) -> i64 {
    { let (a, b, z): (i64, i64, i64) = (p0, p1, p2); if b == 0 { z } else { a.wrapping_rem(b) } }
}

pub fn bool_not(p0: bool) -> bool {
    !p0
}

pub fn bool_and(p0: bool, p1: bool) -> bool {
    if p0 {
        p1
    } else {
        false
    }
}

pub fn bool_or(p0: bool, p1: bool) -> bool {
    if p0 {
        true
    } else {
        p1
    }
}

pub mod fixed_u8 {
    pub fn checked_add(p0: u8, p1: u8) -> Option<u8> {
        u8::checked_add(p0, p1)
    }

    pub fn checked_sub(p0: u8, p1: u8) -> Option<u8> {
        u8::checked_sub(p0, p1)
    }

    pub fn checked_mul(p0: u8, p1: u8) -> Option<u8> {
        u8::checked_mul(p0, p1)
    }

    pub fn checked_quot(p0: u8, p1: u8) -> Option<u8> {
        u8::checked_div(p0, p1)
    }

    pub fn bit_and(p0: u8, p1: u8) -> u8 {
        core::ops::BitAnd::bitand(p0, p1)
    }

    pub fn bit_or(p0: u8, p1: u8) -> u8 {
        core::ops::BitOr::bitor(p0, p1)
    }

    pub fn bit_xor(p0: u8, p1: u8) -> u8 {
        core::ops::BitXor::bitxor(p0, p1)
    }

    pub fn bit_not(p0: u8) -> u8 {
        core::ops::Not::not(p0)
    }

    pub fn shift_left(p0: u8, p1: u32) -> Option<u8> {
        u8::checked_shl(p0, p1)
    }

    pub fn shift_right(p0: u8, p1: u32) -> Option<u8> {
        u8::checked_shr(p0, p1)
    }

    pub fn convert(p0: i128) -> Option<u8> {
        u8::try_from(p0).ok()
    }
}

pub mod fixed_u16 {
    pub fn checked_add(p0: u16, p1: u16) -> Option<u16> {
        u16::checked_add(p0, p1)
    }

    pub fn checked_sub(p0: u16, p1: u16) -> Option<u16> {
        u16::checked_sub(p0, p1)
    }

    pub fn checked_mul(p0: u16, p1: u16) -> Option<u16> {
        u16::checked_mul(p0, p1)
    }

    pub fn checked_quot(p0: u16, p1: u16) -> Option<u16> {
        u16::checked_div(p0, p1)
    }

    pub fn bit_and(p0: u16, p1: u16) -> u16 {
        core::ops::BitAnd::bitand(p0, p1)
    }

    pub fn bit_or(p0: u16, p1: u16) -> u16 {
        core::ops::BitOr::bitor(p0, p1)
    }

    pub fn bit_xor(p0: u16, p1: u16) -> u16 {
        core::ops::BitXor::bitxor(p0, p1)
    }

    pub fn bit_not(p0: u16) -> u16 {
        core::ops::Not::not(p0)
    }

    pub fn shift_left(p0: u16, p1: u32) -> Option<u16> {
        u16::checked_shl(p0, p1)
    }

    pub fn shift_right(p0: u16, p1: u32) -> Option<u16> {
        u16::checked_shr(p0, p1)
    }

    pub fn convert(p0: i128) -> Option<u16> {
        u16::try_from(p0).ok()
    }
}

pub mod fixed_u32 {
    pub fn checked_add(p0: u32, p1: u32) -> Option<u32> {
        u32::checked_add(p0, p1)
    }

    pub fn checked_sub(p0: u32, p1: u32) -> Option<u32> {
        u32::checked_sub(p0, p1)
    }

    pub fn checked_mul(p0: u32, p1: u32) -> Option<u32> {
        u32::checked_mul(p0, p1)
    }

    pub fn checked_quot(p0: u32, p1: u32) -> Option<u32> {
        u32::checked_div(p0, p1)
    }

    pub fn bit_and(p0: u32, p1: u32) -> u32 {
        core::ops::BitAnd::bitand(p0, p1)
    }

    pub fn bit_or(p0: u32, p1: u32) -> u32 {
        core::ops::BitOr::bitor(p0, p1)
    }

    pub fn bit_xor(p0: u32, p1: u32) -> u32 {
        core::ops::BitXor::bitxor(p0, p1)
    }

    pub fn bit_not(p0: u32) -> u32 {
        core::ops::Not::not(p0)
    }

    pub fn shift_left(p0: u32, p1: u32) -> Option<u32> {
        u32::checked_shl(p0, p1)
    }

    pub fn shift_right(p0: u32, p1: u32) -> Option<u32> {
        u32::checked_shr(p0, p1)
    }

    pub fn convert(p0: i128) -> Option<u32> {
        u32::try_from(p0).ok()
    }
}

pub mod fixed_u64 {
    pub fn checked_add(p0: u64, p1: u64) -> Option<u64> {
        u64::checked_add(p0, p1)
    }

    pub fn checked_sub(p0: u64, p1: u64) -> Option<u64> {
        u64::checked_sub(p0, p1)
    }

    pub fn checked_mul(p0: u64, p1: u64) -> Option<u64> {
        u64::checked_mul(p0, p1)
    }

    pub fn checked_quot(p0: u64, p1: u64) -> Option<u64> {
        u64::checked_div(p0, p1)
    }

    pub fn bit_and(p0: u64, p1: u64) -> u64 {
        core::ops::BitAnd::bitand(p0, p1)
    }

    pub fn bit_or(p0: u64, p1: u64) -> u64 {
        core::ops::BitOr::bitor(p0, p1)
    }

    pub fn bit_xor(p0: u64, p1: u64) -> u64 {
        core::ops::BitXor::bitxor(p0, p1)
    }

    pub fn bit_not(p0: u64) -> u64 {
        core::ops::Not::not(p0)
    }

    pub fn shift_left(p0: u64, p1: u32) -> Option<u64> {
        u64::checked_shl(p0, p1)
    }

    pub fn shift_right(p0: u64, p1: u32) -> Option<u64> {
        u64::checked_shr(p0, p1)
    }

    pub fn convert(p0: i128) -> Option<u64> {
        u64::try_from(p0).ok()
    }
}

pub mod fixed_i8 {
    pub fn checked_add(p0: i8, p1: i8) -> Option<i8> {
        i8::checked_add(p0, p1)
    }

    pub fn checked_sub(p0: i8, p1: i8) -> Option<i8> {
        i8::checked_sub(p0, p1)
    }

    pub fn checked_mul(p0: i8, p1: i8) -> Option<i8> {
        i8::checked_mul(p0, p1)
    }

    pub fn checked_quot(p0: i8, p1: i8) -> Option<i8> {
        i8::checked_div(p0, p1)
    }

    pub fn bit_and(p0: i8, p1: i8) -> i8 {
        core::ops::BitAnd::bitand(p0, p1)
    }

    pub fn bit_or(p0: i8, p1: i8) -> i8 {
        core::ops::BitOr::bitor(p0, p1)
    }

    pub fn bit_xor(p0: i8, p1: i8) -> i8 {
        core::ops::BitXor::bitxor(p0, p1)
    }

    pub fn bit_not(p0: i8) -> i8 {
        core::ops::Not::not(p0)
    }

    pub fn shift_left(p0: i8, p1: u32) -> Option<i8> {
        i8::checked_shl(p0, p1)
    }

    pub fn shift_right(p0: i8, p1: u32) -> Option<i8> {
        i8::checked_shr(p0, p1)
    }

    pub fn convert(p0: i128) -> Option<i8> {
        i8::try_from(p0).ok()
    }
}

pub fn checked_neg_i8(p0: i8) -> Option<i8> {
    i8::checked_neg(p0)
}

pub mod fixed_i16 {
    pub fn checked_add(p0: i16, p1: i16) -> Option<i16> {
        i16::checked_add(p0, p1)
    }

    pub fn checked_sub(p0: i16, p1: i16) -> Option<i16> {
        i16::checked_sub(p0, p1)
    }

    pub fn checked_mul(p0: i16, p1: i16) -> Option<i16> {
        i16::checked_mul(p0, p1)
    }

    pub fn checked_quot(p0: i16, p1: i16) -> Option<i16> {
        i16::checked_div(p0, p1)
    }

    pub fn bit_and(p0: i16, p1: i16) -> i16 {
        core::ops::BitAnd::bitand(p0, p1)
    }

    pub fn bit_or(p0: i16, p1: i16) -> i16 {
        core::ops::BitOr::bitor(p0, p1)
    }

    pub fn bit_xor(p0: i16, p1: i16) -> i16 {
        core::ops::BitXor::bitxor(p0, p1)
    }

    pub fn bit_not(p0: i16) -> i16 {
        core::ops::Not::not(p0)
    }

    pub fn shift_left(p0: i16, p1: u32) -> Option<i16> {
        i16::checked_shl(p0, p1)
    }

    pub fn shift_right(p0: i16, p1: u32) -> Option<i16> {
        i16::checked_shr(p0, p1)
    }

    pub fn convert(p0: i128) -> Option<i16> {
        i16::try_from(p0).ok()
    }
}

pub fn checked_neg_i16(p0: i16) -> Option<i16> {
    i16::checked_neg(p0)
}

pub mod fixed_i32 {
    pub fn checked_add(p0: i32, p1: i32) -> Option<i32> {
        i32::checked_add(p0, p1)
    }

    pub fn checked_sub(p0: i32, p1: i32) -> Option<i32> {
        i32::checked_sub(p0, p1)
    }

    pub fn checked_mul(p0: i32, p1: i32) -> Option<i32> {
        i32::checked_mul(p0, p1)
    }

    pub fn checked_quot(p0: i32, p1: i32) -> Option<i32> {
        i32::checked_div(p0, p1)
    }

    pub fn bit_and(p0: i32, p1: i32) -> i32 {
        core::ops::BitAnd::bitand(p0, p1)
    }

    pub fn bit_or(p0: i32, p1: i32) -> i32 {
        core::ops::BitOr::bitor(p0, p1)
    }

    pub fn bit_xor(p0: i32, p1: i32) -> i32 {
        core::ops::BitXor::bitxor(p0, p1)
    }

    pub fn bit_not(p0: i32) -> i32 {
        core::ops::Not::not(p0)
    }

    pub fn shift_left(p0: i32, p1: u32) -> Option<i32> {
        i32::checked_shl(p0, p1)
    }

    pub fn shift_right(p0: i32, p1: u32) -> Option<i32> {
        i32::checked_shr(p0, p1)
    }

    pub fn convert(p0: i128) -> Option<i32> {
        i32::try_from(p0).ok()
    }
}

pub fn checked_neg_i32(p0: i32) -> Option<i32> {
    i32::checked_neg(p0)
}

pub mod fixed_i64 {
    pub fn checked_add(p0: i64, p1: i64) -> Option<i64> {
        i64::checked_add(p0, p1)
    }

    pub fn checked_sub(p0: i64, p1: i64) -> Option<i64> {
        i64::checked_sub(p0, p1)
    }

    pub fn checked_mul(p0: i64, p1: i64) -> Option<i64> {
        i64::checked_mul(p0, p1)
    }

    pub fn checked_quot(p0: i64, p1: i64) -> Option<i64> {
        i64::checked_div(p0, p1)
    }

    pub fn bit_and(p0: i64, p1: i64) -> i64 {
        core::ops::BitAnd::bitand(p0, p1)
    }

    pub fn bit_or(p0: i64, p1: i64) -> i64 {
        core::ops::BitOr::bitor(p0, p1)
    }

    pub fn bit_xor(p0: i64, p1: i64) -> i64 {
        core::ops::BitXor::bitxor(p0, p1)
    }

    pub fn bit_not(p0: i64) -> i64 {
        core::ops::Not::not(p0)
    }

    pub fn shift_left(p0: i64, p1: u32) -> Option<i64> {
        i64::checked_shl(p0, p1)
    }

    pub fn shift_right(p0: i64, p1: u32) -> Option<i64> {
        i64::checked_shr(p0, p1)
    }

    pub fn convert(p0: i128) -> Option<i64> {
        i64::try_from(p0).ok()
    }
}

pub fn checked_neg_i64(p0: i64) -> Option<i64> {
    i64::checked_neg(p0)
}

#[derive(Clone)]
pub struct Str(std::rc::Rc<str>);
impl Str {
    pub fn lit(text: &str) -> Str { Str(std::rc::Rc::from(text)) }
    pub fn text(&self) -> &str { &self.0 }
}
#[derive(Clone)]
pub struct Bytes(std::rc::Rc<[u8]>);
impl Bytes {
    pub fn lit(octets: &[u8]) -> Bytes { Bytes(std::rc::Rc::from(octets)) }
    pub fn octets(&self) -> &[u8] { &self.0 }
}

pub struct Node<T> { head: T, tail: List<T> }
/// A list is a run of cells laid out contiguously, from the newest at the end of a buffer
/// to the oldest at its start, or a cell in front of another list. A `cons` onto a run that
/// owns its buffer alone pushes in place; a `cons` onto any other list is one cell, so
/// building and taking apart a cell is constant work whatever the sharing, and no buffer is
/// written while another list can see it.
pub enum Repr<T> { Nil, Run { buffer: std::rc::Rc<Vec<T>>, len: usize }, Cell(std::rc::Rc<Node<T>>) }
pub struct List<T>(Repr<T>);
impl<T> Clone for List<T> {
    fn clone(&self) -> Self {
        List(match &self.0 {
            Repr::Nil => Repr::Nil,
            Repr::Run { buffer, len } => Repr::Run { buffer: buffer.clone(), len: *len },
            Repr::Cell(cell) => Repr::Cell(cell.clone()),
        })
    }
}
impl<T> List<T> {
    fn take(&mut self) -> Repr<T> { std::mem::replace(&mut self.0, Repr::Nil) }
}
impl<T> Drop for Node<T> {
    fn drop(&mut self) {
        let mut next = match self.tail.take() { Repr::Cell(cell) => Some(cell), _ => None };
        while let Some(cell) = next { // release
            match std::rc::Rc::try_unwrap(cell) {
                Ok(mut node) => next = match node.tail.take() { Repr::Cell(cell) => Some(cell), _ => None },
                Err(_) => break,
            }
        }
    }
}
impl<T: Clone> List<T> {
    pub fn nil() -> Self { List(Repr::Nil) }
    pub fn cons(head: T, mut tail: List<T>) -> Self {
        List(match tail.take() {
            Repr::Nil => {
                let mut buffer = Vec::with_capacity(16);
                buffer.push(head);
                Repr::Run { buffer: std::rc::Rc::new(buffer), len: 1 }
            }
            Repr::Run { mut buffer, len } => {
                if let Some(cells) = std::rc::Rc::get_mut(&mut buffer) {
                    cells.truncate(len);
                    cells.push(head);
                    Repr::Run { buffer, len: len + 1 }
                } else {
                    Repr::Cell(std::rc::Rc::new(Node { head, tail: List(Repr::Run { buffer, len }) }))
                }
            }
            rest => Repr::Cell(std::rc::Rc::new(Node { head, tail: List(rest) })),
        })
    }
    pub fn uncons(&self) -> Option<(T, List<T>)> {
        match &self.0 {
            Repr::Nil => None,
            Repr::Run { buffer, len } => {
                let head = buffer[len - 1].clone();
                Some((head, List(if *len == 1 { Repr::Nil } else { Repr::Run { buffer: buffer.clone(), len: len - 1 } })))
            }
            Repr::Cell(cell) => Some((cell.head.clone(), cell.tail.clone())),
        }
    }
    /// Walk the items in order, ticking one per item, until `each` says to stop: the cells
    /// in front, then the run they end in.
    fn walk(&self, each: &mut dyn FnMut(&T) -> bool) {
        let mut cursor = self;
        while let Repr::Cell(cell) = &cursor.0 { tick(1); if !each(&cell.head) { return; } cursor = &cell.tail; }
        if let Repr::Run { buffer, len } = &cursor.0 {
            for item in buffer[..*len].iter().rev() { tick(1); if !each(item) { return; } }
        }
    }
    fn items(&self) -> Vec<T> {
        let mut out = Vec::new();
        self.walk(&mut |item| { out.push(item.clone()); true });
        out
    }
    fn onto(items: Vec<T>, tail: List<T>) -> List<T> {
        let mut out = tail;
        for item in items.into_iter().rev() { tick(1); out = List::cons(item, out); }
        out
    }
}

fn chars(text: &str) -> u64 {
    let mut count = 0;
    for _ in text.chars() { tick(1); count += 1; }
    count
}

impl Same for Str {
    fn same(&self, other: &Self) -> bool {
        let mut left = self.0.chars();
        let mut right = other.0.chars();
        loop {
            tick(1);
            match (left.next(), right.next()) { (None, None) => return true, (a, b) if a != b => return false, _ => {} }
        }
    }
}
impl Same for Bytes {
    fn same(&self, other: &Self) -> bool { tick(1 + self.0.len().min(other.0.len()) as u64); self.0 == other.0 }
}
impl Key for Str {
    fn key(&self, other: &Self) -> Ordering {
        let mut left = self.0.chars();
        let mut right = other.0.chars();
        loop {
            tick(1);
            match (left.next(), right.next()) {
                (None, None) => return Ordering::Equal,
                (None, Some(_)) => return Ordering::Less,
                (Some(_), None) => return Ordering::Greater,
                (Some(a), Some(b)) => match a.cmp(&b) { Ordering::Equal => {} decided => return decided },
            }
        }
    }
}

pub fn append_list<T: Clone>(a: List<T>, b: List<T>) -> List<T> { List::onto(a.items(), b) }
pub fn append_bytes(a: Bytes, b: Bytes) -> Bytes {
    tick((a.0.len() + b.0.len()) as u64);
    let mut out = Vec::with_capacity(a.0.len() + b.0.len());
    out.extend_from_slice(&a.0);
    out.extend_from_slice(&b.0);
    Bytes(std::rc::Rc::from(out))
}
pub fn length_list<T: Clone>(a: List<T>) -> u64 {
    let mut count = 0;
    a.walk(&mut |_| { count += 1; true });
    count
}
pub fn length_bytes(a: Bytes) -> u64 { a.0.len() as u64 }
pub fn length_string(a: Str) -> u64 { chars(&a.0) }
pub fn index_list<T: Clone>(a: List<T>, i: u64) -> Option<T> {
    let mut position = 0;
    let mut found = None;
    a.walk(&mut |item| { if position == i { found = Some(item.clone()); return false; } position += 1; true });
    found
}
pub fn index_bytes(a: Bytes, i: u64) -> Option<u8> { usize::try_from(i).ok().and_then(|i| a.0.get(i)).copied() }
pub fn slice_list<T: Clone>(a: List<T>, start: u64, count: u64) -> Option<List<T>> {
    let end = u128::from(start) + u128::from(count);
    let mut part = Vec::new();
    let mut position: u128 = 0;
    a.walk(&mut |item| {
        if position >= end { return false; }
        if position >= u128::from(start) { part.push(item.clone()); }
        position += 1;
        true
    });
    if position < end { None } else { Some(List::onto(part, List::nil())) }
}
pub fn slice_bytes(a: Bytes, start: u64, count: u64) -> Option<Bytes> {
    let end = u128::from(start) + u128::from(count);
    if end > a.0.len() as u128 { return None; }
    tick(count);
    Some(Bytes::lit(&a.0[start as usize..end as usize]))
}
pub fn utf8_encode(a: Str) -> Bytes { tick(a.0.len() as u64); Bytes::lit(a.0.as_bytes()) }
pub fn utf8_decode(a: Bytes) -> Option<Str> { tick(a.0.len() as u64); core::str::from_utf8(&a.0).ok().map(Str::lit) }
pub fn compare_bytes(a: Bytes, b: Bytes) -> Ordering { tick(1 + a.0.len().min(b.0.len()) as u64); a.0.cmp(&b.0) }
pub fn split_exact(text: Str, delimiter: Str, maximum: u32) -> Option<List<Str>> {
    if delimiter.0.is_empty() { return None; }
    chars(&text.0);
    let fields: Vec<Str> = text.0.split(&*delimiter.0).map(Str::lit).collect();
    if fields.len() as u128 > u128::from(maximum) { return None; }
    Some(List::onto(fields, List::nil()))
}
pub fn join(texts: List<Str>, delimiter: Str) -> Str {
    let mut out = String::new();
    let mut first = true;
    texts.walk(&mut |text| {
        if !first { chars(&delimiter.0); out.push_str(&delimiter.0); }
        first = false;
        chars(&text.0);
        out.push_str(&text.0);
        true
    });
    Str::lit(&out)
}
fn canonical_decimal(text: &str) -> Option<Option<i128>> {
    let digits = text.strip_prefix('-').unwrap_or(text);
    let mut canonical = !digits.is_empty() && (digits == "0" || !digits.starts_with('0')) && text != "-0";
    for c in digits.chars() { tick(1); canonical = canonical && c.is_ascii_digit(); }
    if !canonical { return None; }
    Some(text.parse::<i128>().ok())
}
pub fn parse_int(text: Str) -> R<Option<i64>> {
    match canonical_decimal(&text.0) {
        None => Ok(None),
        Some(Some(n)) => i64::try_from(n).map(Some).map_err(|_| Overflow),
        Some(None) => Err(Overflow),
    }
}
macro_rules! decimal {
    ($t:ident, $format:ident, $parse:ident) => {
        pub fn $format(a: $t) -> Str { let text = a.to_string(); tick(text.len() as u64); Str::lit(&text) }
        pub fn $parse(text: Str) -> Option<$t> {
            match canonical_decimal(&text.0) { Some(Some(n)) => $t::try_from(n).ok(), _ => None }
        }
    };
}
decimal!(u8, format_u8, parse_u8); decimal!(u16, format_u16, parse_u16);
decimal!(u32, format_u32, parse_u32); decimal!(u64, format_u64, parse_u64);
decimal!(i8, format_i8, parse_i8); decimal!(i16, format_i16, parse_i16);
decimal!(i32, format_i32, parse_i32); decimal!(i64, format_i64, parse_i64);

#[derive(Clone)]
pub enum Adt0 {
    C0,
    C1(Adt1),
}

#[derive(Clone)]
pub enum Adt1 {
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    C9,
    C10,
}

#[derive(Clone)]
pub enum Fn0 {
    F19(Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: (List<u64>, u64)) -> R<(List<u64>, u64)> {
        match self {
            Fn0::F19(k0) => f19(k0.clone(), p0),
        }
    }
}

pub fn f0(v0: Bytes, v1: Bytes, v2: List<Bytes>, v3: Bytes) -> R<List<u64>> {
    Ok(f1(f2((v0, v1), v2, v3)?))
}

pub fn f1(v0: Adt0) -> List<u64> {
    let m1 = v0;
    match m1 {
        Adt0::C0 => {
            List::cons(0u64, List::<u64>::nil())
        }
        Adt0::C1(v1) => {
            List::cons(1u64, List::cons(f3(v1), List::<u64>::nil()))
        }
    }
}

pub fn f2(v0: (Bytes, Bytes), v1: List<Bytes>, v2: Bytes) -> R<Adt0> {
    Ok(f4(f5(v2.clone()), f4(f6(v0, v2.clone())?, f7(f8(v1, List::<List<u64>>::nil())?, f9(v2)?))))
}

pub fn f3(v0: Adt1) -> u64 {
    let m2 = v0;
    match m2 {
        Adt1::C0 => {
            0u64
        }
        Adt1::C1 => {
            1u64
        }
        Adt1::C2 => {
            2u64
        }
        Adt1::C3 => {
            3u64
        }
        Adt1::C4 => {
            4u64
        }
        Adt1::C5 => {
            5u64
        }
        Adt1::C6 => {
            6u64
        }
        Adt1::C7 => {
            7u64
        }
        Adt1::C8 => {
            8u64
        }
        Adt1::C9 => {
            9u64
        }
        Adt1::C10 => {
            10u64
        }
    }
}

pub fn f4(v0: Adt0, v1: Adt0) -> Adt0 {
    let m3 = v0;
    match m3 {
        Adt0::C0 => {
            v1
        }
        Adt0::C1(v2) => {
            Adt0::C1(v2)
        }
    }
}

pub fn f5(v0: Bytes) -> Adt0 {
    let m5 = {
        let a4 = v0;
        utf8_decode(a4)
    };
    match m5 {
        None => {
            Adt0::C1(Adt1::C0)
        }
        Some(_) => {
            Adt0::C0
        }
    }
}

pub fn f6(v0: (Bytes, Bytes), v1: Bytes) -> R<Adt0> {
    if f10(v0, f11(f9(v1)?)?)? {
        Ok(Adt0::C0)
    } else {
        Ok(Adt0::C1(Adt1::C1))
    }
}

pub fn f7(v0: List<List<u64>>, v1: List<u64>) -> Adt0 {
    f4(f12(v1.clone()), f4(f13(f14(v1.clone())), f4(f15(v0, v1.clone()), f16(v1))))
}

pub fn f8(p8: List<Bytes>, p9: List<List<u64>>) -> R<List<List<u64>>> {
    let m7 = (p8, p9);
    {
        let mut m7 = m7;
        loop {
            let step = {
                let (v0, v1) = m7;
                let m6 = v0;
                match m6.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<List<u64>>), R<List<List<u64>>>>(Ok(f17(v1, List::<List<u64>>::nil())))
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<List<u64>>), R<List<List<u64>>>>((v3, List::cons(f9(v2)?, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m7 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f9(v0: Bytes) -> R<List<u64>> {
    let (h10, _) = f18(Fn0::F19(v0.clone()), {
        let a11 = v0.clone();
        length_bytes(a11)
    }, (List::<u64>::nil(), {
        let a12 = v0;
        length_bytes(a12)
    }))?;
    Ok(h10)
}

pub fn f10(v0: (Bytes, Bytes), v1: List<u64>) -> R<bool> {
    if f20(f21(), v1.clone()) {
        Ok(true)
    } else {
        Ok(f22(v1.clone(), f23(v0, v1)?))
    }
}

pub fn f11(v0: List<u64>) -> R<List<u64>> {
    Ok(f24(f25(v0, 0u64, 0u64, List::<u64>::nil())?, List::<u64>::nil()))
}

pub fn f12(v0: List<u64>) -> Adt0 {
    let m13 = v0;
    match m13.uncons() {
        None => {
            Adt0::C0
        }
        Some((v1, _)) => {
            let a14 = v1;
            let a15 = 47u64;
            if nat_eq(a14, a15) {
                Adt0::C1(Adt1::C2)
            } else {
                Adt0::C0
            }
        }
    }
}

pub fn f13(p19: List<List<u64>>) -> Adt0 {
    let m18 = p19;
    {
        let mut m18 = m18;
        loop {
            let step = {
                let v0 = m18;
                let m16 = v0;
                match m16.uncons() {
                    None => {
                        Err::<List<List<u64>>, Adt0>(Adt0::C0)
                    }
                    Some((v1, v2)) => {
                        let m17 = f26(v1);
                        match m17 {
                            Adt0::C0 => {
                                Ok::<List<List<u64>>, Adt0>(v2)
                            }
                            Adt0::C1(v3) => {
                                Err::<List<List<u64>>, Adt0>(Adt0::C1(v3))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m18 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f14(v0: List<u64>) -> List<List<u64>> {
    f17(f27(v0, List::<u64>::nil(), List::<List<u64>>::nil()), List::<List<u64>>::nil())
}

pub fn f15(p22: List<List<u64>>, v1: List<u64>) -> Adt0 {
    let m21 = p22;
    {
        let mut m21 = m21;
        loop {
            let step = {
                let v0 = m21;
                let m20 = v0;
                match m20.uncons() {
                    None => {
                        Err::<List<List<u64>>, Adt0>(Adt0::C1(Adt1::C6))
                    }
                    Some((v2, v3)) => {
                        if f28(v2, v1.clone()) {
                            Err::<List<List<u64>>, Adt0>(Adt0::C0)
                        } else {
                            Ok::<List<List<u64>>, Adt0>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m21 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f16(v0: List<u64>) -> Adt0 {
    let m23 = f29(v0, List::cons(46u64, List::cons(108u64, List::cons(101u64, List::cons(120u64, List::cons(46u64, List::cons(116u64, List::cons(101u64, List::cons(120u64, List::<u64>::nil())))))))));
    if m23 {
        Adt0::C0
    } else {
        Adt0::C1(Adt1::C7)
    }
}

pub fn f17(p26: List<List<u64>>, p27: List<List<u64>>) -> List<List<u64>> {
    let m25 = (p26, p27);
    {
        let mut m25 = m25;
        loop {
            let step = {
                let (v0, v1) = m25;
                let m24 = v0;
                match m24.uncons() {
                    None => {
                        Err::<(List<List<u64>>, List<List<u64>>), List<List<u64>>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<u64>>, List<List<u64>>), List<List<u64>>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m25 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f18(v0: Fn0, p31: u64, p32: (List<u64>, u64)) -> R<(List<u64>, u64)> {
    let m30 = (p31, p32);
    {
        let mut m30 = m30;
        loop {
            let step = {
                let (v1, v2) = m30;
                let m28 = v1;
                if m28 == 0 {
                    Err::<(u64, (List<u64>, u64)), R<(List<u64>, u64)>>(Ok(v2))
                } else {
                    let v3 = m28 - 1;
                    Ok::<(u64, (List<u64>, u64)), R<(List<u64>, u64)>>((v3, {
                        let c29: Fn0 = v0.clone();
                        c29.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m30 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f19(v0: Bytes, v1: (List<u64>, u64)) -> R<(List<u64>, u64)> {
    Ok((List::cons(f30(v0, {
        let a34 = {
            let (_, h33) = v1.clone();
            h33
        };
        let a35 = 1u64;
        nat_sub(a34, a35)
    })?, {
        let (h36, _) = v1;
        h36
    }), {
        let a38 = {
            let (_, h37) = v1;
            h37
        };
        let a39 = 1u64;
        nat_sub(a38, a39)
    }))
}

pub fn f20(v0: u64, p44: List<u64>) -> bool {
    let m43 = p44;
    {
        let mut m43 = m43;
        loop {
            let step = {
                let v1 = m43;
                let m40 = v1;
                match m40.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        let a41 = v2;
                        let a42 = v0;
                        if nat_lt(a41, a42) {
                            Ok::<List<u64>, bool>(v3)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m43 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f21() -> u64 {
    768u64
}

pub fn f22(p51: List<u64>, p52: List<u64>) -> bool {
    let m50 = (p51, p52);
    {
        let mut m50 = m50;
        loop {
            let step = {
                let (v0, v1) = m50;
                let m45 = v0;
                match m45.uncons() {
                    None => {
                        let m46 = v1;
                        match m46.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m47 = v1;
                        match m47.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                            Some((v6, v7)) => {
                                let a48 = v4;
                                let a49 = v6;
                                if nat_eq(a48, a49) {
                                    Ok::<(List<u64>, List<u64>), bool>((v5, v7))
                                } else {
                                    Err::<(List<u64>, List<u64>), bool>(false)
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m50 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f23(v0: (Bytes, Bytes), v1: List<u64>) -> R<List<u64>> {
    f31(v0.clone(), f32(v0, v1)?, false, 0u64, List::<u64>::nil(), List::<u64>::nil())
}

pub fn f24(p55: List<u64>, p56: List<u64>) -> List<u64> {
    let m54 = (p55, p56);
    {
        let mut m54 = m54;
        loop {
            let step = {
                let (v0, v1) = m54;
                let m53 = v0;
                match m53.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m54 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f25(p83: List<u64>, p84: u64, p85: u64, p86: List<u64>) -> R<List<u64>> {
    let m82 = (p83, (p84, (p85, p86)));
    {
        let mut m82 = m82;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m82;
                let m57 = v0;
                match m57.uncons() {
                    None => {
                        Err::<(List<u64>, (u64, (u64, List<u64>))), R<List<u64>>>(Ok(v3))
                    }
                    Some((v4, v5)) => {
                        let a58 = v2;
                        let a59 = 0u64;
                        if nat_eq(a58, a59) {
                            let a60 = v4;
                            let a61 = 128u64;
                            if nat_lt(a60, a61) {
                                Ok::<(List<u64>, (u64, (u64, List<u64>))), R<List<u64>>>((v5, (0u64, (0u64, List::cons(v4, v3)))))
                            } else {
                                let a62 = v4;
                                let a63 = 224u64;
                                if nat_lt(a62, a63) {
                                    Ok::<(List<u64>, (u64, (u64, List<u64>))), R<List<u64>>>((v5, ({
                                        let a64 = v4;
                                        let a65 = 192u64;
                                        nat_sub(a64, a65)
                                    }, (1u64, v3))))
                                } else {
                                    let a66 = v4;
                                    let a67 = 240u64;
                                    if nat_lt(a66, a67) {
                                        Ok::<(List<u64>, (u64, (u64, List<u64>))), R<List<u64>>>((v5, ({
                                            let a68 = v4;
                                            let a69 = 224u64;
                                            nat_sub(a68, a69)
                                        }, (2u64, v3))))
                                    } else {
                                        Ok::<(List<u64>, (u64, (u64, List<u64>))), R<List<u64>>>((v5, ({
                                            let a70 = v4;
                                            let a71 = 240u64;
                                            nat_sub(a70, a71)
                                        }, (3u64, v3))))
                                    }
                                }
                            }
                        } else {
                            let v6: u64 = {
                                let a76 = {
                                    let a72 = v1;
                                    let a73 = 64u64;
                                    nat_mul(a72, a73)?
                                };
                                let a77 = {
                                    let a74 = v4;
                                    let a75 = 128u64;
                                    nat_sub(a74, a75)
                                };
                                nat_add(a76, a77)?
                            };
                            let a78 = v2;
                            let a79 = 1u64;
                            if nat_eq(a78, a79) {
                                Ok::<(List<u64>, (u64, (u64, List<u64>))), R<List<u64>>>((v5, (0u64, (0u64, List::cons(v6, v3)))))
                            } else {
                                Ok::<(List<u64>, (u64, (u64, List<u64>))), R<List<u64>>>((v5, (v6, ({
                                    let a80 = v2;
                                    let a81 = 1u64;
                                    nat_sub(a80, a81)
                                }, v3))))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m82 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f26(v0: List<u64>) -> Adt0 {
    f33(f22(v0.clone(), List::<u64>::nil()), f22(v0.clone(), List::cons(46u64, List::<u64>::nil())), f22(v0, List::cons(46u64, List::cons(46u64, List::<u64>::nil()))))
}

pub fn f27(p91: List<u64>, p92: List<u64>, p93: List<List<u64>>) -> List<List<u64>> {
    let m90 = (p91, (p92, p93));
    {
        let mut m90 = m90;
        loop {
            let step = {
                let (v0, (v1, v2)) = m90;
                let m87 = v0;
                match m87.uncons() {
                    None => {
                        Err::<(List<u64>, (List<u64>, List<List<u64>>)), List<List<u64>>>(List::cons(f24(v1, List::<u64>::nil()), v2))
                    }
                    Some((v3, v4)) => {
                        let a88 = v3;
                        let a89 = 47u64;
                        if nat_eq(a88, a89) {
                            Ok::<(List<u64>, (List<u64>, List<List<u64>>)), List<List<u64>>>((v4, (List::<u64>::nil(), List::cons(f24(v1, List::<u64>::nil()), v2))))
                        } else {
                            Ok::<(List<u64>, (List<u64>, List<List<u64>>)), List<List<u64>>>((v4, (List::cons(v3, v1), v2)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m90 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f28(p102: List<u64>, p103: List<u64>) -> bool {
    let m101 = (p102, p103);
    {
        let mut m101 = m101;
        loop {
            let step = {
                let (v0, v1) = m101;
                let m94 = v0;
                match m94.uncons() {
                    None => {
                        let m95 = v1;
                        match m95.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                            Some((v2, _)) => {
                                let a96 = v2;
                                let a97 = 47u64;
                                Err::<(List<u64>, List<u64>), bool>(nat_eq(a96, a97))
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m98 = v1;
                        match m98.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                            Some((v6, v7)) => {
                                let a99 = v4;
                                let a100 = v6;
                                if nat_eq(a99, a100) {
                                    Ok::<(List<u64>, List<u64>), bool>((v5, v7))
                                } else {
                                    Err::<(List<u64>, List<u64>), bool>(false)
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m101 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f29(p106: List<u64>, v1: List<u64>) -> bool {
    let m105 = p106;
    {
        let mut m105 = m105;
        loop {
            let step = {
                let v0 = m105;
                let m104 = v0;
                match m104.uncons() {
                    None => {
                        Err::<List<u64>, bool>(f22(List::<u64>::nil(), v1.clone()))
                    }
                    Some((v2, v3)) => {
                        if f22(List::cons(v2, v3.clone()), v1.clone()) {
                            Err::<List<u64>, bool>(true)
                        } else {
                            Ok::<List<u64>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m105 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f30(v0: Bytes, v1: u64) -> R<u64> {
    let m109 = {
        let a107 = v0;
        let a108 = v1;
        index_bytes(a107, a108)
    };
    match m109 {
        None => {
            Ok(0u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a154 = {
                let a130 = {
                    let a118 = {
                        let a112 = {
                            let a110 = v3;
                            let a111 = 128u8;
                            fixed_u8::bit_and(a110, a111)
                        };
                        let a113 = 128u8;
                        if equal(a112, a113) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a119 = {
                        let a116 = {
                            let a114 = v3;
                            let a115 = 64u8;
                            fixed_u8::bit_and(a114, a115)
                        };
                        let a117 = 64u8;
                        if equal(a116, a117) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a118, a119)?
                };
                let a131 = {
                    let a128 = {
                        let a122 = {
                            let a120 = v3;
                            let a121 = 32u8;
                            fixed_u8::bit_and(a120, a121)
                        };
                        let a123 = 32u8;
                        if equal(a122, a123) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a129 = {
                        let a126 = {
                            let a124 = v3;
                            let a125 = 16u8;
                            fixed_u8::bit_and(a124, a125)
                        };
                        let a127 = 16u8;
                        if equal(a126, a127) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a128, a129)?
                };
                nat_add(a130, a131)?
            };
            let a155 = {
                let a152 = {
                    let a140 = {
                        let a134 = {
                            let a132 = v3;
                            let a133 = 8u8;
                            fixed_u8::bit_and(a132, a133)
                        };
                        let a135 = 8u8;
                        if equal(a134, a135) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a141 = {
                        let a138 = {
                            let a136 = v3;
                            let a137 = 4u8;
                            fixed_u8::bit_and(a136, a137)
                        };
                        let a139 = 4u8;
                        if equal(a138, a139) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a140, a141)?
                };
                let a153 = {
                    let a150 = {
                        let a144 = {
                            let a142 = v3;
                            let a143 = 2u8;
                            fixed_u8::bit_and(a142, a143)
                        };
                        let a145 = 2u8;
                        if equal(a144, a145) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a151 = {
                        let a148 = {
                            let a146 = v3;
                            let a147 = 1u8;
                            fixed_u8::bit_and(a146, a147)
                        };
                        let a149 = 1u8;
                        if equal(a148, a149) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a150, a151)?
                };
                nat_add(a152, a153)?
            };
            nat_add(a154, a155)
        }
    }
}

pub fn f31(v0: (Bytes, Bytes), p164: List<u64>, p165: bool, p166: u64, p167: List<u64>, p168: List<u64>) -> R<List<u64>> {
    let m163 = (p164, (p165, (p166, (p167, p168))));
    {
        let mut m163 = m163;
        loop {
            let step = {
                let (v1, (v2, (v3, (v4, v5)))) = m163;
                let m156 = v1;
                match m156.uncons() {
                    None => {
                        if v2 {
                            Err::<(List<u64>, (bool, (u64, (List<u64>, List<u64>)))), R<List<u64>>>(Ok(f24(f34(v3, v4, v5), List::<u64>::nil())))
                        } else {
                            Err::<(List<u64>, (bool, (u64, (List<u64>, List<u64>)))), R<List<u64>>>(Ok(f24(v5, List::<u64>::nil())))
                        }
                    }
                    Some((v6, v7)) => {
                        let v8: u64 = f35(v0.clone(), v6)?;
                        if v2 {
                            let v9: u64 = f36(f37(v0.clone(), v4.clone(), v8)?, f38(v0.clone(), v3, v6)?);
                            let a157 = v9;
                            let a158 = 0u64;
                            if nat_eq(a157, a158) {
                                let a159 = v8;
                                let a160 = 0u64;
                                if nat_eq(a159, a160) {
                                    Ok::<(List<u64>, (bool, (u64, (List<u64>, List<u64>)))), R<List<u64>>>((v7, (true, (v6, (List::<u64>::nil(), f34(v3, v4, v5))))))
                                } else {
                                    Ok::<(List<u64>, (bool, (u64, (List<u64>, List<u64>)))), R<List<u64>>>((v7, (true, (v3, (List::cons(v6, v4), v5)))))
                                }
                            } else {
                                Ok::<(List<u64>, (bool, (u64, (List<u64>, List<u64>)))), R<List<u64>>>((v7, (true, (v9, (v4, v5)))))
                            }
                        } else {
                            let a161 = v8;
                            let a162 = 0u64;
                            if nat_eq(a161, a162) {
                                Ok::<(List<u64>, (bool, (u64, (List<u64>, List<u64>)))), R<List<u64>>>((v7, (true, (v6, (List::<u64>::nil(), v5)))))
                            } else {
                                Ok::<(List<u64>, (bool, (u64, (List<u64>, List<u64>)))), R<List<u64>>>((v7, (false, (0u64, (List::<u64>::nil(), List::cons(v6, v5))))))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m163 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f32(v0: (Bytes, Bytes), v1: List<u64>) -> R<List<u64>> {
    f39(v0.clone(), f40(v0, v1)?)
}

pub fn f33(v0: bool, v1: bool, v2: bool) -> Adt0 {
    let m169 = v0;
    if m169 {
        Adt0::C1(Adt1::C3)
    } else {
        let m170 = v1;
        if m170 {
            Adt0::C1(Adt1::C4)
        } else {
            let m171 = v2;
            if m171 {
                Adt0::C1(Adt1::C5)
            } else {
                Adt0::C0
            }
        }
    }
}

pub fn f34(v0: u64, v1: List<u64>, v2: List<u64>) -> List<u64> {
    f24(f24(v1, List::<u64>::nil()), List::cons(v0, v2))
}

pub fn f35(v0: (Bytes, Bytes), v1: u64) -> R<u64> {
    let a172 = v1;
    let a173 = f21();
    if nat_lt(a172, a173) {
        Ok(0u64)
    } else {
        let v2: u64 = f41(v0.clone(), f42(v0.clone(), 0u64)?, 7u64, f43(v0.clone(), 0u64)?, 3u64, v1)?;
        let a174 = v1;
        let a175 = f44(v0.clone(), v2, 3u64, 0u64)?;
        if nat_lt(a174, a175) {
            Ok(0u64)
        } else {
            let a178 = f44(v0.clone(), {
                let a176 = v2;
                let a177 = 3u64;
                nat_add(a176, a177)?
            }, 3u64, 0u64)?;
            let a179 = v1;
            if nat_lt(a178, a179) {
                Ok(0u64)
            } else {
                f45(v0, {
                    let a180 = v2;
                    let a181 = 6u64;
                    nat_add(a180, a181)?
                })
            }
        }
    }
}

pub fn f36(v0: bool, v1: u64) -> u64 {
    if v0 {
        0u64
    } else {
        v1
    }
}

pub fn f37(v0: (Bytes, Bytes), v1: List<u64>, v2: u64) -> R<bool> {
    let m182 = v1;
    match m182.uncons() {
        None => {
            Ok(false)
        }
        Some((v3, _)) => {
            let a183 = f35(v0, v3)?;
            let a184 = v2;
            Ok(!nat_lt(a183, a184))
        }
    }
}

pub fn f38(v0: (Bytes, Bytes), v1: u64, v2: u64) -> R<u64> {
    Ok(f46(f47(v1, v2)?, f48(v0.clone(), v1, v2)?, f49(v0.clone(), f48(v0, v1, v2)?)?))
}

pub fn f39(v0: (Bytes, Bytes), v1: List<u64>) -> R<List<u64>> {
    Ok(f24(f50(v0, v1, List::<u64>::nil())?, List::<u64>::nil()))
}

pub fn f40(v0: (Bytes, Bytes), v1: List<u64>) -> R<List<u64>> {
    f51(v0, f52(), v1)
}

pub fn f41(v0: (Bytes, Bytes), v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<u64> {
    let a187 = v1;
    let a188 = {
        let a185 = f53(16u64, v0, v1, v2, v4, v5, 0u64, v3)?;
        let a186 = v2;
        nat_mul(a185, a186)?
    };
    nat_add(a187, a188)
}

pub fn f42(v0: (Bytes, Bytes), v1: u64) -> R<u64> {
    f44(v0, {
        let a189 = v1;
        let a190 = 6u64;
        nat_mul(a189, a190)?
    }, 3u64, 0u64)
}

pub fn f43(v0: (Bytes, Bytes), v1: u64) -> R<u64> {
    f44(v0, {
        let a193 = {
            let a191 = v1;
            let a192 = 6u64;
            nat_mul(a191, a192)?
        };
        let a194 = 3u64;
        nat_add(a193, a194)?
    }, 3u64, 0u64)
}

pub fn f44(v0: (Bytes, Bytes), p203: u64, p204: u64, p205: u64) -> R<u64> {
    let m202 = (p203, (p204, p205));
    {
        let mut m202 = m202;
        loop {
            let step = {
                let (v1, (v2, v3)) = m202;
                let m195 = v2;
                if m195 == 0 {
                    Err::<(u64, (u64, u64)), R<u64>>(Ok(v3))
                } else {
                    let v4 = m195 - 1;
                    Ok::<(u64, (u64, u64)), R<u64>>(({
                        let a196 = v1;
                        let a197 = 1u64;
                        nat_add(a196, a197)?
                    }, (v4, {
                        let a200 = {
                            let a198 = v3;
                            let a199 = 256u64;
                            nat_mul(a198, a199)?
                        };
                        let a201 = f45(v0.clone(), v1)?;
                        nat_add(a200, a201)?
                    })))
                }
            };
            match step {
                Ok(next) => { m202 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f45(v0: (Bytes, Bytes), v1: u64) -> R<u64> {
    let a206 = v1;
    let a207 = f54();
    if nat_lt(a206, a207) {
        f30({
            let (h208, _) = v0;
            h208
        }, v1)
    } else {
        f30({
            let (_, h209) = v0;
            h209
        }, {
            let a210 = v1;
            let a211 = f54();
            nat_sub(a210, a211)
        })
    }
}

pub fn f46(v0: u64, v1: u64, v2: bool) -> u64 {
    let a212 = v0;
    let a213 = 0u64;
    if nat_eq(a212, a213) {
        let a214 = v1;
        let a215 = 0u64;
        if nat_eq(a214, a215) {
            0u64
        } else {
            if v2 {
                0u64
            } else {
                v1
            }
        }
    } else {
        v0
    }
}

pub fn f47(v0: u64, v1: u64) -> R<u64> {
    if f55(v0) {
        let a221 = {
            let a218 = {
                let a216 = v0;
                let a217 = 44032u64;
                nat_sub(a216, a217)
            };
            let a219 = 28u64;
            let a220 = 0u64;
            nat_rem(a218, a219, a220)
        };
        let a222 = 0u64;
        if nat_eq(a221, a222) {
            let a223 = v1;
            let a224 = 4520u64;
            if nat_lt(a223, a224) {
                Ok(0u64)
            } else {
                let a225 = v1;
                let a226 = 4547u64;
                if nat_lt(a225, a226) {
                    let a229 = v0;
                    let a230 = {
                        let a227 = v1;
                        let a228 = 4519u64;
                        nat_sub(a227, a228)
                    };
                    nat_add(a229, a230)
                } else {
                    Ok(0u64)
                }
            }
        } else {
            Ok(0u64)
        }
    } else {
        let a231 = v0;
        let a232 = 4352u64;
        if nat_lt(a231, a232) {
            Ok(0u64)
        } else {
            let a233 = v0;
            let a234 = 4371u64;
            if nat_lt(a233, a234) {
                let a235 = v1;
                let a236 = 4449u64;
                if nat_lt(a235, a236) {
                    Ok(0u64)
                } else {
                    let a237 = v1;
                    let a238 = 4470u64;
                    if nat_lt(a237, a238) {
                        let a249 = 44032u64;
                        let a250 = {
                            let a247 = {
                                let a245 = {
                                    let a241 = {
                                        let a239 = v0;
                                        let a240 = 4352u64;
                                        nat_sub(a239, a240)
                                    };
                                    let a242 = 21u64;
                                    nat_mul(a241, a242)?
                                };
                                let a246 = {
                                    let a243 = v1;
                                    let a244 = 4449u64;
                                    nat_sub(a243, a244)
                                };
                                nat_add(a245, a246)?
                            };
                            let a248 = 28u64;
                            nat_mul(a247, a248)?
                        };
                        nat_add(a249, a250)
                    } else {
                        Ok(0u64)
                    }
                }
            } else {
                Ok(0u64)
            }
        }
    }
}

pub fn f48(v0: (Bytes, Bytes), v1: u64, v2: u64) -> R<u64> {
    let a251 = v2;
    let a252 = f21();
    if nat_lt(a251, a252) {
        Ok(0u64)
    } else {
        let v3: u64 = {
            let a255 = {
                let a253 = v1;
                let a254 = 16777216u64;
                nat_mul(a253, a254)?
            };
            let a256 = v2;
            nat_add(a255, a256)?
        };
        let v4: u64 = f41(v0.clone(), f42(v0.clone(), 4u64)?, 9u64, f43(v0.clone(), 4u64)?, 6u64, v3)?;
        let a257 = f44(v0.clone(), v4, 6u64, 0u64)?;
        let a258 = v3;
        if nat_eq(a257, a258) {
            f44(v0, {
                let a259 = v4;
                let a260 = 6u64;
                nat_add(a259, a260)?
            }, 3u64, 0u64)
        } else {
            Ok(0u64)
        }
    }
}

pub fn f49(v0: (Bytes, Bytes), v1: u64) -> R<bool> {
    let a261 = v1;
    let a262 = f21();
    if nat_lt(a261, a262) {
        Ok(false)
    } else {
        let v2: u64 = f41(v0.clone(), f42(v0.clone(), 1u64)?, 6u64, f43(v0.clone(), 1u64)?, 3u64, v1)?;
        let a263 = v1;
        let a264 = f44(v0.clone(), v2, 3u64, 0u64)?;
        if nat_lt(a263, a264) {
            Ok(false)
        } else {
            let a267 = f44(v0, {
                let a265 = v2;
                let a266 = 3u64;
                nat_add(a265, a266)?
            }, 3u64, 0u64)?;
            let a268 = v1;
            Ok(!nat_lt(a267, a268))
        }
    }
}

pub fn f50(v0: (Bytes, Bytes), p273: List<u64>, p274: List<u64>) -> R<List<u64>> {
    let m272 = (p273, p274);
    {
        let mut m272 = m272;
        loop {
            let step = {
                let (v1, v2) = m272;
                let m269 = v1;
                match m269.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), R<List<u64>>>(Ok(v2))
                    }
                    Some((v3, v4)) => {
                        let v5: u64 = f35(v0.clone(), v3)?;
                        let a270 = v5;
                        let a271 = 0u64;
                        if nat_eq(a270, a271) {
                            Ok::<(List<u64>, List<u64>), R<List<u64>>>((v4, List::cons(v3, v2)))
                        } else {
                            Ok::<(List<u64>, List<u64>), R<List<u64>>>((v4, f56(v0.clone(), v3, v5, v2, List::<u64>::nil())?))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m272 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f51(v0: (Bytes, Bytes), p277: u64, p278: List<u64>) -> R<List<u64>> {
    let m276 = (p277, p278);
    {
        let mut m276 = m276;
        loop {
            let step = {
                let (v1, v2) = m276;
                let m275 = v1;
                if m275 == 0 {
                    Err::<(u64, List<u64>), R<List<u64>>>(Ok(v2))
                } else {
                    let v3 = m275 - 1;
                    Ok::<(u64, List<u64>), R<List<u64>>>((v3, f57(v0.clone(), v2)?))
                }
            };
            match step {
                Ok(next) => { m276 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f52() -> u64 {
    3u64
}

pub fn f53(p296: u64, v1: (Bytes, Bytes), v2: u64, v3: u64, v4: u64, v5: u64, p297: u64, p298: u64) -> R<u64> {
    let m295 = (p296, (p297, p298));
    {
        let mut m295 = m295;
        loop {
            let step = {
                let (v0, (v6, v7)) = m295;
                let m279 = v0;
                if m279 == 0 {
                    Err::<(u64, (u64, u64)), R<u64>>(Ok(v6))
                } else {
                    let v8 = m279 - 1;
                    let a282 = {
                        let a280 = v7;
                        let a281 = v6;
                        nat_sub(a280, a281)
                    };
                    let a283 = 2u64;
                    if nat_lt(a282, a283) {
                        Err::<(u64, (u64, u64)), R<u64>>(Ok(v6))
                    } else {
                        let v9: u64 = {
                            let a286 = {
                                let a284 = v6;
                                let a285 = v7;
                                nat_add(a284, a285)?
                            };
                            let a287 = 2u64;
                            let a288 = 0u64;
                            nat_quot(a286, a287, a288)
                        };
                        let a293 = v5;
                        let a294 = f44(v1.clone(), {
                            let a291 = v2;
                            let a292 = {
                                let a289 = v9;
                                let a290 = v3;
                                nat_mul(a289, a290)?
                            };
                            nat_add(a291, a292)?
                        }, v4, 0u64)?;
                        if nat_lt(a293, a294) {
                            Ok::<(u64, (u64, u64)), R<u64>>((v8, (v6, v9)))
                        } else {
                            Ok::<(u64, (u64, u64)), R<u64>>((v8, (v9, v7)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m295 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f54() -> u64 {
    16384u64
}

pub fn f55(v0: u64) -> bool {
    let a299 = v0;
    let a300 = 44032u64;
    if nat_lt(a299, a300) {
        false
    } else {
        let a301 = v0;
        let a302 = 55204u64;
        nat_lt(a301, a302)
    }
}

pub fn f56(v0: (Bytes, Bytes), v1: u64, v2: u64, p309: List<u64>, p310: List<u64>) -> R<List<u64>> {
    let m308 = (p309, p310);
    {
        let mut m308 = m308;
        loop {
            let step = {
                let (v3, v4) = m308;
                let m303 = v3;
                match m303.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), R<List<u64>>>(Ok(f24(v4, List::cons(v1, List::<u64>::nil()))))
                    }
                    Some((v5, v6)) => {
                        let v7: u64 = f35(v0.clone(), v5)?;
                        let a304 = v7;
                        let a305 = 0u64;
                        if nat_eq(a304, a305) {
                            Err::<(List<u64>, List<u64>), R<List<u64>>>(Ok(f24(v4, List::cons(v1, List::cons(v5, v6)))))
                        } else {
                            let a306 = v2;
                            let a307 = v7;
                            if nat_lt(a306, a307) {
                                Ok::<(List<u64>, List<u64>), R<List<u64>>>((v6, List::cons(v5, v4)))
                            } else {
                                Err::<(List<u64>, List<u64>), R<List<u64>>>(Ok(f24(v4, List::cons(v1, List::cons(v5, v6)))))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m308 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f57(v0: (Bytes, Bytes), v1: List<u64>) -> R<List<u64>> {
    Ok(f24(f58(v0, v1, List::<u64>::nil())?, List::<u64>::nil()))
}

pub fn f58(v0: (Bytes, Bytes), p313: List<u64>, p314: List<u64>) -> R<List<u64>> {
    let m312 = (p313, p314);
    {
        let mut m312 = m312;
        loop {
            let step = {
                let (v1, v2) = m312;
                let m311 = v1;
                match m311.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), R<List<u64>>>(Ok(v2))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<u64>, List<u64>), R<List<u64>>>((v4, f24(f59(v0.clone(), v3)?, v2)))
                    }
                }
            };
            match step {
                Ok(next) => { m312 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f59(v0: (Bytes, Bytes), v1: u64) -> R<List<u64>> {
    if f55(v1) {
        f60(v1)
    } else {
        let v2: u64 = f61(v0.clone(), v1)?;
        let a315 = v2;
        let a316 = 0u64;
        if nat_eq(a315, a316) {
            let m317 = f62(v0, v1)?;
            match m317.uncons() {
                None => {
                    Ok(List::cons(v1, List::<u64>::nil()))
                }
                Some((v3, v4)) => {
                    Ok(List::cons(v3, v4))
                }
            }
        } else {
            Ok(List::cons(v2, List::<u64>::nil()))
        }
    }
}

pub fn f60(v0: u64) -> R<List<u64>> {
    let v1: u64 = {
        let a318 = v0;
        let a319 = 44032u64;
        nat_sub(a318, a319)
    };
    let v2: u64 = {
        let a323 = 4352u64;
        let a324 = {
            let a320 = v1;
            let a321 = 588u64;
            let a322 = 0u64;
            nat_quot(a320, a321, a322)
        };
        nat_add(a323, a324)?
    };
    let v3: u64 = {
        let a331 = 4449u64;
        let a332 = {
            let a328 = {
                let a325 = v1;
                let a326 = 588u64;
                let a327 = 0u64;
                nat_rem(a325, a326, a327)
            };
            let a329 = 28u64;
            let a330 = 0u64;
            nat_quot(a328, a329, a330)
        };
        nat_add(a331, a332)?
    };
    let v4: u64 = {
        let a336 = 4519u64;
        let a337 = {
            let a333 = v1;
            let a334 = 28u64;
            let a335 = 0u64;
            nat_rem(a333, a334, a335)
        };
        nat_add(a336, a337)?
    };
    let a338 = v4;
    let a339 = 4519u64;
    if nat_eq(a338, a339) {
        Ok(List::cons(v2, List::cons(v3, List::<u64>::nil())))
    } else {
        Ok(List::cons(v2, List::cons(v3, List::cons(v4, List::<u64>::nil()))))
    }
}

pub fn f61(v0: (Bytes, Bytes), v1: u64) -> R<u64> {
    let a340 = v1;
    let a341 = f21();
    if nat_lt(a340, a341) {
        Ok(0u64)
    } else {
        let v2: u64 = f41(v0.clone(), f42(v0.clone(), 2u64)?, 6u64, f43(v0.clone(), 2u64)?, 3u64, v1)?;
        let a342 = f44(v0.clone(), v2, 3u64, 0u64)?;
        let a343 = v1;
        if nat_eq(a342, a343) {
            f44(v0, {
                let a344 = v2;
                let a345 = 3u64;
                nat_add(a344, a345)?
            }, 3u64, 0u64)
        } else {
            Ok(0u64)
        }
    }
}

pub fn f62(v0: (Bytes, Bytes), v1: u64) -> R<List<u64>> {
    let a346 = v1;
    let a347 = f63();
    if nat_lt(a346, a347) {
        Ok(List::<u64>::nil())
    } else {
        let v2: u64 = f41(v0.clone(), f42(v0.clone(), 3u64)?, 9u64, f43(v0.clone(), 3u64)?, 3u64, v1)?;
        let a348 = f44(v0.clone(), v2, 3u64, 0u64)?;
        let a349 = v1;
        if nat_eq(a348, a349) {
            Ok(List::cons(f44(v0.clone(), {
                let a350 = v2;
                let a351 = 3u64;
                nat_add(a350, a351)?
            }, 3u64, 0u64)?, List::cons(f44(v0, {
                let a352 = v2;
                let a353 = 6u64;
                nat_add(a352, a353)?
            }, 3u64, 0u64)?, List::<u64>::nil())))
        } else {
            Ok(List::<u64>::nil())
        }
    }
}

pub fn f63() -> u64 {
    192u64
}

pub fn run(p0: Bytes, p1: Bytes, p2: List<Bytes>, p3: Bytes) -> R<List<u64>> {
    f0(p0, p1, p2, p3)
}
