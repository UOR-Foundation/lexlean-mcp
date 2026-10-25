#![forbid(unsafe_code)]
use core::cmp::Ordering;

use core::sync::atomic::{AtomicU64, Ordering as Memory};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Overflow;

pub type R<T> = Result<T, Overflow>;

static WORK: AtomicU64 = AtomicU64::new(0);

pub fn tick(units: u64) { WORK.fetch_add(units, Memory::Relaxed); }

pub fn work() -> u64 { WORK.load(Memory::Relaxed) }

pub trait Same {
    fn same(&self, other: &Self) -> bool;
}

pub trait Key {
    fn key(&self, other: &Self) -> Ordering;
}

impl Same for bool {
    fn same(&self, other: &Self) -> bool { self == other }
}

impl Key for bool {
    fn key(&self, other: &Self) -> Ordering { self.cmp(other) }
}

impl Same for u8 {
    fn same(&self, other: &Self) -> bool { self == other }
}

impl Key for u8 {
    fn key(&self, other: &Self) -> Ordering { self.cmp(other) }
}

impl Same for u16 {
    fn same(&self, other: &Self) -> bool { self == other }
}

impl Key for u16 {
    fn key(&self, other: &Self) -> Ordering { self.cmp(other) }
}

impl Same for u32 {
    fn same(&self, other: &Self) -> bool { self == other }
}

impl Key for u32 {
    fn key(&self, other: &Self) -> Ordering { self.cmp(other) }
}

impl Same for u64 {
    fn same(&self, other: &Self) -> bool { self == other }
}

impl Key for u64 {
    fn key(&self, other: &Self) -> Ordering { self.cmp(other) }
}

impl Same for i8 {
    fn same(&self, other: &Self) -> bool { self == other }
}

impl Key for i8 {
    fn key(&self, other: &Self) -> Ordering { self.cmp(other) }
}

impl Same for i16 {
    fn same(&self, other: &Self) -> bool { self == other }
}

impl Key for i16 {
    fn key(&self, other: &Self) -> Ordering { self.cmp(other) }
}

impl Same for i32 {
    fn same(&self, other: &Self) -> bool { self == other }
}

impl Key for i32 {
    fn key(&self, other: &Self) -> Ordering { self.cmp(other) }
}

impl Same for i64 {
    fn same(&self, other: &Self) -> bool { self == other }
}

impl Key for i64 {
    fn key(&self, other: &Self) -> Ordering { self.cmp(other) }
}

impl Same for Ordering {
    fn same(&self, other: &Self) -> bool { self == other }
}

impl<A: Key, B: Key> Key for (A, B) {
    fn key(&self, other: &Self) -> Ordering { match self.0.key(&other.0) { Ordering::Equal => self.1.key(&other.1), decided => decided, } }
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
    C0(Adt1, Option<Str>, Adt4, Adt10, List<Adt12>, u64, Bytes),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(Str, Option<Str>, Str, Option<Str>, Option<Str>, Option<List<Adt2>>),
}

#[derive(Clone)]
pub enum Adt2 {
    C0(Str, Option<Str>, Option<List<Str>>, Option<Adt3>),
}

#[derive(Clone)]
pub enum Adt3 {
    C0,
    C1,
}

#[derive(Clone)]
pub enum Adt4 {
    C0(Option<Adt5>, Option<Adt5>, Option<Adt5>, Option<Adt8>, Option<Adt9>, Option<Adt8>, Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt5 {
    C0(List<(Str, Adt6)>),
}

#[derive(Clone)]
pub enum Adt6 {
    C0,
    C1(bool),
    C2(Adt7),
    C3(Str),
    C4(List<Adt6>),
    C5(List<(Str, Adt6)>),
}

#[derive(Clone)]
pub enum Adt7 {
    C0(i64, i64),
}

#[derive(Clone)]
pub enum Adt8 {
    C0(Option<bool>),
}

#[derive(Clone)]
pub enum Adt9 {
    C0(Option<bool>, Option<bool>),
}

#[derive(Clone)]
pub enum Adt10 {
    C0(u64, Adt11),
}

#[derive(Clone)]
pub enum Adt11 {
    C0,
    C1,
}

#[derive(Clone)]
pub enum Adt12 {
    C0(Str, Str, List<Str>, Adt13),
}

#[derive(Clone)]
pub enum Adt13 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt14 {
    C0(List<Adt15>, Adt16),
}

#[derive(Clone)]
pub enum Adt15 {
    C0(u64, Bytes),
}

#[derive(Clone)]
pub enum Adt16 {
    C0(Adt17),
    C1(Adt22),
    C2(Adt23),
    C3(Adt22),
    C4(Adt24),
    C5(Adt22),
    C6(Adt22),
    C7(Adt25),
    C8(Adt26),
    C9(Adt31),
    C10(Adt32),
    C11(Adt33),
}

#[derive(Clone)]
pub enum Adt17 {
    C0(Adt18),
}

#[derive(Clone)]
pub enum Adt18 {
    C0(Str, Adt19, Option<Adt5>, Option<Adt20>, Option<Adt21>, Adt5),
}

#[derive(Clone)]
pub enum Adt19 {
    C0(Option<Adt5>, Option<Adt5>, Option<Adt5>, Option<Adt5>, Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt20 {
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
}

#[derive(Clone)]
pub enum Adt21 {
    C0(Str),
    C1(i64),
}

#[derive(Clone)]
pub enum Adt22 {
    C0(Adt18, Option<Str>),
}

#[derive(Clone)]
pub enum Adt23 {
    C0(Adt18, Str, Option<Adt5>, Option<Adt5>, Option<Str>),
}

#[derive(Clone)]
pub enum Adt24 {
    C0(Adt18, Str, Option<Adt5>, Option<Str>),
}

#[derive(Clone)]
pub enum Adt25 {
    C0(Adt18, Str, Option<List<(Str, Str)>>, Option<Adt5>, Option<Str>),
}

#[derive(Clone)]
pub enum Adt26 {
    C0(Adt18, Adt27, Adt29, Option<Adt30>),
}

#[derive(Clone)]
pub enum Adt27 {
    C0(Adt28),
    C1(Str),
}

#[derive(Clone)]
pub enum Adt28 {
    C0(Str, Option<Str>),
}

#[derive(Clone)]
pub enum Adt29 {
    C0(Str, Str),
}

#[derive(Clone)]
pub enum Adt30 {
    C0(Option<List<(Str, Str)>>),
}

#[derive(Clone)]
pub enum Adt31 {
    C0(Adt18, Str),
}

#[derive(Clone)]
pub enum Adt32 {
    C0(Adt18, Str, Adt5),
}

#[derive(Clone)]
pub enum Adt33 {
    C0(Adt18, Str),
}

#[derive(Clone)]
pub enum Adt34 {
    C0(Adt35),
    C1(Adt36),
    C2(Adt39),
    C3(Adt51),
    C4(Adt53),
    C5(Adt54),
    C6(Adt55),
    C7(Adt57),
    C8(Adt60),
    C9(Adt62),
    C10(Adt64),
    C11,
    C12(Adt83),
    C13(Adt84),
}

#[derive(Clone)]
pub enum Adt35 {
    C0(Adt10, List<Str>, Adt4, Option<Str>),
}

#[derive(Clone)]
pub enum Adt36 {
    C0(Adt10, Option<Str>, List<Adt37>),
}

#[derive(Clone)]
pub enum Adt37 {
    C0(Str, Option<Str>, Option<List<Adt2>>, Option<Str>, Adt5, Option<Adt5>, Option<Adt38>, Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt38 {
    C0(Option<Str>, Option<bool>, Option<bool>, Option<bool>, Option<bool>),
}

#[derive(Clone)]
pub enum Adt39 {
    C0(List<Adt40>, Option<Adt6>, Option<bool>),
}

#[derive(Clone)]
pub enum Adt40 {
    C0(Adt41),
    C1(Adt44),
    C2(Adt45),
    C3(Adt46),
    C4(Adt47),
}

#[derive(Clone)]
pub enum Adt41 {
    C0(Str, Option<Adt42>, Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt42 {
    C0(Option<List<Adt43>>, Option<Adt7>, Option<Str>),
}

#[derive(Clone)]
pub enum Adt43 {
    C0,
    C1,
}

#[derive(Clone)]
pub enum Adt44 {
    C0(Str, Str, Option<Adt42>, Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt45 {
    C0(Str, Str, Option<Adt42>, Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt46 {
    C0(Str, Option<Str>, Option<List<Adt2>>, Str, Option<Str>, Option<Str>, Option<Adt42>, Option<u64>, Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt47 {
    C0(Adt48, Option<Adt42>, Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt48 {
    C0(Adt49),
    C1(Adt50),
}

#[derive(Clone)]
pub enum Adt49 {
    C0(Str, Option<Str>, Str, Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt50 {
    C0(Str, Option<Str>, Str, Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt51 {
    C0(Adt39, List<Adt52>),
}

#[derive(Clone)]
pub enum Adt52 {
    C0(Adt7, Option<Adt7>, Option<Str>),
}

#[derive(Clone)]
pub enum Adt53 {
    C0(Adt10, Option<Str>, List<Adt46>),
}

#[derive(Clone)]
pub enum Adt54 {
    C0(Adt10, List<Adt48>),
}

#[derive(Clone)]
pub enum Adt55 {
    C0(Adt10, Option<Str>, List<Adt56>),
}

#[derive(Clone)]
pub enum Adt56 {
    C0(Str, Option<Str>, Option<List<Adt2>>, Str, Option<Str>, Option<Str>, Option<Adt42>, Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt57 {
    C0(Adt10, Option<Str>, List<Adt58>),
}

#[derive(Clone)]
pub enum Adt58 {
    C0(Str, Option<Str>, Option<List<Adt2>>, Option<Str>, Option<List<Adt59>>, Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt59 {
    C0(Str, Option<Str>, Option<Str>, Option<bool>),
}

#[derive(Clone)]
pub enum Adt60 {
    C0(Option<Str>, List<Adt61>),
}

#[derive(Clone)]
pub enum Adt61 {
    C0(Adt43, Adt40),
}

#[derive(Clone)]
pub enum Adt62 {
    C0(Adt63),
}

#[derive(Clone)]
pub enum Adt63 {
    C0(List<Str>, Option<u64>, Option<bool>),
}

#[derive(Clone)]
pub enum Adt64 {
    C0(Adt65),
    C1(Adt65, List<(Str, Adt66)>),
    C2(Adt65, Adt5),
    C3(Adt65, Adt82),
    C4(Adt65),
}

#[derive(Clone)]
pub enum Adt65 {
    C0(Str, Option<Str>, Str, Str, Option<u64>, Option<u64>),
}

#[derive(Clone)]
pub enum Adt66 {
    C0(Adt67),
    C1(Adt71),
    C2(Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt67 {
    C0(Adt68),
    C1(Adt70),
}

#[derive(Clone)]
pub enum Adt68 {
    C0(Option<Adt69>, Str, Adt5),
}

#[derive(Clone)]
pub enum Adt69 {
    C0,
}

#[derive(Clone)]
pub enum Adt70 {
    C0(Str, Str),
}

#[derive(Clone)]
pub enum Adt71 {
    C0(List<Adt72>, Option<Adt77>, Option<Str>, Option<Adt79>, Option<Adt7>, u64, Option<List<Str>>, Option<Adt5>, Option<List<Adt37>>, Option<Adt80>),
}

#[derive(Clone)]
pub enum Adt72 {
    C0(Adt43, Adt73, Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt73 {
    C0(Adt74),
    C1(List<Adt74>),
}

#[derive(Clone)]
pub enum Adt74 {
    C0(Adt41),
    C1(Adt44),
    C2(Adt45),
    C3(Adt75),
    C4(Adt76),
}

#[derive(Clone)]
pub enum Adt75 {
    C0(Str, Str, Adt5, Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt76 {
    C0(Str, List<Adt40>, Option<Adt6>, Option<bool>, Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt77 {
    C0(Option<List<Adt78>>, Option<Adt7>, Option<Adt7>, Option<Adt7>),
}

#[derive(Clone)]
pub enum Adt78 {
    C0(Option<Str>),
}

#[derive(Clone)]
pub enum Adt79 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt80 {
    C0(Option<Adt81>),
}

#[derive(Clone)]
pub enum Adt81 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt82 {
    C0(i64, Str, Option<Adt6>),
}

#[derive(Clone)]
pub enum Adt83 {
    C0(List<(Str, Adt66)>),
    C1(Str),
    C2(List<(Str, Adt66)>, Str),
}

#[derive(Clone)]
pub enum Adt84 {
    C0(Adt85, Adt65),
}

#[derive(Clone)]
pub enum Adt85 {
    C0,
    C1,
    C2,
    C3,
    C4,
}

#[derive(Clone)]
pub enum Adt86 {
    C0(Str),
    C1(Str),
    C2(Str),
    C3(Str),
    C4(Str),
    C5(Str),
    C6(Str),
    C7(u64),
    C8(Adt19),
    C9(Str),
    C10,
    C11,
    C12(Str),
    C13,
    C14,
    C15(Adt87),
}

#[derive(Clone)]
pub enum Adt87 {
    C0(u64, Bytes),
}

#[derive(Clone)]
pub enum Adt88 {
    C0(List<u64>),
    C1(Adt87),
}

#[derive(Clone)]
pub enum Adt89 {
    C0(Bytes, List<Adt15>),
}

#[derive(Clone)]
pub enum Adt90 {
    C0(u64),
    C1(u64),
}

#[derive(Clone)]
pub enum Adt91 {
    C0(List<u64>, List<u64>, List<Adt92>, Bytes, u64, List<List<u64>>, Bytes),
}

#[derive(Clone)]
pub enum Adt92 {
    C0(List<u64>, List<u64>),
}

#[derive(Clone)]
pub enum Adt93 {
    C0(Str),
    C1(Str),
    C2(Str),
    C3(Str),
    C4(Str, Str),
    C5(Str, Str),
    C6(Str, Str),
    C7(Str),
    C8(Str),
    C9(Str),
    C10(Str),
    C11(Str),
    C12(Str, Str),
}

#[derive(Clone)]
pub enum Adt94 {
    C0(Bytes, u64),
}

#[derive(Clone)]
pub enum Adt95 {
    C0(u64, List<Adt92>, List<List<u64>>, List<u64>),
}

#[derive(Clone)]
pub enum Adt96 {
    C0(Adt97, List<u64>),
}

#[derive(Clone)]
pub enum Adt97 {
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
}

#[derive(Clone)]
pub enum Adt98 {
    C0,
    C1,
}

#[derive(Clone)]
pub enum Adt99 {
    C0,
    C1(List<Adt100>, Str),
    C2(Str),
    C3(Str),
    C4,
}

#[derive(Clone)]
pub enum Adt100 {
    C0(Str),
    C1(u64),
}

#[derive(Clone)]
pub enum Adt101 {
    C0,
    C1(List<Adt100>, Str),
}

#[derive(Clone)]
pub enum Adt102 {
    C0(Adt6, Adt6, List<Adt100>),
    C1(List<Adt6>, Adt6, List<Adt100>),
    C2(List<Adt6>, Adt6, List<Adt100>, u64),
    C3(List<Adt100>),
    C4(Option<Adt6>, Option<Adt6>, Adt6, List<Adt100>),
    C5(Adt103, Adt6, Adt6, List<Adt100>),
}

#[derive(Clone)]
pub enum Adt103 {
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt104 {
    C0(Str),
    C1(i64),
}

#[derive(Clone)]
pub enum Adt105 {
    C0(Adt6, bool, List<Str>),
}

#[derive(Clone)]
pub enum Adt106 {
    C0(Adt101, List<Adt102>),
}

#[derive(Clone)]
pub enum Adt107 {
    C0(u64),
    C1(u64),
    C2(u64),
    C3,
    C4(u64),
    C5(u64),
    C6(u64),
    C7(u64),
    C8(u64),
    C9(u64),
    C10(u64),
    C11,
    C12,
    C13,
    C14,
}

#[derive(Clone)]
pub enum Adt108 {
    C0,
    C1(Option<Adt104>, Adt109),
}

#[derive(Clone)]
pub enum Adt109 {
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
}

#[derive(Clone)]
pub enum Adt110 {
    C0,
    C1,
    C2,
    C3(List<u64>),
}

#[derive(Clone)]
pub enum Adt111 {
    C0,
    C1(List<u64>),
    C2,
}

#[derive(Clone)]
pub enum Adt112 {
    C0(List<Adt12>, List<List<u64>>),
}

#[derive(Clone)]
pub enum Adt113 {
    C0(Str),
    C1(Str),
    C2(Str),
    C3(List<Adt102>),
}

#[derive(Clone)]
pub enum Adt114 {
    C0,
    C1(Adt6),
}

#[derive(Clone)]
pub enum Adt115 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt116 {
    C0(Adt104, Str, Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt117 {
    C0,
    C1(u64),
    C2(u64),
}

#[derive(Clone)]
pub enum Adt118 {
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt119 {
    C0(Adt120, List<Adt124>, u64),
}

#[derive(Clone)]
pub enum Adt120 {
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6(Adt121, List<u64>, bool),
    C7(u64, u64),
    C8(Adt122),
    C9(Adt6),
}

#[derive(Clone)]
pub enum Adt121 {
    C0,
    C1,
    C2(u64, u64, u64),
    C3(u64),
    C4(u64),
}

#[derive(Clone)]
pub enum Adt122 {
    C0(bool, Adt123, i64, u64, i64, bool, i64),
}

#[derive(Clone)]
pub enum Adt123 {
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
}

#[derive(Clone)]
pub enum Adt124 {
    C0(List<Adt6>),
    C1(List<(Str, Adt6)>, Option<Str>),
}

#[derive(Clone)]
pub enum Adt125 {
    C0(List<u64>),
    C1(u64, Adt6),
    C2(u64, List<Adt6>),
    C3(u64, List<Adt6>),
    C4(u64, List<(Str, Adt6)>),
    C5(u64, List<(Str, Adt6)>),
}

#[derive(Clone)]
pub enum Adt126 {
    C0(Option<Str>, Option<Str>, Option<Str>, Option<Adt127>, List<Adt129>),
}

#[derive(Clone)]
pub enum Adt127 {
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6(Adt128),
    C7(Adt128),
    C8(Str),
    C9(Str),
    C10(Str),
}

#[derive(Clone)]
pub enum Adt128 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt129 {
    C0(Str, List<Str>, Adt13, Adt130),
}

#[derive(Clone)]
pub enum Adt130 {
    C0,
    C1,
    C2(Str),
}

#[derive(Clone)]
pub enum Adt131 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt132 {
    C0,
    C1,
}

#[derive(Clone)]
pub enum Adt133 {
    C0(Adt107),
    C1(Adt119),
}

#[derive(Clone)]
pub enum Adt134 {
    C0,
    C1(List<(Adt6, Adt6)>),
}

#[derive(Clone)]
pub enum Adt135 {
    C0(Adt104, Result<Adt136, Adt139>),
}

#[derive(Clone)]
pub enum Adt136 {
    C0(Adt34, Adt137, List<Adt138>),
}

#[derive(Clone)]
pub enum Adt137 {
    C0(Option<Adt1>, Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt138 {
    C0(Adt21, Adt52),
}

#[derive(Clone)]
pub enum Adt139 {
    C0(Adt140),
    C1(Str, List<Str>),
    C2(Adt127),
    C3(Str),
    C4(Adt86),
}

#[derive(Clone)]
pub enum Adt140 {
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7(Str),
    C8(Str),
}

#[derive(Clone)]
pub enum Adt141 {
    C0(Option<Str>, Option<Adt127>),
}

#[derive(Clone)]
pub enum Adt142 {
    C0,
    C1(Adt104),
    C2,
}

#[derive(Clone)]
pub enum Adt143 {
    C0(Adt93),
    C1(Adt112),
}

#[derive(Clone)]
pub enum Adt144 {
    C0,
    C1,
    C2(Adt6),
}

#[derive(Clone)]
pub enum Adt145 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt146 {
    C0(Adt107),
    C1(List<Adt125>, List<u64>),
}

#[derive(Clone)]
pub enum Adt147 {
    C0(Adt104, Adt148, Adt18, Adt5),
}

#[derive(Clone)]
pub enum Adt148 {
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
    C11,
}

#[derive(Clone)]
pub enum Adt149 {
    C0,
    C1(Adt148),
}

#[derive(Clone)]
pub enum Adt150 {
    C0(Adt151),
    C1(Adt139),
}

#[derive(Clone)]
pub enum Adt151 {
    C0(Adt104, Str, Adt18, Adt5),
}

#[derive(Clone)]
pub enum Adt152 {
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt153 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt154 {
    C0(Adt6),
    C1(Adt6),
}

#[derive(Clone)]
pub enum Adt155 {
    C0(List<Adt6>, List<Adt6>),
    C1(Str, List<(Str, Adt6)>, List<(Str, Adt6)>),
}

#[derive(Clone)]
pub enum Adt156 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt157 {
    C0(u64, List<u64>),
}

#[derive(Clone)]
pub enum Adt158 {
    C0,
    C1(Adt20),
}

#[derive(Clone)]
pub enum Adt159 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt160 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt161 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32, Adt160, u64),
}

#[derive(Clone)]
pub enum Fn0 {
    F3,
}

impl Fn0 {
    pub fn apply(&self, p0: Adt14) -> R<Result<Adt34, Adt86>> {
        match self {
            Fn0::F3 => f3(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F15(Adt14),
}

impl Fn1 {
    pub fn apply(&self, p0: Adt23) -> R<Result<Adt51, Adt86>> {
        match self {
            Fn1::F15(k0) => f15(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F45(Fn0, Adt89),
}

impl Fn2 {
    pub fn apply(&self, p0: Adt16) -> R<Result<Adt34, Adt86>> {
        match self {
            Fn2::F45(k0, k1) => f45(k0.clone(), k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F57(Fn1, Adt37),
}

impl Fn3 {
    pub fn apply(&self, p0: Adt23) -> R<Result<Adt34, Adt86>> {
        match self {
            Fn3::F57(k0, k1) => f57(k0.clone(), k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F814(Bytes),
    F839(Bytes, Bytes),
}

impl Fn4 {
    pub fn apply(&self, p0: (u64, u64)) -> R<(u64, u64)> {
        match self {
            Fn4::F814(k0) => f814(k0.clone(), p0),
            Fn4::F839(k0, k1) => f839(k0.clone(), k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F819(Bytes),
}

impl Fn5 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<(u64, Bytes)> {
        match self {
            Fn5::F819(k0) => f819(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F824(Bytes),
}

impl Fn6 {
    pub fn apply(&self, p0: (u64, (Bytes, u64))) -> R<(u64, (Bytes, u64))> {
        match self {
            Fn6::F824(k0) => f824(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F847(Bytes, u64, u64, u64),
    F854(Bytes, u64, u64),
}

impl Fn7 {
    pub fn apply(&self, p0: (u64, Adt159)) -> R<(u64, Adt159)> {
        match self {
            Fn7::F847(k0, k1, k2, k3) => f847(k0.clone(), *k1, *k2, *k3, p0),
            Fn7::F854(k0, k1, k2) => f854(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F849,
}

impl Fn8 {
    pub fn apply(&self, p0: Adt161) -> R<Adt161> {
        match self {
            Fn8::F849 => f849(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F879,
}

impl Fn9 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn9::F879 => f879(p0),
        }
    }
}

pub fn f0(v0: Bytes) -> R<List<u64>> {
    f1(f2()?, Fn0::F3, v0)
}

pub fn f1(v0: Adt0, v1: Fn0, v2: Bytes) -> R<List<u64>> {
    f4(f5(v0, v1, v2)?)
}

pub fn f2() -> R<Adt0> {
    Ok(Adt0::C0(f6(), None::<Str>, f7(), f8(), f9(f10()?)?, f11(), Bytes::lit(&[])))
}

pub fn f3(v0: Adt14) -> R<Result<Adt34, Adt86>> {
    let m1 = f12(f10()?)?;
    match m1 {
        None => {
            let m3 = match v0.clone() {
                Adt14::C0(_, h2) => {
                    h2
                }
            };
            match m3 {
                Adt16::C0(_) => {
                    Ok(f16())
                }
                Adt16::C1(v2) => {
                    Ok(f13(f8(), f10()?, v2))
                }
                Adt16::C2(v3) => {
                    f14(f10()?, Fn1::F15(v0), v3)
                }
                Adt16::C3(_) => {
                    Ok(f16())
                }
                Adt16::C4(_) => {
                    Ok(f16())
                }
                Adt16::C5(_) => {
                    Ok(f16())
                }
                Adt16::C6(_) => {
                    Ok(f16())
                }
                Adt16::C7(_) => {
                    Ok(f16())
                }
                Adt16::C8(_) => {
                    Ok(f16())
                }
                Adt16::C9(_) => {
                    Ok(f16())
                }
                Adt16::C10(_) => {
                    Ok(f16())
                }
                Adt16::C11(_) => {
                    Ok(f16())
                }
            }
        }
        Some(v1) => {
            Ok(Err::<Adt34, Adt86>(v1))
        }
    }
}

pub fn f4(v0: Adt88) -> R<List<u64>> {
    let m4 = v0;
    match m4 {
        Adt88::C0(v1) => {
            let a5 = f17(0u64);
            let a6 = v1;
            Ok(append_list(a5, a6))
        }
        Adt88::C1(v2) => {
            let a11 = f17(1u64);
            let a12 = {
                let a9 = f17(match v2.clone() {
                    Adt87::C0(h7, _) => {
                        h7
                    }
                });
                let a10 = f18(match v2 {
                    Adt87::C0(_, h8) => {
                        h8
                    }
                })?;
                append_list(a9, a10)
            };
            Ok(append_list(a11, a12))
        }
    }
}

pub fn f5(v0: Adt0, v1: Fn0, v2: Bytes) -> R<Adt88> {
    let m13 = f19(v2)?;
    match m13 {
        Ok(v4) => {
            f21(v0, v1, v4.clone(), f22(match v4 {
                Adt89::C0(h14, _) => {
                    h14
                }
            })?)
        }
        Err(_) => {
            Ok(f20())
        }
    }
}

pub fn f6() -> Adt1 {
    Adt1::C0(Str::lit("lexlean"), None::<Str>, Str::lit("0.1.0"), None::<Str>, None::<Str>, None::<List<Adt2>>)
}

pub fn f7() -> Adt4 {
    Adt4::C0(None::<Adt5>, None::<Adt5>, None::<Adt5>, None::<Adt8>, None::<Adt9>, Some(Adt8::C0(None::<bool>)), None::<Adt5>)
}

pub fn f8() -> Adt10 {
    Adt10::C0(0u64, Adt11::C1)
}

pub fn f9(v0: List<Adt37>) -> R<List<Adt12>> {
    let m15 = f23(v0)?;
    match m15 {
        Ok(v1) => {
            Ok(v1)
        }
        Err(_) => {
            Ok(List::<Adt12>::nil())
        }
    }
}

pub fn f10() -> R<List<Adt37>> {
    Ok(List::cons(f24(Str::lit("lexlean_draft_write"), Str::lit("Write a file of a draft in the store."), f25(List::cons((Str::lit("path"), f26()?), f27(Str::lit("content"), f28(Str::lit("string")))), List::cons(Str::lit("path"), List::cons(Str::lit("content"), List::<Str>::nil())))), List::cons(f24(Str::lit("lexlean_draft_read"), Str::lit("Read a file of a draft from the store."), f25(f27(Str::lit("path"), f26()?), List::cons(Str::lit("path"), List::<Str>::nil()))), List::cons(f29(Str::lit("lexlean_check"), Str::lit("Check a project with the LexLean engine: the result is what lexlean check prints with --diagnostic-format json, and a refused project is an error result that carries it."), f25(f30()?, List::cons(Str::lit("project"), List::<Str>::nil())), f31(Str::lit("check"))), List::cons(f29(Str::lit("lexlean_snapshot"), Str::lit("Take the semantic snapshot of a project with the LexLean engine: the result is the snapshot, and a refused project is an error result with the command result of check."), f25(f30()?, List::cons(Str::lit("project"), List::<Str>::nil())), f32()), List::cons(f29(Str::lit("lexlean_format"), Str::lit("Format a project with the LexLean engine as lexlean fmt does, or check its formatting with check_only."), f25({
        let a16 = f30()?;
        let a17 = f27(Str::lit("check_only"), f28(Str::lit("boolean")));
        append_list(a16, a17)
    }, List::cons(Str::lit("project"), List::<Str>::nil())), f31(Str::lit("fmt"))), List::cons(f29(Str::lit("lexlean_build"), Str::lit("Build a project with the LexLean engine as lexlean build does."), f25(f30()?, List::cons(Str::lit("project"), List::<Str>::nil())), f31(Str::lit("build"))), List::cons(f24(Str::lit("lexlean_verify"), Str::lit("Start the verification of a project and return its task."), f25(f33()?, List::cons(Str::lit("project"), List::<Str>::nil()))), List::cons(f24(Str::lit("lexlean_verify_control"), Str::lit("Read the state of a verification task, or cancel it."), f25(List::cons((Str::lit("task"), f26()?), f27(Str::lit("action"), f34(Str::lit("status"), Str::lit("cancel")))), List::cons(Str::lit("task"), List::cons(Str::lit("action"), List::<Str>::nil())))), List::cons(f24(Str::lit("lexlean_lexicon_read"), Str::lit("Read an entry of the lexicon of a profile."), f25(List::cons((Str::lit("profile"), f26()?), f27(Str::lit("entry"), f26()?)), List::cons(Str::lit("profile"), List::cons(Str::lit("entry"), List::<Str>::nil())))), List::cons(f24(Str::lit("lexlean_attestation"), Str::lit("Look an attestation up by its ID or by the digest of its file."), f25(List::cons((Str::lit("by"), f34(Str::lit("id"), Str::lit("digest"))), f27(Str::lit("value"), f26()?)), List::cons(Str::lit("by"), List::cons(Str::lit("value"), List::<Str>::nil())))), List::<Adt37>::nil())))))))))))
}

pub fn f11() -> u64 {
    100u64
}

pub fn f12(v0: List<Adt37>) -> R<Option<Adt86>> {
    let m18 = f23(v0)?;
    match m18 {
        Ok(_) => {
            Ok(None::<Adt86>)
        }
        Err(v1) => {
            Ok(Some(Adt86::C9(f35(v1))))
        }
    }
}

pub fn f13(v0: Adt10, v1: List<Adt37>, v2: Adt22) -> Result<Adt34, Adt86> {
    let m20 = match v2 {
        Adt22::C0(_, h19) => {
            h19
        }
    };
    match m20 {
        None => {
            Ok::<Adt34, Adt86>(Adt34::C1(Adt36::C0(v0, None::<Str>, v1)))
        }
        Some(v3) => {
            Err::<Adt34, Adt86>(Adt86::C5(v3))
        }
    }
}

pub fn f14(v0: List<Adt37>, v1: Fn1, v2: Adt23) -> R<Result<Adt34, Adt86>> {
    let m22 = f36(match v2.clone() {
        Adt23::C0(_, h21, _, _, _) => {
            h21
        }
    }, v0);
    match m22 {
        None => {
            Ok(Err::<Adt34, Adt86>(Adt86::C0(match v2 {
                Adt23::C0(_, h23, _, _, _) => {
                    h23
                }
            })))
        }
        Some(v3) => {
            f37(v3, v1, v2)
        }
    }
}

pub fn f15(v0: Adt14, v1: Adt23) -> R<Result<Adt51, Adt86>> {
    f38(match v0 {
        Adt14::C0(h24, _) => {
            h24
        }
    }, v1)
}

pub fn f16() -> Result<Adt34, Adt86> {
    Err::<Adt34, Adt86>(Adt86::C9(Str::lit("the gateway offers tools only")))
}

pub fn f17(v0: u64) -> List<u64> {
    List::cons({
        let a28 = {
            let a25 = v0;
            let a26 = 16777216u64;
            let a27 = 0u64;
            nat_quot(a25, a26, a27)
        };
        let a29 = 256u64;
        let a30 = 0u64;
        nat_rem(a28, a29, a30)
    }, List::cons({
        let a34 = {
            let a31 = v0;
            let a32 = 65536u64;
            let a33 = 0u64;
            nat_quot(a31, a32, a33)
        };
        let a35 = 256u64;
        let a36 = 0u64;
        nat_rem(a34, a35, a36)
    }, List::cons({
        let a40 = {
            let a37 = v0;
            let a38 = 256u64;
            let a39 = 0u64;
            nat_quot(a37, a38, a39)
        };
        let a41 = 256u64;
        let a42 = 0u64;
        nat_rem(a40, a41, a42)
    }, List::cons({
        let a43 = v0;
        let a44 = 256u64;
        let a45 = 0u64;
        nat_rem(a43, a44, a45)
    }, List::<u64>::nil()))))
}

pub fn f18(v0: Bytes) -> R<List<u64>> {
    f39(v0.clone(), {
        let a46 = v0;
        length_bytes(a46)
    }, List::<u64>::nil())
}

pub fn f19(v0: Bytes) -> R<Result<Adt89, Adt90>> {
    f40(v0.clone(), f41(v0, 0u64)?)
}

pub fn f20() -> Adt88 {
    Adt88::C0(f42(f43()))
}

pub fn f21(v0: Adt0, v1: Fn0, v2: Adt89, v3: Result<Adt91, Adt90>) -> R<Adt88> {
    let m47 = v3;
    match m47 {
        Ok(v5) => {
            f44(v0, Fn2::F45(v1, v2), v5)
        }
        Err(_) => {
            Ok(f20())
        }
    }
}

pub fn f22(v0: Bytes) -> R<Result<Adt91, Adt90>> {
    let m48 = f41(v0.clone(), 0u64)?;
    match m48 {
        None => {
            Ok(Err::<Adt91, Adt90>(Adt90::C0(0u64)))
        }
        Some(v1) => {
            f46(v0.clone(), v1.clone(), f41(v0, match v1 {
                Adt94::C0(_, h49) => {
                    h49
                }
            })?)
        }
    }
}

pub fn f23(v0: List<Adt37>) -> R<Result<List<Adt12>, Adt93>> {
    let m50 = f47(v0.clone());
    match m50 {
        None => {
            f48(v0, List::<Adt12>::nil())
        }
        Some(v1) => {
            Ok(Err::<List<Adt12>, Adt93>(v1))
        }
    }
}

pub fn f24(v0: Str, v1: Str, v2: Adt5) -> Adt37 {
    Adt37::C0(v0, None::<Str>, None::<List<Adt2>>, Some(v1), v2, None::<Adt5>, None::<Adt38>, None::<Adt5>)
}

pub fn f25(v0: List<(Str, Adt6)>, v1: List<Str>) -> Adt5 {
    Adt5::C0(List::cons((Str::lit("type"), f49(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(v0)), List::cons((Str::lit("required"), f50(v1, List::<Adt6>::nil())), f27(Str::lit("additionalProperties"), f51(false))))))
}

pub fn f26() -> R<Adt6> {
    Ok(Adt6::C5(List::cons((Str::lit("type"), f49(Str::lit("string"))), f27(Str::lit("minLength"), f52(1u64)?))))
}

pub fn f27(v0: Str, v1: Adt6) -> List<(Str, Adt6)> {
    List::cons((v0, v1), List::<(Str, Adt6)>::nil())
}

pub fn f28(v0: Str) -> Adt6 {
    Adt6::C5(f27(Str::lit("type"), f49(v0)))
}

pub fn f29(v0: Str, v1: Str, v2: Adt5, v3: Adt5) -> Adt37 {
    Adt37::C0(v0, None::<Str>, None::<List<Adt2>>, Some(v1), v2, Some(v3), None::<Adt38>, None::<Adt5>)
}

pub fn f30() -> R<List<(Str, Adt6)>> {
    let a51 = f33()?;
    let a52 = f27(Str::lit("all"), f28(Str::lit("boolean")));
    Ok(append_list(a51, a52))
}

pub fn f31(v0: Str) -> Adt5 {
    f25(List::cons((Str::lit("spec"), f53(Str::lit("lexlean/command-result/1"))), List::cons((Str::lit("command"), f53(v0)), List::cons((Str::lit("success"), f28(Str::lit("boolean"))), List::cons((Str::lit("exit_code"), f28(Str::lit("integer"))), List::cons((Str::lit("modules"), f54()), List::cons((Str::lit("artifacts"), f54()), List::cons((Str::lit("diagnostics"), f55()), List::cons((Str::lit("source_id"), f28(Str::lit("string"))), List::cons((Str::lit("semantic_id"), f28(Str::lit("string"))), List::cons((Str::lit("build_id"), f28(Str::lit("string"))), List::<(Str, Adt6)>::nil())))))))))), List::cons(Str::lit("spec"), List::cons(Str::lit("command"), List::cons(Str::lit("success"), List::cons(Str::lit("exit_code"), List::cons(Str::lit("modules"), List::cons(Str::lit("artifacts"), List::cons(Str::lit("diagnostics"), List::<Str>::nil()))))))))
}

pub fn f32() -> Adt5 {
    Adt5::C0(List::cons((Str::lit("type"), f49(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(f27(Str::lit("spec"), f28(Str::lit("string"))))), f27(Str::lit("required"), f50(List::cons(Str::lit("spec"), List::<Str>::nil()), List::<Adt6>::nil())))))
}

pub fn f33() -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("project"), f26()?), f27(Str::lit("files"), f54())))
}

pub fn f34(v0: Str, v1: Str) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), f49(Str::lit("string"))), f27(Str::lit("enum"), Adt6::C4(List::cons(f49(v0), List::cons(f49(v1), List::<Adt6>::nil()))))))
}

pub fn f35(v0: Adt93) -> Str {
    let m53 = v0;
    match m53 {
        Adt93::C0(v1) => {
            let a54 = List::cons(Str::lit("tool name is not 1 to 64 characters of letters, digits, underscore, hyphen and dot:"), List::cons(v1, List::<Str>::nil()));
            let a55 = Str::lit(" ");
            join(a54, a55)
        }
        Adt93::C1(v2) => {
            let a56 = List::cons(Str::lit("tool name is used twice:"), List::cons(v2, List::<Str>::nil()));
            let a57 = Str::lit(" ");
            join(a56, a57)
        }
        Adt93::C2(v3) => {
            let a58 = List::cons(Str::lit("the schema dialect of tool"), List::cons(v3, List::cons(Str::lit("is not JSON Schema 2020-12"), List::<Str>::nil())));
            let a59 = Str::lit(" ");
            join(a58, a59)
        }
        Adt93::C3(v4) => {
            let a60 = List::cons(Str::lit("the input schema of tool"), List::cons(v4, List::cons(Str::lit("does not have type object"), List::<Str>::nil())));
            let a61 = Str::lit(" ");
            join(a60, a61)
        }
        Adt93::C4(v5, v6) => {
            let a62 = List::cons(Str::lit("tool"), List::cons(v5, List::cons(Str::lit("uses the unsupported schema keyword"), List::cons(v6, List::<Str>::nil()))));
            let a63 = Str::lit(" ");
            join(a62, a63)
        }
        Adt93::C5(v7, v8) => {
            let a64 = List::cons(Str::lit("tool"), List::cons(v7, List::cons(Str::lit("has a malformed schema keyword"), List::cons(v8, List::<Str>::nil()))));
            let a65 = Str::lit(" ");
            join(a64, a65)
        }
        Adt93::C6(v9, v10) => {
            let a66 = List::cons(Str::lit("tool"), List::cons(v9, List::cons(Str::lit("has an unresolved reference"), List::cons(v10, List::<Str>::nil()))));
            let a67 = Str::lit(" ");
            join(a66, a67)
        }
        Adt93::C7(v11) => {
            let a68 = List::cons(Str::lit("tool"), List::cons(v11, List::cons(Str::lit("has an x-mcp-header that is not text"), List::<Str>::nil())));
            let a69 = Str::lit(" ");
            join(a68, a69)
        }
        Adt93::C8(v12) => {
            let a70 = List::cons(Str::lit("tool"), List::cons(v12, List::cons(Str::lit("has an empty x-mcp-header"), List::<Str>::nil())));
            let a71 = Str::lit(" ");
            join(a70, a71)
        }
        Adt93::C9(v13) => {
            let a72 = List::cons(Str::lit("tool"), List::cons(v13, List::cons(Str::lit("has an x-mcp-header that is not an HTTP field name"), List::<Str>::nil())));
            let a73 = Str::lit(" ");
            join(a72, a73)
        }
        Adt93::C10(v14) => {
            let a74 = List::cons(Str::lit("tool"), List::cons(v14, List::cons(Str::lit("has an x-mcp-header that is not reached through properties alone"), List::<Str>::nil())));
            let a75 = Str::lit(" ");
            join(a74, a75)
        }
        Adt93::C11(v15) => {
            let a76 = List::cons(Str::lit("tool"), List::cons(v15, List::cons(Str::lit("has an x-mcp-header on a parameter that is not a string, integer or boolean"), List::<Str>::nil())));
            let a77 = Str::lit(" ");
            join(a76, a77)
        }
        Adt93::C12(v16, v17) => {
            let a78 = List::cons(Str::lit("tool"), List::cons(v16, List::cons(Str::lit("repeats the x-mcp-header"), List::cons(v17, List::<Str>::nil()))));
            let a79 = Str::lit(" ");
            join(a78, a79)
        }
    }
}

pub fn f36(v0: Str, p85: List<Adt37>) -> Option<Adt37> {
    let m84 = p85;
    {
        let mut m84 = m84;
        loop {
            let step = {
                let v1 = m84;
                let m80 = v1;
                match m80.uncons() {
                    None => {
                        Err::<List<Adt37>, Option<Adt37>>(None::<Adt37>)
                    }
                    Some((v2, v3)) => {
                        let a82 = match v2.clone() {
                            Adt37::C0(h81, _, _, _, _, _, _, _) => {
                                h81
                            }
                        };
                        let a83 = v0.clone();
                        if equal(a82, a83) {
                            Err::<List<Adt37>, Option<Adt37>>(Some(v2))
                        } else {
                            Ok::<List<Adt37>, Option<Adt37>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m84 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f37(v0: Adt37, v1: Fn1, v2: Adt23) -> R<Result<Adt34, Adt86>> {
    f56(v0.clone(), v2, Fn3::F57(v1, v0))
}

pub fn f38(v0: List<Adt15>, v1: Adt23) -> R<Result<Adt51, Adt86>> {
    let m86 = f58(v1)?;
    match m86 {
        None => {
            Ok(Err::<Adt51, Adt86>(Adt86::C4(Str::lit("a file name holds a line feed, a draft is over draft.max_bytes, or all is true with files"))))
        }
        Some(v2) => {
            let m87 = v0;
            match m87.uncons() {
                None => {
                    Ok(f59(f60(match v2.clone() {
                        Adt96::C0(h88, _) => {
                            h88
                        }
                    }), match v2 {
                        Adt96::C0(_, h89) => {
                            h89
                        }
                    }))
                }
                Some((v3, v4)) => {
                    let m90 = v4;
                    match m90.uncons() {
                        None => {
                            let a93 = match v3.clone() {
                                Adt15::C0(h91, _) => {
                                    h91
                                }
                            };
                            let a94 = f60(match v2.clone() {
                                Adt96::C0(h92, _) => {
                                    h92
                                }
                            });
                            if nat_eq(a93, a94) {
                                f61(match v2 {
                                    Adt96::C0(h95, _) => {
                                        h95
                                    }
                                }, match v3 {
                                    Adt15::C0(_, h96) => {
                                        h96
                                    }
                                })
                            } else {
                                Ok(f62(Str::lit("the host performed another effect")))
                            }
                        }
                        Some((_, _)) => {
                            Ok(f62(Str::lit("the host performed more than one effect")))
                        }
                    }
                }
            }
        }
    }
}

pub fn f39(v0: Bytes, p99: u64, p100: List<u64>) -> R<List<u64>> {
    let m98 = (p99, p100);
    {
        let mut m98 = m98;
        loop {
            let step = {
                let (v1, v2) = m98;
                let m97 = v1;
                if m97 == 0 {
                    Err::<(u64, List<u64>), R<List<u64>>>(Ok(v2))
                } else {
                    let v3 = m97 - 1;
                    Ok::<(u64, List<u64>), R<List<u64>>>((v3, List::cons(f63(v0.clone(), v3)?, v2)))
                }
            };
            match step {
                Ok(next) => { m98 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f40(v0: Bytes, v1: Option<Adt94>) -> R<Result<Adt89, Adt90>> {
    let m101 = v1;
    match m101 {
        None => {
            Ok(Err::<Adt89, Adt90>(Adt90::C0(0u64)))
        }
        Some(v2) => {
            f64(v0.clone(), v2.clone(), f65(v0, match v2 {
                Adt94::C0(_, h102) => {
                    h102
                }
            })?)
        }
    }
}

pub fn f41(v0: Bytes, v1: u64) -> R<Option<Adt94>> {
    let m103 = f65(v0.clone(), v1)?;
    match m103 {
        None => {
            Ok(None::<Adt94>)
        }
        Some(v2) => {
            let m109 = {
                let a106 = v0;
                let a107 = {
                    let a104 = v1;
                    let a105 = 4u64;
                    nat_add(a104, a105)?
                };
                let a108 = v2;
                slice_bytes(a106, a107, a108)
            };
            match m109 {
                None => {
                    Ok(None::<Adt94>)
                }
                Some(v3) => {
                    Ok(Some(Adt94::C0(v3, {
                        let a112 = {
                            let a110 = v1;
                            let a111 = 4u64;
                            nat_add(a110, a111)?
                        };
                        let a113 = v2;
                        nat_add(a112, a113)?
                    })))
                }
            }
        }
    }
}

pub fn f42(v0: Adt95) -> List<u64> {
    let a134 = f17(match v0.clone() {
        Adt95::C0(h114, _, _, _) => {
            h114
        }
    });
    let a135 = {
        let a132 = f17({
            let a116 = match v0.clone() {
                Adt95::C0(_, h115, _, _) => {
                    h115
                }
            };
            length_list(a116)
        });
        let a133 = {
            let a130 = f66(match v0.clone() {
                Adt95::C0(_, h117, _, _) => {
                    h117
                }
            });
            let a131 = {
                let a128 = f17({
                    let a119 = match v0.clone() {
                        Adt95::C0(_, _, h118, _) => {
                            h118
                        }
                    };
                    length_list(a119)
                });
                let a129 = {
                    let a126 = f67(match v0.clone() {
                        Adt95::C0(_, _, h120, _) => {
                            h120
                        }
                    });
                    let a127 = {
                        let a124 = f17({
                            let a122 = match v0.clone() {
                                Adt95::C0(_, _, _, h121) => {
                                    h121
                                }
                            };
                            length_list(a122)
                        });
                        let a125 = match v0 {
                            Adt95::C0(_, _, _, h123) => {
                                h123
                            }
                        };
                        append_list(a124, a125)
                    };
                    append_list(a126, a127)
                };
                append_list(a128, a129)
            };
            append_list(a130, a131)
        };
        append_list(a132, a133)
    };
    append_list(a134, a135)
}

pub fn f43() -> Adt95 {
    Adt95::C0(f68(), f69(), List::<List<u64>>::nil(), f70())
}

pub fn f44(v0: Adt0, v1: Fn2, v2: Adt91) -> R<Adt88> {
    f71(v0, v1, v2.clone(), f72(match v2.clone() {
        Adt91::C0(_, _, h136, _, _, _, _) => {
            h136
        }
    }, match v2 {
        Adt91::C0(_, _, _, _, _, h137, _) => {
            h137
        }
    })?)
}

pub fn f45(v0: Fn0, v1: Adt89, v2: Adt16) -> R<Result<Adt34, Adt86>> {
    let c138: Fn0 = v0;
    c138.apply(Adt14::C0(match v1 {
        Adt89::C0(_, h139) => {
            h139
        }
    }, v2))
}

pub fn f46(v0: Bytes, v1: Adt94, v2: Option<Adt94>) -> R<Result<Adt91, Adt90>> {
    let m140 = v2;
    match m140 {
        None => {
            Ok(Err::<Adt91, Adt90>(Adt90::C0(match v1 {
                Adt94::C0(_, h141) => {
                    h141
                }
            })))
        }
        Some(v3) => {
            f73(v0.clone(), v1, v3.clone(), f65(v0, match v3 {
                Adt94::C0(_, h142) => {
                    h142
                }
            })?)
        }
    }
}

pub fn f47(v0: List<Adt37>) -> Option<Adt93> {
    let m143 = f74(v0, List::<Str>::nil());
    match m143 {
        None => {
            None::<Adt93>
        }
        Some(v1) => {
            Some(Adt93::C1(v1))
        }
    }
}

pub fn f48(p147: List<Adt37>, p148: List<Adt12>) -> R<Result<List<Adt12>, Adt93>> {
    let m146 = (p147, p148);
    {
        let mut m146 = m146;
        loop {
            let step = {
                let (v0, v1) = m146;
                let m144 = v0;
                match m144.uncons() {
                    None => {
                        Err::<(List<Adt37>, List<Adt12>), R<Result<List<Adt12>, Adt93>>>(Ok(Ok::<List<Adt12>, Adt93>(f75(v1, List::<Adt12>::nil()))))
                    }
                    Some((v2, v3)) => {
                        let m145 = f76(v2)?;
                        match m145 {
                            Ok(v5) => {
                                Ok::<(List<Adt37>, List<Adt12>), R<Result<List<Adt12>, Adt93>>>((v3, f75(v5, v1)))
                            }
                            Err(v4) => {
                                Err::<(List<Adt37>, List<Adt12>), R<Result<List<Adt12>, Adt93>>>(Ok(Err::<List<Adt12>, Adt93>(v4)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m146 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f49(v0: Str) -> Adt6 {
    Adt6::C3(v0)
}

pub fn f50(p153: List<Str>, p154: List<Adt6>) -> Adt6 {
    let m152 = (p153, p154);
    {
        let mut m152 = m152;
        loop {
            let step = {
                let (v0, v1) = m152;
                let m149 = v0;
                match m149.uncons() {
                    None => {
                        Err::<(List<Str>, List<Adt6>), Adt6>(Adt6::C4(v1))
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Str>, List<Adt6>), Adt6>((v3, {
                            let a150 = v1;
                            let a151 = List::cons(f49(v2), List::<Adt6>::nil());
                            append_list(a150, a151)
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m152 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f51(v0: bool) -> Adt6 {
    Adt6::C1(v0)
}

pub fn f52(v0: u64) -> R<Adt6> {
    Ok(Adt6::C2(Adt7::C0(f77(v0, 0i64)?, 0i64)))
}

pub fn f53(v0: Str) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), f49(Str::lit("string"))), f27(Str::lit("const"), f49(v0))))
}

pub fn f54() -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), f49(Str::lit("array"))), f27(Str::lit("items"), f28(Str::lit("string")))))
}

pub fn f55() -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), f49(Str::lit("array"))), f27(Str::lit("items"), f78())))
}

pub fn f56(v0: Adt37, v1: Adt23, v2: Fn3) -> R<Result<Adt34, Adt86>> {
    let m161 = f79(Adt6::C5({
        let m157 = match v0.clone() {
            Adt37::C0(_, _, _, _, h156, _, _, _) => {
                h156
            }
        };
        match m157 {
            Adt5::C0(h155) => {
                h155
            }
        }
    }), Adt6::C5({
        let m160 = match v0 {
            Adt37::C0(_, _, _, _, h159, _, _, _) => {
                h159
            }
        };
        match m160 {
            Adt5::C0(h158) => {
                h158
            }
        }
    }), f80(v1.clone()))?;
    match m161 {
        Adt99::C0 => {
            let c162: Fn3 = v2;
            c162.apply(v1)
        }
        Adt99::C1(v3, v4) => {
            Ok(Ok::<Adt34, Adt86>(Adt34::C2(f81(f82(v3, v4)?, true))))
        }
        Adt99::C2(_) => {
            Ok(Err::<Adt34, Adt86>(Adt86::C9(Str::lit("the inputSchema is malformed"))))
        }
        Adt99::C3(_) => {
            Ok(Err::<Adt34, Adt86>(Adt86::C9(Str::lit("the inputSchema has an unresolved reference"))))
        }
        Adt99::C4 => {
            Ok(Err::<Adt34, Adt86>(Adt86::C9(Str::lit("the inputSchema exceeded the validation budget"))))
        }
    }
}

pub fn f57(v0: Fn1, v1: Adt37, v2: Adt23) -> R<Result<Adt34, Adt86>> {
    f83(v1, v0, v2)
}

pub fn f58(v0: Adt23) -> R<Option<Adt96>> {
    if f84(v0.clone(), Str::lit("lexlean_draft_write")) {
        let m163 = f85(v0.clone(), Str::lit("path"));
        match m163 {
            None => {
                Ok(None::<Adt96>)
            }
            Some(v1) => {
                let m164 = f85(v0, Str::lit("content"));
                match m164 {
                    None => {
                        Ok(None::<Adt96>)
                    }
                    Some(v2) => {
                        let a166 = f86();
                        let a167 = {
                            let a165 = f87(v2.clone())?;
                            length_list(a165)
                        };
                        if nat_lt(a166, a167) {
                            Ok(None::<Adt96>)
                        } else {
                            Ok(Some(Adt96::C0(Adt97::C1, {
                                let a168 = f88(v1)?;
                                let a169 = f89(f87(v2)?);
                                append_list(a168, a169)
                            })))
                        }
                    }
                }
            }
        }
    } else {
        if f84(v0.clone(), Str::lit("lexlean_draft_read")) {
            let m170 = f85(v0, Str::lit("path"));
            match m170 {
                None => {
                    Ok(None::<Adt96>)
                }
                Some(v3) => {
                    Ok(Some(Adt96::C0(Adt97::C0, f88(v3)?)))
                }
            }
        } else {
            if f84(v0.clone(), Str::lit("lexlean_check")) {
                f90(Adt97::C2, v0, List::<u64>::nil())
            } else {
                if f84(v0.clone(), Str::lit("lexlean_snapshot")) {
                    f90(Adt97::C3, v0, List::<u64>::nil())
                } else {
                    if f84(v0.clone(), Str::lit("lexlean_format")) {
                        f90(Adt97::C4, v0.clone(), f91(f92(v0, Str::lit("check_only"))))
                    } else {
                        if f84(v0.clone(), Str::lit("lexlean_build")) {
                            f90(Adt97::C5, v0, List::<u64>::nil())
                        } else {
                            if f84(v0.clone(), Str::lit("lexlean_verify")) {
                                f90(Adt97::C6, v0, List::<u64>::nil())
                            } else {
                                if f84(v0.clone(), Str::lit("lexlean_verify_control")) {
                                    let m171 = f85(v0.clone(), Str::lit("task"));
                                    match m171 {
                                        None => {
                                            Ok(None::<Adt96>)
                                        }
                                        Some(v4) => {
                                            let m172 = f85(v0, Str::lit("action"));
                                            match m172 {
                                                None => {
                                                    Ok(None::<Adt96>)
                                                }
                                                Some(v5) => {
                                                    Ok(Some(Adt96::C0(Adt97::C7, {
                                                        let a175 = f93(v4)?;
                                                        let a176 = f17({
                                                            let a173 = v5;
                                                            let a174 = Str::lit("cancel");
                                                            if equal(a173, a174) {
                                                                1u64
                                                            } else {
                                                                0u64
                                                            }
                                                        });
                                                        append_list(a175, a176)
                                                    })))
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    if f84(v0.clone(), Str::lit("lexlean_lexicon_read")) {
                                        let m177 = f85(v0.clone(), Str::lit("profile"));
                                        match m177 {
                                            None => {
                                                Ok(None::<Adt96>)
                                            }
                                            Some(v6) => {
                                                let m178 = f85(v0, Str::lit("entry"));
                                                match m178 {
                                                    None => {
                                                        Ok(None::<Adt96>)
                                                    }
                                                    Some(v7) => {
                                                        Ok(Some(Adt96::C0(Adt97::C8, {
                                                            let a179 = f93(v6)?;
                                                            let a180 = f93(v7)?;
                                                            append_list(a179, a180)
                                                        })))
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        if f84(v0.clone(), Str::lit("lexlean_attestation")) {
                                            let m181 = f85(v0.clone(), Str::lit("by"));
                                            match m181 {
                                                None => {
                                                    Ok(None::<Adt96>)
                                                }
                                                Some(v8) => {
                                                    let m182 = f85(v0, Str::lit("value"));
                                                    match m182 {
                                                        None => {
                                                            Ok(None::<Adt96>)
                                                        }
                                                        Some(v9) => {
                                                            Ok(Some(Adt96::C0(Adt97::C9, {
                                                                let a185 = f17({
                                                                    let a183 = v8;
                                                                    let a184 = Str::lit("digest");
                                                                    if equal(a183, a184) {
                                                                        1u64
                                                                    } else {
                                                                        0u64
                                                                    }
                                                                });
                                                                let a186 = f93(v9)?;
                                                                append_list(a185, a186)
                                                            })))
                                                        }
                                                    }
                                                }
                                            }
                                        } else {
                                            Ok(None::<Adt96>)
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f59(v0: u64, v1: List<u64>) -> Result<Adt51, Adt86> {
    Err::<Adt51, Adt86>(Adt86::C15(Adt87::C0(v0, f94(f95(), v1))))
}

pub fn f60(v0: Adt97) -> u64 {
    let m187 = v0;
    match m187 {
        Adt97::C0 => {
            0u64
        }
        Adt97::C1 => {
            1u64
        }
        Adt97::C2 => {
            2u64
        }
        Adt97::C3 => {
            3u64
        }
        Adt97::C4 => {
            4u64
        }
        Adt97::C5 => {
            5u64
        }
        Adt97::C6 => {
            6u64
        }
        Adt97::C7 => {
            7u64
        }
        Adt97::C8 => {
            8u64
        }
        Adt97::C9 => {
            9u64
        }
    }
}

pub fn f61(v0: Adt97, v1: Bytes) -> R<Result<Adt51, Adt86>> {
    let a188 = f96(v1.clone(), 0u64)?;
    let a189 = 1u64;
    if nat_eq(a188, a189) {
        Ok(f97(f98(v0, f96(v1, 4u64)?)))
    } else {
        let m190 = v0.clone();
        match m190 {
            Adt97::C0 => {
                Ok(f97(f99(v1)?))
            }
            Adt97::C1 => {
                Ok(f97(f100(v1)?))
            }
            Adt97::C2 => {
                f101(v0, v1)
            }
            Adt97::C3 => {
                f101(v0, v1)
            }
            Adt97::C4 => {
                f101(v0, v1)
            }
            Adt97::C5 => {
                f101(v0, v1)
            }
            Adt97::C6 => {
                Ok(f97(f102(v1)?))
            }
            Adt97::C7 => {
                Ok(f97(f103(v1)?))
            }
            Adt97::C8 => {
                Ok(f97(f99(v1)?))
            }
            Adt97::C9 => {
                Ok(f97(f99(v1)?))
            }
        }
    }
}

pub fn f62(v0: Str) -> Result<Adt51, Adt86> {
    Err::<Adt51, Adt86>(Adt86::C9(v0))
}

pub fn f63(v0: Bytes, v1: u64) -> R<u64> {
    let m193 = {
        let a191 = v0;
        let a192 = v1;
        index_bytes(a191, a192)
    };
    match m193 {
        None => {
            Ok(0u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a238 = {
                let a214 = {
                    let a202 = {
                        let a196 = {
                            let a194 = v3;
                            let a195 = 128u8;
                            fixed_u8::bit_and(a194, a195)
                        };
                        let a197 = 128u8;
                        if equal(a196, a197) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a203 = {
                        let a200 = {
                            let a198 = v3;
                            let a199 = 64u8;
                            fixed_u8::bit_and(a198, a199)
                        };
                        let a201 = 64u8;
                        if equal(a200, a201) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a202, a203)?
                };
                let a215 = {
                    let a212 = {
                        let a206 = {
                            let a204 = v3;
                            let a205 = 32u8;
                            fixed_u8::bit_and(a204, a205)
                        };
                        let a207 = 32u8;
                        if equal(a206, a207) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a213 = {
                        let a210 = {
                            let a208 = v3;
                            let a209 = 16u8;
                            fixed_u8::bit_and(a208, a209)
                        };
                        let a211 = 16u8;
                        if equal(a210, a211) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a212, a213)?
                };
                nat_add(a214, a215)?
            };
            let a239 = {
                let a236 = {
                    let a224 = {
                        let a218 = {
                            let a216 = v3;
                            let a217 = 8u8;
                            fixed_u8::bit_and(a216, a217)
                        };
                        let a219 = 8u8;
                        if equal(a218, a219) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a225 = {
                        let a222 = {
                            let a220 = v3;
                            let a221 = 4u8;
                            fixed_u8::bit_and(a220, a221)
                        };
                        let a223 = 4u8;
                        if equal(a222, a223) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a224, a225)?
                };
                let a237 = {
                    let a234 = {
                        let a228 = {
                            let a226 = v3;
                            let a227 = 2u8;
                            fixed_u8::bit_and(a226, a227)
                        };
                        let a229 = 2u8;
                        if equal(a228, a229) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a235 = {
                        let a232 = {
                            let a230 = v3;
                            let a231 = 1u8;
                            fixed_u8::bit_and(a230, a231)
                        };
                        let a233 = 1u8;
                        if equal(a232, a233) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a234, a235)?
                };
                nat_add(a236, a237)?
            };
            nat_add(a238, a239)
        }
    }
}

pub fn f64(v0: Bytes, v1: Adt94, v2: Option<u64>) -> R<Result<Adt89, Adt90>> {
    let m240 = v2;
    match m240 {
        None => {
            Ok(Err::<Adt89, Adt90>(Adt90::C0(match v1 {
                Adt94::C0(_, h241) => {
                    h241
                }
            })))
        }
        Some(v3) => {
            let m245 = f104(v0.clone(), {
                let a243 = match v1.clone() {
                    Adt94::C0(_, h242) => {
                        h242
                    }
                };
                let a244 = 4u64;
                nat_add(a243, a244)?
            }, v3, List::<Adt15>::nil())?;
            match m245 {
                Ok(v5) => {
                    Ok(f105(v0, match v1 {
                        Adt94::C0(h246, _) => {
                            h246
                        }
                    }, v5))
                }
                Err(v4) => {
                    Ok(Err::<Adt89, Adt90>(v4))
                }
            }
        }
    }
}

pub fn f65(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a250 = {
        let a247 = v1;
        let a248 = 4u64;
        nat_add(a247, a248)?
    };
    let a251 = {
        let a249 = v0.clone();
        length_bytes(a249)
    };
    if nat_le(a250, a251) {
        Ok(Some({
            let a268 = {
                let a252 = f63(v0.clone(), v1)?;
                let a253 = 16777216u64;
                nat_mul(a252, a253)?
            };
            let a269 = {
                let a266 = {
                    let a256 = f63(v0.clone(), {
                        let a254 = v1;
                        let a255 = 1u64;
                        nat_add(a254, a255)?
                    })?;
                    let a257 = 65536u64;
                    nat_mul(a256, a257)?
                };
                let a267 = {
                    let a264 = {
                        let a260 = f63(v0.clone(), {
                            let a258 = v1;
                            let a259 = 2u64;
                            nat_add(a258, a259)?
                        })?;
                        let a261 = 256u64;
                        nat_mul(a260, a261)?
                    };
                    let a265 = f63(v0, {
                        let a262 = v1;
                        let a263 = 3u64;
                        nat_add(a262, a263)?
                    })?;
                    nat_add(a264, a265)?
                };
                nat_add(a266, a267)?
            };
            nat_add(a268, a269)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f66(v0: List<Adt92>) -> List<u64> {
    f106(f107(v0, List::<Adt92>::nil()), List::<u64>::nil())
}

pub fn f67(v0: List<List<u64>>) -> List<u64> {
    f108(f109(v0, List::<List<u64>>::nil()), List::<u64>::nil())
}

pub fn f68() -> u64 {
    500u64
}

pub fn f69() -> List<Adt92> {
    List::cons(Adt92::C0(f110(), f111()), List::<Adt92>::nil())
}

pub fn f70() -> List<u64> {
    let a272 = f112();
    let a273 = {
        let a270 = f113();
        let a271 = f114();
        append_list(a270, a271)
    };
    append_list(a272, a273)
}

pub fn f71(v0: Adt0, v1: Fn2, v2: Adt91, v3: Option<Adt98>) -> R<Adt88> {
    let m274 = v3;
    match m274 {
        None => {
            f116(v0, v1, v2)
        }
        Some(v4) => {
            Ok(Adt88::C0(f42(f115(v4)?)))
        }
    }
}

pub fn f72(v0: List<Adt92>, v1: List<List<u64>>) -> R<Option<Adt98>> {
    let m275 = f117(v0.clone(), v1.clone())?;
    match m275 {
        None => {
            f118(v0, v1)
        }
        Some(v2) => {
            Ok(Some(v2))
        }
    }
}

pub fn f73(v0: Bytes, v1: Adt94, v2: Adt94, v3: Option<u64>) -> R<Result<Adt91, Adt90>> {
    let m276 = v3;
    match m276 {
        None => {
            Ok(Err::<Adt91, Adt90>(Adt90::C0(match v2 {
                Adt94::C0(_, h277) => {
                    h277
                }
            })))
        }
        Some(v4) => {
            let m281 = f119(v0.clone(), {
                let a279 = match v2.clone() {
                    Adt94::C0(_, h278) => {
                        h278
                    }
                };
                let a280 = 4u64;
                nat_add(a279, a280)?
            }, v4, List::<Adt92>::nil())?;
            match m281 {
                Ok(v6) => {
                    f120(v0, v1, v2, v6)
                }
                Err(v5) => {
                    Ok(Err::<Adt91, Adt90>(v5))
                }
            }
        }
    }
}

pub fn f74(p287: List<Adt37>, p288: List<Str>) -> Option<Str> {
    let m286 = (p287, p288);
    {
        let mut m286 = m286;
        loop {
            let step = {
                let (v0, v1) = m286;
                let m282 = v0;
                match m282.uncons() {
                    None => {
                        Err::<(List<Adt37>, List<Str>), Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        if f121(match v2.clone() {
                            Adt37::C0(h283, _, _, _, _, _, _, _) => {
                                h283
                            }
                        }, v1.clone()) {
                            Err::<(List<Adt37>, List<Str>), Option<Str>>(Some(match v2 {
                                Adt37::C0(h284, _, _, _, _, _, _, _) => {
                                    h284
                                }
                            }))
                        } else {
                            Ok::<(List<Adt37>, List<Str>), Option<Str>>((v3, List::cons(match v2 {
                                Adt37::C0(h285, _, _, _, _, _, _, _) => {
                                    h285
                                }
                            }, v1)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m286 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f75(p291: List<Adt12>, p292: List<Adt12>) -> List<Adt12> {
    let m290 = (p291, p292);
    {
        let mut m290 = m290;
        loop {
            let step = {
                let (v0, v1) = m290;
                let m289 = v0;
                match m289.uncons() {
                    None => {
                        Err::<(List<Adt12>, List<Adt12>), List<Adt12>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt12>, List<Adt12>), List<Adt12>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m290 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f76(v0: Adt37) -> R<Result<List<Adt12>, Adt93>> {
    let m295 = f122(match v0.clone() {
        Adt37::C0(h293, _, _, _, _, _, _, _) => {
            h293
        }
    }, f123(match v0.clone() {
        Adt37::C0(h294, _, _, _, _, _, _, _) => {
            h294
        }
    })?);
    match m295 {
        None => {
            f124(v0)
        }
        Some(v1) => {
            Ok(Err::<List<Adt12>, Adt93>(v1))
        }
    }
}

pub fn f77(p300: u64, p301: i64) -> R<i64> {
    let m299 = (p300, p301);
    {
        let mut m299 = m299;
        loop {
            let step = {
                let (v0, v1) = m299;
                let m296 = v0;
                if m296 == 0 {
                    Err::<(u64, i64), R<i64>>(Ok(v1))
                } else {
                    let v2 = m296 - 1;
                    Ok::<(u64, i64), R<i64>>((v2, {
                        let a297 = v1;
                        let a298 = -1i64;
                        int_sub(a297, a298)?
                    }))
                }
            };
            match step {
                Ok(next) => { m299 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f78() -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), f49(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(List::cons((Str::lit("code"), f28(Str::lit("string"))), f27(Str::lit("message"), f28(Str::lit("string")))))), f27(Str::lit("required"), f50(List::cons(Str::lit("code"), List::cons(Str::lit("message"), List::<Str>::nil())), List::<Adt6>::nil())))))
}

pub fn f79(v0: Adt6, v1: Adt6, v2: Adt6) -> R<Adt99> {
    f125(v0, f126(), Adt101::C0, List::cons(Adt102::C0(v1, v2, List::<Adt100>::nil()), List::<Adt102>::nil()))
}

pub fn f80(v0: Adt23) -> Adt6 {
    let m303 = match v0 {
        Adt23::C0(_, _, h302, _, _) => {
            h302
        }
    };
    match m303 {
        None => {
            Adt6::C5(List::<(Str, Adt6)>::nil())
        }
        Some(v1) => {
            Adt6::C5(match v1 {
                Adt5::C0(h304) => {
                    h304
                }
            })
        }
    }
}

pub fn f81(v0: Str, v1: bool) -> Adt39 {
    Adt39::C0(List::cons(Adt40::C0(Adt41::C0(v0, None::<Adt42>, None::<Adt5>)), List::<Adt40>::nil()), None::<Adt6>, if v1 {
        Some(true)
    } else {
        None::<bool>
    })
}

pub fn f82(v0: List<Adt100>, v1: Str) -> R<Str> {
    let v2: Str = f127(v0)?;
    let a307 = List::cons(Str::lit("Invalid arguments:"), List::cons({
        let a305 = v2.clone();
        let a306 = Str::lit("");
        if equal(a305, a306) {
            Str::lit("the arguments object")
        } else {
            v2
        }
    }, List::cons(Str::lit("fails"), List::cons(v1, List::<Str>::nil()))));
    let a308 = Str::lit(" ");
    Ok(join(a307, a308))
}

pub fn f83(v0: Adt37, v1: Fn1, v2: Adt23) -> R<Result<Adt34, Adt86>> {
    let m310 = {
        let c309: Fn1 = v1;
        c309.apply(v2)?
    };
    match m310 {
        Ok(v4) => {
            f128(v0, v4)
        }
        Err(v3) => {
            Ok(Err::<Adt34, Adt86>(v3))
        }
    }
}

pub fn f84(v0: Adt23, v1: Str) -> bool {
    let a312 = match v0 {
        Adt23::C0(_, h311, _, _, _) => {
            h311
        }
    };
    let a313 = v1;
    equal(a312, a313)
}

pub fn f85(v0: Adt23, v1: Str) -> Option<Str> {
    let m314 = f129(v0, v1);
    match m314 {
        None => {
            None::<Str>
        }
        Some(v2) => {
            f130(v2)
        }
    }
}

pub fn f86() -> u64 {
    8388608u64
}

pub fn f87(v0: Str) -> R<List<u64>> {
    f18({
        let a315 = v0;
        utf8_encode(a315)
    })
}

pub fn f88(v0: Str) -> R<List<u64>> {
    let a316 = f17(f131(Str::lit("drafts")));
    let a317 = f93(v0)?;
    Ok(append_list(a316, a317))
}

pub fn f89(v0: List<u64>) -> List<u64> {
    let a319 = f17({
        let a318 = v0.clone();
        length_list(a318)
    });
    let a320 = v0;
    append_list(a319, a320)
}

pub fn f90(v0: Adt97, v1: Adt23, v2: List<u64>) -> R<Option<Adt96>> {
    let m321 = f85(v1.clone(), Str::lit("project"));
    match m321 {
        None => {
            Ok(None::<Adt96>)
        }
        Some(v3) => {
            let m322 = f132(v1.clone());
            match m322 {
                None => {
                    Ok(None::<Adt96>)
                }
                Some(v4) => {
                    if f133(v4.clone())? {
                        Ok(None::<Adt96>)
                    } else {
                        if f134(f92(v1.clone(), Str::lit("all")), v4.clone()) {
                            Ok(None::<Adt96>)
                        } else {
                            Ok(Some(Adt96::C0(v0, {
                                let a325 = f93(v3)?;
                                let a326 = {
                                    let a323 = f135(f92(v1, Str::lit("all")), v4)?;
                                    let a324 = v2;
                                    append_list(a323, a324)
                                };
                                append_list(a325, a326)
                            })))
                        }
                    }
                }
            }
        }
    }
}

pub fn f91(v0: bool) -> List<u64> {
    if v0 {
        List::cons(1u64, List::<u64>::nil())
    } else {
        List::cons(0u64, List::<u64>::nil())
    }
}

pub fn f92(v0: Adt23, v1: Str) -> bool {
    let m327 = f129(v0, v1);
    match m327 {
        None => {
            false
        }
        Some(v2) => {
            let m328 = f136(v2);
            match m328 {
                None => {
                    false
                }
                Some(v3) => {
                    v3
                }
            }
        }
    }
}

pub fn f93(v0: Str) -> R<List<u64>> {
    Ok(f89(f87(v0)?))
}

pub fn f94(v0: Bytes, v1: List<u64>) -> Bytes {
    let a329 = v0;
    let a330 = f137(f95(), f138(List::<Bytes>::nil(), v1));
    append_bytes(a329, a330)
}

pub fn f95() -> Bytes {
    Bytes::lit(&[])
}

pub fn f96(v0: Bytes, v1: u64) -> R<u64> {
    let m331 = f65(v0, v1)?;
    match m331 {
        None => {
            Ok(0u64)
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f97(v0: Adt51) -> Result<Adt51, Adt86> {
    Ok::<Adt51, Adt86>(v0)
}

pub fn f98(v0: Adt97, v1: u64) -> Adt51 {
    let v2: Str = f139(f140(v0.clone()), v1);
    f141({
        let a332 = List::cons(f142(v0), List::cons(Str::lit("failed:"), List::cons(v2.clone(), List::<Str>::nil())));
        let a333 = Str::lit(" ");
        join(a332, a333)
    }, Adt6::C5(f27(Str::lit("failure"), f49(v2))), true)
}

pub fn f99(v0: Bytes) -> R<Adt51> {
    let m334 = f41(v0.clone(), 5u64)?;
    match m334 {
        None => {
            Ok(f143())
        }
        Some(v1) => {
            let a335 = f63(v0, 4u64)?;
            let a336 = 1u64;
            if nat_eq(a335, a336) {
                Ok(f141(f144(match v1 {
                    Adt94::C0(h337, _) => {
                        h337
                    }
                }), Adt6::C5(f27(Str::lit("found"), f51(true))), false))
            } else {
                Ok(f141(Str::lit("not found"), Adt6::C5(f27(Str::lit("found"), f51(false))), false))
            }
        }
    }
}

pub fn f100(v0: Bytes) -> R<Adt51> {
    let v1: u64 = {
        let a340 = {
            let a338 = f96(v0.clone(), 4u64)?;
            let a339 = 4294967296u64;
            nat_mul(a338, a339)?
        };
        let a341 = f96(v0, 8u64)?;
        nat_add(a340, a341)?
    };
    Ok(f141({
        let a343 = List::cons(Str::lit("written"), List::cons({
            let a342 = f145(v1, 0i64)?;
            format_i64(a342)
        }, List::cons(Str::lit("bytes"), List::<Str>::nil())));
        let a344 = Str::lit(" ");
        join(a343, a344)
    }, Adt6::C5(f27(Str::lit("length"), f52(v1)?)), false))
}

pub fn f101(v0: Adt97, v1: Bytes) -> R<Result<Adt51, Adt86>> {
    let m345 = f41(v1.clone(), 8u64)?;
    match m345 {
        None => {
            Ok(f97(f143()))
        }
        Some(v2) => {
            let m347 = f146(match v2.clone() {
                Adt94::C0(h346, _) => {
                    h346
                }
            })?;
            match m347 {
                Ok(v4) => {
                    let v5: u64 = f96(v1, 4u64)?;
                    let a348 = f142(v0);
                    let a349 = Str::lit("engine.snapshot");
                    if if equal(a348, a349) {
                        let a350 = v5;
                        let a351 = 0u64;
                        if nat_eq(a350, a351) {
                            f147(v4.clone())
                        } else {
                            f148(v4.clone(), v5)?
                        }
                    } else {
                        f148(v4.clone(), v5)?
                    } {
                        Ok(f97(f141(f144(match v2 {
                            Adt94::C0(h352, _) => {
                                h352
                            }
                        }), v4, {
                            let a353 = v5;
                            let a354 = 0u64;
                            !nat_eq(a353, a354)
                        })))
                    } else {
                        Ok(f62(Str::lit("the engine answered with a document that is neither the snapshot nor the command result of its class")))
                    }
                }
                Err(_) => {
                    Ok(f62(Str::lit("the engine answered with bytes that are not JSON")))
                }
            }
        }
    }
}

pub fn f102(v0: Bytes) -> R<Adt51> {
    let m355 = f41(v0.clone(), 4u64)?;
    match m355 {
        None => {
            Ok(f143())
        }
        Some(v1) => {
            let m358 = {
                let a357 = match v1.clone() {
                    Adt94::C0(h356, _) => {
                        h356
                    }
                };
                utf8_decode(a357)
            };
            match m358 {
                None => {
                    Ok(f143())
                }
                Some(v2) => {
                    Ok(f141(v2.clone(), Adt6::C5(List::cons((Str::lit("task"), f49(v2)), f27(Str::lit("state"), f49(f149(f96(v0, match v1 {
                        Adt94::C0(_, h359) => {
                            h359
                        }
                    })?))))), false))
                }
            }
        }
    }
}

pub fn f103(v0: Bytes) -> R<Adt51> {
    let m360 = f41(v0.clone(), 12u64)?;
    match m360 {
        None => {
            Ok(f143())
        }
        Some(v1) => {
            Ok(f141(f144(match v1.clone() {
                Adt94::C0(h361, _) => {
                    h361
                }
            }), Adt6::C5(List::cons((Str::lit("state"), f49(f149(f96(v0.clone(), 4u64)?))), List::cons((Str::lit("class"), f52(f96(v0.clone(), 8u64)?)?), List::cons((Str::lit("groupsEnded"), f52(f96(v0.clone(), match v1.clone() {
                Adt94::C0(_, h362) => {
                    h362
                }
            })?)?), f27(Str::lit("groupsFailed"), f52(f96(v0.clone(), {
                let a364 = match v1 {
                    Adt94::C0(_, h363) => {
                        h363
                    }
                };
                let a365 = 4u64;
                nat_add(a364, a365)?
            })?)?))))), {
                let a366 = f96(v0, 8u64)?;
                let a367 = 0u64;
                !nat_eq(a366, a367)
            }))
        }
    }
}

pub fn f104(v0: Bytes, p378: u64, p379: u64, p380: List<Adt15>) -> R<Result<(List<Adt15>, u64), Adt90>> {
    let m377 = (p378, (p379, p380));
    {
        let mut m377 = m377;
        loop {
            let step = {
                let (v1, (v2, v3)) = m377;
                let m368 = v2;
                if m368 == 0 {
                    Err::<(u64, (u64, List<Adt15>)), R<Result<(List<Adt15>, u64), Adt90>>>(Ok(Ok::<(List<Adt15>, u64), Adt90>((f150(v3, List::<Adt15>::nil()), v1))))
                } else {
                    let v4 = m368 - 1;
                    let m369 = f65(v0.clone(), v1)?;
                    match m369 {
                        None => {
                            Err::<(u64, (u64, List<Adt15>)), R<Result<(List<Adt15>, u64), Adt90>>>(Ok(Err::<(List<Adt15>, u64), Adt90>(Adt90::C0(v1))))
                        }
                        Some(v5) => {
                            let m372 = f41(v0.clone(), {
                                let a370 = v1;
                                let a371 = 4u64;
                                nat_add(a370, a371)?
                            })?;
                            match m372 {
                                None => {
                                    Err::<(u64, (u64, List<Adt15>)), R<Result<(List<Adt15>, u64), Adt90>>>(Ok(Err::<(List<Adt15>, u64), Adt90>(Adt90::C0({
                                        let a373 = v1;
                                        let a374 = 4u64;
                                        nat_add(a373, a374)?
                                    }))))
                                }
                                Some(v6) => {
                                    Ok::<(u64, (u64, List<Adt15>)), R<Result<(List<Adt15>, u64), Adt90>>>((match v6.clone() {
                                        Adt94::C0(_, h375) => {
                                            h375
                                        }
                                    }, (v4, List::cons(Adt15::C0(v5, match v6 {
                                        Adt94::C0(h376, _) => {
                                            h376
                                        }
                                    }), v3))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m377 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f105(v0: Bytes, v1: Bytes, v2: (List<Adt15>, u64)) -> Result<Adt89, Adt90> {
    f151({
        let a383 = {
            let (_, h381) = v2.clone();
            h381
        };
        let a384 = {
            let a382 = v0;
            length_bytes(a382)
        };
        nat_eq(a383, a384)
    }, v1, {
        let (h385, _) = v2;
        h385
    }, {
        let (_, h386) = v2;
        h386
    })
}

pub fn f106(p391: List<Adt92>, p392: List<u64>) -> List<u64> {
    let m390 = (p391, p392);
    {
        let mut m390 = m390;
        loop {
            let step = {
                let (v0, v1) = m390;
                let m387 = v0;
                match m387.uncons() {
                    None => {
                        Err::<(List<Adt92>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt92>, List<u64>), List<u64>>((v3, {
                            let a388 = f152(v2);
                            let a389 = v1;
                            append_list(a388, a389)
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m390 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f107(p395: List<Adt92>, p396: List<Adt92>) -> List<Adt92> {
    let m394 = (p395, p396);
    {
        let mut m394 = m394;
        loop {
            let step = {
                let (v0, v1) = m394;
                let m393 = v0;
                match m393.uncons() {
                    None => {
                        Err::<(List<Adt92>, List<Adt92>), List<Adt92>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt92>, List<Adt92>), List<Adt92>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m394 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f108(p401: List<List<u64>>, p402: List<u64>) -> List<u64> {
    let m400 = (p401, p402);
    {
        let mut m400 = m400;
        loop {
            let step = {
                let (v0, v1) = m400;
                let m397 = v0;
                match m397.uncons() {
                    None => {
                        Err::<(List<List<u64>>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<u64>>, List<u64>), List<u64>>((v3, {
                            let a398 = f153(v2);
                            let a399 = v1;
                            append_list(a398, a399)
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m400 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f109(p405: List<List<u64>>, p406: List<List<u64>>) -> List<List<u64>> {
    let m404 = (p405, p406);
    {
        let mut m404 = m404;
        loop {
            let step = {
                let (v0, v1) = m404;
                let m403 = v0;
                match m403.uncons() {
                    None => {
                        Err::<(List<List<u64>>, List<List<u64>>), List<List<u64>>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<u64>>, List<List<u64>>), List<List<u64>>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m404 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f110() -> List<u64> {
    List::cons(67u64, List::cons(111u64, List::cons(110u64, List::cons(116u64, List::cons(101u64, List::cons(110u64, List::cons(116u64, List::cons(45u64, List::cons(84u64, List::cons(121u64, List::cons(112u64, List::cons(101u64, List::<u64>::nil()))))))))))))
}

pub fn f111() -> List<u64> {
    List::cons(97u64, List::cons(112u64, List::cons(112u64, List::cons(108u64, List::cons(105u64, List::cons(99u64, List::cons(97u64, List::cons(116u64, List::cons(105u64, List::cons(111u64, List::cons(110u64, List::cons(47u64, List::cons(106u64, List::cons(115u64, List::cons(111u64, List::cons(110u64, List::<u64>::nil()))))))))))))))))
}

pub fn f112() -> List<u64> {
    List::cons(123u64, List::cons(34u64, List::cons(106u64, List::cons(115u64, List::cons(111u64, List::cons(110u64, List::cons(114u64, List::cons(112u64, List::cons(99u64, List::cons(34u64, List::cons(58u64, List::cons(34u64, List::cons(50u64, List::cons(46u64, List::cons(48u64, List::cons(34u64, List::<u64>::nil()))))))))))))))))
}

pub fn f113() -> List<u64> {
    List::cons(44u64, List::cons(34u64, List::cons(101u64, List::cons(114u64, List::cons(114u64, List::cons(111u64, List::cons(114u64, List::cons(34u64, List::cons(58u64, List::cons(123u64, List::cons(34u64, List::cons(99u64, List::cons(111u64, List::cons(100u64, List::cons(101u64, List::cons(34u64, List::cons(58u64, List::cons(45u64, List::cons(51u64, List::cons(50u64, List::cons(54u64, List::cons(48u64, List::cons(51u64, List::cons(44u64, List::cons(34u64, List::cons(109u64, List::<u64>::nil()))))))))))))))))))))))))))
}

pub fn f114() -> List<u64> {
    List::cons(101u64, List::cons(115u64, List::cons(115u64, List::cons(97u64, List::cons(103u64, List::cons(101u64, List::cons(34u64, List::cons(58u64, List::cons(34u64, List::cons(73u64, List::cons(110u64, List::cons(116u64, List::cons(101u64, List::cons(114u64, List::cons(110u64, List::cons(97u64, List::cons(108u64, List::cons(32u64, List::cons(101u64, List::cons(114u64, List::cons(114u64, List::cons(111u64, List::cons(114u64, List::cons(34u64, List::cons(125u64, List::cons(125u64, List::<u64>::nil()))))))))))))))))))))))))))
}

pub fn f115(v0: Adt98) -> R<Adt95> {
    f154(f155(v0.clone()), f156(None::<Adt104>, f157(v0)))
}

pub fn f116(v0: Adt0, v1: Fn2, v2: Adt91) -> R<Adt88> {
    if f158(match v2.clone() {
        Adt91::C0(h407, _, _, _, _, _, _) => {
            h407
        }
    }) {
        f159(f160(v0, match v2.clone() {
            Adt91::C0(_, _, _, _, _, _, h408) => {
                h408
            }
        }), v1, match v2.clone() {
            Adt91::C0(_, _, _, _, h409, _, _) => {
                h409
            }
        }, match v2.clone() {
            Adt91::C0(_, _, h410, _, _, _, _) => {
                h410
            }
        }, f146(match v2 {
            Adt91::C0(_, _, _, h411, _, _, _) => {
                h411
            }
        })?)
    } else {
        Ok(Adt88::C0(f42(f161())))
    }
}

pub fn f117(v0: List<Adt92>, v1: List<List<u64>>) -> R<Option<Adt98>> {
    let m412 = f162(v0, f163())?;
    match m412 {
        Adt110::C0 => {
            Ok(None::<Adt98>)
        }
        Adt110::C1 => {
            Ok(Some(Adt98::C0))
        }
        Adt110::C2 => {
            Ok(Some(Adt98::C0))
        }
        Adt110::C3(v2) => {
            f164(f165(v2), v1, Adt98::C0)
        }
    }
}

pub fn f118(v0: List<Adt92>, v1: List<List<u64>>) -> R<Option<Adt98>> {
    let m413 = f162(v0, f166())?;
    match m413 {
        Adt110::C0 => {
            Ok(None::<Adt98>)
        }
        Adt110::C1 => {
            Ok(Some(Adt98::C1))
        }
        Adt110::C2 => {
            Ok(Some(Adt98::C1))
        }
        Adt110::C3(v2) => {
            f164(f167(v2), v1, Adt98::C1)
        }
    }
}

pub fn f119(v0: Bytes, p423: u64, p424: u64, p425: List<Adt92>) -> R<Result<(List<Adt92>, u64), Adt90>> {
    let m422 = (p423, (p424, p425));
    {
        let mut m422 = m422;
        loop {
            let step = {
                let (v1, (v2, v3)) = m422;
                let m414 = v2;
                if m414 == 0 {
                    Err::<(u64, (u64, List<Adt92>)), R<Result<(List<Adt92>, u64), Adt90>>>(Ok(Ok::<(List<Adt92>, u64), Adt90>((f168(v3, List::<Adt92>::nil()), v1))))
                } else {
                    let v4 = m414 - 1;
                    let m415 = f41(v0.clone(), v1)?;
                    match m415 {
                        None => {
                            Err::<(u64, (u64, List<Adt92>)), R<Result<(List<Adt92>, u64), Adt90>>>(Ok(Err::<(List<Adt92>, u64), Adt90>(Adt90::C0(v1))))
                        }
                        Some(v5) => {
                            let m417 = f41(v0.clone(), match v5.clone() {
                                Adt94::C0(_, h416) => {
                                    h416
                                }
                            })?;
                            match m417 {
                                None => {
                                    Err::<(u64, (u64, List<Adt92>)), R<Result<(List<Adt92>, u64), Adt90>>>(Ok(Err::<(List<Adt92>, u64), Adt90>(Adt90::C0(match v5 {
                                        Adt94::C0(_, h418) => {
                                            h418
                                        }
                                    }))))
                                }
                                Some(v6) => {
                                    Ok::<(u64, (u64, List<Adt92>)), R<Result<(List<Adt92>, u64), Adt90>>>((match v6.clone() {
                                        Adt94::C0(_, h419) => {
                                            h419
                                        }
                                    }, (v4, List::cons(Adt92::C0(f18(match v5 {
                                        Adt94::C0(h420, _) => {
                                            h420
                                        }
                                    })?, f18(match v6 {
                                        Adt94::C0(h421, _) => {
                                            h421
                                        }
                                    })?), v3))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m422 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f120(v0: Bytes, v1: Adt94, v2: Adt94, v3: (List<Adt92>, u64)) -> R<Result<Adt91, Adt90>> {
    let m427 = f41(v0.clone(), {
        let (_, h426) = v3.clone();
        h426
    })?;
    match m427 {
        None => {
            Ok(Err::<Adt91, Adt90>(Adt90::C0({
                let (_, h428) = v3;
                h428
            })))
        }
        Some(v4) => {
            let m430 = f65(v0.clone(), match v4.clone() {
                Adt94::C0(_, h429) => {
                    h429
                }
            })?;
            match m430 {
                None => {
                    Ok(Err::<Adt91, Adt90>(Adt90::C0(match v4 {
                        Adt94::C0(_, h431) => {
                            h431
                        }
                    })))
                }
                Some(v5) => {
                    let m435 = f169(v0.clone(), {
                        let a433 = match v4.clone() {
                            Adt94::C0(_, h432) => {
                                h432
                            }
                        };
                        let a434 = 4u64;
                        nat_add(a433, a434)?
                    })?;
                    match m435 {
                        None => {
                            Ok(Err::<Adt91, Adt90>(Adt90::C0({
                                let a437 = match v4 {
                                    Adt94::C0(_, h436) => {
                                        h436
                                    }
                                };
                                let a438 = 4u64;
                                nat_add(a437, a438)?
                            })))
                        }
                        Some(v6) => {
                            f170(v0, Adt91::C0(f18(match v1 {
                                Adt94::C0(h439, _) => {
                                    h439
                                }
                            })?, f18(match v2 {
                                Adt94::C0(h440, _) => {
                                    h440
                                }
                            })?, {
                                let (h441, _) = v3;
                                h441
                            }, match v4 {
                                Adt94::C0(h442, _) => {
                                    h442
                                }
                            }, v5, List::<List<u64>>::nil(), match v6.clone() {
                                Adt94::C0(h443, _) => {
                                    h443
                                }
                            }), match v6 {
                                Adt94::C0(_, h444) => {
                                    h444
                                }
                            })
                        }
                    }
                }
            }
        }
    }
}

pub fn f121(v0: Str, p449: List<Str>) -> bool {
    let m448 = p449;
    {
        let mut m448 = m448;
        loop {
            let step = {
                let v1 = m448;
                let m445 = v1;
                match m445.uncons() {
                    None => {
                        Err::<List<Str>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a446 = v0.clone();
                        let a447 = v2;
                        if equal(a446, a447) {
                            Err::<List<Str>, bool>(true)
                        } else {
                            Ok::<List<Str>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m448 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f122(v0: Str, v1: List<u64>) -> Option<Adt93> {
    if f171(v1) {
        Some(Adt93::C0(v0))
    } else {
        None::<Adt93>
    }
}

pub fn f123(v0: Str) -> R<List<u64>> {
    f18({
        let a450 = v0;
        utf8_encode(a450)
    })
}

pub fn f124(v0: Adt37) -> R<Result<List<Adt12>, Adt93>> {
    let m451 = f172(v0.clone());
    match m451 {
        None => {
            f173(v0.clone(), f174(f175(), match v0.clone() {
                Adt37::C0(h452, _, _, _, _, _, _, _) => {
                    h452
                }
            }, Adt6::C5({
                let m455 = match v0.clone() {
                    Adt37::C0(_, _, _, _, h454, _, _, _) => {
                        h454
                    }
                };
                match m455 {
                    Adt5::C0(h453) => {
                        h453
                    }
                }
            }), List::cons(Adt105::C0(Adt6::C5({
                let m458 = match v0 {
                    Adt37::C0(_, _, _, _, h457, _, _, _) => {
                        h457
                    }
                };
                match m458 {
                    Adt5::C0(h456) => {
                        h456
                    }
                }
            }), true, List::<Str>::nil()), List::<Adt105>::nil()), f176())?)
        }
        Some(v1) => {
            Ok(Err::<List<Adt12>, Adt93>(v1))
        }
    }
}

pub fn f125(v0: Adt6, p477: u64, p478: Adt101, p479: List<Adt102>) -> R<Adt99> {
    let m476 = (p477, (p478, p479));
    {
        let mut m476 = m476;
        loop {
            let step = {
                let (v1, (v2, v3)) = m476;
                let m459 = v1;
                if m459 == 0 {
                    Err::<(u64, (Adt101, List<Adt102>)), R<Adt99>>(Ok(Adt99::C4))
                } else {
                    let v4 = m459 - 1;
                    let m460 = v3;
                    match m460.uncons() {
                        None => {
                            let m461 = v2;
                            match m461 {
                                Adt101::C0 => {
                                    Err::<(u64, (Adt101, List<Adt102>)), R<Adt99>>(Ok(Adt99::C0))
                                }
                                Adt101::C1(v5, v6) => {
                                    Err::<(u64, (Adt101, List<Adt102>)), R<Adt99>>(Ok(Adt99::C1(v5, v6)))
                                }
                            }
                        }
                        Some((v7, v8)) => {
                            let m462 = v2.clone();
                            match m462 {
                                Adt101::C0 => {
                                    let m463 = v7.clone();
                                    match m463 {
                                        Adt102::C0(v9, v10, v11) => {
                                            let m464 = f177(v0.clone(), v9, v10, v11.clone())?;
                                            match m464 {
                                                Adt113::C0(v12) => {
                                                    Ok::<(u64, (Adt101, List<Adt102>)), R<Adt99>>((v4, (Adt101::C1(v11, v12), v8)))
                                                }
                                                Adt113::C1(v13) => {
                                                    Err::<(u64, (Adt101, List<Adt102>)), R<Adt99>>(Ok(Adt99::C2(v13)))
                                                }
                                                Adt113::C2(v14) => {
                                                    Err::<(u64, (Adt101, List<Adt102>)), R<Adt99>>(Ok(Adt99::C3(v14)))
                                                }
                                                Adt113::C3(v15) => {
                                                    let v16: List<Adt102> = {
                                                        let a465 = v15;
                                                        let a466 = v8;
                                                        append_list(a465, a466)
                                                    };
                                                    Ok::<(u64, (Adt101, List<Adt102>)), R<Adt99>>((v4, (v2, v16)))
                                                }
                                            }
                                        }
                                        Adt102::C1(_, _, _) => {
                                            Ok::<(u64, (Adt101, List<Adt102>)), R<Adt99>>((v4, (v2, v8)))
                                        }
                                        Adt102::C2(_, _, _, _) => {
                                            let v24: Adt106 = f178(v7, v8)?;
                                            Ok::<(u64, (Adt101, List<Adt102>)), R<Adt99>>((v4, (match v24.clone() {
                                                Adt106::C0(h467, _) => {
                                                    h467
                                                }
                                            }, match v24 {
                                                Adt106::C0(_, h468) => {
                                                    h468
                                                }
                                            })))
                                        }
                                        Adt102::C3(v25) => {
                                            Ok::<(u64, (Adt101, List<Adt102>)), R<Adt99>>((v4, (Adt101::C1(v25, Str::lit("not")), v8)))
                                        }
                                        Adt102::C4(_, _, _, _) => {
                                            let v39: Adt106 = f178(v7, v8)?;
                                            Ok::<(u64, (Adt101, List<Adt102>)), R<Adt99>>((v4, (match v39.clone() {
                                                Adt106::C0(h472, _) => {
                                                    h472
                                                }
                                            }, match v39 {
                                                Adt106::C0(_, h473) => {
                                                    h473
                                                }
                                            })))
                                        }
                                        Adt102::C5(v26, v27, v28, v29) => {
                                            let m469 = f179(v26, v27, v28, v29.clone());
                                            match m469 {
                                                Adt113::C0(v30) => {
                                                    Ok::<(u64, (Adt101, List<Adt102>)), R<Adt99>>((v4, (Adt101::C1(v29, v30), v8)))
                                                }
                                                Adt113::C1(v31) => {
                                                    Err::<(u64, (Adt101, List<Adt102>)), R<Adt99>>(Ok(Adt99::C2(v31)))
                                                }
                                                Adt113::C2(v32) => {
                                                    Err::<(u64, (Adt101, List<Adt102>)), R<Adt99>>(Ok(Adt99::C3(v32)))
                                                }
                                                Adt113::C3(v33) => {
                                                    let v34: List<Adt102> = {
                                                        let a470 = v33;
                                                        let a471 = v8;
                                                        append_list(a470, a471)
                                                    };
                                                    Ok::<(u64, (Adt101, List<Adt102>)), R<Adt99>>((v4, (v2, v34)))
                                                }
                                            }
                                        }
                                    }
                                }
                                Adt101::C1(v40, v41) => {
                                    let v42: Adt106 = f180(v7, v8, v40, v41);
                                    Ok::<(u64, (Adt101, List<Adt102>)), R<Adt99>>((v4, (match v42.clone() {
                                        Adt106::C0(h474, _) => {
                                            h474
                                        }
                                    }, match v42 {
                                        Adt106::C0(_, h475) => {
                                            h475
                                        }
                                    })))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m476 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f126() -> u64 {
    67108864u64
}

pub fn f127(v0: List<Adt100>) -> R<Str> {
    let a480 = List::cons(Str::lit(""), f181(v0, List::<Str>::nil())?);
    let a481 = Str::lit("/");
    Ok(join(a480, a481))
}

pub fn f128(v0: Adt37, v1: Adt51) -> R<Result<Adt34, Adt86>> {
    if f182({
        let m484 = match v1.clone() {
            Adt51::C0(h483, _) => {
                h483
            }
        };
        match m484 {
            Adt39::C0(h482, _, _) => {
                h482
            }
        }
    })? {
        f183(v0, v1)
    } else {
        Ok(Err::<Adt34, Adt86>(Adt86::C6(Str::lit("a content payload is not canonical Base64"))))
    }
}

pub fn f129(v0: Adt23, v1: Str) -> Option<Adt6> {
    let m486 = match v0 {
        Adt23::C0(_, _, h485, _, _) => {
            h485
        }
    };
    match m486 {
        None => {
            None::<Adt6>
        }
        Some(v2) => {
            let m487 = f184(v1, v2);
            match m487 {
                Adt114::C0 => {
                    None::<Adt6>
                }
                Adt114::C1(v3) => {
                    Some(v3)
                }
            }
        }
    }
}

pub fn f130(v0: Adt6) -> Option<Str> {
    let m488 = v0;
    match m488 {
        Adt6::C0 => {
            None::<Str>
        }
        Adt6::C1(_) => {
            None::<Str>
        }
        Adt6::C2(_) => {
            None::<Str>
        }
        Adt6::C3(v3) => {
            Some(v3)
        }
        Adt6::C4(_) => {
            None::<Str>
        }
        Adt6::C5(_) => {
            None::<Str>
        }
    }
}

pub fn f131(v0: Str) -> u64 {
    let a489 = v0;
    let a490 = Str::lit("drafts");
    if equal(a489, a490) {
        1u64
    } else {
        0u64
    }
}

pub fn f132(v0: Adt23) -> Option<List<Str>> {
    let m491 = f129(v0, Str::lit("files"));
    match m491 {
        None => {
            Some(List::<Str>::nil())
        }
        Some(v1) => {
            let m492 = f185(v1);
            match m492 {
                None => {
                    None::<List<Str>>
                }
                Some(v2) => {
                    let m493 = f186(v2, List::<Str>::nil());
                    match m493 {
                        None => {
                            None::<List<Str>>
                        }
                        Some(v3) => {
                            Some(f187(v3, List::<Str>::nil()))
                        }
                    }
                }
            }
        }
    }
}

pub fn f133(p496: List<Str>) -> R<bool> {
    let m495 = p496;
    {
        let mut m495 = m495;
        loop {
            let step = {
                let v0 = m495;
                let m494 = v0;
                match m494.uncons() {
                    None => {
                        Err::<List<Str>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        if f188(f87(v1)?) {
                            Err::<List<Str>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<Str>, R<bool>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m495 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f134(v0: bool, v1: List<Str>) -> bool {
    if v0 {
        let m497 = v1;
        match m497.uncons() {
            None => {
                false
            }
            Some((_, _)) => {
                true
            }
        }
    } else {
        false
    }
}

pub fn f135(v0: bool, v1: List<Str>) -> R<List<u64>> {
    let m498 = v1.clone();
    match m498.uncons() {
        None => {
            let a499 = f17(if v0 {
                1u64
            } else {
                0u64
            });
            let a500 = f93(Str::lit(""))?;
            Ok(append_list(a499, a500))
        }
        Some((_, _)) => {
            let a503 = f17(2u64);
            let a504 = f93({
                let a501 = v1;
                let a502 = Str::lit("\u{a}");
                join(a501, a502)
            })?;
            Ok(append_list(a503, a504))
        }
    }
}

pub fn f136(v0: Adt6) -> Option<bool> {
    let m505 = v0;
    match m505 {
        Adt6::C0 => {
            None::<bool>
        }
        Adt6::C1(v1) => {
            Some(v1)
        }
        Adt6::C2(_) => {
            None::<bool>
        }
        Adt6::C3(_) => {
            None::<bool>
        }
        Adt6::C4(_) => {
            None::<bool>
        }
        Adt6::C5(_) => {
            None::<bool>
        }
    }
}

pub fn f137(p510: Bytes, p511: List<Bytes>) -> Bytes {
    let m509 = (p510, p511);
    {
        let mut m509 = m509;
        loop {
            let step = {
                let (v0, v1) = m509;
                let m506 = v1;
                match m506.uncons() {
                    None => {
                        Err::<(Bytes, List<Bytes>), Bytes>(v0)
                    }
                    Some((v2, v3)) => {
                        Ok::<(Bytes, List<Bytes>), Bytes>(({
                            let a507 = v2;
                            let a508 = v0;
                            append_bytes(a507, a508)
                        }, v3))
                    }
                }
            };
            match step {
                Ok(next) => { m509 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f138(p514: List<Bytes>, p515: List<u64>) -> List<Bytes> {
    let m513 = (p514, p515);
    {
        let mut m513 = m513;
        loop {
            let step = {
                let (v0, v1) = m513;
                let m512 = v1;
                match m512.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<u64>), List<Bytes>>(v0)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<u64>), List<Bytes>>((f189(f190(v2), v0), v3))
                    }
                }
            };
            match step {
                Ok(next) => { m513 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f139(p519: List<Str>, p520: u64) -> Str {
    let m518 = (p519, p520);
    {
        let mut m518 = m518;
        loop {
            let step = {
                let (v0, v1) = m518;
                let m516 = v0;
                match m516.uncons() {
                    None => {
                        Err::<(List<Str>, u64), Str>(Str::lit("unknown"))
                    }
                    Some((v2, v3)) => {
                        let m517 = v1;
                        if m517 == 0 {
                            Err::<(List<Str>, u64), Str>(v2)
                        } else {
                            let v4 = m517 - 1;
                            Ok::<(List<Str>, u64), Str>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m518 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f140(v0: Adt97) -> List<Str> {
    let m521 = v0;
    match m521 {
        Adt97::C0 => {
            List::cons(Str::lit("area-unknown"), List::cons(Str::lit("key-refused"), List::cons(Str::lit("unreadable"), List::<Str>::nil())))
        }
        Adt97::C1 => {
            List::cons(Str::lit("area-unknown"), List::cons(Str::lit("key-refused"), List::cons(Str::lit("unwritable"), List::<Str>::nil())))
        }
        Adt97::C2 => {
            List::cons(Str::lit("project-refused"), List::cons(Str::lit("engine-unavailable"), List::<Str>::nil()))
        }
        Adt97::C3 => {
            List::cons(Str::lit("project-refused"), List::cons(Str::lit("engine-unavailable"), List::<Str>::nil()))
        }
        Adt97::C4 => {
            List::cons(Str::lit("project-refused"), List::cons(Str::lit("engine-unavailable"), List::<Str>::nil()))
        }
        Adt97::C5 => {
            List::cons(Str::lit("project-refused"), List::cons(Str::lit("engine-unavailable"), List::<Str>::nil()))
        }
        Adt97::C6 => {
            List::cons(Str::lit("project-refused"), List::cons(Str::lit("verifier-unavailable"), List::<Str>::nil()))
        }
        Adt97::C7 => {
            List::cons(Str::lit("unknown-task"), List::<Str>::nil())
        }
        Adt97::C8 => {
            List::cons(Str::lit("profile-unknown"), List::cons(Str::lit("key-refused"), List::<Str>::nil()))
        }
        Adt97::C9 => {
            List::cons(Str::lit("value-refused"), List::<Str>::nil())
        }
    }
}

pub fn f141(v0: Str, v1: Adt6, v2: bool) -> Adt51 {
    Adt51::C0(Adt39::C0(List::cons(f191(v0), List::<Adt40>::nil()), Some(v1), if v2 {
        Some(true)
    } else {
        None::<bool>
    }), List::<Adt52>::nil())
}

pub fn f142(v0: Adt97) -> Str {
    let m522 = v0;
    match m522 {
        Adt97::C0 => {
            Str::lit("store.get")
        }
        Adt97::C1 => {
            Str::lit("store.put")
        }
        Adt97::C2 => {
            Str::lit("engine.check")
        }
        Adt97::C3 => {
            Str::lit("engine.snapshot")
        }
        Adt97::C4 => {
            Str::lit("engine.format")
        }
        Adt97::C5 => {
            Str::lit("engine.build")
        }
        Adt97::C6 => {
            Str::lit("verify.start")
        }
        Adt97::C7 => {
            Str::lit("verify.control")
        }
        Adt97::C8 => {
            Str::lit("lexicon.read")
        }
        Adt97::C9 => {
            Str::lit("attestation.lookup")
        }
    }
}

pub fn f143() -> Adt51 {
    f192(Str::lit("the host answered with a malformed outcome"))
}

pub fn f144(v0: Bytes) -> Str {
    let m524 = {
        let a523 = v0;
        utf8_decode(a523)
    };
    match m524 {
        None => {
            Str::lit("the content is not UTF-8")
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f145(p527: u64, p528: i64) -> R<i64> {
    let m526 = (p527, p528);
    {
        let mut m526 = m526;
        loop {
            let step = {
                let (v0, v1) = m526;
                let m525 = v0;
                if m525 == 0 {
                    Err::<(u64, i64), R<i64>>(Ok(v1))
                } else {
                    let v2 = m525 - 1;
                    Ok::<(u64, i64), R<i64>>((v2, f193(v1, 1i64)?))
                }
            };
            match step {
                Ok(next) => { m526 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f146(v0: Bytes) -> R<Result<Adt6, Adt107>> {
    f194(f18(v0)?)
}

pub fn f147(v0: Adt6) -> bool {
    let m529 = f195(v0);
    match m529 {
        None => {
            false
        }
        Some(v1) => {
            let m530 = f184(Str::lit("spec"), v1);
            match m530 {
                Adt114::C0 => {
                    false
                }
                Adt114::C1(v2) => {
                    let m531 = f130(v2);
                    match m531 {
                        None => {
                            false
                        }
                        Some(v3) => {
                            let a532 = v3;
                            let a533 = Str::lit("lexlean/command-result/1");
                            !equal(a532, a533)
                        }
                    }
                }
            }
        }
    }
}

pub fn f148(v0: Adt6, v1: u64) -> R<bool> {
    let m534 = f195(v0);
    match m534 {
        None => {
            Ok(false)
        }
        Some(v2) => {
            let m535 = f184(Str::lit("exit_code"), v2);
            match m535 {
                Adt114::C0 => {
                    Ok(false)
                }
                Adt114::C1(v3) => {
                    let m536 = f196(v3, f52(v1)?)?;
                    match m536 {
                        Adt115::C0 => {
                            Ok(true)
                        }
                        Adt115::C1 => {
                            Ok(false)
                        }
                        Adt115::C2 => {
                            Ok(false)
                        }
                    }
                }
            }
        }
    }
}

pub fn f149(v0: u64) -> Str {
    f139(List::cons(Str::lit("queued"), List::cons(Str::lit("running"), List::cons(Str::lit("verified"), List::cons(Str::lit("failed"), List::cons(Str::lit("cancelled"), List::<Str>::nil()))))), v0)
}

pub fn f150(p539: List<Adt15>, p540: List<Adt15>) -> List<Adt15> {
    let m538 = (p539, p540);
    {
        let mut m538 = m538;
        loop {
            let step = {
                let (v0, v1) = m538;
                let m537 = v0;
                match m537.uncons() {
                    None => {
                        Err::<(List<Adt15>, List<Adt15>), List<Adt15>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt15>, List<Adt15>), List<Adt15>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m538 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f151(v0: bool, v1: Bytes, v2: List<Adt15>, v3: u64) -> Result<Adt89, Adt90> {
    if v0 {
        Ok::<Adt89, Adt90>(Adt89::C0(v1, v2))
    } else {
        Err::<Adt89, Adt90>(Adt90::C1(v3))
    }
}

pub fn f152(v0: Adt92) -> List<u64> {
    let a551 = f17({
        let a542 = match v0.clone() {
            Adt92::C0(h541, _) => {
                h541
            }
        };
        length_list(a542)
    });
    let a552 = {
        let a549 = match v0.clone() {
            Adt92::C0(h543, _) => {
                h543
            }
        };
        let a550 = {
            let a547 = f17({
                let a545 = match v0.clone() {
                    Adt92::C0(_, h544) => {
                        h544
                    }
                };
                length_list(a545)
            });
            let a548 = match v0 {
                Adt92::C0(_, h546) => {
                    h546
                }
            };
            append_list(a547, a548)
        };
        append_list(a549, a550)
    };
    append_list(a551, a552)
}

pub fn f153(v0: List<u64>) -> List<u64> {
    let a554 = f17({
        let a553 = v0.clone();
        length_list(a553)
    });
    let a555 = v0;
    append_list(a554, a555)
}

pub fn f154(v0: u64, v1: Adt6) -> R<Adt95> {
    let m556 = f197(v1)?;
    match m556 {
        Ok(v3) => {
            Ok(Adt95::C0(v0, f69(), List::<List<u64>>::nil(), v3))
        }
        Err(_) => {
            Ok(f43())
        }
    }
}

pub fn f155(v0: Adt98) -> u64 {
    let m557 = v0;
    match m557 {
        Adt98::C0 => {
            f198()
        }
        Adt98::C1 => {
            f198()
        }
    }
}

pub fn f156(v0: Option<Adt104>, v1: Adt82) -> Adt6 {
    Adt6::C5(f199(v0, v1))
}

pub fn f157(v0: Adt98) -> Adt82 {
    Adt82::C0(f200(), f201(v0), None::<Adt6>)
}

pub fn f158(v0: List<u64>) -> bool {
    f202(v0, f203())
}

pub fn f159(v0: Adt0, v1: Fn2, v2: u64, v3: List<Adt92>, v4: Result<Adt6, Adt107>) -> R<Adt88> {
    let m558 = v4;
    match m558 {
        Ok(v6) => {
            f206(v0, v1, v2, v3, f207(v6)?)
        }
        Err(_) => {
            Ok(Adt88::C0(f42(f154(f204(), f156(None::<Adt104>, f205(Adt108::C0)))?)))
        }
    }
}

pub fn f160(v0: Adt0, v1: Bytes) -> Adt0 {
    Adt0::C0(match v0.clone() {
        Adt0::C0(h559, _, _, _, _, _, _) => {
            h559
        }
    }, match v0.clone() {
        Adt0::C0(_, h560, _, _, _, _, _) => {
            h560
        }
    }, match v0.clone() {
        Adt0::C0(_, _, h561, _, _, _, _) => {
            h561
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, h562, _, _, _) => {
            h562
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, h563, _, _) => {
            h563
        }
    }, match v0 {
        Adt0::C0(_, _, _, _, _, h564, _) => {
            h564
        }
    }, v1)
}

pub fn f161() -> Adt95 {
    Adt95::C0(f208(), List::cons(Adt92::C0(f209(), f210()), List::<Adt92>::nil()), List::<List<u64>>::nil(), List::<u64>::nil())
}

pub fn f162(v0: List<Adt92>, v1: List<u64>) -> R<Adt110> {
    let m565 = f211(v0, v1, Adt111::C0)?;
    match m565 {
        Adt111::C0 => {
            Ok(Adt110::C0)
        }
        Adt111::C1(v2) => {
            let m566 = f212(v2);
            match m566 {
                None => {
                    Ok(Adt110::C1)
                }
                Some(v3) => {
                    Ok(Adt110::C3(v3))
                }
            }
        }
        Adt111::C2 => {
            Ok(Adt110::C2)
        }
    }
}

pub fn f163() -> List<u64> {
    List::cons(79u64, List::cons(114u64, List::cons(105u64, List::cons(103u64, List::cons(105u64, List::cons(110u64, List::<u64>::nil()))))))
}

pub fn f164(v0: Option<List<u64>>, v1: List<List<u64>>, v2: Adt98) -> R<Option<Adt98>> {
    let m567 = v0;
    match m567 {
        None => {
            Ok(Some(v2))
        }
        Some(v3) => {
            if f213(v3, v1)? {
                Ok(None::<Adt98>)
            } else {
                Ok(Some(v2))
            }
        }
    }
}

pub fn f165(v0: List<u64>) -> Option<List<u64>> {
    let m568 = f214(v0);
    match m568 {
        None => {
            None::<List<u64>>
        }
        Some(v1) => {
            f167(v1)
        }
    }
}

pub fn f166() -> List<u64> {
    List::cons(72u64, List::cons(111u64, List::cons(115u64, List::cons(116u64, List::<u64>::nil()))))
}

pub fn f167(v0: List<u64>) -> Option<List<u64>> {
    let m569 = v0.clone();
    match m569.uncons() {
        None => {
            None::<List<u64>>
        }
        Some((v1, v2)) => {
            let a570 = v1;
            let a571 = 91u64;
            if nat_eq(a570, a571) {
                f215(f216(v2, List::cons(v1, List::<u64>::nil())))
            } else {
                f215(f217(v0, List::<u64>::nil()))
            }
        }
    }
}

pub fn f168(p574: List<Adt92>, p575: List<Adt92>) -> List<Adt92> {
    let m573 = (p574, p575);
    {
        let mut m573 = m573;
        loop {
            let step = {
                let (v0, v1) = m573;
                let m572 = v0;
                match m572.uncons() {
                    None => {
                        Err::<(List<Adt92>, List<Adt92>), List<Adt92>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt92>, List<Adt92>), List<Adt92>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m573 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f169(v0: Bytes, v1: u64) -> R<Option<Adt94>> {
    let a577 = v1;
    let a578 = {
        let a576 = v0.clone();
        length_bytes(a576)
    };
    if nat_eq(a577, a578) {
        Ok(Some(Adt94::C0(Bytes::lit(&[]), v1)))
    } else {
        f41(v0, v1)
    }
}

pub fn f170(v0: Bytes, v1: Adt91, v2: u64) -> R<Result<Adt91, Adt90>> {
    let a580 = v2;
    let a581 = {
        let a579 = v0.clone();
        length_bytes(a579)
    };
    if nat_eq(a580, a581) {
        Ok(Ok::<Adt91, Adt90>(v1))
    } else {
        let m582 = f65(v0.clone(), v2)?;
        match m582 {
            None => {
                Ok(Err::<Adt91, Adt90>(Adt90::C0(v2)))
            }
            Some(v3) => {
                let m585 = f218(v0.clone(), {
                    let a583 = v2;
                    let a584 = 4u64;
                    nat_add(a583, a584)?
                }, v3, List::<List<u64>>::nil())?;
                match m585 {
                    Ok(v5) => {
                        let a588 = {
                            let (_, h586) = v5.clone();
                            h586
                        };
                        let a589 = {
                            let a587 = v0;
                            length_bytes(a587)
                        };
                        if nat_eq(a588, a589) {
                            Ok(Ok::<Adt91, Adt90>(f219(v1, {
                                let (h590, _) = v5;
                                h590
                            })))
                        } else {
                            Ok(Err::<Adt91, Adt90>(Adt90::C1({
                                let (_, h591) = v5;
                                h591
                            })))
                        }
                    }
                    Err(v4) => {
                        Ok(Err::<Adt91, Adt90>(v4))
                    }
                }
            }
        }
    }
}

pub fn f171(v0: List<u64>) -> bool {
    let m592 = v0.clone();
    match m592.uncons() {
        None => {
            true
        }
        Some((_, _)) => {
            let a596 = if f220(v0.clone(), false) {
                let a594 = {
                    let a593 = v0;
                    length_list(a593)
                };
                let a595 = f221();
                nat_le(a594, a595)
            } else {
                false
            };
            bool_not(a596)
        }
    }
}

pub fn f172(v0: Adt37) -> Option<Adt93> {
    let m601 = f222(match v0.clone() {
        Adt37::C0(h597, _, _, _, _, _, _, _) => {
            h597
        }
    }, {
        let m600 = match v0.clone() {
            Adt37::C0(_, _, _, _, h599, _, _, _) => {
                h599
            }
        };
        match m600 {
            Adt5::C0(h598) => {
                h598
            }
        }
    });
    match m601 {
        None => {
            f223(match v0.clone() {
                Adt37::C0(h602, _, _, _, _, _, _, _) => {
                    h602
                }
            }, {
                let m605 = match v0 {
                    Adt37::C0(_, _, _, _, h604, _, _, _) => {
                        h604
                    }
                };
                match m605 {
                    Adt5::C0(h603) => {
                        h603
                    }
                }
            })
        }
        Some(v1) => {
            Some(v1)
        }
    }
}

pub fn f173(v0: Adt37, v1: Result<Adt112, Adt93>) -> R<Result<List<Adt12>, Adt93>> {
    let m606 = v1;
    match m606 {
        Ok(v3) => {
            let m607 = f224(v0)?;
            match m607 {
                None => {
                    Ok(Ok::<List<Adt12>, Adt93>(f75(match v3 {
                        Adt112::C0(h608, _) => {
                            h608
                        }
                    }, List::<Adt12>::nil())))
                }
                Some(v4) => {
                    Ok(Err::<List<Adt12>, Adt93>(v4))
                }
            }
        }
        Err(v2) => {
            Ok(Err::<List<Adt12>, Adt93>(v2))
        }
    }
}

pub fn f174(p615: u64, v1: Str, v2: Adt6, p616: List<Adt105>, p617: Adt112) -> R<Result<Adt112, Adt93>> {
    let m614 = (p615, (p616, p617));
    {
        let mut m614 = m614;
        loop {
            let step = {
                let (v0, (v3, v4)) = m614;
                let m609 = v0;
                if m609 == 0 {
                    Err::<(u64, (List<Adt105>, Adt112)), R<Result<Adt112, Adt93>>>(Ok(Err::<Adt112, Adt93>(Adt93::C5(v1.clone(), Str::lit("budget")))))
                } else {
                    let v5 = m609 - 1;
                    let m610 = v3;
                    match m610.uncons() {
                        None => {
                            Err::<(u64, (List<Adt105>, Adt112)), R<Result<Adt112, Adt93>>>(Ok(Ok::<Adt112, Adt93>(v4)))
                        }
                        Some((v6, v7)) => {
                            let m611 = f225(v1.clone(), v2.clone(), v6, v4, v7)?;
                            match m611 {
                                Ok(v9) => {
                                    Ok::<(u64, (List<Adt105>, Adt112)), R<Result<Adt112, Adt93>>>((v5, ({
                                        let (h612, _) = v9;
                                        h612
                                    }, {
                                        let (_, h613) = v9;
                                        h613
                                    })))
                                }
                                Err(v8) => {
                                    Err::<(u64, (List<Adt105>, Adt112)), R<Result<Adt112, Adt93>>>(Ok(Err::<Adt112, Adt93>(v8)))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m614 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f175() -> u64 {
    67108864u64
}

pub fn f176() -> Adt112 {
    Adt112::C0(List::<Adt12>::nil(), List::<List<u64>>::nil())
}

pub fn f177(v0: Adt6, v1: Adt6, v2: Adt6, v3: List<Adt100>) -> R<Adt113> {
    let m618 = f136(v1.clone());
    match m618 {
        None => {
            let m619 = f195(v1);
            match m619 {
                None => {
                    Ok(Adt113::C1(Str::lit("schema")))
                }
                Some(v5) => {
                    f226(v0, match v5 {
                        Adt5::C0(h620) => {
                            h620
                        }
                    }, v2, v3)
                }
            }
        }
        Some(v4) => {
            if v4 {
                Ok(Adt113::C3(List::<Adt102>::nil()))
            } else {
                Ok(Adt113::C0(Str::lit("false")))
            }
        }
    }
}

pub fn f178(v0: Adt102, v1: List<Adt102>) -> R<Adt106> {
    let m621 = v0;
    match m621 {
        Adt102::C0(_, _, _) => {
            Ok(Adt106::C0(Adt101::C0, v1))
        }
        Adt102::C1(_, _, _) => {
            Ok(Adt106::C0(Adt101::C0, v1))
        }
        Adt102::C2(v8, v9, v10, v11) => {
            Ok(f227(v8, v9, v10, {
                let a622 = v11;
                let a623 = 1u64;
                nat_add(a622, a623)?
            }, v1))
        }
        Adt102::C3(v12) => {
            Ok(Adt106::C0(Adt101::C1(v12, Str::lit("not")), v1))
        }
        Adt102::C4(v13, _, v15, v16) => {
            Ok(f228(v13, v15, v16, v1))
        }
        Adt102::C5(_, _, _, _) => {
            Ok(Adt106::C0(Adt101::C0, v1))
        }
    }
}

pub fn f179(v0: Adt103, v1: Adt6, v2: Adt6, v3: List<Adt100>) -> Adt113 {
    let m624 = f195(v1);
    match m624 {
        None => {
            Adt113::C1(Str::lit("schema"))
        }
        Some(v4) => {
            let m625 = v0;
            match m625 {
                Adt103::C0 => {
                    f229(match v4 {
                        Adt5::C0(h626) => {
                            h626
                        }
                    }, v2, v3)
                }
                Adt103::C1 => {
                    f230(match v4 {
                        Adt5::C0(h627) => {
                            h627
                        }
                    }, v2, v3)
                }
                Adt103::C2 => {
                    f231(match v4 {
                        Adt5::C0(h628) => {
                            h628
                        }
                    }, v2, v3)
                }
                Adt103::C3 => {
                    f232(match v4 {
                        Adt5::C0(h629) => {
                            h629
                        }
                    }, v2, v3)
                }
            }
        }
    }
}

pub fn f180(v0: Adt102, v1: List<Adt102>, v2: List<Adt100>, v3: Str) -> Adt106 {
    let m630 = v0;
    match m630 {
        Adt102::C0(_, _, _) => {
            Adt106::C0(Adt101::C1(v2, v3), v1)
        }
        Adt102::C1(v7, v8, v9) => {
            f233(v7, v8, v9, v1)
        }
        Adt102::C2(v10, v11, v12, v13) => {
            f227(v10, v11, v12, v13, v1)
        }
        Adt102::C3(_) => {
            Adt106::C0(Adt101::C0, v1)
        }
        Adt102::C4(_, v16, v17, v18) => {
            f228(v16, v17, v18, v1)
        }
        Adt102::C5(_, _, _, _) => {
            Adt106::C0(Adt101::C1(v2, v3), v1)
        }
    }
}

pub fn f181(p633: List<Adt100>, p634: List<Str>) -> R<List<Str>> {
    let m632 = (p633, p634);
    {
        let mut m632 = m632;
        loop {
            let step = {
                let (v0, v1) = m632;
                let m631 = v0;
                match m631.uncons() {
                    None => {
                        Err::<(List<Adt100>, List<Str>), R<List<Str>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt100>, List<Str>), R<List<Str>>>((v3, List::cons(f234(v2)?, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m632 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f182(p643: List<Adt40>) -> R<bool> {
    let m642 = p643;
    {
        let mut m642 = m642;
        loop {
            let step = {
                let v0 = m642;
                let m635 = v0;
                match m635.uncons() {
                    None => {
                        Err::<List<Adt40>, R<bool>>(Ok(true))
                    }
                    Some((v1, v2)) => {
                        let m636 = v1;
                        match m636 {
                            Adt40::C0(_) => {
                                Ok::<List<Adt40>, R<bool>>(v2)
                            }
                            Adt40::C1(v4) => {
                                if f235(match v4 {
                                    Adt44::C0(h637, _, _, _) => {
                                        h637
                                    }
                                })? {
                                    Ok::<List<Adt40>, R<bool>>(v2)
                                } else {
                                    Err::<List<Adt40>, R<bool>>(Ok(false))
                                }
                            }
                            Adt40::C2(v5) => {
                                if f235(match v5 {
                                    Adt45::C0(h638, _, _, _) => {
                                        h638
                                    }
                                })? {
                                    Ok::<List<Adt40>, R<bool>>(v2)
                                } else {
                                    Err::<List<Adt40>, R<bool>>(Ok(false))
                                }
                            }
                            Adt40::C3(_) => {
                                Ok::<List<Adt40>, R<bool>>(v2)
                            }
                            Adt40::C4(v7) => {
                                let m640 = match v7 {
                                    Adt47::C0(h639, _, _) => {
                                        h639
                                    }
                                };
                                match m640 {
                                    Adt48::C0(_) => {
                                        Ok::<List<Adt40>, R<bool>>(v2)
                                    }
                                    Adt48::C1(v9) => {
                                        if f235(match v9 {
                                            Adt50::C0(_, _, h641, _) => {
                                                h641
                                            }
                                        })? {
                                            Ok::<List<Adt40>, R<bool>>(v2)
                                        } else {
                                            Err::<List<Adt40>, R<bool>>(Ok(false))
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m642 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f183(v0: Adt37, v1: Adt51) -> R<Result<Adt34, Adt86>> {
    if f236(match v1.clone() {
        Adt51::C0(h644, _) => {
            h644
        }
    }) {
        f237(v1)
    } else {
        let m646 = match v0 {
            Adt37::C0(_, _, _, _, _, h645, _, _) => {
                h645
            }
        };
        match m646 {
            None => {
                f237(v1)
            }
            Some(v2) => {
                f238(v2, v1)
            }
        }
    }
}

pub fn f184(v0: Str, v1: Adt5) -> Adt114 {
    f239(v0, match v1 {
        Adt5::C0(h647) => {
            h647
        }
    })
}

pub fn f185(v0: Adt6) -> Option<List<Adt6>> {
    let m648 = v0;
    match m648 {
        Adt6::C0 => {
            None::<List<Adt6>>
        }
        Adt6::C1(_) => {
            None::<List<Adt6>>
        }
        Adt6::C2(_) => {
            None::<List<Adt6>>
        }
        Adt6::C3(_) => {
            None::<List<Adt6>>
        }
        Adt6::C4(v4) => {
            Some(v4)
        }
        Adt6::C5(_) => {
            None::<List<Adt6>>
        }
    }
}

pub fn f186(p652: List<Adt6>, p653: List<Str>) -> Option<List<Str>> {
    let m651 = (p652, p653);
    {
        let mut m651 = m651;
        loop {
            let step = {
                let (v0, v1) = m651;
                let m649 = v0;
                match m649.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<Str>), Option<List<Str>>>(Some(v1))
                    }
                    Some((v2, v3)) => {
                        let m650 = f130(v2);
                        match m650 {
                            None => {
                                Err::<(List<Adt6>, List<Str>), Option<List<Str>>>(None::<List<Str>>)
                            }
                            Some(v4) => {
                                Ok::<(List<Adt6>, List<Str>), Option<List<Str>>>((v3, List::cons(v4, v1)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m651 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f187(p656: List<Str>, p657: List<Str>) -> List<Str> {
    let m655 = (p656, p657);
    {
        let mut m655 = m655;
        loop {
            let step = {
                let (v0, v1) = m655;
                let m654 = v0;
                match m654.uncons() {
                    None => {
                        Err::<(List<Str>, List<Str>), List<Str>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Str>, List<Str>), List<Str>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m655 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f188(p662: List<u64>) -> bool {
    let m661 = p662;
    {
        let mut m661 = m661;
        loop {
            let step = {
                let v0 = m661;
                let m658 = v0;
                match m658.uncons() {
                    None => {
                        Err::<List<u64>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        let a659 = v1;
                        let a660 = 10u64;
                        if nat_eq(a659, a660) {
                            Err::<List<u64>, bool>(true)
                        } else {
                            Ok::<List<u64>, bool>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m661 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f189(p671: Bytes, p672: List<Bytes>) -> List<Bytes> {
    let m670 = (p671, p672);
    {
        let mut m670 = m670;
        loop {
            let step = {
                let (v0, v1) = m670;
                let m663 = v1;
                match m663.uncons() {
                    None => {
                        Err::<(Bytes, List<Bytes>), List<Bytes>>(List::cons(v0, List::<Bytes>::nil()))
                    }
                    Some((v2, v3)) => {
                        let a666 = {
                            let a664 = v2.clone();
                            length_bytes(a664)
                        };
                        let a667 = {
                            let a665 = v0.clone();
                            length_bytes(a665)
                        };
                        if nat_eq(a666, a667) {
                            Ok::<(Bytes, List<Bytes>), List<Bytes>>(({
                                let a668 = v2;
                                let a669 = v0;
                                append_bytes(a668, a669)
                            }, v3))
                        } else {
                            Err::<(Bytes, List<Bytes>), List<Bytes>>(List::cons(v0, List::cons(v2, v3)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m670 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f190(v0: u64) -> Bytes {
    let m676 = {
        let a673 = f240();
        let a674 = v0;
        let a675 = 1u64;
        slice_bytes(a673, a674, a675)
    };
    match m676 {
        None => {
            f95()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f191(v0: Str) -> Adt40 {
    Adt40::C0(Adt41::C0(v0, None::<Adt42>, None::<Adt5>))
}

pub fn f192(v0: Str) -> Adt51 {
    Adt51::C0(Adt39::C0(List::cons(f191(v0), List::<Adt40>::nil()), None::<Adt6>, None::<bool>), List::<Adt52>::nil())
}

pub fn f193(v0: i64, v1: i64) -> R<i64> {
    let a678 = v0;
    let a679 = {
        let a677 = v1;
        int_neg(a677)?
    };
    int_sub(a678, a679)
}

pub fn f194(v0: List<u64>) -> R<Result<Adt6, Adt107>> {
    let m680 = f241(v0.clone())?;
    match m680 {
        Adt117::C0 => {
            f242(f243(), 0u64, v0)
        }
        Adt117::C1(v1) => {
            Ok(Err::<Adt6, Adt107>(Adt107::C0(v1)))
        }
        Adt117::C2(v2) => {
            Ok(Err::<Adt6, Adt107>(Adt107::C1(v2)))
        }
    }
}

pub fn f195(v0: Adt6) -> Option<Adt5> {
    let m681 = v0;
    match m681 {
        Adt6::C0 => {
            None::<Adt5>
        }
        Adt6::C1(_) => {
            None::<Adt5>
        }
        Adt6::C2(_) => {
            None::<Adt5>
        }
        Adt6::C3(_) => {
            None::<Adt5>
        }
        Adt6::C4(_) => {
            None::<Adt5>
        }
        Adt6::C5(v5) => {
            Some(Adt5::C0(v5))
        }
    }
}

pub fn f196(v0: Adt6, v1: Adt6) -> R<Adt115> {
    f244(f245(), List::cons((v0, v1), List::<(Adt6, Adt6)>::nil()))
}

pub fn f197(v0: Adt6) -> R<Result<List<u64>, Adt107>> {
    f246(f247(), v0)
}

pub fn f198() -> u64 {
    403u64
}

pub fn f199(v0: Option<Adt104>, v1: Adt82) -> List<(Str, Adt6)> {
    List::cons((Str::lit("jsonrpc"), Adt6::C3(f248())), {
        let a682 = f249(v0);
        let a683 = List::cons((Str::lit("error"), f250(v1)), List::<(Str, Adt6)>::nil());
        append_list(a682, a683)
    })
}

pub fn f200() -> i64 {
    -32000i64
}

pub fn f201(v0: Adt98) -> Str {
    let m684 = v0;
    match m684 {
        Adt98::C0 => {
            Str::lit("Forbidden: invalid Origin")
        }
        Adt98::C1 => {
            Str::lit("Forbidden: invalid Host")
        }
    }
}

pub fn f202(p691: List<u64>, p692: List<u64>) -> bool {
    let m690 = (p691, p692);
    {
        let mut m690 = m690;
        loop {
            let step = {
                let (v0, v1) = m690;
                let m685 = v0;
                match m685.uncons() {
                    None => {
                        let m686 = v1;
                        match m686.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m687 = v1;
                        match m687.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                            Some((v6, v7)) => {
                                let a688 = v4;
                                let a689 = v6;
                                if nat_eq(a688, a689) {
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
                Ok(next) => { m690 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f203() -> List<u64> {
    List::cons(80u64, List::cons(79u64, List::cons(83u64, List::cons(84u64, List::<u64>::nil()))))
}

pub fn f204() -> u64 {
    400u64
}

pub fn f205(v0: Adt108) -> Adt82 {
    Adt82::C0(f251(v0.clone()), f252(v0), None::<Adt6>)
}

pub fn f206(v0: Adt0, v1: Fn2, v2: u64, v3: List<Adt92>, v4: Result<Adt116, Adt108>) -> R<Adt88> {
    let m693 = v4;
    match m693 {
        Ok(v6) => {
            f255(v0, v1, v2, v3, v6)
        }
        Err(v5) => {
            Ok(Adt88::C0(f42(f154(f253(v5.clone()), f156(f254(v5.clone()), f205(v5)))?)))
        }
    }
}

pub fn f207(v0: Adt6) -> R<Result<Adt116, Adt108>> {
    let m694 = v0;
    match m694 {
        Adt6::C0 => {
            Ok(f257(None::<Adt104>, Adt109::C0))
        }
        Adt6::C1(_) => {
            Ok(f257(None::<Adt104>, Adt109::C0))
        }
        Adt6::C2(_) => {
            Ok(f257(None::<Adt104>, Adt109::C0))
        }
        Adt6::C3(_) => {
            Ok(f257(None::<Adt104>, Adt109::C0))
        }
        Adt6::C4(_) => {
            Ok(f257(None::<Adt104>, Adt109::C1))
        }
        Adt6::C5(v1) => {
            f256(Adt5::C0(v1))
        }
    }
}

pub fn f208() -> u64 {
    405u64
}

pub fn f209() -> List<u64> {
    List::cons(65u64, List::cons(108u64, List::cons(108u64, List::cons(111u64, List::cons(119u64, List::<u64>::nil())))))
}

pub fn f210() -> List<u64> {
    List::cons(80u64, List::cons(79u64, List::cons(83u64, List::cons(84u64, List::<u64>::nil()))))
}

pub fn f211(p699: List<Adt92>, v1: List<u64>, p700: Adt111) -> R<Adt111> {
    let m698 = (p699, p700);
    {
        let mut m698 = m698;
        loop {
            let step = {
                let (v0, v2) = m698;
                let m695 = v0;
                match m695.uncons() {
                    None => {
                        Err::<(List<Adt92>, Adt111), R<Adt111>>(Ok(v2))
                    }
                    Some((v3, v4)) => {
                        if f258(match v3.clone() {
                            Adt92::C0(h696, _) => {
                                h696
                            }
                        }, v1.clone())? {
                            Ok::<(List<Adt92>, Adt111), R<Adt111>>((v4, f259(v2, match v3 {
                                Adt92::C0(_, h697) => {
                                    h697
                                }
                            })))
                        } else {
                            Ok::<(List<Adt92>, Adt111), R<Adt111>>((v4, v2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m698 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f212(v0: List<u64>) -> Option<List<u64>> {
    let v1: List<u64> = f260(v0);
    if f261(v1.clone()) {
        Some(v1)
    } else {
        None::<List<u64>>
    }
}

pub fn f213(v0: List<u64>, v1: List<List<u64>>) -> R<bool> {
    if f258(v0.clone(), f262())? {
        Ok(true)
    } else {
        if f258(v0.clone(), f263())? {
            Ok(true)
        } else {
            if f258(v0.clone(), f264())? {
                Ok(true)
            } else {
                f265(v0, v1)
            }
        }
    }
}

pub fn f214(p705: List<u64>) -> Option<List<u64>> {
    let m704 = p705;
    {
        let mut m704 = m704;
        loop {
            let step = {
                let v0 = m704;
                let m701 = v0;
                match m701.uncons() {
                    None => {
                        Err::<List<u64>, Option<List<u64>>>(None::<List<u64>>)
                    }
                    Some((v1, v2)) => {
                        let a702 = v1;
                        let a703 = 58u64;
                        if nat_eq(a702, a703) {
                            Err::<List<u64>, Option<List<u64>>>(f266(v2))
                        } else {
                            Ok::<List<u64>, Option<List<u64>>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m704 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f215(v0: Option<(List<u64>, List<u64>)>) -> Option<List<u64>> {
    let m706 = v0;
    match m706 {
        None => {
            None::<List<u64>>
        }
        Some(v1) => {
            if f267({
                let (_, h707) = v1;
                h707
            }) {
                Some({
                    let (h708, _) = v1;
                    h708
                })
            } else {
                None::<List<u64>>
            }
        }
    }
}

pub fn f216(p713: List<u64>, p714: List<u64>) -> Option<(List<u64>, List<u64>)> {
    let m712 = (p713, p714);
    {
        let mut m712 = m712;
        loop {
            let step = {
                let (v0, v1) = m712;
                let m709 = v0;
                match m709.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>(None::<(List<u64>, List<u64>)>)
                    }
                    Some((v2, v3)) => {
                        let a710 = v2;
                        let a711 = 93u64;
                        if nat_eq(a710, a711) {
                            Err::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>(Some((f268(List::cons(v2, v1), List::<u64>::nil()), v3)))
                        } else {
                            Ok::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>((v3, List::cons(v2, v1)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m712 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f217(p719: List<u64>, p720: List<u64>) -> Option<(List<u64>, List<u64>)> {
    let m718 = (p719, p720);
    {
        let mut m718 = m718;
        loop {
            let step = {
                let (v0, v1) = m718;
                let m715 = v0;
                match m715.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>(Some((f268(v1, List::<u64>::nil()), List::<u64>::nil())))
                    }
                    Some((v2, v3)) => {
                        let a716 = v2;
                        let a717 = 58u64;
                        if nat_eq(a716, a717) {
                            Err::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>(Some((f268(v1, List::<u64>::nil()), List::cons(v2, v3))))
                        } else {
                            Ok::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>((v3, List::cons(v2, v1)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m718 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f218(v0: Bytes, p726: u64, p727: u64, p728: List<List<u64>>) -> R<Result<(List<List<u64>>, u64), Adt90>> {
    let m725 = (p726, (p727, p728));
    {
        let mut m725 = m725;
        loop {
            let step = {
                let (v1, (v2, v3)) = m725;
                let m721 = v2;
                if m721 == 0 {
                    Err::<(u64, (u64, List<List<u64>>)), R<Result<(List<List<u64>>, u64), Adt90>>>(Ok(Ok::<(List<List<u64>>, u64), Adt90>((f109(v3, List::<List<u64>>::nil()), v1))))
                } else {
                    let v4 = m721 - 1;
                    let m722 = f41(v0.clone(), v1)?;
                    match m722 {
                        None => {
                            Err::<(u64, (u64, List<List<u64>>)), R<Result<(List<List<u64>>, u64), Adt90>>>(Ok(Err::<(List<List<u64>>, u64), Adt90>(Adt90::C0(v1))))
                        }
                        Some(v5) => {
                            Ok::<(u64, (u64, List<List<u64>>)), R<Result<(List<List<u64>>, u64), Adt90>>>((match v5.clone() {
                                Adt94::C0(_, h723) => {
                                    h723
                                }
                            }, (v4, List::cons(f18(match v5 {
                                Adt94::C0(h724, _) => {
                                    h724
                                }
                            })?, v3))))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m725 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f219(v0: Adt91, v1: List<List<u64>>) -> Adt91 {
    Adt91::C0(match v0.clone() {
        Adt91::C0(h729, _, _, _, _, _, _) => {
            h729
        }
    }, match v0.clone() {
        Adt91::C0(_, h730, _, _, _, _, _) => {
            h730
        }
    }, match v0.clone() {
        Adt91::C0(_, _, h731, _, _, _, _) => {
            h731
        }
    }, match v0.clone() {
        Adt91::C0(_, _, _, h732, _, _, _) => {
            h732
        }
    }, match v0.clone() {
        Adt91::C0(_, _, _, _, h733, _, _) => {
            h733
        }
    }, v1, match v0 {
        Adt91::C0(_, _, _, _, _, _, h734) => {
            h734
        }
    })
}

pub fn f220(p737: List<u64>, v1: bool) -> bool {
    let m736 = p737;
    {
        let mut m736 = m736;
        loop {
            let step = {
                let v0 = m736;
                let m735 = v0;
                match m735.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        if if v1 {
                            f269(v2)
                        } else {
                            f270(v2)
                        } {
                            Ok::<List<u64>, bool>(v3)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m736 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f221() -> u64 {
    64u64
}

pub fn f222(v0: Str, v1: List<(Str, Adt6)>) -> Option<Adt93> {
    let m738 = f239(Str::lit("type"), v1);
    match m738 {
        Adt114::C0 => {
            Some(Adt93::C3(v0))
        }
        Adt114::C1(v2) => {
            let m739 = f130(v2);
            match m739 {
                None => {
                    Some(Adt93::C3(v0))
                }
                Some(v3) => {
                    let a740 = v3;
                    let a741 = Str::lit("object");
                    if equal(a740, a741) {
                        None::<Adt93>
                    } else {
                        Some(Adt93::C3(v0))
                    }
                }
            }
        }
    }
}

pub fn f223(v0: Str, v1: List<(Str, Adt6)>) -> Option<Adt93> {
    let m742 = f239(Str::lit("$schema"), v1);
    match m742 {
        Adt114::C0 => {
            None::<Adt93>
        }
        Adt114::C1(v2) => {
            let m743 = f130(v2);
            match m743 {
                None => {
                    Some(Adt93::C2(v0))
                }
                Some(v3) => {
                    let a744 = v3;
                    let a745 = f271();
                    if equal(a744, a745) {
                        None::<Adt93>
                    } else {
                        Some(Adt93::C2(v0))
                    }
                }
            }
        }
    }
}

pub fn f224(v0: Adt37) -> R<Option<Adt93>> {
    let m747 = match v0.clone() {
        Adt37::C0(_, _, _, _, _, h746, _, _) => {
            h746
        }
    };
    match m747 {
        None => {
            Ok(None::<Adt93>)
        }
        Some(v1) => {
            let m750 = f223(match v0.clone() {
                Adt37::C0(h748, _, _, _, _, _, _, _) => {
                    h748
                }
            }, match v1.clone() {
                Adt5::C0(h749) => {
                    h749
                }
            });
            match m750 {
                None => {
                    let m754 = f174(f175(), match v0 {
                        Adt37::C0(h751, _, _, _, _, _, _, _) => {
                            h751
                        }
                    }, Adt6::C5(match v1.clone() {
                        Adt5::C0(h752) => {
                            h752
                        }
                    }), List::cons(Adt105::C0(Adt6::C5(match v1 {
                        Adt5::C0(h753) => {
                            h753
                        }
                    }), false, List::<Str>::nil()), List::<Adt105>::nil()), f176())?;
                    match m754 {
                        Ok(_) => {
                            Ok(None::<Adt93>)
                        }
                        Err(v3) => {
                            Ok(Some(v3))
                        }
                    }
                }
                Some(v2) => {
                    Ok(Some(v2))
                }
            }
        }
    }
}

pub fn f225(v0: Str, v1: Adt6, v2: Adt105, v3: Adt112, v4: List<Adt105>) -> R<Result<(List<Adt105>, Adt112), Adt93>> {
    let m755 = v2;
    match m755 {
        Adt105::C0(v5, v6, v7) => {
            let m756 = f136(v5.clone());
            match m756 {
                None => {
                    let m757 = f195(v5);
                    match m757 {
                        None => {
                            Ok(Err::<(List<Adt105>, Adt112), Adt93>(Adt93::C5(v0, Str::lit("schema"))))
                        }
                        Some(v9) => {
                            f272(v0, v1, match v9 {
                                Adt5::C0(h758) => {
                                    h758
                                }
                            }, v6, v7, v3, v4)
                        }
                    }
                }
                Some(_) => {
                    Ok(Ok::<(List<Adt105>, Adt112), Adt93>((v4, v3)))
                }
            }
        }
    }
}

pub fn f226(v0: Adt6, v1: List<(Str, Adt6)>, v2: Adt6, v3: List<Adt100>) -> R<Adt113> {
    let m759 = f273(v1.clone(), v2.clone())?;
    match m759 {
        Adt113::C0(v4) => {
            Ok(Adt113::C0(v4))
        }
        Adt113::C1(v5) => {
            Ok(Adt113::C1(v5))
        }
        Adt113::C2(v6) => {
            Ok(Adt113::C2(v6))
        }
        Adt113::C3(_) => {
            f274(v0, v1, v2, v3)
        }
    }
}

pub fn f227(v0: List<Adt6>, v1: Adt6, v2: List<Adt100>, v3: u64, v4: List<Adt102>) -> Adt106 {
    let m760 = v0;
    match m760.uncons() {
        None => {
            let a761 = v3;
            let a762 = 1u64;
            if nat_eq(a761, a762) {
                Adt106::C0(Adt101::C0, v4)
            } else {
                Adt106::C0(Adt101::C1(v2, Str::lit("oneOf")), v4)
            }
        }
        Some((v5, v6)) => {
            Adt106::C0(Adt101::C0, List::cons(Adt102::C0(v5, v1.clone(), v2.clone()), List::cons(Adt102::C2(v6, v1, v2, v3), v4)))
        }
    }
}

pub fn f228(v0: Option<Adt6>, v1: Adt6, v2: List<Adt100>, v3: List<Adt102>) -> Adt106 {
    let m763 = v0;
    match m763 {
        None => {
            Adt106::C0(Adt101::C0, v3)
        }
        Some(v4) => {
            Adt106::C0(Adt101::C0, List::cons(Adt102::C0(v4, v1, v2), v3))
        }
    }
}

pub fn f229(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt100>) -> Adt113 {
    let m764 = f239(Str::lit("anyOf"), v0);
    match m764 {
        Adt114::C0 => {
            Adt113::C3(List::<Adt102>::nil())
        }
        Adt114::C1(v3) => {
            let m765 = f185(v3);
            match m765 {
                None => {
                    Adt113::C1(Str::lit("anyOf"))
                }
                Some(v4) => {
                    let m766 = v4;
                    match m766.uncons() {
                        None => {
                            Adt113::C1(Str::lit("anyOf"))
                        }
                        Some((v5, v6)) => {
                            Adt113::C3(List::cons(Adt102::C0(v5, v1.clone(), v2.clone()), List::cons(Adt102::C1(v6, v1, v2), List::<Adt102>::nil())))
                        }
                    }
                }
            }
        }
    }
}

pub fn f230(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt100>) -> Adt113 {
    let m767 = f239(Str::lit("oneOf"), v0);
    match m767 {
        Adt114::C0 => {
            Adt113::C3(List::<Adt102>::nil())
        }
        Adt114::C1(v3) => {
            let m768 = f185(v3);
            match m768 {
                None => {
                    Adt113::C1(Str::lit("oneOf"))
                }
                Some(v4) => {
                    let m769 = v4;
                    match m769.uncons() {
                        None => {
                            Adt113::C1(Str::lit("oneOf"))
                        }
                        Some((v5, v6)) => {
                            Adt113::C3(List::cons(Adt102::C0(v5, v1.clone(), v2.clone()), List::cons(Adt102::C2(v6, v1, v2, 0u64), List::<Adt102>::nil())))
                        }
                    }
                }
            }
        }
    }
}

pub fn f231(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt100>) -> Adt113 {
    let m770 = f239(Str::lit("not"), v0);
    match m770 {
        Adt114::C0 => {
            Adt113::C3(List::<Adt102>::nil())
        }
        Adt114::C1(v3) => {
            Adt113::C3(List::cons(Adt102::C0(v3, v1, v2.clone()), List::cons(Adt102::C3(v2), List::<Adt102>::nil())))
        }
    }
}

pub fn f232(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt100>) -> Adt113 {
    let m771 = f239(Str::lit("if"), v0.clone());
    match m771 {
        Adt114::C0 => {
            Adt113::C3(List::<Adt102>::nil())
        }
        Adt114::C1(v3) => {
            Adt113::C3(List::cons(Adt102::C0(v3, v1.clone(), v2.clone()), List::cons(Adt102::C4(f275(Str::lit("then"), v0.clone()), f275(Str::lit("else"), v0), v1, v2), List::<Adt102>::nil())))
        }
    }
}

pub fn f233(v0: List<Adt6>, v1: Adt6, v2: List<Adt100>, v3: List<Adt102>) -> Adt106 {
    let m772 = v0;
    match m772.uncons() {
        None => {
            Adt106::C0(Adt101::C1(v2, Str::lit("anyOf")), v3)
        }
        Some((v4, v5)) => {
            Adt106::C0(Adt101::C0, List::cons(Adt102::C0(v4, v1.clone(), v2.clone()), List::cons(Adt102::C1(v5, v1, v2), v3)))
        }
    }
}

pub fn f234(v0: Adt100) -> R<Str> {
    let m773 = v0;
    match m773 {
        Adt100::C0(v1) => {
            f276(v1)
        }
        Adt100::C1(v2) => {
            let a774 = f145(v2, 0i64)?;
            Ok(format_i64(a774))
        }
    }
}

pub fn f235(v0: Str) -> R<bool> {
    let m776 = f277(f18({
        let a775 = v0;
        utf8_encode(a775)
    })?)?;
    match m776 {
        None => {
            Ok(false)
        }
        Some(_) => {
            Ok(true)
        }
    }
}

pub fn f236(v0: Adt39) -> bool {
    let m778 = match v0 {
        Adt39::C0(_, _, h777) => {
            h777
        }
    };
    match m778 {
        None => {
            false
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f237(v0: Adt51) -> R<Result<Adt34, Adt86>> {
    let m780 = match v0.clone() {
        Adt51::C0(_, h779) => {
            h779
        }
    };
    match m780.uncons() {
        None => {
            Ok(Ok::<Adt34, Adt86>(Adt34::C2(match v0 {
                Adt51::C0(h781, _) => {
                    h781
                }
            })))
        }
        Some((_, _)) => {
            if f278(match v0.clone() {
                Adt51::C0(_, h782) => {
                    h782
                }
            })? {
                Ok(Ok::<Adt34, Adt86>(Adt34::C3(v0)))
            } else {
                Ok(Err::<Adt34, Adt86>(Adt86::C6(Str::lit("progress did not increase"))))
            }
        }
    }
}

pub fn f238(v0: Adt5, v1: Adt51) -> R<Result<Adt34, Adt86>> {
    let m786 = {
        let m785 = match v1.clone() {
            Adt51::C0(h784, _) => {
                h784
            }
        };
        match m785 {
            Adt39::C0(_, h783, _) => {
                h783
            }
        }
    };
    match m786 {
        None => {
            Ok(Err::<Adt34, Adt86>(Adt86::C6(Str::lit("the tool declares an outputSchema and returned no structuredContent"))))
        }
        Some(v2) => {
            let m789 = f79(Adt6::C5(match v0.clone() {
                Adt5::C0(h787) => {
                    h787
                }
            }), Adt6::C5(match v0 {
                Adt5::C0(h788) => {
                    h788
                }
            }), v2)?;
            match m789 {
                Adt99::C0 => {
                    f237(v1)
                }
                Adt99::C1(_, _) => {
                    Ok(Err::<Adt34, Adt86>(Adt86::C6(Str::lit("structuredContent does not conform to the outputSchema"))))
                }
                Adt99::C2(_) => {
                    Ok(Err::<Adt34, Adt86>(Adt86::C9(Str::lit("the outputSchema is malformed"))))
                }
                Adt99::C3(_) => {
                    Ok(Err::<Adt34, Adt86>(Adt86::C9(Str::lit("the outputSchema has an unresolved reference"))))
                }
                Adt99::C4 => {
                    Ok(Err::<Adt34, Adt86>(Adt86::C9(Str::lit("the outputSchema exceeded the validation budget"))))
                }
            }
        }
    }
}

pub fn f239(v0: Str, p795: List<(Str, Adt6)>) -> Adt114 {
    let m794 = p795;
    {
        let mut m794 = m794;
        loop {
            let step = {
                let v1 = m794;
                let m790 = v1;
                match m790.uncons() {
                    None => {
                        Err::<List<(Str, Adt6)>, Adt114>(Adt114::C0)
                    }
                    Some((v2, v3)) => {
                        let m792 = f279({
                            let (h791, _) = v2;
                            h791
                        }, v0.clone());
                        match m792 {
                            Adt132::C0 => {
                                Err::<List<(Str, Adt6)>, Adt114>(Adt114::C1({
                                    let (_, h793) = v2;
                                    h793
                                }))
                            }
                            Adt132::C1 => {
                                Ok::<List<(Str, Adt6)>, Adt114>(v3)
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m794 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f240() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f241(v0: List<u64>) -> R<Adt117> {
    Ok(f280({
        let a796 = v0.clone();
        length_list(a796)
    }, f281(Adt118::C0, 0u64, 0u64, v0)?))
}

pub fn f242(p803: Adt119, p804: u64, p805: List<u64>) -> R<Result<Adt6, Adt107>> {
    let m802 = (p803, (p804, p805));
    {
        let mut m802 = m802;
        loop {
            let step = {
                let (v0, (v1, v2)) = m802;
                let m797 = v2;
                match m797.uncons() {
                    None => {
                        let m798 = f282(v0, v1)?;
                        match m798 {
                            Ok(v3) => {
                                Err::<(Adt119, (u64, List<u64>)), R<Result<Adt6, Adt107>>>(Ok(Ok::<Adt6, Adt107>(v3)))
                            }
                            Err(v4) => {
                                Err::<(Adt119, (u64, List<u64>)), R<Result<Adt6, Adt107>>>(Ok(Err::<Adt6, Adt107>(v4)))
                            }
                        }
                    }
                    Some((v5, v6)) => {
                        let m799 = f283(v0, v1, v5)?;
                        match m799 {
                            Adt133::C0(v7) => {
                                Err::<(Adt119, (u64, List<u64>)), R<Result<Adt6, Adt107>>>(Ok(Err::<Adt6, Adt107>(v7)))
                            }
                            Adt133::C1(v8) => {
                                Ok::<(Adt119, (u64, List<u64>)), R<Result<Adt6, Adt107>>>((v8, ({
                                    let a800 = v1;
                                    let a801 = 1u64;
                                    nat_add(a800, a801)?
                                }, v6)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m802 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f243() -> Adt119 {
    Adt119::C0(Adt120::C0, List::<Adt124>::nil(), 0u64)
}

pub fn f244(p812: u64, p813: List<(Adt6, Adt6)>) -> R<Adt115> {
    let m811 = (p812, p813);
    {
        let mut m811 = m811;
        loop {
            let step = {
                let (v0, v1) = m811;
                let m806 = v0;
                if m806 == 0 {
                    Err::<(u64, List<(Adt6, Adt6)>), R<Adt115>>(Ok(Adt115::C2))
                } else {
                    let v2 = m806 - 1;
                    let m807 = v1;
                    match m807.uncons() {
                        None => {
                            Err::<(u64, List<(Adt6, Adt6)>), R<Adt115>>(Ok(Adt115::C0))
                        }
                        Some((v3, v4)) => {
                            let m810 = f284({
                                let (h808, _) = v3;
                                h808
                            }, {
                                let (_, h809) = v3;
                                h809
                            }, v4)?;
                            match m810 {
                                Adt134::C0 => {
                                    Err::<(u64, List<(Adt6, Adt6)>), R<Adt115>>(Ok(Adt115::C1))
                                }
                                Adt134::C1(v5) => {
                                    Ok::<(u64, List<(Adt6, Adt6)>), R<Adt115>>((v2, v5))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m811 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f245() -> u64 {
    67108864u64
}

pub fn f246(v0: u64, v1: Adt6) -> R<Result<List<u64>, Adt107>> {
    let m814 = f285(v0, List::cons(Adt125::C1(0u64, v1), List::<Adt125>::nil()), List::<u64>::nil())?;
    match m814 {
        Ok(v2) => {
            Ok(Ok::<List<u64>, Adt107>(f286(v2, List::<u64>::nil())))
        }
        Err(v3) => {
            Ok(Err::<List<u64>, Adt107>(v3))
        }
    }
}

pub fn f247() -> u64 {
    67108864u64
}

pub fn f248() -> Str {
    Str::lit("2.0")
}

pub fn f249(v0: Option<Adt104>) -> List<(Str, Adt6)> {
    let m815 = v0;
    match m815 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("id"), f287(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f250(v0: Adt82) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("code"), Adt6::C2(Adt7::C0(match v0.clone() {
        Adt82::C0(h816, _, _) => {
            h816
        }
    }, 0i64))), List::cons((Str::lit("message"), Adt6::C3(match v0.clone() {
        Adt82::C0(_, h817, _) => {
            h817
        }
    })), f288(match v0 {
        Adt82::C0(_, _, h818) => {
            h818
        }
    }))))
}

pub fn f251(v0: Adt108) -> i64 {
    let m819 = v0;
    match m819 {
        Adt108::C0 => {
            f289()
        }
        Adt108::C1(_, _) => {
            f290()
        }
    }
}

pub fn f252(v0: Adt108) -> Str {
    let m820 = v0;
    match m820 {
        Adt108::C0 => {
            Str::lit("Parse error")
        }
        Adt108::C1(_, v2) => {
            f291(v2)
        }
    }
}

pub fn f253(v0: Adt108) -> u64 {
    let m821 = v0;
    match m821 {
        Adt108::C0 => {
            f204()
        }
        Adt108::C1(_, _) => {
            f204()
        }
    }
}

pub fn f254(v0: Adt108) -> Option<Adt104> {
    let m822 = v0;
    match m822 {
        Adt108::C0 => {
            None::<Adt104>
        }
        Adt108::C1(v1, _) => {
            v1
        }
    }
}

pub fn f255(v0: Adt0, v1: Fn2, v2: u64, v3: List<Adt92>, v4: Adt116) -> R<Adt88> {
    f292(f293(v3.clone())?, f294(v0.clone(), v1, v2, Some(f295(v3, match v0 {
        Adt0::C0(_, _, _, _, h823, _, _) => {
            h823
        }
    }, f296(v4.clone()), match v4.clone() {
        Adt116::C0(_, h824, _) => {
            h824
        }
    })?), f297(), v4)?)
}

pub fn f256(v0: Adt5) -> R<Result<Adt116, Adt108>> {
    let m825 = f298(v0.clone())?;
    match m825 {
        Adt142::C0 => {
            let m826 = f184(Str::lit("method"), v0.clone());
            match m826 {
                Adt114::C0 => {
                    Ok(f257(None::<Adt104>, f299(v0)))
                }
                Adt114::C1(_) => {
                    Ok(f257(None::<Adt104>, if f300(v0) {
                        Adt109::C8
                    } else {
                        Adt109::C7
                    }))
                }
            }
        }
        Adt142::C1(v2) => {
            Ok(f301(v2, v0))
        }
        Adt142::C2 => {
            Ok(f257(None::<Adt104>, Adt109::C5))
        }
    }
}

pub fn f257(v0: Option<Adt104>, v1: Adt109) -> Result<Adt116, Adt108> {
    Err::<Adt116, Adt108>(Adt108::C1(v0, v1))
}

pub fn f258(p833: List<u64>, p834: List<u64>) -> R<bool> {
    let m832 = (p833, p834);
    {
        let mut m832 = m832;
        loop {
            let step = {
                let (v0, v1) = m832;
                let m827 = v0;
                match m827.uncons() {
                    None => {
                        let m828 = v1;
                        match m828.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), R<bool>>(Ok(true))
                            }
                            Some((_, _)) => {
                                Err::<(List<u64>, List<u64>), R<bool>>(Ok(false))
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m829 = v1;
                        match m829.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), R<bool>>(Ok(false))
                            }
                            Some((v6, v7)) => {
                                let a830 = f302(v4)?;
                                let a831 = f302(v6)?;
                                if nat_eq(a830, a831) {
                                    Ok::<(List<u64>, List<u64>), R<bool>>((v5, v7))
                                } else {
                                    Err::<(List<u64>, List<u64>), R<bool>>(Ok(false))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m832 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f259(v0: Adt111, v1: List<u64>) -> Adt111 {
    let m835 = v0;
    match m835 {
        Adt111::C0 => {
            Adt111::C1(v1)
        }
        Adt111::C1(_) => {
            Adt111::C2
        }
        Adt111::C2 => {
            Adt111::C2
        }
    }
}

pub fn f260(v0: List<u64>) -> List<u64> {
    f268(f303(f268(f303(v0), List::<u64>::nil())), List::<u64>::nil())
}

pub fn f261(p838: List<u64>) -> bool {
    let m837 = p838;
    {
        let mut m837 = m837;
        loop {
            let step = {
                let v0 = m837;
                let m836 = v0;
                match m836.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f304(v1) {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m837 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f262() -> List<u64> {
    List::cons(108u64, List::cons(111u64, List::cons(99u64, List::cons(97u64, List::cons(108u64, List::cons(104u64, List::cons(111u64, List::cons(115u64, List::cons(116u64, List::<u64>::nil())))))))))
}

pub fn f263() -> List<u64> {
    List::cons(49u64, List::cons(50u64, List::cons(55u64, List::cons(46u64, List::cons(48u64, List::cons(46u64, List::cons(48u64, List::cons(46u64, List::cons(49u64, List::<u64>::nil())))))))))
}

pub fn f264() -> List<u64> {
    List::cons(91u64, List::cons(58u64, List::cons(58u64, List::cons(49u64, List::cons(93u64, List::<u64>::nil())))))
}

pub fn f265(v0: List<u64>, p841: List<List<u64>>) -> R<bool> {
    let m840 = p841;
    {
        let mut m840 = m840;
        loop {
            let step = {
                let v1 = m840;
                let m839 = v1;
                match m839.uncons() {
                    None => {
                        Err::<List<List<u64>>, R<bool>>(Ok(false))
                    }
                    Some((v2, v3)) => {
                        if f258(v0.clone(), v2)? {
                            Err::<List<List<u64>>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<List<u64>>, R<bool>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m840 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f266(v0: List<u64>) -> Option<List<u64>> {
    let m842 = v0;
    match m842.uncons() {
        None => {
            None::<List<u64>>
        }
        Some((v1, v2)) => {
            let m843 = v2;
            match m843.uncons() {
                None => {
                    None::<List<u64>>
                }
                Some((v3, v4)) => {
                    let a844 = v1;
                    let a845 = 47u64;
                    if if nat_eq(a844, a845) {
                        let a846 = v3;
                        let a847 = 47u64;
                        nat_eq(a846, a847)
                    } else {
                        false
                    } {
                        Some(v4)
                    } else {
                        None::<List<u64>>
                    }
                }
            }
        }
    }
}

pub fn f267(v0: List<u64>) -> bool {
    let m848 = v0;
    match m848.uncons() {
        None => {
            true
        }
        Some((v1, v2)) => {
            let a849 = v1;
            let a850 = 58u64;
            if nat_eq(a849, a850) {
                let m851 = v2.clone();
                match m851.uncons() {
                    None => {
                        false
                    }
                    Some((_, _)) => {
                        f305(v2)
                    }
                }
            } else {
                false
            }
        }
    }
}

pub fn f268(p854: List<u64>, p855: List<u64>) -> List<u64> {
    let m853 = (p854, p855);
    {
        let mut m853 = m853;
        loop {
            let step = {
                let (v0, v1) = m853;
                let m852 = v0;
                match m852.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m853 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f269(v0: u64) -> bool {
    let a856 = 48u64;
    let a857 = v0;
    if if nat_le(a856, a857) {
        let a858 = v0;
        let a859 = 57u64;
        nat_le(a858, a859)
    } else {
        false
    } {
        true
    } else {
        let a860 = 65u64;
        let a861 = v0;
        if if nat_le(a860, a861) {
            let a862 = v0;
            let a863 = 90u64;
            nat_le(a862, a863)
        } else {
            false
        } {
            true
        } else {
            let a864 = 97u64;
            let a865 = v0;
            if if nat_le(a864, a865) {
                let a866 = v0;
                let a867 = 122u64;
                nat_le(a866, a867)
            } else {
                false
            } {
                true
            } else {
                let a868 = v0;
                let a869 = 33u64;
                if nat_eq(a868, a869) {
                    true
                } else {
                    let a870 = v0;
                    let a871 = 35u64;
                    if nat_eq(a870, a871) {
                        true
                    } else {
                        let a872 = v0;
                        let a873 = 36u64;
                        if nat_eq(a872, a873) {
                            true
                        } else {
                            let a874 = v0;
                            let a875 = 37u64;
                            if nat_eq(a874, a875) {
                                true
                            } else {
                                let a876 = v0;
                                let a877 = 38u64;
                                if nat_eq(a876, a877) {
                                    true
                                } else {
                                    let a878 = v0;
                                    let a879 = 39u64;
                                    if nat_eq(a878, a879) {
                                        true
                                    } else {
                                        let a880 = v0;
                                        let a881 = 42u64;
                                        if nat_eq(a880, a881) {
                                            true
                                        } else {
                                            let a882 = v0;
                                            let a883 = 43u64;
                                            if nat_eq(a882, a883) {
                                                true
                                            } else {
                                                let a884 = v0;
                                                let a885 = 45u64;
                                                if nat_eq(a884, a885) {
                                                    true
                                                } else {
                                                    let a886 = v0;
                                                    let a887 = 46u64;
                                                    if nat_eq(a886, a887) {
                                                        true
                                                    } else {
                                                        let a888 = v0;
                                                        let a889 = 94u64;
                                                        if nat_eq(a888, a889) {
                                                            true
                                                        } else {
                                                            let a890 = v0;
                                                            let a891 = 95u64;
                                                            if nat_eq(a890, a891) {
                                                                true
                                                            } else {
                                                                let a892 = v0;
                                                                let a893 = 96u64;
                                                                if nat_eq(a892, a893) {
                                                                    true
                                                                } else {
                                                                    let a894 = v0;
                                                                    let a895 = 124u64;
                                                                    if nat_eq(a894, a895) {
                                                                        true
                                                                    } else {
                                                                        let a896 = v0;
                                                                        let a897 = 126u64;
                                                                        nat_eq(a896, a897)
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f270(v0: u64) -> bool {
    let a898 = 48u64;
    let a899 = v0;
    if if nat_le(a898, a899) {
        let a900 = v0;
        let a901 = 57u64;
        nat_le(a900, a901)
    } else {
        false
    } {
        true
    } else {
        let a902 = 65u64;
        let a903 = v0;
        if if nat_le(a902, a903) {
            let a904 = v0;
            let a905 = 90u64;
            nat_le(a904, a905)
        } else {
            false
        } {
            true
        } else {
            let a906 = 97u64;
            let a907 = v0;
            if if nat_le(a906, a907) {
                let a908 = v0;
                let a909 = 122u64;
                nat_le(a908, a909)
            } else {
                false
            } {
                true
            } else {
                let a910 = v0;
                let a911 = 95u64;
                if nat_eq(a910, a911) {
                    true
                } else {
                    let a912 = v0;
                    let a913 = 45u64;
                    if nat_eq(a912, a913) {
                        true
                    } else {
                        let a914 = v0;
                        let a915 = 46u64;
                        nat_eq(a914, a915)
                    }
                }
            }
        }
    }
}

pub fn f271() -> Str {
    Str::lit("https://json-schema.org/draft/2020-12/schema")
}

pub fn f272(v0: Str, v1: Adt6, v2: List<(Str, Adt6)>, v3: bool, v4: List<Str>, v5: Adt112, v6: List<Adt105>) -> R<Result<(List<Adt105>, Adt112), Adt93>> {
    let m916 = f306(v0.clone(), v1, v2.clone());
    match m916 {
        None => {
            let m917 = f307(v0, v2.clone(), v3, v4.clone(), v5)?;
            match m917 {
                Adt143::C0(v8) => {
                    Ok(Err::<(List<Adt105>, Adt112), Adt93>(v8))
                }
                Adt143::C1(v9) => {
                    Ok(Ok::<(List<Adt105>, Adt112), Adt93>((f308(v2, v3, v4, v6), v9)))
                }
            }
        }
        Some(v7) => {
            Ok(Err::<(List<Adt105>, Adt112), Adt93>(v7))
        }
    }
}

pub fn f273(v0: List<(Str, Adt6)>, v1: Adt6) -> R<Adt113> {
    Ok(f309(List::cons((Str::lit("type"), f310(v0.clone(), v1.clone())?), List::cons((Str::lit("enum"), f311(v0.clone(), v1.clone())?), List::cons((Str::lit("const"), f312(v0.clone(), v1.clone())?), List::cons((Str::lit("minimum"), f313(Str::lit("minimum"), 0u64, v0.clone(), v1.clone())?), List::cons((Str::lit("maximum"), f313(Str::lit("maximum"), 1u64, v0.clone(), v1.clone())?), List::cons((Str::lit("exclusiveMinimum"), f313(Str::lit("exclusiveMinimum"), 2u64, v0.clone(), v1.clone())?), List::cons((Str::lit("exclusiveMaximum"), f313(Str::lit("exclusiveMaximum"), 3u64, v0.clone(), v1.clone())?), {
        let a918 = f314(v0.clone(), v1.clone())?;
        let a919 = List::cons((Str::lit("required"), f315(v0, v1)), List::<(Str, Adt131)>::nil());
        append_list(a918, a919)
    })))))))))
}

pub fn f274(v0: Adt6, v1: List<(Str, Adt6)>, v2: Adt6, v3: List<Adt100>) -> R<Adt113> {
    Ok(f316(f317(v0, v1.clone(), v2.clone(), v3.clone()), f316(f318(v1.clone(), v2.clone(), v3.clone()), f316(f319(v1.clone(), v2.clone(), v3.clone()), f316(f320(v1.clone(), v2.clone(), v3.clone())?, f316(f321(v1.clone(), v2.clone(), v3.clone()), f316(f322(Adt103::C0, f229(v1.clone(), v2.clone(), v3.clone()), v1.clone(), v2.clone(), v3.clone()), f316(f322(Adt103::C1, f230(v1.clone(), v2.clone(), v3.clone()), v1.clone(), v2.clone(), v3.clone()), f316(f322(Adt103::C2, f231(v1.clone(), v2.clone(), v3.clone()), v1.clone(), v2.clone(), v3.clone()), f322(Adt103::C3, f232(v1.clone(), v2.clone(), v3.clone()), v1, v2, v3))))))))))
}

pub fn f275(v0: Str, v1: List<(Str, Adt6)>) -> Option<Adt6> {
    let m920 = f239(v0, v1);
    match m920 {
        Adt114::C0 => {
            None::<Adt6>
        }
        Adt114::C1(v2) => {
            Some(v2)
        }
    }
}

pub fn f276(v0: Str) -> R<Str> {
    let m923 = {
        let a922 = f94(f95(), f286(f323(f18({
            let a921 = v0.clone();
            utf8_encode(a921)
        })?, List::<u64>::nil()), List::<u64>::nil()));
        utf8_decode(a922)
    };
    match m923 {
        None => {
            Ok(v0)
        }
        Some(v1) => {
            Ok(v1)
        }
    }
}

pub fn f277(v0: List<u64>) -> R<Option<List<u64>>> {
    let m925 = f324({
        let a924 = v0.clone();
        length_list(a924)
    }, v0, List::<u64>::nil())?;
    match m925 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v1) => {
            Ok(Some(f286(v1, List::<u64>::nil())))
        }
    }
}

pub fn f278(v0: List<Adt52>) -> R<bool> {
    let m926 = v0;
    match m926.uncons() {
        None => {
            Ok(true)
        }
        Some((v1, v2)) => {
            f325(v2, match v1 {
                Adt52::C0(h927, _, _) => {
                    h927
                }
            })
        }
    }
}

pub fn f279(v0: Str, v1: Str) -> Adt132 {
    let a928 = v0;
    let a929 = v1;
    if equal(a928, a929) {
        Adt132::C0
    } else {
        Adt132::C1
    }
}

pub fn f280(v0: u64, v1: u64) -> Adt117 {
    let a930 = v1;
    let a931 = 0u64;
    if nat_eq(a930, a931) {
        Adt117::C0
    } else {
        let a932 = v1;
        let a933 = 2u64;
        if nat_eq(a932, a933) {
            Adt117::C2(v0)
        } else {
            Adt117::C1({
                let a937 = v0;
                let a938 = {
                    let a934 = v1;
                    let a935 = 3u64;
                    let a936 = 0u64;
                    nat_quot(a934, a935, a936)
                };
                nat_sub(a937, a938)
            })
        }
    }
}

pub fn f281(p966: Adt118, p967: u64, p968: u64, p969: List<u64>) -> R<u64> {
    let m965 = (p966, (p967, (p968, p969)));
    {
        let mut m965 = m965;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m965;
                let m939 = v3;
                match m939.uncons() {
                    None => {
                        let m940 = v0;
                        match m940 {
                            Adt118::C0 => {
                                Err::<(Adt118, (u64, (u64, List<u64>))), R<u64>>(Ok(0u64))
                            }
                            Adt118::C1 => {
                                Err::<(Adt118, (u64, (u64, List<u64>))), R<u64>>(Ok(2u64))
                            }
                            Adt118::C2 => {
                                Err::<(Adt118, (u64, (u64, List<u64>))), R<u64>>(Ok(2u64))
                            }
                            Adt118::C3 => {
                                Err::<(Adt118, (u64, (u64, List<u64>))), R<u64>>(Ok(2u64))
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m941 = v0;
                        match m941 {
                            Adt118::C0 => {
                                let a942 = v4;
                                let a943 = 128u64;
                                if nat_lt(a942, a943) {
                                    Ok::<(Adt118, (u64, (u64, List<u64>))), R<u64>>((Adt118::C0, (0u64, (0u64, v5))))
                                } else {
                                    let a944 = v4;
                                    let a945 = 194u64;
                                    if nat_lt(a944, a945) {
                                        let v6: u64 = f326({
                                            let a946 = List::cons(v4, v5);
                                            length_list(a946)
                                        })?;
                                        Err::<(Adt118, (u64, (u64, List<u64>))), R<u64>>(Ok(v6))
                                    } else {
                                        let a947 = v4;
                                        let a948 = 224u64;
                                        if nat_lt(a947, a948) {
                                            Ok::<(Adt118, (u64, (u64, List<u64>))), R<u64>>((Adt118::C1, (128u64, (191u64, v5))))
                                        } else {
                                            let a949 = v4;
                                            let a950 = 224u64;
                                            if nat_eq(a949, a950) {
                                                Ok::<(Adt118, (u64, (u64, List<u64>))), R<u64>>((Adt118::C2, (160u64, (191u64, v5))))
                                            } else {
                                                let a951 = v4;
                                                let a952 = 237u64;
                                                if nat_eq(a951, a952) {
                                                    Ok::<(Adt118, (u64, (u64, List<u64>))), R<u64>>((Adt118::C2, (128u64, (159u64, v5))))
                                                } else {
                                                    let a953 = v4;
                                                    let a954 = 240u64;
                                                    if nat_lt(a953, a954) {
                                                        Ok::<(Adt118, (u64, (u64, List<u64>))), R<u64>>((Adt118::C2, (128u64, (191u64, v5))))
                                                    } else {
                                                        let a955 = v4;
                                                        let a956 = 240u64;
                                                        if nat_eq(a955, a956) {
                                                            Ok::<(Adt118, (u64, (u64, List<u64>))), R<u64>>((Adt118::C3, (144u64, (191u64, v5))))
                                                        } else {
                                                            let a957 = v4;
                                                            let a958 = 244u64;
                                                            if nat_eq(a957, a958) {
                                                                Ok::<(Adt118, (u64, (u64, List<u64>))), R<u64>>((Adt118::C3, (128u64, (143u64, v5))))
                                                            } else {
                                                                let a959 = v4;
                                                                let a960 = 244u64;
                                                                if nat_lt(a959, a960) {
                                                                    Ok::<(Adt118, (u64, (u64, List<u64>))), R<u64>>((Adt118::C3, (128u64, (191u64, v5))))
                                                                } else {
                                                                    let v7: u64 = f326({
                                                                        let a961 = List::cons(v4, v5);
                                                                        length_list(a961)
                                                                    })?;
                                                                    Err::<(Adt118, (u64, (u64, List<u64>))), R<u64>>(Ok(v7))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            Adt118::C1 => {
                                if f327(v4, v1, v2) {
                                    Ok::<(Adt118, (u64, (u64, List<u64>))), R<u64>>((Adt118::C0, (0u64, (0u64, v5))))
                                } else {
                                    let v8: u64 = f326({
                                        let a962 = List::cons(v4, v5);
                                        length_list(a962)
                                    })?;
                                    Err::<(Adt118, (u64, (u64, List<u64>))), R<u64>>(Ok(v8))
                                }
                            }
                            Adt118::C2 => {
                                if f327(v4, v1, v2) {
                                    Ok::<(Adt118, (u64, (u64, List<u64>))), R<u64>>((Adt118::C1, (128u64, (191u64, v5))))
                                } else {
                                    let v9: u64 = f326({
                                        let a963 = List::cons(v4, v5);
                                        length_list(a963)
                                    })?;
                                    Err::<(Adt118, (u64, (u64, List<u64>))), R<u64>>(Ok(v9))
                                }
                            }
                            Adt118::C3 => {
                                if f327(v4, v1, v2) {
                                    Ok::<(Adt118, (u64, (u64, List<u64>))), R<u64>>((Adt118::C2, (128u64, (191u64, v5))))
                                } else {
                                    let v10: u64 = f326({
                                        let a964 = List::cons(v4, v5);
                                        length_list(a964)
                                    })?;
                                    Err::<(Adt118, (u64, (u64, List<u64>))), R<u64>>(Ok(v10))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m965 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f282(v0: Adt119, v1: u64) -> R<Result<Adt6, Adt107>> {
    let m971 = match v0.clone() {
        Adt119::C0(h970, _, _) => {
            h970
        }
    };
    match m971 {
        Adt120::C0 => {
            Ok(Err::<Adt6, Adt107>(Adt107::C3))
        }
        Adt120::C1 => {
            Ok(Err::<Adt6, Adt107>(Adt107::C3))
        }
        Adt120::C2 => {
            Ok(Err::<Adt6, Adt107>(Adt107::C3))
        }
        Adt120::C3 => {
            Ok(Err::<Adt6, Adt107>(Adt107::C3))
        }
        Adt120::C4 => {
            Ok(Err::<Adt6, Adt107>(Adt107::C3))
        }
        Adt120::C5 => {
            Ok(Err::<Adt6, Adt107>(Adt107::C3))
        }
        Adt120::C6(_, _, _) => {
            Ok(Err::<Adt6, Adt107>(Adt107::C3))
        }
        Adt120::C7(_, _) => {
            Ok(Err::<Adt6, Adt107>(Adt107::C3))
        }
        Adt120::C8(v3) => {
            let m972 = f328(v3)?;
            match m972 {
                Adt144::C0 => {
                    Ok(Err::<Adt6, Adt107>(Adt107::C7(v1)))
                }
                Adt144::C1 => {
                    Ok(Err::<Adt6, Adt107>(Adt107::C8(v1)))
                }
                Adt144::C2(v4) => {
                    let m977 = {
                        let m976 = f329(v4, match v0.clone() {
                            Adt119::C0(_, h974, _) => {
                                h974
                            }
                        }, match v0 {
                            Adt119::C0(_, _, h975) => {
                                h975
                            }
                        });
                        match m976 {
                            Adt119::C0(h973, _, _) => {
                                h973
                            }
                        }
                    };
                    match m977 {
                        Adt120::C0 => {
                            Ok(Err::<Adt6, Adt107>(Adt107::C3))
                        }
                        Adt120::C1 => {
                            Ok(Err::<Adt6, Adt107>(Adt107::C3))
                        }
                        Adt120::C2 => {
                            Ok(Err::<Adt6, Adt107>(Adt107::C3))
                        }
                        Adt120::C3 => {
                            Ok(Err::<Adt6, Adt107>(Adt107::C3))
                        }
                        Adt120::C4 => {
                            Ok(Err::<Adt6, Adt107>(Adt107::C3))
                        }
                        Adt120::C5 => {
                            Ok(Err::<Adt6, Adt107>(Adt107::C3))
                        }
                        Adt120::C6(_, _, _) => {
                            Ok(Err::<Adt6, Adt107>(Adt107::C3))
                        }
                        Adt120::C7(_, _) => {
                            Ok(Err::<Adt6, Adt107>(Adt107::C3))
                        }
                        Adt120::C8(_) => {
                            Ok(Err::<Adt6, Adt107>(Adt107::C3))
                        }
                        Adt120::C9(v5) => {
                            Ok(Ok::<Adt6, Adt107>(v5))
                        }
                    }
                }
            }
        }
        Adt120::C9(v2) => {
            Ok(Ok::<Adt6, Adt107>(v2))
        }
    }
}

pub fn f283(v0: Adt119, v1: u64, v2: u64) -> R<Adt133> {
    let m979 = match v0.clone() {
        Adt119::C0(h978, _, _) => {
            h978
        }
    };
    match m979 {
        Adt120::C0 => {
            if f330(v2) {
                Ok(Adt133::C1(v0))
            } else {
                f331(v0, v1, v2)
            }
        }
        Adt120::C1 => {
            if f330(v2) {
                Ok(Adt133::C1(v0))
            } else {
                let a980 = v2;
                let a981 = 93u64;
                if nat_eq(a980, a981) {
                    Ok(f332(v0, v1))
                } else {
                    f331(v0, v1, v2)
                }
            }
        }
        Adt120::C2 => {
            Ok(f333(v0, v1, v2))
        }
        Adt120::C3 => {
            if f330(v2) {
                Ok(Adt133::C1(v0))
            } else {
                let a982 = v2;
                let a983 = 125u64;
                if nat_eq(a982, a983) {
                    Ok(f334(v0, v1))
                } else {
                    let a984 = v2;
                    let a985 = 34u64;
                    if nat_eq(a984, a985) {
                        Ok(f335(v0, Adt120::C6(Adt121::C0, List::<u64>::nil(), true)))
                    } else {
                        Ok(f336(Adt107::C2(v1)))
                    }
                }
            }
        }
        Adt120::C4 => {
            if f330(v2) {
                Ok(Adt133::C1(v0))
            } else {
                let a986 = v2;
                let a987 = 34u64;
                if nat_eq(a986, a987) {
                    Ok(f335(v0, Adt120::C6(Adt121::C0, List::<u64>::nil(), true)))
                } else {
                    Ok(f336(Adt107::C2(v1)))
                }
            }
        }
        Adt120::C5 => {
            if f330(v2) {
                Ok(Adt133::C1(v0))
            } else {
                let a988 = v2;
                let a989 = 58u64;
                if nat_eq(a988, a989) {
                    Ok(f335(v0, Adt120::C0))
                } else {
                    Ok(f336(Adt107::C2(v1)))
                }
            }
        }
        Adt120::C6(v3, v4, v5) => {
            f337(v0, v1, v2, v3, v4, v5)
        }
        Adt120::C7(v6, v7) => {
            f338(v0, v1, v2, v6, v7)
        }
        Adt120::C8(v8) => {
            f339(v0, v1, v2, v8)
        }
        Adt120::C9(_) => {
            Ok(f333(v0, v1, v2))
        }
    }
}

pub fn f284(v0: Adt6, v1: Adt6, v2: List<(Adt6, Adt6)>) -> R<Adt134> {
    let a990 = f340(v0.clone());
    let a991 = f340(v1.clone());
    if nat_eq(a990, a991) {
        let m992 = v0;
        match m992 {
            Adt6::C0 => {
                Ok(Adt134::C1(v2))
            }
            Adt6::C1(v3) => {
                let m993 = f136(v1);
                match m993 {
                    None => {
                        Ok(Adt134::C0)
                    }
                    Some(v4) => {
                        let a994 = v3;
                        let a995 = v4;
                        if equal(a994, a995) {
                            Ok(Adt134::C1(v2))
                        } else {
                            Ok(Adt134::C0)
                        }
                    }
                }
            }
            Adt6::C2(v5) => {
                let m996 = f341(v1);
                match m996 {
                    None => {
                        Ok(Adt134::C0)
                    }
                    Some(v6) => {
                        if f342(f343(v5, v6)?) {
                            Ok(Adt134::C1(v2))
                        } else {
                            Ok(Adt134::C0)
                        }
                    }
                }
            }
            Adt6::C3(v7) => {
                let m997 = f130(v1);
                match m997 {
                    None => {
                        Ok(Adt134::C0)
                    }
                    Some(v8) => {
                        let a998 = v7;
                        let a999 = v8;
                        if equal(a998, a999) {
                            Ok(Adt134::C1(v2))
                        } else {
                            Ok(Adt134::C0)
                        }
                    }
                }
            }
            Adt6::C4(v9) => {
                let m1000 = f185(v1);
                match m1000 {
                    None => {
                        Ok(Adt134::C0)
                    }
                    Some(v10) => {
                        Ok(f344(f345(v9, v10, List::<(Adt6, Adt6)>::nil()), v2))
                    }
                }
            }
            Adt6::C5(v11) => {
                let m1001 = f195(v1);
                match m1001 {
                    None => {
                        Ok(Adt134::C0)
                    }
                    Some(v12) => {
                        let a1003 = f346(v11.clone(), 0u64)?;
                        let a1004 = f346(match v12.clone() {
                            Adt5::C0(h1002) => {
                                h1002
                            }
                        }, 0u64)?;
                        if nat_eq(a1003, a1004) {
                            Ok(f344(f347(v11, match v12 {
                                Adt5::C0(h1005) => {
                                    h1005
                                }
                            }, List::<(Adt6, Adt6)>::nil()), v2))
                        } else {
                            Ok(Adt134::C0)
                        }
                    }
                }
            }
        }
    } else {
        Ok(Adt134::C0)
    }
}

pub fn f285(p1010: u64, p1011: List<Adt125>, p1012: List<u64>) -> R<Result<List<u64>, Adt107>> {
    let m1009 = (p1010, (p1011, p1012));
    {
        let mut m1009 = m1009;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1009;
                let m1006 = v0;
                if m1006 == 0 {
                    Err::<(u64, (List<Adt125>, List<u64>)), R<Result<List<u64>, Adt107>>>(Ok(Err::<List<u64>, Adt107>(Adt107::C13)))
                } else {
                    let v3 = m1006 - 1;
                    let m1007 = v1;
                    match m1007.uncons() {
                        None => {
                            Err::<(u64, (List<Adt125>, List<u64>)), R<Result<List<u64>, Adt107>>>(Ok(Ok::<List<u64>, Adt107>(v2)))
                        }
                        Some((v4, v5)) => {
                            let m1008 = f348(v4, v5, v2)?;
                            match m1008 {
                                Adt146::C0(v6) => {
                                    Err::<(u64, (List<Adt125>, List<u64>)), R<Result<List<u64>, Adt107>>>(Ok(Err::<List<u64>, Adt107>(v6)))
                                }
                                Adt146::C1(v7, v8) => {
                                    Ok::<(u64, (List<Adt125>, List<u64>)), R<Result<List<u64>, Adt107>>>((v3, (v7, v8)))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1009 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f286(p1015: List<u64>, p1016: List<u64>) -> List<u64> {
    let m1014 = (p1015, p1016);
    {
        let mut m1014 = m1014;
        loop {
            let step = {
                let (v0, v1) = m1014;
                let m1013 = v0;
                match m1013.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1014 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f287(v0: Adt104) -> Adt6 {
    let m1017 = v0;
    match m1017 {
        Adt104::C0(v1) => {
            Adt6::C3(v1)
        }
        Adt104::C1(v2) => {
            Adt6::C2(Adt7::C0(v2, 0i64))
        }
    }
}

pub fn f288(v0: Option<Adt6>) -> List<(Str, Adt6)> {
    let m1018 = v0;
    match m1018 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("data"), v1), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f289() -> i64 {
    -32700i64
}

pub fn f290() -> i64 {
    -32600i64
}

pub fn f291(v0: Adt109) -> Str {
    let m1019 = v0;
    match m1019 {
        Adt109::C0 => {
            Str::lit("Invalid Request: the body is not a JSON object")
        }
        Adt109::C1 => {
            Str::lit("Invalid Request: batches are not supported")
        }
        Adt109::C2 => {
            Str::lit("Invalid Request: jsonrpc must be 2.0")
        }
        Adt109::C3 => {
            Str::lit("Invalid Request: method is required")
        }
        Adt109::C4 => {
            Str::lit("Invalid Request: method must be a string")
        }
        Adt109::C5 => {
            Str::lit("Invalid Request: ident must be a string or an integer")
        }
        Adt109::C6 => {
            Str::lit("Invalid Request: params must be an object")
        }
        Adt109::C7 => {
            Str::lit("Invalid Request: notifications are not accepted")
        }
        Adt109::C8 => {
            Str::lit("Invalid Request: the client must not send responses")
        }
    }
}

pub fn f292(v0: bool, v1: Adt135) -> R<Adt88> {
    let m1021 = f349(match v1.clone() {
        Adt135::C0(_, h1020) => {
            h1020
        }
    });
    match m1021 {
        None => {
            Ok(Adt88::C0(f42(f351(v0, v1)?)))
        }
        Some(v2) => {
            f350(v0, v1, v2)
        }
    }
}

pub fn f293(p1026: List<Adt92>) -> R<bool> {
    let m1025 = p1026;
    {
        let mut m1025 = m1025;
        loop {
            let step = {
                let v0 = m1025;
                let m1022 = v0;
                match m1022.uncons() {
                    None => {
                        Err::<List<Adt92>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        if f258(match v1.clone() {
                            Adt92::C0(h1023, _) => {
                                h1023
                            }
                        }, f352())? {
                            if f353(f354(44u64, match v1 {
                                Adt92::C0(_, h1024) => {
                                    h1024
                                }
                            }, List::<u64>::nil(), List::<List<u64>>::nil()))? {
                                Err::<List<Adt92>, R<bool>>(Ok(true))
                            } else {
                                Ok::<List<Adt92>, R<bool>>(v2)
                            }
                        } else {
                            Ok::<List<Adt92>, R<bool>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1025 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f294(v0: Adt0, v1: Fn2, v2: u64, v3: Option<Adt126>, v4: List<Str>, v5: Adt116) -> R<Adt135> {
    Ok(Adt135::C0(match v5.clone() {
        Adt116::C0(h1027, _, _) => {
            h1027
        }
    }, f355(v0, v1, v2, f356(v3, v4, v5)?)?))
}

pub fn f295(v0: List<Adt92>, v1: List<Adt12>, v2: Option<Str>, v3: Str) -> R<Adt126> {
    let v4: Adt141 = f357(Adt128::C0, f162(v0.clone(), f358())?);
    let v5: Adt141 = f357(Adt128::C1, f162(v0.clone(), f359())?);
    let v6: Adt141 = if f360(v3) {
        f357(Adt128::C2, f361(f162(v0.clone(), f362())?)?)
    } else {
        f363()
    };
    Ok(Adt126::C0(match v4.clone() {
        Adt141::C0(h1028, _) => {
            h1028
        }
    }, match v5.clone() {
        Adt141::C0(h1029, _) => {
            h1029
        }
    }, match v6.clone() {
        Adt141::C0(h1030, _) => {
            h1030
        }
    }, f364(match v4 {
        Adt141::C0(_, h1031) => {
            h1031
        }
    }, f364(match v5 {
        Adt141::C0(_, h1032) => {
            h1032
        }
    }, match v6 {
        Adt141::C0(_, h1033) => {
            h1033
        }
    })), f365(v0, v1, v2)?))
}

pub fn f296(v0: Adt116) -> Option<Str> {
    let a1035 = match v0.clone() {
        Adt116::C0(_, h1034, _) => {
            h1034
        }
    };
    let a1036 = Str::lit("tools/call");
    if equal(a1035, a1036) {
        let m1038 = match v0 {
            Adt116::C0(_, _, h1037) => {
                h1037
            }
        };
        match m1038 {
            None => {
                None::<Str>
            }
            Some(v1) => {
                let m1039 = f184(Str::lit("name"), v1);
                match m1039 {
                    Adt114::C0 => {
                        None::<Str>
                    }
                    Adt114::C1(v2) => {
                        f130(v2)
                    }
                }
            }
        }
    } else {
        None::<Str>
    }
}

pub fn f297() -> List<Str> {
    List::cons(f366(), List::<Str>::nil())
}

pub fn f298(v0: Adt5) -> R<Adt142> {
    let m1040 = f184(Str::lit("id"), v0);
    match m1040 {
        Adt114::C0 => {
            Ok(Adt142::C0)
        }
        Adt114::C1(v1) => {
            let m1041 = v1;
            match m1041 {
                Adt6::C0 => {
                    Ok(Adt142::C2)
                }
                Adt6::C1(_) => {
                    Ok(Adt142::C2)
                }
                Adt6::C2(v3) => {
                    let m1042 = f367(v3)?;
                    match m1042 {
                        None => {
                            Ok(Adt142::C2)
                        }
                        Some(v4) => {
                            Ok(Adt142::C1(Adt104::C1(v4)))
                        }
                    }
                }
                Adt6::C3(v2) => {
                    Ok(Adt142::C1(Adt104::C0(v2)))
                }
                Adt6::C4(_) => {
                    Ok(Adt142::C2)
                }
                Adt6::C5(_) => {
                    Ok(Adt142::C2)
                }
            }
        }
    }
}

pub fn f299(v0: Adt5) -> Adt109 {
    if f300(v0) {
        Adt109::C8
    } else {
        Adt109::C3
    }
}

pub fn f300(v0: Adt5) -> bool {
    if f368(Str::lit("result"), v0.clone()) {
        true
    } else {
        f368(Str::lit("error"), v0)
    }
}

pub fn f301(v0: Adt104, v1: Adt5) -> Result<Adt116, Adt108> {
    let m1043 = f184(Str::lit("jsonrpc"), v1.clone());
    match m1043 {
        Adt114::C0 => {
            f257(Some(v0), Adt109::C2)
        }
        Adt114::C1(v2) => {
            let m1044 = f130(v2);
            match m1044 {
                None => {
                    f257(Some(v0), Adt109::C2)
                }
                Some(v3) => {
                    let a1045 = v3;
                    let a1046 = f248();
                    if equal(a1045, a1046) {
                        f369(v0, v1)
                    } else {
                        f257(Some(v0), Adt109::C2)
                    }
                }
            }
        }
    }
}

pub fn f302(v0: u64) -> R<u64> {
    let a1047 = 64u64;
    let a1048 = v0;
    if if nat_lt(a1047, a1048) {
        let a1049 = v0;
        let a1050 = 91u64;
        nat_lt(a1049, a1050)
    } else {
        false
    } {
        let a1051 = v0;
        let a1052 = 32u64;
        nat_add(a1051, a1052)
    } else {
        Ok(v0)
    }
}

pub fn f303(p1055: List<u64>) -> List<u64> {
    let m1054 = p1055;
    {
        let mut m1054 = m1054;
        loop {
            let step = {
                let v0 = m1054;
                let m1053 = v0;
                match m1053.uncons() {
                    None => {
                        Err::<List<u64>, List<u64>>(List::<u64>::nil())
                    }
                    Some((v1, v2)) => {
                        if f370(v1) {
                            Ok::<List<u64>, List<u64>>(v2)
                        } else {
                            Err::<List<u64>, List<u64>>(List::cons(v1, v2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1054 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f304(v0: u64) -> bool {
    let a1056 = v0;
    let a1057 = 9u64;
    if nat_eq(a1056, a1057) {
        true
    } else {
        let a1058 = 31u64;
        let a1059 = v0;
        if nat_lt(a1058, a1059) {
            let a1060 = v0;
            let a1061 = 127u64;
            nat_lt(a1060, a1061)
        } else {
            false
        }
    }
}

pub fn f305(p1064: List<u64>) -> bool {
    let m1063 = p1064;
    {
        let mut m1063 = m1063;
        loop {
            let step = {
                let v0 = m1063;
                let m1062 = v0;
                match m1062.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f371(v1) {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1063 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f306(v0: Str, v1: Adt6, v2: List<(Str, Adt6)>) -> Option<Adt93> {
    let m1065 = f372(f373(), v2.clone());
    match m1065 {
        None => {
            let m1066 = f374(f375(), v2.clone());
            match m1066 {
                None => {
                    f376(v0, v1, v2)
                }
                Some(v4) => {
                    Some(Adt93::C5(v0, v4))
                }
            }
        }
        Some(v3) => {
            Some(Adt93::C4(v0, v3))
        }
    }
}

pub fn f307(v0: Str, v1: List<(Str, Adt6)>, v2: bool, v3: List<Str>, v4: Adt112) -> R<Adt143> {
    let m1067 = f239(Str::lit("x-mcp-header"), v1.clone());
    match m1067 {
        Adt114::C0 => {
            Ok(Adt143::C1(v4))
        }
        Adt114::C1(v5) => {
            f377(v0, v1, v5, v2, v3, v4)
        }
    }
}

pub fn f308(v0: List<(Str, Adt6)>, v1: bool, v2: List<Str>, v3: List<Adt105>) -> List<Adt105> {
    f378(Str::lit("properties"), v0.clone(), v1, v2, f378(Str::lit("$defs"), v0.clone(), false, List::<Str>::nil(), f379(Str::lit("items"), v0.clone(), f379(Str::lit("additionalProperties"), v0.clone(), f380(Str::lit("allOf"), v0.clone(), f380(Str::lit("anyOf"), v0.clone(), f380(Str::lit("oneOf"), v0.clone(), f379(Str::lit("not"), v0.clone(), f379(Str::lit("if"), v0.clone(), f379(Str::lit("then"), v0.clone(), f379(Str::lit("else"), v0, v3)))))))))))
}

pub fn f309(p1074: List<(Str, Adt131)>) -> Adt113 {
    let m1073 = p1074;
    {
        let mut m1073 = m1073;
        loop {
            let step = {
                let v0 = m1073;
                let m1068 = v0;
                match m1068.uncons() {
                    None => {
                        Err::<List<(Str, Adt131)>, Adt113>(Adt113::C3(List::<Adt102>::nil()))
                    }
                    Some((v1, v2)) => {
                        let m1070 = {
                            let (_, h1069) = v1;
                            h1069
                        };
                        match m1070 {
                            Adt131::C0 => {
                                Ok::<List<(Str, Adt131)>, Adt113>(v2)
                            }
                            Adt131::C1 => {
                                Err::<List<(Str, Adt131)>, Adt113>(Adt113::C0({
                                    let (h1071, _) = v1;
                                    h1071
                                }))
                            }
                            Adt131::C2 => {
                                Err::<List<(Str, Adt131)>, Adt113>(Adt113::C1({
                                    let (h1072, _) = v1;
                                    h1072
                                }))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1073 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f310(v0: List<(Str, Adt6)>, v1: Adt6) -> R<Adt131> {
    let m1075 = f239(Str::lit("type"), v0);
    match m1075 {
        Adt114::C0 => {
            Ok(Adt131::C0)
        }
        Adt114::C1(v2) => {
            let m1076 = f130(v2.clone());
            match m1076 {
                None => {
                    let m1077 = f185(v2);
                    match m1077 {
                        None => {
                            Ok(Adt131::C2)
                        }
                        Some(v4) => {
                            f383(v4, v1, false)
                        }
                    }
                }
                Some(v3) => {
                    if f381(v3.clone()) {
                        if f382(v3, v1)? {
                            Ok(Adt131::C0)
                        } else {
                            Ok(Adt131::C1)
                        }
                    } else {
                        Ok(Adt131::C2)
                    }
                }
            }
        }
    }
}

pub fn f311(v0: List<(Str, Adt6)>, v1: Adt6) -> R<Adt131> {
    let m1078 = f239(Str::lit("enum"), v0);
    match m1078 {
        Adt114::C0 => {
            Ok(Adt131::C0)
        }
        Adt114::C1(v2) => {
            let m1079 = f185(v2);
            match m1079 {
                None => {
                    Ok(Adt131::C2)
                }
                Some(v3) => {
                    f384(v3, v1)
                }
            }
        }
    }
}

pub fn f312(v0: List<(Str, Adt6)>, v1: Adt6) -> R<Adt131> {
    let m1080 = f239(Str::lit("const"), v0);
    match m1080 {
        Adt114::C0 => {
            Ok(Adt131::C0)
        }
        Adt114::C1(v2) => {
            let m1081 = f196(v2, v1)?;
            match m1081 {
                Adt115::C0 => {
                    Ok(Adt131::C0)
                }
                Adt115::C1 => {
                    Ok(Adt131::C1)
                }
                Adt115::C2 => {
                    Ok(Adt131::C2)
                }
            }
        }
    }
}

pub fn f313(v0: Str, v1: u64, v2: List<(Str, Adt6)>, v3: Adt6) -> R<Adt131> {
    let m1082 = f239(v0, v2);
    match m1082 {
        Adt114::C0 => {
            Ok(Adt131::C0)
        }
        Adt114::C1(v4) => {
            let m1083 = f341(v4);
            match m1083 {
                None => {
                    Ok(Adt131::C2)
                }
                Some(v5) => {
                    let m1084 = f341(v3);
                    match m1084 {
                        None => {
                            Ok(Adt131::C0)
                        }
                        Some(v6) => {
                            if f385(v1, f343(v6, v5)?) {
                                Ok(Adt131::C0)
                            } else {
                                Ok(Adt131::C1)
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f314(v0: List<(Str, Adt6)>, v1: Adt6) -> R<List<(Str, Adt131)>> {
    let m1085 = v1;
    match m1085 {
        Adt6::C0 => {
            Ok(List::<(Str, Adt131)>::nil())
        }
        Adt6::C1(_) => {
            Ok(List::<(Str, Adt131)>::nil())
        }
        Adt6::C2(_) => {
            Ok(List::<(Str, Adt131)>::nil())
        }
        Adt6::C3(v4) => {
            Ok(List::cons((Str::lit("minLength"), f386(Str::lit("minLength"), true, v0.clone(), f387(v4.clone())?)?), List::cons((Str::lit("maxLength"), f386(Str::lit("maxLength"), false, v0, f387(v4)?)?), List::<(Str, Adt131)>::nil())))
        }
        Adt6::C4(v5) => {
            Ok(List::cons((Str::lit("minItems"), f386(Str::lit("minItems"), true, v0.clone(), f388(v5.clone(), 0u64)?)?), List::cons((Str::lit("maxItems"), f386(Str::lit("maxItems"), false, v0, f388(v5, 0u64)?)?), List::<(Str, Adt131)>::nil())))
        }
        Adt6::C5(v6) => {
            Ok(List::cons((Str::lit("minProperties"), f386(Str::lit("minProperties"), true, v0.clone(), f346(v6.clone(), 0u64)?)?), List::cons((Str::lit("maxProperties"), f386(Str::lit("maxProperties"), false, v0, f346(v6, 0u64)?)?), List::<(Str, Adt131)>::nil())))
        }
    }
}

pub fn f315(v0: List<(Str, Adt6)>, v1: Adt6) -> Adt131 {
    let m1086 = f239(Str::lit("required"), v0);
    match m1086 {
        Adt114::C0 => {
            Adt131::C0
        }
        Adt114::C1(v2) => {
            let m1087 = f185(v2);
            match m1087 {
                None => {
                    Adt131::C2
                }
                Some(v3) => {
                    let m1088 = f195(v1);
                    match m1088 {
                        None => {
                            Adt131::C0
                        }
                        Some(v4) => {
                            f389(v3, v4)
                        }
                    }
                }
            }
        }
    }
}

pub fn f316(v0: Adt113, v1: Adt113) -> Adt113 {
    let m1089 = v0.clone();
    match m1089 {
        Adt113::C0(_) => {
            v0
        }
        Adt113::C1(_) => {
            v0
        }
        Adt113::C2(_) => {
            v0
        }
        Adt113::C3(v5) => {
            let m1090 = v1.clone();
            match m1090 {
                Adt113::C0(_) => {
                    v1
                }
                Adt113::C1(_) => {
                    v1
                }
                Adt113::C2(_) => {
                    v1
                }
                Adt113::C3(v9) => {
                    Adt113::C3({
                        let a1091 = v5;
                        let a1092 = v9;
                        append_list(a1091, a1092)
                    })
                }
            }
        }
    }
}

pub fn f317(v0: Adt6, v1: List<(Str, Adt6)>, v2: Adt6, v3: List<Adt100>) -> Adt113 {
    let m1093 = f239(Str::lit("$ref"), v1);
    match m1093 {
        Adt114::C0 => {
            Adt113::C3(List::<Adt102>::nil())
        }
        Adt114::C1(v4) => {
            let m1094 = f130(v4);
            match m1094 {
                None => {
                    Adt113::C1(Str::lit("$ref"))
                }
                Some(v5) => {
                    let m1095 = f390(v0, v5.clone());
                    match m1095 {
                        None => {
                            Adt113::C2(v5)
                        }
                        Some(v6) => {
                            Adt113::C3(List::cons(Adt102::C0(v6, v2, v3), List::<Adt102>::nil()))
                        }
                    }
                }
            }
        }
    }
}

pub fn f318(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt100>) -> Adt113 {
    let m1096 = f239(Str::lit("properties"), v0);
    match m1096 {
        Adt114::C0 => {
            Adt113::C3(List::<Adt102>::nil())
        }
        Adt114::C1(v3) => {
            let m1097 = f195(v3);
            match m1097 {
                None => {
                    Adt113::C1(Str::lit("properties"))
                }
                Some(v4) => {
                    let m1098 = f195(v1);
                    match m1098 {
                        None => {
                            Adt113::C3(List::<Adt102>::nil())
                        }
                        Some(v5) => {
                            Adt113::C3(f391(match v4 {
                                Adt5::C0(h1099) => {
                                    h1099
                                }
                            }, match v5 {
                                Adt5::C0(h1100) => {
                                    h1100
                                }
                            }, v2, List::<Adt102>::nil()))
                        }
                    }
                }
            }
        }
    }
}

pub fn f319(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt100>) -> Adt113 {
    let m1101 = f239(Str::lit("additionalProperties"), v0.clone());
    match m1101 {
        Adt114::C0 => {
            Adt113::C3(List::<Adt102>::nil())
        }
        Adt114::C1(v3) => {
            let m1102 = f195(v1);
            match m1102 {
                None => {
                    Adt113::C3(List::<Adt102>::nil())
                }
                Some(v4) => {
                    let m1103 = f136(v3.clone());
                    match m1103 {
                        None => {
                            Adt113::C3(f394(match v4 {
                                Adt5::C0(h1105) => {
                                    h1105
                                }
                            }, f393(v0), v3, v2, List::<Adt102>::nil()))
                        }
                        Some(v5) => {
                            if v5 {
                                Adt113::C3(List::<Adt102>::nil())
                            } else {
                                if f392(match v4 {
                                    Adt5::C0(h1104) => {
                                        h1104
                                    }
                                }, f393(v0)) {
                                    Adt113::C0(Str::lit("additionalProperties"))
                                } else {
                                    Adt113::C3(List::<Adt102>::nil())
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f320(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt100>) -> R<Adt113> {
    let m1106 = f239(Str::lit("items"), v0);
    match m1106 {
        Adt114::C0 => {
            Ok(Adt113::C3(List::<Adt102>::nil()))
        }
        Adt114::C1(v3) => {
            let m1107 = f185(v1);
            match m1107 {
                None => {
                    Ok(Adt113::C3(List::<Adt102>::nil()))
                }
                Some(v4) => {
                    Ok(Adt113::C3(f395(v4, 0u64, v3, v2, List::<Adt102>::nil())?))
                }
            }
        }
    }
}

pub fn f321(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt100>) -> Adt113 {
    let m1108 = f239(Str::lit("allOf"), v0);
    match m1108 {
        Adt114::C0 => {
            Adt113::C3(List::<Adt102>::nil())
        }
        Adt114::C1(v3) => {
            let m1109 = f185(v3);
            match m1109 {
                None => {
                    Adt113::C1(Str::lit("allOf"))
                }
                Some(v4) => {
                    let m1110 = v4.clone();
                    match m1110.uncons() {
                        None => {
                            Adt113::C1(Str::lit("allOf"))
                        }
                        Some((_, _)) => {
                            Adt113::C3(f396(v4, v1, v2, List::<Adt102>::nil()))
                        }
                    }
                }
            }
        }
    }
}

pub fn f322(v0: Adt103, v1: Adt113, v2: List<(Str, Adt6)>, v3: Adt6, v4: List<Adt100>) -> Adt113 {
    let m1111 = v1.clone();
    match m1111 {
        Adt113::C0(_) => {
            v1
        }
        Adt113::C1(_) => {
            v1
        }
        Adt113::C2(_) => {
            v1
        }
        Adt113::C3(v8) => {
            let m1112 = v8;
            match m1112.uncons() {
                None => {
                    v1
                }
                Some((_, _)) => {
                    Adt113::C3(List::cons(Adt102::C5(v0, Adt6::C5(v2), v3, v4), List::<Adt102>::nil()))
                }
            }
        }
    }
}

pub fn f323(p1119: List<u64>, p1120: List<u64>) -> List<u64> {
    let m1118 = (p1119, p1120);
    {
        let mut m1118 = m1118;
        loop {
            let step = {
                let (v0, v1) = m1118;
                let m1113 = v0;
                match m1113.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        let a1114 = v2;
                        let a1115 = 126u64;
                        if nat_eq(a1114, a1115) {
                            Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(48u64, List::cons(126u64, v1))))
                        } else {
                            let a1116 = v2;
                            let a1117 = 47u64;
                            if nat_eq(a1116, a1117) {
                                Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(49u64, List::cons(126u64, v1))))
                            } else {
                                Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1118 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f324(p1129: u64, p1130: List<u64>, p1131: List<u64>) -> R<Option<List<u64>>> {
    let m1128 = (p1129, (p1130, p1131));
    {
        let mut m1128 = m1128;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1128;
                let m1121 = v0;
                if m1121 == 0 {
                    let m1122 = v1;
                    match m1122.uncons() {
                        None => {
                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(Some(v2)))
                        }
                        Some((_, _)) => {
                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                        }
                    }
                } else {
                    let v5 = m1121 - 1;
                    let m1123 = v1;
                    match m1123.uncons() {
                        None => {
                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(Some(v2)))
                        }
                        Some((v6, v7)) => {
                            let m1124 = v7;
                            match m1124.uncons() {
                                None => {
                                    Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                }
                                Some((v8, v9)) => {
                                    let m1125 = v9;
                                    match m1125.uncons() {
                                        None => {
                                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                        }
                                        Some((v10, v11)) => {
                                            let m1126 = v11;
                                            match m1126.uncons() {
                                                None => {
                                                    Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                                }
                                                Some((v12, v13)) => {
                                                    let m1127 = f397(v6, v8, v10, v12, f398(v13.clone()))?;
                                                    match m1127 {
                                                        None => {
                                                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                                        }
                                                        Some(v14) => {
                                                            Ok::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>((v5, (v13, f286(v14, v2))))
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1128 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f325(p1136: List<Adt52>, p1137: Adt7) -> R<bool> {
    let m1135 = (p1136, p1137);
    {
        let mut m1135 = m1135;
        loop {
            let step = {
                let (v0, v1) = m1135;
                let m1132 = v0;
                match m1132.uncons() {
                    None => {
                        Err::<(List<Adt52>, Adt7), R<bool>>(Ok(true))
                    }
                    Some((v2, v3)) => {
                        if f399(f343(match v2.clone() {
                            Adt52::C0(h1133, _, _) => {
                                h1133
                            }
                        }, v1)?) {
                            Ok::<(List<Adt52>, Adt7), R<bool>>((v3, match v2 {
                                Adt52::C0(h1134, _, _) => {
                                    h1134
                                }
                            }))
                        } else {
                            Err::<(List<Adt52>, Adt7), R<bool>>(Ok(false))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1135 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f326(v0: u64) -> R<u64> {
    let a1140 = {
        let a1138 = v0;
        let a1139 = 3u64;
        nat_mul(a1138, a1139)?
    };
    let a1141 = 1u64;
    nat_add(a1140, a1141)
}

pub fn f327(v0: u64, v1: u64, v2: u64) -> bool {
    let a1142 = v1;
    let a1143 = v0;
    if nat_le(a1142, a1143) {
        let a1144 = v0;
        let a1145 = v2;
        nat_le(a1144, a1145)
    } else {
        false
    }
}

pub fn f328(v0: Adt122) -> R<Adt144> {
    if f400(match v0.clone() {
        Adt122::C0(_, h1146, _, _, _, _, _) => {
            h1146
        }
    }) {
        let v1: i64 = {
            let a1152 = if match v0.clone() {
                Adt122::C0(_, _, _, _, _, h1147, _) => {
                    h1147
                }
            } {
                let a1149 = match v0.clone() {
                    Adt122::C0(_, _, _, _, _, _, h1148) => {
                        h1148
                    }
                };
                int_neg(a1149)?
            } else {
                match v0.clone() {
                    Adt122::C0(_, _, _, _, _, _, h1150) => {
                        h1150
                    }
                }
            };
            let a1153 = match v0.clone() {
                Adt122::C0(_, _, _, _, h1151, _, _) => {
                    h1151
                }
            };
            int_sub(a1152, a1153)?
        };
        let m1156 = {
            let a1154 = v1;
            let a1155 = -9999i64;
            compare(a1154, a1155)
        };
        if if match m1156 {
            Ordering::Less => {
                true
            }
            Ordering::Equal => {
                false
            }
            Ordering::Greater => {
                false
            }
        } {
            true
        } else {
            let m1159 = {
                let a1157 = 9999i64;
                let a1158 = v1;
                compare(a1157, a1158)
            };
            match m1159 {
                Ordering::Less => {
                    true
                }
                Ordering::Equal => {
                    false
                }
                Ordering::Greater => {
                    false
                }
            }
        } {
            Ok(Adt144::C1)
        } else {
            Ok(Adt144::C2(Adt6::C2(Adt7::C0(if match v0.clone() {
                Adt122::C0(h1160, _, _, _, _, _, _) => {
                    h1160
                }
            } {
                let a1162 = match v0 {
                    Adt122::C0(_, _, h1161, _, _, _, _) => {
                        h1161
                    }
                };
                int_neg(a1162)?
            } else {
                match v0 {
                    Adt122::C0(_, _, h1163, _, _, _, _) => {
                        h1163
                    }
                }
            }, v1))))
        }
    } else {
        Ok(Adt144::C0)
    }
}

pub fn f329(v0: Adt6, v1: List<Adt124>, v2: u64) -> Adt119 {
    let m1164 = v1.clone();
    match m1164.uncons() {
        None => {
            Adt119::C0(Adt120::C9(v0), List::<Adt124>::nil(), 0u64)
        }
        Some((v3, v4)) => {
            let m1165 = v3;
            match m1165 {
                Adt124::C0(v5) => {
                    Adt119::C0(Adt120::C2, List::cons(Adt124::C0(List::cons(v0, v5)), v4), v2)
                }
                Adt124::C1(v6, v7) => {
                    let m1166 = v7;
                    match m1166 {
                        None => {
                            Adt119::C0(Adt120::C2, v1, v2)
                        }
                        Some(v8) => {
                            Adt119::C0(Adt120::C2, List::cons(Adt124::C1(List::cons((v8, v0), v6), None::<Str>), v4), v2)
                        }
                    }
                }
            }
        }
    }
}

pub fn f330(v0: u64) -> bool {
    let a1167 = v0;
    let a1168 = 32u64;
    if nat_eq(a1167, a1168) {
        true
    } else {
        let a1169 = v0;
        let a1170 = 10u64;
        if nat_eq(a1169, a1170) {
            true
        } else {
            let a1171 = v0;
            let a1172 = 13u64;
            if nat_eq(a1171, a1172) {
                true
            } else {
                let a1173 = v0;
                let a1174 = 9u64;
                nat_eq(a1173, a1174)
            }
        }
    }
}

pub fn f331(v0: Adt119, v1: u64, v2: u64) -> R<Adt133> {
    let a1175 = v2;
    let a1176 = 34u64;
    if nat_eq(a1175, a1176) {
        Ok(f335(v0, Adt120::C6(Adt121::C0, List::<u64>::nil(), false)))
    } else {
        let a1177 = v2;
        let a1178 = 123u64;
        if nat_eq(a1177, a1178) {
            f401(v0, Adt124::C1(List::<(Str, Adt6)>::nil(), None::<Str>), Adt120::C3, v1)
        } else {
            let a1179 = v2;
            let a1180 = 91u64;
            if nat_eq(a1179, a1180) {
                f401(v0, Adt124::C0(List::<Adt6>::nil()), Adt120::C1, v1)
            } else {
                let a1181 = v2;
                let a1182 = 116u64;
                if nat_eq(a1181, a1182) {
                    Ok(f335(v0, Adt120::C7(0u64, 1u64)))
                } else {
                    let a1183 = v2;
                    let a1184 = 102u64;
                    if nat_eq(a1183, a1184) {
                        Ok(f335(v0, Adt120::C7(1u64, 1u64)))
                    } else {
                        let a1185 = v2;
                        let a1186 = 110u64;
                        if nat_eq(a1185, a1186) {
                            Ok(f335(v0, Adt120::C7(2u64, 1u64)))
                        } else {
                            let a1187 = v2;
                            let a1188 = 45u64;
                            if nat_eq(a1187, a1188) {
                                Ok(f335(v0, f402(true, Adt123::C0, 0i64, 0u64, 0i64, false, 0i64)))
                            } else {
                                let a1189 = v2;
                                let a1190 = 48u64;
                                if nat_eq(a1189, a1190) {
                                    Ok(f335(v0, f402(false, Adt123::C1, 0i64, 0u64, 0i64, false, 0i64)))
                                } else {
                                    if f327(v2, 49u64, 57u64) {
                                        Ok(f335(v0, f402(false, Adt123::C2, f403({
                                            let a1191 = v2;
                                            let a1192 = 48u64;
                                            nat_sub(a1191, a1192)
                                        }), 1u64, 0i64, false, 0i64)))
                                    } else {
                                        Ok(f336(Adt107::C2(v1)))
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f332(v0: Adt119, v1: u64) -> Adt133 {
    let m1194 = match v0.clone() {
        Adt119::C0(_, h1193, _) => {
            h1193
        }
    };
    match m1194.uncons() {
        None => {
            f336(Adt107::C2(v1))
        }
        Some((v2, v3)) => {
            let m1195 = v2;
            match m1195 {
                Adt124::C0(v4) => {
                    Adt133::C1(f329(Adt6::C4(f404(v4, List::<Adt6>::nil())), v3, {
                        let a1197 = match v0 {
                            Adt119::C0(_, _, h1196) => {
                                h1196
                            }
                        };
                        let a1198 = 1u64;
                        nat_sub(a1197, a1198)
                    }))
                }
                Adt124::C1(_, _) => {
                    f336(Adt107::C2(v1))
                }
            }
        }
    }
}

pub fn f333(v0: Adt119, v1: u64, v2: u64) -> Adt133 {
    let m1200 = match v0.clone() {
        Adt119::C0(h1199, _, _) => {
            h1199
        }
    };
    match m1200 {
        Adt120::C0 => {
            f336(Adt107::C2(v1))
        }
        Adt120::C1 => {
            f336(Adt107::C2(v1))
        }
        Adt120::C2 => {
            if f330(v2) {
                Adt133::C1(v0)
            } else {
                let a1201 = v2;
                let a1202 = 44u64;
                if nat_eq(a1201, a1202) {
                    f405(v0, v1)
                } else {
                    let a1203 = v2;
                    let a1204 = 93u64;
                    if nat_eq(a1203, a1204) {
                        f332(v0, v1)
                    } else {
                        let a1205 = v2;
                        let a1206 = 125u64;
                        if nat_eq(a1205, a1206) {
                            f334(v0, v1)
                        } else {
                            f336(Adt107::C2(v1))
                        }
                    }
                }
            }
        }
        Adt120::C3 => {
            f336(Adt107::C2(v1))
        }
        Adt120::C4 => {
            f336(Adt107::C2(v1))
        }
        Adt120::C5 => {
            f336(Adt107::C2(v1))
        }
        Adt120::C6(_, _, _) => {
            f336(Adt107::C2(v1))
        }
        Adt120::C7(_, _) => {
            f336(Adt107::C2(v1))
        }
        Adt120::C8(_) => {
            f336(Adt107::C2(v1))
        }
        Adt120::C9(_) => {
            if f330(v2) {
                Adt133::C1(v0)
            } else {
                f336(Adt107::C10(v1))
            }
        }
    }
}

pub fn f334(v0: Adt119, v1: u64) -> Adt133 {
    let m1208 = match v0.clone() {
        Adt119::C0(_, h1207, _) => {
            h1207
        }
    };
    match m1208.uncons() {
        None => {
            f336(Adt107::C2(v1))
        }
        Some((v2, v3)) => {
            let m1209 = v2;
            match m1209 {
                Adt124::C0(_) => {
                    f336(Adt107::C2(v1))
                }
                Adt124::C1(v5, _) => {
                    Adt133::C1(f329(Adt6::C5(f406(v5, List::<(Str, Adt6)>::nil())), v3, {
                        let a1211 = match v0 {
                            Adt119::C0(_, _, h1210) => {
                                h1210
                            }
                        };
                        let a1212 = 1u64;
                        nat_sub(a1211, a1212)
                    }))
                }
            }
        }
    }
}

pub fn f335(v0: Adt119, v1: Adt120) -> Adt133 {
    Adt133::C1(f407(v0, v1))
}

pub fn f336(v0: Adt107) -> Adt133 {
    Adt133::C0(v0)
}

pub fn f337(v0: Adt119, v1: u64, v2: u64, v3: Adt121, v4: List<u64>, v5: bool) -> R<Adt133> {
    let m1213 = v3;
    match m1213 {
        Adt121::C0 => {
            let a1214 = v2;
            let a1215 = 34u64;
            if nat_eq(a1214, a1215) {
                Ok(f408(v0, v1, v4, v5))
            } else {
                let a1216 = v2;
                let a1217 = 92u64;
                if nat_eq(a1216, a1217) {
                    Ok(f335(v0, Adt120::C6(Adt121::C1, v4, v5)))
                } else {
                    let a1218 = v2;
                    let a1219 = 32u64;
                    if nat_lt(a1218, a1219) {
                        Ok(f336(Adt107::C4(v1)))
                    } else {
                        Ok(f335(v0, Adt120::C6(Adt121::C0, List::cons(v2, v4), v5)))
                    }
                }
            }
        }
        Adt121::C1 => {
            let a1220 = v2;
            let a1221 = 117u64;
            if nat_eq(a1220, a1221) {
                Ok(f335(v0, Adt120::C6(Adt121::C2(0u64, 0u64, 0u64), v4, v5)))
            } else {
                let a1222 = f409(v2);
                let a1223 = 0u64;
                if nat_eq(a1222, a1223) {
                    Ok(f336(Adt107::C5(v1)))
                } else {
                    Ok(f335(v0, Adt120::C6(Adt121::C0, List::cons(f409(v2), v4), v5)))
                }
            }
        }
        Adt121::C2(v6, v7, v8) => {
            let a1224 = 15u64;
            let a1225 = f410(v2);
            if nat_lt(a1224, a1225) {
                Ok(f336(Adt107::C5(v1)))
            } else {
                let a1228 = {
                    let a1226 = v6;
                    let a1227 = 1u64;
                    nat_add(a1226, a1227)?
                };
                let a1229 = 4u64;
                if nat_eq(a1228, a1229) {
                    f411(v0, v1, {
                        let a1232 = {
                            let a1230 = v7;
                            let a1231 = 16u64;
                            nat_mul(a1230, a1231)?
                        };
                        let a1233 = f410(v2);
                        nat_add(a1232, a1233)?
                    }, v8, v4, v5)
                } else {
                    Ok(f335(v0, Adt120::C6(Adt121::C2({
                        let a1234 = v6;
                        let a1235 = 1u64;
                        nat_add(a1234, a1235)?
                    }, {
                        let a1238 = {
                            let a1236 = v7;
                            let a1237 = 16u64;
                            nat_mul(a1236, a1237)?
                        };
                        let a1239 = f410(v2);
                        nat_add(a1238, a1239)?
                    }, v8), v4, v5)))
                }
            }
        }
        Adt121::C3(v9) => {
            let a1240 = v2;
            let a1241 = 92u64;
            if nat_eq(a1240, a1241) {
                Ok(f335(v0, Adt120::C6(Adt121::C4(v9), v4, v5)))
            } else {
                Ok(f336(Adt107::C6(v1)))
            }
        }
        Adt121::C4(v10) => {
            let a1242 = v2;
            let a1243 = 117u64;
            if nat_eq(a1242, a1243) {
                Ok(f335(v0, Adt120::C6(Adt121::C2(0u64, 0u64, v10), v4, v5)))
            } else {
                Ok(f336(Adt107::C6(v1)))
            }
        }
    }
}

pub fn f338(v0: Adt119, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Adt133> {
    let a1244 = v2;
    let a1245 = f412(v3, v4);
    if nat_eq(a1244, a1245) {
        let a1249 = {
            let a1246 = v4;
            let a1247 = 1u64;
            nat_add(a1246, a1247)?
        };
        let a1250 = {
            let a1248 = f413(v3);
            length_list(a1248)
        };
        if nat_eq(a1249, a1250) {
            Ok(f414(v0, f415(v3)))
        } else {
            Ok(f335(v0, Adt120::C7(v3, {
                let a1251 = v4;
                let a1252 = 1u64;
                nat_add(a1251, a1252)?
            })))
        }
    } else {
        Ok(f336(Adt107::C2(v1)))
    }
}

pub fn f339(v0: Adt119, v1: u64, v2: u64, v3: Adt122) -> R<Adt133> {
    let m1254 = match v3.clone() {
        Adt122::C0(_, h1253, _, _, _, _, _) => {
            h1253
        }
    };
    match m1254 {
        Adt123::C0 => {
            let a1255 = v2;
            let a1256 = 48u64;
            if nat_eq(a1255, a1256) {
                Ok(f416(v0, v3, Adt123::C1))
            } else {
                if f327(v2, 49u64, 57u64) {
                    f417(v0, v1, v2, v3, false)
                } else {
                    Ok(f336(Adt107::C7(v1)))
                }
            }
        }
        Adt123::C1 => {
            let a1257 = v2;
            let a1258 = 46u64;
            if nat_eq(a1257, a1258) {
                Ok(f416(v0, v3, Adt123::C3))
            } else {
                if f418(v2) {
                    Ok(f416(v0, v3, Adt123::C5))
                } else {
                    if f419(v2) {
                        Ok(f336(Adt107::C7(v1)))
                    } else {
                        f420(v0, v1, v2, v3)
                    }
                }
            }
        }
        Adt123::C2 => {
            if f419(v2) {
                f417(v0, v1, v2, v3, false)
            } else {
                let a1259 = v2;
                let a1260 = 46u64;
                if nat_eq(a1259, a1260) {
                    Ok(f416(v0, v3, Adt123::C3))
                } else {
                    if f418(v2) {
                        Ok(f416(v0, v3, Adt123::C5))
                    } else {
                        f420(v0, v1, v2, v3)
                    }
                }
            }
        }
        Adt123::C3 => {
            if f419(v2) {
                f417(v0, v1, v2, v3, true)
            } else {
                Ok(f336(Adt107::C7(v1)))
            }
        }
        Adt123::C4 => {
            if f419(v2) {
                f417(v0, v1, v2, v3, true)
            } else {
                if f418(v2) {
                    Ok(f416(v0, v3, Adt123::C5))
                } else {
                    f420(v0, v1, v2, v3)
                }
            }
        }
        Adt123::C5 => {
            let a1261 = v2;
            let a1262 = 43u64;
            if nat_eq(a1261, a1262) {
                Ok(f335(v0, f402(match v3.clone() {
                    Adt122::C0(h1263, _, _, _, _, _, _) => {
                        h1263
                    }
                }, Adt123::C6, match v3.clone() {
                    Adt122::C0(_, _, h1264, _, _, _, _) => {
                        h1264
                    }
                }, match v3.clone() {
                    Adt122::C0(_, _, _, h1265, _, _, _) => {
                        h1265
                    }
                }, match v3 {
                    Adt122::C0(_, _, _, _, h1266, _, _) => {
                        h1266
                    }
                }, false, 0i64)))
            } else {
                let a1267 = v2;
                let a1268 = 45u64;
                if nat_eq(a1267, a1268) {
                    Ok(f335(v0, f402(match v3.clone() {
                        Adt122::C0(h1269, _, _, _, _, _, _) => {
                            h1269
                        }
                    }, Adt123::C6, match v3.clone() {
                        Adt122::C0(_, _, h1270, _, _, _, _) => {
                            h1270
                        }
                    }, match v3.clone() {
                        Adt122::C0(_, _, _, h1271, _, _, _) => {
                            h1271
                        }
                    }, match v3 {
                        Adt122::C0(_, _, _, _, h1272, _, _) => {
                            h1272
                        }
                    }, true, 0i64)))
                } else {
                    if f419(v2) {
                        f421(v0, v1, v2, Adt122::C0(match v3.clone() {
                            Adt122::C0(h1273, _, _, _, _, _, _) => {
                                h1273
                            }
                        }, match v3.clone() {
                            Adt122::C0(_, h1274, _, _, _, _, _) => {
                                h1274
                            }
                        }, match v3.clone() {
                            Adt122::C0(_, _, h1275, _, _, _, _) => {
                                h1275
                            }
                        }, match v3.clone() {
                            Adt122::C0(_, _, _, h1276, _, _, _) => {
                                h1276
                            }
                        }, match v3 {
                            Adt122::C0(_, _, _, _, h1277, _, _) => {
                                h1277
                            }
                        }, false, 0i64), false)
                    } else {
                        Ok(f336(Adt107::C7(v1)))
                    }
                }
            }
        }
        Adt123::C6 => {
            if f419(v2) {
                f421(v0, v1, v2, v3.clone(), match v3 {
                    Adt122::C0(_, _, _, _, _, h1278, _) => {
                        h1278
                    }
                })
            } else {
                Ok(f336(Adt107::C7(v1)))
            }
        }
        Adt123::C7 => {
            if f419(v2) {
                f421(v0, v1, v2, v3.clone(), match v3 {
                    Adt122::C0(_, _, _, _, _, h1279, _) => {
                        h1279
                    }
                })
            } else {
                f420(v0, v1, v2, v3)
            }
        }
    }
}

pub fn f340(v0: Adt6) -> u64 {
    let m1280 = v0;
    match m1280 {
        Adt6::C0 => {
            0u64
        }
        Adt6::C1(_) => {
            1u64
        }
        Adt6::C2(_) => {
            2u64
        }
        Adt6::C3(_) => {
            3u64
        }
        Adt6::C4(_) => {
            4u64
        }
        Adt6::C5(_) => {
            5u64
        }
    }
}

pub fn f341(v0: Adt6) -> Option<Adt7> {
    let m1281 = v0;
    match m1281 {
        Adt6::C0 => {
            None::<Adt7>
        }
        Adt6::C1(_) => {
            None::<Adt7>
        }
        Adt6::C2(v2) => {
            Some(v2)
        }
        Adt6::C3(_) => {
            None::<Adt7>
        }
        Adt6::C4(_) => {
            None::<Adt7>
        }
        Adt6::C5(_) => {
            None::<Adt7>
        }
    }
}

pub fn f342(v0: Adt145) -> bool {
    let m1282 = v0;
    match m1282 {
        Adt145::C0 => {
            false
        }
        Adt145::C1 => {
            true
        }
        Adt145::C2 => {
            false
        }
    }
}

pub fn f343(v0: Adt7, v1: Adt7) -> R<Adt145> {
    f422(v0.clone(), v1.clone(), f423(match v0 {
        Adt7::C0(h1283, _) => {
            h1283
        }
    }), f423(match v1 {
        Adt7::C0(h1284, _) => {
            h1284
        }
    }))
}

pub fn f344(v0: Option<List<(Adt6, Adt6)>>, v1: List<(Adt6, Adt6)>) -> Adt134 {
    let m1285 = v0;
    match m1285 {
        None => {
            Adt134::C0
        }
        Some(v2) => {
            Adt134::C1({
                let a1286 = v2;
                let a1287 = v1;
                append_list(a1286, a1287)
            })
        }
    }
}

pub fn f345(p1292: List<Adt6>, p1293: List<Adt6>, p1294: List<(Adt6, Adt6)>) -> Option<List<(Adt6, Adt6)>> {
    let m1291 = (p1292, (p1293, p1294));
    {
        let mut m1291 = m1291;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1291;
                let m1288 = v0;
                match m1288.uncons() {
                    None => {
                        let m1289 = v1;
                        match m1289.uncons() {
                            None => {
                                Err::<(List<Adt6>, (List<Adt6>, List<(Adt6, Adt6)>)), Option<List<(Adt6, Adt6)>>>(Some(v2))
                            }
                            Some((_, _)) => {
                                Err::<(List<Adt6>, (List<Adt6>, List<(Adt6, Adt6)>)), Option<List<(Adt6, Adt6)>>>(None::<List<(Adt6, Adt6)>>)
                            }
                        }
                    }
                    Some((v5, v6)) => {
                        let m1290 = v1;
                        match m1290.uncons() {
                            None => {
                                Err::<(List<Adt6>, (List<Adt6>, List<(Adt6, Adt6)>)), Option<List<(Adt6, Adt6)>>>(None::<List<(Adt6, Adt6)>>)
                            }
                            Some((v7, v8)) => {
                                Ok::<(List<Adt6>, (List<Adt6>, List<(Adt6, Adt6)>)), Option<List<(Adt6, Adt6)>>>((v6, (v8, List::cons((v5, v7), v2))))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1291 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f346(p1299: List<(Str, Adt6)>, p1300: u64) -> R<u64> {
    let m1298 = (p1299, p1300);
    {
        let mut m1298 = m1298;
        loop {
            let step = {
                let (v0, v1) = m1298;
                let m1295 = v0;
                match m1295.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, u64), R<u64>>(Ok(v1))
                    }
                    Some((_, v3)) => {
                        Ok::<(List<(Str, Adt6)>, u64), R<u64>>((v3, {
                            let a1296 = v1;
                            let a1297 = 1u64;
                            nat_add(a1296, a1297)?
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m1298 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f347(p1306: List<(Str, Adt6)>, v1: List<(Str, Adt6)>, p1307: List<(Adt6, Adt6)>) -> Option<List<(Adt6, Adt6)>> {
    let m1305 = (p1306, p1307);
    {
        let mut m1305 = m1305;
        loop {
            let step = {
                let (v0, v2) = m1305;
                let m1301 = v0;
                match m1301.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<(Adt6, Adt6)>), Option<List<(Adt6, Adt6)>>>(Some(v2))
                    }
                    Some((v3, v4)) => {
                        let m1303 = f239({
                            let (h1302, _) = v3;
                            h1302
                        }, v1.clone());
                        match m1303 {
                            Adt114::C0 => {
                                Err::<(List<(Str, Adt6)>, List<(Adt6, Adt6)>), Option<List<(Adt6, Adt6)>>>(None::<List<(Adt6, Adt6)>>)
                            }
                            Adt114::C1(v5) => {
                                Ok::<(List<(Str, Adt6)>, List<(Adt6, Adt6)>), Option<List<(Adt6, Adt6)>>>((v4, List::cons(({
                                    let (_, h1304) = v3;
                                    h1304
                                }, v5), v2)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1305 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f348(v0: Adt125, v1: List<Adt125>, v2: List<u64>) -> R<Adt146> {
    let m1308 = v0;
    match m1308 {
        Adt125::C0(v3) => {
            Ok(Adt146::C1(v1, f286(v3, v2)))
        }
        Adt125::C1(v4, v5) => {
            f424(v4, v5, v1, v2)
        }
        Adt125::C2(v6, v7) => {
            let m1309 = v7;
            match m1309.uncons() {
                None => {
                    Ok(Adt146::C1(List::cons(Adt125::C0(List::cons(93u64, List::<u64>::nil())), v1), v2))
                }
                Some((v8, v9)) => {
                    Ok(Adt146::C1(List::cons(Adt125::C1(v6, v8), List::cons(Adt125::C3(v6, v9), v1)), v2))
                }
            }
        }
        Adt125::C3(v10, v11) => {
            let m1310 = v11.clone();
            match m1310.uncons() {
                None => {
                    Ok(Adt146::C1(List::cons(Adt125::C0(List::cons(93u64, List::<u64>::nil())), v1), v2))
                }
                Some((_, _)) => {
                    Ok(Adt146::C1(List::cons(Adt125::C0(List::cons(44u64, List::<u64>::nil())), List::cons(Adt125::C2(v10, v11), v1)), v2))
                }
            }
        }
        Adt125::C4(v14, v15) => {
            let m1311 = v15;
            match m1311.uncons() {
                None => {
                    Ok(Adt146::C1(List::cons(Adt125::C0(List::cons(125u64, List::<u64>::nil())), v1), v2))
                }
                Some((v16, v17)) => {
                    Ok(Adt146::C1(List::cons(Adt125::C0({
                        let a1313 = f425({
                            let (h1312, _) = v16;
                            h1312
                        })?;
                        let a1314 = List::cons(58u64, List::<u64>::nil());
                        append_list(a1313, a1314)
                    }), List::cons(Adt125::C1(v14, {
                        let (_, h1315) = v16;
                        h1315
                    }), List::cons(Adt125::C5(v14, v17), v1))), v2))
                }
            }
        }
        Adt125::C5(v18, v19) => {
            let m1316 = v19.clone();
            match m1316.uncons() {
                None => {
                    Ok(Adt146::C1(List::cons(Adt125::C0(List::cons(125u64, List::<u64>::nil())), v1), v2))
                }
                Some((_, _)) => {
                    Ok(Adt146::C1(List::cons(Adt125::C0(List::cons(44u64, List::<u64>::nil())), List::cons(Adt125::C4(v18, v19), v1)), v2))
                }
            }
        }
    }
}

pub fn f349(v0: Result<Adt136, Adt139>) -> Option<Adt87> {
    let m1317 = v0;
    match m1317 {
        Ok(_) => {
            None::<Adt87>
        }
        Err(v2) => {
            let m1318 = v2;
            match m1318 {
                Adt139::C0(_) => {
                    None::<Adt87>
                }
                Adt139::C1(_, _) => {
                    None::<Adt87>
                }
                Adt139::C2(_) => {
                    None::<Adt87>
                }
                Adt139::C3(_) => {
                    None::<Adt87>
                }
                Adt139::C4(v8) => {
                    f426(v8)
                }
            }
        }
    }
}

pub fn f350(v0: bool, v1: Adt135, v2: Adt87) -> R<Adt88> {
    if f427(match v2.clone() {
        Adt87::C0(h1319, _) => {
            h1319
        }
    }) {
        Ok(Adt88::C1(v2))
    } else {
        Ok(Adt88::C0(f42(f351(v0, Adt135::C0(match v1 {
            Adt135::C0(h1320, _) => {
                h1320
            }
        }, Err::<Adt136, Adt139>(f428())))?)))
    }
}

pub fn f351(v0: bool, v1: Adt135) -> R<Adt95> {
    let m1322 = match v1.clone() {
        Adt135::C0(_, h1321) => {
            h1321
        }
    };
    match m1322 {
        Ok(v2) => {
            let m1323 = f429(v2.clone())?;
            match m1323 {
                None => {
                    Ok(f43())
                }
                Some(v3) => {
                    Ok(f430(v0, f154(f431(), f432(match v1 {
                        Adt135::C0(h1324, _) => {
                            h1324
                        }
                    }, v3))?, f433(match v2 {
                        Adt136::C0(_, _, h1325) => {
                            h1325
                        }
                    }, List::<List<u64>>::nil())?))
                }
            }
        }
        Err(v4) => {
            f154(f434(v4.clone()), f156(Some(match v1 {
                Adt135::C0(h1326, _) => {
                    h1326
                }
            }), f435(v4)?))
        }
    }
}

pub fn f352() -> List<u64> {
    List::cons(65u64, List::cons(99u64, List::cons(99u64, List::cons(101u64, List::cons(112u64, List::cons(116u64, List::<u64>::nil()))))))
}

pub fn f353(p1329: List<List<u64>>) -> R<bool> {
    let m1328 = p1329;
    {
        let mut m1328 = m1328;
        loop {
            let step = {
                let v0 = m1328;
                let m1327 = v0;
                match m1327.uncons() {
                    None => {
                        Err::<List<List<u64>>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        if f436(v1)? {
                            Err::<List<List<u64>>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<List<u64>>, R<bool>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1328 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f354(v0: u64, p1334: List<u64>, p1335: List<u64>, p1336: List<List<u64>>) -> List<List<u64>> {
    let m1333 = (p1334, (p1335, p1336));
    {
        let mut m1333 = m1333;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1333;
                let m1330 = v1;
                match m1330.uncons() {
                    None => {
                        Err::<(List<u64>, (List<u64>, List<List<u64>>)), List<List<u64>>>(f109(List::cons(f268(v2, List::<u64>::nil()), v3), List::<List<u64>>::nil()))
                    }
                    Some((v4, v5)) => {
                        let a1331 = v4;
                        let a1332 = v0;
                        if nat_eq(a1331, a1332) {
                            Ok::<(List<u64>, (List<u64>, List<List<u64>>)), List<List<u64>>>((v5, (List::<u64>::nil(), List::cons(f268(v2, List::<u64>::nil()), v3))))
                        } else {
                            Ok::<(List<u64>, (List<u64>, List<List<u64>>)), List<List<u64>>>((v5, (List::cons(v4, v2), v3)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1333 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f355(v0: Adt0, v1: Fn2, v2: u64, v3: Result<Adt147, Adt139>) -> R<Result<Adt136, Adt139>> {
    let m1337 = v3;
    match m1337 {
        Ok(v5) => {
            f437(v0, v1, v2, v5)
        }
        Err(v4) => {
            Ok(Err::<Adt136, Adt139>(v4))
        }
    }
}

pub fn f356(v0: Option<Adt126>, v1: List<Str>, v2: Adt116) -> R<Result<Adt147, Adt139>> {
    Ok(f438(f439(match v2.clone() {
        Adt116::C0(_, h1338, _) => {
            h1338
        }
    }), f440(v0, v1, v2)?))
}

pub fn f357(v0: Adt128, v1: Adt110) -> Adt141 {
    let m1339 = v1;
    match m1339 {
        Adt110::C0 => {
            f363()
        }
        Adt110::C1 => {
            Adt141::C0(None::<Str>, Some(Adt127::C6(v0)))
        }
        Adt110::C2 => {
            Adt141::C0(None::<Str>, Some(Adt127::C7(v0)))
        }
        Adt110::C3(v2) => {
            let m1340 = f441(v2);
            match m1340 {
                None => {
                    Adt141::C0(None::<Str>, Some(Adt127::C6(v0)))
                }
                Some(v3) => {
                    Adt141::C0(Some(v3), None::<Adt127>)
                }
            }
        }
    }
}

pub fn f358() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(112u64, List::cons(114u64, List::cons(111u64, List::cons(116u64, List::cons(111u64, List::cons(99u64, List::cons(111u64, List::cons(108u64, List::cons(45u64, List::cons(118u64, List::cons(101u64, List::cons(114u64, List::cons(115u64, List::cons(105u64, List::cons(111u64, List::cons(110u64, List::<u64>::nil()))))))))))))))))))))
}

pub fn f359() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(109u64, List::cons(101u64, List::cons(116u64, List::cons(104u64, List::cons(111u64, List::cons(100u64, List::<u64>::nil()))))))))))
}

pub fn f360(v0: Str) -> bool {
    let m1341 = f442(v0);
    match m1341 {
        Adt152::C0 => {
            false
        }
        Adt152::C1 => {
            true
        }
        Adt152::C2 => {
            true
        }
        Adt152::C3 => {
            true
        }
    }
}

pub fn f361(v0: Adt110) -> R<Adt110> {
    let m1342 = v0.clone();
    match m1342 {
        Adt110::C0 => {
            Ok(v0)
        }
        Adt110::C1 => {
            Ok(v0)
        }
        Adt110::C2 => {
            Ok(v0)
        }
        Adt110::C3(v1) => {
            let m1343 = f443(v1);
            match m1343 {
                None => {
                    Ok(v0)
                }
                Some(v2) => {
                    let m1344 = f277(v2)?;
                    match m1344 {
                        None => {
                            Ok(Adt110::C1)
                        }
                        Some(v3) => {
                            Ok(Adt110::C3(v3))
                        }
                    }
                }
            }
        }
    }
}

pub fn f362() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(110u64, List::cons(97u64, List::cons(109u64, List::cons(101u64, List::<u64>::nil()))))))))
}

pub fn f363() -> Adt141 {
    Adt141::C0(None::<Str>, None::<Adt127>)
}

pub fn f364(v0: Option<Adt127>, v1: Option<Adt127>) -> Option<Adt127> {
    let m1345 = v0.clone();
    match m1345 {
        None => {
            v1
        }
        Some(_) => {
            v0
        }
    }
}

pub fn f365(v0: List<Adt92>, v1: List<Adt12>, v2: Option<Str>) -> R<List<Adt129>> {
    Ok(f444(f445(v0, v1, v2, List::<Adt129>::nil())?, List::<Adt129>::nil()))
}

pub fn f366() -> Str {
    Str::lit("2026-07-28")
}

pub fn f367(v0: Adt7) -> R<Option<i64>> {
    let m1348 = f446(40u64, match v0.clone() {
        Adt7::C0(h1346, _) => {
            h1346
        }
    }, match v0 {
        Adt7::C0(_, h1347) => {
            h1347
        }
    })?;
    match m1348 {
        None => {
            Ok(None::<i64>)
        }
        Some(v1) => {
            Ok(f447(v1))
        }
    }
}

pub fn f368(v0: Str, v1: Adt5) -> bool {
    f448(f184(v0, v1))
}

pub fn f369(v0: Adt104, v1: Adt5) -> Result<Adt116, Adt108> {
    let m1349 = f184(Str::lit("method"), v1.clone());
    match m1349 {
        Adt114::C0 => {
            f257(Some(v0), f299(v1))
        }
        Adt114::C1(v2) => {
            let m1350 = f130(v2);
            match m1350 {
                None => {
                    f257(Some(v0), Adt109::C4)
                }
                Some(v3) => {
                    f449(v0, v3, v1)
                }
            }
        }
    }
}

pub fn f370(v0: u64) -> bool {
    let a1351 = v0;
    let a1352 = 9u64;
    if nat_eq(a1351, a1352) {
        true
    } else {
        let a1353 = v0;
        let a1354 = 32u64;
        nat_eq(a1353, a1354)
    }
}

pub fn f371(v0: u64) -> bool {
    let a1355 = 47u64;
    let a1356 = v0;
    if nat_lt(a1355, a1356) {
        let a1357 = v0;
        let a1358 = 58u64;
        nat_lt(a1357, a1358)
    } else {
        false
    }
}

pub fn f372(p1362: List<Str>, v1: List<(Str, Adt6)>) -> Option<Str> {
    let m1361 = p1362;
    {
        let mut m1361 = m1361;
        loop {
            let step = {
                let v0 = m1361;
                let m1359 = v0;
                match m1359.uncons() {
                    None => {
                        Err::<List<Str>, Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        let m1360 = f239(v2.clone(), v1.clone());
                        match m1360 {
                            Adt114::C0 => {
                                Ok::<List<Str>, Option<Str>>(v3)
                            }
                            Adt114::C1(_) => {
                                Err::<List<Str>, Option<Str>>(Some(v2))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1361 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f373() -> List<Str> {
    List::cons(Str::lit("pattern"), List::cons(Str::lit("patternProperties"), List::cons(Str::lit("propertyNames"), List::cons(Str::lit("multipleOf"), List::cons(Str::lit("uniqueItems"), List::cons(Str::lit("contains"), List::cons(Str::lit("minContains"), List::cons(Str::lit("maxContains"), List::cons(Str::lit("prefixItems"), List::cons(Str::lit("dependentRequired"), List::cons(Str::lit("dependentSchemas"), List::cons(Str::lit("dependencies"), List::cons(Str::lit("additionalItems"), List::cons(Str::lit("unevaluatedProperties"), List::cons(Str::lit("unevaluatedItems"), List::cons(Str::lit("$dynamicRef"), List::cons(Str::lit("$dynamicAnchor"), List::cons(Str::lit("$id"), List::cons(Str::lit("$vocabulary"), List::cons(Str::lit("contentEncoding"), List::cons(Str::lit("contentMediaType"), List::cons(Str::lit("contentSchema"), List::<Str>::nil()))))))))))))))))))))))
}

pub fn f374(p1369: List<(Str, u64)>, v1: List<(Str, Adt6)>) -> Option<Str> {
    let m1368 = p1369;
    {
        let mut m1368 = m1368;
        loop {
            let step = {
                let v0 = m1368;
                let m1363 = v0;
                match m1363.uncons() {
                    None => {
                        Err::<List<(Str, u64)>, Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        let m1365 = f239({
                            let (h1364, _) = v2.clone();
                            h1364
                        }, v1.clone());
                        match m1365 {
                            Adt114::C0 => {
                                Ok::<List<(Str, u64)>, Option<Str>>(v3)
                            }
                            Adt114::C1(v4) => {
                                if f450({
                                    let (_, h1366) = v2;
                                    h1366
                                }, v4) {
                                    Ok::<List<(Str, u64)>, Option<Str>>(v3)
                                } else {
                                    Err::<List<(Str, u64)>, Option<Str>>(Some({
                                        let (h1367, _) = v2;
                                        h1367
                                    }))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1368 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f375() -> List<(Str, u64)> {
    List::cons((Str::lit("$schema"), 0u64), List::cons((Str::lit("$ref"), 0u64), List::cons((Str::lit("$anchor"), 0u64), List::cons((Str::lit("$comment"), 0u64), List::cons((Str::lit("title"), 0u64), List::cons((Str::lit("description"), 0u64), List::cons((Str::lit("format"), 0u64), List::cons((Str::lit("type"), 7u64), List::cons((Str::lit("enum"), 3u64), List::cons((Str::lit("required"), 5u64), List::cons((Str::lit("properties"), 4u64), List::cons((Str::lit("$defs"), 4u64), List::cons((Str::lit("allOf"), 8u64), List::cons((Str::lit("anyOf"), 8u64), List::cons((Str::lit("oneOf"), 8u64), List::cons((Str::lit("minimum"), 1u64), List::cons((Str::lit("maximum"), 1u64), List::cons((Str::lit("exclusiveMinimum"), 1u64), List::cons((Str::lit("exclusiveMaximum"), 1u64), List::cons((Str::lit("minItems"), 2u64), List::cons((Str::lit("maxItems"), 2u64), List::cons((Str::lit("minLength"), 2u64), List::cons((Str::lit("maxLength"), 2u64), List::cons((Str::lit("minProperties"), 2u64), List::cons((Str::lit("maxProperties"), 2u64), List::cons((Str::lit("deprecated"), 6u64), List::cons((Str::lit("readOnly"), 6u64), List::cons((Str::lit("writeOnly"), 6u64), List::<(Str, u64)>::nil()))))))))))))))))))))))))))))
}

pub fn f376(v0: Str, v1: Adt6, v2: List<(Str, Adt6)>) -> Option<Adt93> {
    let m1370 = f239(Str::lit("$ref"), v2);
    match m1370 {
        Adt114::C0 => {
            None::<Adt93>
        }
        Adt114::C1(v3) => {
            let m1371 = f130(v3);
            match m1371 {
                None => {
                    None::<Adt93>
                }
                Some(v4) => {
                    let m1372 = f390(v1, v4.clone());
                    match m1372 {
                        None => {
                            Some(Adt93::C6(v0, v4))
                        }
                        Some(_) => {
                            None::<Adt93>
                        }
                    }
                }
            }
        }
    }
}

pub fn f377(v0: Str, v1: List<(Str, Adt6)>, v2: Adt6, v3: bool, v4: List<Str>, v5: Adt112) -> R<Adt143> {
    if v3 {
        let m1373 = f130(v2);
        match m1373 {
            None => {
                Ok(Adt143::C0(Adt93::C7(v0)))
            }
            Some(v6) => {
                f451(v0, v1, v6.clone(), f123(v6)?, v4, v5)
            }
        }
    } else {
        Ok(Adt143::C0(Adt93::C10(v0)))
    }
}

pub fn f378(v0: Str, v1: List<(Str, Adt6)>, v2: bool, v3: List<Str>, v4: List<Adt105>) -> List<Adt105> {
    let m1374 = f239(v0.clone(), v1);
    match m1374 {
        Adt114::C0 => {
            v4
        }
        Adt114::C1(v5) => {
            let m1375 = f195(v5);
            match m1375 {
                None => {
                    v4
                }
                Some(v6) => {
                    let a1376 = v0;
                    let a1377 = Str::lit("properties");
                    if equal(a1376, a1377) {
                        f452(match v6 {
                            Adt5::C0(h1378) => {
                                h1378
                            }
                        }, v2, v3, v4)
                    } else {
                        f453(match v6 {
                            Adt5::C0(h1379) => {
                                h1379
                            }
                        }, v4)
                    }
                }
            }
        }
    }
}

pub fn f379(v0: Str, v1: List<(Str, Adt6)>, v2: List<Adt105>) -> List<Adt105> {
    let m1380 = f239(v0, v1);
    match m1380 {
        Adt114::C0 => {
            v2
        }
        Adt114::C1(v3) => {
            List::cons(Adt105::C0(v3, false, List::<Str>::nil()), v2)
        }
    }
}

pub fn f380(v0: Str, v1: List<(Str, Adt6)>, v2: List<Adt105>) -> List<Adt105> {
    let m1381 = f239(v0, v1);
    match m1381 {
        Adt114::C0 => {
            v2
        }
        Adt114::C1(v3) => {
            let m1382 = f185(v3);
            match m1382 {
                None => {
                    v2
                }
                Some(v4) => {
                    f454(v4, v2)
                }
            }
        }
    }
}

pub fn f381(v0: Str) -> bool {
    let a1383 = v0.clone();
    let a1384 = Str::lit("null");
    if equal(a1383, a1384) {
        true
    } else {
        let a1385 = v0.clone();
        let a1386 = Str::lit("boolean");
        if equal(a1385, a1386) {
            true
        } else {
            let a1387 = v0.clone();
            let a1388 = Str::lit("object");
            if equal(a1387, a1388) {
                true
            } else {
                let a1389 = v0.clone();
                let a1390 = Str::lit("array");
                if equal(a1389, a1390) {
                    true
                } else {
                    let a1391 = v0.clone();
                    let a1392 = Str::lit("number");
                    if equal(a1391, a1392) {
                        true
                    } else {
                        let a1393 = v0.clone();
                        let a1394 = Str::lit("string");
                        if equal(a1393, a1394) {
                            true
                        } else {
                            let a1395 = v0;
                            let a1396 = Str::lit("integer");
                            equal(a1395, a1396)
                        }
                    }
                }
            }
        }
    }
}

pub fn f382(v0: Str, v1: Adt6) -> R<bool> {
    let a1397 = v0.clone();
    let a1398 = f455(v1.clone());
    if equal(a1397, a1398) {
        Ok(true)
    } else {
        let a1399 = v0;
        let a1400 = Str::lit("integer");
        if equal(a1399, a1400) {
            let m1401 = f341(v1);
            match m1401 {
                None => {
                    Ok(false)
                }
                Some(v2) => {
                    f456(v2)
                }
            }
        } else {
            Ok(false)
        }
    }
}

pub fn f383(p1404: List<Adt6>, v1: Adt6, p1405: bool) -> R<Adt131> {
    let m1403 = (p1404, p1405);
    {
        let mut m1403 = m1403;
        loop {
            let step = {
                let (v0, v2) = m1403;
                let m1402 = v0;
                match m1402.uncons() {
                    None => {
                        if v2 {
                            Err::<(List<Adt6>, bool), R<Adt131>>(Ok(Adt131::C0))
                        } else {
                            Err::<(List<Adt6>, bool), R<Adt131>>(Ok(Adt131::C1))
                        }
                    }
                    Some((v3, v4)) => {
                        if f381(f457(v3.clone())) {
                            Ok::<(List<Adt6>, bool), R<Adt131>>((v4, if v2 {
                                true
                            } else {
                                f382(f457(v3), v1.clone())?
                            }))
                        } else {
                            Err::<(List<Adt6>, bool), R<Adt131>>(Ok(Adt131::C2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1403 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f384(p1409: List<Adt6>, v1: Adt6) -> R<Adt131> {
    let m1408 = p1409;
    {
        let mut m1408 = m1408;
        loop {
            let step = {
                let v0 = m1408;
                let m1406 = v0;
                match m1406.uncons() {
                    None => {
                        Err::<List<Adt6>, R<Adt131>>(Ok(Adt131::C1))
                    }
                    Some((v2, v3)) => {
                        let m1407 = f196(v2, v1.clone())?;
                        match m1407 {
                            Adt115::C0 => {
                                Err::<List<Adt6>, R<Adt131>>(Ok(Adt131::C0))
                            }
                            Adt115::C1 => {
                                Ok::<List<Adt6>, R<Adt131>>(v3)
                            }
                            Adt115::C2 => {
                                Err::<List<Adt6>, R<Adt131>>(Ok(Adt131::C2))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1408 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f385(v0: u64, v1: Adt145) -> bool {
    let a1410 = v0;
    let a1411 = 0u64;
    if nat_eq(a1410, a1411) {
        let a1412 = f458(v1);
        bool_not(a1412)
    } else {
        let a1413 = v0;
        let a1414 = 1u64;
        if nat_eq(a1413, a1414) {
            let a1415 = f399(v1);
            bool_not(a1415)
        } else {
            let a1416 = v0;
            let a1417 = 2u64;
            if nat_eq(a1416, a1417) {
                f399(v1)
            } else {
                f458(v1)
            }
        }
    }
}

pub fn f386(v0: Str, v1: bool, v2: List<(Str, Adt6)>, v3: u64) -> R<Adt131> {
    let m1418 = f239(v0, v2);
    match m1418 {
        Adt114::C0 => {
            Ok(Adt131::C0)
        }
        Adt114::C1(v4) => {
            let m1419 = f459(v4);
            match m1419 {
                None => {
                    Ok(Adt131::C2)
                }
                Some(v5) => {
                    let m1422 = {
                        let a1420 = v5;
                        let a1421 = 0i64;
                        compare(a1420, a1421)
                    };
                    if match m1422 {
                        Ordering::Less => {
                            true
                        }
                        Ordering::Equal => {
                            false
                        }
                        Ordering::Greater => {
                            false
                        }
                    } {
                        Ok(Adt131::C2)
                    } else {
                        if v1 {
                            if f458(f460(f145(v3, 0i64)?, v5)) {
                                Ok(Adt131::C1)
                            } else {
                                Ok(Adt131::C0)
                            }
                        } else {
                            if f399(f460(f145(v3, 0i64)?, v5)) {
                                Ok(Adt131::C1)
                            } else {
                                Ok(Adt131::C0)
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f387(v0: Str) -> R<u64> {
    f461(f18({
        let a1423 = v0;
        utf8_encode(a1423)
    })?, 0u64)
}

pub fn f388(p1428: List<Adt6>, p1429: u64) -> R<u64> {
    let m1427 = (p1428, p1429);
    {
        let mut m1427 = m1427;
        loop {
            let step = {
                let (v0, v1) = m1427;
                let m1424 = v0;
                match m1424.uncons() {
                    None => {
                        Err::<(List<Adt6>, u64), R<u64>>(Ok(v1))
                    }
                    Some((_, v3)) => {
                        Ok::<(List<Adt6>, u64), R<u64>>((v3, {
                            let a1425 = v1;
                            let a1426 = 1u64;
                            nat_add(a1425, a1426)?
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m1427 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f389(p1433: List<Adt6>, v1: Adt5) -> Adt131 {
    let m1432 = p1433;
    {
        let mut m1432 = m1432;
        loop {
            let step = {
                let v0 = m1432;
                let m1430 = v0;
                match m1430.uncons() {
                    None => {
                        Err::<List<Adt6>, Adt131>(Adt131::C0)
                    }
                    Some((v2, v3)) => {
                        let m1431 = f130(v2);
                        match m1431 {
                            None => {
                                Err::<List<Adt6>, Adt131>(Adt131::C2)
                            }
                            Some(v4) => {
                                if f368(v4, v1.clone()) {
                                    Ok::<List<Adt6>, Adt131>(v3)
                                } else {
                                    Err::<List<Adt6>, Adt131>(Adt131::C1)
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1432 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f390(v0: Adt6, v1: Str) -> Option<Adt6> {
    let a1434 = v1.clone();
    let a1435 = Str::lit("#");
    if equal(a1434, a1435) {
        Some(v0)
    } else {
        let m1436 = f195(v0);
        match m1436 {
            None => {
                None::<Adt6>
            }
            Some(v2) => {
                let m1437 = f184(Str::lit("$defs"), v2);
                match m1437 {
                    Adt114::C0 => {
                        None::<Adt6>
                    }
                    Adt114::C1(v3) => {
                        let m1438 = f195(v3);
                        match m1438 {
                            None => {
                                None::<Adt6>
                            }
                            Some(v4) => {
                                f462(v1, match v4 {
                                    Adt5::C0(h1439) => {
                                        h1439
                                    }
                                })
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f391(p1446: List<(Str, Adt6)>, v1: List<(Str, Adt6)>, v2: List<Adt100>, p1447: List<Adt102>) -> List<Adt102> {
    let m1445 = (p1446, p1447);
    {
        let mut m1445 = m1445;
        loop {
            let step = {
                let (v0, v3) = m1445;
                let m1440 = v0;
                match m1440.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Adt102>), List<Adt102>>(v3)
                    }
                    Some((v4, v5)) => {
                        let m1442 = f239({
                            let (h1441, _) = v4.clone();
                            h1441
                        }, v1.clone());
                        match m1442 {
                            Adt114::C0 => {
                                Ok::<(List<(Str, Adt6)>, List<Adt102>), List<Adt102>>((v5, v3))
                            }
                            Adt114::C1(v6) => {
                                Ok::<(List<(Str, Adt6)>, List<Adt102>), List<Adt102>>((v5, List::cons(Adt102::C0({
                                    let (_, h1443) = v4;
                                    h1443
                                }, v6, List::cons(Adt100::C0({
                                    let (h1444, _) = v4;
                                    h1444
                                }), v2.clone())), v3)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1445 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f392(p1452: List<(Str, Adt6)>, v1: List<(Str, Adt6)>) -> bool {
    let m1451 = p1452;
    {
        let mut m1451 = m1451;
        loop {
            let step = {
                let v0 = m1451;
                let m1448 = v0;
                match m1448.uncons() {
                    None => {
                        Err::<List<(Str, Adt6)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let m1450 = f239({
                            let (h1449, _) = v2;
                            h1449
                        }, v1.clone());
                        match m1450 {
                            Adt114::C0 => {
                                Err::<List<(Str, Adt6)>, bool>(true)
                            }
                            Adt114::C1(_) => {
                                Ok::<List<(Str, Adt6)>, bool>(v3)
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1451 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f393(v0: List<(Str, Adt6)>) -> List<(Str, Adt6)> {
    let m1453 = f239(Str::lit("properties"), v0);
    match m1453 {
        Adt114::C0 => {
            List::<(Str, Adt6)>::nil()
        }
        Adt114::C1(v1) => {
            let m1454 = f195(v1);
            match m1454 {
                None => {
                    List::<(Str, Adt6)>::nil()
                }
                Some(v2) => {
                    match v2 {
                        Adt5::C0(h1455) => {
                            h1455
                        }
                    }
                }
            }
        }
    }
}

pub fn f394(p1462: List<(Str, Adt6)>, v1: List<(Str, Adt6)>, v2: Adt6, v3: List<Adt100>, p1463: List<Adt102>) -> List<Adt102> {
    let m1461 = (p1462, p1463);
    {
        let mut m1461 = m1461;
        loop {
            let step = {
                let (v0, v4) = m1461;
                let m1456 = v0;
                match m1456.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Adt102>), List<Adt102>>(v4)
                    }
                    Some((v5, v6)) => {
                        let m1458 = f239({
                            let (h1457, _) = v5.clone();
                            h1457
                        }, v1.clone());
                        match m1458 {
                            Adt114::C0 => {
                                Ok::<(List<(Str, Adt6)>, List<Adt102>), List<Adt102>>((v6, List::cons(Adt102::C0(v2.clone(), {
                                    let (_, h1459) = v5;
                                    h1459
                                }, List::cons(Adt100::C0({
                                    let (h1460, _) = v5;
                                    h1460
                                }), v3.clone())), v4)))
                            }
                            Adt114::C1(_) => {
                                Ok::<(List<(Str, Adt6)>, List<Adt102>), List<Adt102>>((v6, v4))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1461 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f395(p1468: List<Adt6>, p1469: u64, v2: Adt6, v3: List<Adt100>, p1470: List<Adt102>) -> R<List<Adt102>> {
    let m1467 = (p1468, (p1469, p1470));
    {
        let mut m1467 = m1467;
        loop {
            let step = {
                let (v0, (v1, v4)) = m1467;
                let m1464 = v0;
                match m1464.uncons() {
                    None => {
                        Err::<(List<Adt6>, (u64, List<Adt102>)), R<List<Adt102>>>(Ok(v4))
                    }
                    Some((v5, v6)) => {
                        Ok::<(List<Adt6>, (u64, List<Adt102>)), R<List<Adt102>>>((v6, ({
                            let a1465 = v1;
                            let a1466 = 1u64;
                            nat_add(a1465, a1466)?
                        }, List::cons(Adt102::C0(v2.clone(), v5, List::cons(Adt100::C1(v1), v3.clone())), v4))))
                    }
                }
            };
            match step {
                Ok(next) => { m1467 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f396(p1473: List<Adt6>, v1: Adt6, v2: List<Adt100>, p1474: List<Adt102>) -> List<Adt102> {
    let m1472 = (p1473, p1474);
    {
        let mut m1472 = m1472;
        loop {
            let step = {
                let (v0, v3) = m1472;
                let m1471 = v0;
                match m1471.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<Adt102>), List<Adt102>>(v3)
                    }
                    Some((v4, v5)) => {
                        Ok::<(List<Adt6>, List<Adt102>), List<Adt102>>((v5, List::cons(Adt102::C0(v4, v1.clone(), v2.clone()), v3)))
                    }
                }
            };
            match step {
                Ok(next) => { m1472 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f397(v0: u64, v1: u64, v2: u64, v3: u64, v4: bool) -> R<Option<List<u64>>> {
    let m1475 = f463(v0)?;
    match m1475 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v5) => {
            let m1476 = f463(v1)?;
            match m1476 {
                None => {
                    Ok(None::<List<u64>>)
                }
                Some(v6) => {
                    let a1477 = v2;
                    let a1478 = 61u64;
                    if nat_eq(a1477, a1478) {
                        f464(v5, v6, v4, v3)
                    } else {
                        let m1479 = f463(v2)?;
                        match m1479 {
                            None => {
                                Ok(None::<List<u64>>)
                            }
                            Some(v7) => {
                                let a1480 = v3;
                                let a1481 = 61u64;
                                if nat_eq(a1480, a1481) {
                                    f465(v5, v6, v7, v4)
                                } else {
                                    let m1482 = f463(v3)?;
                                    match m1482 {
                                        None => {
                                            Ok(None::<List<u64>>)
                                        }
                                        Some(v8) => {
                                            Ok(Some(f466(v5, v6, v7, v8)?))
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f398(v0: List<u64>) -> bool {
    let m1483 = v0;
    match m1483.uncons() {
        None => {
            true
        }
        Some((_, _)) => {
            false
        }
    }
}

pub fn f399(v0: Adt145) -> bool {
    let m1484 = v0;
    match m1484 {
        Adt145::C0 => {
            false
        }
        Adt145::C1 => {
            false
        }
        Adt145::C2 => {
            true
        }
    }
}

pub fn f400(v0: Adt123) -> bool {
    let m1485 = v0;
    match m1485 {
        Adt123::C0 => {
            false
        }
        Adt123::C1 => {
            true
        }
        Adt123::C2 => {
            true
        }
        Adt123::C3 => {
            false
        }
        Adt123::C4 => {
            true
        }
        Adt123::C5 => {
            false
        }
        Adt123::C6 => {
            false
        }
        Adt123::C7 => {
            true
        }
    }
}

pub fn f401(v0: Adt119, v1: Adt124, v2: Adt120, v3: u64) -> R<Adt133> {
    let a1487 = match v0.clone() {
        Adt119::C0(_, _, h1486) => {
            h1486
        }
    };
    let a1488 = f467();
    if nat_lt(a1487, a1488) {
        Ok(Adt133::C1(Adt119::C0(v2, List::cons(v1, match v0.clone() {
            Adt119::C0(_, h1489, _) => {
                h1489
            }
        }), {
            let a1491 = match v0 {
                Adt119::C0(_, _, h1490) => {
                    h1490
                }
            };
            let a1492 = 1u64;
            nat_add(a1491, a1492)?
        })))
    } else {
        Ok(f336(Adt107::C9(v3)))
    }
}

pub fn f402(v0: bool, v1: Adt123, v2: i64, v3: u64, v4: i64, v5: bool, v6: i64) -> Adt120 {
    Adt120::C8(Adt122::C0(v0, v1, v2, v3, v4, v5, v6))
}

pub fn f403(v0: u64) -> i64 {
    let a1493 = v0;
    let a1494 = 0u64;
    if nat_eq(a1493, a1494) {
        0i64
    } else {
        let a1495 = v0;
        let a1496 = 1u64;
        if nat_eq(a1495, a1496) {
            1i64
        } else {
            let a1497 = v0;
            let a1498 = 2u64;
            if nat_eq(a1497, a1498) {
                2i64
            } else {
                let a1499 = v0;
                let a1500 = 3u64;
                if nat_eq(a1499, a1500) {
                    3i64
                } else {
                    let a1501 = v0;
                    let a1502 = 4u64;
                    if nat_eq(a1501, a1502) {
                        4i64
                    } else {
                        let a1503 = v0;
                        let a1504 = 5u64;
                        if nat_eq(a1503, a1504) {
                            5i64
                        } else {
                            let a1505 = v0;
                            let a1506 = 6u64;
                            if nat_eq(a1505, a1506) {
                                6i64
                            } else {
                                let a1507 = v0;
                                let a1508 = 7u64;
                                if nat_eq(a1507, a1508) {
                                    7i64
                                } else {
                                    let a1509 = v0;
                                    let a1510 = 8u64;
                                    if nat_eq(a1509, a1510) {
                                        8i64
                                    } else {
                                        9i64
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f404(p1513: List<Adt6>, p1514: List<Adt6>) -> List<Adt6> {
    let m1512 = (p1513, p1514);
    {
        let mut m1512 = m1512;
        loop {
            let step = {
                let (v0, v1) = m1512;
                let m1511 = v0;
                match m1511.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt6>, List<Adt6>), List<Adt6>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1512 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f405(v0: Adt119, v1: u64) -> Adt133 {
    let m1516 = match v0.clone() {
        Adt119::C0(_, h1515, _) => {
            h1515
        }
    };
    match m1516.uncons() {
        None => {
            f336(Adt107::C2(v1))
        }
        Some((v2, _)) => {
            let m1517 = v2;
            match m1517 {
                Adt124::C0(_) => {
                    f335(v0, Adt120::C0)
                }
                Adt124::C1(_, _) => {
                    f335(v0, Adt120::C4)
                }
            }
        }
    }
}

pub fn f406(p1520: List<(Str, Adt6)>, p1521: List<(Str, Adt6)>) -> List<(Str, Adt6)> {
    let m1519 = (p1520, p1521);
    {
        let mut m1519 = m1519;
        loop {
            let step = {
                let (v0, v1) = m1519;
                let m1518 = v0;
                match m1518.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<(Str, Adt6)>), List<(Str, Adt6)>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Adt6)>, List<(Str, Adt6)>), List<(Str, Adt6)>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1519 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f407(v0: Adt119, v1: Adt120) -> Adt119 {
    Adt119::C0(v1, match v0.clone() {
        Adt119::C0(_, h1522, _) => {
            h1522
        }
    }, match v0 {
        Adt119::C0(_, _, h1523) => {
            h1523
        }
    })
}

pub fn f408(v0: Adt119, v1: u64, v2: List<u64>, v3: bool) -> Adt133 {
    let m1525 = {
        let a1524 = f94(f95(), f286(v2, List::<u64>::nil()));
        utf8_decode(a1524)
    };
    match m1525 {
        None => {
            f336(Adt107::C14)
        }
        Some(v4) => {
            if v3 {
                f468(v0, v1, v4)
            } else {
                f414(v0, Adt6::C3(v4))
            }
        }
    }
}

pub fn f409(v0: u64) -> u64 {
    let a1526 = v0;
    let a1527 = 34u64;
    if nat_eq(a1526, a1527) {
        34u64
    } else {
        let a1528 = v0;
        let a1529 = 92u64;
        if nat_eq(a1528, a1529) {
            92u64
        } else {
            let a1530 = v0;
            let a1531 = 47u64;
            if nat_eq(a1530, a1531) {
                47u64
            } else {
                let a1532 = v0;
                let a1533 = 98u64;
                if nat_eq(a1532, a1533) {
                    8u64
                } else {
                    let a1534 = v0;
                    let a1535 = 102u64;
                    if nat_eq(a1534, a1535) {
                        12u64
                    } else {
                        let a1536 = v0;
                        let a1537 = 110u64;
                        if nat_eq(a1536, a1537) {
                            10u64
                        } else {
                            let a1538 = v0;
                            let a1539 = 114u64;
                            if nat_eq(a1538, a1539) {
                                13u64
                            } else {
                                let a1540 = v0;
                                let a1541 = 116u64;
                                if nat_eq(a1540, a1541) {
                                    9u64
                                } else {
                                    0u64
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f410(v0: u64) -> u64 {
    let a1542 = v0;
    let a1543 = 48u64;
    if nat_lt(a1542, a1543) {
        16u64
    } else {
        let a1544 = v0;
        let a1545 = 58u64;
        if nat_lt(a1544, a1545) {
            let a1546 = v0;
            let a1547 = 48u64;
            nat_sub(a1546, a1547)
        } else {
            let a1548 = v0;
            let a1549 = 65u64;
            if nat_lt(a1548, a1549) {
                16u64
            } else {
                let a1550 = v0;
                let a1551 = 71u64;
                if nat_lt(a1550, a1551) {
                    let a1552 = v0;
                    let a1553 = 55u64;
                    nat_sub(a1552, a1553)
                } else {
                    let a1554 = v0;
                    let a1555 = 97u64;
                    if nat_lt(a1554, a1555) {
                        16u64
                    } else {
                        let a1556 = v0;
                        let a1557 = 103u64;
                        if nat_lt(a1556, a1557) {
                            let a1558 = v0;
                            let a1559 = 87u64;
                            nat_sub(a1558, a1559)
                        } else {
                            16u64
                        }
                    }
                }
            }
        }
    }
}

pub fn f411(v0: Adt119, v1: u64, v2: u64, v3: u64, v4: List<u64>, v5: bool) -> R<Adt133> {
    let a1560 = v3;
    let a1561 = 0u64;
    if nat_eq(a1560, a1561) {
        if f327(v2, 55296u64, 56319u64) {
            Ok(f335(v0, Adt120::C6(Adt121::C3(v2), v4, v5)))
        } else {
            if f327(v2, 56320u64, 57343u64) {
                Ok(f336(Adt107::C6(v1)))
            } else {
                Ok(f335(v0, Adt120::C6(Adt121::C0, f469(v2, v4)?, v5)))
            }
        }
    } else {
        if f327(v2, 56320u64, 57343u64) {
            Ok(f335(v0, Adt120::C6(Adt121::C0, f469({
                let a1570 = 65536u64;
                let a1571 = {
                    let a1568 = {
                        let a1564 = {
                            let a1562 = v3;
                            let a1563 = 55296u64;
                            nat_sub(a1562, a1563)
                        };
                        let a1565 = 1024u64;
                        nat_mul(a1564, a1565)?
                    };
                    let a1569 = {
                        let a1566 = v2;
                        let a1567 = 56320u64;
                        nat_sub(a1566, a1567)
                    };
                    nat_add(a1568, a1569)?
                };
                nat_add(a1570, a1571)?
            }, v4)?, v5)))
        } else {
            Ok(f336(Adt107::C6(v1)))
        }
    }
}

pub fn f412(v0: u64, v1: u64) -> u64 {
    let m1574 = {
        let a1572 = f413(v0);
        let a1573 = v1;
        index_list(a1572, a1573)
    };
    match m1574 {
        None => {
            0u64
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f413(v0: u64) -> List<u64> {
    let a1575 = v0;
    let a1576 = 0u64;
    if nat_eq(a1575, a1576) {
        List::cons(116u64, List::cons(114u64, List::cons(117u64, List::cons(101u64, List::<u64>::nil()))))
    } else {
        let a1577 = v0;
        let a1578 = 1u64;
        if nat_eq(a1577, a1578) {
            List::cons(102u64, List::cons(97u64, List::cons(108u64, List::cons(115u64, List::cons(101u64, List::<u64>::nil())))))
        } else {
            List::cons(110u64, List::cons(117u64, List::cons(108u64, List::cons(108u64, List::<u64>::nil()))))
        }
    }
}

pub fn f414(v0: Adt119, v1: Adt6) -> Adt133 {
    Adt133::C1(f329(v1, match v0.clone() {
        Adt119::C0(_, h1579, _) => {
            h1579
        }
    }, match v0 {
        Adt119::C0(_, _, h1580) => {
            h1580
        }
    }))
}

pub fn f415(v0: u64) -> Adt6 {
    let a1581 = v0;
    let a1582 = 0u64;
    if nat_eq(a1581, a1582) {
        Adt6::C1(true)
    } else {
        let a1583 = v0;
        let a1584 = 1u64;
        if nat_eq(a1583, a1584) {
            Adt6::C1(false)
        } else {
            Adt6::C0
        }
    }
}

pub fn f416(v0: Adt119, v1: Adt122, v2: Adt123) -> Adt133 {
    f335(v0, Adt120::C8(Adt122::C0(match v1.clone() {
        Adt122::C0(h1585, _, _, _, _, _, _) => {
            h1585
        }
    }, v2, match v1.clone() {
        Adt122::C0(_, _, h1586, _, _, _, _) => {
            h1586
        }
    }, match v1.clone() {
        Adt122::C0(_, _, _, h1587, _, _, _) => {
            h1587
        }
    }, match v1.clone() {
        Adt122::C0(_, _, _, _, h1588, _, _) => {
            h1588
        }
    }, match v1.clone() {
        Adt122::C0(_, _, _, _, _, h1589, _) => {
            h1589
        }
    }, match v1 {
        Adt122::C0(_, _, _, _, _, _, h1590) => {
            h1590
        }
    })))
}

pub fn f417(v0: Adt119, v1: u64, v2: u64, v3: Adt122, v4: bool) -> R<Adt133> {
    let v5: u64 = {
        let a1592 = match v3.clone() {
            Adt122::C0(_, _, _, h1591, _, _, _) => {
                h1591
            }
        };
        let a1593 = 0u64;
        if if nat_eq(a1592, a1593) {
            let a1594 = v2;
            let a1595 = 48u64;
            nat_eq(a1594, a1595)
        } else {
            false
        } {
            0u64
        } else {
            let a1597 = match v3.clone() {
                Adt122::C0(_, _, _, h1596, _, _, _) => {
                    h1596
                }
            };
            let a1598 = 1u64;
            nat_add(a1597, a1598)?
        }
    };
    let v6: i64 = if v4 {
        f193(match v3.clone() {
            Adt122::C0(_, _, _, _, h1599, _, _) => {
                h1599
            }
        }, 1i64)?
    } else {
        match v3.clone() {
            Adt122::C0(_, _, _, _, h1600, _, _) => {
                h1600
            }
        }
    };
    let a1601 = f470();
    let a1602 = v5;
    if nat_lt(a1601, a1602) {
        Ok(f336(Adt107::C8(v1)))
    } else {
        let m1605 = {
            let a1603 = 99999i64;
            let a1604 = v6;
            compare(a1603, a1604)
        };
        if match m1605 {
            Ordering::Less => {
                true
            }
            Ordering::Equal => {
                false
            }
            Ordering::Greater => {
                false
            }
        } {
            Ok(f336(Adt107::C8(v1)))
        } else {
            Ok(f335(v0, f402(match v3.clone() {
                Adt122::C0(h1606, _, _, _, _, _, _) => {
                    h1606
                }
            }, if v4 {
                Adt123::C4
            } else {
                Adt123::C2
            }, f193({
                let a1608 = match v3.clone() {
                    Adt122::C0(_, _, h1607, _, _, _, _) => {
                        h1607
                    }
                };
                let a1609 = 10i64;
                int_mul(a1608, a1609)?
            }, f403({
                let a1610 = v2;
                let a1611 = 48u64;
                nat_sub(a1610, a1611)
            }))?, v5, v6, match v3.clone() {
                Adt122::C0(_, _, _, _, _, h1612, _) => {
                    h1612
                }
            }, match v3 {
                Adt122::C0(_, _, _, _, _, _, h1613) => {
                    h1613
                }
            })))
        }
    }
}

pub fn f418(v0: u64) -> bool {
    let a1614 = v0;
    let a1615 = 101u64;
    if nat_eq(a1614, a1615) {
        true
    } else {
        let a1616 = v0;
        let a1617 = 69u64;
        nat_eq(a1616, a1617)
    }
}

pub fn f419(v0: u64) -> bool {
    f327(v0, 48u64, 57u64)
}

pub fn f420(v0: Adt119, v1: u64, v2: u64, v3: Adt122) -> R<Adt133> {
    let m1618 = f328(v3)?;
    match m1618 {
        Adt144::C0 => {
            Ok(f336(Adt107::C7(v1)))
        }
        Adt144::C1 => {
            Ok(f336(Adt107::C8(v1)))
        }
        Adt144::C2(v4) => {
            Ok(f333(f329(v4, match v0.clone() {
                Adt119::C0(_, h1619, _) => {
                    h1619
                }
            }, match v0 {
                Adt119::C0(_, _, h1620) => {
                    h1620
                }
            }), v1, v2))
        }
    }
}

pub fn f421(v0: Adt119, v1: u64, v2: u64, v3: Adt122, v4: bool) -> R<Adt133> {
    let v5: i64 = f193({
        let a1622 = match v3.clone() {
            Adt122::C0(_, _, _, _, _, _, h1621) => {
                h1621
            }
        };
        let a1623 = 10i64;
        int_mul(a1622, a1623)?
    }, f403({
        let a1624 = v2;
        let a1625 = 48u64;
        nat_sub(a1624, a1625)
    }))?;
    let m1628 = {
        let a1626 = 99999i64;
        let a1627 = v5;
        compare(a1626, a1627)
    };
    if match m1628 {
        Ordering::Less => {
            true
        }
        Ordering::Equal => {
            false
        }
        Ordering::Greater => {
            false
        }
    } {
        Ok(f336(Adt107::C8(v1)))
    } else {
        Ok(f335(v0, f402(match v3.clone() {
            Adt122::C0(h1629, _, _, _, _, _, _) => {
                h1629
            }
        }, Adt123::C7, match v3.clone() {
            Adt122::C0(_, _, h1630, _, _, _, _) => {
                h1630
            }
        }, match v3.clone() {
            Adt122::C0(_, _, _, h1631, _, _, _) => {
                h1631
            }
        }, match v3 {
            Adt122::C0(_, _, _, _, h1632, _, _) => {
                h1632
            }
        }, v4, v5)))
    }
}

pub fn f422(v0: Adt7, v1: Adt7, v2: Adt145, v3: Adt145) -> R<Adt145> {
    let m1633 = v2;
    match m1633 {
        Adt145::C0 => {
            let m1635 = v3;
            match m1635 {
                Adt145::C0 => {
                    Ok(f471(Adt145::C0, f472(f473(match v0.clone() {
                        Adt7::C0(h1636, _) => {
                            h1636
                        }
                    })?, match v0 {
                        Adt7::C0(_, h1637) => {
                            h1637
                        }
                    }, f473(match v1.clone() {
                        Adt7::C0(h1638, _) => {
                            h1638
                        }
                    })?, match v1 {
                        Adt7::C0(_, h1639) => {
                            h1639
                        }
                    })?))
                }
                Adt145::C1 => {
                    Ok(Adt145::C0)
                }
                Adt145::C2 => {
                    Ok(Adt145::C0)
                }
            }
        }
        Adt145::C1 => {
            let m1634 = v3;
            match m1634 {
                Adt145::C0 => {
                    Ok(Adt145::C2)
                }
                Adt145::C1 => {
                    Ok(Adt145::C1)
                }
                Adt145::C2 => {
                    Ok(Adt145::C0)
                }
            }
        }
        Adt145::C2 => {
            let m1640 = v3;
            match m1640 {
                Adt145::C0 => {
                    Ok(Adt145::C2)
                }
                Adt145::C1 => {
                    Ok(Adt145::C2)
                }
                Adt145::C2 => {
                    f472(match v0.clone() {
                        Adt7::C0(h1641, _) => {
                            h1641
                        }
                    }, match v0 {
                        Adt7::C0(_, h1642) => {
                            h1642
                        }
                    }, match v1.clone() {
                        Adt7::C0(h1643, _) => {
                            h1643
                        }
                    }, match v1 {
                        Adt7::C0(_, h1644) => {
                            h1644
                        }
                    })
                }
            }
        }
    }
}

pub fn f423(v0: i64) -> Adt145 {
    f460(v0, 0i64)
}

pub fn f424(v0: u64, v1: Adt6, v2: List<Adt125>, v3: List<u64>) -> R<Adt146> {
    let m1645 = v1;
    match m1645 {
        Adt6::C0 => {
            Ok(Adt146::C1(v2, f286(f474(2u64), v3)))
        }
        Adt6::C1(v4) => {
            Ok(Adt146::C1(v2, f286(if v4 {
                f474(0u64)
            } else {
                f474(1u64)
            }, v3)))
        }
        Adt6::C2(v5) => {
            if f475(v5.clone())? {
                Ok(Adt146::C1(v2, f286(f476(v5)?, v3)))
            } else {
                Ok(Adt146::C0(Adt107::C11))
            }
        }
        Adt6::C3(v6) => {
            Ok(Adt146::C1(v2, f286(f425(v6)?, v3)))
        }
        Adt6::C4(v7) => {
            let a1646 = v0;
            let a1647 = f467();
            if nat_lt(a1646, a1647) {
                Ok(Adt146::C1(f477(91u64, Adt125::C2({
                    let a1648 = v0;
                    let a1649 = 1u64;
                    nat_add(a1648, a1649)?
                }, v7), v2), v3))
            } else {
                Ok(Adt146::C0(Adt107::C12))
            }
        }
        Adt6::C5(v8) => {
            let a1650 = v0;
            let a1651 = f467();
            if nat_lt(a1650, a1651) {
                Ok(Adt146::C1(f477(123u64, Adt125::C4({
                    let a1652 = v0;
                    let a1653 = 1u64;
                    nat_add(a1652, a1653)?
                }, v8), v2), v3))
            } else {
                Ok(Adt146::C0(Adt107::C12))
            }
        }
    }
}

pub fn f425(v0: Str) -> R<List<u64>> {
    Ok(List::cons(34u64, {
        let a1655 = f286(f478(f18({
            let a1654 = v0;
            utf8_encode(a1654)
        })?, List::<u64>::nil())?, List::<u64>::nil());
        let a1656 = List::cons(34u64, List::<u64>::nil());
        append_list(a1655, a1656)
    }))
}

pub fn f426(v0: Adt86) -> Option<Adt87> {
    let m1657 = v0;
    match m1657 {
        Adt86::C0(_) => {
            None::<Adt87>
        }
        Adt86::C1(_) => {
            None::<Adt87>
        }
        Adt86::C2(_) => {
            None::<Adt87>
        }
        Adt86::C3(_) => {
            None::<Adt87>
        }
        Adt86::C4(_) => {
            None::<Adt87>
        }
        Adt86::C5(_) => {
            None::<Adt87>
        }
        Adt86::C6(_) => {
            None::<Adt87>
        }
        Adt86::C7(_) => {
            None::<Adt87>
        }
        Adt86::C8(_) => {
            None::<Adt87>
        }
        Adt86::C9(_) => {
            None::<Adt87>
        }
        Adt86::C10 => {
            None::<Adt87>
        }
        Adt86::C11 => {
            None::<Adt87>
        }
        Adt86::C12(_) => {
            None::<Adt87>
        }
        Adt86::C13 => {
            None::<Adt87>
        }
        Adt86::C14 => {
            None::<Adt87>
        }
        Adt86::C15(v12) => {
            Some(v12)
        }
    }
}

pub fn f427(v0: u64) -> bool {
    let a1658 = v0;
    let a1659 = f479();
    nat_lt(a1658, a1659)
}

pub fn f428() -> Adt139 {
    Adt139::C4(Adt86::C9(Str::lit("the effect tag does not fit thirty-two bits")))
}

pub fn f429(v0: Adt136) -> R<Option<Adt5>> {
    let m1661 = f480(match v0.clone() {
        Adt136::C0(h1660, _, _) => {
            h1660
        }
    })?;
    match m1661 {
        None => {
            Ok(None::<Adt5>)
        }
        Some(v1) => {
            Ok(Some(Adt5::C0({
                let a1664 = v1;
                let a1665 = {
                    let m1663 = f481(v0);
                    match m1663 {
                        Adt5::C0(h1662) => {
                            h1662
                        }
                    }
                };
                append_list(a1664, a1665)
            })))
        }
    }
}

pub fn f430(v0: bool, v1: Adt95, v2: Option<List<List<u64>>>) -> Adt95 {
    let m1666 = v2;
    match m1666 {
        None => {
            f43()
        }
        Some(v3) => {
            f482(v0, v1, f109(v3, List::<List<u64>>::nil()))
        }
    }
}

pub fn f431() -> u64 {
    200u64
}

pub fn f432(v0: Adt104, v1: Adt5) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("jsonrpc"), Adt6::C3(f248())), List::cons((Str::lit("id"), f287(v0)), List::cons((Str::lit("result"), Adt6::C5(match v1 {
        Adt5::C0(h1667) => {
            h1667
        }
    })), List::<(Str, Adt6)>::nil()))))
}

pub fn f433(p1671: List<Adt138>, p1672: List<List<u64>>) -> R<Option<List<List<u64>>>> {
    let m1670 = (p1671, p1672);
    {
        let mut m1670 = m1670;
        loop {
            let step = {
                let (v0, v1) = m1670;
                let m1668 = v0;
                match m1668.uncons() {
                    None => {
                        Err::<(List<Adt138>, List<List<u64>>), R<Option<List<List<u64>>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m1669 = f483(v2)?;
                        match m1669 {
                            None => {
                                Err::<(List<Adt138>, List<List<u64>>), R<Option<List<List<u64>>>>>(Ok(None::<List<List<u64>>>))
                            }
                            Some(v4) => {
                                Ok::<(List<Adt138>, List<List<u64>>), R<Option<List<List<u64>>>>>((v3, List::cons(v4, v1)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1670 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f434(v0: Adt139) -> u64 {
    let m1673 = v0;
    match m1673 {
        Adt139::C0(_) => {
            f204()
        }
        Adt139::C1(_, _) => {
            f204()
        }
        Adt139::C2(_) => {
            f204()
        }
        Adt139::C3(_) => {
            f484()
        }
        Adt139::C4(v6) => {
            f485(v6)
        }
    }
}

pub fn f435(v0: Adt139) -> R<Adt82> {
    Ok(Adt82::C0(f486(v0.clone()), f487(v0.clone()), f488(v0)?))
}

pub fn f436(v0: List<u64>) -> R<bool> {
    let m1674 = f354(59u64, v0, List::<u64>::nil(), List::<List<u64>>::nil());
    match m1674.uncons() {
        None => {
            Ok(false)
        }
        Some((v1, v2)) => {
            if f489(v2)? {
                Ok(false)
            } else {
                f258(f260(v1), f490())
            }
        }
    }
}

pub fn f437(v0: Adt0, v1: Fn2, v2: u64, v3: Adt147) -> R<Result<Adt136, Adt139>> {
    f491(v0.clone(), v1, v2, v3.clone(), f492(match v0 {
        Adt0::C0(_, _, h1675, _, _, _, _) => {
            h1675
        }
    }, match v3 {
        Adt147::C0(_, h1676, _, _) => {
            h1676
        }
    }))
}

pub fn f438(v0: Adt149, v1: Adt150) -> Result<Adt147, Adt139> {
    let m1677 = v1;
    match m1677 {
        Adt150::C0(v3) => {
            f493(v3, v0)
        }
        Adt150::C1(v2) => {
            Err::<Adt147, Adt139>(v2)
        }
    }
}

pub fn f439(v0: Str) -> Adt149 {
    let a1678 = v0.clone();
    let a1679 = Str::lit("server/discover");
    if equal(a1678, a1679) {
        Adt149::C1(Adt148::C0)
    } else {
        let a1680 = v0.clone();
        let a1681 = Str::lit("tools/list");
        if equal(a1680, a1681) {
            Adt149::C1(Adt148::C1)
        } else {
            let a1682 = v0.clone();
            let a1683 = Str::lit("tools/call");
            if equal(a1682, a1683) {
                Adt149::C1(Adt148::C2)
            } else {
                let a1684 = v0.clone();
                let a1685 = Str::lit("resources/list");
                if equal(a1684, a1685) {
                    Adt149::C1(Adt148::C3)
                } else {
                    let a1686 = v0.clone();
                    let a1687 = Str::lit("resources/read");
                    if equal(a1686, a1687) {
                        Adt149::C1(Adt148::C4)
                    } else {
                        let a1688 = v0.clone();
                        let a1689 = Str::lit("resources/templates/list");
                        if equal(a1688, a1689) {
                            Adt149::C1(Adt148::C5)
                        } else {
                            let a1690 = v0.clone();
                            let a1691 = Str::lit("prompts/list");
                            if equal(a1690, a1691) {
                                Adt149::C1(Adt148::C6)
                            } else {
                                let a1692 = v0.clone();
                                let a1693 = Str::lit("prompts/get");
                                if equal(a1692, a1693) {
                                    Adt149::C1(Adt148::C7)
                                } else {
                                    let a1694 = v0.clone();
                                    let a1695 = Str::lit("completion/complete");
                                    if equal(a1694, a1695) {
                                        Adt149::C1(Adt148::C8)
                                    } else {
                                        let a1696 = v0.clone();
                                        let a1697 = Str::lit("tasks/get");
                                        if equal(a1696, a1697) {
                                            Adt149::C1(Adt148::C9)
                                        } else {
                                            let a1698 = v0.clone();
                                            let a1699 = Str::lit("tasks/update");
                                            if equal(a1698, a1699) {
                                                Adt149::C1(Adt148::C10)
                                            } else {
                                                let a1700 = v0;
                                                let a1701 = Str::lit("tasks/cancel");
                                                if equal(a1700, a1701) {
                                                    Adt149::C1(Adt148::C11)
                                                } else {
                                                    Adt149::C0
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f440(v0: Option<Adt126>, v1: List<Str>, v2: Adt116) -> R<Adt150> {
    f494(v0, v1, v2.clone(), f495(match v2 {
        Adt116::C0(_, _, h1702) => {
            h1702
        }
    }))
}

pub fn f441(v0: List<u64>) -> Option<Str> {
    let a1703 = f94(f95(), v0);
    utf8_decode(a1703)
}

pub fn f442(v0: Str) -> Adt152 {
    let m1704 = f439(v0);
    match m1704 {
        Adt149::C0 => {
            Adt152::C0
        }
        Adt149::C1(v1) => {
            f496(v1)
        }
    }
}

pub fn f443(v0: List<u64>) -> Option<List<u64>> {
    let m1705 = f497(f498(), v0);
    match m1705 {
        None => {
            None::<List<u64>>
        }
        Some(v1) => {
            let m1706 = f497(f499(), f268(v1, List::<u64>::nil()));
            match m1706 {
                None => {
                    None::<List<u64>>
                }
                Some(v2) => {
                    Some(f268(v2, List::<u64>::nil()))
                }
            }
        }
    }
}

pub fn f444(p1709: List<Adt129>, p1710: List<Adt129>) -> List<Adt129> {
    let m1708 = (p1709, p1710);
    {
        let mut m1708 = m1708;
        loop {
            let step = {
                let (v0, v1) = m1708;
                let m1707 = v0;
                match m1707.uncons() {
                    None => {
                        Err::<(List<Adt129>, List<Adt129>), List<Adt129>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt129>, List<Adt129>), List<Adt129>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1708 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f445(v0: List<Adt92>, p1717: List<Adt12>, v2: Option<Str>, p1718: List<Adt129>) -> R<List<Adt129>> {
    let m1716 = (p1717, p1718);
    {
        let mut m1716 = m1716;
        loop {
            let step = {
                let (v1, v3) = m1716;
                let m1711 = v1;
                match m1711.uncons() {
                    None => {
                        Err::<(List<Adt12>, List<Adt129>), R<List<Adt129>>>(Ok(v3))
                    }
                    Some((v4, v5)) => {
                        if f500(v4.clone(), v2.clone()) {
                            Ok::<(List<Adt12>, List<Adt129>), R<List<Adt129>>>((v5, List::cons(Adt129::C0(match v4.clone() {
                                Adt12::C0(_, h1712, _, _) => {
                                    h1712
                                }
                            }, match v4.clone() {
                                Adt12::C0(_, _, h1713, _) => {
                                    h1713
                                }
                            }, match v4.clone() {
                                Adt12::C0(_, _, _, h1714) => {
                                    h1714
                                }
                            }, f501(v0.clone(), match v4 {
                                Adt12::C0(_, h1715, _, _) => {
                                    h1715
                                }
                            })?), v3)))
                        } else {
                            Ok::<(List<Adt12>, List<Adt129>), R<List<Adt129>>>((v5, v3))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1716 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f446(p1734: u64, p1735: i64, p1736: i64) -> R<Option<i64>> {
    let m1733 = (p1734, (p1735, p1736));
    {
        let mut m1733 = m1733;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1733;
                let m1719 = v0;
                if m1719 == 0 {
                    Err::<(u64, (i64, i64)), R<Option<i64>>>(Ok(None::<i64>))
                } else {
                    let v3 = m1719 - 1;
                    if f502(v2) {
                        Err::<(u64, (i64, i64)), R<Option<i64>>>(Ok(Some(v1)))
                    } else {
                        let m1722 = {
                            let a1720 = v2;
                            let a1721 = 0i64;
                            compare(a1720, a1721)
                        };
                        if match m1722 {
                            Ordering::Less => {
                                true
                            }
                            Ordering::Equal => {
                                false
                            }
                            Ordering::Greater => {
                                false
                            }
                        } {
                            if f502({
                                let a1723 = v1;
                                let a1724 = 10i64;
                                let a1725 = 0i64;
                                int_rem(a1723, a1724, a1725)
                            }) {
                                Ok::<(u64, (i64, i64)), R<Option<i64>>>((v3, ({
                                    let a1726 = v1;
                                    let a1727 = 10i64;
                                    let a1728 = 0i64;
                                    int_quot(a1726, a1727, a1728)?
                                }, f193(v2, 1i64)?)))
                            } else {
                                Err::<(u64, (i64, i64)), R<Option<i64>>>(Ok(None::<i64>))
                            }
                        } else {
                            Ok::<(u64, (i64, i64)), R<Option<i64>>>((v3, ({
                                let a1729 = v1;
                                let a1730 = 10i64;
                                int_mul(a1729, a1730)?
                            }, {
                                let a1731 = v2;
                                let a1732 = 1i64;
                                int_sub(a1731, a1732)?
                            })))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1733 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f447(v0: i64) -> Option<i64> {
    let m1739 = {
        let a1737 = 9007199254740991i64;
        let a1738 = v0;
        compare(a1737, a1738)
    };
    if if match m1739 {
        Ordering::Less => {
            true
        }
        Ordering::Equal => {
            false
        }
        Ordering::Greater => {
            false
        }
    } {
        true
    } else {
        let m1742 = {
            let a1740 = v0;
            let a1741 = -9007199254740991i64;
            compare(a1740, a1741)
        };
        match m1742 {
            Ordering::Less => {
                true
            }
            Ordering::Equal => {
                false
            }
            Ordering::Greater => {
                false
            }
        }
    } {
        None::<i64>
    } else {
        Some(v0)
    }
}

pub fn f448(v0: Adt114) -> bool {
    let m1743 = v0;
    match m1743 {
        Adt114::C0 => {
            false
        }
        Adt114::C1(_) => {
            true
        }
    }
}

pub fn f449(v0: Adt104, v1: Str, v2: Adt5) -> Result<Adt116, Adt108> {
    let m1744 = f184(Str::lit("params"), v2);
    match m1744 {
        Adt114::C0 => {
            Ok::<Adt116, Adt108>(Adt116::C0(v0, v1, None::<Adt5>))
        }
        Adt114::C1(v3) => {
            let m1745 = f195(v3);
            match m1745 {
                None => {
                    f257(Some(v0), Adt109::C6)
                }
                Some(v4) => {
                    Ok::<Adt116, Adt108>(Adt116::C0(v0, v1, Some(v4)))
                }
            }
        }
    }
}

pub fn f450(v0: u64, v1: Adt6) -> bool {
    let a1746 = v0;
    let a1747 = 0u64;
    if nat_eq(a1746, a1747) {
        f503(v1)
    } else {
        let a1748 = v0;
        let a1749 = 1u64;
        if nat_eq(a1748, a1749) {
            f504(v1)
        } else {
            let a1750 = v0;
            let a1751 = 2u64;
            if nat_eq(a1750, a1751) {
                f505(v1)
            } else {
                let a1752 = v0;
                let a1753 = 3u64;
                if nat_eq(a1752, a1753) {
                    f506(v1)
                } else {
                    let a1754 = v0;
                    let a1755 = 4u64;
                    if nat_eq(a1754, a1755) {
                        f507(v1)
                    } else {
                        let a1756 = v0;
                        let a1757 = 5u64;
                        if nat_eq(a1756, a1757) {
                            f508(v1)
                        } else {
                            let a1758 = v0;
                            let a1759 = 6u64;
                            if nat_eq(a1758, a1759) {
                                f509(v1)
                            } else {
                                let a1760 = v0;
                                let a1761 = 7u64;
                                if nat_eq(a1760, a1761) {
                                    f510(v1)
                                } else {
                                    f511(v1)
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f451(v0: Str, v1: List<(Str, Adt6)>, v2: Str, v3: List<u64>, v4: List<Str>, v5: Adt112) -> R<Adt143> {
    let m1762 = v3.clone();
    match m1762.uncons() {
        None => {
            Ok(Adt143::C0(Adt93::C8(v0)))
        }
        Some((_, _)) => {
            if f220(v3.clone(), true) {
                let m1763 = f512(v1);
                match m1763 {
                    None => {
                        Ok(Adt143::C0(Adt93::C11(v0)))
                    }
                    Some(v8) => {
                        let v9: List<u64> = f513(v3)?;
                        if f514(v9.clone(), match v5.clone() {
                            Adt112::C0(_, h1764) => {
                                h1764
                            }
                        }) {
                            Ok(Adt143::C0(Adt93::C12(v0, v2)))
                        } else {
                            Ok(Adt143::C1(Adt112::C0(List::cons(Adt12::C0(v0, v2, f515(v4, List::<Str>::nil()), v8), match v5.clone() {
                                Adt112::C0(h1765, _) => {
                                    h1765
                                }
                            }), List::cons(v9, match v5 {
                                Adt112::C0(_, h1766) => {
                                    h1766
                                }
                            }))))
                        }
                    }
                }
            } else {
                Ok(Adt143::C0(Adt93::C9(v0)))
            }
        }
    }
}

pub fn f452(p1771: List<(Str, Adt6)>, v1: bool, v2: List<Str>, p1772: List<Adt105>) -> List<Adt105> {
    let m1770 = (p1771, p1772);
    {
        let mut m1770 = m1770;
        loop {
            let step = {
                let (v0, v3) = m1770;
                let m1767 = v0;
                match m1767.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Adt105>), List<Adt105>>(v3)
                    }
                    Some((v4, v5)) => {
                        Ok::<(List<(Str, Adt6)>, List<Adt105>), List<Adt105>>((v5, List::cons(Adt105::C0({
                            let (_, h1768) = v4;
                            h1768
                        }, v1, List::cons({
                            let (h1769, _) = v4;
                            h1769
                        }, v2.clone())), v3)))
                    }
                }
            };
            match step {
                Ok(next) => { m1770 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f453(p1776: List<(Str, Adt6)>, p1777: List<Adt105>) -> List<Adt105> {
    let m1775 = (p1776, p1777);
    {
        let mut m1775 = m1775;
        loop {
            let step = {
                let (v0, v1) = m1775;
                let m1773 = v0;
                match m1773.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Adt105>), List<Adt105>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Adt6)>, List<Adt105>), List<Adt105>>((v3, List::cons(Adt105::C0({
                            let (_, h1774) = v2;
                            h1774
                        }, false, List::<Str>::nil()), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1775 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f454(p1780: List<Adt6>, p1781: List<Adt105>) -> List<Adt105> {
    let m1779 = (p1780, p1781);
    {
        let mut m1779 = m1779;
        loop {
            let step = {
                let (v0, v1) = m1779;
                let m1778 = v0;
                match m1778.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<Adt105>), List<Adt105>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt6>, List<Adt105>), List<Adt105>>((v3, List::cons(Adt105::C0(v2, false, List::<Str>::nil()), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1779 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f455(v0: Adt6) -> Str {
    let m1782 = v0;
    match m1782 {
        Adt6::C0 => {
            Str::lit("null")
        }
        Adt6::C1(_) => {
            Str::lit("boolean")
        }
        Adt6::C2(_) => {
            Str::lit("number")
        }
        Adt6::C3(_) => {
            Str::lit("string")
        }
        Adt6::C4(_) => {
            Str::lit("array")
        }
        Adt6::C5(_) => {
            Str::lit("object")
        }
    }
}

pub fn f456(v0: Adt7) -> R<bool> {
    if f502(match v0.clone() {
        Adt7::C0(h1783, _) => {
            h1783
        }
    }) {
        Ok(true)
    } else {
        f516(40u64, match v0.clone() {
            Adt7::C0(h1784, _) => {
                h1784
            }
        }, match v0 {
            Adt7::C0(_, h1785) => {
                h1785
            }
        })
    }
}

pub fn f457(v0: Adt6) -> Str {
    let m1786 = f130(v0);
    match m1786 {
        None => {
            Str::lit("")
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f458(v0: Adt145) -> bool {
    let m1787 = v0;
    match m1787 {
        Adt145::C0 => {
            true
        }
        Adt145::C1 => {
            false
        }
        Adt145::C2 => {
            false
        }
    }
}

pub fn f459(v0: Adt6) -> Option<i64> {
    let m1788 = v0;
    match m1788 {
        Adt6::C0 => {
            None::<i64>
        }
        Adt6::C1(_) => {
            None::<i64>
        }
        Adt6::C2(v2) => {
            if f502(match v2.clone() {
                Adt7::C0(_, h1789) => {
                    h1789
                }
            }) {
                Some(match v2 {
                    Adt7::C0(h1790, _) => {
                        h1790
                    }
                })
            } else {
                None::<i64>
            }
        }
        Adt6::C3(_) => {
            None::<i64>
        }
        Adt6::C4(_) => {
            None::<i64>
        }
        Adt6::C5(_) => {
            None::<i64>
        }
    }
}

pub fn f460(v0: i64, v1: i64) -> Adt145 {
    let m1793 = {
        let a1791 = v0;
        let a1792 = v1;
        compare(a1791, a1792)
    };
    if match m1793 {
        Ordering::Less => {
            true
        }
        Ordering::Equal => {
            false
        }
        Ordering::Greater => {
            false
        }
    } {
        Adt145::C0
    } else {
        let m1796 = {
            let a1794 = v1;
            let a1795 = v0;
            compare(a1794, a1795)
        };
        if match m1796 {
            Ordering::Less => {
                true
            }
            Ordering::Equal => {
                false
            }
            Ordering::Greater => {
                false
            }
        } {
            Adt145::C2
        } else {
            Adt145::C1
        }
    }
}

pub fn f461(p1805: List<u64>, p1806: u64) -> R<u64> {
    let m1804 = (p1805, p1806);
    {
        let mut m1804 = m1804;
        loop {
            let step = {
                let (v0, v1) = m1804;
                let m1797 = v0;
                match m1797.uncons() {
                    None => {
                        Err::<(List<u64>, u64), R<u64>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        let a1798 = v2;
                        let a1799 = 128u64;
                        if if nat_lt(a1798, a1799) {
                            true
                        } else {
                            let a1800 = 191u64;
                            let a1801 = v2;
                            nat_lt(a1800, a1801)
                        } {
                            Ok::<(List<u64>, u64), R<u64>>((v3, {
                                let a1802 = v1;
                                let a1803 = 1u64;
                                nat_add(a1802, a1803)?
                            }))
                        } else {
                            Ok::<(List<u64>, u64), R<u64>>((v3, v1))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1804 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f462(v0: Str, p1824: List<(Str, Adt6)>) -> Option<Adt6> {
    let m1823 = p1824;
    {
        let mut m1823 = m1823;
        loop {
            let step = {
                let v1 = m1823;
                let m1807 = v1;
                match m1807.uncons() {
                    None => {
                        Err::<List<(Str, Adt6)>, Option<Adt6>>(None::<Adt6>)
                    }
                    Some((v2, v3)) => {
                        let a1811 = v0.clone();
                        let a1812 = {
                            let a1809 = List::cons(Str::lit("#/$defs/"), List::cons({
                                let (h1808, _) = v2.clone();
                                h1808
                            }, List::<Str>::nil()));
                            let a1810 = Str::lit("");
                            join(a1809, a1810)
                        };
                        if equal(a1811, a1812) {
                            Err::<List<(Str, Adt6)>, Option<Adt6>>(Some({
                                let (_, h1813) = v2;
                                h1813
                            }))
                        } else {
                            let m1815 = f195({
                                let (_, h1814) = v2.clone();
                                h1814
                            });
                            match m1815 {
                                None => {
                                    Ok::<List<(Str, Adt6)>, Option<Adt6>>(v3)
                                }
                                Some(v4) => {
                                    let m1816 = f184(Str::lit("$anchor"), v4);
                                    match m1816 {
                                        Adt114::C0 => {
                                            Ok::<List<(Str, Adt6)>, Option<Adt6>>(v3)
                                        }
                                        Adt114::C1(v5) => {
                                            let m1817 = f130(v5);
                                            match m1817 {
                                                None => {
                                                    Ok::<List<(Str, Adt6)>, Option<Adt6>>(v3)
                                                }
                                                Some(v6) => {
                                                    let a1820 = v0.clone();
                                                    let a1821 = {
                                                        let a1818 = List::cons(Str::lit("#"), List::cons(v6, List::<Str>::nil()));
                                                        let a1819 = Str::lit("");
                                                        join(a1818, a1819)
                                                    };
                                                    if equal(a1820, a1821) {
                                                        Err::<List<(Str, Adt6)>, Option<Adt6>>(Some({
                                                            let (_, h1822) = v2;
                                                            h1822
                                                        }))
                                                    } else {
                                                        Ok::<List<(Str, Adt6)>, Option<Adt6>>(v3)
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1823 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f463(v0: u64) -> R<Option<u64>> {
    let a1825 = 64u64;
    let a1826 = v0;
    if if nat_lt(a1825, a1826) {
        let a1827 = v0;
        let a1828 = 91u64;
        nat_lt(a1827, a1828)
    } else {
        false
    } {
        Ok(Some({
            let a1829 = v0;
            let a1830 = 65u64;
            nat_sub(a1829, a1830)
        }))
    } else {
        let a1831 = 96u64;
        let a1832 = v0;
        if if nat_lt(a1831, a1832) {
            let a1833 = v0;
            let a1834 = 123u64;
            nat_lt(a1833, a1834)
        } else {
            false
        } {
            Ok(Some({
                let a1835 = v0;
                let a1836 = 71u64;
                nat_sub(a1835, a1836)
            }))
        } else {
            let a1837 = 47u64;
            let a1838 = v0;
            if if nat_lt(a1837, a1838) {
                let a1839 = v0;
                let a1840 = 58u64;
                nat_lt(a1839, a1840)
            } else {
                false
            } {
                Ok(Some({
                    let a1841 = v0;
                    let a1842 = 4u64;
                    nat_add(a1841, a1842)?
                }))
            } else {
                let a1843 = v0;
                let a1844 = 43u64;
                if nat_eq(a1843, a1844) {
                    Ok(Some(62u64))
                } else {
                    let a1845 = v0;
                    let a1846 = 47u64;
                    if nat_eq(a1845, a1846) {
                        Ok(Some(63u64))
                    } else {
                        Ok(None::<u64>)
                    }
                }
            }
        }
    }
}

pub fn f464(v0: u64, v1: u64, v2: bool, v3: u64) -> R<Option<List<u64>>> {
    if if v2 {
        let a1847 = v3;
        let a1848 = 61u64;
        if nat_eq(a1847, a1848) {
            let a1849 = f517(v1, 16u64);
            let a1850 = 0u64;
            nat_eq(a1849, a1850)
        } else {
            false
        }
    } else {
        false
    } {
        Ok(Some(f518(v0, v1)?))
    } else {
        Ok(None::<List<u64>>)
    }
}

pub fn f465(v0: u64, v1: u64, v2: u64, v3: bool) -> R<Option<List<u64>>> {
    if if v3 {
        let a1851 = f517(v2, 4u64);
        let a1852 = 0u64;
        nat_eq(a1851, a1852)
    } else {
        false
    } {
        Ok(Some(f519(v0, v1, v2)?))
    } else {
        Ok(None::<List<u64>>)
    }
}

pub fn f466(v0: u64, v1: u64, v2: u64, v3: u64) -> R<List<u64>> {
    Ok(List::cons({
        let a1855 = {
            let a1853 = v0;
            let a1854 = 4u64;
            nat_mul(a1853, a1854)?
        };
        let a1856 = f520(v1, 16u64);
        nat_add(a1855, a1856)?
    }, List::cons({
        let a1859 = {
            let a1857 = f517(v1, 16u64);
            let a1858 = 16u64;
            nat_mul(a1857, a1858)?
        };
        let a1860 = f520(v2, 4u64);
        nat_add(a1859, a1860)?
    }, List::cons({
        let a1863 = {
            let a1861 = f517(v2, 4u64);
            let a1862 = 64u64;
            nat_mul(a1861, a1862)?
        };
        let a1864 = v3;
        nat_add(a1863, a1864)?
    }, List::<u64>::nil()))))
}

pub fn f467() -> u64 {
    512u64
}

pub fn f468(v0: Adt119, v1: u64, v2: Str) -> Adt133 {
    let m1866 = match v0.clone() {
        Adt119::C0(_, h1865, _) => {
            h1865
        }
    };
    match m1866.uncons() {
        None => {
            f336(Adt107::C2(v1))
        }
        Some((v3, v4)) => {
            let m1867 = v3;
            match m1867 {
                Adt124::C0(_) => {
                    f336(Adt107::C2(v1))
                }
                Adt124::C1(v6, _) => {
                    Adt133::C1(Adt119::C0(Adt120::C5, List::cons(Adt124::C1(v6, Some(v2)), v4), match v0 {
                        Adt119::C0(_, _, h1868) => {
                            h1868
                        }
                    }))
                }
            }
        }
    }
}

pub fn f469(v0: u64, v1: List<u64>) -> R<List<u64>> {
    let a1869 = v0;
    let a1870 = 128u64;
    if nat_lt(a1869, a1870) {
        Ok(List::cons(v0, v1))
    } else {
        let a1871 = v0;
        let a1872 = 2048u64;
        if nat_lt(a1871, a1872) {
            Ok(List::cons({
                let a1873 = 128u64;
                let a1874 = f521(v0, 64u64);
                nat_add(a1873, a1874)?
            }, List::cons({
                let a1875 = 192u64;
                let a1876 = f522(v0, 64u64);
                nat_add(a1875, a1876)?
            }, v1)))
        } else {
            let a1877 = v0;
            let a1878 = 65536u64;
            if nat_lt(a1877, a1878) {
                Ok(List::cons({
                    let a1879 = 128u64;
                    let a1880 = f521(v0, 64u64);
                    nat_add(a1879, a1880)?
                }, List::cons({
                    let a1881 = 128u64;
                    let a1882 = f521(f522(v0, 64u64), 64u64);
                    nat_add(a1881, a1882)?
                }, List::cons({
                    let a1883 = 224u64;
                    let a1884 = f522(v0, 4096u64);
                    nat_add(a1883, a1884)?
                }, v1))))
            } else {
                Ok(List::cons({
                    let a1885 = 128u64;
                    let a1886 = f521(v0, 64u64);
                    nat_add(a1885, a1886)?
                }, List::cons({
                    let a1887 = 128u64;
                    let a1888 = f521(f522(v0, 64u64), 64u64);
                    nat_add(a1887, a1888)?
                }, List::cons({
                    let a1889 = 128u64;
                    let a1890 = f521(f522(v0, 4096u64), 64u64);
                    nat_add(a1889, a1890)?
                }, List::cons({
                    let a1891 = 240u64;
                    let a1892 = f522(v0, 262144u64);
                    nat_add(a1891, a1892)?
                }, v1)))))
            }
        }
    }
}

pub fn f470() -> u64 {
    18u64
}

pub fn f471(v0: Adt145, v1: Adt145) -> Adt145 {
    let m1893 = v0;
    match m1893 {
        Adt145::C0 => {
            f523(v1)
        }
        Adt145::C1 => {
            Adt145::C1
        }
        Adt145::C2 => {
            v1
        }
    }
}

pub fn f472(v0: i64, v1: i64, v2: i64, v3: i64) -> R<Adt145> {
    let m1894 = f460(f524(v0, v1)?, f524(v2, v3)?);
    match m1894 {
        Adt145::C0 => {
            Ok(Adt145::C0)
        }
        Adt145::C1 => {
            f525(v0, f526(v0)?, v2, f526(v2)?)
        }
        Adt145::C2 => {
            Ok(Adt145::C2)
        }
    }
}

pub fn f473(v0: i64) -> R<i64> {
    let m1897 = {
        let a1895 = v0;
        let a1896 = 0i64;
        compare(a1895, a1896)
    };
    if match m1897 {
        Ordering::Less => {
            true
        }
        Ordering::Equal => {
            false
        }
        Ordering::Greater => {
            false
        }
    } {
        let a1898 = v0;
        int_neg(a1898)
    } else {
        Ok(v0)
    }
}

pub fn f474(v0: u64) -> List<u64> {
    f413(v0)
}

pub fn f475(v0: Adt7) -> R<bool> {
    let a1916 = {
        let m1902 = {
            let a1900 = f527();
            let a1901 = match v0.clone() {
                Adt7::C0(h1899, _) => {
                    h1899
                }
            };
            compare(a1900, a1901)
        };
        if match m1902 {
            Ordering::Less => {
                true
            }
            Ordering::Equal => {
                false
            }
            Ordering::Greater => {
                false
            }
        } {
            true
        } else {
            let m1907 = {
                let a1905 = match v0.clone() {
                    Adt7::C0(h1903, _) => {
                        h1903
                    }
                };
                let a1906 = {
                    let a1904 = f527();
                    int_neg(a1904)?
                };
                compare(a1905, a1906)
            };
            if match m1907 {
                Ordering::Less => {
                    true
                }
                Ordering::Equal => {
                    false
                }
                Ordering::Greater => {
                    false
                }
            } {
                true
            } else {
                let m1911 = {
                    let a1909 = 9999i64;
                    let a1910 = match v0.clone() {
                        Adt7::C0(_, h1908) => {
                            h1908
                        }
                    };
                    compare(a1909, a1910)
                };
                if match m1911 {
                    Ordering::Less => {
                        true
                    }
                    Ordering::Equal => {
                        false
                    }
                    Ordering::Greater => {
                        false
                    }
                } {
                    true
                } else {
                    let m1915 = {
                        let a1913 = match v0 {
                            Adt7::C0(_, h1912) => {
                                h1912
                            }
                        };
                        let a1914 = -9999i64;
                        compare(a1913, a1914)
                    };
                    match m1915 {
                        Ordering::Less => {
                            true
                        }
                        Ordering::Equal => {
                            false
                        }
                        Ordering::Greater => {
                            false
                        }
                    }
                }
            }
        }
    };
    Ok(bool_not(a1916))
}

pub fn f476(v0: Adt7) -> R<List<u64>> {
    let m1920 = {
        let a1918 = match v0.clone() {
            Adt7::C0(_, h1917) => {
                h1917
            }
        };
        let a1919 = 0i64;
        compare(a1918, a1919)
    };
    if if match m1920 {
        Ordering::Less => {
            true
        }
        Ordering::Equal => {
            false
        }
        Ordering::Greater => {
            false
        }
    } {
        true
    } else {
        let m1924 = {
            let a1922 = 0i64;
            let a1923 = match v0.clone() {
                Adt7::C0(_, h1921) => {
                    h1921
                }
            };
            compare(a1922, a1923)
        };
        match m1924 {
            Ordering::Less => {
                true
            }
            Ordering::Equal => {
                false
            }
            Ordering::Greater => {
                false
            }
        }
    } {
        let a1927 = f528(match v0.clone() {
            Adt7::C0(h1925, _) => {
                h1925
            }
        })?;
        let a1928 = List::cons(101u64, f528(match v0 {
            Adt7::C0(_, h1926) => {
                h1926
            }
        })?);
        Ok(append_list(a1927, a1928))
    } else {
        f528(match v0 {
            Adt7::C0(h1929, _) => {
                h1929
            }
        })
    }
}

pub fn f477(v0: u64, v1: Adt125, v2: List<Adt125>) -> List<Adt125> {
    List::cons(Adt125::C0(List::cons(v0, List::<u64>::nil())), List::cons(v1, v2))
}

pub fn f478(p1932: List<u64>, p1933: List<u64>) -> R<List<u64>> {
    let m1931 = (p1932, p1933);
    {
        let mut m1931 = m1931;
        loop {
            let step = {
                let (v0, v1) = m1931;
                let m1930 = v0;
                match m1930.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), R<List<u64>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), R<List<u64>>>((v3, f286(f529(v2)?, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1931 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f479() -> u64 {
    4294967296u64
}

pub fn f480(v0: Adt34) -> R<Option<List<(Str, Adt6)>>> {
    let m1934 = v0;
    match m1934 {
        Adt34::C0(v1) => {
            Ok(Some(f530(v1)?))
        }
        Adt34::C1(v2) => {
            Ok(Some(f531(v2)?))
        }
        Adt34::C2(v3) => {
            f532(f533(v3)?)
        }
        Adt34::C3(v4) => {
            f532(f533(match v4 {
                Adt51::C0(h1935, _) => {
                    h1935
                }
            })?)
        }
        Adt34::C4(v5) => {
            Ok(Some(f534(v5)?))
        }
        Adt34::C5(v6) => {
            Ok(Some(f535(v6)?))
        }
        Adt34::C6(v7) => {
            Ok(Some(f536(v7)?))
        }
        Adt34::C7(v8) => {
            Ok(Some(f537(v8)?))
        }
        Adt34::C8(v9) => {
            f532(f538(v9)?)
        }
        Adt34::C9(v10) => {
            Ok(Some(f539(v10)?))
        }
        Adt34::C10(_) => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Adt34::C11 => {
            Ok(Some(List::<(Str, Adt6)>::nil()))
        }
        Adt34::C12(v12) => {
            f532(f540(v12)?)
        }
        Adt34::C13(_) => {
            Ok(None::<List<(Str, Adt6)>>)
        }
    }
}

pub fn f481(v0: Adt136) -> Adt5 {
    Adt5::C0(List::cons((Str::lit("resultType"), Adt6::C3(f541(f542(match v0.clone() {
        Adt136::C0(h1936, _, _) => {
            h1936
        }
    })))), List::cons((Str::lit("_meta"), Adt6::C5(f543({
        let m1939 = match v0 {
            Adt136::C0(_, h1938, _) => {
                h1938
            }
        };
        match m1939 {
            Adt137::C0(h1937, _) => {
                h1937
            }
        }
    }))), List::<(Str, Adt6)>::nil())))
}

pub fn f482(v0: bool, v1: Adt95, v2: List<List<u64>>) -> Adt95 {
    if f544(v0, match v1.clone() {
        Adt95::C0(h1940, _, _, _) => {
            h1940
        }
    }, v2.clone()) {
        f545(match v1.clone() {
            Adt95::C0(h1941, _, _, _) => {
                h1941
            }
        }, {
            let a1943 = v2;
            let a1944 = List::cons(match v1 {
                Adt95::C0(_, _, _, h1942) => {
                    h1942
                }
            }, List::<List<u64>>::nil());
            append_list(a1943, a1944)
        })
    } else {
        v1
    }
}

pub fn f483(v0: Adt138) -> R<Option<List<u64>>> {
    let m1945 = f546(f547(v0))?;
    match m1945 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v1) => {
            let m1946 = f197(v1)?;
            match m1946 {
                Ok(v3) => {
                    Ok(Some(v3))
                }
                Err(_) => {
                    Ok(None::<List<u64>>)
                }
            }
        }
    }
}

pub fn f484() -> u64 {
    404u64
}

pub fn f485(v0: Adt86) -> u64 {
    let m1947 = v0;
    match m1947 {
        Adt86::C0(_) => {
            f204()
        }
        Adt86::C1(_) => {
            f204()
        }
        Adt86::C2(_) => {
            f204()
        }
        Adt86::C3(_) => {
            f204()
        }
        Adt86::C4(_) => {
            f204()
        }
        Adt86::C5(_) => {
            f204()
        }
        Adt86::C6(_) => {
            f68()
        }
        Adt86::C7(_) => {
            f548()
        }
        Adt86::C8(_) => {
            f204()
        }
        Adt86::C9(_) => {
            f68()
        }
        Adt86::C10 => {
            f204()
        }
        Adt86::C11 => {
            f204()
        }
        Adt86::C12(_) => {
            f68()
        }
        Adt86::C13 => {
            f68()
        }
        Adt86::C14 => {
            f68()
        }
        Adt86::C15(_) => {
            f68()
        }
    }
}

pub fn f486(v0: Adt139) -> i64 {
    let m1948 = v0;
    match m1948 {
        Adt139::C0(_) => {
            f549()
        }
        Adt139::C1(_, _) => {
            f550()
        }
        Adt139::C2(_) => {
            f551()
        }
        Adt139::C3(_) => {
            f552()
        }
        Adt139::C4(v6) => {
            f553(v6)
        }
    }
}

pub fn f487(v0: Adt139) -> Str {
    let m1949 = v0;
    match m1949 {
        Adt139::C0(v1) => {
            f554(v1)
        }
        Adt139::C1(_, _) => {
            Str::lit("Unsupported protocol version")
        }
        Adt139::C2(v4) => {
            f555(v4)
        }
        Adt139::C3(_) => {
            Str::lit("Method not found")
        }
        Adt139::C4(v6) => {
            let m1950 = v6;
            match m1950 {
                Adt86::C0(_) => {
                    Str::lit("Invalid params: unknown tool")
                }
                Adt86::C1(_) => {
                    Str::lit("Invalid params: unknown prompt")
                }
                Adt86::C2(_) => {
                    Str::lit("Invalid params: resource not found")
                }
                Adt86::C3(_) => {
                    Str::lit("Invalid params: unknown task")
                }
                Adt86::C4(_) => {
                    Str::lit("Invalid params: invalid arguments")
                }
                Adt86::C5(_) => {
                    Str::lit("Invalid params: invalid cursor")
                }
                Adt86::C6(_) => {
                    Str::lit("Internal error")
                }
                Adt86::C7(_) => {
                    Str::lit("Rate limit exceeded: too many tool calls")
                }
                Adt86::C8(_) => {
                    Str::lit("Missing required client capability")
                }
                Adt86::C9(_) => {
                    Str::lit("Internal error")
                }
                Adt86::C10 => {
                    Str::lit("Invalid params: requestState failed verification")
                }
                Adt86::C11 => {
                    Str::lit("Invalid params: inputResponses is not a valid InputResponses object")
                }
                Adt86::C12(_) => {
                    Str::lit("Internal error")
                }
                Adt86::C13 => {
                    Str::lit("Internal error")
                }
                Adt86::C14 => {
                    Str::lit("Internal error")
                }
                Adt86::C15(_) => {
                    Str::lit("Internal error")
                }
            }
        }
    }
}

pub fn f488(v0: Adt139) -> R<Option<Adt6>> {
    let m1951 = v0;
    match m1951 {
        Adt139::C0(_) => {
            Ok(None::<Adt6>)
        }
        Adt139::C1(v2, v3) => {
            Ok(Some(Adt6::C5(List::cons((Str::lit("supported"), Adt6::C4(f556(v3))), List::cons((Str::lit("requested"), Adt6::C3(v2)), List::<(Str, Adt6)>::nil())))))
        }
        Adt139::C2(_) => {
            Ok(None::<Adt6>)
        }
        Adt139::C3(_) => {
            Ok(None::<Adt6>)
        }
        Adt139::C4(v6) => {
            let m1952 = v6;
            match m1952 {
                Adt86::C0(_) => {
                    Ok(None::<Adt6>)
                }
                Adt86::C1(_) => {
                    Ok(None::<Adt6>)
                }
                Adt86::C2(v9) => {
                    Ok(Some(Adt6::C5(List::cons((Str::lit("uri"), Adt6::C3(v9)), List::<(Str, Adt6)>::nil()))))
                }
                Adt86::C3(_) => {
                    Ok(None::<Adt6>)
                }
                Adt86::C4(v11) => {
                    Ok(Some(Adt6::C5(List::cons((Str::lit("reason"), Adt6::C3(v11)), List::<(Str, Adt6)>::nil()))))
                }
                Adt86::C5(_) => {
                    Ok(None::<Adt6>)
                }
                Adt86::C6(_) => {
                    Ok(None::<Adt6>)
                }
                Adt86::C7(v14) => {
                    Ok(Some(Adt6::C5(List::cons((Str::lit("limit"), Adt6::C2(Adt7::C0(f145(v14, 0i64)?, 0i64))), List::<(Str, Adt6)>::nil()))))
                }
                Adt86::C8(v15) => {
                    Ok(Some(Adt6::C5(List::cons((Str::lit("requiredCapabilities"), f557(v15)), List::<(Str, Adt6)>::nil()))))
                }
                Adt86::C9(_) => {
                    Ok(None::<Adt6>)
                }
                Adt86::C10 => {
                    Ok(None::<Adt6>)
                }
                Adt86::C11 => {
                    Ok(None::<Adt6>)
                }
                Adt86::C12(_) => {
                    Ok(None::<Adt6>)
                }
                Adt86::C13 => {
                    Ok(None::<Adt6>)
                }
                Adt86::C14 => {
                    Ok(None::<Adt6>)
                }
                Adt86::C15(_) => {
                    Ok(None::<Adt6>)
                }
            }
        }
    }
}

pub fn f489(p1955: List<List<u64>>) -> R<bool> {
    let m1954 = p1955;
    {
        let mut m1954 = m1954;
        loop {
            let step = {
                let v0 = m1954;
                let m1953 = v0;
                match m1953.uncons() {
                    None => {
                        Err::<List<List<u64>>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        if f558(f260(v1))? {
                            Err::<List<List<u64>>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<List<u64>>, R<bool>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1954 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f490() -> List<u64> {
    List::cons(116u64, List::cons(101u64, List::cons(120u64, List::cons(116u64, List::cons(47u64, List::cons(101u64, List::cons(118u64, List::cons(101u64, List::cons(110u64, List::cons(116u64, List::cons(45u64, List::cons(115u64, List::cons(116u64, List::cons(114u64, List::cons(101u64, List::cons(97u64, List::cons(109u64, List::<u64>::nil())))))))))))))))))
}

pub fn f491(v0: Adt0, v1: Fn2, v2: u64, v3: Adt147, v4: bool) -> R<Result<Adt136, Adt139>> {
    if v4 {
        f559(v0, v1, v2, f560(match v3.clone() {
            Adt147::C0(_, h1956, _, _) => {
                h1956
            }
        }, match v3.clone() {
            Adt147::C0(_, _, h1957, _) => {
                h1957
            }
        }, match v3 {
            Adt147::C0(_, _, _, h1958) => {
                h1958
            }
        }))
    } else {
        Ok(Err::<Adt136, Adt139>(Adt139::C3(f561(match v3 {
            Adt147::C0(_, h1959, _, _) => {
                h1959
            }
        }))))
    }
}

pub fn f492(v0: Adt4, v1: Adt148) -> bool {
    let m1960 = v1;
    match m1960 {
        Adt148::C0 => {
            true
        }
        Adt148::C1 => {
            f562(match v0 {
                Adt4::C0(_, _, _, _, _, h1961, _) => {
                    h1961
                }
            })
        }
        Adt148::C2 => {
            f562(match v0 {
                Adt4::C0(_, _, _, _, _, h1962, _) => {
                    h1962
                }
            })
        }
        Adt148::C3 => {
            f563(match v0 {
                Adt4::C0(_, _, _, _, h1963, _, _) => {
                    h1963
                }
            })
        }
        Adt148::C4 => {
            f563(match v0 {
                Adt4::C0(_, _, _, _, h1964, _, _) => {
                    h1964
                }
            })
        }
        Adt148::C5 => {
            f563(match v0 {
                Adt4::C0(_, _, _, _, h1965, _, _) => {
                    h1965
                }
            })
        }
        Adt148::C6 => {
            f562(match v0 {
                Adt4::C0(_, _, _, h1966, _, _, _) => {
                    h1966
                }
            })
        }
        Adt148::C7 => {
            f562(match v0 {
                Adt4::C0(_, _, _, h1967, _, _, _) => {
                    h1967
                }
            })
        }
        Adt148::C8 => {
            f564(match v0 {
                Adt4::C0(_, _, h1968, _, _, _, _) => {
                    h1968
                }
            })
        }
        Adt148::C9 => {
            f565(match v0 {
                Adt4::C0(_, _, _, _, _, _, h1969) => {
                    h1969
                }
            }, f566())
        }
        Adt148::C10 => {
            f565(match v0 {
                Adt4::C0(_, _, _, _, _, _, h1970) => {
                    h1970
                }
            }, f566())
        }
        Adt148::C11 => {
            f565(match v0 {
                Adt4::C0(_, _, _, _, _, _, h1971) => {
                    h1971
                }
            }, f566())
        }
    }
}

pub fn f493(v0: Adt151, v1: Adt149) -> Result<Adt147, Adt139> {
    let m1972 = v1;
    match m1972 {
        Adt149::C0 => {
            Err::<Adt147, Adt139>(Adt139::C3(match v0 {
                Adt151::C0(_, h1973, _, _) => {
                    h1973
                }
            }))
        }
        Adt149::C1(v2) => {
            Ok::<Adt147, Adt139>(Adt147::C0(match v0.clone() {
                Adt151::C0(h1974, _, _, _) => {
                    h1974
                }
            }, v2, match v0.clone() {
                Adt151::C0(_, _, h1975, _) => {
                    h1975
                }
            }, match v0 {
                Adt151::C0(_, _, _, h1976) => {
                    h1976
                }
            }))
        }
    }
}

pub fn f494(v0: Option<Adt126>, v1: List<Str>, v2: Adt116, v3: Result<Adt18, Adt140>) -> R<Adt150> {
    let m1977 = v3;
    match m1977 {
        Ok(v5) => {
            Ok(f567(v1, v2.clone(), v5.clone(), f568(v0, f442(match v2.clone() {
                Adt116::C0(_, h1978, _) => {
                    h1978
                }
            }), match v2.clone() {
                Adt116::C0(_, h1979, _) => {
                    h1979
                }
            }, match v5 {
                Adt18::C0(h1980, _, _, _, _, _) => {
                    h1980
                }
            }, f569(match v2 {
                Adt116::C0(_, _, h1981) => {
                    h1981
                }
            }))?))
        }
        Err(v4) => {
            Ok(Adt150::C1(Adt139::C0(v4)))
        }
    }
}

pub fn f495(v0: Option<Adt5>) -> Result<Adt18, Adt140> {
    let m1982 = v0;
    match m1982 {
        None => {
            Err::<Adt18, Adt140>(Adt140::C0)
        }
        Some(v1) => {
            let m1983 = f184(Str::lit("_meta"), v1);
            match m1983 {
                Adt114::C0 => {
                    Err::<Adt18, Adt140>(Adt140::C1)
                }
                Adt114::C1(v2) => {
                    let m1984 = f195(v2);
                    match m1984 {
                        None => {
                            Err::<Adt18, Adt140>(Adt140::C4)
                        }
                        Some(v3) => {
                            f570(v3)
                        }
                    }
                }
            }
        }
    }
}

pub fn f496(v0: Adt148) -> Adt152 {
    let m1985 = v0;
    match m1985 {
        Adt148::C0 => {
            Adt152::C0
        }
        Adt148::C1 => {
            Adt152::C0
        }
        Adt148::C2 => {
            Adt152::C1
        }
        Adt148::C3 => {
            Adt152::C0
        }
        Adt148::C4 => {
            Adt152::C2
        }
        Adt148::C5 => {
            Adt152::C0
        }
        Adt148::C6 => {
            Adt152::C0
        }
        Adt148::C7 => {
            Adt152::C1
        }
        Adt148::C8 => {
            Adt152::C0
        }
        Adt148::C9 => {
            Adt152::C3
        }
        Adt148::C10 => {
            Adt152::C3
        }
        Adt148::C11 => {
            Adt152::C3
        }
    }
}

pub fn f497(p1991: List<u64>, p1992: List<u64>) -> Option<List<u64>> {
    let m1990 = (p1991, p1992);
    {
        let mut m1990 = m1990;
        loop {
            let step = {
                let (v0, v1) = m1990;
                let m1986 = v0;
                match m1986.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), Option<List<u64>>>(Some(v1))
                    }
                    Some((v2, v3)) => {
                        let m1987 = v1;
                        match m1987.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), Option<List<u64>>>(None::<List<u64>>)
                            }
                            Some((v4, v5)) => {
                                let a1988 = v4;
                                let a1989 = v2;
                                if nat_eq(a1988, a1989) {
                                    Ok::<(List<u64>, List<u64>), Option<List<u64>>>((v3, v5))
                                } else {
                                    Err::<(List<u64>, List<u64>), Option<List<u64>>>(None::<List<u64>>)
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1990 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f498() -> List<u64> {
    List::cons(61u64, List::cons(63u64, List::cons(98u64, List::cons(97u64, List::cons(115u64, List::cons(101u64, List::cons(54u64, List::cons(52u64, List::cons(63u64, List::<u64>::nil())))))))))
}

pub fn f499() -> List<u64> {
    List::cons(61u64, List::cons(63u64, List::<u64>::nil()))
}

pub fn f500(v0: Adt12, v1: Option<Str>) -> bool {
    let m1993 = v1;
    match m1993 {
        None => {
            false
        }
        Some(v2) => {
            let a1995 = match v0 {
                Adt12::C0(h1994, _, _, _) => {
                    h1994
                }
            };
            let a1996 = v2;
            equal(a1995, a1996)
        }
    }
}

pub fn f501(v0: List<Adt92>, v1: Str) -> R<Adt130> {
    let m1999 = f361(f162(v0, {
        let a1997 = f571();
        let a1998 = f87(v1)?;
        append_list(a1997, a1998)
    })?)?;
    match m1999 {
        Adt110::C0 => {
            Ok(Adt130::C0)
        }
        Adt110::C1 => {
            Ok(Adt130::C1)
        }
        Adt110::C2 => {
            Ok(Adt130::C1)
        }
        Adt110::C3(v2) => {
            let m2000 = f441(v2);
            match m2000 {
                None => {
                    Ok(Adt130::C1)
                }
                Some(v3) => {
                    Ok(Adt130::C2(v3))
                }
            }
        }
    }
}

pub fn f502(v0: i64) -> bool {
    let a2007 = {
        let m2003 = {
            let a2001 = v0;
            let a2002 = 0i64;
            compare(a2001, a2002)
        };
        if match m2003 {
            Ordering::Less => {
                true
            }
            Ordering::Equal => {
                false
            }
            Ordering::Greater => {
                false
            }
        } {
            true
        } else {
            let m2006 = {
                let a2004 = 0i64;
                let a2005 = v0;
                compare(a2004, a2005)
            };
            match m2006 {
                Ordering::Less => {
                    true
                }
                Ordering::Equal => {
                    false
                }
                Ordering::Greater => {
                    false
                }
            }
        }
    };
    bool_not(a2007)
}

pub fn f503(v0: Adt6) -> bool {
    let m2008 = f130(v0);
    match m2008 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f504(v0: Adt6) -> bool {
    let m2009 = f341(v0);
    match m2009 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f505(v0: Adt6) -> bool {
    let m2010 = f459(v0);
    match m2010 {
        None => {
            false
        }
        Some(v1) => {
            let a2014 = {
                let m2013 = {
                    let a2011 = v1;
                    let a2012 = 0i64;
                    compare(a2011, a2012)
                };
                match m2013 {
                    Ordering::Less => {
                        true
                    }
                    Ordering::Equal => {
                        false
                    }
                    Ordering::Greater => {
                        false
                    }
                }
            };
            bool_not(a2014)
        }
    }
}

pub fn f506(v0: Adt6) -> bool {
    let m2015 = f185(v0);
    match m2015 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f507(v0: Adt6) -> bool {
    let m2016 = f195(v0);
    match m2016 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f508(v0: Adt6) -> bool {
    let m2017 = f185(v0);
    match m2017 {
        None => {
            false
        }
        Some(v1) => {
            f572(v1)
        }
    }
}

pub fn f509(v0: Adt6) -> bool {
    let m2018 = f136(v0);
    match m2018 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f510(v0: Adt6) -> bool {
    let m2019 = f130(v0.clone());
    match m2019 {
        None => {
            let m2020 = f185(v0);
            match m2020 {
                None => {
                    false
                }
                Some(v2) => {
                    f573(v2)
                }
            }
        }
        Some(v1) => {
            f381(v1)
        }
    }
}

pub fn f511(v0: Adt6) -> bool {
    let m2021 = f185(v0);
    match m2021 {
        None => {
            false
        }
        Some(v1) => {
            let m2022 = v1;
            match m2022.uncons() {
                None => {
                    false
                }
                Some((_, _)) => {
                    true
                }
            }
        }
    }
}

pub fn f512(v0: List<(Str, Adt6)>) -> Option<Adt13> {
    let m2023 = f239(Str::lit("type"), v0);
    match m2023 {
        Adt114::C0 => {
            None::<Adt13>
        }
        Adt114::C1(v1) => {
            let m2024 = f130(v1);
            match m2024 {
                None => {
                    None::<Adt13>
                }
                Some(v2) => {
                    f574(v2)
                }
            }
        }
    }
}

pub fn f513(v0: List<u64>) -> R<List<u64>> {
    Ok(f286(f575(v0, List::<u64>::nil())?, List::<u64>::nil()))
}

pub fn f514(v0: List<u64>, p2027: List<List<u64>>) -> bool {
    let m2026 = p2027;
    {
        let mut m2026 = m2026;
        loop {
            let step = {
                let v1 = m2026;
                let m2025 = v1;
                match m2025.uncons() {
                    None => {
                        Err::<List<List<u64>>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f576(v0.clone(), v2) {
                            Err::<List<List<u64>>, bool>(true)
                        } else {
                            Ok::<List<List<u64>>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2026 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f515(p2030: List<Str>, p2031: List<Str>) -> List<Str> {
    let m2029 = (p2030, p2031);
    {
        let mut m2029 = m2029;
        loop {
            let step = {
                let (v0, v1) = m2029;
                let m2028 = v0;
                match m2028.uncons() {
                    None => {
                        Err::<(List<Str>, List<Str>), List<Str>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Str>, List<Str>), List<Str>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2029 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f516(p2043: u64, p2044: i64, p2045: i64) -> R<bool> {
    let m2042 = (p2043, (p2044, p2045));
    {
        let mut m2042 = m2042;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2042;
                let m2032 = v0;
                if m2032 == 0 {
                    Err::<(u64, (i64, i64)), R<bool>>(Ok(false))
                } else {
                    let v3 = m2032 - 1;
                    let m2035 = {
                        let a2033 = v2;
                        let a2034 = 0i64;
                        compare(a2033, a2034)
                    };
                    if match m2035 {
                        Ordering::Less => {
                            true
                        }
                        Ordering::Equal => {
                            false
                        }
                        Ordering::Greater => {
                            false
                        }
                    } {
                        if f502({
                            let a2036 = v1;
                            let a2037 = 10i64;
                            let a2038 = 0i64;
                            int_rem(a2036, a2037, a2038)
                        }) {
                            Ok::<(u64, (i64, i64)), R<bool>>((v3, ({
                                let a2039 = v1;
                                let a2040 = 10i64;
                                let a2041 = 0i64;
                                int_quot(a2039, a2040, a2041)?
                            }, f193(v2, 1i64)?)))
                        } else {
                            Err::<(u64, (i64, i64)), R<bool>>(Ok(false))
                        }
                    } else {
                        Err::<(u64, (i64, i64)), R<bool>>(Ok(true))
                    }
                }
            };
            match step {
                Ok(next) => { m2042 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f517(v0: u64, v1: u64) -> u64 {
    let a2046 = v0;
    let a2047 = v1;
    let a2048 = 0u64;
    nat_rem(a2046, a2047, a2048)
}

pub fn f518(v0: u64, v1: u64) -> R<List<u64>> {
    Ok(List::cons({
        let a2051 = {
            let a2049 = v0;
            let a2050 = 4u64;
            nat_mul(a2049, a2050)?
        };
        let a2052 = f520(v1, 16u64);
        nat_add(a2051, a2052)?
    }, List::<u64>::nil()))
}

pub fn f519(v0: u64, v1: u64, v2: u64) -> R<List<u64>> {
    Ok(List::cons({
        let a2055 = {
            let a2053 = v0;
            let a2054 = 4u64;
            nat_mul(a2053, a2054)?
        };
        let a2056 = f520(v1, 16u64);
        nat_add(a2055, a2056)?
    }, List::cons({
        let a2059 = {
            let a2057 = f517(v1, 16u64);
            let a2058 = 16u64;
            nat_mul(a2057, a2058)?
        };
        let a2060 = f520(v2, 4u64);
        nat_add(a2059, a2060)?
    }, List::<u64>::nil())))
}

pub fn f520(v0: u64, v1: u64) -> u64 {
    let a2061 = v0;
    let a2062 = v1;
    let a2063 = 0u64;
    nat_quot(a2061, a2062, a2063)
}

pub fn f521(v0: u64, v1: u64) -> u64 {
    let a2064 = v0;
    let a2065 = v1;
    let a2066 = 0u64;
    nat_rem(a2064, a2065, a2066)
}

pub fn f522(v0: u64, v1: u64) -> u64 {
    let a2067 = v0;
    let a2068 = v1;
    let a2069 = 0u64;
    nat_quot(a2067, a2068, a2069)
}

pub fn f523(v0: Adt145) -> Adt145 {
    let m2070 = v0;
    match m2070 {
        Adt145::C0 => {
            Adt145::C2
        }
        Adt145::C1 => {
            Adt145::C1
        }
        Adt145::C2 => {
            Adt145::C0
        }
    }
}

pub fn f524(v0: i64, v1: i64) -> R<i64> {
    f193(f145(f526(v0)?, 0i64)?, v1)
}

pub fn f525(v0: i64, v1: u64, v2: i64, v3: u64) -> R<Adt145> {
    let a2071 = v1;
    let a2072 = v3;
    if nat_lt(a2071, a2072) {
        Ok(f460(f577({
            let a2073 = v3;
            let a2074 = v1;
            nat_sub(a2073, a2074)
        }, v0)?, v2))
    } else {
        let a2075 = v3;
        let a2076 = v1;
        if nat_lt(a2075, a2076) {
            Ok(f460(v0, f577({
                let a2077 = v1;
                let a2078 = v3;
                nat_sub(a2077, a2078)
            }, v2)?))
        } else {
            Ok(f460(v0, v2))
        }
    }
}

pub fn f526(v0: i64) -> R<u64> {
    f578(20u64, v0, 0u64)
}

pub fn f527() -> i64 {
    999999999999999999i64
}

pub fn f528(v0: i64) -> R<List<u64>> {
    let m2081 = {
        let a2079 = v0;
        let a2080 = 0i64;
        compare(a2079, a2080)
    };
    if match m2081 {
        Ordering::Less => {
            true
        }
        Ordering::Equal => {
            false
        }
        Ordering::Greater => {
            false
        }
    } {
        Ok(List::cons(45u64, f579(20u64, {
            let a2082 = v0;
            int_neg(a2082)?
        }, List::<u64>::nil())?))
    } else {
        f579(20u64, v0, List::<u64>::nil())
    }
}

pub fn f529(v0: u64) -> R<List<u64>> {
    let a2083 = v0;
    let a2084 = 34u64;
    if nat_eq(a2083, a2084) {
        Ok(List::cons(92u64, List::cons(34u64, List::<u64>::nil())))
    } else {
        let a2085 = v0;
        let a2086 = 92u64;
        if nat_eq(a2085, a2086) {
            Ok(List::cons(92u64, List::cons(92u64, List::<u64>::nil())))
        } else {
            let a2087 = v0;
            let a2088 = 8u64;
            if nat_eq(a2087, a2088) {
                Ok(List::cons(92u64, List::cons(98u64, List::<u64>::nil())))
            } else {
                let a2089 = v0;
                let a2090 = 9u64;
                if nat_eq(a2089, a2090) {
                    Ok(List::cons(92u64, List::cons(116u64, List::<u64>::nil())))
                } else {
                    let a2091 = v0;
                    let a2092 = 10u64;
                    if nat_eq(a2091, a2092) {
                        Ok(List::cons(92u64, List::cons(110u64, List::<u64>::nil())))
                    } else {
                        let a2093 = v0;
                        let a2094 = 12u64;
                        if nat_eq(a2093, a2094) {
                            Ok(List::cons(92u64, List::cons(102u64, List::<u64>::nil())))
                        } else {
                            let a2095 = v0;
                            let a2096 = 13u64;
                            if nat_eq(a2095, a2096) {
                                Ok(List::cons(92u64, List::cons(114u64, List::<u64>::nil())))
                            } else {
                                let a2097 = v0;
                                let a2098 = 32u64;
                                if nat_lt(a2097, a2098) {
                                    Ok(List::cons(92u64, List::cons(117u64, List::cons(48u64, List::cons(48u64, List::cons(f580(f522(v0, 16u64))?, List::cons(f580(f521(v0, 16u64))?, List::<u64>::nil())))))))
                                } else {
                                    Ok(List::cons(v0, List::<u64>::nil()))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f530(v0: Adt35) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("supportedVersions"), f581(match v0.clone() {
        Adt35::C0(_, h2099, _, _) => {
            h2099
        }
    })), List::cons((Str::lit("capabilities"), f582(match v0.clone() {
        Adt35::C0(_, _, h2100, _) => {
            h2100
        }
    })), {
        let a2103 = f583(Str::lit("instructions"), match v0.clone() {
            Adt35::C0(_, _, _, h2101) => {
                h2101
            }
        });
        let a2104 = f584(match v0 {
            Adt35::C0(h2102, _, _, _) => {
                h2102
            }
        })?;
        append_list(a2103, a2104)
    })))
}

pub fn f531(v0: Adt36) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("tools"), Adt6::C4(f585(match v0.clone() {
        Adt36::C0(_, _, h2105) => {
            h2105
        }
    }))), {
        let a2108 = f583(Str::lit("nextCursor"), match v0.clone() {
            Adt36::C0(_, h2106, _) => {
                h2106
            }
        });
        let a2109 = f584(match v0 {
            Adt36::C0(h2107, _, _) => {
                h2107
            }
        })?;
        append_list(a2108, a2109)
    }))
}

pub fn f532(v0: Option<List<(Str, Adt6)>>) -> R<Option<List<(Str, Adt6)>>> {
    let m2110 = v0;
    match m2110 {
        None => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Some(v1) => {
            let m2111 = f546(Adt6::C5(v1))?;
            match m2111 {
                None => {
                    Ok(None::<List<(Str, Adt6)>>)
                }
                Some(v2) => {
                    let m2112 = f195(v2);
                    match m2112 {
                        None => {
                            Ok(None::<List<(Str, Adt6)>>)
                        }
                        Some(v3) => {
                            Ok(Some(match v3 {
                                Adt5::C0(h2113) => {
                                    h2113
                                }
                            }))
                        }
                    }
                }
            }
        }
    }
}

pub fn f533(v0: Adt39) -> R<Option<List<(Str, Adt6)>>> {
    let m2115 = f586(match v0.clone() {
        Adt39::C0(h2114, _, _) => {
            h2114
        }
    })?;
    match m2115 {
        None => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Some(v1) => {
            Ok(Some(List::cons((Str::lit("content"), Adt6::C4(v1)), {
                let a2118 = f587(Str::lit("structuredContent"), match v0.clone() {
                    Adt39::C0(_, h2116, _) => {
                        h2116
                    }
                });
                let a2119 = f588(Str::lit("isError"), match v0 {
                    Adt39::C0(_, _, h2117) => {
                        h2117
                    }
                });
                append_list(a2118, a2119)
            })))
        }
    }
}

pub fn f534(v0: Adt53) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("resources"), Adt6::C4(f589(match v0.clone() {
        Adt53::C0(_, _, h2120) => {
            h2120
        }
    })?)), {
        let a2123 = f583(Str::lit("nextCursor"), match v0.clone() {
            Adt53::C0(_, h2121, _) => {
                h2121
            }
        });
        let a2124 = f584(match v0 {
            Adt53::C0(h2122, _, _) => {
                h2122
            }
        })?;
        append_list(a2123, a2124)
    }))
}

pub fn f535(v0: Adt54) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("contents"), Adt6::C4(f590(match v0.clone() {
        Adt54::C0(_, h2125) => {
            h2125
        }
    }))), f584(match v0 {
        Adt54::C0(h2126, _) => {
            h2126
        }
    })?))
}

pub fn f536(v0: Adt55) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("resourceTemplates"), Adt6::C4(f591(match v0.clone() {
        Adt55::C0(_, _, h2127) => {
            h2127
        }
    }))), {
        let a2130 = f583(Str::lit("nextCursor"), match v0.clone() {
            Adt55::C0(_, h2128, _) => {
                h2128
            }
        });
        let a2131 = f584(match v0 {
            Adt55::C0(h2129, _, _) => {
                h2129
            }
        })?;
        append_list(a2130, a2131)
    }))
}

pub fn f537(v0: Adt57) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("prompts"), Adt6::C4(f592(match v0.clone() {
        Adt57::C0(_, _, h2132) => {
            h2132
        }
    }))), {
        let a2135 = f583(Str::lit("nextCursor"), match v0.clone() {
            Adt57::C0(_, h2133, _) => {
                h2133
            }
        });
        let a2136 = f584(match v0 {
            Adt57::C0(h2134, _, _) => {
                h2134
            }
        })?;
        append_list(a2135, a2136)
    }))
}

pub fn f538(v0: Adt60) -> R<Option<List<(Str, Adt6)>>> {
    let m2138 = f593(match v0.clone() {
        Adt60::C0(_, h2137) => {
            h2137
        }
    })?;
    match m2138 {
        None => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Some(v1) => {
            Ok(Some({
                let a2140 = f583(Str::lit("description"), match v0 {
                    Adt60::C0(h2139, _) => {
                        h2139
                    }
                });
                let a2141 = List::cons((Str::lit("messages"), Adt6::C4(v1)), List::<(Str, Adt6)>::nil());
                append_list(a2140, a2141)
            }))
        }
    }
}

pub fn f539(v0: Adt62) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("completion"), Adt6::C5(List::cons((Str::lit("values"), Adt6::C4(f556({
        let m2144 = match v0.clone() {
            Adt62::C0(h2143) => {
                h2143
            }
        };
        match m2144 {
            Adt63::C0(h2142, _, _) => {
                h2142
            }
        }
    }))), {
        let a2151 = f594(Str::lit("total"), {
            let m2147 = match v0.clone() {
                Adt62::C0(h2146) => {
                    h2146
                }
            };
            match m2147 {
                Adt63::C0(_, h2145, _) => {
                    h2145
                }
            }
        })?;
        let a2152 = f588(Str::lit("hasMore"), {
            let m2150 = match v0 {
                Adt62::C0(h2149) => {
                    h2149
                }
            };
            match m2150 {
                Adt63::C0(_, _, h2148) => {
                    h2148
                }
            }
        });
        append_list(a2151, a2152)
    }))), List::<(Str, Adt6)>::nil()))
}

pub fn f540(v0: Adt83) -> R<Option<List<(Str, Adt6)>>> {
    let m2153 = f595(f596(v0.clone()), List::<(Str, Adt6)>::nil())?;
    match m2153 {
        None => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Some(v1) => {
            Ok(Some({
                let a2154 = f597(v1);
                let a2155 = f598(f599(v0));
                append_list(a2154, a2155)
            }))
        }
    }
}

pub fn f541(v0: Adt153) -> Str {
    let m2156 = v0;
    match m2156 {
        Adt153::C0 => {
            Str::lit("complete")
        }
        Adt153::C1 => {
            Str::lit("input_required")
        }
        Adt153::C2 => {
            Str::lit("task")
        }
    }
}

pub fn f542(v0: Adt34) -> Adt153 {
    let m2157 = v0;
    match m2157 {
        Adt34::C0(_) => {
            Adt153::C0
        }
        Adt34::C1(_) => {
            Adt153::C0
        }
        Adt34::C2(_) => {
            Adt153::C0
        }
        Adt34::C3(_) => {
            Adt153::C0
        }
        Adt34::C4(_) => {
            Adt153::C0
        }
        Adt34::C5(_) => {
            Adt153::C0
        }
        Adt34::C6(_) => {
            Adt153::C0
        }
        Adt34::C7(_) => {
            Adt153::C0
        }
        Adt34::C8(_) => {
            Adt153::C0
        }
        Adt34::C9(_) => {
            Adt153::C0
        }
        Adt34::C10(_) => {
            Adt153::C0
        }
        Adt34::C11 => {
            Adt153::C0
        }
        Adt34::C12(_) => {
            Adt153::C1
        }
        Adt34::C13(_) => {
            Adt153::C2
        }
    }
}

pub fn f543(v0: Option<Adt1>) -> List<(Str, Adt6)> {
    let m2158 = v0;
    match m2158 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((f600(), f601(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f544(v0: bool, v1: u64, v2: List<List<u64>>) -> bool {
    if v0 {
        if f602(v2) {
            let a2159 = v1;
            let a2160 = 200u64;
            nat_eq(a2159, a2160)
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f545(v0: u64, v1: List<List<u64>>) -> Adt95 {
    Adt95::C0(v0, f603(), List::<List<u64>>::nil(), f604(v1))
}

pub fn f546(v0: Adt6) -> R<Option<Adt6>> {
    f605(f606(), Adt154::C0(v0), List::<Adt155>::nil())
}

pub fn f547(v0: Adt138) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("jsonrpc"), Adt6::C3(f248())), List::cons((Str::lit("method"), Adt6::C3(Str::lit("notifications/progress"))), List::cons((Str::lit("params"), Adt6::C5(List::cons((Str::lit("progressToken"), f607(match v0.clone() {
        Adt138::C0(h2161, _) => {
            h2161
        }
    })), List::cons((Str::lit("progress"), Adt6::C2({
        let m2164 = match v0.clone() {
            Adt138::C0(_, h2163) => {
                h2163
            }
        };
        match m2164 {
            Adt52::C0(h2162, _, _) => {
                h2162
            }
        }
    })), {
        let a2171 = f608({
            let m2167 = match v0.clone() {
                Adt138::C0(_, h2166) => {
                    h2166
                }
            };
            match m2167 {
                Adt52::C0(_, h2165, _) => {
                    h2165
                }
            }
        });
        let a2172 = f609({
            let m2170 = match v0 {
                Adt138::C0(_, h2169) => {
                    h2169
                }
            };
            match m2170 {
                Adt52::C0(_, _, h2168) => {
                    h2168
                }
            }
        });
        append_list(a2171, a2172)
    })))), List::<(Str, Adt6)>::nil()))))
}

pub fn f548() -> u64 {
    429u64
}

pub fn f549() -> i64 {
    -32602i64
}

pub fn f550() -> i64 {
    -32022i64
}

pub fn f551() -> i64 {
    -32020i64
}

pub fn f552() -> i64 {
    -32601i64
}

pub fn f553(v0: Adt86) -> i64 {
    let m2173 = v0;
    match m2173 {
        Adt86::C0(_) => {
            f549()
        }
        Adt86::C1(_) => {
            f549()
        }
        Adt86::C2(_) => {
            f549()
        }
        Adt86::C3(_) => {
            f549()
        }
        Adt86::C4(_) => {
            f549()
        }
        Adt86::C5(_) => {
            f549()
        }
        Adt86::C6(_) => {
            f610()
        }
        Adt86::C7(_) => {
            f611()
        }
        Adt86::C8(_) => {
            f612()
        }
        Adt86::C9(_) => {
            f610()
        }
        Adt86::C10 => {
            f549()
        }
        Adt86::C11 => {
            f549()
        }
        Adt86::C12(_) => {
            f610()
        }
        Adt86::C13 => {
            f610()
        }
        Adt86::C14 => {
            f610()
        }
        Adt86::C15(_) => {
            f610()
        }
    }
}

pub fn f554(v0: Adt140) -> Str {
    let m2174 = v0;
    match m2174 {
        Adt140::C0 => {
            Str::lit("Invalid params: params is required")
        }
        Adt140::C1 => {
            Str::lit("Invalid params: _meta is required")
        }
        Adt140::C2 => {
            Str::lit("Invalid params: _meta lacks io.modelcontextprotocol/protocolVersion")
        }
        Adt140::C3 => {
            Str::lit("Invalid params: _meta lacks io.modelcontextprotocol/clientCapabilities")
        }
        Adt140::C4 => {
            Str::lit("Invalid params: _meta is not an object")
        }
        Adt140::C5 => {
            Str::lit("Invalid params: io.modelcontextprotocol/protocolVersion is not a string")
        }
        Adt140::C6 => {
            Str::lit("Invalid params: io.modelcontextprotocol/clientCapabilities is not a valid capabilities object")
        }
        Adt140::C7(_) => {
            Str::lit("Invalid params: a required field is missing")
        }
        Adt140::C8(_) => {
            Str::lit("Invalid params: a field has the wrong type")
        }
    }
}

pub fn f555(v0: Adt127) -> Str {
    let m2175 = v0;
    match m2175 {
        Adt127::C0 => {
            Str::lit("Header mismatch: MCP-Protocol-Version header is missing")
        }
        Adt127::C1 => {
            Str::lit("Header mismatch: Mcp-Method header is missing")
        }
        Adt127::C2 => {
            Str::lit("Header mismatch: Mcp-Name header is missing")
        }
        Adt127::C3 => {
            Str::lit("Header mismatch: MCP-Protocol-Version differs from the request body")
        }
        Adt127::C4 => {
            Str::lit("Header mismatch: Mcp-Method differs from the request body")
        }
        Adt127::C5 => {
            Str::lit("Header mismatch: Mcp-Name differs from the request body")
        }
        Adt127::C6(v1) => {
            let m2176 = v1;
            match m2176 {
                Adt128::C0 => {
                    Str::lit("Header mismatch: MCP-Protocol-Version has a malformed value")
                }
                Adt128::C1 => {
                    Str::lit("Header mismatch: Mcp-Method has a malformed value")
                }
                Adt128::C2 => {
                    Str::lit("Header mismatch: Mcp-Name has a malformed value")
                }
            }
        }
        Adt127::C7(v2) => {
            let m2177 = v2;
            match m2177 {
                Adt128::C0 => {
                    Str::lit("Header mismatch: MCP-Protocol-Version is repeated")
                }
                Adt128::C1 => {
                    Str::lit("Header mismatch: Mcp-Method is repeated")
                }
                Adt128::C2 => {
                    Str::lit("Header mismatch: Mcp-Name is repeated")
                }
            }
        }
        Adt127::C8(_) => {
            Str::lit("Header mismatch: a required Mcp-Param header is missing")
        }
        Adt127::C9(_) => {
            Str::lit("Header mismatch: an Mcp-Param header differs from the request body")
        }
        Adt127::C10(_) => {
            Str::lit("Header mismatch: an Mcp-Param header has a malformed value")
        }
    }
}

pub fn f556(v0: List<Str>) -> List<Adt6> {
    f404(f613(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f557(v0: Adt19) -> Adt6 {
    Adt6::C5({
        let a2189 = f614(Str::lit("experimental"), match v0.clone() {
            Adt19::C0(h2178, _, _, _, _) => {
                h2178
            }
        });
        let a2190 = {
            let a2187 = f614(Str::lit("roots"), match v0.clone() {
                Adt19::C0(_, h2179, _, _, _) => {
                    h2179
                }
            });
            let a2188 = {
                let a2185 = f614(Str::lit("sampling"), match v0.clone() {
                    Adt19::C0(_, _, h2180, _, _) => {
                        h2180
                    }
                });
                let a2186 = {
                    let a2183 = f614(Str::lit("elicitation"), match v0.clone() {
                        Adt19::C0(_, _, _, h2181, _) => {
                            h2181
                        }
                    });
                    let a2184 = f614(Str::lit("extensions"), match v0 {
                        Adt19::C0(_, _, _, _, h2182) => {
                            h2182
                        }
                    });
                    append_list(a2183, a2184)
                };
                append_list(a2185, a2186)
            };
            append_list(a2187, a2188)
        };
        append_list(a2189, a2190)
    })
}

pub fn f558(v0: List<u64>) -> R<bool> {
    let m2191 = v0;
    match m2191.uncons() {
        None => {
            Ok(false)
        }
        Some((v1, v2)) => {
            let m2192 = v2;
            match m2192.uncons() {
                None => {
                    Ok(false)
                }
                Some((v3, v4)) => {
                    let a2193 = f302(v1)?;
                    let a2194 = 113u64;
                    if if nat_eq(a2193, a2194) {
                        let a2195 = v3;
                        let a2196 = 61u64;
                        nat_eq(a2195, a2196)
                    } else {
                        false
                    } {
                        Ok(f615(v4))
                    } else {
                        Ok(false)
                    }
                }
            }
        }
    }
}

pub fn f559(v0: Adt0, v1: Fn2, v2: u64, v3: Result<Adt16, Adt140>) -> R<Result<Adt136, Adt139>> {
    let m2197 = v3;
    match m2197 {
        Ok(v5) => {
            f616(v0, v1, v2, v5)
        }
        Err(v4) => {
            Ok(Err::<Adt136, Adt139>(Adt139::C0(v4)))
        }
    }
}

pub fn f560(v0: Adt148, v1: Adt18, v2: Adt5) -> Result<Adt16, Adt140> {
    let m2198 = v0;
    match m2198 {
        Adt148::C0 => {
            f617(v1, v2)
        }
        Adt148::C1 => {
            f618(v1, v2)
        }
        Adt148::C2 => {
            f619(v1, v2)
        }
        Adt148::C3 => {
            f620(v1, v2)
        }
        Adt148::C4 => {
            f621(v1, v2)
        }
        Adt148::C5 => {
            f622(v1, v2)
        }
        Adt148::C6 => {
            f623(v1, v2)
        }
        Adt148::C7 => {
            f624(v1, v2)
        }
        Adt148::C8 => {
            f625(v1, v2)
        }
        Adt148::C9 => {
            f626(v1, v2)
        }
        Adt148::C10 => {
            f627(v1, v2)
        }
        Adt148::C11 => {
            f628(v1, v2)
        }
    }
}

pub fn f561(v0: Adt148) -> Str {
    let m2199 = v0;
    match m2199 {
        Adt148::C0 => {
            Str::lit("server/discover")
        }
        Adt148::C1 => {
            Str::lit("tools/list")
        }
        Adt148::C2 => {
            Str::lit("tools/call")
        }
        Adt148::C3 => {
            Str::lit("resources/list")
        }
        Adt148::C4 => {
            Str::lit("resources/read")
        }
        Adt148::C5 => {
            Str::lit("resources/templates/list")
        }
        Adt148::C6 => {
            Str::lit("prompts/list")
        }
        Adt148::C7 => {
            Str::lit("prompts/get")
        }
        Adt148::C8 => {
            Str::lit("completion/complete")
        }
        Adt148::C9 => {
            Str::lit("tasks/get")
        }
        Adt148::C10 => {
            Str::lit("tasks/update")
        }
        Adt148::C11 => {
            Str::lit("tasks/cancel")
        }
    }
}

pub fn f562(v0: Option<Adt8>) -> bool {
    let m2200 = v0;
    match m2200 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f563(v0: Option<Adt9>) -> bool {
    let m2201 = v0;
    match m2201 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f564(v0: Option<Adt5>) -> bool {
    let m2202 = v0;
    match m2202 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f565(v0: Option<Adt5>, v1: Str) -> bool {
    let m2203 = v0;
    match m2203 {
        None => {
            false
        }
        Some(v2) => {
            f368(v1, v2)
        }
    }
}

pub fn f566() -> Str {
    Str::lit("io.modelcontextprotocol/tasks")
}

pub fn f567(v0: List<Str>, v1: Adt116, v2: Adt18, v3: Option<Adt127>) -> Adt150 {
    let m2204 = v3;
    match m2204 {
        None => {
            f629(v1, v2.clone(), f630(match v2 {
                Adt18::C0(h2205, _, _, _, _, _) => {
                    h2205
                }
            }, v0))
        }
        Some(v4) => {
            Adt150::C1(Adt139::C2(v4))
        }
    }
}

pub fn f568(v0: Option<Adt126>, v1: Adt152, v2: Str, v3: Str, v4: Adt5) -> R<Option<Adt127>> {
    let m2206 = v0;
    match m2206 {
        None => {
            Ok(None::<Adt127>)
        }
        Some(v5) => {
            let m2208 = match v5.clone() {
                Adt126::C0(_, _, _, h2207, _) => {
                    h2207
                }
            };
            match m2208 {
                None => {
                    f631(v5, v1, v2, v3, v4)
                }
                Some(v6) => {
                    Ok(Some(v6))
                }
            }
        }
    }
}

pub fn f569(v0: Option<Adt5>) -> Adt5 {
    let m2209 = v0;
    match m2209 {
        None => {
            f632()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f570(v0: Adt5) -> Result<Adt18, Adt140> {
    let m2210 = f633(v0.clone());
    match m2210 {
        Adt156::C0 => {
            f634(v0.clone(), f184(f635(), v0.clone()), f184(f636(), v0))
        }
        Adt156::C1 => {
            Err::<Adt18, Adt140>(Adt140::C2)
        }
        Adt156::C2 => {
            Err::<Adt18, Adt140>(Adt140::C3)
        }
    }
}

pub fn f571() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(112u64, List::cons(97u64, List::cons(114u64, List::cons(97u64, List::cons(109u64, List::cons(45u64, List::<u64>::nil()))))))))))
}

pub fn f572(p2214: List<Adt6>) -> bool {
    let m2213 = p2214;
    {
        let mut m2213 = m2213;
        loop {
            let step = {
                let v0 = m2213;
                let m2211 = v0;
                match m2211.uncons() {
                    None => {
                        Err::<List<Adt6>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let m2212 = f130(v1);
                        match m2212 {
                            None => {
                                Err::<List<Adt6>, bool>(false)
                            }
                            Some(_) => {
                                Ok::<List<Adt6>, bool>(v2)
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2213 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f573(p2218: List<Adt6>) -> bool {
    let m2217 = p2218;
    {
        let mut m2217 = m2217;
        loop {
            let step = {
                let v0 = m2217;
                let m2215 = v0;
                match m2215.uncons() {
                    None => {
                        Err::<List<Adt6>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let m2216 = f130(v1);
                        match m2216 {
                            None => {
                                Err::<List<Adt6>, bool>(false)
                            }
                            Some(v3) => {
                                if f381(v3) {
                                    Ok::<List<Adt6>, bool>(v2)
                                } else {
                                    Err::<List<Adt6>, bool>(false)
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2217 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f574(v0: Str) -> Option<Adt13> {
    let a2219 = v0.clone();
    let a2220 = Str::lit("string");
    if equal(a2219, a2220) {
        Some(Adt13::C0)
    } else {
        let a2221 = v0.clone();
        let a2222 = Str::lit("integer");
        if equal(a2221, a2222) {
            Some(Adt13::C1)
        } else {
            let a2223 = v0;
            let a2224 = Str::lit("boolean");
            if equal(a2223, a2224) {
                Some(Adt13::C2)
            } else {
                None::<Adt13>
            }
        }
    }
}

pub fn f575(p2233: List<u64>, p2234: List<u64>) -> R<List<u64>> {
    let m2232 = (p2233, p2234);
    {
        let mut m2232 = m2232;
        loop {
            let step = {
                let (v0, v1) = m2232;
                let m2225 = v0;
                match m2225.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), R<List<u64>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        let a2226 = 65u64;
                        let a2227 = v2;
                        if if nat_le(a2226, a2227) {
                            let a2228 = v2;
                            let a2229 = 90u64;
                            nat_le(a2228, a2229)
                        } else {
                            false
                        } {
                            Ok::<(List<u64>, List<u64>), R<List<u64>>>((v3, List::cons({
                                let a2230 = v2;
                                let a2231 = 32u64;
                                nat_add(a2230, a2231)?
                            }, v1)))
                        } else {
                            Ok::<(List<u64>, List<u64>), R<List<u64>>>((v3, List::cons(v2, v1)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2232 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f576(p2241: List<u64>, p2242: List<u64>) -> bool {
    let m2240 = (p2241, p2242);
    {
        let mut m2240 = m2240;
        loop {
            let step = {
                let (v0, v1) = m2240;
                let m2235 = v0;
                match m2235.uncons() {
                    None => {
                        let m2236 = v1;
                        match m2236.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m2237 = v1;
                        match m2237.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                            Some((v6, v7)) => {
                                let a2238 = v4;
                                let a2239 = v6;
                                if nat_eq(a2238, a2239) {
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
                Ok(next) => { m2240 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f577(p2247: u64, p2248: i64) -> R<i64> {
    let m2246 = (p2247, p2248);
    {
        let mut m2246 = m2246;
        loop {
            let step = {
                let (v0, v1) = m2246;
                let m2243 = v0;
                if m2243 == 0 {
                    Err::<(u64, i64), R<i64>>(Ok(v1))
                } else {
                    let v2 = m2243 - 1;
                    Ok::<(u64, i64), R<i64>>((v2, {
                        let a2244 = v1;
                        let a2245 = 10i64;
                        int_mul(a2244, a2245)?
                    }))
                }
            };
            match step {
                Ok(next) => { m2246 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f578(p2259: u64, p2260: i64, p2261: u64) -> R<u64> {
    let m2258 = (p2259, (p2260, p2261));
    {
        let mut m2258 = m2258;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2258;
                let m2249 = v0;
                if m2249 == 0 {
                    Err::<(u64, (i64, u64)), R<u64>>(Ok(v2))
                } else {
                    let v3 = m2249 - 1;
                    let v4: u64 = {
                        let a2250 = v2;
                        let a2251 = 1u64;
                        nat_add(a2250, a2251)?
                    };
                    let m2254 = {
                        let a2252 = v1;
                        let a2253 = 10i64;
                        compare(a2252, a2253)
                    };
                    if match m2254 {
                        Ordering::Less => {
                            true
                        }
                        Ordering::Equal => {
                            false
                        }
                        Ordering::Greater => {
                            false
                        }
                    } {
                        Err::<(u64, (i64, u64)), R<u64>>(Ok(v4))
                    } else {
                        Ok::<(u64, (i64, u64)), R<u64>>((v3, ({
                            let a2255 = v1;
                            let a2256 = 10i64;
                            let a2257 = 0i64;
                            int_quot(a2255, a2256, a2257)?
                        }, v4)))
                    }
                }
            };
            match step {
                Ok(next) => { m2258 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f579(p2273: u64, p2274: i64, p2275: List<u64>) -> R<List<u64>> {
    let m2272 = (p2273, (p2274, p2275));
    {
        let mut m2272 = m2272;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2272;
                let m2262 = v0;
                if m2262 == 0 {
                    Err::<(u64, (i64, List<u64>)), R<List<u64>>>(Ok(v2))
                } else {
                    let v3 = m2262 - 1;
                    let v4: i64 = {
                        let a2263 = v1;
                        let a2264 = 10i64;
                        let a2265 = 0i64;
                        int_quot(a2263, a2264, a2265)?
                    };
                    let v5: i64 = {
                        let a2266 = v1;
                        let a2267 = 10i64;
                        let a2268 = 0i64;
                        int_rem(a2266, a2267, a2268)
                    };
                    let m2271 = {
                        let a2269 = v4;
                        let a2270 = 1i64;
                        compare(a2269, a2270)
                    };
                    if match m2271 {
                        Ordering::Less => {
                            true
                        }
                        Ordering::Equal => {
                            false
                        }
                        Ordering::Greater => {
                            false
                        }
                    } {
                        Err::<(u64, (i64, List<u64>)), R<List<u64>>>(Ok(List::cons(f637(v5), v2)))
                    } else {
                        Ok::<(u64, (i64, List<u64>)), R<List<u64>>>((v3, (v4, List::cons(f637(v5), v2))))
                    }
                }
            };
            match step {
                Ok(next) => { m2272 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f580(v0: u64) -> R<u64> {
    let a2276 = v0;
    let a2277 = 10u64;
    if nat_lt(a2276, a2277) {
        let a2278 = 48u64;
        let a2279 = v0;
        nat_add(a2278, a2279)
    } else {
        let a2280 = 87u64;
        let a2281 = v0;
        nat_add(a2280, a2281)
    }
}

pub fn f581(v0: List<Str>) -> Adt6 {
    Adt6::C4(f556(v0))
}

pub fn f582(v0: Adt4) -> Adt6 {
    Adt6::C5({
        let a2299 = f614(Str::lit("experimental"), match v0.clone() {
            Adt4::C0(h2282, _, _, _, _, _, _) => {
                h2282
            }
        });
        let a2300 = {
            let a2297 = f614(Str::lit("logging"), match v0.clone() {
                Adt4::C0(_, h2283, _, _, _, _, _) => {
                    h2283
                }
            });
            let a2298 = {
                let a2295 = f614(Str::lit("completions"), match v0.clone() {
                    Adt4::C0(_, _, h2284, _, _, _, _) => {
                        h2284
                    }
                });
                let a2296 = {
                    let a2293 = f638(Str::lit("prompts"), match v0.clone() {
                        Adt4::C0(_, _, _, h2285, _, _, _) => {
                            h2285
                        }
                    });
                    let a2294 = {
                        let a2291 = f639(match v0.clone() {
                            Adt4::C0(_, _, _, _, h2286, _, _) => {
                                h2286
                            }
                        });
                        let a2292 = {
                            let a2289 = f638(Str::lit("tools"), match v0.clone() {
                                Adt4::C0(_, _, _, _, _, h2287, _) => {
                                    h2287
                                }
                            });
                            let a2290 = f614(Str::lit("extensions"), match v0 {
                                Adt4::C0(_, _, _, _, _, _, h2288) => {
                                    h2288
                                }
                            });
                            append_list(a2289, a2290)
                        };
                        append_list(a2291, a2292)
                    };
                    append_list(a2293, a2294)
                };
                append_list(a2295, a2296)
            };
            append_list(a2297, a2298)
        };
        append_list(a2299, a2300)
    })
}

pub fn f583(v0: Str, v1: Option<Str>) -> List<(Str, Adt6)> {
    let m2301 = v1;
    match m2301 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C3(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f584(v0: Adt10) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("ttlMs"), f52(match v0.clone() {
        Adt10::C0(h2302, _) => {
            h2302
        }
    })?), List::cons((Str::lit("cacheScope"), Adt6::C3(f640(match v0 {
        Adt10::C0(_, h2303) => {
            h2303
        }
    }))), List::<(Str, Adt6)>::nil())))
}

pub fn f585(v0: List<Adt37>) -> List<Adt6> {
    f404(f641(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f586(v0: List<Adt40>) -> R<Option<List<Adt6>>> {
    let m2304 = f642(v0, List::<Adt6>::nil())?;
    match m2304 {
        None => {
            Ok(None::<List<Adt6>>)
        }
        Some(v1) => {
            Ok(Some(f404(v1, List::<Adt6>::nil())))
        }
    }
}

pub fn f587(v0: Str, v1: Option<Adt6>) -> List<(Str, Adt6)> {
    let m2305 = v1;
    match m2305 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, v2), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f588(v0: Str, v1: Option<bool>) -> List<(Str, Adt6)> {
    let m2306 = v1;
    match m2306 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C1(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f589(v0: List<Adt46>) -> R<List<Adt6>> {
    Ok(f404(f643(v0, List::<Adt6>::nil())?, List::<Adt6>::nil()))
}

pub fn f590(v0: List<Adt48>) -> List<Adt6> {
    f404(f644(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f591(v0: List<Adt56>) -> List<Adt6> {
    f404(f645(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f592(v0: List<Adt58>) -> List<Adt6> {
    f404(f646(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f593(v0: List<Adt61>) -> R<Option<List<Adt6>>> {
    let m2307 = f647(v0, List::<Adt6>::nil())?;
    match m2307 {
        None => {
            Ok(None::<List<Adt6>>)
        }
        Some(v1) => {
            Ok(Some(f404(v1, List::<Adt6>::nil())))
        }
    }
}

pub fn f594(v0: Str, v1: Option<u64>) -> R<List<(Str, Adt6)>> {
    let m2308 = v1;
    match m2308 {
        None => {
            Ok(List::<(Str, Adt6)>::nil())
        }
        Some(v2) => {
            Ok(List::cons((v0, f52(v2)?), List::<(Str, Adt6)>::nil()))
        }
    }
}

pub fn f595(p2314: List<(Str, Adt66)>, p2315: List<(Str, Adt6)>) -> R<Option<List<(Str, Adt6)>>> {
    let m2313 = (p2314, p2315);
    {
        let mut m2313 = m2313;
        loop {
            let step = {
                let (v0, v1) = m2313;
                let m2309 = v0;
                match m2309.uncons() {
                    None => {
                        Err::<(List<(Str, Adt66)>, List<(Str, Adt6)>), R<Option<List<(Str, Adt6)>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m2311 = f648({
                            let (_, h2310) = v2;
                            h2310
                        })?;
                        match m2311 {
                            None => {
                                Err::<(List<(Str, Adt66)>, List<(Str, Adt6)>), R<Option<List<(Str, Adt6)>>>>(Ok(None::<List<(Str, Adt6)>>))
                            }
                            Some(v4) => {
                                Ok::<(List<(Str, Adt66)>, List<(Str, Adt6)>), R<Option<List<(Str, Adt6)>>>>((v3, List::cons(({
                                    let (h2312, _) = v2;
                                    h2312
                                }, v4), v1)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2313 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f596(v0: Adt83) -> List<(Str, Adt66)> {
    let m2316 = v0;
    match m2316 {
        Adt83::C0(v1) => {
            v1
        }
        Adt83::C1(_) => {
            List::<(Str, Adt66)>::nil()
        }
        Adt83::C2(v3, _) => {
            v3
        }
    }
}

pub fn f597(v0: List<(Str, Adt6)>) -> List<(Str, Adt6)> {
    let m2317 = v0.clone();
    match m2317.uncons() {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some((_, _)) => {
            List::cons((Str::lit("inputRequests"), Adt6::C5(f406(v0, List::<(Str, Adt6)>::nil()))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f598(v0: Option<Str>) -> List<(Str, Adt6)> {
    let m2318 = v0;
    match m2318 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("requestState"), Adt6::C3(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f599(v0: Adt83) -> Option<Str> {
    let m2319 = v0;
    match m2319 {
        Adt83::C0(_) => {
            None::<Str>
        }
        Adt83::C1(v2) => {
            Some(v2)
        }
        Adt83::C2(_, v4) => {
            Some(v4)
        }
    }
}

pub fn f600() -> Str {
    Str::lit("io.modelcontextprotocol/serverInfo")
}

pub fn f601(v0: Adt1) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
        Adt1::C0(h2320, _, _, _, _, _) => {
            h2320
        }
    })), List::cons((Str::lit("version"), Adt6::C3(match v0.clone() {
        Adt1::C0(_, _, h2321, _, _, _) => {
            h2321
        }
    })), {
        let a2330 = f583(Str::lit("title"), match v0.clone() {
            Adt1::C0(_, h2322, _, _, _, _) => {
                h2322
            }
        });
        let a2331 = {
            let a2328 = f583(Str::lit("description"), match v0.clone() {
                Adt1::C0(_, _, _, h2323, _, _) => {
                    h2323
                }
            });
            let a2329 = {
                let a2326 = f583(Str::lit("websiteUrl"), match v0.clone() {
                    Adt1::C0(_, _, _, _, h2324, _) => {
                        h2324
                    }
                });
                let a2327 = f649(match v0 {
                    Adt1::C0(_, _, _, _, _, h2325) => {
                        h2325
                    }
                });
                append_list(a2326, a2327)
            };
            append_list(a2328, a2329)
        };
        append_list(a2330, a2331)
    })))
}

pub fn f602(v0: List<List<u64>>) -> bool {
    let m2332 = v0;
    match m2332.uncons() {
        None => {
            false
        }
        Some((_, _)) => {
            true
        }
    }
}

pub fn f603() -> List<Adt92> {
    List::cons(Adt92::C0(f650(), f490()), List::cons(Adt92::C0(f651(), f652()), List::cons(Adt92::C0(f653(), f654()), List::<Adt92>::nil())))
}

pub fn f604(v0: List<List<u64>>) -> List<u64> {
    f655(f109(v0, List::<List<u64>>::nil()), List::<u64>::nil())
}

pub fn f605(p2347: u64, p2348: Adt154, p2349: List<Adt155>) -> R<Option<Adt6>> {
    let m2346 = (p2347, (p2348, p2349));
    {
        let mut m2346 = m2346;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2346;
                let m2333 = v0;
                if m2333 == 0 {
                    Err::<(u64, (Adt154, List<Adt155>)), R<Option<Adt6>>>(Ok(None::<Adt6>))
                } else {
                    let v3 = m2333 - 1;
                    let m2334 = v1;
                    match m2334 {
                        Adt154::C0(v4) => {
                            let m2335 = v4.clone();
                            match m2335 {
                                Adt6::C0 => {
                                    Ok::<(u64, (Adt154, List<Adt155>)), R<Option<Adt6>>>((v3, (Adt154::C1(v4), v2)))
                                }
                                Adt6::C1(_) => {
                                    Ok::<(u64, (Adt154, List<Adt155>)), R<Option<Adt6>>>((v3, (Adt154::C1(v4), v2)))
                                }
                                Adt6::C2(_) => {
                                    Ok::<(u64, (Adt154, List<Adt155>)), R<Option<Adt6>>>((v3, (Adt154::C1(v4), v2)))
                                }
                                Adt6::C3(v7) => {
                                    let v8: Str = f656(v7)?;
                                    Ok::<(u64, (Adt154, List<Adt155>)), R<Option<Adt6>>>((v3, (Adt154::C1(Adt6::C3(v8)), v2)))
                                }
                                Adt6::C4(v9) => {
                                    let m2336 = v9;
                                    match m2336.uncons() {
                                        None => {
                                            Ok::<(u64, (Adt154, List<Adt155>)), R<Option<Adt6>>>((v3, (Adt154::C1(v4), v2)))
                                        }
                                        Some((v10, v11)) => {
                                            Ok::<(u64, (Adt154, List<Adt155>)), R<Option<Adt6>>>((v3, (Adt154::C0(v10), List::cons(Adt155::C0(List::<Adt6>::nil(), v11), v2))))
                                        }
                                    }
                                }
                                Adt6::C5(v12) => {
                                    let m2337 = v12;
                                    match m2337.uncons() {
                                        None => {
                                            Ok::<(u64, (Adt154, List<Adt155>)), R<Option<Adt6>>>((v3, (Adt154::C1(v4), v2)))
                                        }
                                        Some((v13, v14)) => {
                                            let v15: Str = f656({
                                                let (h2338, _) = v13;
                                                h2338
                                            })?;
                                            Ok::<(u64, (Adt154, List<Adt155>)), R<Option<Adt6>>>((v3, (Adt154::C0({
                                                let (_, h2339) = v13;
                                                h2339
                                            }), List::cons(Adt155::C1(v15, List::<(Str, Adt6)>::nil(), v14), v2))))
                                        }
                                    }
                                }
                            }
                        }
                        Adt154::C1(v16) => {
                            let m2340 = v2;
                            match m2340.uncons() {
                                None => {
                                    Err::<(u64, (Adt154, List<Adt155>)), R<Option<Adt6>>>(Ok(Some(v16)))
                                }
                                Some((v17, v18)) => {
                                    let m2341 = v17;
                                    match m2341 {
                                        Adt155::C0(v19, v20) => {
                                            let v21: List<Adt6> = List::cons(v16, v19);
                                            let m2342 = v20;
                                            match m2342.uncons() {
                                                None => {
                                                    Ok::<(u64, (Adt154, List<Adt155>)), R<Option<Adt6>>>((v3, (Adt154::C1(Adt6::C4(f404(v21, List::<Adt6>::nil()))), v18)))
                                                }
                                                Some((v22, v23)) => {
                                                    Ok::<(u64, (Adt154, List<Adt155>)), R<Option<Adt6>>>((v3, (Adt154::C0(v22), List::cons(Adt155::C0(v21, v23), v18))))
                                                }
                                            }
                                        }
                                        Adt155::C1(v24, v25, v26) => {
                                            let v27: List<(Str, Adt6)> = List::cons((v24, v16), v25);
                                            let m2343 = v26;
                                            match m2343.uncons() {
                                                None => {
                                                    Ok::<(u64, (Adt154, List<Adt155>)), R<Option<Adt6>>>((v3, (Adt154::C1(Adt6::C5(f406(v27, List::<(Str, Adt6)>::nil()))), v18)))
                                                }
                                                Some((v28, v29)) => {
                                                    let v30: Str = f656({
                                                        let (h2344, _) = v28;
                                                        h2344
                                                    })?;
                                                    Ok::<(u64, (Adt154, List<Adt155>)), R<Option<Adt6>>>((v3, (Adt154::C0({
                                                        let (_, h2345) = v28;
                                                        h2345
                                                    }), List::cons(Adt155::C1(v30, v27, v29), v18))))
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2346 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f606() -> u64 {
    67108864u64
}

pub fn f607(v0: Adt21) -> Adt6 {
    let m2350 = v0;
    match m2350 {
        Adt21::C0(v1) => {
            Adt6::C3(v1)
        }
        Adt21::C1(v2) => {
            Adt6::C2(Adt7::C0(v2, 0i64))
        }
    }
}

pub fn f608(v0: Option<Adt7>) -> List<(Str, Adt6)> {
    let m2351 = v0;
    match m2351 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("total"), Adt6::C2(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f609(v0: Option<Str>) -> List<(Str, Adt6)> {
    let m2352 = v0;
    match m2352 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("message"), Adt6::C3(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f610() -> i64 {
    -32603i64
}

pub fn f611() -> i64 {
    429i64
}

pub fn f612() -> i64 {
    -32021i64
}

pub fn f613(p2355: List<Str>, p2356: List<Adt6>) -> List<Adt6> {
    let m2354 = (p2355, p2356);
    {
        let mut m2354 = m2354;
        loop {
            let step = {
                let (v0, v1) = m2354;
                let m2353 = v0;
                match m2353.uncons() {
                    None => {
                        Err::<(List<Str>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Str>, List<Adt6>), List<Adt6>>((v3, List::cons(Adt6::C3(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2354 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f614(v0: Str, v1: Option<Adt5>) -> List<(Str, Adt6)> {
    let m2357 = v1;
    match m2357 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C5(match v2 {
                Adt5::C0(h2358) => {
                    h2358
                }
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f615(v0: List<u64>) -> bool {
    let m2359 = v0;
    match m2359.uncons() {
        None => {
            false
        }
        Some((v1, v2)) => {
            let a2360 = v1;
            let a2361 = 48u64;
            if nat_eq(a2360, a2361) {
                f657(v2)
            } else {
                false
            }
        }
    }
}

pub fn f616(v0: Adt0, v1: Fn2, v2: u64, v3: Adt16) -> R<Result<Adt136, Adt139>> {
    if f658(f659(v3.clone())) {
        Ok(Ok::<Adt136, Adt139>(f660(v0.clone(), Adt34::C0(f661(v0)))))
    } else {
        Ok(f662(v0.clone(), v3.clone(), f663(match v0.clone() {
            Adt0::C0(_, _, _, _, _, _, h2362) => {
                h2362
            }
        }, match v0 {
            Adt0::C0(_, _, _, _, _, h2363, _) => {
                h2363
            }
        }, v2, v1, v3)?))
    }
}

pub fn f617(v0: Adt18, _: Adt5) -> Result<Adt16, Adt140> {
    Ok::<Adt16, Adt140>(Adt16::C0(Adt17::C0(v0)))
}

pub fn f618(v0: Adt18, v1: Adt5) -> Result<Adt16, Adt140> {
    let m2364 = f664(Str::lit("cursor"), v1);
    match m2364 {
        Ok(v2) => {
            Ok::<Adt16, Adt140>(Adt16::C1(Adt22::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt16, Adt140>(v3)
        }
    }
}

pub fn f619(v0: Adt18, v1: Adt5) -> Result<Adt16, Adt140> {
    let m2365 = f665(Str::lit("name"), v1.clone());
    match m2365 {
        Ok(v2) => {
            let m2366 = f666(Str::lit("arguments"), v1.clone());
            match m2366 {
                Ok(v3) => {
                    let m2367 = f666(Str::lit("inputResponses"), v1.clone());
                    match m2367 {
                        Ok(v4) => {
                            let m2368 = f664(Str::lit("requestState"), v1);
                            match m2368 {
                                Ok(v5) => {
                                    Ok::<Adt16, Adt140>(Adt16::C2(Adt23::C0(v0, v2, v3, v4, v5)))
                                }
                                Err(v6) => {
                                    Err::<Adt16, Adt140>(v6)
                                }
                            }
                        }
                        Err(v7) => {
                            Err::<Adt16, Adt140>(v7)
                        }
                    }
                }
                Err(v8) => {
                    Err::<Adt16, Adt140>(v8)
                }
            }
        }
        Err(v9) => {
            Err::<Adt16, Adt140>(v9)
        }
    }
}

pub fn f620(v0: Adt18, v1: Adt5) -> Result<Adt16, Adt140> {
    let m2369 = f664(Str::lit("cursor"), v1);
    match m2369 {
        Ok(v2) => {
            Ok::<Adt16, Adt140>(Adt16::C3(Adt22::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt16, Adt140>(v3)
        }
    }
}

pub fn f621(v0: Adt18, v1: Adt5) -> Result<Adt16, Adt140> {
    let m2370 = f665(Str::lit("uri"), v1.clone());
    match m2370 {
        Ok(v2) => {
            let m2371 = f666(Str::lit("inputResponses"), v1.clone());
            match m2371 {
                Ok(v3) => {
                    let m2372 = f664(Str::lit("requestState"), v1);
                    match m2372 {
                        Ok(v4) => {
                            Ok::<Adt16, Adt140>(Adt16::C4(Adt24::C0(v0, v2, v3, v4)))
                        }
                        Err(v5) => {
                            Err::<Adt16, Adt140>(v5)
                        }
                    }
                }
                Err(v6) => {
                    Err::<Adt16, Adt140>(v6)
                }
            }
        }
        Err(v7) => {
            Err::<Adt16, Adt140>(v7)
        }
    }
}

pub fn f622(v0: Adt18, v1: Adt5) -> Result<Adt16, Adt140> {
    let m2373 = f664(Str::lit("cursor"), v1);
    match m2373 {
        Ok(v2) => {
            Ok::<Adt16, Adt140>(Adt16::C5(Adt22::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt16, Adt140>(v3)
        }
    }
}

pub fn f623(v0: Adt18, v1: Adt5) -> Result<Adt16, Adt140> {
    let m2374 = f664(Str::lit("cursor"), v1);
    match m2374 {
        Ok(v2) => {
            Ok::<Adt16, Adt140>(Adt16::C6(Adt22::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt16, Adt140>(v3)
        }
    }
}

pub fn f624(v0: Adt18, v1: Adt5) -> Result<Adt16, Adt140> {
    let m2375 = f665(Str::lit("name"), v1.clone());
    match m2375 {
        Ok(v2) => {
            let m2376 = f667(Str::lit("arguments"), v1.clone());
            match m2376 {
                Ok(v3) => {
                    let m2377 = f666(Str::lit("inputResponses"), v1.clone());
                    match m2377 {
                        Ok(v4) => {
                            let m2378 = f664(Str::lit("requestState"), v1);
                            match m2378 {
                                Ok(v5) => {
                                    Ok::<Adt16, Adt140>(Adt16::C7(Adt25::C0(v0, v2, v3, v4, v5)))
                                }
                                Err(v6) => {
                                    Err::<Adt16, Adt140>(v6)
                                }
                            }
                        }
                        Err(v7) => {
                            Err::<Adt16, Adt140>(v7)
                        }
                    }
                }
                Err(v8) => {
                    Err::<Adt16, Adt140>(v8)
                }
            }
        }
        Err(v9) => {
            Err::<Adt16, Adt140>(v9)
        }
    }
}

pub fn f625(v0: Adt18, v1: Adt5) -> Result<Adt16, Adt140> {
    let m2379 = f668(Str::lit("ref"), v1.clone());
    match m2379 {
        Ok(v2) => {
            let m2380 = f669(v2);
            match m2380 {
                Ok(v3) => {
                    let m2381 = f668(Str::lit("argument"), v1.clone());
                    match m2381 {
                        Ok(v4) => {
                            let m2382 = f670(v4);
                            match m2382 {
                                Ok(v5) => {
                                    let m2383 = f671(v1);
                                    match m2383 {
                                        Ok(v6) => {
                                            Ok::<Adt16, Adt140>(Adt16::C8(Adt26::C0(v0, v3, v5, v6)))
                                        }
                                        Err(v7) => {
                                            Err::<Adt16, Adt140>(v7)
                                        }
                                    }
                                }
                                Err(v8) => {
                                    Err::<Adt16, Adt140>(v8)
                                }
                            }
                        }
                        Err(v9) => {
                            Err::<Adt16, Adt140>(v9)
                        }
                    }
                }
                Err(v10) => {
                    Err::<Adt16, Adt140>(v10)
                }
            }
        }
        Err(v11) => {
            Err::<Adt16, Adt140>(v11)
        }
    }
}

pub fn f626(v0: Adt18, v1: Adt5) -> Result<Adt16, Adt140> {
    let m2384 = f665(Str::lit("taskId"), v1);
    match m2384 {
        Ok(v2) => {
            Ok::<Adt16, Adt140>(Adt16::C9(Adt31::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt16, Adt140>(v3)
        }
    }
}

pub fn f627(v0: Adt18, v1: Adt5) -> Result<Adt16, Adt140> {
    let m2385 = f665(Str::lit("taskId"), v1.clone());
    match m2385 {
        Ok(v2) => {
            let m2386 = f668(Str::lit("inputResponses"), v1);
            match m2386 {
                Ok(v3) => {
                    Ok::<Adt16, Adt140>(Adt16::C10(Adt32::C0(v0, v2, v3)))
                }
                Err(v4) => {
                    Err::<Adt16, Adt140>(v4)
                }
            }
        }
        Err(v5) => {
            Err::<Adt16, Adt140>(v5)
        }
    }
}

pub fn f628(v0: Adt18, v1: Adt5) -> Result<Adt16, Adt140> {
    let m2387 = f665(Str::lit("taskId"), v1);
    match m2387 {
        Ok(v2) => {
            Ok::<Adt16, Adt140>(Adt16::C11(Adt33::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt16, Adt140>(v3)
        }
    }
}

pub fn f629(v0: Adt116, v1: Adt18, v2: Option<Adt139>) -> Adt150 {
    let m2388 = v2;
    match m2388 {
        None => {
            Adt150::C0(Adt151::C0(match v0.clone() {
                Adt116::C0(h2389, _, _) => {
                    h2389
                }
            }, match v0.clone() {
                Adt116::C0(_, h2390, _) => {
                    h2390
                }
            }, v1, f569(match v0 {
                Adt116::C0(_, _, h2391) => {
                    h2391
                }
            })))
        }
        Some(v3) => {
            Adt150::C1(v3)
        }
    }
}

pub fn f630(v0: Str, v1: List<Str>) -> Option<Adt139> {
    f672(f673(v0.clone(), v1.clone()), v0, v1)
}

pub fn f631(v0: Adt126, v1: Adt152, v2: Str, v3: Str, v4: Adt5) -> R<Option<Adt127>> {
    let m2393 = match v0.clone() {
        Adt126::C0(h2392, _, _, _, _) => {
            h2392
        }
    };
    match m2393 {
        None => {
            Ok(Some(Adt127::C0))
        }
        Some(v5) => {
            let a2394 = v5;
            let a2395 = v3;
            if equal(a2394, a2395) {
                let m2397 = match v0.clone() {
                    Adt126::C0(_, h2396, _, _, _) => {
                        h2396
                    }
                };
                match m2397 {
                    None => {
                        Ok(Some(Adt127::C1))
                    }
                    Some(v6) => {
                        let a2398 = v6;
                        let a2399 = v2;
                        if equal(a2398, a2399) {
                            f674(f675(match v0.clone() {
                                Adt126::C0(_, _, h2400, _, _) => {
                                    h2400
                                }
                            }, v1, v4.clone()), match v0 {
                                Adt126::C0(_, _, _, _, h2401) => {
                                    h2401
                                }
                            }, v4)
                        } else {
                            Ok(Some(Adt127::C4))
                        }
                    }
                }
            } else {
                Ok(Some(Adt127::C3))
            }
        }
    }
}

pub fn f632() -> Adt5 {
    Adt5::C0(List::<(Str, Adt6)>::nil())
}

pub fn f633(v0: Adt5) -> Adt156 {
    f676(f184(f635(), v0.clone()), f184(f636(), v0))
}

pub fn f634(v0: Adt5, v1: Adt114, v2: Adt114) -> Result<Adt18, Adt140> {
    let m2402 = v1;
    match m2402 {
        Adt114::C0 => {
            Err::<Adt18, Adt140>(Adt140::C2)
        }
        Adt114::C1(v3) => {
            let m2403 = v2;
            match m2403 {
                Adt114::C0 => {
                    Err::<Adt18, Adt140>(Adt140::C3)
                }
                Adt114::C1(v4) => {
                    f677(v0, v3, v4)
                }
            }
        }
    }
}

pub fn f635() -> Str {
    Str::lit("io.modelcontextprotocol/protocolVersion")
}

pub fn f636() -> Str {
    Str::lit("io.modelcontextprotocol/clientCapabilities")
}

pub fn f637(v0: i64) -> u64 {
    let m2406 = {
        let a2404 = v0;
        let a2405 = 1i64;
        compare(a2404, a2405)
    };
    if match m2406 {
        Ordering::Less => {
            true
        }
        Ordering::Equal => {
            false
        }
        Ordering::Greater => {
            false
        }
    } {
        48u64
    } else {
        let m2409 = {
            let a2407 = v0;
            let a2408 = 2i64;
            compare(a2407, a2408)
        };
        if match m2409 {
            Ordering::Less => {
                true
            }
            Ordering::Equal => {
                false
            }
            Ordering::Greater => {
                false
            }
        } {
            49u64
        } else {
            let m2412 = {
                let a2410 = v0;
                let a2411 = 3i64;
                compare(a2410, a2411)
            };
            if match m2412 {
                Ordering::Less => {
                    true
                }
                Ordering::Equal => {
                    false
                }
                Ordering::Greater => {
                    false
                }
            } {
                50u64
            } else {
                let m2415 = {
                    let a2413 = v0;
                    let a2414 = 4i64;
                    compare(a2413, a2414)
                };
                if match m2415 {
                    Ordering::Less => {
                        true
                    }
                    Ordering::Equal => {
                        false
                    }
                    Ordering::Greater => {
                        false
                    }
                } {
                    51u64
                } else {
                    let m2418 = {
                        let a2416 = v0;
                        let a2417 = 5i64;
                        compare(a2416, a2417)
                    };
                    if match m2418 {
                        Ordering::Less => {
                            true
                        }
                        Ordering::Equal => {
                            false
                        }
                        Ordering::Greater => {
                            false
                        }
                    } {
                        52u64
                    } else {
                        let m2421 = {
                            let a2419 = v0;
                            let a2420 = 6i64;
                            compare(a2419, a2420)
                        };
                        if match m2421 {
                            Ordering::Less => {
                                true
                            }
                            Ordering::Equal => {
                                false
                            }
                            Ordering::Greater => {
                                false
                            }
                        } {
                            53u64
                        } else {
                            let m2424 = {
                                let a2422 = v0;
                                let a2423 = 7i64;
                                compare(a2422, a2423)
                            };
                            if match m2424 {
                                Ordering::Less => {
                                    true
                                }
                                Ordering::Equal => {
                                    false
                                }
                                Ordering::Greater => {
                                    false
                                }
                            } {
                                54u64
                            } else {
                                let m2427 = {
                                    let a2425 = v0;
                                    let a2426 = 8i64;
                                    compare(a2425, a2426)
                                };
                                if match m2427 {
                                    Ordering::Less => {
                                        true
                                    }
                                    Ordering::Equal => {
                                        false
                                    }
                                    Ordering::Greater => {
                                        false
                                    }
                                } {
                                    55u64
                                } else {
                                    let m2430 = {
                                        let a2428 = v0;
                                        let a2429 = 9i64;
                                        compare(a2428, a2429)
                                    };
                                    if match m2430 {
                                        Ordering::Less => {
                                            true
                                        }
                                        Ordering::Equal => {
                                            false
                                        }
                                        Ordering::Greater => {
                                            false
                                        }
                                    } {
                                        56u64
                                    } else {
                                        57u64
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f638(v0: Str, v1: Option<Adt8>) -> List<(Str, Adt6)> {
    let m2431 = v1;
    match m2431 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, f678(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f639(v0: Option<Adt9>) -> List<(Str, Adt6)> {
    let m2432 = v0;
    match m2432 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("resources"), Adt6::C5({
                let a2435 = f588(Str::lit("subscribe"), match v1.clone() {
                    Adt9::C0(h2433, _) => {
                        h2433
                    }
                });
                let a2436 = f588(Str::lit("listChanged"), match v1 {
                    Adt9::C0(_, h2434) => {
                        h2434
                    }
                });
                append_list(a2435, a2436)
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f640(v0: Adt11) -> Str {
    let m2437 = v0;
    match m2437 {
        Adt11::C0 => {
            Str::lit("public")
        }
        Adt11::C1 => {
            Str::lit("private")
        }
    }
}

pub fn f641(p2440: List<Adt37>, p2441: List<Adt6>) -> List<Adt6> {
    let m2439 = (p2440, p2441);
    {
        let mut m2439 = m2439;
        loop {
            let step = {
                let (v0, v1) = m2439;
                let m2438 = v0;
                match m2438.uncons() {
                    None => {
                        Err::<(List<Adt37>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt37>, List<Adt6>), List<Adt6>>((v3, List::cons(f679(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2439 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f642(p2445: List<Adt40>, p2446: List<Adt6>) -> R<Option<List<Adt6>>> {
    let m2444 = (p2445, p2446);
    {
        let mut m2444 = m2444;
        loop {
            let step = {
                let (v0, v1) = m2444;
                let m2442 = v0;
                match m2442.uncons() {
                    None => {
                        Err::<(List<Adt40>, List<Adt6>), R<Option<List<Adt6>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m2443 = f680(v2)?;
                        match m2443 {
                            None => {
                                Err::<(List<Adt40>, List<Adt6>), R<Option<List<Adt6>>>>(Ok(None::<List<Adt6>>))
                            }
                            Some(v4) => {
                                Ok::<(List<Adt40>, List<Adt6>), R<Option<List<Adt6>>>>((v3, List::cons(v4, v1)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2444 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f643(p2449: List<Adt46>, p2450: List<Adt6>) -> R<List<Adt6>> {
    let m2448 = (p2449, p2450);
    {
        let mut m2448 = m2448;
        loop {
            let step = {
                let (v0, v1) = m2448;
                let m2447 = v0;
                match m2447.uncons() {
                    None => {
                        Err::<(List<Adt46>, List<Adt6>), R<List<Adt6>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt46>, List<Adt6>), R<List<Adt6>>>((v3, List::cons(f681(v2)?, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2448 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f644(p2453: List<Adt48>, p2454: List<Adt6>) -> List<Adt6> {
    let m2452 = (p2453, p2454);
    {
        let mut m2452 = m2452;
        loop {
            let step = {
                let (v0, v1) = m2452;
                let m2451 = v0;
                match m2451.uncons() {
                    None => {
                        Err::<(List<Adt48>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt48>, List<Adt6>), List<Adt6>>((v3, List::cons(f682(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2452 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f645(p2457: List<Adt56>, p2458: List<Adt6>) -> List<Adt6> {
    let m2456 = (p2457, p2458);
    {
        let mut m2456 = m2456;
        loop {
            let step = {
                let (v0, v1) = m2456;
                let m2455 = v0;
                match m2455.uncons() {
                    None => {
                        Err::<(List<Adt56>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt56>, List<Adt6>), List<Adt6>>((v3, List::cons(f683(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2456 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f646(p2461: List<Adt58>, p2462: List<Adt6>) -> List<Adt6> {
    let m2460 = (p2461, p2462);
    {
        let mut m2460 = m2460;
        loop {
            let step = {
                let (v0, v1) = m2460;
                let m2459 = v0;
                match m2459.uncons() {
                    None => {
                        Err::<(List<Adt58>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt58>, List<Adt6>), List<Adt6>>((v3, List::cons(f684(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2460 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f647(p2466: List<Adt61>, p2467: List<Adt6>) -> R<Option<List<Adt6>>> {
    let m2465 = (p2466, p2467);
    {
        let mut m2465 = m2465;
        loop {
            let step = {
                let (v0, v1) = m2465;
                let m2463 = v0;
                match m2463.uncons() {
                    None => {
                        Err::<(List<Adt61>, List<Adt6>), R<Option<List<Adt6>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m2464 = f685(v2)?;
                        match m2464 {
                            None => {
                                Err::<(List<Adt61>, List<Adt6>), R<Option<List<Adt6>>>>(Ok(None::<List<Adt6>>))
                            }
                            Some(v4) => {
                                Ok::<(List<Adt61>, List<Adt6>), R<Option<List<Adt6>>>>((v3, List::cons(v4, v1)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2465 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f648(v0: Adt66) -> R<Option<Adt6>> {
    let m2468 = v0;
    match m2468 {
        Adt66::C0(v1) => {
            Ok(Some(f686(Str::lit("elicitation/create"), Some(f687(v1)))))
        }
        Adt66::C1(v2) => {
            let m2469 = f688(v2)?;
            match m2469 {
                None => {
                    Ok(None::<Adt6>)
                }
                Some(v3) => {
                    Ok(Some(f686(Str::lit("sampling/createMessage"), Some(v3))))
                }
            }
        }
        Adt66::C2(v4) => {
            Ok(Some(f686(Str::lit("roots/list"), {
                let m2470 = v4;
                match m2470 {
                    None => {
                        None::<Adt6>
                    }
                    Some(v5) => {
                        Some(Adt6::C5(match v5 {
                            Adt5::C0(h2471) => {
                                h2471
                            }
                        }))
                    }
                }
            })))
        }
    }
}

pub fn f649(v0: Option<List<Adt2>>) -> List<(Str, Adt6)> {
    let m2472 = v0;
    match m2472 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("icons"), Adt6::C4(f689(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f650() -> List<u64> {
    List::cons(67u64, List::cons(111u64, List::cons(110u64, List::cons(116u64, List::cons(101u64, List::cons(110u64, List::cons(116u64, List::cons(45u64, List::cons(84u64, List::cons(121u64, List::cons(112u64, List::cons(101u64, List::<u64>::nil()))))))))))))
}

pub fn f651() -> List<u64> {
    List::cons(67u64, List::cons(97u64, List::cons(99u64, List::cons(104u64, List::cons(101u64, List::cons(45u64, List::cons(67u64, List::cons(111u64, List::cons(110u64, List::cons(116u64, List::cons(114u64, List::cons(111u64, List::cons(108u64, List::<u64>::nil())))))))))))))
}

pub fn f652() -> List<u64> {
    List::cons(110u64, List::cons(111u64, List::cons(45u64, List::cons(99u64, List::cons(97u64, List::cons(99u64, List::cons(104u64, List::cons(101u64, List::<u64>::nil()))))))))
}

pub fn f653() -> List<u64> {
    List::cons(88u64, List::cons(45u64, List::cons(65u64, List::cons(99u64, List::cons(99u64, List::cons(101u64, List::cons(108u64, List::cons(45u64, List::cons(66u64, List::cons(117u64, List::cons(102u64, List::cons(102u64, List::cons(101u64, List::cons(114u64, List::cons(105u64, List::cons(110u64, List::cons(103u64, List::<u64>::nil())))))))))))))))))
}

pub fn f654() -> List<u64> {
    List::cons(110u64, List::cons(111u64, List::<u64>::nil()))
}

pub fn f655(p2477: List<List<u64>>, p2478: List<u64>) -> List<u64> {
    let m2476 = (p2477, p2478);
    {
        let mut m2476 = m2476;
        loop {
            let step = {
                let (v0, v1) = m2476;
                let m2473 = v0;
                match m2473.uncons() {
                    None => {
                        Err::<(List<List<u64>>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<u64>>, List<u64>), List<u64>>((v3, {
                            let a2474 = f690(v2);
                            let a2475 = v1;
                            append_list(a2474, a2475)
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m2476 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f656(v0: Str) -> R<Str> {
    let m2481 = {
        let a2480 = f94(f95(), f286(f691(f18({
            let a2479 = v0;
            utf8_encode(a2479)
        })?, List::<u64>::nil()), List::<u64>::nil()));
        utf8_decode(a2480)
    };
    match m2481 {
        None => {
            Ok(Str::lit("?"))
        }
        Some(v1) => {
            Ok(v1)
        }
    }
}

pub fn f657(p2488: List<u64>) -> bool {
    let m2487 = p2488;
    {
        let mut m2487 = m2487;
        loop {
            let step = {
                let v0 = m2487;
                let m2482 = v0;
                match m2482.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let a2483 = v1;
                        let a2484 = 48u64;
                        if if nat_eq(a2483, a2484) {
                            true
                        } else {
                            let a2485 = v1;
                            let a2486 = 46u64;
                            nat_eq(a2485, a2486)
                        } {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2487 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f658(v0: Adt148) -> bool {
    let m2489 = v0;
    match m2489 {
        Adt148::C0 => {
            true
        }
        Adt148::C1 => {
            false
        }
        Adt148::C2 => {
            false
        }
        Adt148::C3 => {
            false
        }
        Adt148::C4 => {
            false
        }
        Adt148::C5 => {
            false
        }
        Adt148::C6 => {
            false
        }
        Adt148::C7 => {
            false
        }
        Adt148::C8 => {
            false
        }
        Adt148::C9 => {
            false
        }
        Adt148::C10 => {
            false
        }
        Adt148::C11 => {
            false
        }
    }
}

pub fn f659(v0: Adt16) -> Adt148 {
    let m2490 = v0;
    match m2490 {
        Adt16::C0(_) => {
            Adt148::C0
        }
        Adt16::C1(_) => {
            Adt148::C1
        }
        Adt16::C2(_) => {
            Adt148::C2
        }
        Adt16::C3(_) => {
            Adt148::C3
        }
        Adt16::C4(_) => {
            Adt148::C4
        }
        Adt16::C5(_) => {
            Adt148::C5
        }
        Adt16::C6(_) => {
            Adt148::C6
        }
        Adt16::C7(_) => {
            Adt148::C7
        }
        Adt16::C8(_) => {
            Adt148::C8
        }
        Adt16::C9(_) => {
            Adt148::C9
        }
        Adt16::C10(_) => {
            Adt148::C10
        }
        Adt16::C11(_) => {
            Adt148::C11
        }
    }
}

pub fn f660(v0: Adt0, v1: Adt34) -> Adt136 {
    Adt136::C0(v1, Adt137::C0(Some(match v0 {
        Adt0::C0(h2491, _, _, _, _, _, _) => {
            h2491
        }
    }), None::<Adt5>), List::<Adt138>::nil())
}

pub fn f661(v0: Adt0) -> Adt35 {
    Adt35::C0(match v0.clone() {
        Adt0::C0(_, _, _, h2492, _, _, _) => {
            h2492
        }
    }, f297(), match v0.clone() {
        Adt0::C0(_, _, h2493, _, _, _, _) => {
            h2493
        }
    }, match v0 {
        Adt0::C0(_, h2494, _, _, _, _, _) => {
            h2494
        }
    })
}

pub fn f662(v0: Adt0, v1: Adt16, v2: Result<Adt34, Adt86>) -> Result<Adt136, Adt139> {
    let m2495 = v2;
    match m2495 {
        Ok(v4) => {
            if f692(f659(v1.clone()), v4.clone()) {
                f693(v0, v1, v4)
            } else {
                Err::<Adt136, Adt139>(Adt139::C4(f694()))
            }
        }
        Err(v3) => {
            Err::<Adt136, Adt139>(Adt139::C4(v3))
        }
    }
}

pub fn f663(v0: Bytes, v1: u64, v2: u64, v3: Fn2, v4: Adt16) -> R<Result<Adt34, Adt86>> {
    f695(v0.clone(), v1, v2, v3, v4.clone(), f696(v0, v4)?)
}

pub fn f664(v0: Str, v1: Adt5) -> Result<Option<Str>, Adt140> {
    let m2496 = f184(v0.clone(), v1);
    match m2496 {
        Adt114::C0 => {
            Ok::<Option<Str>, Adt140>(None::<Str>)
        }
        Adt114::C1(v2) => {
            let m2497 = f130(v2);
            match m2497 {
                None => {
                    Err::<Option<Str>, Adt140>(Adt140::C8(v0))
                }
                Some(v3) => {
                    Ok::<Option<Str>, Adt140>(Some(v3))
                }
            }
        }
    }
}

pub fn f665(v0: Str, v1: Adt5) -> Result<Str, Adt140> {
    let m2498 = f184(v0.clone(), v1);
    match m2498 {
        Adt114::C0 => {
            Err::<Str, Adt140>(Adt140::C7(v0))
        }
        Adt114::C1(v2) => {
            let m2499 = f130(v2);
            match m2499 {
                None => {
                    Err::<Str, Adt140>(Adt140::C8(v0))
                }
                Some(v3) => {
                    Ok::<Str, Adt140>(v3)
                }
            }
        }
    }
}

pub fn f666(v0: Str, v1: Adt5) -> Result<Option<Adt5>, Adt140> {
    let m2500 = f184(v0.clone(), v1);
    match m2500 {
        Adt114::C0 => {
            Ok::<Option<Adt5>, Adt140>(None::<Adt5>)
        }
        Adt114::C1(v2) => {
            let m2501 = f195(v2);
            match m2501 {
                None => {
                    Err::<Option<Adt5>, Adt140>(Adt140::C8(v0))
                }
                Some(v3) => {
                    Ok::<Option<Adt5>, Adt140>(Some(v3))
                }
            }
        }
    }
}

pub fn f667(v0: Str, v1: Adt5) -> Result<Option<List<(Str, Str)>>, Adt140> {
    let m2502 = f184(v0.clone(), v1);
    match m2502 {
        Adt114::C0 => {
            Ok::<Option<List<(Str, Str)>>, Adt140>(None::<List<(Str, Str)>>)
        }
        Adt114::C1(v2) => {
            let m2503 = f195(v2);
            match m2503 {
                None => {
                    Err::<Option<List<(Str, Str)>>, Adt140>(Adt140::C8(v0))
                }
                Some(v3) => {
                    let m2505 = f697(v0, match v3 {
                        Adt5::C0(h2504) => {
                            h2504
                        }
                    });
                    match m2505 {
                        Ok(v4) => {
                            Ok::<Option<List<(Str, Str)>>, Adt140>(Some(v4))
                        }
                        Err(v5) => {
                            Err::<Option<List<(Str, Str)>>, Adt140>(v5)
                        }
                    }
                }
            }
        }
    }
}

pub fn f668(v0: Str, v1: Adt5) -> Result<Adt5, Adt140> {
    let m2506 = f184(v0.clone(), v1);
    match m2506 {
        Adt114::C0 => {
            Err::<Adt5, Adt140>(Adt140::C7(v0))
        }
        Adt114::C1(v2) => {
            let m2507 = f195(v2);
            match m2507 {
                None => {
                    Err::<Adt5, Adt140>(Adt140::C8(v0))
                }
                Some(v3) => {
                    Ok::<Adt5, Adt140>(v3)
                }
            }
        }
    }
}

pub fn f669(v0: Adt5) -> Result<Adt27, Adt140> {
    let m2508 = f665(Str::lit("type"), v0.clone());
    match m2508 {
        Ok(v1) => {
            let a2509 = v1.clone();
            let a2510 = Str::lit("ref/prompt");
            if equal(a2509, a2510) {
                let m2511 = f665(Str::lit("name"), v0.clone());
                match m2511 {
                    Ok(v2) => {
                        let m2512 = f664(Str::lit("title"), v0);
                        match m2512 {
                            Ok(v3) => {
                                Ok::<Adt27, Adt140>(Adt27::C0(Adt28::C0(v2, v3)))
                            }
                            Err(v4) => {
                                Err::<Adt27, Adt140>(v4)
                            }
                        }
                    }
                    Err(v5) => {
                        Err::<Adt27, Adt140>(v5)
                    }
                }
            } else {
                let a2513 = v1;
                let a2514 = Str::lit("ref/resource");
                if equal(a2513, a2514) {
                    let m2515 = f665(Str::lit("uri"), v0);
                    match m2515 {
                        Ok(v6) => {
                            Ok::<Adt27, Adt140>(Adt27::C1(v6))
                        }
                        Err(v7) => {
                            Err::<Adt27, Adt140>(v7)
                        }
                    }
                } else {
                    Err::<Adt27, Adt140>(Adt140::C8(Str::lit("type")))
                }
            }
        }
        Err(v8) => {
            Err::<Adt27, Adt140>(v8)
        }
    }
}

pub fn f670(v0: Adt5) -> Result<Adt29, Adt140> {
    let m2516 = f665(Str::lit("name"), v0.clone());
    match m2516 {
        Ok(v1) => {
            let m2517 = f665(Str::lit("value"), v0);
            match m2517 {
                Ok(v2) => {
                    Ok::<Adt29, Adt140>(Adt29::C0(v1, v2))
                }
                Err(v3) => {
                    Err::<Adt29, Adt140>(v3)
                }
            }
        }
        Err(v4) => {
            Err::<Adt29, Adt140>(v4)
        }
    }
}

pub fn f671(v0: Adt5) -> Result<Option<Adt30>, Adt140> {
    let m2518 = f184(Str::lit("context"), v0);
    match m2518 {
        Adt114::C0 => {
            Ok::<Option<Adt30>, Adt140>(None::<Adt30>)
        }
        Adt114::C1(v1) => {
            let m2519 = f195(v1);
            match m2519 {
                None => {
                    Err::<Option<Adt30>, Adt140>(Adt140::C8(Str::lit("context")))
                }
                Some(v2) => {
                    let m2520 = f667(Str::lit("arguments"), v2);
                    match m2520 {
                        Ok(v3) => {
                            Ok::<Option<Adt30>, Adt140>(Some(Adt30::C0(v3)))
                        }
                        Err(v4) => {
                            Err::<Option<Adt30>, Adt140>(v4)
                        }
                    }
                }
            }
        }
    }
}

pub fn f672(v0: bool, v1: Str, v2: List<Str>) -> Option<Adt139> {
    if v0 {
        None::<Adt139>
    } else {
        Some(Adt139::C1(v1, v2))
    }
}

pub fn f673(v0: Str, p2525: List<Str>) -> bool {
    let m2524 = p2525;
    {
        let mut m2524 = m2524;
        loop {
            let step = {
                let v1 = m2524;
                let m2521 = v1;
                match m2521.uncons() {
                    None => {
                        Err::<List<Str>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a2522 = v2;
                        let a2523 = v0.clone();
                        if equal(a2522, a2523) {
                            Err::<List<Str>, bool>(true)
                        } else {
                            Ok::<List<Str>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2524 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f674(v0: Option<Adt127>, v1: List<Adt129>, v2: Adt5) -> R<Option<Adt127>> {
    let m2526 = v0;
    match m2526 {
        None => {
            f698(v1, f699(v2))
        }
        Some(v3) => {
            Ok(Some(v3))
        }
    }
}

pub fn f675(v0: Option<Str>, v1: Adt152, v2: Adt5) -> Option<Adt127> {
    let m2527 = v1.clone();
    match m2527 {
        Adt152::C0 => {
            None::<Adt127>
        }
        Adt152::C1 => {
            f700(v0, f701(v1, v2))
        }
        Adt152::C2 => {
            f700(v0, f701(v1, v2))
        }
        Adt152::C3 => {
            f700(v0, f701(v1, v2))
        }
    }
}

pub fn f676(v0: Adt114, v1: Adt114) -> Adt156 {
    let m2528 = v0;
    match m2528 {
        Adt114::C0 => {
            Adt156::C1
        }
        Adt114::C1(_) => {
            let m2529 = v1;
            match m2529 {
                Adt114::C0 => {
                    Adt156::C2
                }
                Adt114::C1(_) => {
                    Adt156::C0
                }
            }
        }
    }
}

pub fn f677(v0: Adt5, v1: Adt6, v2: Adt6) -> Result<Adt18, Adt140> {
    let m2530 = f130(v1);
    match m2530 {
        None => {
            Err::<Adt18, Adt140>(Adt140::C5)
        }
        Some(v3) => {
            let m2531 = f702(v2);
            match m2531 {
                None => {
                    Err::<Adt18, Adt140>(Adt140::C6)
                }
                Some(v4) => {
                    let m2532 = f666(f703(), v0.clone());
                    match m2532 {
                        Ok(v6) => {
                            let m2533 = f704(v0.clone());
                            match m2533 {
                                Ok(v8) => {
                                    let m2534 = f705(v0.clone());
                                    match m2534 {
                                        Ok(v10) => {
                                            Ok::<Adt18, Adt140>(Adt18::C0(v3, v4, v6, v8, v10, v0))
                                        }
                                        Err(v9) => {
                                            Err::<Adt18, Adt140>(v9)
                                        }
                                    }
                                }
                                Err(v7) => {
                                    Err::<Adt18, Adt140>(v7)
                                }
                            }
                        }
                        Err(v5) => {
                            Err::<Adt18, Adt140>(v5)
                        }
                    }
                }
            }
        }
    }
}

pub fn f678(v0: Adt8) -> Adt6 {
    Adt6::C5(f588(Str::lit("listChanged"), match v0 {
        Adt8::C0(h2535) => {
            h2535
        }
    }))
}

pub fn f679(v0: Adt37) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
        Adt37::C0(h2536, _, _, _, _, _, _, _) => {
            h2536
        }
    })), {
        let a2554 = f583(Str::lit("title"), match v0.clone() {
            Adt37::C0(_, h2537, _, _, _, _, _, _) => {
                h2537
            }
        });
        let a2555 = {
            let a2552 = f649(match v0.clone() {
                Adt37::C0(_, _, h2538, _, _, _, _, _) => {
                    h2538
                }
            });
            let a2553 = {
                let a2550 = f583(Str::lit("description"), match v0.clone() {
                    Adt37::C0(_, _, _, h2539, _, _, _, _) => {
                        h2539
                    }
                });
                let a2551 = List::cons((Str::lit("inputSchema"), Adt6::C5({
                    let m2542 = match v0.clone() {
                        Adt37::C0(_, _, _, _, h2541, _, _, _) => {
                            h2541
                        }
                    };
                    match m2542 {
                        Adt5::C0(h2540) => {
                            h2540
                        }
                    }
                })), {
                    let a2548 = f614(Str::lit("outputSchema"), match v0.clone() {
                        Adt37::C0(_, _, _, _, _, h2543, _, _) => {
                            h2543
                        }
                    });
                    let a2549 = {
                        let a2546 = f706(match v0.clone() {
                            Adt37::C0(_, _, _, _, _, _, h2544, _) => {
                                h2544
                            }
                        });
                        let a2547 = f707(match v0 {
                            Adt37::C0(_, _, _, _, _, _, _, h2545) => {
                                h2545
                            }
                        });
                        append_list(a2546, a2547)
                    };
                    append_list(a2548, a2549)
                });
                append_list(a2550, a2551)
            };
            append_list(a2552, a2553)
        };
        append_list(a2554, a2555)
    }))
}

pub fn f680(v0: Adt40) -> R<Option<Adt6>> {
    let m2556 = v0;
    match m2556 {
        Adt40::C0(v1) => {
            Ok(Some(f708(v1)))
        }
        Adt40::C1(v2) => {
            Ok(Some(f709(Str::lit("image"), match v2.clone() {
                Adt44::C0(h2557, _, _, _) => {
                    h2557
                }
            }, match v2.clone() {
                Adt44::C0(_, h2558, _, _) => {
                    h2558
                }
            }, match v2.clone() {
                Adt44::C0(_, _, h2559, _) => {
                    h2559
                }
            }, match v2 {
                Adt44::C0(_, _, _, h2560) => {
                    h2560
                }
            })))
        }
        Adt40::C2(v3) => {
            Ok(Some(f709(Str::lit("audio"), match v3.clone() {
                Adt45::C0(h2561, _, _, _) => {
                    h2561
                }
            }, match v3.clone() {
                Adt45::C0(_, h2562, _, _) => {
                    h2562
                }
            }, match v3.clone() {
                Adt45::C0(_, _, h2563, _) => {
                    h2563
                }
            }, match v3 {
                Adt45::C0(_, _, _, h2564) => {
                    h2564
                }
            })))
        }
        Adt40::C3(v4) => {
            Ok(Some(f710(v4)?))
        }
        Adt40::C4(v5) => {
            Ok(Some(f711(v5)))
        }
    }
}

pub fn f681(v0: Adt46) -> R<Adt6> {
    Ok(Adt6::C5({
        let a2588 = List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt46::C0(h2565, _, _, _, _, _, _, _, _) => {
                h2565
            }
        })), List::<(Str, Adt6)>::nil());
        let a2589 = {
            let a2586 = f583(Str::lit("title"), match v0.clone() {
                Adt46::C0(_, h2566, _, _, _, _, _, _, _) => {
                    h2566
                }
            });
            let a2587 = {
                let a2584 = f649(match v0.clone() {
                    Adt46::C0(_, _, h2567, _, _, _, _, _, _) => {
                        h2567
                    }
                });
                let a2585 = {
                    let a2582 = List::cons((Str::lit("uri"), Adt6::C3(match v0.clone() {
                        Adt46::C0(_, _, _, h2568, _, _, _, _, _) => {
                            h2568
                        }
                    })), List::<(Str, Adt6)>::nil());
                    let a2583 = {
                        let a2580 = f583(Str::lit("description"), match v0.clone() {
                            Adt46::C0(_, _, _, _, h2569, _, _, _, _) => {
                                h2569
                            }
                        });
                        let a2581 = {
                            let a2578 = f583(Str::lit("mimeType"), match v0.clone() {
                                Adt46::C0(_, _, _, _, _, h2570, _, _, _) => {
                                    h2570
                                }
                            });
                            let a2579 = {
                                let a2576 = f712(match v0.clone() {
                                    Adt46::C0(_, _, _, _, _, _, h2571, _, _) => {
                                        h2571
                                    }
                                });
                                let a2577 = {
                                    let a2574 = f713(match v0.clone() {
                                        Adt46::C0(_, _, _, _, _, _, _, h2572, _) => {
                                            h2572
                                        }
                                    })?;
                                    let a2575 = f707(match v0 {
                                        Adt46::C0(_, _, _, _, _, _, _, _, h2573) => {
                                            h2573
                                        }
                                    });
                                    append_list(a2574, a2575)
                                };
                                append_list(a2576, a2577)
                            };
                            append_list(a2578, a2579)
                        };
                        append_list(a2580, a2581)
                    };
                    append_list(a2582, a2583)
                };
                append_list(a2584, a2585)
            };
            append_list(a2586, a2587)
        };
        append_list(a2588, a2589)
    }))
}

pub fn f682(v0: Adt48) -> Adt6 {
    let m2590 = v0;
    match m2590 {
        Adt48::C0(v1) => {
            Adt6::C5(List::cons((Str::lit("uri"), Adt6::C3(match v1.clone() {
                Adt49::C0(h2591, _, _, _) => {
                    h2591
                }
            })), {
                let a2595 = f583(Str::lit("mimeType"), match v1.clone() {
                    Adt49::C0(_, h2592, _, _) => {
                        h2592
                    }
                });
                let a2596 = List::cons((Str::lit("text"), Adt6::C3(match v1.clone() {
                    Adt49::C0(_, _, h2593, _) => {
                        h2593
                    }
                })), f707(match v1 {
                    Adt49::C0(_, _, _, h2594) => {
                        h2594
                    }
                }));
                append_list(a2595, a2596)
            }))
        }
        Adt48::C1(v2) => {
            Adt6::C5(List::cons((Str::lit("uri"), Adt6::C3(match v2.clone() {
                Adt50::C0(h2597, _, _, _) => {
                    h2597
                }
            })), {
                let a2601 = f583(Str::lit("mimeType"), match v2.clone() {
                    Adt50::C0(_, h2598, _, _) => {
                        h2598
                    }
                });
                let a2602 = List::cons((Str::lit("blob"), Adt6::C3(match v2.clone() {
                    Adt50::C0(_, _, h2599, _) => {
                        h2599
                    }
                })), f707(match v2 {
                    Adt50::C0(_, _, _, h2600) => {
                        h2600
                    }
                }));
                append_list(a2601, a2602)
            }))
        }
    }
}

pub fn f683(v0: Adt56) -> Adt6 {
    Adt6::C5({
        let a2623 = List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt56::C0(h2603, _, _, _, _, _, _, _) => {
                h2603
            }
        })), List::<(Str, Adt6)>::nil());
        let a2624 = {
            let a2621 = f583(Str::lit("title"), match v0.clone() {
                Adt56::C0(_, h2604, _, _, _, _, _, _) => {
                    h2604
                }
            });
            let a2622 = {
                let a2619 = f649(match v0.clone() {
                    Adt56::C0(_, _, h2605, _, _, _, _, _) => {
                        h2605
                    }
                });
                let a2620 = {
                    let a2617 = List::cons((Str::lit("uriTemplate"), Adt6::C3(match v0.clone() {
                        Adt56::C0(_, _, _, h2606, _, _, _, _) => {
                            h2606
                        }
                    })), List::<(Str, Adt6)>::nil());
                    let a2618 = {
                        let a2615 = f583(Str::lit("description"), match v0.clone() {
                            Adt56::C0(_, _, _, _, h2607, _, _, _) => {
                                h2607
                            }
                        });
                        let a2616 = {
                            let a2613 = f583(Str::lit("mimeType"), match v0.clone() {
                                Adt56::C0(_, _, _, _, _, h2608, _, _) => {
                                    h2608
                                }
                            });
                            let a2614 = {
                                let a2611 = f712(match v0.clone() {
                                    Adt56::C0(_, _, _, _, _, _, h2609, _) => {
                                        h2609
                                    }
                                });
                                let a2612 = f707(match v0 {
                                    Adt56::C0(_, _, _, _, _, _, _, h2610) => {
                                        h2610
                                    }
                                });
                                append_list(a2611, a2612)
                            };
                            append_list(a2613, a2614)
                        };
                        append_list(a2615, a2616)
                    };
                    append_list(a2617, a2618)
                };
                append_list(a2619, a2620)
            };
            append_list(a2621, a2622)
        };
        append_list(a2623, a2624)
    })
}

pub fn f684(v0: Adt58) -> Adt6 {
    Adt6::C5({
        let a2640 = List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt58::C0(h2625, _, _, _, _, _) => {
                h2625
            }
        })), List::<(Str, Adt6)>::nil());
        let a2641 = {
            let a2638 = f583(Str::lit("title"), match v0.clone() {
                Adt58::C0(_, h2626, _, _, _, _) => {
                    h2626
                }
            });
            let a2639 = {
                let a2636 = f649(match v0.clone() {
                    Adt58::C0(_, _, h2627, _, _, _) => {
                        h2627
                    }
                });
                let a2637 = {
                    let a2634 = f583(Str::lit("description"), match v0.clone() {
                        Adt58::C0(_, _, _, h2628, _, _) => {
                            h2628
                        }
                    });
                    let a2635 = {
                        let a2632 = {
                            let m2630 = match v0.clone() {
                                Adt58::C0(_, _, _, _, h2629, _) => {
                                    h2629
                                }
                            };
                            match m2630 {
                                None => {
                                    List::<(Str, Adt6)>::nil()
                                }
                                Some(v1) => {
                                    List::cons((Str::lit("arguments"), Adt6::C4(f714(v1))), List::<(Str, Adt6)>::nil())
                                }
                            }
                        };
                        let a2633 = f707(match v0 {
                            Adt58::C0(_, _, _, _, _, h2631) => {
                                h2631
                            }
                        });
                        append_list(a2632, a2633)
                    };
                    append_list(a2634, a2635)
                };
                append_list(a2636, a2637)
            };
            append_list(a2638, a2639)
        };
        append_list(a2640, a2641)
    })
}

pub fn f685(v0: Adt61) -> R<Option<Adt6>> {
    let m2643 = f680(match v0.clone() {
        Adt61::C0(_, h2642) => {
            h2642
        }
    })?;
    match m2643 {
        None => {
            Ok(None::<Adt6>)
        }
        Some(v1) => {
            Ok(Some(Adt6::C5(List::cons((Str::lit("role"), Adt6::C3(f715(match v0 {
                Adt61::C0(h2644, _) => {
                    h2644
                }
            }))), List::cons((Str::lit("content"), v1), List::<(Str, Adt6)>::nil())))))
        }
    }
}

pub fn f686(v0: Str, v1: Option<Adt6>) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("method"), Adt6::C3(v0)), f587(Str::lit("params"), v1)))
}

pub fn f687(v0: Adt67) -> Adt6 {
    let m2645 = v0;
    match m2645 {
        Adt67::C0(v1) => {
            f716(v1)
        }
        Adt67::C1(v2) => {
            Adt6::C5(List::cons((Str::lit("mode"), Adt6::C3(Str::lit("url"))), List::cons((Str::lit("message"), Adt6::C3(match v2.clone() {
                Adt70::C0(h2646, _) => {
                    h2646
                }
            })), List::cons((Str::lit("url"), Adt6::C3(match v2 {
                Adt70::C0(_, h2647) => {
                    h2647
                }
            })), List::<(Str, Adt6)>::nil()))))
        }
    }
}

pub fn f688(v0: Adt71) -> R<Option<Adt6>> {
    let m2649 = f717(match v0.clone() {
        Adt71::C0(h2648, _, _, _, _, _, _, _, _, _) => {
            h2648
        }
    })?;
    match m2649 {
        None => {
            Ok(None::<Adt6>)
        }
        Some(v1) => {
            Ok(Some(Adt6::C5(List::cons((Str::lit("messages"), Adt6::C4(v1)), {
                let a2651 = f718(v0.clone());
                let a2652 = List::cons((Str::lit("maxTokens"), f52(match v0.clone() {
                    Adt71::C0(_, _, _, _, _, h2650, _, _, _, _) => {
                        h2650
                    }
                })?), f719(v0));
                append_list(a2651, a2652)
            }))))
        }
    }
}

pub fn f689(v0: List<Adt2>) -> List<Adt6> {
    f404(f720(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f690(v0: List<u64>) -> List<u64> {
    let a2655 = f721();
    let a2656 = {
        let a2653 = v0;
        let a2654 = f722();
        append_list(a2653, a2654)
    };
    append_list(a2655, a2656)
}

pub fn f691(v0: List<u64>, v1: List<u64>) -> List<u64> {
    f723(v0, 0u64, v1)
}

pub fn f692(v0: Adt148, v1: Adt34) -> bool {
    let m2657 = v1;
    match m2657 {
        Adt34::C0(_) => {
            f724(v0, Adt148::C0)
        }
        Adt34::C1(_) => {
            f724(v0, Adt148::C1)
        }
        Adt34::C2(_) => {
            f724(v0, Adt148::C2)
        }
        Adt34::C3(_) => {
            f724(v0, Adt148::C2)
        }
        Adt34::C4(_) => {
            f724(v0, Adt148::C3)
        }
        Adt34::C5(_) => {
            f724(v0, Adt148::C4)
        }
        Adt34::C6(_) => {
            f724(v0, Adt148::C5)
        }
        Adt34::C7(_) => {
            f724(v0, Adt148::C6)
        }
        Adt34::C8(_) => {
            f724(v0, Adt148::C7)
        }
        Adt34::C9(_) => {
            f724(v0, Adt148::C8)
        }
        Adt34::C10(_) => {
            f724(v0, Adt148::C9)
        }
        Adt34::C11 => {
            if f724(v0.clone(), Adt148::C10) {
                true
            } else {
                f724(v0, Adt148::C11)
            }
        }
        Adt34::C12(_) => {
            f725(v0)
        }
        Adt34::C13(_) => {
            f726(v0)
        }
    }
}

pub fn f693(v0: Adt0, v1: Adt16, v2: Adt34) -> Result<Adt136, Adt139> {
    let m2658 = v2.clone();
    match m2658 {
        Adt34::C0(_) => {
            Ok::<Adt136, Adt139>(f660(v0, v2))
        }
        Adt34::C1(_) => {
            Ok::<Adt136, Adt139>(f660(v0, v2))
        }
        Adt34::C2(_) => {
            Ok::<Adt136, Adt139>(f660(v0, v2))
        }
        Adt34::C3(v8) => {
            Ok::<Adt136, Adt139>(f730(v0, f728(v1), v8))
        }
        Adt34::C4(_) => {
            Ok::<Adt136, Adt139>(f660(v0, v2))
        }
        Adt34::C5(_) => {
            Ok::<Adt136, Adt139>(f660(v0, v2))
        }
        Adt34::C6(_) => {
            Ok::<Adt136, Adt139>(f660(v0, v2))
        }
        Adt34::C7(_) => {
            Ok::<Adt136, Adt139>(f660(v0, v2))
        }
        Adt34::C8(_) => {
            Ok::<Adt136, Adt139>(f660(v0, v2))
        }
        Adt34::C9(_) => {
            Ok::<Adt136, Adt139>(f660(v0, v2))
        }
        Adt34::C10(_) => {
            Ok::<Adt136, Adt139>(f660(v0, v2))
        }
        Adt34::C11 => {
            Ok::<Adt136, Adt139>(f660(v0, v2))
        }
        Adt34::C12(v3) => {
            f727(v0, f728(v1), v3)
        }
        Adt34::C13(v4) => {
            f729(v0, f728(v1), v4)
        }
    }
}

pub fn f694() -> Adt86 {
    Adt86::C9(Str::lit("the provider answered with a result that does not belong to the method"))
}

pub fn f695(v0: Bytes, v1: u64, v2: u64, v3: Fn2, v4: Adt16, v5: Result<Adt16, Adt86>) -> R<Result<Adt34, Adt86>> {
    let m2659 = v5;
    match m2659 {
        Ok(v7) => {
            f731(v0, v4, f732(v1, v2, v3, v7)?)
        }
        Err(v6) => {
            Ok(Err::<Adt34, Adt86>(v6))
        }
    }
}

pub fn f696(v0: Bytes, v1: Adt16) -> R<Result<Adt16, Adt86>> {
    let m2660 = v1.clone();
    match m2660 {
        Adt16::C0(_) => {
            Ok(Ok::<Adt16, Adt86>(v1))
        }
        Adt16::C1(_) => {
            Ok(Ok::<Adt16, Adt86>(v1))
        }
        Adt16::C2(v2) => {
            f733(v0, v2)
        }
        Adt16::C3(_) => {
            Ok(Ok::<Adt16, Adt86>(v1))
        }
        Adt16::C4(v3) => {
            f734(v0, v3)
        }
        Adt16::C5(_) => {
            Ok(Ok::<Adt16, Adt86>(v1))
        }
        Adt16::C6(_) => {
            Ok(Ok::<Adt16, Adt86>(v1))
        }
        Adt16::C7(v4) => {
            f735(v0, v4)
        }
        Adt16::C8(_) => {
            Ok(Ok::<Adt16, Adt86>(v1))
        }
        Adt16::C9(_) => {
            Ok(Ok::<Adt16, Adt86>(v1))
        }
        Adt16::C10(_) => {
            Ok(Ok::<Adt16, Adt86>(v1))
        }
        Adt16::C11(_) => {
            Ok(Ok::<Adt16, Adt86>(v1))
        }
    }
}

pub fn f697(v0: Str, v1: List<(Str, Adt6)>) -> Result<List<(Str, Str)>, Adt140> {
    let m2661 = f736(v0, v1, List::<(Str, Str)>::nil());
    match m2661 {
        Ok(v2) => {
            Ok::<List<(Str, Str)>, Adt140>(f737(v2, List::<(Str, Str)>::nil()))
        }
        Err(v3) => {
            Err::<List<(Str, Str)>, Adt140>(v3)
        }
    }
}

pub fn f698(p2665: List<Adt129>, v1: Adt5) -> R<Option<Adt127>> {
    let m2664 = p2665;
    {
        let mut m2664 = m2664;
        loop {
            let step = {
                let v0 = m2664;
                let m2662 = v0;
                match m2662.uncons() {
                    None => {
                        Err::<List<Adt129>, R<Option<Adt127>>>(Ok(None::<Adt127>))
                    }
                    Some((v2, v3)) => {
                        let m2663 = f738(v2, v1.clone())?;
                        match m2663 {
                            None => {
                                Ok::<List<Adt129>, R<Option<Adt127>>>(v3)
                            }
                            Some(v4) => {
                                Err::<List<Adt129>, R<Option<Adt127>>>(Ok(Some(v4)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2664 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f699(v0: Adt5) -> Adt5 {
    let m2666 = f184(Str::lit("arguments"), v0);
    match m2666 {
        Adt114::C0 => {
            f632()
        }
        Adt114::C1(v1) => {
            let m2667 = f195(v1);
            match m2667 {
                None => {
                    f632()
                }
                Some(v2) => {
                    v2
                }
            }
        }
    }
}

pub fn f700(v0: Option<Str>, v1: Option<Str>) -> Option<Adt127> {
    let m2668 = v0;
    match m2668 {
        None => {
            Some(Adt127::C2)
        }
        Some(v2) => {
            let m2669 = v1;
            match m2669 {
                None => {
                    None::<Adt127>
                }
                Some(v3) => {
                    let a2670 = v2;
                    let a2671 = v3;
                    if equal(a2670, a2671) {
                        None::<Adt127>
                    } else {
                        Some(Adt127::C5)
                    }
                }
            }
        }
    }
}

pub fn f701(v0: Adt152, v1: Adt5) -> Option<Str> {
    let m2672 = v0;
    match m2672 {
        Adt152::C0 => {
            None::<Str>
        }
        Adt152::C1 => {
            let m2673 = f184(Str::lit("name"), v1);
            match m2673 {
                Adt114::C0 => {
                    None::<Str>
                }
                Adt114::C1(v2) => {
                    f130(v2)
                }
            }
        }
        Adt152::C2 => {
            let m2674 = f184(Str::lit("uri"), v1);
            match m2674 {
                Adt114::C0 => {
                    None::<Str>
                }
                Adt114::C1(v3) => {
                    f130(v3)
                }
            }
        }
        Adt152::C3 => {
            let m2675 = f184(Str::lit("taskId"), v1);
            match m2675 {
                Adt114::C0 => {
                    None::<Str>
                }
                Adt114::C1(v4) => {
                    f130(v4)
                }
            }
        }
    }
}

pub fn f702(v0: Adt6) -> Option<Adt19> {
    let m2676 = f195(v0);
    match m2676 {
        None => {
            None::<Adt19>
        }
        Some(v1) => {
            let m2677 = f739(Str::lit("experimental"), v1.clone());
            match m2677 {
                None => {
                    None::<Adt19>
                }
                Some(v2) => {
                    let m2678 = f739(Str::lit("roots"), v1.clone());
                    match m2678 {
                        None => {
                            None::<Adt19>
                        }
                        Some(v3) => {
                            let m2679 = f739(Str::lit("sampling"), v1.clone());
                            match m2679 {
                                None => {
                                    None::<Adt19>
                                }
                                Some(v4) => {
                                    let m2680 = f739(Str::lit("elicitation"), v1.clone());
                                    match m2680 {
                                        None => {
                                            None::<Adt19>
                                        }
                                        Some(v5) => {
                                            let m2681 = f739(Str::lit("extensions"), v1);
                                            match m2681 {
                                                None => {
                                                    None::<Adt19>
                                                }
                                                Some(v6) => {
                                                    Some(Adt19::C0(v2, v3, v4, v5, v6))
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f703() -> Str {
    Str::lit("io.modelcontextprotocol/clientInfo")
}

pub fn f704(v0: Adt5) -> Result<Option<Adt20>, Adt140> {
    let m2682 = f664(f740(), v0);
    match m2682 {
        Ok(v2) => {
            let m2683 = v2;
            match m2683 {
                None => {
                    Ok::<Option<Adt20>, Adt140>(None::<Adt20>)
                }
                Some(v3) => {
                    let m2684 = f741(v3);
                    match m2684 {
                        Adt158::C0 => {
                            Err::<Option<Adt20>, Adt140>(Adt140::C8(f740()))
                        }
                        Adt158::C1(v4) => {
                            Ok::<Option<Adt20>, Adt140>(Some(v4))
                        }
                    }
                }
            }
        }
        Err(v1) => {
            Err::<Option<Adt20>, Adt140>(v1)
        }
    }
}

pub fn f705(v0: Adt5) -> Result<Option<Adt21>, Adt140> {
    let m2685 = f184(f742(), v0);
    match m2685 {
        Adt114::C0 => {
            Ok::<Option<Adt21>, Adt140>(None::<Adt21>)
        }
        Adt114::C1(v1) => {
            let m2686 = f130(v1.clone());
            match m2686 {
                None => {
                    let m2687 = f459(v1);
                    match m2687 {
                        None => {
                            Err::<Option<Adt21>, Adt140>(Adt140::C8(f742()))
                        }
                        Some(v3) => {
                            Ok::<Option<Adt21>, Adt140>(Some(Adt21::C1(v3)))
                        }
                    }
                }
                Some(v2) => {
                    Ok::<Option<Adt21>, Adt140>(Some(Adt21::C0(v2)))
                }
            }
        }
    }
}

pub fn f706(v0: Option<Adt38>) -> List<(Str, Adt6)> {
    let m2688 = v0;
    match m2688 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("annotations"), Adt6::C5({
                let a2700 = f583(Str::lit("title"), match v1.clone() {
                    Adt38::C0(h2689, _, _, _, _) => {
                        h2689
                    }
                });
                let a2701 = {
                    let a2698 = f588(Str::lit("readOnlyHint"), match v1.clone() {
                        Adt38::C0(_, h2690, _, _, _) => {
                            h2690
                        }
                    });
                    let a2699 = {
                        let a2696 = f588(Str::lit("destructiveHint"), match v1.clone() {
                            Adt38::C0(_, _, h2691, _, _) => {
                                h2691
                            }
                        });
                        let a2697 = {
                            let a2694 = f588(Str::lit("idempotentHint"), match v1.clone() {
                                Adt38::C0(_, _, _, h2692, _) => {
                                    h2692
                                }
                            });
                            let a2695 = f588(Str::lit("openWorldHint"), match v1 {
                                Adt38::C0(_, _, _, _, h2693) => {
                                    h2693
                                }
                            });
                            append_list(a2694, a2695)
                        };
                        append_list(a2696, a2697)
                    };
                    append_list(a2698, a2699)
                };
                append_list(a2700, a2701)
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f707(v0: Option<Adt5>) -> List<(Str, Adt6)> {
    f614(Str::lit("_meta"), v0)
}

pub fn f708(v0: Adt41) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("text"))), List::cons((Str::lit("text"), Adt6::C3(match v0.clone() {
        Adt41::C0(h2702, _, _) => {
            h2702
        }
    })), {
        let a2705 = f712(match v0.clone() {
            Adt41::C0(_, h2703, _) => {
                h2703
            }
        });
        let a2706 = f707(match v0 {
            Adt41::C0(_, _, h2704) => {
                h2704
            }
        });
        append_list(a2705, a2706)
    })))
}

pub fn f709(v0: Str, v1: Str, v2: Str, v3: Option<Adt42>, v4: Option<Adt5>) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(v0)), List::cons((Str::lit("data"), Adt6::C3(v1)), List::cons((Str::lit("mimeType"), Adt6::C3(v2)), {
        let a2707 = f712(v3);
        let a2708 = f707(v4);
        append_list(a2707, a2708)
    }))))
}

pub fn f710(v0: Adt46) -> R<Adt6> {
    Ok(Adt6::C5({
        let a2732 = List::cons((Str::lit("type"), Adt6::C3(Str::lit("resource_link"))), List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt46::C0(h2709, _, _, _, _, _, _, _, _) => {
                h2709
            }
        })), List::<(Str, Adt6)>::nil()));
        let a2733 = {
            let a2730 = f583(Str::lit("title"), match v0.clone() {
                Adt46::C0(_, h2710, _, _, _, _, _, _, _) => {
                    h2710
                }
            });
            let a2731 = {
                let a2728 = f649(match v0.clone() {
                    Adt46::C0(_, _, h2711, _, _, _, _, _, _) => {
                        h2711
                    }
                });
                let a2729 = {
                    let a2726 = List::cons((Str::lit("uri"), Adt6::C3(match v0.clone() {
                        Adt46::C0(_, _, _, h2712, _, _, _, _, _) => {
                            h2712
                        }
                    })), List::<(Str, Adt6)>::nil());
                    let a2727 = {
                        let a2724 = f583(Str::lit("description"), match v0.clone() {
                            Adt46::C0(_, _, _, _, h2713, _, _, _, _) => {
                                h2713
                            }
                        });
                        let a2725 = {
                            let a2722 = f583(Str::lit("mimeType"), match v0.clone() {
                                Adt46::C0(_, _, _, _, _, h2714, _, _, _) => {
                                    h2714
                                }
                            });
                            let a2723 = {
                                let a2720 = f712(match v0.clone() {
                                    Adt46::C0(_, _, _, _, _, _, h2715, _, _) => {
                                        h2715
                                    }
                                });
                                let a2721 = {
                                    let a2718 = f713(match v0.clone() {
                                        Adt46::C0(_, _, _, _, _, _, _, h2716, _) => {
                                            h2716
                                        }
                                    })?;
                                    let a2719 = f707(match v0 {
                                        Adt46::C0(_, _, _, _, _, _, _, _, h2717) => {
                                            h2717
                                        }
                                    });
                                    append_list(a2718, a2719)
                                };
                                append_list(a2720, a2721)
                            };
                            append_list(a2722, a2723)
                        };
                        append_list(a2724, a2725)
                    };
                    append_list(a2726, a2727)
                };
                append_list(a2728, a2729)
            };
            append_list(a2730, a2731)
        };
        append_list(a2732, a2733)
    }))
}

pub fn f711(v0: Adt47) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("resource"))), List::cons((Str::lit("resource"), f682(match v0.clone() {
        Adt47::C0(h2734, _, _) => {
            h2734
        }
    })), {
        let a2737 = f712(match v0.clone() {
            Adt47::C0(_, h2735, _) => {
                h2735
            }
        });
        let a2738 = f707(match v0 {
            Adt47::C0(_, _, h2736) => {
                h2736
            }
        });
        append_list(a2737, a2738)
    })))
}

pub fn f712(v0: Option<Adt42>) -> List<(Str, Adt6)> {
    let m2739 = v0;
    match m2739 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("annotations"), Adt6::C5({
                let a2745 = f743(match v1.clone() {
                    Adt42::C0(h2740, _, _) => {
                        h2740
                    }
                });
                let a2746 = {
                    let a2743 = f744(match v1.clone() {
                        Adt42::C0(_, h2741, _) => {
                            h2741
                        }
                    });
                    let a2744 = f583(Str::lit("lastModified"), match v1 {
                        Adt42::C0(_, _, h2742) => {
                            h2742
                        }
                    });
                    append_list(a2743, a2744)
                };
                append_list(a2745, a2746)
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f713(v0: Option<u64>) -> R<List<(Str, Adt6)>> {
    let m2747 = v0;
    match m2747 {
        None => {
            Ok(List::<(Str, Adt6)>::nil())
        }
        Some(v1) => {
            Ok(List::cons((Str::lit("size"), f52(v1)?), List::<(Str, Adt6)>::nil()))
        }
    }
}

pub fn f714(v0: List<Adt59>) -> List<Adt6> {
    f404(f745(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f715(v0: Adt43) -> Str {
    let m2748 = v0;
    match m2748 {
        Adt43::C0 => {
            Str::lit("user")
        }
        Adt43::C1 => {
            Str::lit("assistant")
        }
    }
}

pub fn f716(v0: Adt68) -> Adt6 {
    Adt6::C5({
        let a2755 = {
            let m2750 = match v0.clone() {
                Adt68::C0(h2749, _, _) => {
                    h2749
                }
            };
            match m2750 {
                None => {
                    List::<(Str, Adt6)>::nil()
                }
                Some(_) => {
                    List::cons((Str::lit("mode"), Adt6::C3(Str::lit("form"))), List::<(Str, Adt6)>::nil())
                }
            }
        };
        let a2756 = List::cons((Str::lit("message"), Adt6::C3(match v0.clone() {
            Adt68::C0(_, h2751, _) => {
                h2751
            }
        })), List::cons((Str::lit("requestedSchema"), Adt6::C5({
            let m2754 = match v0 {
                Adt68::C0(_, _, h2753) => {
                    h2753
                }
            };
            match m2754 {
                Adt5::C0(h2752) => {
                    h2752
                }
            }
        })), List::<(Str, Adt6)>::nil()));
        append_list(a2755, a2756)
    })
}

pub fn f717(v0: List<Adt72>) -> R<Option<List<Adt6>>> {
    let m2757 = f746(v0, List::<Adt6>::nil())?;
    match m2757 {
        None => {
            Ok(None::<List<Adt6>>)
        }
        Some(v1) => {
            Ok(Some(f404(v1, List::<Adt6>::nil())))
        }
    }
}

pub fn f718(v0: Adt71) -> List<(Str, Adt6)> {
    let a2768 = {
        let m2759 = match v0.clone() {
            Adt71::C0(_, h2758, _, _, _, _, _, _, _, _) => {
                h2758
            }
        };
        match m2759 {
            None => {
                List::<(Str, Adt6)>::nil()
            }
            Some(v1) => {
                List::cons((Str::lit("modelPreferences"), f747(v1)), List::<(Str, Adt6)>::nil())
            }
        }
    };
    let a2769 = {
        let a2766 = f583(Str::lit("systemPrompt"), match v0.clone() {
            Adt71::C0(_, _, h2760, _, _, _, _, _, _, _) => {
                h2760
            }
        });
        let a2767 = {
            let a2764 = {
                let m2762 = match v0.clone() {
                    Adt71::C0(_, _, _, h2761, _, _, _, _, _, _) => {
                        h2761
                    }
                };
                match m2762 {
                    None => {
                        List::<(Str, Adt6)>::nil()
                    }
                    Some(v2) => {
                        List::cons((Str::lit("includeContext"), Adt6::C3(f748(v2))), List::<(Str, Adt6)>::nil())
                    }
                }
            };
            let a2765 = f749(Str::lit("temperature"), match v0 {
                Adt71::C0(_, _, _, _, h2763, _, _, _, _, _) => {
                    h2763
                }
            });
            append_list(a2764, a2765)
        };
        append_list(a2766, a2767)
    };
    append_list(a2768, a2769)
}

pub fn f719(v0: Adt71) -> List<(Str, Adt6)> {
    let a2781 = {
        let m2771 = match v0.clone() {
            Adt71::C0(_, _, _, _, _, _, h2770, _, _, _) => {
                h2770
            }
        };
        match m2771 {
            None => {
                List::<(Str, Adt6)>::nil()
            }
            Some(v1) => {
                List::cons((Str::lit("stopSequences"), f581(v1)), List::<(Str, Adt6)>::nil())
            }
        }
    };
    let a2782 = {
        let a2779 = f614(Str::lit("metadata"), match v0.clone() {
            Adt71::C0(_, _, _, _, _, _, _, h2772, _, _) => {
                h2772
            }
        });
        let a2780 = {
            let a2777 = {
                let m2774 = match v0.clone() {
                    Adt71::C0(_, _, _, _, _, _, _, _, h2773, _) => {
                        h2773
                    }
                };
                match m2774 {
                    None => {
                        List::<(Str, Adt6)>::nil()
                    }
                    Some(v2) => {
                        List::cons((Str::lit("tools"), Adt6::C4(f585(v2))), List::<(Str, Adt6)>::nil())
                    }
                }
            };
            let a2778 = {
                let m2776 = match v0 {
                    Adt71::C0(_, _, _, _, _, _, _, _, _, h2775) => {
                        h2775
                    }
                };
                match m2776 {
                    None => {
                        List::<(Str, Adt6)>::nil()
                    }
                    Some(v3) => {
                        List::cons((Str::lit("toolChoice"), Adt6::C5(f750(v3))), List::<(Str, Adt6)>::nil())
                    }
                }
            };
            append_list(a2777, a2778)
        };
        append_list(a2779, a2780)
    };
    append_list(a2781, a2782)
}

pub fn f720(p2785: List<Adt2>, p2786: List<Adt6>) -> List<Adt6> {
    let m2784 = (p2785, p2786);
    {
        let mut m2784 = m2784;
        loop {
            let step = {
                let (v0, v1) = m2784;
                let m2783 = v0;
                match m2783.uncons() {
                    None => {
                        Err::<(List<Adt2>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt2>, List<Adt6>), List<Adt6>>((v3, List::cons(f751(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2784 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f721() -> List<u64> {
    List::cons(100u64, List::cons(97u64, List::cons(116u64, List::cons(97u64, List::cons(58u64, List::cons(32u64, List::<u64>::nil()))))))
}

pub fn f722() -> List<u64> {
    List::cons(10u64, List::cons(10u64, List::<u64>::nil()))
}

pub fn f723(p2791: List<u64>, p2792: u64, p2793: List<u64>) -> List<u64> {
    let m2790 = (p2791, (p2792, p2793));
    {
        let mut m2790 = m2790;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2790;
                let m2787 = v0;
                match m2787.uncons() {
                    None => {
                        Err::<(List<u64>, (u64, List<u64>)), List<u64>>(f752(v1, v2))
                    }
                    Some((v3, v4)) => {
                        let v5: Adt157 = f753(v1, v3, v2);
                        Ok::<(List<u64>, (u64, List<u64>)), List<u64>>((v4, (match v5.clone() {
                            Adt157::C0(h2788, _) => {
                                h2788
                            }
                        }, match v5 {
                            Adt157::C0(_, h2789) => {
                                h2789
                            }
                        })))
                    }
                }
            };
            match step {
                Ok(next) => { m2790 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f724(v0: Adt148, v1: Adt148) -> bool {
    let a2794 = f561(v0);
    let a2795 = f561(v1);
    equal(a2794, a2795)
}

pub fn f725(v0: Adt148) -> bool {
    let m2796 = v0;
    match m2796 {
        Adt148::C0 => {
            false
        }
        Adt148::C1 => {
            false
        }
        Adt148::C2 => {
            true
        }
        Adt148::C3 => {
            false
        }
        Adt148::C4 => {
            true
        }
        Adt148::C5 => {
            false
        }
        Adt148::C6 => {
            false
        }
        Adt148::C7 => {
            true
        }
        Adt148::C8 => {
            false
        }
        Adt148::C9 => {
            false
        }
        Adt148::C10 => {
            false
        }
        Adt148::C11 => {
            false
        }
    }
}

pub fn f726(v0: Adt148) -> bool {
    let m2797 = v0;
    match m2797 {
        Adt148::C0 => {
            false
        }
        Adt148::C1 => {
            false
        }
        Adt148::C2 => {
            true
        }
        Adt148::C3 => {
            false
        }
        Adt148::C4 => {
            false
        }
        Adt148::C5 => {
            false
        }
        Adt148::C6 => {
            false
        }
        Adt148::C7 => {
            false
        }
        Adt148::C8 => {
            false
        }
        Adt148::C9 => {
            false
        }
        Adt148::C10 => {
            false
        }
        Adt148::C11 => {
            false
        }
    }
}

pub fn f727(v0: Adt0, v1: Adt18, v2: Adt83) -> Result<Adt136, Adt139> {
    f754(v0, v2.clone(), f755(match v1 {
        Adt18::C0(_, h2798, _, _, _, _) => {
            h2798
        }
    }, f596(v2)))
}

pub fn f728(v0: Adt16) -> Adt18 {
    let m2799 = v0;
    match m2799 {
        Adt16::C0(v1) => {
            match v1 {
                Adt17::C0(h2800) => {
                    h2800
                }
            }
        }
        Adt16::C1(v2) => {
            match v2 {
                Adt22::C0(h2801, _) => {
                    h2801
                }
            }
        }
        Adt16::C2(v3) => {
            match v3 {
                Adt23::C0(h2802, _, _, _, _) => {
                    h2802
                }
            }
        }
        Adt16::C3(v4) => {
            match v4 {
                Adt22::C0(h2803, _) => {
                    h2803
                }
            }
        }
        Adt16::C4(v5) => {
            match v5 {
                Adt24::C0(h2804, _, _, _) => {
                    h2804
                }
            }
        }
        Adt16::C5(v6) => {
            match v6 {
                Adt22::C0(h2805, _) => {
                    h2805
                }
            }
        }
        Adt16::C6(v7) => {
            match v7 {
                Adt22::C0(h2806, _) => {
                    h2806
                }
            }
        }
        Adt16::C7(v8) => {
            match v8 {
                Adt25::C0(h2807, _, _, _, _) => {
                    h2807
                }
            }
        }
        Adt16::C8(v9) => {
            match v9 {
                Adt26::C0(h2808, _, _, _) => {
                    h2808
                }
            }
        }
        Adt16::C9(v10) => {
            match v10 {
                Adt31::C0(h2809, _) => {
                    h2809
                }
            }
        }
        Adt16::C10(v11) => {
            match v11 {
                Adt32::C0(h2810, _, _) => {
                    h2810
                }
            }
        }
        Adt16::C11(v12) => {
            match v12 {
                Adt33::C0(h2811, _) => {
                    h2811
                }
            }
        }
    }
}

pub fn f729(v0: Adt0, v1: Adt18, v2: Adt84) -> Result<Adt136, Adt139> {
    f756(v0, v2, f757(match v1 {
        Adt18::C0(_, h2812, _, _, _, _) => {
            h2812
        }
    }))
}

pub fn f730(v0: Adt0, v1: Adt18, v2: Adt51) -> Adt136 {
    Adt136::C0(Adt34::C2(match v2.clone() {
        Adt51::C0(h2813, _) => {
            h2813
        }
    }), {
        let m2815 = f660(v0, Adt34::C11);
        match m2815 {
            Adt136::C0(_, h2814, _) => {
                h2814
            }
        }
    }, f758(v1, match v2 {
        Adt51::C0(_, h2816) => {
            h2816
        }
    }))
}

pub fn f731(v0: Bytes, v1: Adt16, v2: Result<Adt34, Adt86>) -> R<Result<Adt34, Adt86>> {
    let m2817 = v2;
    match m2817 {
        Ok(v4) => {
            f759(v0, f659(v1.clone()), f760(v1), v4)
        }
        Err(v3) => {
            Ok(Err::<Adt34, Adt86>(v3))
        }
    }
}

pub fn f732(v0: u64, v1: u64, v2: Fn2, v3: Adt16) -> R<Result<Adt34, Adt86>> {
    let m2818 = v3.clone();
    match m2818 {
        Adt16::C0(_) => {
            let c2820: Fn2 = v2;
            c2820.apply(v3)
        }
        Adt16::C1(_) => {
            let c2821: Fn2 = v2;
            c2821.apply(v3)
        }
        Adt16::C2(_) => {
            if f761(v0, v1) {
                let c2819: Fn2 = v2;
                c2819.apply(v3)
            } else {
                Ok(Err::<Adt34, Adt86>(Adt86::C7(v0)))
            }
        }
        Adt16::C3(_) => {
            let c2822: Fn2 = v2;
            c2822.apply(v3)
        }
        Adt16::C4(_) => {
            let c2823: Fn2 = v2;
            c2823.apply(v3)
        }
        Adt16::C5(_) => {
            let c2824: Fn2 = v2;
            c2824.apply(v3)
        }
        Adt16::C6(_) => {
            let c2825: Fn2 = v2;
            c2825.apply(v3)
        }
        Adt16::C7(_) => {
            let c2826: Fn2 = v2;
            c2826.apply(v3)
        }
        Adt16::C8(_) => {
            let c2827: Fn2 = v2;
            c2827.apply(v3)
        }
        Adt16::C9(_) => {
            let c2828: Fn2 = v2;
            c2828.apply(v3)
        }
        Adt16::C10(_) => {
            let c2829: Fn2 = v2;
            c2829.apply(v3)
        }
        Adt16::C11(_) => {
            let c2830: Fn2 = v2;
            c2830.apply(v3)
        }
    }
}

pub fn f733(v0: Bytes, v1: Adt23) -> R<Result<Adt16, Adt86>> {
    let m2834 = f762(v0, Str::lit("tools/call"), match v1.clone() {
        Adt23::C0(_, h2831, _, _, _) => {
            h2831
        }
    }, match v1.clone() {
        Adt23::C0(_, _, _, _, h2832) => {
            h2832
        }
    }, match v1.clone() {
        Adt23::C0(_, _, _, h2833, _) => {
            h2833
        }
    })?;
    match m2834 {
        Ok(v3) => {
            Ok(Ok::<Adt16, Adt86>(Adt16::C2(Adt23::C0(match v1.clone() {
                Adt23::C0(h2835, _, _, _, _) => {
                    h2835
                }
            }, match v1.clone() {
                Adt23::C0(_, h2836, _, _, _) => {
                    h2836
                }
            }, match v1.clone() {
                Adt23::C0(_, _, h2837, _, _) => {
                    h2837
                }
            }, match v1 {
                Adt23::C0(_, _, _, h2838, _) => {
                    h2838
                }
            }, v3))))
        }
        Err(v2) => {
            Ok(Err::<Adt16, Adt86>(v2))
        }
    }
}

pub fn f734(v0: Bytes, v1: Adt24) -> R<Result<Adt16, Adt86>> {
    let m2842 = f762(v0, Str::lit("resources/read"), match v1.clone() {
        Adt24::C0(_, h2839, _, _) => {
            h2839
        }
    }, match v1.clone() {
        Adt24::C0(_, _, _, h2840) => {
            h2840
        }
    }, match v1.clone() {
        Adt24::C0(_, _, h2841, _) => {
            h2841
        }
    })?;
    match m2842 {
        Ok(v3) => {
            Ok(Ok::<Adt16, Adt86>(Adt16::C4(Adt24::C0(match v1.clone() {
                Adt24::C0(h2843, _, _, _) => {
                    h2843
                }
            }, match v1.clone() {
                Adt24::C0(_, h2844, _, _) => {
                    h2844
                }
            }, match v1 {
                Adt24::C0(_, _, h2845, _) => {
                    h2845
                }
            }, v3))))
        }
        Err(v2) => {
            Ok(Err::<Adt16, Adt86>(v2))
        }
    }
}

pub fn f735(v0: Bytes, v1: Adt25) -> R<Result<Adt16, Adt86>> {
    let m2849 = f762(v0, Str::lit("prompts/get"), match v1.clone() {
        Adt25::C0(_, h2846, _, _, _) => {
            h2846
        }
    }, match v1.clone() {
        Adt25::C0(_, _, _, _, h2847) => {
            h2847
        }
    }, match v1.clone() {
        Adt25::C0(_, _, _, h2848, _) => {
            h2848
        }
    })?;
    match m2849 {
        Ok(v3) => {
            Ok(Ok::<Adt16, Adt86>(Adt16::C7(Adt25::C0(match v1.clone() {
                Adt25::C0(h2850, _, _, _, _) => {
                    h2850
                }
            }, match v1.clone() {
                Adt25::C0(_, h2851, _, _, _) => {
                    h2851
                }
            }, match v1.clone() {
                Adt25::C0(_, _, h2852, _, _) => {
                    h2852
                }
            }, match v1 {
                Adt25::C0(_, _, _, h2853, _) => {
                    h2853
                }
            }, v3))))
        }
        Err(v2) => {
            Ok(Err::<Adt16, Adt86>(v2))
        }
    }
}

pub fn f736(v0: Str, p2859: List<(Str, Adt6)>, p2860: List<(Str, Str)>) -> Result<List<(Str, Str)>, Adt140> {
    let m2858 = (p2859, p2860);
    {
        let mut m2858 = m2858;
        loop {
            let step = {
                let (v1, v2) = m2858;
                let m2854 = v1;
                match m2854.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<(Str, Str)>), Result<List<(Str, Str)>, Adt140>>(Ok::<List<(Str, Str)>, Adt140>(v2))
                    }
                    Some((v3, v4)) => {
                        let m2856 = f130({
                            let (_, h2855) = v3;
                            h2855
                        });
                        match m2856 {
                            None => {
                                Err::<(List<(Str, Adt6)>, List<(Str, Str)>), Result<List<(Str, Str)>, Adt140>>(Err::<List<(Str, Str)>, Adt140>(Adt140::C8(v0.clone())))
                            }
                            Some(v5) => {
                                Ok::<(List<(Str, Adt6)>, List<(Str, Str)>), Result<List<(Str, Str)>, Adt140>>((v4, List::cons(({
                                    let (h2857, _) = v3;
                                    h2857
                                }, v5), v2)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2858 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f737(p2863: List<(Str, Str)>, p2864: List<(Str, Str)>) -> List<(Str, Str)> {
    let m2862 = (p2863, p2864);
    {
        let mut m2862 = m2862;
        loop {
            let step = {
                let (v0, v1) = m2862;
                let m2861 = v0;
                match m2861.uncons() {
                    None => {
                        Err::<(List<(Str, Str)>, List<(Str, Str)>), List<(Str, Str)>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Str)>, List<(Str, Str)>), List<(Str, Str)>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2862 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f738(v0: Adt129, v1: Adt5) -> R<Option<Adt127>> {
    let m2866 = f763(match v0.clone() {
        Adt129::C0(_, h2865, _, _) => {
            h2865
        }
    }, v1);
    match m2866 {
        Adt114::C0 => {
            Ok(f764(v0))
        }
        Adt114::C1(v2) => {
            if f765(v2.clone()) {
                Ok(f764(v0))
            } else {
                f766(v0, v2)
            }
        }
    }
}

pub fn f739(v0: Str, v1: Adt5) -> Option<Option<Adt5>> {
    let m2867 = f184(v0, v1);
    match m2867 {
        Adt114::C0 => {
            Some(None::<Adt5>)
        }
        Adt114::C1(v2) => {
            let m2868 = f195(v2);
            match m2868 {
                None => {
                    None::<Option<Adt5>>
                }
                Some(v3) => {
                    Some(Some(v3))
                }
            }
        }
    }
}

pub fn f740() -> Str {
    Str::lit("io.modelcontextprotocol/logLevel")
}

pub fn f741(v0: Str) -> Adt158 {
    let a2869 = v0.clone();
    let a2870 = Str::lit("debug");
    if equal(a2869, a2870) {
        Adt158::C1(Adt20::C0)
    } else {
        let a2871 = v0.clone();
        let a2872 = Str::lit("info");
        if equal(a2871, a2872) {
            Adt158::C1(Adt20::C1)
        } else {
            let a2873 = v0.clone();
            let a2874 = Str::lit("notice");
            if equal(a2873, a2874) {
                Adt158::C1(Adt20::C2)
            } else {
                let a2875 = v0.clone();
                let a2876 = Str::lit("warning");
                if equal(a2875, a2876) {
                    Adt158::C1(Adt20::C3)
                } else {
                    let a2877 = v0.clone();
                    let a2878 = Str::lit("error");
                    if equal(a2877, a2878) {
                        Adt158::C1(Adt20::C4)
                    } else {
                        let a2879 = v0.clone();
                        let a2880 = Str::lit("critical");
                        if equal(a2879, a2880) {
                            Adt158::C1(Adt20::C5)
                        } else {
                            let a2881 = v0.clone();
                            let a2882 = Str::lit("alert");
                            if equal(a2881, a2882) {
                                Adt158::C1(Adt20::C6)
                            } else {
                                let a2883 = v0;
                                let a2884 = Str::lit("emergency");
                                if equal(a2883, a2884) {
                                    Adt158::C1(Adt20::C7)
                                } else {
                                    Adt158::C0
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f742() -> Str {
    Str::lit("progressToken")
}

pub fn f743(v0: Option<List<Adt43>>) -> List<(Str, Adt6)> {
    let m2885 = v0;
    match m2885 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("audience"), Adt6::C4(f767(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f744(v0: Option<Adt7>) -> List<(Str, Adt6)> {
    let m2886 = v0;
    match m2886 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("priority"), Adt6::C2(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f745(p2889: List<Adt59>, p2890: List<Adt6>) -> List<Adt6> {
    let m2888 = (p2889, p2890);
    {
        let mut m2888 = m2888;
        loop {
            let step = {
                let (v0, v1) = m2888;
                let m2887 = v0;
                match m2887.uncons() {
                    None => {
                        Err::<(List<Adt59>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt59>, List<Adt6>), List<Adt6>>((v3, List::cons(f768(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2888 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f746(p2894: List<Adt72>, p2895: List<Adt6>) -> R<Option<List<Adt6>>> {
    let m2893 = (p2894, p2895);
    {
        let mut m2893 = m2893;
        loop {
            let step = {
                let (v0, v1) = m2893;
                let m2891 = v0;
                match m2891.uncons() {
                    None => {
                        Err::<(List<Adt72>, List<Adt6>), R<Option<List<Adt6>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m2892 = f769(v2)?;
                        match m2892 {
                            None => {
                                Err::<(List<Adt72>, List<Adt6>), R<Option<List<Adt6>>>>(Ok(None::<List<Adt6>>))
                            }
                            Some(v4) => {
                                Ok::<(List<Adt72>, List<Adt6>), R<Option<List<Adt6>>>>((v3, List::cons(v4, v1)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2893 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f747(v0: Adt77) -> Adt6 {
    Adt6::C5({
        let a2905 = {
            let m2897 = match v0.clone() {
                Adt77::C0(h2896, _, _, _) => {
                    h2896
                }
            };
            match m2897 {
                None => {
                    List::<(Str, Adt6)>::nil()
                }
                Some(v1) => {
                    List::cons((Str::lit("hints"), Adt6::C4(f404(f770(v1, List::<Adt6>::nil()), List::<Adt6>::nil()))), List::<(Str, Adt6)>::nil())
                }
            }
        };
        let a2906 = {
            let a2903 = f749(Str::lit("costPriority"), match v0.clone() {
                Adt77::C0(_, h2898, _, _) => {
                    h2898
                }
            });
            let a2904 = {
                let a2901 = f749(Str::lit("speedPriority"), match v0.clone() {
                    Adt77::C0(_, _, h2899, _) => {
                        h2899
                    }
                });
                let a2902 = f749(Str::lit("intelligencePriority"), match v0 {
                    Adt77::C0(_, _, _, h2900) => {
                        h2900
                    }
                });
                append_list(a2901, a2902)
            };
            append_list(a2903, a2904)
        };
        append_list(a2905, a2906)
    })
}

pub fn f748(v0: Adt79) -> Str {
    let m2907 = v0;
    match m2907 {
        Adt79::C0 => {
            Str::lit("none")
        }
        Adt79::C1 => {
            Str::lit("thisServer")
        }
        Adt79::C2 => {
            Str::lit("allServers")
        }
    }
}

pub fn f749(v0: Str, v1: Option<Adt7>) -> List<(Str, Adt6)> {
    let m2908 = v1;
    match m2908 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C2(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f750(v0: Adt80) -> List<(Str, Adt6)> {
    let m2910 = match v0 {
        Adt80::C0(h2909) => {
            h2909
        }
    };
    match m2910 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("mode"), Adt6::C3(f771(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f751(v0: Adt2) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("src"), Adt6::C3(match v0.clone() {
        Adt2::C0(h2911, _, _, _) => {
            h2911
        }
    })), {
        let a2917 = f583(Str::lit("mimeType"), match v0.clone() {
            Adt2::C0(_, h2912, _, _) => {
                h2912
            }
        });
        let a2918 = {
            let a2915 = f772(match v0.clone() {
                Adt2::C0(_, _, h2913, _) => {
                    h2913
                }
            });
            let a2916 = f773(match v0 {
                Adt2::C0(_, _, _, h2914) => {
                    h2914
                }
            });
            append_list(a2915, a2916)
        };
        append_list(a2917, a2918)
    }))
}

pub fn f752(v0: u64, v1: List<u64>) -> List<u64> {
    let a2919 = v0;
    let a2920 = 0u64;
    if nat_eq(a2919, a2920) {
        v1
    } else {
        let a2921 = v0;
        let a2922 = 1u64;
        if nat_eq(a2921, a2922) {
            List::cons(194u64, v1)
        } else {
            let a2923 = v0;
            let a2924 = 2u64;
            if nat_eq(a2923, a2924) {
                List::cons(226u64, v1)
            } else {
                let a2925 = v0;
                let a2926 = 3u64;
                if nat_eq(a2925, a2926) {
                    List::cons(128u64, List::cons(226u64, v1))
                } else {
                    List::cons(129u64, List::cons(226u64, v1))
                }
            }
        }
    }
}

pub fn f753(v0: u64, v1: u64, v2: List<u64>) -> Adt157 {
    let a2927 = v0;
    let a2928 = 0u64;
    if nat_eq(a2927, a2928) {
        f774(v1, v2)
    } else {
        let a2929 = v0;
        let a2930 = 1u64;
        if nat_eq(a2929, a2930) {
            let a2931 = 128u64;
            let a2932 = v1;
            if if nat_le(a2931, a2932) {
                let a2933 = v1;
                let a2934 = 159u64;
                nat_le(a2933, a2934)
            } else {
                false
            } {
                Adt157::C0(0u64, f775(v2))
            } else {
                f774(v1, List::cons(194u64, v2))
            }
        } else {
            let a2935 = v0;
            let a2936 = 2u64;
            if nat_eq(a2935, a2936) {
                let a2937 = v1;
                let a2938 = 128u64;
                if nat_eq(a2937, a2938) {
                    Adt157::C0(3u64, v2)
                } else {
                    let a2939 = v1;
                    let a2940 = 129u64;
                    if nat_eq(a2939, a2940) {
                        Adt157::C0(4u64, v2)
                    } else {
                        f774(v1, List::cons(226u64, v2))
                    }
                }
            } else {
                let a2941 = v0;
                let a2942 = 3u64;
                if nat_eq(a2941, a2942) {
                    let a2943 = 170u64;
                    let a2944 = v1;
                    if if nat_le(a2943, a2944) {
                        let a2945 = v1;
                        let a2946 = 174u64;
                        nat_le(a2945, a2946)
                    } else {
                        false
                    } {
                        Adt157::C0(0u64, f775(v2))
                    } else {
                        f774(v1, List::cons(128u64, List::cons(226u64, v2)))
                    }
                } else {
                    let a2947 = 166u64;
                    let a2948 = v1;
                    if if nat_le(a2947, a2948) {
                        let a2949 = v1;
                        let a2950 = 169u64;
                        nat_le(a2949, a2950)
                    } else {
                        false
                    } {
                        Adt157::C0(0u64, f775(v2))
                    } else {
                        f774(v1, List::cons(129u64, List::cons(226u64, v2)))
                    }
                }
            }
        }
    }
}

pub fn f754(v0: Adt0, v1: Adt83, v2: bool) -> Result<Adt136, Adt139> {
    if v2 {
        Ok::<Adt136, Adt139>(f660(v0, Adt34::C12(v1)))
    } else {
        Err::<Adt136, Adt139>(Adt139::C4(Adt86::C8(f776(f596(v1)))))
    }
}

pub fn f755(v0: Adt19, v1: List<(Str, Adt66)>) -> bool {
    let a2951 = f777(v1.clone());
    if if if bool_not(a2951) {
        true
    } else {
        f564(match v0.clone() {
            Adt19::C0(_, _, _, h2952, _) => {
                h2952
            }
        })
    } {
        let a2953 = f778(v1.clone());
        if bool_not(a2953) {
            true
        } else {
            f564(match v0.clone() {
                Adt19::C0(_, _, h2954, _, _) => {
                    h2954
                }
            })
        }
    } else {
        false
    } {
        let a2955 = f779(v1);
        if bool_not(a2955) {
            true
        } else {
            f564(match v0 {
                Adt19::C0(_, h2956, _, _, _) => {
                    h2956
                }
            })
        }
    } else {
        false
    }
}

pub fn f756(v0: Adt0, v1: Adt84, v2: bool) -> Result<Adt136, Adt139> {
    if v2 {
        Ok::<Adt136, Adt139>(f660(v0, Adt34::C13(v1)))
    } else {
        Err::<Adt136, Adt139>(Adt139::C4(Adt86::C8(f780())))
    }
}

pub fn f757(v0: Adt19) -> bool {
    f565(match v0 {
        Adt19::C0(_, _, _, _, h2957) => {
            h2957
        }
    }, f566())
}

pub fn f758(v0: Adt18, v1: List<Adt52>) -> List<Adt138> {
    let m2959 = match v0 {
        Adt18::C0(_, _, _, _, h2958, _) => {
            h2958
        }
    };
    match m2959 {
        None => {
            List::<Adt138>::nil()
        }
        Some(v2) => {
            f781(f782(v2, v1, List::<Adt138>::nil()), List::<Adt138>::nil())
        }
    }
}

pub fn f759(v0: Bytes, v1: Adt148, v2: Str, v3: Adt34) -> R<Result<Adt34, Adt86>> {
    let m2960 = v3.clone();
    match m2960 {
        Adt34::C0(_) => {
            Ok(Ok::<Adt34, Adt86>(v3))
        }
        Adt34::C1(_) => {
            Ok(Ok::<Adt34, Adt86>(v3))
        }
        Adt34::C2(_) => {
            Ok(Ok::<Adt34, Adt86>(v3))
        }
        Adt34::C3(_) => {
            Ok(Ok::<Adt34, Adt86>(v3))
        }
        Adt34::C4(_) => {
            Ok(Ok::<Adt34, Adt86>(v3))
        }
        Adt34::C5(_) => {
            Ok(Ok::<Adt34, Adt86>(v3))
        }
        Adt34::C6(_) => {
            Ok(Ok::<Adt34, Adt86>(v3))
        }
        Adt34::C7(_) => {
            Ok(Ok::<Adt34, Adt86>(v3))
        }
        Adt34::C8(_) => {
            Ok(Ok::<Adt34, Adt86>(v3))
        }
        Adt34::C9(_) => {
            Ok(Ok::<Adt34, Adt86>(v3))
        }
        Adt34::C10(_) => {
            Ok(Ok::<Adt34, Adt86>(v3))
        }
        Adt34::C11 => {
            Ok(Ok::<Adt34, Adt86>(v3))
        }
        Adt34::C12(v4) => {
            f783(v0, v1, v2, v4)
        }
        Adt34::C13(_) => {
            Ok(Ok::<Adt34, Adt86>(v3))
        }
    }
}

pub fn f760(v0: Adt16) -> Str {
    let m2961 = v0;
    match m2961 {
        Adt16::C0(_) => {
            Str::lit("")
        }
        Adt16::C1(_) => {
            Str::lit("")
        }
        Adt16::C2(v1) => {
            match v1 {
                Adt23::C0(_, h2962, _, _, _) => {
                    h2962
                }
            }
        }
        Adt16::C3(_) => {
            Str::lit("")
        }
        Adt16::C4(v2) => {
            match v2 {
                Adt24::C0(_, h2963, _, _) => {
                    h2963
                }
            }
        }
        Adt16::C5(_) => {
            Str::lit("")
        }
        Adt16::C6(_) => {
            Str::lit("")
        }
        Adt16::C7(v3) => {
            match v3 {
                Adt25::C0(_, h2964, _, _, _) => {
                    h2964
                }
            }
        }
        Adt16::C8(_) => {
            Str::lit("")
        }
        Adt16::C9(_) => {
            Str::lit("")
        }
        Adt16::C10(_) => {
            Str::lit("")
        }
        Adt16::C11(_) => {
            Str::lit("")
        }
    }
}

pub fn f761(v0: u64, v1: u64) -> bool {
    let a2965 = v1;
    let a2966 = v0;
    nat_lt(a2965, a2966)
}

pub fn f762(v0: Bytes, v1: Str, v2: Str, v3: Option<Str>, v4: Option<Adt5>) -> R<Result<Option<Str>, Adt86>> {
    Ok(f784(f785(v0, v1, v2, v3)?, f786(v4)))
}

pub fn f763(p2972: List<Str>, p2973: Adt5) -> Adt114 {
    let m2971 = (p2972, p2973);
    {
        let mut m2971 = m2971;
        loop {
            let step = {
                let (v0, v1) = m2971;
                let m2967 = v0;
                match m2967.uncons() {
                    None => {
                        Err::<(List<Str>, Adt5), Adt114>(Adt114::C0)
                    }
                    Some((v2, v3)) => {
                        let m2968 = f184(v2, v1);
                        match m2968 {
                            Adt114::C0 => {
                                Err::<(List<Str>, Adt5), Adt114>(Adt114::C0)
                            }
                            Adt114::C1(v4) => {
                                let m2969 = v3.clone();
                                match m2969.uncons() {
                                    None => {
                                        Err::<(List<Str>, Adt5), Adt114>(Adt114::C1(v4))
                                    }
                                    Some((_, _)) => {
                                        let m2970 = f195(v4);
                                        match m2970 {
                                            None => {
                                                Err::<(List<Str>, Adt5), Adt114>(Adt114::C0)
                                            }
                                            Some(v7) => {
                                                Ok::<(List<Str>, Adt5), Adt114>((v3, v7))
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2971 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f764(v0: Adt129) -> Option<Adt127> {
    let m2975 = match v0.clone() {
        Adt129::C0(_, _, _, h2974) => {
            h2974
        }
    };
    match m2975 {
        Adt130::C0 => {
            None::<Adt127>
        }
        Adt130::C1 => {
            Some(Adt127::C10(match v0 {
                Adt129::C0(h2976, _, _, _) => {
                    h2976
                }
            }))
        }
        Adt130::C2(_) => {
            None::<Adt127>
        }
    }
}

pub fn f765(v0: Adt6) -> bool {
    let m2977 = v0;
    match m2977 {
        Adt6::C0 => {
            true
        }
        Adt6::C1(_) => {
            false
        }
        Adt6::C2(_) => {
            false
        }
        Adt6::C3(_) => {
            false
        }
        Adt6::C4(_) => {
            false
        }
        Adt6::C5(_) => {
            false
        }
    }
}

pub fn f766(v0: Adt129, v1: Adt6) -> R<Option<Adt127>> {
    let m2979 = match v0.clone() {
        Adt129::C0(_, _, _, h2978) => {
            h2978
        }
    };
    match m2979 {
        Adt130::C0 => {
            Ok(Some(Adt127::C8(match v0 {
                Adt129::C0(h2980, _, _, _) => {
                    h2980
                }
            })))
        }
        Adt130::C1 => {
            Ok(Some(Adt127::C10(match v0 {
                Adt129::C0(h2981, _, _, _) => {
                    h2981
                }
            })))
        }
        Adt130::C2(v2) => {
            if f787(match v0.clone() {
                Adt129::C0(_, _, h2982, _) => {
                    h2982
                }
            }, v2, v1)? {
                Ok(None::<Adt127>)
            } else {
                Ok(Some(Adt127::C9(match v0 {
                    Adt129::C0(h2983, _, _, _) => {
                        h2983
                    }
                })))
            }
        }
    }
}

pub fn f767(v0: List<Adt43>) -> List<Adt6> {
    f404(f788(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f768(v0: Adt59) -> Adt6 {
    Adt6::C5({
        let a2992 = List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt59::C0(h2984, _, _, _) => {
                h2984
            }
        })), List::<(Str, Adt6)>::nil());
        let a2993 = {
            let a2990 = f583(Str::lit("title"), match v0.clone() {
                Adt59::C0(_, h2985, _, _) => {
                    h2985
                }
            });
            let a2991 = {
                let a2988 = f583(Str::lit("description"), match v0.clone() {
                    Adt59::C0(_, _, h2986, _) => {
                        h2986
                    }
                });
                let a2989 = f588(Str::lit("required"), match v0 {
                    Adt59::C0(_, _, _, h2987) => {
                        h2987
                    }
                });
                append_list(a2988, a2989)
            };
            append_list(a2990, a2991)
        };
        append_list(a2992, a2993)
    })
}

pub fn f769(v0: Adt72) -> R<Option<Adt6>> {
    let m2995 = f789(match v0.clone() {
        Adt72::C0(_, h2994, _) => {
            h2994
        }
    })?;
    match m2995 {
        None => {
            Ok(None::<Adt6>)
        }
        Some(v1) => {
            Ok(Some(Adt6::C5(List::cons((Str::lit("role"), Adt6::C3(f715(match v0.clone() {
                Adt72::C0(h2996, _, _) => {
                    h2996
                }
            }))), List::cons((Str::lit("content"), v1), f707(match v0 {
                Adt72::C0(_, _, h2997) => {
                    h2997
                }
            }))))))
        }
    }
}

pub fn f770(p3001: List<Adt78>, p3002: List<Adt6>) -> List<Adt6> {
    let m3000 = (p3001, p3002);
    {
        let mut m3000 = m3000;
        loop {
            let step = {
                let (v0, v1) = m3000;
                let m2998 = v0;
                match m2998.uncons() {
                    None => {
                        Err::<(List<Adt78>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt78>, List<Adt6>), List<Adt6>>((v3, List::cons(Adt6::C5(f583(Str::lit("name"), match v2 {
                            Adt78::C0(h2999) => {
                                h2999
                            }
                        })), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m3000 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f771(v0: Adt81) -> Str {
    let m3003 = v0;
    match m3003 {
        Adt81::C0 => {
            Str::lit("auto")
        }
        Adt81::C1 => {
            Str::lit("required")
        }
        Adt81::C2 => {
            Str::lit("none")
        }
    }
}

pub fn f772(v0: Option<List<Str>>) -> List<(Str, Adt6)> {
    let m3004 = v0;
    match m3004 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("sizes"), Adt6::C4(f556(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f773(v0: Option<Adt3>) -> List<(Str, Adt6)> {
    let m3005 = v0;
    match m3005 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("theme"), Adt6::C3(f790(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f774(v0: u64, v1: List<u64>) -> Adt157 {
    let a3006 = v0;
    let a3007 = 32u64;
    if nat_lt(a3006, a3007) {
        if f791(v0) {
            Adt157::C0(0u64, List::cons(v0, v1))
        } else {
            Adt157::C0(0u64, f775(v1))
        }
    } else {
        let a3008 = v0;
        let a3009 = 127u64;
        if nat_eq(a3008, a3009) {
            Adt157::C0(0u64, f775(v1))
        } else {
            let a3010 = v0;
            let a3011 = 194u64;
            if nat_eq(a3010, a3011) {
                Adt157::C0(1u64, v1)
            } else {
                let a3012 = v0;
                let a3013 = 226u64;
                if nat_eq(a3012, a3013) {
                    Adt157::C0(2u64, v1)
                } else {
                    Adt157::C0(0u64, List::cons(v0, v1))
                }
            }
        }
    }
}

pub fn f775(v0: List<u64>) -> List<u64> {
    List::cons(189u64, List::cons(191u64, List::cons(239u64, v0)))
}

pub fn f776(v0: List<(Str, Adt66)>) -> Adt19 {
    Adt19::C0(None::<Adt5>, if f779(v0.clone()) {
        Some(f632())
    } else {
        None::<Adt5>
    }, if f778(v0.clone()) {
        Some(f632())
    } else {
        None::<Adt5>
    }, if f777(v0) {
        Some(f632())
    } else {
        None::<Adt5>
    }, None::<Adt5>)
}

pub fn f777(p3018: List<(Str, Adt66)>) -> bool {
    let m3017 = p3018;
    {
        let mut m3017 = m3017;
        loop {
            let step = {
                let v0 = m3017;
                let m3014 = v0;
                match m3014.uncons() {
                    None => {
                        Err::<List<(Str, Adt66)>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        let m3016 = {
                            let (_, h3015) = v1;
                            h3015
                        };
                        match m3016 {
                            Adt66::C0(_) => {
                                Err::<List<(Str, Adt66)>, bool>(true)
                            }
                            Adt66::C1(_) => {
                                Ok::<List<(Str, Adt66)>, bool>(v2)
                            }
                            Adt66::C2(_) => {
                                Ok::<List<(Str, Adt66)>, bool>(v2)
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3017 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f778(p3023: List<(Str, Adt66)>) -> bool {
    let m3022 = p3023;
    {
        let mut m3022 = m3022;
        loop {
            let step = {
                let v0 = m3022;
                let m3019 = v0;
                match m3019.uncons() {
                    None => {
                        Err::<List<(Str, Adt66)>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        let m3021 = {
                            let (_, h3020) = v1;
                            h3020
                        };
                        match m3021 {
                            Adt66::C0(_) => {
                                Ok::<List<(Str, Adt66)>, bool>(v2)
                            }
                            Adt66::C1(_) => {
                                Err::<List<(Str, Adt66)>, bool>(true)
                            }
                            Adt66::C2(_) => {
                                Ok::<List<(Str, Adt66)>, bool>(v2)
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3022 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f779(p3028: List<(Str, Adt66)>) -> bool {
    let m3027 = p3028;
    {
        let mut m3027 = m3027;
        loop {
            let step = {
                let v0 = m3027;
                let m3024 = v0;
                match m3024.uncons() {
                    None => {
                        Err::<List<(Str, Adt66)>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        let m3026 = {
                            let (_, h3025) = v1;
                            h3025
                        };
                        match m3026 {
                            Adt66::C0(_) => {
                                Ok::<List<(Str, Adt66)>, bool>(v2)
                            }
                            Adt66::C1(_) => {
                                Ok::<List<(Str, Adt66)>, bool>(v2)
                            }
                            Adt66::C2(_) => {
                                Err::<List<(Str, Adt66)>, bool>(true)
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3027 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f780() -> Adt19 {
    Adt19::C0(None::<Adt5>, None::<Adt5>, None::<Adt5>, None::<Adt5>, Some(Adt5::C0(List::cons((f566(), Adt6::C5(List::<(Str, Adt6)>::nil())), List::<(Str, Adt6)>::nil()))))
}

pub fn f781(p3031: List<Adt138>, p3032: List<Adt138>) -> List<Adt138> {
    let m3030 = (p3031, p3032);
    {
        let mut m3030 = m3030;
        loop {
            let step = {
                let (v0, v1) = m3030;
                let m3029 = v0;
                match m3029.uncons() {
                    None => {
                        Err::<(List<Adt138>, List<Adt138>), List<Adt138>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt138>, List<Adt138>), List<Adt138>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m3030 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f782(v0: Adt21, p3035: List<Adt52>, p3036: List<Adt138>) -> List<Adt138> {
    let m3034 = (p3035, p3036);
    {
        let mut m3034 = m3034;
        loop {
            let step = {
                let (v1, v2) = m3034;
                let m3033 = v1;
                match m3033.uncons() {
                    None => {
                        Err::<(List<Adt52>, List<Adt138>), List<Adt138>>(v2)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt52>, List<Adt138>), List<Adt138>>((v4, List::cons(Adt138::C0(v0.clone(), v3), v2)))
                    }
                }
            };
            match step {
                Ok(next) => { m3034 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f783(v0: Bytes, v1: Adt148, v2: Str, v3: Adt83) -> R<Result<Adt34, Adt86>> {
    if f725(v1.clone()) {
        if f792(v3.clone()) {
            let m3037 = f793(v0, f561(v1), v2, v3)?;
            match m3037 {
                Ok(v5) => {
                    Ok(Ok::<Adt34, Adt86>(Adt34::C12(v5)))
                }
                Err(v4) => {
                    Ok(Err::<Adt34, Adt86>(v4))
                }
            }
        } else {
            Ok(Err::<Adt34, Adt86>(Adt86::C13))
        }
    } else {
        Ok(Err::<Adt34, Adt86>(Adt86::C12(f561(v1))))
    }
}

pub fn f784(v0: Result<Option<Str>, Adt86>, v1: bool) -> Result<Option<Str>, Adt86> {
    let m3038 = v0;
    match m3038 {
        Ok(v3) => {
            if v1 {
                Ok::<Option<Str>, Adt86>(v3)
            } else {
                Err::<Option<Str>, Adt86>(Adt86::C11)
            }
        }
        Err(v2) => {
            Err::<Option<Str>, Adt86>(v2)
        }
    }
}

pub fn f785(v0: Bytes, v1: Str, v2: Str, v3: Option<Str>) -> R<Result<Option<Str>, Adt86>> {
    let m3039 = v3;
    match m3039 {
        None => {
            Ok(Ok::<Option<Str>, Adt86>(None::<Str>))
        }
        Some(v4) => {
            Ok(f794(f795(v0, v1, v2, v4)?))
        }
    }
}

pub fn f786(v0: Option<Adt5>) -> bool {
    let m3040 = v0;
    match m3040 {
        None => {
            true
        }
        Some(v1) => {
            f796(match v1 {
                Adt5::C0(h3041) => {
                    h3041
                }
            })
        }
    }
}

pub fn f787(v0: Adt13, v1: Str, v2: Adt6) -> R<bool> {
    let m3042 = v0;
    match m3042 {
        Adt13::C0 => {
            let m3043 = f130(v2);
            match m3043 {
                None => {
                    Ok(false)
                }
                Some(v3) => {
                    let a3044 = v1;
                    let a3045 = v3;
                    Ok(equal(a3044, a3045))
                }
            }
        }
        Adt13::C1 => {
            f797(v1, v2)
        }
        Adt13::C2 => {
            let m3046 = f136(v2);
            match m3046 {
                None => {
                    Ok(false)
                }
                Some(v4) => {
                    if v4 {
                        let a3047 = v1;
                        let a3048 = Str::lit("true");
                        Ok(equal(a3047, a3048))
                    } else {
                        let a3049 = v1;
                        let a3050 = Str::lit("false");
                        Ok(equal(a3049, a3050))
                    }
                }
            }
        }
    }
}

pub fn f788(p3053: List<Adt43>, p3054: List<Adt6>) -> List<Adt6> {
    let m3052 = (p3053, p3054);
    {
        let mut m3052 = m3052;
        loop {
            let step = {
                let (v0, v1) = m3052;
                let m3051 = v0;
                match m3051.uncons() {
                    None => {
                        Err::<(List<Adt43>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt43>, List<Adt6>), List<Adt6>>((v3, List::cons(Adt6::C3(f715(v2)), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m3052 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f789(v0: Adt73) -> R<Option<Adt6>> {
    let m3055 = v0;
    match m3055 {
        Adt73::C0(v1) => {
            f798(v1)
        }
        Adt73::C1(v2) => {
            let m3056 = f799(v2, List::<Adt6>::nil())?;
            match m3056 {
                None => {
                    Ok(None::<Adt6>)
                }
                Some(v3) => {
                    Ok(Some(Adt6::C4(f404(v3, List::<Adt6>::nil()))))
                }
            }
        }
    }
}

pub fn f790(v0: Adt3) -> Str {
    let m3057 = v0;
    match m3057 {
        Adt3::C0 => {
            Str::lit("light")
        }
        Adt3::C1 => {
            Str::lit("dark")
        }
    }
}

pub fn f791(v0: u64) -> bool {
    let a3058 = v0;
    let a3059 = 9u64;
    if nat_eq(a3058, a3059) {
        true
    } else {
        let a3060 = v0;
        let a3061 = 10u64;
        if nat_eq(a3060, a3061) {
            true
        } else {
            let a3062 = v0;
            let a3063 = 13u64;
            nat_eq(a3062, a3063)
        }
    }
}

pub fn f792(v0: Adt83) -> bool {
    let m3064 = v0;
    match m3064 {
        Adt83::C0(v1) => {
            let m3065 = v1;
            match m3065.uncons() {
                None => {
                    false
                }
                Some((_, _)) => {
                    true
                }
            }
        }
        Adt83::C1(_) => {
            true
        }
        Adt83::C2(_, _) => {
            true
        }
    }
}

pub fn f793(v0: Bytes, v1: Str, v2: Str, v3: Adt83) -> R<Result<Adt83, Adt86>> {
    let m3066 = v3.clone();
    match m3066 {
        Adt83::C0(_) => {
            Ok(Ok::<Adt83, Adt86>(v3))
        }
        Adt83::C1(v5) => {
            Ok(f800(f801(v0, v1, v2, v5)?, List::<(Str, Adt66)>::nil()))
        }
        Adt83::C2(v6, v7) => {
            Ok(f800(f801(v0, v1, v2, v7)?, v6))
        }
    }
}

pub fn f794(v0: Option<Str>) -> Result<Option<Str>, Adt86> {
    let m3067 = v0;
    match m3067 {
        None => {
            Err::<Option<Str>, Adt86>(Adt86::C10)
        }
        Some(v1) => {
            Ok::<Option<Str>, Adt86>(Some(v1))
        }
    }
}

pub fn f795(v0: Bytes, v1: Str, v2: Str, v3: Str) -> R<Option<Str>> {
    let m3071 = f802(v0, {
        let a3068 = v1;
        utf8_encode(a3068)
    }, {
        let a3069 = v2;
        utf8_encode(a3069)
    }, {
        let a3070 = v3;
        utf8_encode(a3070)
    })?;
    match m3071 {
        None => {
            Ok(None::<Str>)
        }
        Some(v4) => {
            let a3072 = v4;
            Ok(utf8_decode(a3072))
        }
    }
}

pub fn f796(p3077: List<(Str, Adt6)>) -> bool {
    let m3076 = p3077;
    {
        let mut m3076 = m3076;
        loop {
            let step = {
                let v0 = m3076;
                let m3073 = v0;
                match m3073.uncons() {
                    None => {
                        Err::<List<(Str, Adt6)>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let m3075 = f195({
                            let (_, h3074) = v1;
                            h3074
                        });
                        match m3075 {
                            None => {
                                Err::<List<(Str, Adt6)>, bool>(false)
                            }
                            Some(_) => {
                                Ok::<List<(Str, Adt6)>, bool>(v2)
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3076 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f797(v0: Str, v1: Adt6) -> R<bool> {
    let m3078 = f341(v1);
    match m3078 {
        None => {
            Ok(false)
        }
        Some(v2) => {
            let m3079 = f367(v2)?;
            match m3079 {
                None => {
                    Ok(false)
                }
                Some(v3) => {
                    let m3081 = f803(f18({
                        let a3080 = v0;
                        utf8_encode(a3080)
                    })?)?;
                    match m3081 {
                        None => {
                            Ok(false)
                        }
                        Some(v4) => {
                            Ok(f502({
                                let a3082 = v3;
                                let a3083 = v4;
                                int_sub(a3082, a3083)?
                            }))
                        }
                    }
                }
            }
        }
    }
}

pub fn f798(v0: Adt74) -> R<Option<Adt6>> {
    let m3084 = v0;
    match m3084 {
        Adt74::C0(v1) => {
            Ok(Some(f708(v1)))
        }
        Adt74::C1(v2) => {
            Ok(Some(f709(Str::lit("image"), match v2.clone() {
                Adt44::C0(h3085, _, _, _) => {
                    h3085
                }
            }, match v2.clone() {
                Adt44::C0(_, h3086, _, _) => {
                    h3086
                }
            }, match v2.clone() {
                Adt44::C0(_, _, h3087, _) => {
                    h3087
                }
            }, match v2 {
                Adt44::C0(_, _, _, h3088) => {
                    h3088
                }
            })))
        }
        Adt74::C2(v3) => {
            Ok(Some(f709(Str::lit("audio"), match v3.clone() {
                Adt45::C0(h3089, _, _, _) => {
                    h3089
                }
            }, match v3.clone() {
                Adt45::C0(_, h3090, _, _) => {
                    h3090
                }
            }, match v3.clone() {
                Adt45::C0(_, _, h3091, _) => {
                    h3091
                }
            }, match v3 {
                Adt45::C0(_, _, _, h3092) => {
                    h3092
                }
            })))
        }
        Adt74::C3(v4) => {
            Ok(Some(f804(v4)))
        }
        Adt74::C4(v5) => {
            f805(v5)
        }
    }
}

pub fn f799(p3096: List<Adt74>, p3097: List<Adt6>) -> R<Option<List<Adt6>>> {
    let m3095 = (p3096, p3097);
    {
        let mut m3095 = m3095;
        loop {
            let step = {
                let (v0, v1) = m3095;
                let m3093 = v0;
                match m3093.uncons() {
                    None => {
                        Err::<(List<Adt74>, List<Adt6>), R<Option<List<Adt6>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m3094 = f798(v2)?;
                        match m3094 {
                            None => {
                                Err::<(List<Adt74>, List<Adt6>), R<Option<List<Adt6>>>>(Ok(None::<List<Adt6>>))
                            }
                            Some(v4) => {
                                Ok::<(List<Adt74>, List<Adt6>), R<Option<List<Adt6>>>>((v3, List::cons(v4, v1)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3095 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f800(v0: Option<Str>, v1: List<(Str, Adt66)>) -> Result<Adt83, Adt86> {
    let m3098 = v0;
    match m3098 {
        None => {
            Err::<Adt83, Adt86>(Adt86::C14)
        }
        Some(v2) => {
            let m3099 = v1.clone();
            match m3099.uncons() {
                None => {
                    Ok::<Adt83, Adt86>(Adt83::C1(v2))
                }
                Some((_, _)) => {
                    Ok::<Adt83, Adt86>(Adt83::C2(v1, v2))
                }
            }
        }
    }
}

pub fn f801(v0: Bytes, v1: Str, v2: Str, v3: Str) -> R<Option<Str>> {
    let m3103 = f806(v0, {
        let a3100 = v1;
        utf8_encode(a3100)
    }, {
        let a3101 = v2;
        utf8_encode(a3101)
    }, {
        let a3102 = v3;
        utf8_encode(a3102)
    })?;
    match m3103 {
        None => {
            Ok(None::<Str>)
        }
        Some(v4) => {
            let a3104 = v4;
            Ok(utf8_decode(a3104))
        }
    }
}

pub fn f802(v0: Bytes, v1: Bytes, v2: Bytes, v3: Bytes) -> R<Option<Bytes>> {
    let a3106 = f807();
    let a3107 = {
        let a3105 = v0.clone();
        length_bytes(a3105)
    };
    if nat_le(a3106, a3107) {
        let a3109 = f808(v3.clone())?;
        let a3110 = {
            let a3108 = v3.clone();
            length_bytes(a3108)
        };
        if nat_eq(a3109, a3110) {
            Ok(None::<Bytes>)
        } else {
            f809(v0, v1, v2, v3.clone(), f808(v3)?)
        }
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f803(v0: List<u64>) -> R<Option<i64>> {
    let m3111 = v0.clone();
    match m3111.uncons() {
        None => {
            Ok(None::<i64>)
        }
        Some((v1, v2)) => {
            let a3112 = v1;
            let a3113 = 45u64;
            if nat_eq(a3112, a3113) {
                let m3114 = v2.clone();
                match m3114.uncons() {
                    None => {
                        Ok(None::<i64>)
                    }
                    Some((_, _)) => {
                        let m3115 = f810(v2, 0i64)?;
                        match m3115 {
                            None => {
                                Ok(None::<i64>)
                            }
                            Some(v5) => {
                                Ok(Some({
                                    let a3116 = v5;
                                    int_neg(a3116)?
                                }))
                            }
                        }
                    }
                }
            } else {
                f810(v0, 0i64)
            }
        }
    }
}

pub fn f804(v0: Adt75) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("tool_use"))), List::cons((Str::lit("id"), Adt6::C3(match v0.clone() {
        Adt75::C0(h3117, _, _, _) => {
            h3117
        }
    })), List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
        Adt75::C0(_, h3118, _, _) => {
            h3118
        }
    })), List::cons((Str::lit("input"), Adt6::C5({
        let m3121 = match v0.clone() {
            Adt75::C0(_, _, h3120, _) => {
                h3120
            }
        };
        match m3121 {
            Adt5::C0(h3119) => {
                h3119
            }
        }
    })), f707(match v0 {
        Adt75::C0(_, _, _, h3122) => {
            h3122
        }
    }))))))
}

pub fn f805(v0: Adt76) -> R<Option<Adt6>> {
    let m3124 = f586(match v0.clone() {
        Adt76::C0(_, h3123, _, _, _) => {
            h3123
        }
    })?;
    match m3124 {
        None => {
            Ok(None::<Adt6>)
        }
        Some(v1) => {
            Ok(Some(Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("tool_result"))), List::cons((Str::lit("toolUseId"), Adt6::C3(match v0.clone() {
                Adt76::C0(h3125, _, _, _, _) => {
                    h3125
                }
            })), List::cons((Str::lit("content"), Adt6::C4(v1)), {
                let a3131 = f587(Str::lit("structuredContent"), match v0.clone() {
                    Adt76::C0(_, _, h3126, _, _) => {
                        h3126
                    }
                });
                let a3132 = {
                    let a3129 = f588(Str::lit("isError"), match v0.clone() {
                        Adt76::C0(_, _, _, h3127, _) => {
                            h3127
                        }
                    });
                    let a3130 = f707(match v0 {
                        Adt76::C0(_, _, _, _, h3128) => {
                            h3128
                        }
                    });
                    append_list(a3129, a3130)
                };
                append_list(a3131, a3132)
            }))))))
        }
    }
}

pub fn f806(v0: Bytes, v1: Bytes, v2: Bytes, v3: Bytes) -> R<Option<Bytes>> {
    let a3134 = f807();
    let a3135 = {
        let a3133 = v0.clone();
        length_bytes(a3133)
    };
    if nat_le(a3134, a3135) {
        Ok(Some({
            let a3138 = f811(v3.clone())?;
            let a3139 = {
                let a3136 = Bytes::lit(&[0x2e]);
                let a3137 = f811(f812(v0, v1, v2, v3)?)?;
                append_bytes(a3136, a3137)
            };
            append_bytes(a3138, a3139)
        }))
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f807() -> u64 {
    32u64
}

pub fn f808(v0: Bytes) -> R<u64> {
    let (_, h3140) = f813(Fn4::F814(v0.clone()), {
        let a3141 = v0.clone();
        length_bytes(a3141)
    }, (0u64, {
        let a3142 = v0;
        length_bytes(a3142)
    }))?;
    Ok(h3140)
}

pub fn f809(v0: Bytes, v1: Bytes, v2: Bytes, v3: Bytes, v4: u64) -> R<Option<Bytes>> {
    let m3146 = {
        let a3143 = v3.clone();
        let a3144 = 0u64;
        let a3145 = v4;
        slice_bytes(a3143, a3144, a3145)
    };
    match m3146 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v5) => {
            let m3157 = {
                let a3154 = v3.clone();
                let a3155 = {
                    let a3147 = v4;
                    let a3148 = 1u64;
                    nat_add(a3147, a3148)?
                };
                let a3156 = {
                    let a3152 = {
                        let a3149 = v3;
                        length_bytes(a3149)
                    };
                    let a3153 = {
                        let a3150 = v4;
                        let a3151 = 1u64;
                        nat_add(a3150, a3151)?
                    };
                    nat_sub(a3152, a3153)
                };
                slice_bytes(a3154, a3155, a3156)
            };
            match m3157 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v6) => {
                    f815(v0, v1, v2, f816(v5)?, f816(v6)?)
                }
            }
        }
    }
}

pub fn f810(p3163: List<u64>, p3164: i64) -> R<Option<i64>> {
    let m3162 = (p3163, p3164);
    {
        let mut m3162 = m3162;
        loop {
            let step = {
                let (v0, v1) = m3162;
                let m3158 = v0;
                match m3158.uncons() {
                    None => {
                        Err::<(List<u64>, i64), R<Option<i64>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m3159 = f817(v2);
                        match m3159 {
                            None => {
                                Err::<(List<u64>, i64), R<Option<i64>>>(Ok(None::<i64>))
                            }
                            Some(v4) => {
                                Ok::<(List<u64>, i64), R<Option<i64>>>((v3, f193({
                                    let a3160 = v1;
                                    let a3161 = 10i64;
                                    int_mul(a3160, a3161)?
                                }, f403(v4))?))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3162 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f811(v0: Bytes) -> R<Bytes> {
    let (_, h3165) = f818(Fn5::F819(v0.clone()), {
        let a3166 = v0;
        length_bytes(a3166)
    }, (0u64, f95()))?;
    Ok(h3165)
}

pub fn f812(v0: Bytes, v1: Bytes, v2: Bytes, v3: Bytes) -> R<Bytes> {
    f820(v0, {
        let a3167 = f821(v1, v2);
        let a3168 = v3;
        append_bytes(a3167, a3168)
    })
}

pub fn f813(v0: Fn4, p3172: u64, p3173: (u64, u64)) -> R<(u64, u64)> {
    let m3171 = (p3172, p3173);
    {
        let mut m3171 = m3171;
        loop {
            let step = {
                let (v1, v2) = m3171;
                let m3169 = v1;
                if m3169 == 0 {
                    Err::<(u64, (u64, u64)), R<(u64, u64)>>(Ok(v2))
                } else {
                    let v3 = m3169 - 1;
                    Ok::<(u64, (u64, u64)), R<(u64, u64)>>((v3, {
                        let c3170: Fn4 = v0.clone();
                        c3170.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m3171 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f814(v0: Bytes, v1: (u64, u64)) -> R<(u64, u64)> {
    Ok(({
        let a3175 = {
            let (h3174, _) = v1;
            h3174
        };
        let a3176 = 1u64;
        nat_add(a3175, a3176)?
    }, {
        let a3179 = {
            let (_, h3177) = v1;
            h3177
        };
        let a3180 = {
            let a3178 = v0.clone();
            length_bytes(a3178)
        };
        if nat_eq(a3179, a3180) {
            let a3182 = f63(v0, {
                let (h3181, _) = v1;
                h3181
            })?;
            let a3183 = 46u64;
            if nat_eq(a3182, a3183) {
                let (h3184, _) = v1;
                h3184
            } else {
                let (_, h3185) = v1;
                h3185
            }
        } else {
            let (_, h3186) = v1;
            h3186
        }
    }))
}

pub fn f815(v0: Bytes, v1: Bytes, v2: Bytes, v3: Option<Bytes>, v4: Option<Bytes>) -> R<Option<Bytes>> {
    let m3187 = v3;
    match m3187 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v5) => {
            let m3188 = v4;
            match m3188 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v6) => {
                    if f822(f812(v0, v1, v2, v5.clone())?, v6)? {
                        Ok(Some(v5))
                    } else {
                        Ok(None::<Bytes>)
                    }
                }
            }
        }
    }
}

pub fn f816(v0: Bytes) -> R<Option<Bytes>> {
    let a3193 = {
        let a3190 = {
            let a3189 = v0.clone();
            length_bytes(a3189)
        };
        let a3191 = 2u64;
        let a3192 = 0u64;
        nat_rem(a3190, a3191, a3192)
    };
    let a3194 = 0u64;
    if nat_eq(a3193, a3194) {
        let v1: (u64, (Bytes, u64)) = f823(Fn6::F824(v0.clone()), {
            let a3196 = {
                let a3195 = v0;
                length_bytes(a3195)
            };
            let a3197 = 2u64;
            let a3198 = 0u64;
            nat_quot(a3196, a3197, a3198)
        }, (0u64, (f95(), 0u64)))?;
        let a3201 = {
            let (_, h3199) = {
                let (_, h3200) = v1.clone();
                h3200
            };
            h3199
        };
        let a3202 = 0u64;
        if nat_eq(a3201, a3202) {
            Ok(Some({
                let (h3203, _) = {
                    let (_, h3204) = v1;
                    h3204
                };
                h3203
            }))
        } else {
            Ok(None::<Bytes>)
        }
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f817(v0: u64) -> Option<u64> {
    let a3205 = v0;
    let a3206 = 48u64;
    if if nat_lt(a3205, a3206) {
        true
    } else {
        let a3207 = 57u64;
        let a3208 = v0;
        nat_lt(a3207, a3208)
    } {
        None::<u64>
    } else {
        Some({
            let a3209 = v0;
            let a3210 = 48u64;
            nat_sub(a3209, a3210)
        })
    }
}

pub fn f818(v0: Fn5, p3214: u64, p3215: (u64, Bytes)) -> R<(u64, Bytes)> {
    let m3213 = (p3214, p3215);
    {
        let mut m3213 = m3213;
        loop {
            let step = {
                let (v1, v2) = m3213;
                let m3211 = v1;
                if m3211 == 0 {
                    Err::<(u64, (u64, Bytes)), R<(u64, Bytes)>>(Ok(v2))
                } else {
                    let v3 = m3211 - 1;
                    Ok::<(u64, (u64, Bytes)), R<(u64, Bytes)>>((v3, {
                        let c3212: Fn5 = v0.clone();
                        c3212.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m3213 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f819(v0: Bytes, v1: (u64, Bytes)) -> R<(u64, Bytes)> {
    Ok(({
        let a3217 = {
            let (h3216, _) = v1.clone();
            h3216
        };
        let a3218 = 1u64;
        nat_add(a3217, a3218)?
    }, {
        let a3221 = {
            let (_, h3219) = v1;
            h3219
        };
        let a3222 = f825(f63(v0, {
            let (h3220, _) = v1;
            h3220
        })?)?;
        append_bytes(a3221, a3222)
    }))
}

pub fn f820(v0: Bytes, v1: Bytes) -> R<Bytes> {
    f826(f827(v0, v1)?)
}

pub fn f821(v0: Bytes, v1: Bytes) -> Bytes {
    let a3229 = f828({
        let a3223 = v0.clone();
        length_bytes(a3223)
    });
    let a3230 = {
        let a3227 = v0;
        let a3228 = {
            let a3225 = f828({
                let a3224 = v1.clone();
                length_bytes(a3224)
            });
            let a3226 = v1;
            append_bytes(a3225, a3226)
        };
        append_bytes(a3227, a3228)
    };
    append_bytes(a3229, a3230)
}

pub fn f822(v0: Bytes, v1: Bytes) -> R<bool> {
    let a3233 = {
        let a3231 = v0.clone();
        length_bytes(a3231)
    };
    let a3234 = {
        let a3232 = v1.clone();
        length_bytes(a3232)
    };
    if nat_eq(a3233, a3234) {
        let a3235 = f829(v0, v1)?;
        let a3236 = 0u64;
        Ok(nat_eq(a3235, a3236))
    } else {
        Ok(false)
    }
}

pub fn f823(v0: Fn6, p3240: u64, p3241: (u64, (Bytes, u64))) -> R<(u64, (Bytes, u64))> {
    let m3239 = (p3240, p3241);
    {
        let mut m3239 = m3239;
        loop {
            let step = {
                let (v1, v2) = m3239;
                let m3237 = v1;
                if m3237 == 0 {
                    Err::<(u64, (u64, (Bytes, u64))), R<(u64, (Bytes, u64))>>(Ok(v2))
                } else {
                    let v3 = m3237 - 1;
                    Ok::<(u64, (u64, (Bytes, u64))), R<(u64, (Bytes, u64))>>((v3, {
                        let c3238: Fn6 = v0.clone();
                        c3238.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m3239 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f824(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<(u64, (Bytes, u64))> {
    f830(v0, v1)
}

pub fn f825(v0: u64) -> R<Bytes> {
    let a3248 = f831({
        let a3242 = v0;
        let a3243 = 16u64;
        let a3244 = 0u64;
        nat_quot(a3242, a3243, a3244)
    })?;
    let a3249 = f831({
        let a3245 = v0;
        let a3246 = 16u64;
        let a3247 = 0u64;
        nat_rem(a3245, a3246, a3247)
    })?;
    Ok(append_bytes(a3248, a3249))
}

pub fn f826(v0: Adt159) -> R<Bytes> {
    let a3270 = f832(match v0.clone() {
        Adt159::C0(h3250, _, _, _, _, _, _, _) => {
            h3250
        }
    })?;
    let a3271 = {
        let a3268 = f832(match v0.clone() {
            Adt159::C0(_, h3251, _, _, _, _, _, _) => {
                h3251
            }
        })?;
        let a3269 = {
            let a3266 = f832(match v0.clone() {
                Adt159::C0(_, _, h3252, _, _, _, _, _) => {
                    h3252
                }
            })?;
            let a3267 = {
                let a3264 = f832(match v0.clone() {
                    Adt159::C0(_, _, _, h3253, _, _, _, _) => {
                        h3253
                    }
                })?;
                let a3265 = {
                    let a3262 = f832(match v0.clone() {
                        Adt159::C0(_, _, _, _, h3254, _, _, _) => {
                            h3254
                        }
                    })?;
                    let a3263 = {
                        let a3260 = f832(match v0.clone() {
                            Adt159::C0(_, _, _, _, _, h3255, _, _) => {
                                h3255
                            }
                        })?;
                        let a3261 = {
                            let a3258 = f832(match v0.clone() {
                                Adt159::C0(_, _, _, _, _, _, h3256, _) => {
                                    h3256
                                }
                            })?;
                            let a3259 = f832(match v0 {
                                Adt159::C0(_, _, _, _, _, _, _, h3257) => {
                                    h3257
                                }
                            })?;
                            append_bytes(a3258, a3259)
                        };
                        append_bytes(a3260, a3261)
                    };
                    append_bytes(a3262, a3263)
                };
                append_bytes(a3264, a3265)
            };
            append_bytes(a3266, a3267)
        };
        append_bytes(a3268, a3269)
    };
    Ok(append_bytes(a3270, a3271))
}

pub fn f827(v0: Bytes, v1: Bytes) -> R<Adt159> {
    let v2: Adt160 = f833(v0)?;
    let v3: Adt159 = f834(f835(f836(), f837(v2.clone(), 909522486u32))?, 64u64, v1)?;
    f835(f835(f836(), f837(v2, 1549556828u32))?, f838(v3))
}

pub fn f828(v0: u64) -> Bytes {
    let a3297 = f190({
        let a3275 = {
            let a3272 = v0;
            let a3273 = 16777216u64;
            let a3274 = 0u64;
            nat_quot(a3272, a3273, a3274)
        };
        let a3276 = 256u64;
        let a3277 = 0u64;
        nat_rem(a3275, a3276, a3277)
    });
    let a3298 = {
        let a3295 = f190({
            let a3281 = {
                let a3278 = v0;
                let a3279 = 65536u64;
                let a3280 = 0u64;
                nat_quot(a3278, a3279, a3280)
            };
            let a3282 = 256u64;
            let a3283 = 0u64;
            nat_rem(a3281, a3282, a3283)
        });
        let a3296 = {
            let a3293 = f190({
                let a3287 = {
                    let a3284 = v0;
                    let a3285 = 256u64;
                    let a3286 = 0u64;
                    nat_quot(a3284, a3285, a3286)
                };
                let a3288 = 256u64;
                let a3289 = 0u64;
                nat_rem(a3287, a3288, a3289)
            });
            let a3294 = f190({
                let a3290 = v0;
                let a3291 = 256u64;
                let a3292 = 0u64;
                nat_rem(a3290, a3291, a3292)
            });
            append_bytes(a3293, a3294)
        };
        append_bytes(a3295, a3296)
    };
    append_bytes(a3297, a3298)
}

pub fn f829(v0: Bytes, v1: Bytes) -> R<u64> {
    let (_, h3299) = f813(Fn4::F839(v0.clone(), v1), {
        let a3300 = v0;
        length_bytes(a3300)
    }, (0u64, 0u64))?;
    Ok(h3299)
}

pub fn f830(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<(u64, (Bytes, u64))> {
    let v2: u64 = f840(f63(v0.clone(), {
        let (h3301, _) = v1.clone();
        h3301
    })?);
    let v3: u64 = f840(f63(v0, {
        let a3303 = {
            let (h3302, _) = v1.clone();
            h3302
        };
        let a3304 = 1u64;
        nat_add(a3303, a3304)?
    })?);
    let a3305 = v2;
    let a3306 = 16u64;
    if if nat_lt(a3305, a3306) {
        let a3307 = v3;
        let a3308 = 16u64;
        nat_lt(a3307, a3308)
    } else {
        false
    } {
        Ok(({
            let a3310 = {
                let (h3309, _) = v1.clone();
                h3309
            };
            let a3311 = 2u64;
            nat_add(a3310, a3311)?
        }, ({
            let a3318 = {
                let (h3312, _) = {
                    let (_, h3313) = v1.clone();
                    h3313
                };
                h3312
            };
            let a3319 = f190({
                let a3316 = {
                    let a3314 = v2;
                    let a3315 = 16u64;
                    nat_mul(a3314, a3315)?
                };
                let a3317 = v3;
                nat_add(a3316, a3317)?
            });
            append_bytes(a3318, a3319)
        }, {
            let (_, h3320) = {
                let (_, h3321) = v1;
                h3321
            };
            h3320
        })))
    } else {
        Ok(({
            let a3323 = {
                let (h3322, _) = v1.clone();
                h3322
            };
            let a3324 = 2u64;
            nat_add(a3323, a3324)?
        }, ({
            let (h3325, _) = {
                let (_, h3326) = v1.clone();
                h3326
            };
            h3325
        }, {
            let a3329 = {
                let (_, h3327) = {
                    let (_, h3328) = v1;
                    h3328
                };
                h3327
            };
            let a3330 = 1u64;
            nat_add(a3329, a3330)?
        })))
    }
}

pub fn f831(v0: u64) -> R<Bytes> {
    Ok(f190({
        let a3331 = v0;
        let a3332 = 10u64;
        if nat_lt(a3331, a3332) {
            let a3333 = 48u64;
            let a3334 = v0;
            nat_add(a3333, a3334)?
        } else {
            let a3335 = 87u64;
            let a3336 = v0;
            nat_add(a3335, a3336)?
        }
    }))
}

pub fn f832(v0: u32) -> R<Bytes> {
    let a3341 = f841(v0, 24u32)?;
    let a3342 = {
        let a3339 = f841(v0, 16u32)?;
        let a3340 = {
            let a3337 = f841(v0, 8u32)?;
            let a3338 = f841(v0, 0u32)?;
            append_bytes(a3337, a3338)
        };
        append_bytes(a3339, a3340)
    };
    Ok(append_bytes(a3341, a3342))
}

pub fn f833(v0: Bytes) -> R<Adt160> {
    let a3344 = f842();
    let a3345 = {
        let a3343 = v0.clone();
        length_bytes(a3343)
    };
    if nat_lt(a3344, a3345) {
        Ok(f843(f844(v0)?))
    } else {
        f845(v0)
    }
}

pub fn f834(v0: Adt159, v1: u64, v2: Bytes) -> R<Adt159> {
    let v3: u64 = {
        let a3346 = v2.clone();
        length_bytes(a3346)
    };
    let v4: u64 = {
        let a3352 = {
            let a3349 = {
                let a3347 = v3;
                let a3348 = 72u64;
                nat_add(a3347, a3348)?
            };
            let a3350 = 64u64;
            let a3351 = 0u64;
            nat_quot(a3349, a3350, a3351)
        };
        let a3353 = 64u64;
        nat_mul(a3352, a3353)?
    };
    let (_, h3354) = f846(Fn7::F847(v2, v1, v3, v4), {
        let a3355 = v4;
        let a3356 = 64u64;
        let a3357 = 0u64;
        nat_quot(a3355, a3356, a3357)
    }, (0u64, v0))?;
    Ok(h3354)
}

pub fn f835(v0: Adt159, v1: Adt160) -> R<Adt159> {
    let v2: Adt161 = f848(Fn8::F849, 64u64, Adt161::C0(match v0.clone() {
        Adt159::C0(h3358, _, _, _, _, _, _, _) => {
            h3358
        }
    }, match v0.clone() {
        Adt159::C0(_, h3359, _, _, _, _, _, _) => {
            h3359
        }
    }, match v0.clone() {
        Adt159::C0(_, _, h3360, _, _, _, _, _) => {
            h3360
        }
    }, match v0.clone() {
        Adt159::C0(_, _, _, h3361, _, _, _, _) => {
            h3361
        }
    }, match v0.clone() {
        Adt159::C0(_, _, _, _, h3362, _, _, _) => {
            h3362
        }
    }, match v0.clone() {
        Adt159::C0(_, _, _, _, _, h3363, _, _) => {
            h3363
        }
    }, match v0.clone() {
        Adt159::C0(_, _, _, _, _, _, h3364, _) => {
            h3364
        }
    }, match v0.clone() {
        Adt159::C0(_, _, _, _, _, _, _, h3365) => {
            h3365
        }
    }, v1, 0u64))?;
    Ok(Adt159::C0(f850(match v0.clone() {
        Adt159::C0(h3366, _, _, _, _, _, _, _) => {
            h3366
        }
    }, match v2.clone() {
        Adt161::C0(h3367, _, _, _, _, _, _, _, _, _) => {
            h3367
        }
    }), f850(match v0.clone() {
        Adt159::C0(_, h3368, _, _, _, _, _, _) => {
            h3368
        }
    }, match v2.clone() {
        Adt161::C0(_, h3369, _, _, _, _, _, _, _, _) => {
            h3369
        }
    }), f850(match v0.clone() {
        Adt159::C0(_, _, h3370, _, _, _, _, _) => {
            h3370
        }
    }, match v2.clone() {
        Adt161::C0(_, _, h3371, _, _, _, _, _, _, _) => {
            h3371
        }
    }), f850(match v0.clone() {
        Adt159::C0(_, _, _, h3372, _, _, _, _) => {
            h3372
        }
    }, match v2.clone() {
        Adt161::C0(_, _, _, h3373, _, _, _, _, _, _) => {
            h3373
        }
    }), f850(match v0.clone() {
        Adt159::C0(_, _, _, _, h3374, _, _, _) => {
            h3374
        }
    }, match v2.clone() {
        Adt161::C0(_, _, _, _, h3375, _, _, _, _, _) => {
            h3375
        }
    }), f850(match v0.clone() {
        Adt159::C0(_, _, _, _, _, h3376, _, _) => {
            h3376
        }
    }, match v2.clone() {
        Adt161::C0(_, _, _, _, _, h3377, _, _, _, _) => {
            h3377
        }
    }), f850(match v0.clone() {
        Adt159::C0(_, _, _, _, _, _, h3378, _) => {
            h3378
        }
    }, match v2.clone() {
        Adt161::C0(_, _, _, _, _, _, h3379, _, _, _) => {
            h3379
        }
    }), f850(match v0 {
        Adt159::C0(_, _, _, _, _, _, _, h3380) => {
            h3380
        }
    }, match v2 {
        Adt161::C0(_, _, _, _, _, _, _, h3381, _, _) => {
            h3381
        }
    })))
}

pub fn f836() -> Adt159 {
    Adt159::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f837(v0: Adt160, v1: u32) -> Adt160 {
    Adt160::C0({
        let a3383 = match v0.clone() {
            Adt160::C0(h3382, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h3382
            }
        };
        let a3384 = v1;
        fixed_u32::bit_xor(a3383, a3384)
    }, {
        let a3386 = match v0.clone() {
            Adt160::C0(_, h3385, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h3385
            }
        };
        let a3387 = v1;
        fixed_u32::bit_xor(a3386, a3387)
    }, {
        let a3389 = match v0.clone() {
            Adt160::C0(_, _, h3388, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h3388
            }
        };
        let a3390 = v1;
        fixed_u32::bit_xor(a3389, a3390)
    }, {
        let a3392 = match v0.clone() {
            Adt160::C0(_, _, _, h3391, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h3391
            }
        };
        let a3393 = v1;
        fixed_u32::bit_xor(a3392, a3393)
    }, {
        let a3395 = match v0.clone() {
            Adt160::C0(_, _, _, _, h3394, _, _, _, _, _, _, _, _, _, _, _) => {
                h3394
            }
        };
        let a3396 = v1;
        fixed_u32::bit_xor(a3395, a3396)
    }, {
        let a3398 = match v0.clone() {
            Adt160::C0(_, _, _, _, _, h3397, _, _, _, _, _, _, _, _, _, _) => {
                h3397
            }
        };
        let a3399 = v1;
        fixed_u32::bit_xor(a3398, a3399)
    }, {
        let a3401 = match v0.clone() {
            Adt160::C0(_, _, _, _, _, _, h3400, _, _, _, _, _, _, _, _, _) => {
                h3400
            }
        };
        let a3402 = v1;
        fixed_u32::bit_xor(a3401, a3402)
    }, {
        let a3404 = match v0.clone() {
            Adt160::C0(_, _, _, _, _, _, _, h3403, _, _, _, _, _, _, _, _) => {
                h3403
            }
        };
        let a3405 = v1;
        fixed_u32::bit_xor(a3404, a3405)
    }, {
        let a3407 = match v0.clone() {
            Adt160::C0(_, _, _, _, _, _, _, _, h3406, _, _, _, _, _, _, _) => {
                h3406
            }
        };
        let a3408 = v1;
        fixed_u32::bit_xor(a3407, a3408)
    }, {
        let a3410 = match v0.clone() {
            Adt160::C0(_, _, _, _, _, _, _, _, _, h3409, _, _, _, _, _, _) => {
                h3409
            }
        };
        let a3411 = v1;
        fixed_u32::bit_xor(a3410, a3411)
    }, {
        let a3413 = match v0.clone() {
            Adt160::C0(_, _, _, _, _, _, _, _, _, _, h3412, _, _, _, _, _) => {
                h3412
            }
        };
        let a3414 = v1;
        fixed_u32::bit_xor(a3413, a3414)
    }, {
        let a3416 = match v0.clone() {
            Adt160::C0(_, _, _, _, _, _, _, _, _, _, _, h3415, _, _, _, _) => {
                h3415
            }
        };
        let a3417 = v1;
        fixed_u32::bit_xor(a3416, a3417)
    }, {
        let a3419 = match v0.clone() {
            Adt160::C0(_, _, _, _, _, _, _, _, _, _, _, _, h3418, _, _, _) => {
                h3418
            }
        };
        let a3420 = v1;
        fixed_u32::bit_xor(a3419, a3420)
    }, {
        let a3422 = match v0.clone() {
            Adt160::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h3421, _, _) => {
                h3421
            }
        };
        let a3423 = v1;
        fixed_u32::bit_xor(a3422, a3423)
    }, {
        let a3425 = match v0.clone() {
            Adt160::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h3424, _) => {
                h3424
            }
        };
        let a3426 = v1;
        fixed_u32::bit_xor(a3425, a3426)
    }, {
        let a3428 = match v0 {
            Adt160::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h3427) => {
                h3427
            }
        };
        let a3429 = v1;
        fixed_u32::bit_xor(a3428, a3429)
    })
}

pub fn f838(v0: Adt159) -> Adt160 {
    Adt160::C0(match v0.clone() {
        Adt159::C0(h3430, _, _, _, _, _, _, _) => {
            h3430
        }
    }, match v0.clone() {
        Adt159::C0(_, h3431, _, _, _, _, _, _) => {
            h3431
        }
    }, match v0.clone() {
        Adt159::C0(_, _, h3432, _, _, _, _, _) => {
            h3432
        }
    }, match v0.clone() {
        Adt159::C0(_, _, _, h3433, _, _, _, _) => {
            h3433
        }
    }, match v0.clone() {
        Adt159::C0(_, _, _, _, h3434, _, _, _) => {
            h3434
        }
    }, match v0.clone() {
        Adt159::C0(_, _, _, _, _, h3435, _, _) => {
            h3435
        }
    }, match v0.clone() {
        Adt159::C0(_, _, _, _, _, _, h3436, _) => {
            h3436
        }
    }, match v0 {
        Adt159::C0(_, _, _, _, _, _, _, h3437) => {
            h3437
        }
    }, 2147483648u32, f851(), f851(), f851(), f851(), f851(), f851(), 768u32)
}

pub fn f839(v0: Bytes, v1: Bytes, v2: (u64, u64)) -> R<(u64, u64)> {
    Ok(({
        let a3439 = {
            let (h3438, _) = v2;
            h3438
        };
        let a3440 = 1u64;
        nat_add(a3439, a3440)?
    }, {
        let a3492 = {
            let (_, h3441) = v2;
            h3441
        };
        let a3493 = {
            let v3: u8 = {
                let a3444 = f852(v0, {
                    let (h3442, _) = v2;
                    h3442
                });
                let a3445 = f852(v1, {
                    let (h3443, _) = v2;
                    h3443
                });
                fixed_u8::bit_xor(a3444, a3445)
            };
            let a3490 = {
                let a3466 = {
                    let a3454 = {
                        let a3448 = {
                            let a3446 = v3;
                            let a3447 = 128u8;
                            fixed_u8::bit_and(a3446, a3447)
                        };
                        let a3449 = 128u8;
                        if equal(a3448, a3449) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a3455 = {
                        let a3452 = {
                            let a3450 = v3;
                            let a3451 = 64u8;
                            fixed_u8::bit_and(a3450, a3451)
                        };
                        let a3453 = 64u8;
                        if equal(a3452, a3453) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a3454, a3455)?
                };
                let a3467 = {
                    let a3464 = {
                        let a3458 = {
                            let a3456 = v3;
                            let a3457 = 32u8;
                            fixed_u8::bit_and(a3456, a3457)
                        };
                        let a3459 = 32u8;
                        if equal(a3458, a3459) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a3465 = {
                        let a3462 = {
                            let a3460 = v3;
                            let a3461 = 16u8;
                            fixed_u8::bit_and(a3460, a3461)
                        };
                        let a3463 = 16u8;
                        if equal(a3462, a3463) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a3464, a3465)?
                };
                nat_add(a3466, a3467)?
            };
            let a3491 = {
                let a3488 = {
                    let a3476 = {
                        let a3470 = {
                            let a3468 = v3;
                            let a3469 = 8u8;
                            fixed_u8::bit_and(a3468, a3469)
                        };
                        let a3471 = 8u8;
                        if equal(a3470, a3471) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a3477 = {
                        let a3474 = {
                            let a3472 = v3;
                            let a3473 = 4u8;
                            fixed_u8::bit_and(a3472, a3473)
                        };
                        let a3475 = 4u8;
                        if equal(a3474, a3475) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a3476, a3477)?
                };
                let a3489 = {
                    let a3486 = {
                        let a3480 = {
                            let a3478 = v3;
                            let a3479 = 2u8;
                            fixed_u8::bit_and(a3478, a3479)
                        };
                        let a3481 = 2u8;
                        if equal(a3480, a3481) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a3487 = {
                        let a3484 = {
                            let a3482 = v3;
                            let a3483 = 1u8;
                            fixed_u8::bit_and(a3482, a3483)
                        };
                        let a3485 = 1u8;
                        if equal(a3484, a3485) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a3486, a3487)?
                };
                nat_add(a3488, a3489)?
            };
            nat_add(a3490, a3491)?
        };
        nat_add(a3492, a3493)?
    }))
}

pub fn f840(v0: u64) -> u64 {
    let a3494 = v0;
    let a3495 = 48u64;
    if nat_lt(a3494, a3495) {
        16u64
    } else {
        let a3496 = v0;
        let a3497 = 58u64;
        if nat_lt(a3496, a3497) {
            let a3498 = v0;
            let a3499 = 48u64;
            nat_sub(a3498, a3499)
        } else {
            let a3500 = v0;
            let a3501 = 97u64;
            if nat_lt(a3500, a3501) {
                16u64
            } else {
                let a3502 = v0;
                let a3503 = 103u64;
                if nat_lt(a3502, a3503) {
                    let a3504 = v0;
                    let a3505 = 87u64;
                    nat_sub(a3504, a3505)
                } else {
                    16u64
                }
            }
        }
    }
}

pub fn f841(v0: u32, v1: u32) -> R<Bytes> {
    Ok(f190({
        let v2: u32 = {
            let a3506 = f853(v0, v1);
            let a3507 = 255u32;
            fixed_u32::bit_and(a3506, a3507)
        };
        let a3696 = {
            let a3600 = {
                let a3552 = {
                    let a3528 = {
                        let a3516 = {
                            let a3510 = {
                                let a3508 = v2;
                                let a3509 = 2147483648u32;
                                fixed_u32::bit_and(a3508, a3509)
                            };
                            let a3511 = 2147483648u32;
                            if equal(a3510, a3511) {
                                2147483648u64
                            } else {
                                0u64
                            }
                        };
                        let a3517 = {
                            let a3514 = {
                                let a3512 = v2;
                                let a3513 = 1073741824u32;
                                fixed_u32::bit_and(a3512, a3513)
                            };
                            let a3515 = 1073741824u32;
                            if equal(a3514, a3515) {
                                1073741824u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3516, a3517)?
                    };
                    let a3529 = {
                        let a3526 = {
                            let a3520 = {
                                let a3518 = v2;
                                let a3519 = 536870912u32;
                                fixed_u32::bit_and(a3518, a3519)
                            };
                            let a3521 = 536870912u32;
                            if equal(a3520, a3521) {
                                536870912u64
                            } else {
                                0u64
                            }
                        };
                        let a3527 = {
                            let a3524 = {
                                let a3522 = v2;
                                let a3523 = 268435456u32;
                                fixed_u32::bit_and(a3522, a3523)
                            };
                            let a3525 = 268435456u32;
                            if equal(a3524, a3525) {
                                268435456u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3526, a3527)?
                    };
                    nat_add(a3528, a3529)?
                };
                let a3553 = {
                    let a3550 = {
                        let a3538 = {
                            let a3532 = {
                                let a3530 = v2;
                                let a3531 = 134217728u32;
                                fixed_u32::bit_and(a3530, a3531)
                            };
                            let a3533 = 134217728u32;
                            if equal(a3532, a3533) {
                                134217728u64
                            } else {
                                0u64
                            }
                        };
                        let a3539 = {
                            let a3536 = {
                                let a3534 = v2;
                                let a3535 = 67108864u32;
                                fixed_u32::bit_and(a3534, a3535)
                            };
                            let a3537 = 67108864u32;
                            if equal(a3536, a3537) {
                                67108864u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3538, a3539)?
                    };
                    let a3551 = {
                        let a3548 = {
                            let a3542 = {
                                let a3540 = v2;
                                let a3541 = 33554432u32;
                                fixed_u32::bit_and(a3540, a3541)
                            };
                            let a3543 = 33554432u32;
                            if equal(a3542, a3543) {
                                33554432u64
                            } else {
                                0u64
                            }
                        };
                        let a3549 = {
                            let a3546 = {
                                let a3544 = v2;
                                let a3545 = 16777216u32;
                                fixed_u32::bit_and(a3544, a3545)
                            };
                            let a3547 = 16777216u32;
                            if equal(a3546, a3547) {
                                16777216u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3548, a3549)?
                    };
                    nat_add(a3550, a3551)?
                };
                nat_add(a3552, a3553)?
            };
            let a3601 = {
                let a3598 = {
                    let a3574 = {
                        let a3562 = {
                            let a3556 = {
                                let a3554 = v2;
                                let a3555 = 8388608u32;
                                fixed_u32::bit_and(a3554, a3555)
                            };
                            let a3557 = 8388608u32;
                            if equal(a3556, a3557) {
                                8388608u64
                            } else {
                                0u64
                            }
                        };
                        let a3563 = {
                            let a3560 = {
                                let a3558 = v2;
                                let a3559 = 4194304u32;
                                fixed_u32::bit_and(a3558, a3559)
                            };
                            let a3561 = 4194304u32;
                            if equal(a3560, a3561) {
                                4194304u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3562, a3563)?
                    };
                    let a3575 = {
                        let a3572 = {
                            let a3566 = {
                                let a3564 = v2;
                                let a3565 = 2097152u32;
                                fixed_u32::bit_and(a3564, a3565)
                            };
                            let a3567 = 2097152u32;
                            if equal(a3566, a3567) {
                                2097152u64
                            } else {
                                0u64
                            }
                        };
                        let a3573 = {
                            let a3570 = {
                                let a3568 = v2;
                                let a3569 = 1048576u32;
                                fixed_u32::bit_and(a3568, a3569)
                            };
                            let a3571 = 1048576u32;
                            if equal(a3570, a3571) {
                                1048576u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3572, a3573)?
                    };
                    nat_add(a3574, a3575)?
                };
                let a3599 = {
                    let a3596 = {
                        let a3584 = {
                            let a3578 = {
                                let a3576 = v2;
                                let a3577 = 524288u32;
                                fixed_u32::bit_and(a3576, a3577)
                            };
                            let a3579 = 524288u32;
                            if equal(a3578, a3579) {
                                524288u64
                            } else {
                                0u64
                            }
                        };
                        let a3585 = {
                            let a3582 = {
                                let a3580 = v2;
                                let a3581 = 262144u32;
                                fixed_u32::bit_and(a3580, a3581)
                            };
                            let a3583 = 262144u32;
                            if equal(a3582, a3583) {
                                262144u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3584, a3585)?
                    };
                    let a3597 = {
                        let a3594 = {
                            let a3588 = {
                                let a3586 = v2;
                                let a3587 = 131072u32;
                                fixed_u32::bit_and(a3586, a3587)
                            };
                            let a3589 = 131072u32;
                            if equal(a3588, a3589) {
                                131072u64
                            } else {
                                0u64
                            }
                        };
                        let a3595 = {
                            let a3592 = {
                                let a3590 = v2;
                                let a3591 = 65536u32;
                                fixed_u32::bit_and(a3590, a3591)
                            };
                            let a3593 = 65536u32;
                            if equal(a3592, a3593) {
                                65536u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3594, a3595)?
                    };
                    nat_add(a3596, a3597)?
                };
                nat_add(a3598, a3599)?
            };
            nat_add(a3600, a3601)?
        };
        let a3697 = {
            let a3694 = {
                let a3646 = {
                    let a3622 = {
                        let a3610 = {
                            let a3604 = {
                                let a3602 = v2;
                                let a3603 = 32768u32;
                                fixed_u32::bit_and(a3602, a3603)
                            };
                            let a3605 = 32768u32;
                            if equal(a3604, a3605) {
                                32768u64
                            } else {
                                0u64
                            }
                        };
                        let a3611 = {
                            let a3608 = {
                                let a3606 = v2;
                                let a3607 = 16384u32;
                                fixed_u32::bit_and(a3606, a3607)
                            };
                            let a3609 = 16384u32;
                            if equal(a3608, a3609) {
                                16384u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3610, a3611)?
                    };
                    let a3623 = {
                        let a3620 = {
                            let a3614 = {
                                let a3612 = v2;
                                let a3613 = 8192u32;
                                fixed_u32::bit_and(a3612, a3613)
                            };
                            let a3615 = 8192u32;
                            if equal(a3614, a3615) {
                                8192u64
                            } else {
                                0u64
                            }
                        };
                        let a3621 = {
                            let a3618 = {
                                let a3616 = v2;
                                let a3617 = 4096u32;
                                fixed_u32::bit_and(a3616, a3617)
                            };
                            let a3619 = 4096u32;
                            if equal(a3618, a3619) {
                                4096u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3620, a3621)?
                    };
                    nat_add(a3622, a3623)?
                };
                let a3647 = {
                    let a3644 = {
                        let a3632 = {
                            let a3626 = {
                                let a3624 = v2;
                                let a3625 = 2048u32;
                                fixed_u32::bit_and(a3624, a3625)
                            };
                            let a3627 = 2048u32;
                            if equal(a3626, a3627) {
                                2048u64
                            } else {
                                0u64
                            }
                        };
                        let a3633 = {
                            let a3630 = {
                                let a3628 = v2;
                                let a3629 = 1024u32;
                                fixed_u32::bit_and(a3628, a3629)
                            };
                            let a3631 = 1024u32;
                            if equal(a3630, a3631) {
                                1024u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3632, a3633)?
                    };
                    let a3645 = {
                        let a3642 = {
                            let a3636 = {
                                let a3634 = v2;
                                let a3635 = 512u32;
                                fixed_u32::bit_and(a3634, a3635)
                            };
                            let a3637 = 512u32;
                            if equal(a3636, a3637) {
                                512u64
                            } else {
                                0u64
                            }
                        };
                        let a3643 = {
                            let a3640 = {
                                let a3638 = v2;
                                let a3639 = 256u32;
                                fixed_u32::bit_and(a3638, a3639)
                            };
                            let a3641 = 256u32;
                            if equal(a3640, a3641) {
                                256u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3642, a3643)?
                    };
                    nat_add(a3644, a3645)?
                };
                nat_add(a3646, a3647)?
            };
            let a3695 = {
                let a3692 = {
                    let a3668 = {
                        let a3656 = {
                            let a3650 = {
                                let a3648 = v2;
                                let a3649 = 128u32;
                                fixed_u32::bit_and(a3648, a3649)
                            };
                            let a3651 = 128u32;
                            if equal(a3650, a3651) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a3657 = {
                            let a3654 = {
                                let a3652 = v2;
                                let a3653 = 64u32;
                                fixed_u32::bit_and(a3652, a3653)
                            };
                            let a3655 = 64u32;
                            if equal(a3654, a3655) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3656, a3657)?
                    };
                    let a3669 = {
                        let a3666 = {
                            let a3660 = {
                                let a3658 = v2;
                                let a3659 = 32u32;
                                fixed_u32::bit_and(a3658, a3659)
                            };
                            let a3661 = 32u32;
                            if equal(a3660, a3661) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a3667 = {
                            let a3664 = {
                                let a3662 = v2;
                                let a3663 = 16u32;
                                fixed_u32::bit_and(a3662, a3663)
                            };
                            let a3665 = 16u32;
                            if equal(a3664, a3665) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3666, a3667)?
                    };
                    nat_add(a3668, a3669)?
                };
                let a3693 = {
                    let a3690 = {
                        let a3678 = {
                            let a3672 = {
                                let a3670 = v2;
                                let a3671 = 8u32;
                                fixed_u32::bit_and(a3670, a3671)
                            };
                            let a3673 = 8u32;
                            if equal(a3672, a3673) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a3679 = {
                            let a3676 = {
                                let a3674 = v2;
                                let a3675 = 4u32;
                                fixed_u32::bit_and(a3674, a3675)
                            };
                            let a3677 = 4u32;
                            if equal(a3676, a3677) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3678, a3679)?
                    };
                    let a3691 = {
                        let a3688 = {
                            let a3682 = {
                                let a3680 = v2;
                                let a3681 = 2u32;
                                fixed_u32::bit_and(a3680, a3681)
                            };
                            let a3683 = 2u32;
                            if equal(a3682, a3683) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a3689 = {
                            let a3686 = {
                                let a3684 = v2;
                                let a3685 = 1u32;
                                fixed_u32::bit_and(a3684, a3685)
                            };
                            let a3687 = 1u32;
                            if equal(a3686, a3687) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3688, a3689)?
                    };
                    nat_add(a3690, a3691)?
                };
                nat_add(a3692, a3693)?
            };
            nat_add(a3694, a3695)?
        };
        nat_add(a3696, a3697)?
    }))
}

pub fn f842() -> u64 {
    64u64
}

pub fn f843(v0: Adt159) -> Adt160 {
    Adt160::C0(match v0.clone() {
        Adt159::C0(h3698, _, _, _, _, _, _, _) => {
            h3698
        }
    }, match v0.clone() {
        Adt159::C0(_, h3699, _, _, _, _, _, _) => {
            h3699
        }
    }, match v0.clone() {
        Adt159::C0(_, _, h3700, _, _, _, _, _) => {
            h3700
        }
    }, match v0.clone() {
        Adt159::C0(_, _, _, h3701, _, _, _, _) => {
            h3701
        }
    }, match v0.clone() {
        Adt159::C0(_, _, _, _, h3702, _, _, _) => {
            h3702
        }
    }, match v0.clone() {
        Adt159::C0(_, _, _, _, _, h3703, _, _) => {
            h3703
        }
    }, match v0.clone() {
        Adt159::C0(_, _, _, _, _, _, h3704, _) => {
            h3704
        }
    }, match v0 {
        Adt159::C0(_, _, _, _, _, _, _, h3705) => {
            h3705
        }
    }, f851(), f851(), f851(), f851(), f851(), f851(), f851(), f851())
}

pub fn f844(v0: Bytes) -> R<Adt159> {
    let v1: u64 = {
        let a3706 = v0.clone();
        length_bytes(a3706)
    };
    let v2: u64 = {
        let a3712 = {
            let a3709 = {
                let a3707 = v1;
                let a3708 = 72u64;
                nat_add(a3707, a3708)?
            };
            let a3710 = 64u64;
            let a3711 = 0u64;
            nat_quot(a3709, a3710, a3711)
        };
        let a3713 = 64u64;
        nat_mul(a3712, a3713)?
    };
    let (_, h3714) = f846(Fn7::F854(v0, v1, v2), {
        let a3715 = v2;
        let a3716 = 64u64;
        let a3717 = 0u64;
        nat_quot(a3715, a3716, a3717)
    }, (0u64, f836()))?;
    Ok(h3714)
}

pub fn f845(v0: Bytes) -> R<Adt160> {
    Ok(Adt160::C0(f855(v0.clone(), 0u64)?, f855(v0.clone(), 4u64)?, f855(v0.clone(), 8u64)?, f855(v0.clone(), 12u64)?, f855(v0.clone(), 16u64)?, f855(v0.clone(), 20u64)?, f855(v0.clone(), 24u64)?, f855(v0.clone(), 28u64)?, f855(v0.clone(), 32u64)?, f855(v0.clone(), 36u64)?, f855(v0.clone(), 40u64)?, f855(v0.clone(), 44u64)?, f855(v0.clone(), 48u64)?, f855(v0.clone(), 52u64)?, f855(v0.clone(), 56u64)?, f855(v0, 60u64)?))
}

pub fn f846(v0: Fn7, p3721: u64, p3722: (u64, Adt159)) -> R<(u64, Adt159)> {
    let m3720 = (p3721, p3722);
    {
        let mut m3720 = m3720;
        loop {
            let step = {
                let (v1, v2) = m3720;
                let m3718 = v1;
                if m3718 == 0 {
                    Err::<(u64, (u64, Adt159)), R<(u64, Adt159)>>(Ok(v2))
                } else {
                    let v3 = m3718 - 1;
                    Ok::<(u64, (u64, Adt159)), R<(u64, Adt159)>>((v3, {
                        let c3719: Fn7 = v0.clone();
                        c3719.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m3720 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f847(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: (u64, Adt159)) -> R<(u64, Adt159)> {
    Ok(({
        let a3724 = {
            let (h3723, _) = v4.clone();
            h3723
        };
        let a3725 = 64u64;
        nat_add(a3724, a3725)?
    }, f835({
        let (_, h3726) = v4;
        h3726
    }, f856(v0, v2, v3, {
        let a3729 = {
            let a3727 = v1;
            let a3728 = v2;
            nat_add(a3727, a3728)?
        };
        let a3730 = 8u64;
        nat_mul(a3729, a3730)?
    }, {
        let (h3731, _) = v4;
        h3731
    })?)?))
}

pub fn f848(v0: Fn8, p3735: u64, p3736: Adt161) -> R<Adt161> {
    let m3734 = (p3735, p3736);
    {
        let mut m3734 = m3734;
        loop {
            let step = {
                let (v1, v2) = m3734;
                let m3732 = v1;
                if m3732 == 0 {
                    Err::<(u64, Adt161), R<Adt161>>(Ok(v2))
                } else {
                    let v3 = m3732 - 1;
                    Ok::<(u64, Adt161), R<Adt161>>((v3, {
                        let c3733: Fn8 = v0.clone();
                        c3733.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m3734 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f849(v0: Adt161) -> R<Adt161> {
    f857(v0)
}

pub fn f850(v0: u32, v1: u32) -> u32 {
    let m3739 = {
        let a3737 = f858(v0);
        let a3738 = f858(v1);
        fixed_u64::checked_add(a3737, a3738)
    };
    match m3739 {
        None => {
            0u32
        }
        Some(v2) => {
            f859(v2)
        }
    }
}

pub fn f851() -> u32 {
    0u32
}

pub fn f852(v0: Bytes, v1: u64) -> u8 {
    let m3742 = {
        let a3740 = v0;
        let a3741 = v1;
        index_bytes(a3740, a3741)
    };
    match m3742 {
        None => {
            0u8
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f853(v0: u32, v1: u32) -> u32 {
    let m3745 = {
        let a3743 = v0;
        let a3744 = v1;
        fixed_u32::shift_right(a3743, a3744)
    };
    match m3745 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f854(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt159)) -> R<(u64, Adt159)> {
    Ok(({
        let a3747 = {
            let (h3746, _) = v3.clone();
            h3746
        };
        let a3748 = 64u64;
        nat_add(a3747, a3748)?
    }, f835({
        let (_, h3749) = v3;
        h3749
    }, f860(v0, v1, v2, {
        let (h3750, _) = v3;
        h3750
    })?)?))
}

pub fn f855(v0: Bytes, v1: u64) -> R<u32> {
    Ok(f861(f862(v0.clone(), v1), f862(v0.clone(), {
        let a3751 = v1;
        let a3752 = 1u64;
        nat_add(a3751, a3752)?
    }), f862(v0.clone(), {
        let a3753 = v1;
        let a3754 = 2u64;
        nat_add(a3753, a3754)?
    }), f862(v0, {
        let a3755 = v1;
        let a3756 = 3u64;
        nat_add(a3755, a3756)?
    })))
}

pub fn f856(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Adt160> {
    Ok(Adt160::C0(f863(v0.clone(), v1, v2, v3, v4)?, f863(v0.clone(), v1, v2, v3, {
        let a3757 = v4;
        let a3758 = 4u64;
        nat_add(a3757, a3758)?
    })?, f863(v0.clone(), v1, v2, v3, {
        let a3759 = v4;
        let a3760 = 8u64;
        nat_add(a3759, a3760)?
    })?, f863(v0.clone(), v1, v2, v3, {
        let a3761 = v4;
        let a3762 = 12u64;
        nat_add(a3761, a3762)?
    })?, f863(v0.clone(), v1, v2, v3, {
        let a3763 = v4;
        let a3764 = 16u64;
        nat_add(a3763, a3764)?
    })?, f863(v0.clone(), v1, v2, v3, {
        let a3765 = v4;
        let a3766 = 20u64;
        nat_add(a3765, a3766)?
    })?, f863(v0.clone(), v1, v2, v3, {
        let a3767 = v4;
        let a3768 = 24u64;
        nat_add(a3767, a3768)?
    })?, f863(v0.clone(), v1, v2, v3, {
        let a3769 = v4;
        let a3770 = 28u64;
        nat_add(a3769, a3770)?
    })?, f863(v0.clone(), v1, v2, v3, {
        let a3771 = v4;
        let a3772 = 32u64;
        nat_add(a3771, a3772)?
    })?, f863(v0.clone(), v1, v2, v3, {
        let a3773 = v4;
        let a3774 = 36u64;
        nat_add(a3773, a3774)?
    })?, f863(v0.clone(), v1, v2, v3, {
        let a3775 = v4;
        let a3776 = 40u64;
        nat_add(a3775, a3776)?
    })?, f863(v0.clone(), v1, v2, v3, {
        let a3777 = v4;
        let a3778 = 44u64;
        nat_add(a3777, a3778)?
    })?, f863(v0.clone(), v1, v2, v3, {
        let a3779 = v4;
        let a3780 = 48u64;
        nat_add(a3779, a3780)?
    })?, f863(v0.clone(), v1, v2, v3, {
        let a3781 = v4;
        let a3782 = 52u64;
        nat_add(a3781, a3782)?
    })?, f863(v0.clone(), v1, v2, v3, {
        let a3783 = v4;
        let a3784 = 56u64;
        nat_add(a3783, a3784)?
    })?, f863(v0, v1, v2, v3, {
        let a3785 = v4;
        let a3786 = 60u64;
        nat_add(a3785, a3786)?
    })?))
}

pub fn f857(v0: Adt161) -> R<Adt161> {
    let v1: u32 = f864(match v0.clone() {
        Adt161::C0(_, _, _, _, _, _, _, _, _, h3787) => {
            h3787
        }
    });
    let v2: u32 = f850(match v0.clone() {
        Adt161::C0(_, _, _, _, _, _, _, h3788, _, _) => {
            h3788
        }
    }, f850(f865(match v0.clone() {
        Adt161::C0(_, _, _, _, h3789, _, _, _, _, _) => {
            h3789
        }
    }), f850(f866(match v0.clone() {
        Adt161::C0(_, _, _, _, h3790, _, _, _, _, _) => {
            h3790
        }
    }, match v0.clone() {
        Adt161::C0(_, _, _, _, _, h3791, _, _, _, _) => {
            h3791
        }
    }, match v0.clone() {
        Adt161::C0(_, _, _, _, _, _, h3792, _, _, _) => {
            h3792
        }
    }), f850(v1, {
        let m3795 = match v0.clone() {
            Adt161::C0(_, _, _, _, _, _, _, _, h3794, _) => {
                h3794
            }
        };
        match m3795 {
            Adt160::C0(h3793, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h3793
            }
        }
    }))));
    let v3: u32 = f850(f867(match v0.clone() {
        Adt161::C0(h3796, _, _, _, _, _, _, _, _, _) => {
            h3796
        }
    }), f868(match v0.clone() {
        Adt161::C0(h3797, _, _, _, _, _, _, _, _, _) => {
            h3797
        }
    }, match v0.clone() {
        Adt161::C0(_, h3798, _, _, _, _, _, _, _, _) => {
            h3798
        }
    }, match v0.clone() {
        Adt161::C0(_, _, h3799, _, _, _, _, _, _, _) => {
            h3799
        }
    }));
    Ok(Adt161::C0(f850(v2, v3), match v0.clone() {
        Adt161::C0(h3800, _, _, _, _, _, _, _, _, _) => {
            h3800
        }
    }, match v0.clone() {
        Adt161::C0(_, h3801, _, _, _, _, _, _, _, _) => {
            h3801
        }
    }, match v0.clone() {
        Adt161::C0(_, _, h3802, _, _, _, _, _, _, _) => {
            h3802
        }
    }, f850(match v0.clone() {
        Adt161::C0(_, _, _, h3803, _, _, _, _, _, _) => {
            h3803
        }
    }, v2), match v0.clone() {
        Adt161::C0(_, _, _, _, h3804, _, _, _, _, _) => {
            h3804
        }
    }, match v0.clone() {
        Adt161::C0(_, _, _, _, _, h3805, _, _, _, _) => {
            h3805
        }
    }, match v0.clone() {
        Adt161::C0(_, _, _, _, _, _, h3806, _, _, _) => {
            h3806
        }
    }, f869(match v0.clone() {
        Adt161::C0(_, _, _, _, _, _, _, _, h3807, _) => {
            h3807
        }
    }), {
        let a3809 = match v0 {
            Adt161::C0(_, _, _, _, _, _, _, _, _, h3808) => {
                h3808
            }
        };
        let a3810 = 1u64;
        nat_add(a3809, a3810)?
    }))
}

pub fn f858(v0: u32) -> u64 {
    let m3812 = {
        let a3811 = v0;
        fixed_u64::convert(i128::from(a3811))
    };
    match m3812 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f859(v0: u64) -> u32 {
    let m3816 = {
        let a3815 = {
            let a3813 = v0;
            let a3814 = 4294967295u64;
            fixed_u64::bit_and(a3813, a3814)
        };
        fixed_u32::convert(i128::from(a3815))
    };
    match m3816 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f860(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt160> {
    Ok(Adt160::C0(f870(v0.clone(), v1, v2, v3)?, f870(v0.clone(), v1, v2, {
        let a3817 = v3;
        let a3818 = 4u64;
        nat_add(a3817, a3818)?
    })?, f870(v0.clone(), v1, v2, {
        let a3819 = v3;
        let a3820 = 8u64;
        nat_add(a3819, a3820)?
    })?, f870(v0.clone(), v1, v2, {
        let a3821 = v3;
        let a3822 = 12u64;
        nat_add(a3821, a3822)?
    })?, f870(v0.clone(), v1, v2, {
        let a3823 = v3;
        let a3824 = 16u64;
        nat_add(a3823, a3824)?
    })?, f870(v0.clone(), v1, v2, {
        let a3825 = v3;
        let a3826 = 20u64;
        nat_add(a3825, a3826)?
    })?, f870(v0.clone(), v1, v2, {
        let a3827 = v3;
        let a3828 = 24u64;
        nat_add(a3827, a3828)?
    })?, f870(v0.clone(), v1, v2, {
        let a3829 = v3;
        let a3830 = 28u64;
        nat_add(a3829, a3830)?
    })?, f870(v0.clone(), v1, v2, {
        let a3831 = v3;
        let a3832 = 32u64;
        nat_add(a3831, a3832)?
    })?, f870(v0.clone(), v1, v2, {
        let a3833 = v3;
        let a3834 = 36u64;
        nat_add(a3833, a3834)?
    })?, f870(v0.clone(), v1, v2, {
        let a3835 = v3;
        let a3836 = 40u64;
        nat_add(a3835, a3836)?
    })?, f870(v0.clone(), v1, v2, {
        let a3837 = v3;
        let a3838 = 44u64;
        nat_add(a3837, a3838)?
    })?, f870(v0.clone(), v1, v2, {
        let a3839 = v3;
        let a3840 = 48u64;
        nat_add(a3839, a3840)?
    })?, f870(v0.clone(), v1, v2, {
        let a3841 = v3;
        let a3842 = 52u64;
        nat_add(a3841, a3842)?
    })?, f870(v0.clone(), v1, v2, {
        let a3843 = v3;
        let a3844 = 56u64;
        nat_add(a3843, a3844)?
    })?, f870(v0, v1, v2, {
        let a3845 = v3;
        let a3846 = 60u64;
        nat_add(a3845, a3846)?
    })?))
}

pub fn f861(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a3851 = f871(v0, 24u32);
    let a3852 = {
        let a3849 = f871(v1, 16u32);
        let a3850 = {
            let a3847 = f871(v2, 8u32);
            let a3848 = v3;
            fixed_u32::bit_or(a3847, a3848)
        };
        fixed_u32::bit_or(a3849, a3850)
    };
    fixed_u32::bit_or(a3851, a3852)
}

pub fn f862(v0: Bytes, v1: u64) -> u32 {
    let m3855 = {
        let a3853 = v0;
        let a3854 = v1;
        index_bytes(a3853, a3854)
    };
    match m3855 {
        None => {
            0u32
        }
        Some(v2) => {
            let m3857 = {
                let a3856 = v2;
                fixed_u32::convert(i128::from(a3856))
            };
            match m3857 {
                None => {
                    0u32
                }
                Some(v3) => {
                    v3
                }
            }
        }
    }
}

pub fn f863(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u32> {
    let a3860 = {
        let a3858 = v4;
        let a3859 = 8u64;
        nat_add(a3858, a3859)?
    };
    let a3861 = v2;
    if nat_eq(a3860, a3861) {
        Ok(f872(v3, true))
    } else {
        let a3864 = {
            let a3862 = v4;
            let a3863 = 4u64;
            nat_add(a3862, a3863)?
        };
        let a3865 = v2;
        if nat_eq(a3864, a3865) {
            Ok(f872(v3, false))
        } else {
            Ok(f861(f873(v0.clone(), v1, v4), f873(v0.clone(), v1, {
                let a3866 = v4;
                let a3867 = 1u64;
                nat_add(a3866, a3867)?
            }), f873(v0.clone(), v1, {
                let a3868 = v4;
                let a3869 = 2u64;
                nat_add(a3868, a3869)?
            }), f873(v0, v1, {
                let a3870 = v4;
                let a3871 = 3u64;
                nat_add(a3870, a3871)?
            })))
        }
    }
}

pub fn f864(v0: u64) -> u32 {
    let a3872 = v0;
    let a3873 = 32u64;
    if nat_lt(a3872, a3873) {
        let a3874 = v0;
        let a3875 = 16u64;
        if nat_lt(a3874, a3875) {
            let a3876 = v0;
            let a3877 = 8u64;
            if nat_lt(a3876, a3877) {
                let a3878 = v0;
                let a3879 = 4u64;
                if nat_lt(a3878, a3879) {
                    let a3880 = v0;
                    let a3881 = 2u64;
                    if nat_lt(a3880, a3881) {
                        let a3882 = v0;
                        let a3883 = 1u64;
                        if nat_lt(a3882, a3883) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a3884 = v0;
                        let a3885 = 3u64;
                        if nat_lt(a3884, a3885) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a3886 = v0;
                    let a3887 = 6u64;
                    if nat_lt(a3886, a3887) {
                        let a3888 = v0;
                        let a3889 = 5u64;
                        if nat_lt(a3888, a3889) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a3890 = v0;
                        let a3891 = 7u64;
                        if nat_lt(a3890, a3891) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a3892 = v0;
                let a3893 = 12u64;
                if nat_lt(a3892, a3893) {
                    let a3894 = v0;
                    let a3895 = 10u64;
                    if nat_lt(a3894, a3895) {
                        let a3896 = v0;
                        let a3897 = 9u64;
                        if nat_lt(a3896, a3897) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a3898 = v0;
                        let a3899 = 11u64;
                        if nat_lt(a3898, a3899) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a3900 = v0;
                    let a3901 = 14u64;
                    if nat_lt(a3900, a3901) {
                        let a3902 = v0;
                        let a3903 = 13u64;
                        if nat_lt(a3902, a3903) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a3904 = v0;
                        let a3905 = 15u64;
                        if nat_lt(a3904, a3905) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a3906 = v0;
            let a3907 = 24u64;
            if nat_lt(a3906, a3907) {
                let a3908 = v0;
                let a3909 = 20u64;
                if nat_lt(a3908, a3909) {
                    let a3910 = v0;
                    let a3911 = 18u64;
                    if nat_lt(a3910, a3911) {
                        let a3912 = v0;
                        let a3913 = 17u64;
                        if nat_lt(a3912, a3913) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a3914 = v0;
                        let a3915 = 19u64;
                        if nat_lt(a3914, a3915) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a3916 = v0;
                    let a3917 = 22u64;
                    if nat_lt(a3916, a3917) {
                        let a3918 = v0;
                        let a3919 = 21u64;
                        if nat_lt(a3918, a3919) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a3920 = v0;
                        let a3921 = 23u64;
                        if nat_lt(a3920, a3921) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a3922 = v0;
                let a3923 = 28u64;
                if nat_lt(a3922, a3923) {
                    let a3924 = v0;
                    let a3925 = 26u64;
                    if nat_lt(a3924, a3925) {
                        let a3926 = v0;
                        let a3927 = 25u64;
                        if nat_lt(a3926, a3927) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a3928 = v0;
                        let a3929 = 27u64;
                        if nat_lt(a3928, a3929) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a3930 = v0;
                    let a3931 = 30u64;
                    if nat_lt(a3930, a3931) {
                        let a3932 = v0;
                        let a3933 = 29u64;
                        if nat_lt(a3932, a3933) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a3934 = v0;
                        let a3935 = 31u64;
                        if nat_lt(a3934, a3935) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a3936 = v0;
        let a3937 = 48u64;
        if nat_lt(a3936, a3937) {
            let a3938 = v0;
            let a3939 = 40u64;
            if nat_lt(a3938, a3939) {
                let a3940 = v0;
                let a3941 = 36u64;
                if nat_lt(a3940, a3941) {
                    let a3942 = v0;
                    let a3943 = 34u64;
                    if nat_lt(a3942, a3943) {
                        let a3944 = v0;
                        let a3945 = 33u64;
                        if nat_lt(a3944, a3945) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a3946 = v0;
                        let a3947 = 35u64;
                        if nat_lt(a3946, a3947) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a3948 = v0;
                    let a3949 = 38u64;
                    if nat_lt(a3948, a3949) {
                        let a3950 = v0;
                        let a3951 = 37u64;
                        if nat_lt(a3950, a3951) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a3952 = v0;
                        let a3953 = 39u64;
                        if nat_lt(a3952, a3953) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a3954 = v0;
                let a3955 = 44u64;
                if nat_lt(a3954, a3955) {
                    let a3956 = v0;
                    let a3957 = 42u64;
                    if nat_lt(a3956, a3957) {
                        let a3958 = v0;
                        let a3959 = 41u64;
                        if nat_lt(a3958, a3959) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a3960 = v0;
                        let a3961 = 43u64;
                        if nat_lt(a3960, a3961) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a3962 = v0;
                    let a3963 = 46u64;
                    if nat_lt(a3962, a3963) {
                        let a3964 = v0;
                        let a3965 = 45u64;
                        if nat_lt(a3964, a3965) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a3966 = v0;
                        let a3967 = 47u64;
                        if nat_lt(a3966, a3967) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a3968 = v0;
            let a3969 = 56u64;
            if nat_lt(a3968, a3969) {
                let a3970 = v0;
                let a3971 = 52u64;
                if nat_lt(a3970, a3971) {
                    let a3972 = v0;
                    let a3973 = 50u64;
                    if nat_lt(a3972, a3973) {
                        let a3974 = v0;
                        let a3975 = 49u64;
                        if nat_lt(a3974, a3975) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a3976 = v0;
                        let a3977 = 51u64;
                        if nat_lt(a3976, a3977) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a3978 = v0;
                    let a3979 = 54u64;
                    if nat_lt(a3978, a3979) {
                        let a3980 = v0;
                        let a3981 = 53u64;
                        if nat_lt(a3980, a3981) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a3982 = v0;
                        let a3983 = 55u64;
                        if nat_lt(a3982, a3983) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a3984 = v0;
                let a3985 = 60u64;
                if nat_lt(a3984, a3985) {
                    let a3986 = v0;
                    let a3987 = 58u64;
                    if nat_lt(a3986, a3987) {
                        let a3988 = v0;
                        let a3989 = 57u64;
                        if nat_lt(a3988, a3989) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a3990 = v0;
                        let a3991 = 59u64;
                        if nat_lt(a3990, a3991) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a3992 = v0;
                    let a3993 = 62u64;
                    if nat_lt(a3992, a3993) {
                        let a3994 = v0;
                        let a3995 = 61u64;
                        if nat_lt(a3994, a3995) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a3996 = v0;
                        let a3997 = 63u64;
                        if nat_lt(a3996, a3997) {
                            3204031479u32
                        } else {
                            3329325298u32
                        }
                    }
                }
            }
        }
    }
}

pub fn f865(v0: u32) -> u32 {
    let a4000 = {
        let a3998 = f874(v0, 6u32, 26u32);
        let a3999 = f874(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a3998, a3999)
    };
    let a4001 = f874(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a4000, a4001)
}

pub fn f866(v0: u32, v1: u32, v2: u32) -> u32 {
    let a4007 = {
        let a4002 = v0;
        let a4003 = v1;
        fixed_u32::bit_and(a4002, a4003)
    };
    let a4008 = {
        let a4005 = {
            let a4004 = v0;
            fixed_u32::bit_not(a4004)
        };
        let a4006 = v2;
        fixed_u32::bit_and(a4005, a4006)
    };
    fixed_u32::bit_xor(a4007, a4008)
}

pub fn f867(v0: u32) -> u32 {
    let a4011 = {
        let a4009 = f874(v0, 2u32, 30u32);
        let a4010 = f874(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a4009, a4010)
    };
    let a4012 = f874(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a4011, a4012)
}

pub fn f868(v0: u32, v1: u32, v2: u32) -> u32 {
    let a4021 = {
        let a4017 = {
            let a4013 = v0;
            let a4014 = v1;
            fixed_u32::bit_and(a4013, a4014)
        };
        let a4018 = {
            let a4015 = v0;
            let a4016 = v2;
            fixed_u32::bit_and(a4015, a4016)
        };
        fixed_u32::bit_xor(a4017, a4018)
    };
    let a4022 = {
        let a4019 = v1;
        let a4020 = v2;
        fixed_u32::bit_and(a4019, a4020)
    };
    fixed_u32::bit_xor(a4021, a4022)
}

pub fn f869(v0: Adt160) -> Adt160 {
    Adt160::C0(match v0.clone() {
        Adt160::C0(_, h4023, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h4023
        }
    }, match v0.clone() {
        Adt160::C0(_, _, h4024, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h4024
        }
    }, match v0.clone() {
        Adt160::C0(_, _, _, h4025, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h4025
        }
    }, match v0.clone() {
        Adt160::C0(_, _, _, _, h4026, _, _, _, _, _, _, _, _, _, _, _) => {
            h4026
        }
    }, match v0.clone() {
        Adt160::C0(_, _, _, _, _, h4027, _, _, _, _, _, _, _, _, _, _) => {
            h4027
        }
    }, match v0.clone() {
        Adt160::C0(_, _, _, _, _, _, h4028, _, _, _, _, _, _, _, _, _) => {
            h4028
        }
    }, match v0.clone() {
        Adt160::C0(_, _, _, _, _, _, _, h4029, _, _, _, _, _, _, _, _) => {
            h4029
        }
    }, match v0.clone() {
        Adt160::C0(_, _, _, _, _, _, _, _, h4030, _, _, _, _, _, _, _) => {
            h4030
        }
    }, match v0.clone() {
        Adt160::C0(_, _, _, _, _, _, _, _, _, h4031, _, _, _, _, _, _) => {
            h4031
        }
    }, match v0.clone() {
        Adt160::C0(_, _, _, _, _, _, _, _, _, _, h4032, _, _, _, _, _) => {
            h4032
        }
    }, match v0.clone() {
        Adt160::C0(_, _, _, _, _, _, _, _, _, _, _, h4033, _, _, _, _) => {
            h4033
        }
    }, match v0.clone() {
        Adt160::C0(_, _, _, _, _, _, _, _, _, _, _, _, h4034, _, _, _) => {
            h4034
        }
    }, match v0.clone() {
        Adt160::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h4035, _, _) => {
            h4035
        }
    }, match v0.clone() {
        Adt160::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h4036, _) => {
            h4036
        }
    }, match v0.clone() {
        Adt160::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h4037) => {
            h4037
        }
    }, f850(f875(match v0.clone() {
        Adt160::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h4038, _) => {
            h4038
        }
    }), f850(match v0.clone() {
        Adt160::C0(_, _, _, _, _, _, _, _, _, h4039, _, _, _, _, _, _) => {
            h4039
        }
    }, f850(f876(match v0.clone() {
        Adt160::C0(_, h4040, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h4040
        }
    }), match v0 {
        Adt160::C0(h4041, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h4041
        }
    }))))
}

pub fn f870(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a4044 = {
        let a4042 = v3;
        let a4043 = 8u64;
        nat_add(a4042, a4043)?
    };
    let a4045 = v2;
    if nat_eq(a4044, a4045) {
        Ok(f877({
            let a4048 = {
                let a4046 = v1;
                let a4047 = 8u64;
                nat_mul(a4046, a4047)?
            };
            let a4049 = 4294967296u64;
            let a4050 = 0u64;
            nat_quot(a4048, a4049, a4050)
        }))
    } else {
        let a4053 = {
            let a4051 = v3;
            let a4052 = 4u64;
            nat_add(a4051, a4052)?
        };
        let a4054 = v2;
        if nat_eq(a4053, a4054) {
            Ok(f877({
                let a4057 = {
                    let a4055 = v1;
                    let a4056 = 8u64;
                    nat_mul(a4055, a4056)?
                };
                let a4058 = 4294967296u64;
                let a4059 = 0u64;
                nat_rem(a4057, a4058, a4059)
            }))
        } else {
            Ok(f861(f873(v0.clone(), v1, v3), f873(v0.clone(), v1, {
                let a4060 = v3;
                let a4061 = 1u64;
                nat_add(a4060, a4061)?
            }), f873(v0.clone(), v1, {
                let a4062 = v3;
                let a4063 = 2u64;
                nat_add(a4062, a4063)?
            }), f873(v0, v1, {
                let a4064 = v3;
                let a4065 = 3u64;
                nat_add(a4064, a4065)?
            })))
        }
    }
}

pub fn f871(v0: u32, v1: u32) -> u32 {
    let m4068 = {
        let a4066 = v0;
        let a4067 = v1;
        fixed_u32::shift_left(a4066, a4067)
    };
    match m4068 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f872(v0: u64, v1: bool) -> u32 {
    if v1 {
        f877({
            let a4069 = v0;
            let a4070 = 4294967296u64;
            let a4071 = 0u64;
            nat_quot(a4069, a4070, a4071)
        })
    } else {
        f877({
            let a4072 = v0;
            let a4073 = 4294967296u64;
            let a4074 = 0u64;
            nat_rem(a4072, a4073, a4074)
        })
    }
}

pub fn f873(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a4075 = v2;
    let a4076 = v1;
    if nat_lt(a4075, a4076) {
        let m4079 = {
            let a4077 = v0;
            let a4078 = v2;
            index_bytes(a4077, a4078)
        };
        match m4079 {
            None => {
                0u32
            }
            Some(v3) => {
                let m4081 = {
                    let a4080 = v3;
                    fixed_u32::convert(i128::from(a4080))
                };
                match m4081 {
                    None => {
                        0u32
                    }
                    Some(v4) => {
                        v4
                    }
                }
            }
        }
    } else {
        let a4082 = v2;
        let a4083 = v1;
        if nat_eq(a4082, a4083) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f874(v0: u32, v1: u32, v2: u32) -> u32 {
    let a4084 = f853(v0, v1);
    let a4085 = f871(v0, v2);
    fixed_u32::bit_or(a4084, a4085)
}

pub fn f875(v0: u32) -> u32 {
    let a4088 = {
        let a4086 = f874(v0, 17u32, 15u32);
        let a4087 = f874(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a4086, a4087)
    };
    let a4089 = f853(v0, 10u32);
    fixed_u32::bit_xor(a4088, a4089)
}

pub fn f876(v0: u32) -> u32 {
    let a4092 = {
        let a4090 = f874(v0, 7u32, 25u32);
        let a4091 = f874(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a4090, a4091)
    };
    let a4093 = f853(v0, 3u32);
    fixed_u32::bit_xor(a4092, a4093)
}

pub fn f877(v0: u64) -> u32 {
    let (_, h4094) = {
        let (_, h4095) = f878(Fn9::F879, 32u64, (v0, (1u32, 0u32)));
        h4095
    };
    h4094
}

pub fn f878(v0: Fn9, p4099: u64, p4100: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m4098 = (p4099, p4100);
    {
        let mut m4098 = m4098;
        loop {
            let step = {
                let (v1, v2) = m4098;
                let m4096 = v1;
                if m4096 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m4096 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c4097: Fn9 = v0.clone();
                        c4097.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m4098 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f879(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h4101, _) = v0;
        h4101
    };
    let v2: u32 = {
        let (h4102, _) = {
            let (_, h4103) = v0;
            h4103
        };
        h4102
    };
    let v3: u32 = {
        let (_, h4104) = {
            let (_, h4105) = v0;
            h4105
        };
        h4104
    };
    ({
        let a4106 = v1;
        let a4107 = 2u64;
        let a4108 = 0u64;
        nat_quot(a4106, a4107, a4108)
    }, (f871(v2, 1u32), {
        let a4112 = {
            let a4109 = v1;
            let a4110 = 2u64;
            let a4111 = 0u64;
            nat_rem(a4109, a4110, a4111)
        };
        let a4113 = 1u64;
        if nat_eq(a4112, a4113) {
            let a4114 = v3;
            let a4115 = v2;
            fixed_u32::bit_or(a4114, a4115)
        } else {
            v3
        }
    }))
}

pub fn run(p0: Bytes) -> R<List<u64>> {
    f0(p0)
}
