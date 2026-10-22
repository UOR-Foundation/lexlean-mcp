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
pub enum Fn0 {
    F10(Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: (List<u64>, u64)) -> R<(List<u64>, u64)> {
        match self {
            Fn0::F10(k0) => f10(k0.clone(), p0),
        }
    }
}

pub fn f0(v0: u64, v1: Bytes, v2: Bytes, v3: Bytes) -> R<List<u64>> {
    let v4: (Bytes, Bytes) = (v1, v2);
    let v5: List<u64> = f1(f2(v3)?)?;
    let a1 = v0;
    let a2 = 0u64;
    if nat_eq(a1, a2) {
        Ok(f3(f4(v4, v5)?))
    } else {
        let a3 = v0;
        let a4 = 1u64;
        if nat_eq(a3, a4) {
            Ok(v5)
        } else {
            let a5 = v0;
            let a6 = 2u64;
            if nat_eq(a5, a6) {
                f5(v4, v5)
            } else {
                f6(v4, v5)
            }
        }
    }
}

pub fn f1(v0: List<u64>) -> R<List<u64>> {
    Ok(f7(f8(v0, 0u64, 0u64, List::<u64>::nil())?, List::<u64>::nil()))
}

pub fn f2(v0: Bytes) -> R<List<u64>> {
    let (h7, _) = f9(Fn0::F10(v0.clone()), {
        let a8 = v0.clone();
        length_bytes(a8)
    }, (List::<u64>::nil(), {
        let a9 = v0;
        length_bytes(a9)
    }))?;
    Ok(h7)
}

pub fn f3(v0: bool) -> List<u64> {
    if v0 {
        List::cons(1u64, List::<u64>::nil())
    } else {
        List::cons(0u64, List::<u64>::nil())
    }
}

pub fn f4(v0: (Bytes, Bytes), v1: List<u64>) -> R<bool> {
    if f11(f12(), v1.clone()) {
        Ok(true)
    } else {
        Ok(f13(v1.clone(), f6(v0, v1)?))
    }
}

pub fn f5(v0: (Bytes, Bytes), v1: List<u64>) -> R<List<u64>> {
    f14(v0.clone(), f15(v0, v1)?)
}

pub fn f6(v0: (Bytes, Bytes), v1: List<u64>) -> R<List<u64>> {
    f16(v0.clone(), f5(v0, v1)?, false, 0u64, List::<u64>::nil(), List::<u64>::nil())
}

pub fn f7(p12: List<u64>, p13: List<u64>) -> List<u64> {
    let m11 = (p12, p13);
    {
        let mut m11 = m11;
        loop {
            let step = {
                let (v0, v1) = m11;
                let m10 = v0;
                match m10.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m11 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f8(p40: List<u64>, p41: u64, p42: u64, p43: List<u64>) -> R<List<u64>> {
    let m39 = (p40, (p41, (p42, p43)));
    {
        let mut m39 = m39;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m39;
                let m14 = v0;
                match m14.uncons() {
                    None => {
                        Err::<(List<u64>, (u64, (u64, List<u64>))), R<List<u64>>>(Ok(v3))
                    }
                    Some((v4, v5)) => {
                        let a15 = v2;
                        let a16 = 0u64;
                        if nat_eq(a15, a16) {
                            let a17 = v4;
                            let a18 = 128u64;
                            if nat_lt(a17, a18) {
                                Ok::<(List<u64>, (u64, (u64, List<u64>))), R<List<u64>>>((v5, (0u64, (0u64, List::cons(v4, v3)))))
                            } else {
                                let a19 = v4;
                                let a20 = 224u64;
                                if nat_lt(a19, a20) {
                                    Ok::<(List<u64>, (u64, (u64, List<u64>))), R<List<u64>>>((v5, ({
                                        let a21 = v4;
                                        let a22 = 192u64;
                                        nat_sub(a21, a22)
                                    }, (1u64, v3))))
                                } else {
                                    let a23 = v4;
                                    let a24 = 240u64;
                                    if nat_lt(a23, a24) {
                                        Ok::<(List<u64>, (u64, (u64, List<u64>))), R<List<u64>>>((v5, ({
                                            let a25 = v4;
                                            let a26 = 224u64;
                                            nat_sub(a25, a26)
                                        }, (2u64, v3))))
                                    } else {
                                        Ok::<(List<u64>, (u64, (u64, List<u64>))), R<List<u64>>>((v5, ({
                                            let a27 = v4;
                                            let a28 = 240u64;
                                            nat_sub(a27, a28)
                                        }, (3u64, v3))))
                                    }
                                }
                            }
                        } else {
                            let v6: u64 = {
                                let a33 = {
                                    let a29 = v1;
                                    let a30 = 64u64;
                                    nat_mul(a29, a30)?
                                };
                                let a34 = {
                                    let a31 = v4;
                                    let a32 = 128u64;
                                    nat_sub(a31, a32)
                                };
                                nat_add(a33, a34)?
                            };
                            let a35 = v2;
                            let a36 = 1u64;
                            if nat_eq(a35, a36) {
                                Ok::<(List<u64>, (u64, (u64, List<u64>))), R<List<u64>>>((v5, (0u64, (0u64, List::cons(v6, v3)))))
                            } else {
                                Ok::<(List<u64>, (u64, (u64, List<u64>))), R<List<u64>>>((v5, (v6, ({
                                    let a37 = v2;
                                    let a38 = 1u64;
                                    nat_sub(a37, a38)
                                }, v3))))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m39 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f9(v0: Fn0, p47: u64, p48: (List<u64>, u64)) -> R<(List<u64>, u64)> {
    let m46 = (p47, p48);
    {
        let mut m46 = m46;
        loop {
            let step = {
                let (v1, v2) = m46;
                let m44 = v1;
                if m44 == 0 {
                    Err::<(u64, (List<u64>, u64)), R<(List<u64>, u64)>>(Ok(v2))
                } else {
                    let v3 = m44 - 1;
                    Ok::<(u64, (List<u64>, u64)), R<(List<u64>, u64)>>((v3, {
                        let c45: Fn0 = v0.clone();
                        c45.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m46 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f10(v0: Bytes, v1: (List<u64>, u64)) -> R<(List<u64>, u64)> {
    Ok((List::cons(f17(v0, {
        let a50 = {
            let (_, h49) = v1.clone();
            h49
        };
        let a51 = 1u64;
        nat_sub(a50, a51)
    })?, {
        let (h52, _) = v1;
        h52
    }), {
        let a54 = {
            let (_, h53) = v1;
            h53
        };
        let a55 = 1u64;
        nat_sub(a54, a55)
    }))
}

pub fn f11(v0: u64, p60: List<u64>) -> bool {
    let m59 = p60;
    {
        let mut m59 = m59;
        loop {
            let step = {
                let v1 = m59;
                let m56 = v1;
                match m56.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        let a57 = v2;
                        let a58 = v0;
                        if nat_lt(a57, a58) {
                            Ok::<List<u64>, bool>(v3)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m59 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f12() -> u64 {
    768u64
}

pub fn f13(p67: List<u64>, p68: List<u64>) -> bool {
    let m66 = (p67, p68);
    {
        let mut m66 = m66;
        loop {
            let step = {
                let (v0, v1) = m66;
                let m61 = v0;
                match m61.uncons() {
                    None => {
                        let m62 = v1;
                        match m62.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m63 = v1;
                        match m63.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                            Some((v6, v7)) => {
                                let a64 = v4;
                                let a65 = v6;
                                if nat_eq(a64, a65) {
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
                Ok(next) => { m66 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f14(v0: (Bytes, Bytes), v1: List<u64>) -> R<List<u64>> {
    Ok(f7(f18(v0, v1, List::<u64>::nil())?, List::<u64>::nil()))
}

pub fn f15(v0: (Bytes, Bytes), v1: List<u64>) -> R<List<u64>> {
    f19(v0, f20(), v1)
}

pub fn f16(v0: (Bytes, Bytes), p77: List<u64>, p78: bool, p79: u64, p80: List<u64>, p81: List<u64>) -> R<List<u64>> {
    let m76 = (p77, (p78, (p79, (p80, p81))));
    {
        let mut m76 = m76;
        loop {
            let step = {
                let (v1, (v2, (v3, (v4, v5)))) = m76;
                let m69 = v1;
                match m69.uncons() {
                    None => {
                        if v2 {
                            Err::<(List<u64>, (bool, (u64, (List<u64>, List<u64>)))), R<List<u64>>>(Ok(f7(f21(v3, v4, v5), List::<u64>::nil())))
                        } else {
                            Err::<(List<u64>, (bool, (u64, (List<u64>, List<u64>)))), R<List<u64>>>(Ok(f7(v5, List::<u64>::nil())))
                        }
                    }
                    Some((v6, v7)) => {
                        let v8: u64 = f22(v0.clone(), v6)?;
                        if v2 {
                            let v9: u64 = f23(f24(v0.clone(), v4.clone(), v8)?, f25(v0.clone(), v3, v6)?);
                            let a70 = v9;
                            let a71 = 0u64;
                            if nat_eq(a70, a71) {
                                let a72 = v8;
                                let a73 = 0u64;
                                if nat_eq(a72, a73) {
                                    Ok::<(List<u64>, (bool, (u64, (List<u64>, List<u64>)))), R<List<u64>>>((v7, (true, (v6, (List::<u64>::nil(), f21(v3, v4, v5))))))
                                } else {
                                    Ok::<(List<u64>, (bool, (u64, (List<u64>, List<u64>)))), R<List<u64>>>((v7, (true, (v3, (List::cons(v6, v4), v5)))))
                                }
                            } else {
                                Ok::<(List<u64>, (bool, (u64, (List<u64>, List<u64>)))), R<List<u64>>>((v7, (true, (v9, (v4, v5)))))
                            }
                        } else {
                            let a74 = v8;
                            let a75 = 0u64;
                            if nat_eq(a74, a75) {
                                Ok::<(List<u64>, (bool, (u64, (List<u64>, List<u64>)))), R<List<u64>>>((v7, (true, (v6, (List::<u64>::nil(), v5)))))
                            } else {
                                Ok::<(List<u64>, (bool, (u64, (List<u64>, List<u64>)))), R<List<u64>>>((v7, (false, (0u64, (List::<u64>::nil(), List::cons(v6, v5))))))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m76 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f17(v0: Bytes, v1: u64) -> R<u64> {
    let m84 = {
        let a82 = v0;
        let a83 = v1;
        index_bytes(a82, a83)
    };
    match m84 {
        None => {
            Ok(0u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a129 = {
                let a105 = {
                    let a93 = {
                        let a87 = {
                            let a85 = v3;
                            let a86 = 128u8;
                            fixed_u8::bit_and(a85, a86)
                        };
                        let a88 = 128u8;
                        if equal(a87, a88) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a94 = {
                        let a91 = {
                            let a89 = v3;
                            let a90 = 64u8;
                            fixed_u8::bit_and(a89, a90)
                        };
                        let a92 = 64u8;
                        if equal(a91, a92) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a93, a94)?
                };
                let a106 = {
                    let a103 = {
                        let a97 = {
                            let a95 = v3;
                            let a96 = 32u8;
                            fixed_u8::bit_and(a95, a96)
                        };
                        let a98 = 32u8;
                        if equal(a97, a98) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a104 = {
                        let a101 = {
                            let a99 = v3;
                            let a100 = 16u8;
                            fixed_u8::bit_and(a99, a100)
                        };
                        let a102 = 16u8;
                        if equal(a101, a102) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a103, a104)?
                };
                nat_add(a105, a106)?
            };
            let a130 = {
                let a127 = {
                    let a115 = {
                        let a109 = {
                            let a107 = v3;
                            let a108 = 8u8;
                            fixed_u8::bit_and(a107, a108)
                        };
                        let a110 = 8u8;
                        if equal(a109, a110) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a116 = {
                        let a113 = {
                            let a111 = v3;
                            let a112 = 4u8;
                            fixed_u8::bit_and(a111, a112)
                        };
                        let a114 = 4u8;
                        if equal(a113, a114) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a115, a116)?
                };
                let a128 = {
                    let a125 = {
                        let a119 = {
                            let a117 = v3;
                            let a118 = 2u8;
                            fixed_u8::bit_and(a117, a118)
                        };
                        let a120 = 2u8;
                        if equal(a119, a120) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a126 = {
                        let a123 = {
                            let a121 = v3;
                            let a122 = 1u8;
                            fixed_u8::bit_and(a121, a122)
                        };
                        let a124 = 1u8;
                        if equal(a123, a124) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a125, a126)?
                };
                nat_add(a127, a128)?
            };
            nat_add(a129, a130)
        }
    }
}

pub fn f18(v0: (Bytes, Bytes), p135: List<u64>, p136: List<u64>) -> R<List<u64>> {
    let m134 = (p135, p136);
    {
        let mut m134 = m134;
        loop {
            let step = {
                let (v1, v2) = m134;
                let m131 = v1;
                match m131.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), R<List<u64>>>(Ok(v2))
                    }
                    Some((v3, v4)) => {
                        let v5: u64 = f22(v0.clone(), v3)?;
                        let a132 = v5;
                        let a133 = 0u64;
                        if nat_eq(a132, a133) {
                            Ok::<(List<u64>, List<u64>), R<List<u64>>>((v4, List::cons(v3, v2)))
                        } else {
                            Ok::<(List<u64>, List<u64>), R<List<u64>>>((v4, f26(v0.clone(), v3, v5, v2, List::<u64>::nil())?))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m134 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f19(v0: (Bytes, Bytes), p139: u64, p140: List<u64>) -> R<List<u64>> {
    let m138 = (p139, p140);
    {
        let mut m138 = m138;
        loop {
            let step = {
                let (v1, v2) = m138;
                let m137 = v1;
                if m137 == 0 {
                    Err::<(u64, List<u64>), R<List<u64>>>(Ok(v2))
                } else {
                    let v3 = m137 - 1;
                    Ok::<(u64, List<u64>), R<List<u64>>>((v3, f27(v0.clone(), v2)?))
                }
            };
            match step {
                Ok(next) => { m138 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f20() -> u64 {
    3u64
}

pub fn f21(v0: u64, v1: List<u64>, v2: List<u64>) -> List<u64> {
    f7(f7(v1, List::<u64>::nil()), List::cons(v0, v2))
}

pub fn f22(v0: (Bytes, Bytes), v1: u64) -> R<u64> {
    let a141 = v1;
    let a142 = f12();
    if nat_lt(a141, a142) {
        Ok(0u64)
    } else {
        let v2: u64 = f28(v0.clone(), f29(v0.clone(), 0u64)?, 7u64, f30(v0.clone(), 0u64)?, 3u64, v1)?;
        let a143 = v1;
        let a144 = f31(v0.clone(), v2, 3u64, 0u64)?;
        if nat_lt(a143, a144) {
            Ok(0u64)
        } else {
            let a147 = f31(v0.clone(), {
                let a145 = v2;
                let a146 = 3u64;
                nat_add(a145, a146)?
            }, 3u64, 0u64)?;
            let a148 = v1;
            if nat_lt(a147, a148) {
                Ok(0u64)
            } else {
                f32(v0, {
                    let a149 = v2;
                    let a150 = 6u64;
                    nat_add(a149, a150)?
                })
            }
        }
    }
}

pub fn f23(v0: bool, v1: u64) -> u64 {
    if v0 {
        0u64
    } else {
        v1
    }
}

pub fn f24(v0: (Bytes, Bytes), v1: List<u64>, v2: u64) -> R<bool> {
    let m151 = v1;
    match m151.uncons() {
        None => {
            Ok(false)
        }
        Some((v3, _)) => {
            let a152 = f22(v0, v3)?;
            let a153 = v2;
            Ok(!nat_lt(a152, a153))
        }
    }
}

pub fn f25(v0: (Bytes, Bytes), v1: u64, v2: u64) -> R<u64> {
    Ok(f33(f34(v1, v2)?, f35(v0.clone(), v1, v2)?, f36(v0.clone(), f35(v0, v1, v2)?)?))
}

pub fn f26(v0: (Bytes, Bytes), v1: u64, v2: u64, p160: List<u64>, p161: List<u64>) -> R<List<u64>> {
    let m159 = (p160, p161);
    {
        let mut m159 = m159;
        loop {
            let step = {
                let (v3, v4) = m159;
                let m154 = v3;
                match m154.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), R<List<u64>>>(Ok(f7(v4, List::cons(v1, List::<u64>::nil()))))
                    }
                    Some((v5, v6)) => {
                        let v7: u64 = f22(v0.clone(), v5)?;
                        let a155 = v7;
                        let a156 = 0u64;
                        if nat_eq(a155, a156) {
                            Err::<(List<u64>, List<u64>), R<List<u64>>>(Ok(f7(v4, List::cons(v1, List::cons(v5, v6)))))
                        } else {
                            let a157 = v2;
                            let a158 = v7;
                            if nat_lt(a157, a158) {
                                Ok::<(List<u64>, List<u64>), R<List<u64>>>((v6, List::cons(v5, v4)))
                            } else {
                                Err::<(List<u64>, List<u64>), R<List<u64>>>(Ok(f7(v4, List::cons(v1, List::cons(v5, v6)))))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m159 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f27(v0: (Bytes, Bytes), v1: List<u64>) -> R<List<u64>> {
    Ok(f7(f37(v0, v1, List::<u64>::nil())?, List::<u64>::nil()))
}

pub fn f28(v0: (Bytes, Bytes), v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<u64> {
    let a164 = v1;
    let a165 = {
        let a162 = f38(16u64, v0, v1, v2, v4, v5, 0u64, v3)?;
        let a163 = v2;
        nat_mul(a162, a163)?
    };
    nat_add(a164, a165)
}

pub fn f29(v0: (Bytes, Bytes), v1: u64) -> R<u64> {
    f31(v0, {
        let a166 = v1;
        let a167 = 6u64;
        nat_mul(a166, a167)?
    }, 3u64, 0u64)
}

pub fn f30(v0: (Bytes, Bytes), v1: u64) -> R<u64> {
    f31(v0, {
        let a170 = {
            let a168 = v1;
            let a169 = 6u64;
            nat_mul(a168, a169)?
        };
        let a171 = 3u64;
        nat_add(a170, a171)?
    }, 3u64, 0u64)
}

pub fn f31(v0: (Bytes, Bytes), p180: u64, p181: u64, p182: u64) -> R<u64> {
    let m179 = (p180, (p181, p182));
    {
        let mut m179 = m179;
        loop {
            let step = {
                let (v1, (v2, v3)) = m179;
                let m172 = v2;
                if m172 == 0 {
                    Err::<(u64, (u64, u64)), R<u64>>(Ok(v3))
                } else {
                    let v4 = m172 - 1;
                    Ok::<(u64, (u64, u64)), R<u64>>(({
                        let a173 = v1;
                        let a174 = 1u64;
                        nat_add(a173, a174)?
                    }, (v4, {
                        let a177 = {
                            let a175 = v3;
                            let a176 = 256u64;
                            nat_mul(a175, a176)?
                        };
                        let a178 = f32(v0.clone(), v1)?;
                        nat_add(a177, a178)?
                    })))
                }
            };
            match step {
                Ok(next) => { m179 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f32(v0: (Bytes, Bytes), v1: u64) -> R<u64> {
    let a183 = v1;
    let a184 = f39();
    if nat_lt(a183, a184) {
        f17({
            let (h185, _) = v0;
            h185
        }, v1)
    } else {
        f17({
            let (_, h186) = v0;
            h186
        }, {
            let a187 = v1;
            let a188 = f39();
            nat_sub(a187, a188)
        })
    }
}

pub fn f33(v0: u64, v1: u64, v2: bool) -> u64 {
    let a189 = v0;
    let a190 = 0u64;
    if nat_eq(a189, a190) {
        let a191 = v1;
        let a192 = 0u64;
        if nat_eq(a191, a192) {
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

pub fn f34(v0: u64, v1: u64) -> R<u64> {
    if f40(v0) {
        let a198 = {
            let a195 = {
                let a193 = v0;
                let a194 = 44032u64;
                nat_sub(a193, a194)
            };
            let a196 = 28u64;
            let a197 = 0u64;
            nat_rem(a195, a196, a197)
        };
        let a199 = 0u64;
        if nat_eq(a198, a199) {
            let a200 = v1;
            let a201 = 4520u64;
            if nat_lt(a200, a201) {
                Ok(0u64)
            } else {
                let a202 = v1;
                let a203 = 4547u64;
                if nat_lt(a202, a203) {
                    let a206 = v0;
                    let a207 = {
                        let a204 = v1;
                        let a205 = 4519u64;
                        nat_sub(a204, a205)
                    };
                    nat_add(a206, a207)
                } else {
                    Ok(0u64)
                }
            }
        } else {
            Ok(0u64)
        }
    } else {
        let a208 = v0;
        let a209 = 4352u64;
        if nat_lt(a208, a209) {
            Ok(0u64)
        } else {
            let a210 = v0;
            let a211 = 4371u64;
            if nat_lt(a210, a211) {
                let a212 = v1;
                let a213 = 4449u64;
                if nat_lt(a212, a213) {
                    Ok(0u64)
                } else {
                    let a214 = v1;
                    let a215 = 4470u64;
                    if nat_lt(a214, a215) {
                        let a226 = 44032u64;
                        let a227 = {
                            let a224 = {
                                let a222 = {
                                    let a218 = {
                                        let a216 = v0;
                                        let a217 = 4352u64;
                                        nat_sub(a216, a217)
                                    };
                                    let a219 = 21u64;
                                    nat_mul(a218, a219)?
                                };
                                let a223 = {
                                    let a220 = v1;
                                    let a221 = 4449u64;
                                    nat_sub(a220, a221)
                                };
                                nat_add(a222, a223)?
                            };
                            let a225 = 28u64;
                            nat_mul(a224, a225)?
                        };
                        nat_add(a226, a227)
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

pub fn f35(v0: (Bytes, Bytes), v1: u64, v2: u64) -> R<u64> {
    let a228 = v2;
    let a229 = f12();
    if nat_lt(a228, a229) {
        Ok(0u64)
    } else {
        let v3: u64 = {
            let a232 = {
                let a230 = v1;
                let a231 = 16777216u64;
                nat_mul(a230, a231)?
            };
            let a233 = v2;
            nat_add(a232, a233)?
        };
        let v4: u64 = f28(v0.clone(), f29(v0.clone(), 4u64)?, 9u64, f30(v0.clone(), 4u64)?, 6u64, v3)?;
        let a234 = f31(v0.clone(), v4, 6u64, 0u64)?;
        let a235 = v3;
        if nat_eq(a234, a235) {
            f31(v0, {
                let a236 = v4;
                let a237 = 6u64;
                nat_add(a236, a237)?
            }, 3u64, 0u64)
        } else {
            Ok(0u64)
        }
    }
}

pub fn f36(v0: (Bytes, Bytes), v1: u64) -> R<bool> {
    let a238 = v1;
    let a239 = f12();
    if nat_lt(a238, a239) {
        Ok(false)
    } else {
        let v2: u64 = f28(v0.clone(), f29(v0.clone(), 1u64)?, 6u64, f30(v0.clone(), 1u64)?, 3u64, v1)?;
        let a240 = v1;
        let a241 = f31(v0.clone(), v2, 3u64, 0u64)?;
        if nat_lt(a240, a241) {
            Ok(false)
        } else {
            let a244 = f31(v0, {
                let a242 = v2;
                let a243 = 3u64;
                nat_add(a242, a243)?
            }, 3u64, 0u64)?;
            let a245 = v1;
            Ok(!nat_lt(a244, a245))
        }
    }
}

pub fn f37(v0: (Bytes, Bytes), p248: List<u64>, p249: List<u64>) -> R<List<u64>> {
    let m247 = (p248, p249);
    {
        let mut m247 = m247;
        loop {
            let step = {
                let (v1, v2) = m247;
                let m246 = v1;
                match m246.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), R<List<u64>>>(Ok(v2))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<u64>, List<u64>), R<List<u64>>>((v4, f7(f41(v0.clone(), v3)?, v2)))
                    }
                }
            };
            match step {
                Ok(next) => { m247 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f38(p267: u64, v1: (Bytes, Bytes), v2: u64, v3: u64, v4: u64, v5: u64, p268: u64, p269: u64) -> R<u64> {
    let m266 = (p267, (p268, p269));
    {
        let mut m266 = m266;
        loop {
            let step = {
                let (v0, (v6, v7)) = m266;
                let m250 = v0;
                if m250 == 0 {
                    Err::<(u64, (u64, u64)), R<u64>>(Ok(v6))
                } else {
                    let v8 = m250 - 1;
                    let a253 = {
                        let a251 = v7;
                        let a252 = v6;
                        nat_sub(a251, a252)
                    };
                    let a254 = 2u64;
                    if nat_lt(a253, a254) {
                        Err::<(u64, (u64, u64)), R<u64>>(Ok(v6))
                    } else {
                        let v9: u64 = {
                            let a257 = {
                                let a255 = v6;
                                let a256 = v7;
                                nat_add(a255, a256)?
                            };
                            let a258 = 2u64;
                            let a259 = 0u64;
                            nat_quot(a257, a258, a259)
                        };
                        let a264 = v5;
                        let a265 = f31(v1.clone(), {
                            let a262 = v2;
                            let a263 = {
                                let a260 = v9;
                                let a261 = v3;
                                nat_mul(a260, a261)?
                            };
                            nat_add(a262, a263)?
                        }, v4, 0u64)?;
                        if nat_lt(a264, a265) {
                            Ok::<(u64, (u64, u64)), R<u64>>((v8, (v6, v9)))
                        } else {
                            Ok::<(u64, (u64, u64)), R<u64>>((v8, (v9, v7)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m266 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f39() -> u64 {
    16384u64
}

pub fn f40(v0: u64) -> bool {
    let a270 = v0;
    let a271 = 44032u64;
    if nat_lt(a270, a271) {
        false
    } else {
        let a272 = v0;
        let a273 = 55204u64;
        nat_lt(a272, a273)
    }
}

pub fn f41(v0: (Bytes, Bytes), v1: u64) -> R<List<u64>> {
    if f40(v1) {
        f42(v1)
    } else {
        let v2: u64 = f43(v0.clone(), v1)?;
        let a274 = v2;
        let a275 = 0u64;
        if nat_eq(a274, a275) {
            let m276 = f44(v0, v1)?;
            match m276.uncons() {
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

pub fn f42(v0: u64) -> R<List<u64>> {
    let v1: u64 = {
        let a277 = v0;
        let a278 = 44032u64;
        nat_sub(a277, a278)
    };
    let v2: u64 = {
        let a282 = 4352u64;
        let a283 = {
            let a279 = v1;
            let a280 = 588u64;
            let a281 = 0u64;
            nat_quot(a279, a280, a281)
        };
        nat_add(a282, a283)?
    };
    let v3: u64 = {
        let a290 = 4449u64;
        let a291 = {
            let a287 = {
                let a284 = v1;
                let a285 = 588u64;
                let a286 = 0u64;
                nat_rem(a284, a285, a286)
            };
            let a288 = 28u64;
            let a289 = 0u64;
            nat_quot(a287, a288, a289)
        };
        nat_add(a290, a291)?
    };
    let v4: u64 = {
        let a295 = 4519u64;
        let a296 = {
            let a292 = v1;
            let a293 = 28u64;
            let a294 = 0u64;
            nat_rem(a292, a293, a294)
        };
        nat_add(a295, a296)?
    };
    let a297 = v4;
    let a298 = 4519u64;
    if nat_eq(a297, a298) {
        Ok(List::cons(v2, List::cons(v3, List::<u64>::nil())))
    } else {
        Ok(List::cons(v2, List::cons(v3, List::cons(v4, List::<u64>::nil()))))
    }
}

pub fn f43(v0: (Bytes, Bytes), v1: u64) -> R<u64> {
    let a299 = v1;
    let a300 = f12();
    if nat_lt(a299, a300) {
        Ok(0u64)
    } else {
        let v2: u64 = f28(v0.clone(), f29(v0.clone(), 2u64)?, 6u64, f30(v0.clone(), 2u64)?, 3u64, v1)?;
        let a301 = f31(v0.clone(), v2, 3u64, 0u64)?;
        let a302 = v1;
        if nat_eq(a301, a302) {
            f31(v0, {
                let a303 = v2;
                let a304 = 3u64;
                nat_add(a303, a304)?
            }, 3u64, 0u64)
        } else {
            Ok(0u64)
        }
    }
}

pub fn f44(v0: (Bytes, Bytes), v1: u64) -> R<List<u64>> {
    let a305 = v1;
    let a306 = f45();
    if nat_lt(a305, a306) {
        Ok(List::<u64>::nil())
    } else {
        let v2: u64 = f28(v0.clone(), f29(v0.clone(), 3u64)?, 9u64, f30(v0.clone(), 3u64)?, 3u64, v1)?;
        let a307 = f31(v0.clone(), v2, 3u64, 0u64)?;
        let a308 = v1;
        if nat_eq(a307, a308) {
            Ok(List::cons(f31(v0.clone(), {
                let a309 = v2;
                let a310 = 3u64;
                nat_add(a309, a310)?
            }, 3u64, 0u64)?, List::cons(f31(v0, {
                let a311 = v2;
                let a312 = 6u64;
                nat_add(a311, a312)?
            }, 3u64, 0u64)?, List::<u64>::nil())))
        } else {
            Ok(List::<u64>::nil())
        }
    }
}

pub fn f45() -> u64 {
    192u64
}

pub fn run(p0: u64, p1: Bytes, p2: Bytes, p3: Bytes) -> R<List<u64>> {
    f0(p0, p1, p2, p3)
}
