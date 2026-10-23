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
    Ok(f7(f18(v0, v1, List::<(u64, u64)>::nil(), List::<u64>::nil())?, List::<u64>::nil()))
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

pub fn f18(v0: (Bytes, Bytes), p135: List<u64>, p136: List<(u64, u64)>, p137: List<u64>) -> R<List<u64>> {
    let m134 = (p135, (p136, p137));
    {
        let mut m134 = m134;
        loop {
            let step = {
                let (v1, (v2, v3)) = m134;
                let m131 = v1;
                match m131.uncons() {
                    None => {
                        Err::<(List<u64>, (List<(u64, u64)>, List<u64>)), R<List<u64>>>(Ok(f26(f27(v2)?, v3)))
                    }
                    Some((v4, v5)) => {
                        let v6: u64 = f22(v0.clone(), v4)?;
                        let a132 = v6;
                        let a133 = 0u64;
                        if nat_eq(a132, a133) {
                            Ok::<(List<u64>, (List<(u64, u64)>, List<u64>)), R<List<u64>>>((v5, (List::<(u64, u64)>::nil(), List::cons(v4, f26(f27(v2)?, v3)))))
                        } else {
                            Ok::<(List<u64>, (List<(u64, u64)>, List<u64>)), R<List<u64>>>((v5, (List::cons((v6, v4), v2), v3)))
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

pub fn f19(v0: (Bytes, Bytes), p140: u64, p141: List<u64>) -> R<List<u64>> {
    let m139 = (p140, p141);
    {
        let mut m139 = m139;
        loop {
            let step = {
                let (v1, v2) = m139;
                let m138 = v1;
                if m138 == 0 {
                    Err::<(u64, List<u64>), R<List<u64>>>(Ok(v2))
                } else {
                    let v3 = m138 - 1;
                    Ok::<(u64, List<u64>), R<List<u64>>>((v3, f28(v0.clone(), v2)?))
                }
            };
            match step {
                Ok(next) => { m139 = next; }
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
    let a142 = v1;
    let a143 = f12();
    if nat_lt(a142, a143) {
        Ok(0u64)
    } else {
        let v2: u64 = f29(v0.clone(), f30(v0.clone(), 0u64)?, 7u64, f31(v0.clone(), 0u64)?, 3u64, v1)?;
        let a144 = v1;
        let a145 = f32(v0.clone(), v2, 3u64, 0u64)?;
        if nat_lt(a144, a145) {
            Ok(0u64)
        } else {
            let a148 = f32(v0.clone(), {
                let a146 = v2;
                let a147 = 3u64;
                nat_add(a146, a147)?
            }, 3u64, 0u64)?;
            let a149 = v1;
            if nat_lt(a148, a149) {
                Ok(0u64)
            } else {
                f33(v0, {
                    let a150 = v2;
                    let a151 = 6u64;
                    nat_add(a150, a151)?
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
    let m152 = v1;
    match m152.uncons() {
        None => {
            Ok(false)
        }
        Some((v3, _)) => {
            let a153 = f22(v0, v3)?;
            let a154 = v2;
            Ok(!nat_lt(a153, a154))
        }
    }
}

pub fn f25(v0: (Bytes, Bytes), v1: u64, v2: u64) -> R<u64> {
    Ok(f34(f35(v1, v2)?, f36(v0.clone(), v1, v2)?, f37(v0.clone(), f36(v0, v1, v2)?)?))
}

pub fn f26(p158: List<(u64, u64)>, p159: List<u64>) -> List<u64> {
    let m157 = (p158, p159);
    {
        let mut m157 = m157;
        loop {
            let step = {
                let (v0, v1) = m157;
                let m155 = v0;
                match m155.uncons() {
                    None => {
                        Err::<(List<(u64, u64)>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(u64, u64)>, List<u64>), List<u64>>((v3, List::cons({
                            let (_, h156) = v2;
                            h156
                        }, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m157 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f27(v0: List<(u64, u64)>) -> R<List<(u64, u64)>> {
    let v1: u64 = f38(v0.clone(), 0u64)?;
    Ok(f39({
        let a160 = v1;
        let a161 = 1u64;
        nat_add(a160, a161)?
    }, {
        let a162 = v1;
        let a163 = 1u64;
        nat_add(a162, a163)?
    }, f40(v0, List::<List<(u64, u64)>>::nil())))
}

pub fn f28(v0: (Bytes, Bytes), v1: List<u64>) -> R<List<u64>> {
    Ok(f7(f41(v0, v1, List::<u64>::nil())?, List::<u64>::nil()))
}

pub fn f29(v0: (Bytes, Bytes), v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<u64> {
    let a166 = v1;
    let a167 = {
        let a164 = f42(16u64, v0, v1, v2, v4, v5, 0u64, v3)?;
        let a165 = v2;
        nat_mul(a164, a165)?
    };
    nat_add(a166, a167)
}

pub fn f30(v0: (Bytes, Bytes), v1: u64) -> R<u64> {
    f32(v0, {
        let a168 = v1;
        let a169 = 6u64;
        nat_mul(a168, a169)?
    }, 3u64, 0u64)
}

pub fn f31(v0: (Bytes, Bytes), v1: u64) -> R<u64> {
    f32(v0, {
        let a172 = {
            let a170 = v1;
            let a171 = 6u64;
            nat_mul(a170, a171)?
        };
        let a173 = 3u64;
        nat_add(a172, a173)?
    }, 3u64, 0u64)
}

pub fn f32(v0: (Bytes, Bytes), p182: u64, p183: u64, p184: u64) -> R<u64> {
    let m181 = (p182, (p183, p184));
    {
        let mut m181 = m181;
        loop {
            let step = {
                let (v1, (v2, v3)) = m181;
                let m174 = v2;
                if m174 == 0 {
                    Err::<(u64, (u64, u64)), R<u64>>(Ok(v3))
                } else {
                    let v4 = m174 - 1;
                    Ok::<(u64, (u64, u64)), R<u64>>(({
                        let a175 = v1;
                        let a176 = 1u64;
                        nat_add(a175, a176)?
                    }, (v4, {
                        let a179 = {
                            let a177 = v3;
                            let a178 = 256u64;
                            nat_mul(a177, a178)?
                        };
                        let a180 = f33(v0.clone(), v1)?;
                        nat_add(a179, a180)?
                    })))
                }
            };
            match step {
                Ok(next) => { m181 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f33(v0: (Bytes, Bytes), v1: u64) -> R<u64> {
    let a185 = v1;
    let a186 = f43();
    if nat_lt(a185, a186) {
        f17({
            let (h187, _) = v0;
            h187
        }, v1)
    } else {
        f17({
            let (_, h188) = v0;
            h188
        }, {
            let a189 = v1;
            let a190 = f43();
            nat_sub(a189, a190)
        })
    }
}

pub fn f34(v0: u64, v1: u64, v2: bool) -> u64 {
    let a191 = v0;
    let a192 = 0u64;
    if nat_eq(a191, a192) {
        let a193 = v1;
        let a194 = 0u64;
        if nat_eq(a193, a194) {
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

pub fn f35(v0: u64, v1: u64) -> R<u64> {
    if f44(v0) {
        let a200 = {
            let a197 = {
                let a195 = v0;
                let a196 = 44032u64;
                nat_sub(a195, a196)
            };
            let a198 = 28u64;
            let a199 = 0u64;
            nat_rem(a197, a198, a199)
        };
        let a201 = 0u64;
        if nat_eq(a200, a201) {
            let a202 = v1;
            let a203 = 4520u64;
            if nat_lt(a202, a203) {
                Ok(0u64)
            } else {
                let a204 = v1;
                let a205 = 4547u64;
                if nat_lt(a204, a205) {
                    let a208 = v0;
                    let a209 = {
                        let a206 = v1;
                        let a207 = 4519u64;
                        nat_sub(a206, a207)
                    };
                    nat_add(a208, a209)
                } else {
                    Ok(0u64)
                }
            }
        } else {
            Ok(0u64)
        }
    } else {
        let a210 = v0;
        let a211 = 4352u64;
        if nat_lt(a210, a211) {
            Ok(0u64)
        } else {
            let a212 = v0;
            let a213 = 4371u64;
            if nat_lt(a212, a213) {
                let a214 = v1;
                let a215 = 4449u64;
                if nat_lt(a214, a215) {
                    Ok(0u64)
                } else {
                    let a216 = v1;
                    let a217 = 4470u64;
                    if nat_lt(a216, a217) {
                        let a228 = 44032u64;
                        let a229 = {
                            let a226 = {
                                let a224 = {
                                    let a220 = {
                                        let a218 = v0;
                                        let a219 = 4352u64;
                                        nat_sub(a218, a219)
                                    };
                                    let a221 = 21u64;
                                    nat_mul(a220, a221)?
                                };
                                let a225 = {
                                    let a222 = v1;
                                    let a223 = 4449u64;
                                    nat_sub(a222, a223)
                                };
                                nat_add(a224, a225)?
                            };
                            let a227 = 28u64;
                            nat_mul(a226, a227)?
                        };
                        nat_add(a228, a229)
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

pub fn f36(v0: (Bytes, Bytes), v1: u64, v2: u64) -> R<u64> {
    let a230 = v2;
    let a231 = f12();
    if nat_lt(a230, a231) {
        Ok(0u64)
    } else {
        let v3: u64 = {
            let a234 = {
                let a232 = v1;
                let a233 = 16777216u64;
                nat_mul(a232, a233)?
            };
            let a235 = v2;
            nat_add(a234, a235)?
        };
        let v4: u64 = f29(v0.clone(), f30(v0.clone(), 4u64)?, 9u64, f31(v0.clone(), 4u64)?, 6u64, v3)?;
        let a236 = f32(v0.clone(), v4, 6u64, 0u64)?;
        let a237 = v3;
        if nat_eq(a236, a237) {
            f32(v0, {
                let a238 = v4;
                let a239 = 6u64;
                nat_add(a238, a239)?
            }, 3u64, 0u64)
        } else {
            Ok(0u64)
        }
    }
}

pub fn f37(v0: (Bytes, Bytes), v1: u64) -> R<bool> {
    let a240 = v1;
    let a241 = f12();
    if nat_lt(a240, a241) {
        Ok(false)
    } else {
        let v2: u64 = f29(v0.clone(), f30(v0.clone(), 1u64)?, 6u64, f31(v0.clone(), 1u64)?, 3u64, v1)?;
        let a242 = v1;
        let a243 = f32(v0.clone(), v2, 3u64, 0u64)?;
        if nat_lt(a242, a243) {
            Ok(false)
        } else {
            let a246 = f32(v0, {
                let a244 = v2;
                let a245 = 3u64;
                nat_add(a244, a245)?
            }, 3u64, 0u64)?;
            let a247 = v1;
            Ok(!nat_lt(a246, a247))
        }
    }
}

pub fn f38(p252: List<(u64, u64)>, p253: u64) -> R<u64> {
    let m251 = (p252, p253);
    {
        let mut m251 = m251;
        loop {
            let step = {
                let (v0, v1) = m251;
                let m248 = v0;
                match m248.uncons() {
                    None => {
                        Err::<(List<(u64, u64)>, u64), R<u64>>(Ok(v1))
                    }
                    Some((_, v3)) => {
                        Ok::<(List<(u64, u64)>, u64), R<u64>>((v3, {
                            let a249 = v1;
                            let a250 = 1u64;
                            nat_add(a249, a250)?
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m251 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f39(p259: u64, v1: u64, p260: List<List<(u64, u64)>>) -> List<(u64, u64)> {
    let m258 = (p259, p260);
    {
        let mut m258 = m258;
        loop {
            let step = {
                let (v0, v2) = m258;
                let m254 = v0;
                if m254 == 0 {
                    let m255 = v2;
                    match m255.uncons() {
                        None => {
                            Err::<(u64, List<List<(u64, u64)>>), List<(u64, u64)>>(List::<(u64, u64)>::nil())
                        }
                        Some((v3, _)) => {
                            Err::<(u64, List<List<(u64, u64)>>), List<(u64, u64)>>(v3)
                        }
                    }
                } else {
                    let v5 = m254 - 1;
                    let m256 = v2.clone();
                    match m256.uncons() {
                        None => {
                            Err::<(u64, List<List<(u64, u64)>>), List<(u64, u64)>>(List::<(u64, u64)>::nil())
                        }
                        Some((v6, v7)) => {
                            let m257 = v7;
                            match m257.uncons() {
                                None => {
                                    Err::<(u64, List<List<(u64, u64)>>), List<(u64, u64)>>(v6)
                                }
                                Some((_, _)) => {
                                    Ok::<(u64, List<List<(u64, u64)>>), List<(u64, u64)>>((v5, f45(v1, v2, List::<List<(u64, u64)>>::nil(), List::<List<(u64, u64)>>::nil())))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m258 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f40(p263: List<(u64, u64)>, p264: List<List<(u64, u64)>>) -> List<List<(u64, u64)>> {
    let m262 = (p263, p264);
    {
        let mut m262 = m262;
        loop {
            let step = {
                let (v0, v1) = m262;
                let m261 = v0;
                match m261.uncons() {
                    None => {
                        Err::<(List<(u64, u64)>, List<List<(u64, u64)>>), List<List<(u64, u64)>>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(u64, u64)>, List<List<(u64, u64)>>), List<List<(u64, u64)>>>((v3, List::cons(List::cons(v2, List::<(u64, u64)>::nil()), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m262 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f41(v0: (Bytes, Bytes), p267: List<u64>, p268: List<u64>) -> R<List<u64>> {
    let m266 = (p267, p268);
    {
        let mut m266 = m266;
        loop {
            let step = {
                let (v1, v2) = m266;
                let m265 = v1;
                match m265.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), R<List<u64>>>(Ok(v2))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<u64>, List<u64>), R<List<u64>>>((v4, f7(f46(v0.clone(), v3)?, v2)))
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

pub fn f42(p286: u64, v1: (Bytes, Bytes), v2: u64, v3: u64, v4: u64, v5: u64, p287: u64, p288: u64) -> R<u64> {
    let m285 = (p286, (p287, p288));
    {
        let mut m285 = m285;
        loop {
            let step = {
                let (v0, (v6, v7)) = m285;
                let m269 = v0;
                if m269 == 0 {
                    Err::<(u64, (u64, u64)), R<u64>>(Ok(v6))
                } else {
                    let v8 = m269 - 1;
                    let a272 = {
                        let a270 = v7;
                        let a271 = v6;
                        nat_sub(a270, a271)
                    };
                    let a273 = 2u64;
                    if nat_lt(a272, a273) {
                        Err::<(u64, (u64, u64)), R<u64>>(Ok(v6))
                    } else {
                        let v9: u64 = {
                            let a276 = {
                                let a274 = v6;
                                let a275 = v7;
                                nat_add(a274, a275)?
                            };
                            let a277 = 2u64;
                            let a278 = 0u64;
                            nat_quot(a276, a277, a278)
                        };
                        let a283 = v5;
                        let a284 = f32(v1.clone(), {
                            let a281 = v2;
                            let a282 = {
                                let a279 = v9;
                                let a280 = v3;
                                nat_mul(a279, a280)?
                            };
                            nat_add(a281, a282)?
                        }, v4, 0u64)?;
                        if nat_lt(a283, a284) {
                            Ok::<(u64, (u64, u64)), R<u64>>((v8, (v6, v9)))
                        } else {
                            Ok::<(u64, (u64, u64)), R<u64>>((v8, (v9, v7)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m285 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f43() -> u64 {
    16384u64
}

pub fn f44(v0: u64) -> bool {
    let a289 = v0;
    let a290 = 44032u64;
    if nat_lt(a289, a290) {
        false
    } else {
        let a291 = v0;
        let a292 = 55204u64;
        nat_lt(a291, a292)
    }
}

pub fn f45(v0: u64, p297: List<List<(u64, u64)>>, p298: List<List<(u64, u64)>>, p299: List<List<(u64, u64)>>) -> List<List<(u64, u64)>> {
    let m296 = (p297, (p298, p299));
    {
        let mut m296 = m296;
        loop {
            let step = {
                let (v1, (v2, v3)) = m296;
                let m293 = v1;
                match m293.uncons() {
                    None => {
                        let m294 = v2;
                        match m294.uncons() {
                            None => {
                                Err::<(List<List<(u64, u64)>>, (List<List<(u64, u64)>>, List<List<(u64, u64)>>)), List<List<(u64, u64)>>>(f47(v3, List::<List<(u64, u64)>>::nil()))
                            }
                            Some((v4, _)) => {
                                Err::<(List<List<(u64, u64)>>, (List<List<(u64, u64)>>, List<List<(u64, u64)>>)), List<List<(u64, u64)>>>(f47(List::cons(v4, v3), List::<List<(u64, u64)>>::nil()))
                            }
                        }
                    }
                    Some((v6, v7)) => {
                        let m295 = v2;
                        match m295.uncons() {
                            None => {
                                Ok::<(List<List<(u64, u64)>>, (List<List<(u64, u64)>>, List<List<(u64, u64)>>)), List<List<(u64, u64)>>>((v7, (List::cons(v6, List::<List<(u64, u64)>>::nil()), v3)))
                            }
                            Some((v8, _)) => {
                                Ok::<(List<List<(u64, u64)>>, (List<List<(u64, u64)>>, List<List<(u64, u64)>>)), List<List<(u64, u64)>>>((v7, (List::<List<(u64, u64)>>::nil(), List::cons(f48(v0, v8, v6, List::<(u64, u64)>::nil()), v3))))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m296 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f46(v0: (Bytes, Bytes), v1: u64) -> R<List<u64>> {
    if f44(v1) {
        f49(v1)
    } else {
        let v2: u64 = f50(v0.clone(), v1)?;
        let a300 = v2;
        let a301 = 0u64;
        if nat_eq(a300, a301) {
            let m302 = f51(v0, v1)?;
            match m302.uncons() {
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

pub fn f47(p305: List<List<(u64, u64)>>, p306: List<List<(u64, u64)>>) -> List<List<(u64, u64)>> {
    let m304 = (p305, p306);
    {
        let mut m304 = m304;
        loop {
            let step = {
                let (v0, v1) = m304;
                let m303 = v0;
                match m303.uncons() {
                    None => {
                        Err::<(List<List<(u64, u64)>>, List<List<(u64, u64)>>), List<List<(u64, u64)>>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<(u64, u64)>>, List<List<(u64, u64)>>), List<List<(u64, u64)>>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m304 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f48(p315: u64, p316: List<(u64, u64)>, p317: List<(u64, u64)>, p318: List<(u64, u64)>) -> List<(u64, u64)> {
    let m314 = (p315, (p316, (p317, p318)));
    {
        let mut m314 = m314;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m314;
                let m307 = v0;
                if m307 == 0 {
                    Err::<(u64, (List<(u64, u64)>, (List<(u64, u64)>, List<(u64, u64)>))), List<(u64, u64)>>(f52(v3, v1))
                } else {
                    let v4 = m307 - 1;
                    let m308 = v1.clone();
                    match m308.uncons() {
                        None => {
                            Err::<(u64, (List<(u64, u64)>, (List<(u64, u64)>, List<(u64, u64)>))), List<(u64, u64)>>(f52(v3, v2))
                        }
                        Some((v5, v6)) => {
                            let m309 = v2.clone();
                            match m309.uncons() {
                                None => {
                                    Err::<(u64, (List<(u64, u64)>, (List<(u64, u64)>, List<(u64, u64)>))), List<(u64, u64)>>(f52(v3, v1))
                                }
                                Some((v7, v8)) => {
                                    let a312 = {
                                        let (h310, _) = v7;
                                        h310
                                    };
                                    let a313 = {
                                        let (h311, _) = v5;
                                        h311
                                    };
                                    if nat_lt(a312, a313) {
                                        Ok::<(u64, (List<(u64, u64)>, (List<(u64, u64)>, List<(u64, u64)>))), List<(u64, u64)>>((v4, (v1, (v8, List::cons(v7, v3)))))
                                    } else {
                                        Ok::<(u64, (List<(u64, u64)>, (List<(u64, u64)>, List<(u64, u64)>))), List<(u64, u64)>>((v4, (v6, (v2, List::cons(v5, v3)))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m314 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f49(v0: u64) -> R<List<u64>> {
    let v1: u64 = {
        let a319 = v0;
        let a320 = 44032u64;
        nat_sub(a319, a320)
    };
    let v2: u64 = {
        let a324 = 4352u64;
        let a325 = {
            let a321 = v1;
            let a322 = 588u64;
            let a323 = 0u64;
            nat_quot(a321, a322, a323)
        };
        nat_add(a324, a325)?
    };
    let v3: u64 = {
        let a332 = 4449u64;
        let a333 = {
            let a329 = {
                let a326 = v1;
                let a327 = 588u64;
                let a328 = 0u64;
                nat_rem(a326, a327, a328)
            };
            let a330 = 28u64;
            let a331 = 0u64;
            nat_quot(a329, a330, a331)
        };
        nat_add(a332, a333)?
    };
    let v4: u64 = {
        let a337 = 4519u64;
        let a338 = {
            let a334 = v1;
            let a335 = 28u64;
            let a336 = 0u64;
            nat_rem(a334, a335, a336)
        };
        nat_add(a337, a338)?
    };
    let a339 = v4;
    let a340 = 4519u64;
    if nat_eq(a339, a340) {
        Ok(List::cons(v2, List::cons(v3, List::<u64>::nil())))
    } else {
        Ok(List::cons(v2, List::cons(v3, List::cons(v4, List::<u64>::nil()))))
    }
}

pub fn f50(v0: (Bytes, Bytes), v1: u64) -> R<u64> {
    let a341 = v1;
    let a342 = f12();
    if nat_lt(a341, a342) {
        Ok(0u64)
    } else {
        let v2: u64 = f29(v0.clone(), f30(v0.clone(), 2u64)?, 6u64, f31(v0.clone(), 2u64)?, 3u64, v1)?;
        let a343 = f32(v0.clone(), v2, 3u64, 0u64)?;
        let a344 = v1;
        if nat_eq(a343, a344) {
            f32(v0, {
                let a345 = v2;
                let a346 = 3u64;
                nat_add(a345, a346)?
            }, 3u64, 0u64)
        } else {
            Ok(0u64)
        }
    }
}

pub fn f51(v0: (Bytes, Bytes), v1: u64) -> R<List<u64>> {
    let a347 = v1;
    let a348 = f53();
    if nat_lt(a347, a348) {
        Ok(List::<u64>::nil())
    } else {
        let v2: u64 = f29(v0.clone(), f30(v0.clone(), 3u64)?, 9u64, f31(v0.clone(), 3u64)?, 3u64, v1)?;
        let a349 = f32(v0.clone(), v2, 3u64, 0u64)?;
        let a350 = v1;
        if nat_eq(a349, a350) {
            Ok(List::cons(f32(v0.clone(), {
                let a351 = v2;
                let a352 = 3u64;
                nat_add(a351, a352)?
            }, 3u64, 0u64)?, List::cons(f32(v0, {
                let a353 = v2;
                let a354 = 6u64;
                nat_add(a353, a354)?
            }, 3u64, 0u64)?, List::<u64>::nil())))
        } else {
            Ok(List::<u64>::nil())
        }
    }
}

pub fn f52(p357: List<(u64, u64)>, p358: List<(u64, u64)>) -> List<(u64, u64)> {
    let m356 = (p357, p358);
    {
        let mut m356 = m356;
        loop {
            let step = {
                let (v0, v1) = m356;
                let m355 = v0;
                match m355.uncons() {
                    None => {
                        Err::<(List<(u64, u64)>, List<(u64, u64)>), List<(u64, u64)>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(u64, u64)>, List<(u64, u64)>), List<(u64, u64)>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m356 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f53() -> u64 {
    192u64
}

pub fn run(p0: u64, p1: Bytes, p2: Bytes, p3: Bytes) -> R<List<u64>> {
    f0(p0, p1, p2, p3)
}
