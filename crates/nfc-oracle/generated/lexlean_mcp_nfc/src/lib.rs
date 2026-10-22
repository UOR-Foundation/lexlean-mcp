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
    F9(Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: (List<u64>, u64)) -> R<(List<u64>, u64)> {
        match self {
            Fn0::F9(k0) => f9(k0.clone(), p0),
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
    f7(v0, 0u64, 0u64)
}

pub fn f2(v0: Bytes) -> R<List<u64>> {
    let (h7, _) = f8(Fn0::F9(v0.clone()), {
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
    if f10(f11(), v1.clone()) {
        Ok(true)
    } else {
        Ok(f12(v1.clone(), f6(v0, v1)?))
    }
}

pub fn f5(v0: (Bytes, Bytes), v1: List<u64>) -> R<List<u64>> {
    f13(v0.clone(), f14(v0, v1)?)
}

pub fn f6(v0: (Bytes, Bytes), v1: List<u64>) -> R<List<u64>> {
    f15(v0.clone(), f5(v0, v1)?, false, 0u64, List::<u64>::nil())
}

pub fn f7(v0: List<u64>, v1: u64, v2: u64) -> R<List<u64>> {
    let m10 = v0;
    match m10.uncons() {
        None => {
            Ok(List::<u64>::nil())
        }
        Some((v3, v4)) => {
            let a11 = v2;
            let a12 = 0u64;
            if nat_eq(a11, a12) {
                let a13 = v3;
                let a14 = 128u64;
                if nat_lt(a13, a14) {
                    Ok(List::cons(v3, f7(v4, 0u64, 0u64)?))
                } else {
                    let a15 = v3;
                    let a16 = 224u64;
                    if nat_lt(a15, a16) {
                        f7(v4, {
                            let a17 = v3;
                            let a18 = 192u64;
                            nat_sub(a17, a18)
                        }, 1u64)
                    } else {
                        let a19 = v3;
                        let a20 = 240u64;
                        if nat_lt(a19, a20) {
                            f7(v4, {
                                let a21 = v3;
                                let a22 = 224u64;
                                nat_sub(a21, a22)
                            }, 2u64)
                        } else {
                            f7(v4, {
                                let a23 = v3;
                                let a24 = 240u64;
                                nat_sub(a23, a24)
                            }, 3u64)
                        }
                    }
                }
            } else {
                let v5: u64 = {
                    let a29 = {
                        let a25 = v1;
                        let a26 = 64u64;
                        nat_mul(a25, a26)?
                    };
                    let a30 = {
                        let a27 = v3;
                        let a28 = 128u64;
                        nat_sub(a27, a28)
                    };
                    nat_add(a29, a30)?
                };
                let a31 = v2;
                let a32 = 1u64;
                if nat_eq(a31, a32) {
                    Ok(List::cons(v5, f7(v4, 0u64, 0u64)?))
                } else {
                    f7(v4, v5, {
                        let a33 = v2;
                        let a34 = 1u64;
                        nat_sub(a33, a34)
                    })
                }
            }
        }
    }
}

pub fn f8(v0: Fn0, p38: u64, p39: (List<u64>, u64)) -> R<(List<u64>, u64)> {
    let m37 = (p38, p39);
    {
        let mut m37 = m37;
        loop {
            let step = {
                let (v1, v2) = m37;
                let m35 = v1;
                if m35 == 0 {
                    Err::<(u64, (List<u64>, u64)), R<(List<u64>, u64)>>(Ok(v2))
                } else {
                    let v3 = m35 - 1;
                    Ok::<(u64, (List<u64>, u64)), R<(List<u64>, u64)>>((v3, {
                        let c36: Fn0 = v0.clone();
                        c36.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m37 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f9(v0: Bytes, v1: (List<u64>, u64)) -> R<(List<u64>, u64)> {
    Ok((List::cons(f16(v0, {
        let a41 = {
            let (_, h40) = v1.clone();
            h40
        };
        let a42 = 1u64;
        nat_sub(a41, a42)
    })?, {
        let (h43, _) = v1;
        h43
    }), {
        let a45 = {
            let (_, h44) = v1;
            h44
        };
        let a46 = 1u64;
        nat_sub(a45, a46)
    }))
}

pub fn f10(v0: u64, p51: List<u64>) -> bool {
    let m50 = p51;
    {
        let mut m50 = m50;
        loop {
            let step = {
                let v1 = m50;
                let m47 = v1;
                match m47.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        let a48 = v2;
                        let a49 = v0;
                        if nat_lt(a48, a49) {
                            Ok::<List<u64>, bool>(v3)
                        } else {
                            Err::<List<u64>, bool>(false)
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

pub fn f11() -> u64 {
    768u64
}

pub fn f12(p58: List<u64>, p59: List<u64>) -> bool {
    let m57 = (p58, p59);
    {
        let mut m57 = m57;
        loop {
            let step = {
                let (v0, v1) = m57;
                let m52 = v0;
                match m52.uncons() {
                    None => {
                        let m53 = v1;
                        match m53.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m54 = v1;
                        match m54.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                            Some((v6, v7)) => {
                                let a55 = v4;
                                let a56 = v6;
                                if nat_eq(a55, a56) {
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
                Ok(next) => { m57 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f13(v0: (Bytes, Bytes), v1: List<u64>) -> R<List<u64>> {
    let m60 = v1;
    match m60.uncons() {
        None => {
            Ok(List::<u64>::nil())
        }
        Some((v2, v3)) => {
            let v4: u64 = f17(v0.clone(), v2)?;
            let a61 = v4;
            let a62 = 0u64;
            if nat_eq(a61, a62) {
                Ok(List::cons(v2, f13(v0, v3)?))
            } else {
                f18(v0.clone(), v2, v4, f13(v0, v3)?)
            }
        }
    }
}

pub fn f14(v0: (Bytes, Bytes), v1: List<u64>) -> R<List<u64>> {
    f19(v0, f20(), v1)
}

pub fn f15(v0: (Bytes, Bytes), v1: List<u64>, v2: bool, v3: u64, v4: List<u64>) -> R<List<u64>> {
    let m63 = v1;
    match m63.uncons() {
        None => {
            if v2 {
                Ok(f21(v3, v4, List::<u64>::nil()))
            } else {
                Ok(List::<u64>::nil())
            }
        }
        Some((v5, v6)) => {
            let v7: u64 = f17(v0.clone(), v5)?;
            if v2 {
                let v8: u64 = f22(f23(v0.clone(), v4.clone(), v7)?, f24(v0.clone(), v3, v5)?);
                let a64 = v8;
                let a65 = 0u64;
                if nat_eq(a64, a65) {
                    let a66 = v7;
                    let a67 = 0u64;
                    if nat_eq(a66, a67) {
                        Ok(f21(v3, v4, f15(v0, v6, true, v5, List::<u64>::nil())?))
                    } else {
                        f15(v0, v6, true, v3, List::cons(v5, v4))
                    }
                } else {
                    f15(v0, v6, true, v8, v4)
                }
            } else {
                let a68 = v7;
                let a69 = 0u64;
                if nat_eq(a68, a69) {
                    f15(v0, v6, true, v5, List::<u64>::nil())
                } else {
                    Ok(List::cons(v5, f15(v0, v6, false, 0u64, List::<u64>::nil())?))
                }
            }
        }
    }
}

pub fn f16(v0: Bytes, v1: u64) -> R<u64> {
    let m72 = {
        let a70 = v0;
        let a71 = v1;
        index_bytes(a70, a71)
    };
    match m72 {
        None => {
            Ok(0u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a117 = {
                let a93 = {
                    let a81 = {
                        let a75 = {
                            let a73 = v3;
                            let a74 = 128u8;
                            fixed_u8::bit_and(a73, a74)
                        };
                        let a76 = 128u8;
                        if equal(a75, a76) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a82 = {
                        let a79 = {
                            let a77 = v3;
                            let a78 = 64u8;
                            fixed_u8::bit_and(a77, a78)
                        };
                        let a80 = 64u8;
                        if equal(a79, a80) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a81, a82)?
                };
                let a94 = {
                    let a91 = {
                        let a85 = {
                            let a83 = v3;
                            let a84 = 32u8;
                            fixed_u8::bit_and(a83, a84)
                        };
                        let a86 = 32u8;
                        if equal(a85, a86) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a92 = {
                        let a89 = {
                            let a87 = v3;
                            let a88 = 16u8;
                            fixed_u8::bit_and(a87, a88)
                        };
                        let a90 = 16u8;
                        if equal(a89, a90) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a91, a92)?
                };
                nat_add(a93, a94)?
            };
            let a118 = {
                let a115 = {
                    let a103 = {
                        let a97 = {
                            let a95 = v3;
                            let a96 = 8u8;
                            fixed_u8::bit_and(a95, a96)
                        };
                        let a98 = 8u8;
                        if equal(a97, a98) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a104 = {
                        let a101 = {
                            let a99 = v3;
                            let a100 = 4u8;
                            fixed_u8::bit_and(a99, a100)
                        };
                        let a102 = 4u8;
                        if equal(a101, a102) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a103, a104)?
                };
                let a116 = {
                    let a113 = {
                        let a107 = {
                            let a105 = v3;
                            let a106 = 2u8;
                            fixed_u8::bit_and(a105, a106)
                        };
                        let a108 = 2u8;
                        if equal(a107, a108) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a114 = {
                        let a111 = {
                            let a109 = v3;
                            let a110 = 1u8;
                            fixed_u8::bit_and(a109, a110)
                        };
                        let a112 = 1u8;
                        if equal(a111, a112) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a113, a114)?
                };
                nat_add(a115, a116)?
            };
            nat_add(a117, a118)
        }
    }
}

pub fn f17(v0: (Bytes, Bytes), v1: u64) -> R<u64> {
    let a119 = v1;
    let a120 = f11();
    if nat_lt(a119, a120) {
        Ok(0u64)
    } else {
        let v2: u64 = f25(v0.clone(), f26(v0.clone(), 0u64)?, 7u64, f27(v0.clone(), 0u64)?, 3u64, v1)?;
        let a121 = v1;
        let a122 = f28(v0.clone(), v2, 3u64, 0u64)?;
        if nat_lt(a121, a122) {
            Ok(0u64)
        } else {
            let a125 = f28(v0.clone(), {
                let a123 = v2;
                let a124 = 3u64;
                nat_add(a123, a124)?
            }, 3u64, 0u64)?;
            let a126 = v1;
            if nat_lt(a125, a126) {
                Ok(0u64)
            } else {
                f29(v0, {
                    let a127 = v2;
                    let a128 = 6u64;
                    nat_add(a127, a128)?
                })
            }
        }
    }
}

pub fn f18(v0: (Bytes, Bytes), v1: u64, v2: u64, v3: List<u64>) -> R<List<u64>> {
    let m129 = v3;
    match m129.uncons() {
        None => {
            Ok(List::cons(v1, List::<u64>::nil()))
        }
        Some((v4, v5)) => {
            let v6: u64 = f17(v0.clone(), v4)?;
            let a130 = v6;
            let a131 = 0u64;
            if nat_eq(a130, a131) {
                Ok(List::cons(v1, List::cons(v4, v5)))
            } else {
                let a132 = v6;
                let a133 = v2;
                if nat_lt(a132, a133) {
                    Ok(List::cons(v4, f18(v0, v1, v2, v5)?))
                } else {
                    Ok(List::cons(v1, List::cons(v4, v5)))
                }
            }
        }
    }
}

pub fn f19(v0: (Bytes, Bytes), p136: u64, p137: List<u64>) -> R<List<u64>> {
    let m135 = (p136, p137);
    {
        let mut m135 = m135;
        loop {
            let step = {
                let (v1, v2) = m135;
                let m134 = v1;
                if m134 == 0 {
                    Err::<(u64, List<u64>), R<List<u64>>>(Ok(v2))
                } else {
                    let v3 = m134 - 1;
                    Ok::<(u64, List<u64>), R<List<u64>>>((v3, f30(v0.clone(), v2)?))
                }
            };
            match step {
                Ok(next) => { m135 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f20() -> u64 {
    3u64
}

pub fn f21(v0: u64, v1: List<u64>, v2: List<u64>) -> List<u64> {
    List::cons(v0, f31(v1, v2))
}

pub fn f22(v0: bool, v1: u64) -> u64 {
    if v0 {
        0u64
    } else {
        v1
    }
}

pub fn f23(v0: (Bytes, Bytes), v1: List<u64>, v2: u64) -> R<bool> {
    let m138 = v1;
    match m138.uncons() {
        None => {
            Ok(false)
        }
        Some((v3, _)) => {
            let a139 = f17(v0, v3)?;
            let a140 = v2;
            Ok(!nat_lt(a139, a140))
        }
    }
}

pub fn f24(v0: (Bytes, Bytes), v1: u64, v2: u64) -> R<u64> {
    Ok(f32(f33(v1, v2)?, f34(v0.clone(), v1, v2)?, f35(v0.clone(), f34(v0, v1, v2)?)?))
}

pub fn f25(v0: (Bytes, Bytes), v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<u64> {
    let a143 = v1;
    let a144 = {
        let a141 = f36(16u64, v0, v1, v2, v4, v5, 0u64, v3)?;
        let a142 = v2;
        nat_mul(a141, a142)?
    };
    nat_add(a143, a144)
}

pub fn f26(v0: (Bytes, Bytes), v1: u64) -> R<u64> {
    f28(v0, {
        let a145 = v1;
        let a146 = 6u64;
        nat_mul(a145, a146)?
    }, 3u64, 0u64)
}

pub fn f27(v0: (Bytes, Bytes), v1: u64) -> R<u64> {
    f28(v0, {
        let a149 = {
            let a147 = v1;
            let a148 = 6u64;
            nat_mul(a147, a148)?
        };
        let a150 = 3u64;
        nat_add(a149, a150)?
    }, 3u64, 0u64)
}

pub fn f28(v0: (Bytes, Bytes), p159: u64, p160: u64, p161: u64) -> R<u64> {
    let m158 = (p159, (p160, p161));
    {
        let mut m158 = m158;
        loop {
            let step = {
                let (v1, (v2, v3)) = m158;
                let m151 = v2;
                if m151 == 0 {
                    Err::<(u64, (u64, u64)), R<u64>>(Ok(v3))
                } else {
                    let v4 = m151 - 1;
                    Ok::<(u64, (u64, u64)), R<u64>>(({
                        let a152 = v1;
                        let a153 = 1u64;
                        nat_add(a152, a153)?
                    }, (v4, {
                        let a156 = {
                            let a154 = v3;
                            let a155 = 256u64;
                            nat_mul(a154, a155)?
                        };
                        let a157 = f29(v0.clone(), v1)?;
                        nat_add(a156, a157)?
                    })))
                }
            };
            match step {
                Ok(next) => { m158 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f29(v0: (Bytes, Bytes), v1: u64) -> R<u64> {
    let a162 = v1;
    let a163 = f37();
    if nat_lt(a162, a163) {
        f16({
            let (h164, _) = v0;
            h164
        }, v1)
    } else {
        f16({
            let (_, h165) = v0;
            h165
        }, {
            let a166 = v1;
            let a167 = f37();
            nat_sub(a166, a167)
        })
    }
}

pub fn f30(v0: (Bytes, Bytes), v1: List<u64>) -> R<List<u64>> {
    let m168 = v1;
    match m168.uncons() {
        None => {
            Ok(List::<u64>::nil())
        }
        Some((v2, v3)) => {
            Ok(f38(f39(v0.clone(), v2)?, f30(v0, v3)?))
        }
    }
}

pub fn f31(p171: List<u64>, p172: List<u64>) -> List<u64> {
    let m170 = (p171, p172);
    {
        let mut m170 = m170;
        loop {
            let step = {
                let (v0, v1) = m170;
                let m169 = v0;
                match m169.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m170 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f32(v0: u64, v1: u64, v2: bool) -> u64 {
    let a173 = v0;
    let a174 = 0u64;
    if nat_eq(a173, a174) {
        let a175 = v1;
        let a176 = 0u64;
        if nat_eq(a175, a176) {
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

pub fn f33(v0: u64, v1: u64) -> R<u64> {
    if f40(v0) {
        let a182 = {
            let a179 = {
                let a177 = v0;
                let a178 = 44032u64;
                nat_sub(a177, a178)
            };
            let a180 = 28u64;
            let a181 = 0u64;
            nat_rem(a179, a180, a181)
        };
        let a183 = 0u64;
        if nat_eq(a182, a183) {
            let a184 = v1;
            let a185 = 4520u64;
            if nat_lt(a184, a185) {
                Ok(0u64)
            } else {
                let a186 = v1;
                let a187 = 4547u64;
                if nat_lt(a186, a187) {
                    let a190 = v0;
                    let a191 = {
                        let a188 = v1;
                        let a189 = 4519u64;
                        nat_sub(a188, a189)
                    };
                    nat_add(a190, a191)
                } else {
                    Ok(0u64)
                }
            }
        } else {
            Ok(0u64)
        }
    } else {
        let a192 = v0;
        let a193 = 4352u64;
        if nat_lt(a192, a193) {
            Ok(0u64)
        } else {
            let a194 = v0;
            let a195 = 4371u64;
            if nat_lt(a194, a195) {
                let a196 = v1;
                let a197 = 4449u64;
                if nat_lt(a196, a197) {
                    Ok(0u64)
                } else {
                    let a198 = v1;
                    let a199 = 4470u64;
                    if nat_lt(a198, a199) {
                        let a210 = 44032u64;
                        let a211 = {
                            let a208 = {
                                let a206 = {
                                    let a202 = {
                                        let a200 = v0;
                                        let a201 = 4352u64;
                                        nat_sub(a200, a201)
                                    };
                                    let a203 = 21u64;
                                    nat_mul(a202, a203)?
                                };
                                let a207 = {
                                    let a204 = v1;
                                    let a205 = 4449u64;
                                    nat_sub(a204, a205)
                                };
                                nat_add(a206, a207)?
                            };
                            let a209 = 28u64;
                            nat_mul(a208, a209)?
                        };
                        nat_add(a210, a211)
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

pub fn f34(v0: (Bytes, Bytes), v1: u64, v2: u64) -> R<u64> {
    let a212 = v2;
    let a213 = f11();
    if nat_lt(a212, a213) {
        Ok(0u64)
    } else {
        let v3: u64 = {
            let a216 = {
                let a214 = v1;
                let a215 = 16777216u64;
                nat_mul(a214, a215)?
            };
            let a217 = v2;
            nat_add(a216, a217)?
        };
        let v4: u64 = f25(v0.clone(), f26(v0.clone(), 4u64)?, 9u64, f27(v0.clone(), 4u64)?, 6u64, v3)?;
        let a218 = f28(v0.clone(), v4, 6u64, 0u64)?;
        let a219 = v3;
        if nat_eq(a218, a219) {
            f28(v0, {
                let a220 = v4;
                let a221 = 6u64;
                nat_add(a220, a221)?
            }, 3u64, 0u64)
        } else {
            Ok(0u64)
        }
    }
}

pub fn f35(v0: (Bytes, Bytes), v1: u64) -> R<bool> {
    let a222 = v1;
    let a223 = f11();
    if nat_lt(a222, a223) {
        Ok(false)
    } else {
        let v2: u64 = f25(v0.clone(), f26(v0.clone(), 1u64)?, 6u64, f27(v0.clone(), 1u64)?, 3u64, v1)?;
        let a224 = v1;
        let a225 = f28(v0.clone(), v2, 3u64, 0u64)?;
        if nat_lt(a224, a225) {
            Ok(false)
        } else {
            let a228 = f28(v0, {
                let a226 = v2;
                let a227 = 3u64;
                nat_add(a226, a227)?
            }, 3u64, 0u64)?;
            let a229 = v1;
            Ok(!nat_lt(a228, a229))
        }
    }
}

pub fn f36(p247: u64, v1: (Bytes, Bytes), v2: u64, v3: u64, v4: u64, v5: u64, p248: u64, p249: u64) -> R<u64> {
    let m246 = (p247, (p248, p249));
    {
        let mut m246 = m246;
        loop {
            let step = {
                let (v0, (v6, v7)) = m246;
                let m230 = v0;
                if m230 == 0 {
                    Err::<(u64, (u64, u64)), R<u64>>(Ok(v6))
                } else {
                    let v8 = m230 - 1;
                    let a233 = {
                        let a231 = v7;
                        let a232 = v6;
                        nat_sub(a231, a232)
                    };
                    let a234 = 2u64;
                    if nat_lt(a233, a234) {
                        Err::<(u64, (u64, u64)), R<u64>>(Ok(v6))
                    } else {
                        let v9: u64 = {
                            let a237 = {
                                let a235 = v6;
                                let a236 = v7;
                                nat_add(a235, a236)?
                            };
                            let a238 = 2u64;
                            let a239 = 0u64;
                            nat_quot(a237, a238, a239)
                        };
                        let a244 = v5;
                        let a245 = f28(v1.clone(), {
                            let a242 = v2;
                            let a243 = {
                                let a240 = v9;
                                let a241 = v3;
                                nat_mul(a240, a241)?
                            };
                            nat_add(a242, a243)?
                        }, v4, 0u64)?;
                        if nat_lt(a244, a245) {
                            Ok::<(u64, (u64, u64)), R<u64>>((v8, (v6, v9)))
                        } else {
                            Ok::<(u64, (u64, u64)), R<u64>>((v8, (v9, v7)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m246 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f37() -> u64 {
    16384u64
}

pub fn f38(v0: List<u64>, v1: List<u64>) -> List<u64> {
    let m250 = v0;
    match m250.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f38(v3, v1))
        }
    }
}

pub fn f39(v0: (Bytes, Bytes), v1: u64) -> R<List<u64>> {
    if f40(v1) {
        f41(v1)
    } else {
        let v2: u64 = f42(v0.clone(), v1)?;
        let a251 = v2;
        let a252 = 0u64;
        if nat_eq(a251, a252) {
            let m253 = f43(v0, v1)?;
            match m253.uncons() {
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

pub fn f40(v0: u64) -> bool {
    let a254 = v0;
    let a255 = 44032u64;
    if nat_lt(a254, a255) {
        false
    } else {
        let a256 = v0;
        let a257 = 55204u64;
        nat_lt(a256, a257)
    }
}

pub fn f41(v0: u64) -> R<List<u64>> {
    let v1: u64 = {
        let a258 = v0;
        let a259 = 44032u64;
        nat_sub(a258, a259)
    };
    let v2: u64 = {
        let a263 = 4352u64;
        let a264 = {
            let a260 = v1;
            let a261 = 588u64;
            let a262 = 0u64;
            nat_quot(a260, a261, a262)
        };
        nat_add(a263, a264)?
    };
    let v3: u64 = {
        let a271 = 4449u64;
        let a272 = {
            let a268 = {
                let a265 = v1;
                let a266 = 588u64;
                let a267 = 0u64;
                nat_rem(a265, a266, a267)
            };
            let a269 = 28u64;
            let a270 = 0u64;
            nat_quot(a268, a269, a270)
        };
        nat_add(a271, a272)?
    };
    let v4: u64 = {
        let a276 = 4519u64;
        let a277 = {
            let a273 = v1;
            let a274 = 28u64;
            let a275 = 0u64;
            nat_rem(a273, a274, a275)
        };
        nat_add(a276, a277)?
    };
    let a278 = v4;
    let a279 = 4519u64;
    if nat_eq(a278, a279) {
        Ok(List::cons(v2, List::cons(v3, List::<u64>::nil())))
    } else {
        Ok(List::cons(v2, List::cons(v3, List::cons(v4, List::<u64>::nil()))))
    }
}

pub fn f42(v0: (Bytes, Bytes), v1: u64) -> R<u64> {
    let a280 = v1;
    let a281 = f11();
    if nat_lt(a280, a281) {
        Ok(0u64)
    } else {
        let v2: u64 = f25(v0.clone(), f26(v0.clone(), 2u64)?, 6u64, f27(v0.clone(), 2u64)?, 3u64, v1)?;
        let a282 = f28(v0.clone(), v2, 3u64, 0u64)?;
        let a283 = v1;
        if nat_eq(a282, a283) {
            f28(v0, {
                let a284 = v2;
                let a285 = 3u64;
                nat_add(a284, a285)?
            }, 3u64, 0u64)
        } else {
            Ok(0u64)
        }
    }
}

pub fn f43(v0: (Bytes, Bytes), v1: u64) -> R<List<u64>> {
    let a286 = v1;
    let a287 = f44();
    if nat_lt(a286, a287) {
        Ok(List::<u64>::nil())
    } else {
        let v2: u64 = f25(v0.clone(), f26(v0.clone(), 3u64)?, 9u64, f27(v0.clone(), 3u64)?, 3u64, v1)?;
        let a288 = f28(v0.clone(), v2, 3u64, 0u64)?;
        let a289 = v1;
        if nat_eq(a288, a289) {
            Ok(List::cons(f28(v0.clone(), {
                let a290 = v2;
                let a291 = 3u64;
                nat_add(a290, a291)?
            }, 3u64, 0u64)?, List::cons(f28(v0, {
                let a292 = v2;
                let a293 = 6u64;
                nat_add(a292, a293)?
            }, 3u64, 0u64)?, List::<u64>::nil())))
        } else {
            Ok(List::<u64>::nil())
        }
    }
}

pub fn f44() -> u64 {
    192u64
}

pub fn run(p0: u64, p1: Bytes, p2: Bytes, p3: Bytes) -> R<List<u64>> {
    f0(p0, p1, p2, p3)
}
