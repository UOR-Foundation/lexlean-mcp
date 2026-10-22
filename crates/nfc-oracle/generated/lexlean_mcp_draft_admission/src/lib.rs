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
    F33(Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: (List<u64>, u64)) -> R<(List<u64>, u64)> {
        match self {
            Fn0::F33(k0) => f33(k0.clone(), p0),
        }
    }
}

pub fn f0(v0: Bytes, v1: Bytes, v2: u64, v3: u64, v4: List<Bytes>, v5: List<(Bytes, Bytes)>) -> R<List<u64>> {
    Ok(f1(f2((v0, v1), v2, v3, v4, v5)?))
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

pub fn f2(v0: (Bytes, Bytes), v1: u64, v2: u64, v3: List<Bytes>, v4: List<(Bytes, Bytes)>) -> R<Adt0> {
    f4(f5(f6(v0, v3, v4.clone())?, f5(f7(v1, v4.clone())?, f8(v2, v4.clone())?)), v4)
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

pub fn f4(v0: Adt0, v1: List<(Bytes, Bytes)>) -> R<Adt0> {
    let m3 = v0;
    match m3 {
        Adt0::C0 => {
            f9(v1)
        }
        Adt0::C1(v2) => {
            Ok(Adt0::C1(v2))
        }
    }
}

pub fn f5(v0: Adt0, v1: Adt0) -> Adt0 {
    let m4 = v0;
    match m4 {
        Adt0::C0 => {
            v1
        }
        Adt0::C1(v2) => {
            Adt0::C1(v2)
        }
    }
}

pub fn f6(v0: (Bytes, Bytes), v1: List<Bytes>, p9: List<(Bytes, Bytes)>) -> R<Adt0> {
    let m8 = p9;
    {
        let mut m8 = m8;
        loop {
            let step = {
                let v2 = m8;
                let m5 = v2;
                match m5.uncons() {
                    None => {
                        Err::<List<(Bytes, Bytes)>, R<Adt0>>(Ok(Adt0::C0))
                    }
                    Some((v3, v4)) => {
                        let v5: Adt0 = f10(v0.clone(), v1.clone(), {
                            let (h6, _) = v3;
                            h6
                        })?;
                        let m7 = v5;
                        match m7 {
                            Adt0::C0 => {
                                Ok::<List<(Bytes, Bytes)>, R<Adt0>>(v4)
                            }
                            Adt0::C1(v6) => {
                                Err::<List<(Bytes, Bytes)>, R<Adt0>>(Ok(Adt0::C1(v6)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m8 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f7(v0: u64, v1: List<(Bytes, Bytes)>) -> R<Adt0> {
    Ok(f11(Adt1::C8, {
        let a10 = v0;
        let a11 = f12(v1)?;
        nat_lt(a10, a11)
    }))
}

pub fn f8(v0: u64, v1: List<(Bytes, Bytes)>) -> R<Adt0> {
    Ok(f11(Adt1::C9, {
        let a12 = v0;
        let a13 = f13(v1)?;
        nat_lt(a12, a13)
    }))
}

pub fn f9(v0: List<(Bytes, Bytes)>) -> R<Adt0> {
    Ok(f11(Adt1::C10, f14(f15(v0, List::<List<u64>>::nil())?)))
}

pub fn f10(v0: (Bytes, Bytes), v1: List<Bytes>, v2: Bytes) -> R<Adt0> {
    Ok(f5(f16(v2.clone()), f5(f17(v0, v2.clone())?, f18(f19(v1, List::<List<u64>>::nil())?, f20(v2)?))))
}

pub fn f11(v0: Adt1, v1: bool) -> Adt0 {
    let m14 = v1;
    if m14 {
        Adt0::C1(v0)
    } else {
        Adt0::C0
    }
}

pub fn f12(v0: List<(Bytes, Bytes)>) -> R<u64> {
    f21(v0, 0u64)
}

pub fn f13(v0: List<(Bytes, Bytes)>) -> R<u64> {
    f22(v0, 0u64)
}

pub fn f14(p17: List<List<u64>>) -> bool {
    let m16 = p17;
    {
        let mut m16 = m16;
        loop {
            let step = {
                let v0 = m16;
                let m15 = v0;
                match m15.uncons() {
                    None => {
                        Err::<List<List<u64>>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        if f23(v1, v2.clone()) {
                            Err::<List<List<u64>>, bool>(true)
                        } else {
                            Ok::<List<List<u64>>, bool>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m16 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f15(p21: List<(Bytes, Bytes)>, p22: List<List<u64>>) -> R<List<List<u64>>> {
    let m20 = (p21, p22);
    {
        let mut m20 = m20;
        loop {
            let step = {
                let (v0, v1) = m20;
                let m18 = v0;
                match m18.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<List<u64>>), R<List<List<u64>>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<List<u64>>), R<List<List<u64>>>>((v3, List::cons(f20({
                            let (h19, _) = v2;
                            h19
                        })?, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m20 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f16(v0: Bytes) -> Adt0 {
    let m24 = {
        let a23 = v0;
        utf8_decode(a23)
    };
    match m24 {
        None => {
            Adt0::C1(Adt1::C0)
        }
        Some(_) => {
            Adt0::C0
        }
    }
}

pub fn f17(v0: (Bytes, Bytes), v1: Bytes) -> R<Adt0> {
    if f24(v0, f25(f20(v1)?)?)? {
        Ok(Adt0::C0)
    } else {
        Ok(Adt0::C1(Adt1::C1))
    }
}

pub fn f18(v0: List<List<u64>>, v1: List<u64>) -> Adt0 {
    f5(f26(v1.clone()), f5(f27(f28(v1.clone())), f5(f29(v0, v1.clone()), f30(v1))))
}

pub fn f19(p27: List<Bytes>, p28: List<List<u64>>) -> R<List<List<u64>>> {
    let m26 = (p27, p28);
    {
        let mut m26 = m26;
        loop {
            let step = {
                let (v0, v1) = m26;
                let m25 = v0;
                match m25.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<List<u64>>), R<List<List<u64>>>>(Ok(f31(v1, List::<List<u64>>::nil())))
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<List<u64>>), R<List<List<u64>>>>((v3, List::cons(f20(v2)?, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m26 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f20(v0: Bytes) -> R<List<u64>> {
    let (h29, _) = f32(Fn0::F33(v0.clone()), {
        let a30 = v0.clone();
        length_bytes(a30)
    }, (List::<u64>::nil(), {
        let a31 = v0;
        length_bytes(a31)
    }))?;
    Ok(h29)
}

pub fn f21(p36: List<(Bytes, Bytes)>, p37: u64) -> R<u64> {
    let m35 = (p36, p37);
    {
        let mut m35 = m35;
        loop {
            let step = {
                let (v0, v1) = m35;
                let m32 = v0;
                match m32.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, u64), R<u64>>(Ok(v1))
                    }
                    Some((_, v3)) => {
                        Ok::<(List<(Bytes, Bytes)>, u64), R<u64>>((v3, {
                            let a33 = v1;
                            let a34 = 1u64;
                            nat_add(a33, a34)?
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m35 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f22(p44: List<(Bytes, Bytes)>, p45: u64) -> R<u64> {
    let m43 = (p44, p45);
    {
        let mut m43 = m43;
        loop {
            let step = {
                let (v0, v1) = m43;
                let m38 = v0;
                match m38.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, u64), R<u64>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Bytes, Bytes)>, u64), R<u64>>((v3, {
                            let a41 = v1;
                            let a42 = {
                                let a40 = {
                                    let (_, h39) = v2;
                                    h39
                                };
                                length_bytes(a40)
                            };
                            nat_add(a41, a42)?
                        }))
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

pub fn f23(v0: List<u64>, p48: List<List<u64>>) -> bool {
    let m47 = p48;
    {
        let mut m47 = m47;
        loop {
            let step = {
                let v1 = m47;
                let m46 = v1;
                match m46.uncons() {
                    None => {
                        Err::<List<List<u64>>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f34(v0.clone(), v2) {
                            Err::<List<List<u64>>, bool>(true)
                        } else {
                            Ok::<List<List<u64>>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m47 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f24(v0: (Bytes, Bytes), v1: List<u64>) -> R<bool> {
    if f35(f36(), v1.clone()) {
        Ok(true)
    } else {
        Ok(f34(v1.clone(), f37(v0, v1)?))
    }
}

pub fn f25(v0: List<u64>) -> R<List<u64>> {
    Ok(f38(f39(v0, 0u64, 0u64, List::<u64>::nil())?, List::<u64>::nil()))
}

pub fn f26(v0: List<u64>) -> Adt0 {
    let m49 = v0;
    match m49.uncons() {
        None => {
            Adt0::C0
        }
        Some((v1, _)) => {
            let a50 = v1;
            let a51 = 47u64;
            if nat_eq(a50, a51) {
                Adt0::C1(Adt1::C2)
            } else {
                Adt0::C0
            }
        }
    }
}

pub fn f27(p55: List<List<u64>>) -> Adt0 {
    let m54 = p55;
    {
        let mut m54 = m54;
        loop {
            let step = {
                let v0 = m54;
                let m52 = v0;
                match m52.uncons() {
                    None => {
                        Err::<List<List<u64>>, Adt0>(Adt0::C0)
                    }
                    Some((v1, v2)) => {
                        let m53 = f40(v1);
                        match m53 {
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
                Ok(next) => { m54 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f28(v0: List<u64>) -> List<List<u64>> {
    f31(f41(v0, List::<u64>::nil(), List::<List<u64>>::nil()), List::<List<u64>>::nil())
}

pub fn f29(p58: List<List<u64>>, v1: List<u64>) -> Adt0 {
    let m57 = p58;
    {
        let mut m57 = m57;
        loop {
            let step = {
                let v0 = m57;
                let m56 = v0;
                match m56.uncons() {
                    None => {
                        Err::<List<List<u64>>, Adt0>(Adt0::C1(Adt1::C6))
                    }
                    Some((v2, v3)) => {
                        if f42(v2, v1.clone()) {
                            Err::<List<List<u64>>, Adt0>(Adt0::C0)
                        } else {
                            Ok::<List<List<u64>>, Adt0>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m57 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f30(v0: List<u64>) -> Adt0 {
    let m59 = f43(v0, List::cons(46u64, List::cons(108u64, List::cons(101u64, List::cons(120u64, List::cons(46u64, List::cons(116u64, List::cons(101u64, List::cons(120u64, List::<u64>::nil())))))))));
    if m59 {
        Adt0::C0
    } else {
        Adt0::C1(Adt1::C7)
    }
}

pub fn f31(p62: List<List<u64>>, p63: List<List<u64>>) -> List<List<u64>> {
    let m61 = (p62, p63);
    {
        let mut m61 = m61;
        loop {
            let step = {
                let (v0, v1) = m61;
                let m60 = v0;
                match m60.uncons() {
                    None => {
                        Err::<(List<List<u64>>, List<List<u64>>), List<List<u64>>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<u64>>, List<List<u64>>), List<List<u64>>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m61 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f32(v0: Fn0, p67: u64, p68: (List<u64>, u64)) -> R<(List<u64>, u64)> {
    let m66 = (p67, p68);
    {
        let mut m66 = m66;
        loop {
            let step = {
                let (v1, v2) = m66;
                let m64 = v1;
                if m64 == 0 {
                    Err::<(u64, (List<u64>, u64)), R<(List<u64>, u64)>>(Ok(v2))
                } else {
                    let v3 = m64 - 1;
                    Ok::<(u64, (List<u64>, u64)), R<(List<u64>, u64)>>((v3, {
                        let c65: Fn0 = v0.clone();
                        c65.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m66 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f33(v0: Bytes, v1: (List<u64>, u64)) -> R<(List<u64>, u64)> {
    Ok((List::cons(f44(v0, {
        let a70 = {
            let (_, h69) = v1.clone();
            h69
        };
        let a71 = 1u64;
        nat_sub(a70, a71)
    })?, {
        let (h72, _) = v1;
        h72
    }), {
        let a74 = {
            let (_, h73) = v1;
            h73
        };
        let a75 = 1u64;
        nat_sub(a74, a75)
    }))
}

pub fn f34(p82: List<u64>, p83: List<u64>) -> bool {
    let m81 = (p82, p83);
    {
        let mut m81 = m81;
        loop {
            let step = {
                let (v0, v1) = m81;
                let m76 = v0;
                match m76.uncons() {
                    None => {
                        let m77 = v1;
                        match m77.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m78 = v1;
                        match m78.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                            Some((v6, v7)) => {
                                let a79 = v4;
                                let a80 = v6;
                                if nat_eq(a79, a80) {
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
                Ok(next) => { m81 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f35(v0: u64, p88: List<u64>) -> bool {
    let m87 = p88;
    {
        let mut m87 = m87;
        loop {
            let step = {
                let v1 = m87;
                let m84 = v1;
                match m84.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        let a85 = v2;
                        let a86 = v0;
                        if nat_lt(a85, a86) {
                            Ok::<List<u64>, bool>(v3)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m87 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f36() -> u64 {
    768u64
}

pub fn f37(v0: (Bytes, Bytes), v1: List<u64>) -> R<List<u64>> {
    f45(v0.clone(), f46(v0, v1)?, false, 0u64, List::<u64>::nil(), List::<u64>::nil())
}

pub fn f38(p91: List<u64>, p92: List<u64>) -> List<u64> {
    let m90 = (p91, p92);
    {
        let mut m90 = m90;
        loop {
            let step = {
                let (v0, v1) = m90;
                let m89 = v0;
                match m89.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
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

pub fn f39(p119: List<u64>, p120: u64, p121: u64, p122: List<u64>) -> R<List<u64>> {
    let m118 = (p119, (p120, (p121, p122)));
    {
        let mut m118 = m118;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m118;
                let m93 = v0;
                match m93.uncons() {
                    None => {
                        Err::<(List<u64>, (u64, (u64, List<u64>))), R<List<u64>>>(Ok(v3))
                    }
                    Some((v4, v5)) => {
                        let a94 = v2;
                        let a95 = 0u64;
                        if nat_eq(a94, a95) {
                            let a96 = v4;
                            let a97 = 128u64;
                            if nat_lt(a96, a97) {
                                Ok::<(List<u64>, (u64, (u64, List<u64>))), R<List<u64>>>((v5, (0u64, (0u64, List::cons(v4, v3)))))
                            } else {
                                let a98 = v4;
                                let a99 = 224u64;
                                if nat_lt(a98, a99) {
                                    Ok::<(List<u64>, (u64, (u64, List<u64>))), R<List<u64>>>((v5, ({
                                        let a100 = v4;
                                        let a101 = 192u64;
                                        nat_sub(a100, a101)
                                    }, (1u64, v3))))
                                } else {
                                    let a102 = v4;
                                    let a103 = 240u64;
                                    if nat_lt(a102, a103) {
                                        Ok::<(List<u64>, (u64, (u64, List<u64>))), R<List<u64>>>((v5, ({
                                            let a104 = v4;
                                            let a105 = 224u64;
                                            nat_sub(a104, a105)
                                        }, (2u64, v3))))
                                    } else {
                                        Ok::<(List<u64>, (u64, (u64, List<u64>))), R<List<u64>>>((v5, ({
                                            let a106 = v4;
                                            let a107 = 240u64;
                                            nat_sub(a106, a107)
                                        }, (3u64, v3))))
                                    }
                                }
                            }
                        } else {
                            let v6: u64 = {
                                let a112 = {
                                    let a108 = v1;
                                    let a109 = 64u64;
                                    nat_mul(a108, a109)?
                                };
                                let a113 = {
                                    let a110 = v4;
                                    let a111 = 128u64;
                                    nat_sub(a110, a111)
                                };
                                nat_add(a112, a113)?
                            };
                            let a114 = v2;
                            let a115 = 1u64;
                            if nat_eq(a114, a115) {
                                Ok::<(List<u64>, (u64, (u64, List<u64>))), R<List<u64>>>((v5, (0u64, (0u64, List::cons(v6, v3)))))
                            } else {
                                Ok::<(List<u64>, (u64, (u64, List<u64>))), R<List<u64>>>((v5, (v6, ({
                                    let a116 = v2;
                                    let a117 = 1u64;
                                    nat_sub(a116, a117)
                                }, v3))))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m118 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f40(v0: List<u64>) -> Adt0 {
    f47(f34(v0.clone(), List::<u64>::nil()), f34(v0.clone(), List::cons(46u64, List::<u64>::nil())), f34(v0, List::cons(46u64, List::cons(46u64, List::<u64>::nil()))))
}

pub fn f41(p127: List<u64>, p128: List<u64>, p129: List<List<u64>>) -> List<List<u64>> {
    let m126 = (p127, (p128, p129));
    {
        let mut m126 = m126;
        loop {
            let step = {
                let (v0, (v1, v2)) = m126;
                let m123 = v0;
                match m123.uncons() {
                    None => {
                        Err::<(List<u64>, (List<u64>, List<List<u64>>)), List<List<u64>>>(List::cons(f38(v1, List::<u64>::nil()), v2))
                    }
                    Some((v3, v4)) => {
                        let a124 = v3;
                        let a125 = 47u64;
                        if nat_eq(a124, a125) {
                            Ok::<(List<u64>, (List<u64>, List<List<u64>>)), List<List<u64>>>((v4, (List::<u64>::nil(), List::cons(f38(v1, List::<u64>::nil()), v2))))
                        } else {
                            Ok::<(List<u64>, (List<u64>, List<List<u64>>)), List<List<u64>>>((v4, (List::cons(v3, v1), v2)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m126 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f42(p138: List<u64>, p139: List<u64>) -> bool {
    let m137 = (p138, p139);
    {
        let mut m137 = m137;
        loop {
            let step = {
                let (v0, v1) = m137;
                let m130 = v0;
                match m130.uncons() {
                    None => {
                        let m131 = v1;
                        match m131.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                            Some((v2, _)) => {
                                let a132 = v2;
                                let a133 = 47u64;
                                Err::<(List<u64>, List<u64>), bool>(nat_eq(a132, a133))
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m134 = v1;
                        match m134.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                            Some((v6, v7)) => {
                                let a135 = v4;
                                let a136 = v6;
                                if nat_eq(a135, a136) {
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
                Ok(next) => { m137 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f43(p142: List<u64>, v1: List<u64>) -> bool {
    let m141 = p142;
    {
        let mut m141 = m141;
        loop {
            let step = {
                let v0 = m141;
                let m140 = v0;
                match m140.uncons() {
                    None => {
                        Err::<List<u64>, bool>(f34(List::<u64>::nil(), v1.clone()))
                    }
                    Some((v2, v3)) => {
                        if f34(List::cons(v2, v3.clone()), v1.clone()) {
                            Err::<List<u64>, bool>(true)
                        } else {
                            Ok::<List<u64>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m141 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f44(v0: Bytes, v1: u64) -> R<u64> {
    let m145 = {
        let a143 = v0;
        let a144 = v1;
        index_bytes(a143, a144)
    };
    match m145 {
        None => {
            Ok(0u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a190 = {
                let a166 = {
                    let a154 = {
                        let a148 = {
                            let a146 = v3;
                            let a147 = 128u8;
                            fixed_u8::bit_and(a146, a147)
                        };
                        let a149 = 128u8;
                        if equal(a148, a149) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a155 = {
                        let a152 = {
                            let a150 = v3;
                            let a151 = 64u8;
                            fixed_u8::bit_and(a150, a151)
                        };
                        let a153 = 64u8;
                        if equal(a152, a153) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a154, a155)?
                };
                let a167 = {
                    let a164 = {
                        let a158 = {
                            let a156 = v3;
                            let a157 = 32u8;
                            fixed_u8::bit_and(a156, a157)
                        };
                        let a159 = 32u8;
                        if equal(a158, a159) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a165 = {
                        let a162 = {
                            let a160 = v3;
                            let a161 = 16u8;
                            fixed_u8::bit_and(a160, a161)
                        };
                        let a163 = 16u8;
                        if equal(a162, a163) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a164, a165)?
                };
                nat_add(a166, a167)?
            };
            let a191 = {
                let a188 = {
                    let a176 = {
                        let a170 = {
                            let a168 = v3;
                            let a169 = 8u8;
                            fixed_u8::bit_and(a168, a169)
                        };
                        let a171 = 8u8;
                        if equal(a170, a171) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a177 = {
                        let a174 = {
                            let a172 = v3;
                            let a173 = 4u8;
                            fixed_u8::bit_and(a172, a173)
                        };
                        let a175 = 4u8;
                        if equal(a174, a175) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a176, a177)?
                };
                let a189 = {
                    let a186 = {
                        let a180 = {
                            let a178 = v3;
                            let a179 = 2u8;
                            fixed_u8::bit_and(a178, a179)
                        };
                        let a181 = 2u8;
                        if equal(a180, a181) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a187 = {
                        let a184 = {
                            let a182 = v3;
                            let a183 = 1u8;
                            fixed_u8::bit_and(a182, a183)
                        };
                        let a185 = 1u8;
                        if equal(a184, a185) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a186, a187)?
                };
                nat_add(a188, a189)?
            };
            nat_add(a190, a191)
        }
    }
}

pub fn f45(v0: (Bytes, Bytes), p200: List<u64>, p201: bool, p202: u64, p203: List<u64>, p204: List<u64>) -> R<List<u64>> {
    let m199 = (p200, (p201, (p202, (p203, p204))));
    {
        let mut m199 = m199;
        loop {
            let step = {
                let (v1, (v2, (v3, (v4, v5)))) = m199;
                let m192 = v1;
                match m192.uncons() {
                    None => {
                        if v2 {
                            Err::<(List<u64>, (bool, (u64, (List<u64>, List<u64>)))), R<List<u64>>>(Ok(f38(f48(v3, v4, v5), List::<u64>::nil())))
                        } else {
                            Err::<(List<u64>, (bool, (u64, (List<u64>, List<u64>)))), R<List<u64>>>(Ok(f38(v5, List::<u64>::nil())))
                        }
                    }
                    Some((v6, v7)) => {
                        let v8: u64 = f49(v0.clone(), v6)?;
                        if v2 {
                            let v9: u64 = f50(f51(v0.clone(), v4.clone(), v8)?, f52(v0.clone(), v3, v6)?);
                            let a193 = v9;
                            let a194 = 0u64;
                            if nat_eq(a193, a194) {
                                let a195 = v8;
                                let a196 = 0u64;
                                if nat_eq(a195, a196) {
                                    Ok::<(List<u64>, (bool, (u64, (List<u64>, List<u64>)))), R<List<u64>>>((v7, (true, (v6, (List::<u64>::nil(), f48(v3, v4, v5))))))
                                } else {
                                    Ok::<(List<u64>, (bool, (u64, (List<u64>, List<u64>)))), R<List<u64>>>((v7, (true, (v3, (List::cons(v6, v4), v5)))))
                                }
                            } else {
                                Ok::<(List<u64>, (bool, (u64, (List<u64>, List<u64>)))), R<List<u64>>>((v7, (true, (v9, (v4, v5)))))
                            }
                        } else {
                            let a197 = v8;
                            let a198 = 0u64;
                            if nat_eq(a197, a198) {
                                Ok::<(List<u64>, (bool, (u64, (List<u64>, List<u64>)))), R<List<u64>>>((v7, (true, (v6, (List::<u64>::nil(), v5)))))
                            } else {
                                Ok::<(List<u64>, (bool, (u64, (List<u64>, List<u64>)))), R<List<u64>>>((v7, (false, (0u64, (List::<u64>::nil(), List::cons(v6, v5))))))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m199 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f46(v0: (Bytes, Bytes), v1: List<u64>) -> R<List<u64>> {
    f53(v0.clone(), f54(v0, v1)?)
}

pub fn f47(v0: bool, v1: bool, v2: bool) -> Adt0 {
    let m205 = v0;
    if m205 {
        Adt0::C1(Adt1::C3)
    } else {
        let m206 = v1;
        if m206 {
            Adt0::C1(Adt1::C4)
        } else {
            let m207 = v2;
            if m207 {
                Adt0::C1(Adt1::C5)
            } else {
                Adt0::C0
            }
        }
    }
}

pub fn f48(v0: u64, v1: List<u64>, v2: List<u64>) -> List<u64> {
    f38(f38(v1, List::<u64>::nil()), List::cons(v0, v2))
}

pub fn f49(v0: (Bytes, Bytes), v1: u64) -> R<u64> {
    let a208 = v1;
    let a209 = f36();
    if nat_lt(a208, a209) {
        Ok(0u64)
    } else {
        let v2: u64 = f55(v0.clone(), f56(v0.clone(), 0u64)?, 7u64, f57(v0.clone(), 0u64)?, 3u64, v1)?;
        let a210 = v1;
        let a211 = f58(v0.clone(), v2, 3u64, 0u64)?;
        if nat_lt(a210, a211) {
            Ok(0u64)
        } else {
            let a214 = f58(v0.clone(), {
                let a212 = v2;
                let a213 = 3u64;
                nat_add(a212, a213)?
            }, 3u64, 0u64)?;
            let a215 = v1;
            if nat_lt(a214, a215) {
                Ok(0u64)
            } else {
                f59(v0, {
                    let a216 = v2;
                    let a217 = 6u64;
                    nat_add(a216, a217)?
                })
            }
        }
    }
}

pub fn f50(v0: bool, v1: u64) -> u64 {
    if v0 {
        0u64
    } else {
        v1
    }
}

pub fn f51(v0: (Bytes, Bytes), v1: List<u64>, v2: u64) -> R<bool> {
    let m218 = v1;
    match m218.uncons() {
        None => {
            Ok(false)
        }
        Some((v3, _)) => {
            let a219 = f49(v0, v3)?;
            let a220 = v2;
            Ok(!nat_lt(a219, a220))
        }
    }
}

pub fn f52(v0: (Bytes, Bytes), v1: u64, v2: u64) -> R<u64> {
    Ok(f60(f61(v1, v2)?, f62(v0.clone(), v1, v2)?, f63(v0.clone(), f62(v0, v1, v2)?)?))
}

pub fn f53(v0: (Bytes, Bytes), v1: List<u64>) -> R<List<u64>> {
    Ok(f38(f64(v0, v1, List::<u64>::nil())?, List::<u64>::nil()))
}

pub fn f54(v0: (Bytes, Bytes), v1: List<u64>) -> R<List<u64>> {
    f65(v0, f66(), v1)
}

pub fn f55(v0: (Bytes, Bytes), v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<u64> {
    let a223 = v1;
    let a224 = {
        let a221 = f67(16u64, v0, v1, v2, v4, v5, 0u64, v3)?;
        let a222 = v2;
        nat_mul(a221, a222)?
    };
    nat_add(a223, a224)
}

pub fn f56(v0: (Bytes, Bytes), v1: u64) -> R<u64> {
    f58(v0, {
        let a225 = v1;
        let a226 = 6u64;
        nat_mul(a225, a226)?
    }, 3u64, 0u64)
}

pub fn f57(v0: (Bytes, Bytes), v1: u64) -> R<u64> {
    f58(v0, {
        let a229 = {
            let a227 = v1;
            let a228 = 6u64;
            nat_mul(a227, a228)?
        };
        let a230 = 3u64;
        nat_add(a229, a230)?
    }, 3u64, 0u64)
}

pub fn f58(v0: (Bytes, Bytes), p239: u64, p240: u64, p241: u64) -> R<u64> {
    let m238 = (p239, (p240, p241));
    {
        let mut m238 = m238;
        loop {
            let step = {
                let (v1, (v2, v3)) = m238;
                let m231 = v2;
                if m231 == 0 {
                    Err::<(u64, (u64, u64)), R<u64>>(Ok(v3))
                } else {
                    let v4 = m231 - 1;
                    Ok::<(u64, (u64, u64)), R<u64>>(({
                        let a232 = v1;
                        let a233 = 1u64;
                        nat_add(a232, a233)?
                    }, (v4, {
                        let a236 = {
                            let a234 = v3;
                            let a235 = 256u64;
                            nat_mul(a234, a235)?
                        };
                        let a237 = f59(v0.clone(), v1)?;
                        nat_add(a236, a237)?
                    })))
                }
            };
            match step {
                Ok(next) => { m238 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f59(v0: (Bytes, Bytes), v1: u64) -> R<u64> {
    let a242 = v1;
    let a243 = f68();
    if nat_lt(a242, a243) {
        f44({
            let (h244, _) = v0;
            h244
        }, v1)
    } else {
        f44({
            let (_, h245) = v0;
            h245
        }, {
            let a246 = v1;
            let a247 = f68();
            nat_sub(a246, a247)
        })
    }
}

pub fn f60(v0: u64, v1: u64, v2: bool) -> u64 {
    let a248 = v0;
    let a249 = 0u64;
    if nat_eq(a248, a249) {
        let a250 = v1;
        let a251 = 0u64;
        if nat_eq(a250, a251) {
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

pub fn f61(v0: u64, v1: u64) -> R<u64> {
    if f69(v0) {
        let a257 = {
            let a254 = {
                let a252 = v0;
                let a253 = 44032u64;
                nat_sub(a252, a253)
            };
            let a255 = 28u64;
            let a256 = 0u64;
            nat_rem(a254, a255, a256)
        };
        let a258 = 0u64;
        if nat_eq(a257, a258) {
            let a259 = v1;
            let a260 = 4520u64;
            if nat_lt(a259, a260) {
                Ok(0u64)
            } else {
                let a261 = v1;
                let a262 = 4547u64;
                if nat_lt(a261, a262) {
                    let a265 = v0;
                    let a266 = {
                        let a263 = v1;
                        let a264 = 4519u64;
                        nat_sub(a263, a264)
                    };
                    nat_add(a265, a266)
                } else {
                    Ok(0u64)
                }
            }
        } else {
            Ok(0u64)
        }
    } else {
        let a267 = v0;
        let a268 = 4352u64;
        if nat_lt(a267, a268) {
            Ok(0u64)
        } else {
            let a269 = v0;
            let a270 = 4371u64;
            if nat_lt(a269, a270) {
                let a271 = v1;
                let a272 = 4449u64;
                if nat_lt(a271, a272) {
                    Ok(0u64)
                } else {
                    let a273 = v1;
                    let a274 = 4470u64;
                    if nat_lt(a273, a274) {
                        let a285 = 44032u64;
                        let a286 = {
                            let a283 = {
                                let a281 = {
                                    let a277 = {
                                        let a275 = v0;
                                        let a276 = 4352u64;
                                        nat_sub(a275, a276)
                                    };
                                    let a278 = 21u64;
                                    nat_mul(a277, a278)?
                                };
                                let a282 = {
                                    let a279 = v1;
                                    let a280 = 4449u64;
                                    nat_sub(a279, a280)
                                };
                                nat_add(a281, a282)?
                            };
                            let a284 = 28u64;
                            nat_mul(a283, a284)?
                        };
                        nat_add(a285, a286)
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

pub fn f62(v0: (Bytes, Bytes), v1: u64, v2: u64) -> R<u64> {
    let a287 = v2;
    let a288 = f36();
    if nat_lt(a287, a288) {
        Ok(0u64)
    } else {
        let v3: u64 = {
            let a291 = {
                let a289 = v1;
                let a290 = 16777216u64;
                nat_mul(a289, a290)?
            };
            let a292 = v2;
            nat_add(a291, a292)?
        };
        let v4: u64 = f55(v0.clone(), f56(v0.clone(), 4u64)?, 9u64, f57(v0.clone(), 4u64)?, 6u64, v3)?;
        let a293 = f58(v0.clone(), v4, 6u64, 0u64)?;
        let a294 = v3;
        if nat_eq(a293, a294) {
            f58(v0, {
                let a295 = v4;
                let a296 = 6u64;
                nat_add(a295, a296)?
            }, 3u64, 0u64)
        } else {
            Ok(0u64)
        }
    }
}

pub fn f63(v0: (Bytes, Bytes), v1: u64) -> R<bool> {
    let a297 = v1;
    let a298 = f36();
    if nat_lt(a297, a298) {
        Ok(false)
    } else {
        let v2: u64 = f55(v0.clone(), f56(v0.clone(), 1u64)?, 6u64, f57(v0.clone(), 1u64)?, 3u64, v1)?;
        let a299 = v1;
        let a300 = f58(v0.clone(), v2, 3u64, 0u64)?;
        if nat_lt(a299, a300) {
            Ok(false)
        } else {
            let a303 = f58(v0, {
                let a301 = v2;
                let a302 = 3u64;
                nat_add(a301, a302)?
            }, 3u64, 0u64)?;
            let a304 = v1;
            Ok(!nat_lt(a303, a304))
        }
    }
}

pub fn f64(v0: (Bytes, Bytes), p309: List<u64>, p310: List<u64>) -> R<List<u64>> {
    let m308 = (p309, p310);
    {
        let mut m308 = m308;
        loop {
            let step = {
                let (v1, v2) = m308;
                let m305 = v1;
                match m305.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), R<List<u64>>>(Ok(v2))
                    }
                    Some((v3, v4)) => {
                        let v5: u64 = f49(v0.clone(), v3)?;
                        let a306 = v5;
                        let a307 = 0u64;
                        if nat_eq(a306, a307) {
                            Ok::<(List<u64>, List<u64>), R<List<u64>>>((v4, List::cons(v3, v2)))
                        } else {
                            Ok::<(List<u64>, List<u64>), R<List<u64>>>((v4, f70(v0.clone(), v3, v5, v2, List::<u64>::nil())?))
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

pub fn f65(v0: (Bytes, Bytes), p313: u64, p314: List<u64>) -> R<List<u64>> {
    let m312 = (p313, p314);
    {
        let mut m312 = m312;
        loop {
            let step = {
                let (v1, v2) = m312;
                let m311 = v1;
                if m311 == 0 {
                    Err::<(u64, List<u64>), R<List<u64>>>(Ok(v2))
                } else {
                    let v3 = m311 - 1;
                    Ok::<(u64, List<u64>), R<List<u64>>>((v3, f71(v0.clone(), v2)?))
                }
            };
            match step {
                Ok(next) => { m312 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f66() -> u64 {
    3u64
}

pub fn f67(p332: u64, v1: (Bytes, Bytes), v2: u64, v3: u64, v4: u64, v5: u64, p333: u64, p334: u64) -> R<u64> {
    let m331 = (p332, (p333, p334));
    {
        let mut m331 = m331;
        loop {
            let step = {
                let (v0, (v6, v7)) = m331;
                let m315 = v0;
                if m315 == 0 {
                    Err::<(u64, (u64, u64)), R<u64>>(Ok(v6))
                } else {
                    let v8 = m315 - 1;
                    let a318 = {
                        let a316 = v7;
                        let a317 = v6;
                        nat_sub(a316, a317)
                    };
                    let a319 = 2u64;
                    if nat_lt(a318, a319) {
                        Err::<(u64, (u64, u64)), R<u64>>(Ok(v6))
                    } else {
                        let v9: u64 = {
                            let a322 = {
                                let a320 = v6;
                                let a321 = v7;
                                nat_add(a320, a321)?
                            };
                            let a323 = 2u64;
                            let a324 = 0u64;
                            nat_quot(a322, a323, a324)
                        };
                        let a329 = v5;
                        let a330 = f58(v1.clone(), {
                            let a327 = v2;
                            let a328 = {
                                let a325 = v9;
                                let a326 = v3;
                                nat_mul(a325, a326)?
                            };
                            nat_add(a327, a328)?
                        }, v4, 0u64)?;
                        if nat_lt(a329, a330) {
                            Ok::<(u64, (u64, u64)), R<u64>>((v8, (v6, v9)))
                        } else {
                            Ok::<(u64, (u64, u64)), R<u64>>((v8, (v9, v7)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m331 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f68() -> u64 {
    16384u64
}

pub fn f69(v0: u64) -> bool {
    let a335 = v0;
    let a336 = 44032u64;
    if nat_lt(a335, a336) {
        false
    } else {
        let a337 = v0;
        let a338 = 55204u64;
        nat_lt(a337, a338)
    }
}

pub fn f70(v0: (Bytes, Bytes), v1: u64, v2: u64, p345: List<u64>, p346: List<u64>) -> R<List<u64>> {
    let m344 = (p345, p346);
    {
        let mut m344 = m344;
        loop {
            let step = {
                let (v3, v4) = m344;
                let m339 = v3;
                match m339.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), R<List<u64>>>(Ok(f38(v4, List::cons(v1, List::<u64>::nil()))))
                    }
                    Some((v5, v6)) => {
                        let v7: u64 = f49(v0.clone(), v5)?;
                        let a340 = v7;
                        let a341 = 0u64;
                        if nat_eq(a340, a341) {
                            Err::<(List<u64>, List<u64>), R<List<u64>>>(Ok(f38(v4, List::cons(v1, List::cons(v5, v6)))))
                        } else {
                            let a342 = v2;
                            let a343 = v7;
                            if nat_lt(a342, a343) {
                                Ok::<(List<u64>, List<u64>), R<List<u64>>>((v6, List::cons(v5, v4)))
                            } else {
                                Err::<(List<u64>, List<u64>), R<List<u64>>>(Ok(f38(v4, List::cons(v1, List::cons(v5, v6)))))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m344 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f71(v0: (Bytes, Bytes), v1: List<u64>) -> R<List<u64>> {
    Ok(f38(f72(v0, v1, List::<u64>::nil())?, List::<u64>::nil()))
}

pub fn f72(v0: (Bytes, Bytes), p349: List<u64>, p350: List<u64>) -> R<List<u64>> {
    let m348 = (p349, p350);
    {
        let mut m348 = m348;
        loop {
            let step = {
                let (v1, v2) = m348;
                let m347 = v1;
                match m347.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), R<List<u64>>>(Ok(v2))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<u64>, List<u64>), R<List<u64>>>((v4, f38(f73(v0.clone(), v3)?, v2)))
                    }
                }
            };
            match step {
                Ok(next) => { m348 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f73(v0: (Bytes, Bytes), v1: u64) -> R<List<u64>> {
    if f69(v1) {
        f74(v1)
    } else {
        let v2: u64 = f75(v0.clone(), v1)?;
        let a351 = v2;
        let a352 = 0u64;
        if nat_eq(a351, a352) {
            let m353 = f76(v0, v1)?;
            match m353.uncons() {
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

pub fn f74(v0: u64) -> R<List<u64>> {
    let v1: u64 = {
        let a354 = v0;
        let a355 = 44032u64;
        nat_sub(a354, a355)
    };
    let v2: u64 = {
        let a359 = 4352u64;
        let a360 = {
            let a356 = v1;
            let a357 = 588u64;
            let a358 = 0u64;
            nat_quot(a356, a357, a358)
        };
        nat_add(a359, a360)?
    };
    let v3: u64 = {
        let a367 = 4449u64;
        let a368 = {
            let a364 = {
                let a361 = v1;
                let a362 = 588u64;
                let a363 = 0u64;
                nat_rem(a361, a362, a363)
            };
            let a365 = 28u64;
            let a366 = 0u64;
            nat_quot(a364, a365, a366)
        };
        nat_add(a367, a368)?
    };
    let v4: u64 = {
        let a372 = 4519u64;
        let a373 = {
            let a369 = v1;
            let a370 = 28u64;
            let a371 = 0u64;
            nat_rem(a369, a370, a371)
        };
        nat_add(a372, a373)?
    };
    let a374 = v4;
    let a375 = 4519u64;
    if nat_eq(a374, a375) {
        Ok(List::cons(v2, List::cons(v3, List::<u64>::nil())))
    } else {
        Ok(List::cons(v2, List::cons(v3, List::cons(v4, List::<u64>::nil()))))
    }
}

pub fn f75(v0: (Bytes, Bytes), v1: u64) -> R<u64> {
    let a376 = v1;
    let a377 = f36();
    if nat_lt(a376, a377) {
        Ok(0u64)
    } else {
        let v2: u64 = f55(v0.clone(), f56(v0.clone(), 2u64)?, 6u64, f57(v0.clone(), 2u64)?, 3u64, v1)?;
        let a378 = f58(v0.clone(), v2, 3u64, 0u64)?;
        let a379 = v1;
        if nat_eq(a378, a379) {
            f58(v0, {
                let a380 = v2;
                let a381 = 3u64;
                nat_add(a380, a381)?
            }, 3u64, 0u64)
        } else {
            Ok(0u64)
        }
    }
}

pub fn f76(v0: (Bytes, Bytes), v1: u64) -> R<List<u64>> {
    let a382 = v1;
    let a383 = f77();
    if nat_lt(a382, a383) {
        Ok(List::<u64>::nil())
    } else {
        let v2: u64 = f55(v0.clone(), f56(v0.clone(), 3u64)?, 9u64, f57(v0.clone(), 3u64)?, 3u64, v1)?;
        let a384 = f58(v0.clone(), v2, 3u64, 0u64)?;
        let a385 = v1;
        if nat_eq(a384, a385) {
            Ok(List::cons(f58(v0.clone(), {
                let a386 = v2;
                let a387 = 3u64;
                nat_add(a386, a387)?
            }, 3u64, 0u64)?, List::cons(f58(v0, {
                let a388 = v2;
                let a389 = 6u64;
                nat_add(a388, a389)?
            }, 3u64, 0u64)?, List::<u64>::nil())))
        } else {
            Ok(List::<u64>::nil())
        }
    }
}

pub fn f77() -> u64 {
    192u64
}

pub fn run(p0: Bytes, p1: Bytes, p2: u64, p3: u64, p4: List<Bytes>, p5: List<(Bytes, Bytes)>) -> R<List<u64>> {
    f0(p0, p1, p2, p3, p4, p5)
}
