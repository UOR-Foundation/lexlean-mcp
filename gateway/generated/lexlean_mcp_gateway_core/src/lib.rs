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
    C0(Adt104, Str, Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt116 {
    C0(List<u64>),
    C1(u64, Adt6),
    C2(u64, List<Adt6>),
    C3(u64, List<Adt6>),
    C4(u64, List<(Str, Adt6)>),
    C5(u64, List<(Str, Adt6)>),
}

#[derive(Clone)]
pub enum Adt117 {
    C0(Option<Str>, Option<Str>, Option<Str>, Option<Adt118>, List<Adt120>),
}

#[derive(Clone)]
pub enum Adt118 {
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6(Adt119),
    C7(Adt119),
    C8(Str),
    C9(Str),
    C10(Str),
}

#[derive(Clone)]
pub enum Adt119 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt120 {
    C0(Str, List<Str>, Adt13, Adt121),
}

#[derive(Clone)]
pub enum Adt121 {
    C0,
    C1,
    C2(Str),
}

#[derive(Clone)]
pub enum Adt122 {
    C0,
    C1(u64),
    C2(u64),
}

#[derive(Clone)]
pub enum Adt123 {
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt124 {
    C0(Adt125, List<Adt129>, u64),
}

#[derive(Clone)]
pub enum Adt125 {
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6(Adt126, List<u64>, bool),
    C7(u64, u64),
    C8(Adt127),
    C9(Adt6),
}

#[derive(Clone)]
pub enum Adt126 {
    C0,
    C1,
    C2(u64, u64, u64),
    C3(u64),
    C4(u64),
}

#[derive(Clone)]
pub enum Adt127 {
    C0(bool, Adt128, i64, u64, i64, bool, i64),
}

#[derive(Clone)]
pub enum Adt128 {
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
pub enum Adt129 {
    C0(List<Adt6>),
    C1(List<(Str, Adt6)>, Option<Str>),
}

#[derive(Clone)]
pub enum Adt130 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt131 {
    C0,
    C1,
}

#[derive(Clone)]
pub enum Adt132 {
    C0(Adt104, Result<Adt133, Adt136>),
}

#[derive(Clone)]
pub enum Adt133 {
    C0(Adt34, Adt134, List<Adt135>),
}

#[derive(Clone)]
pub enum Adt134 {
    C0(Option<Adt1>, Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt135 {
    C0(Adt21, Adt52),
}

#[derive(Clone)]
pub enum Adt136 {
    C0(Adt137),
    C1(Str, List<Str>),
    C2(Adt118),
    C3(Str),
    C4(Adt86),
}

#[derive(Clone)]
pub enum Adt137 {
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
pub enum Adt138 {
    C0(Option<Str>, Option<Adt118>),
}

#[derive(Clone)]
pub enum Adt139 {
    C0,
    C1(Adt104),
    C2,
}

#[derive(Clone)]
pub enum Adt140 {
    C0(Adt107),
    C1(Adt124),
}

#[derive(Clone)]
pub enum Adt141 {
    C0(Adt93),
    C1(Adt112),
}

#[derive(Clone)]
pub enum Adt142 {
    C0(Adt107),
    C1(List<Adt116>, List<u64>),
}

#[derive(Clone)]
pub enum Adt143 {
    C0(Adt104, Adt144, Adt18, Adt5),
}

#[derive(Clone)]
pub enum Adt144 {
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
pub enum Adt145 {
    C0,
    C1,
    C2(Adt6),
}

#[derive(Clone)]
pub enum Adt146 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt147 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt148 {
    C0,
    C1(Adt144),
}

#[derive(Clone)]
pub enum Adt149 {
    C0(Adt150),
    C1(Adt136),
}

#[derive(Clone)]
pub enum Adt150 {
    C0(Adt104, Str, Adt18, Adt5),
}

#[derive(Clone)]
pub enum Adt151 {
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt152 {
    C0,
    C1(List<(Adt6, Adt6)>),
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
    F41(Fn0, Adt89),
}

impl Fn2 {
    pub fn apply(&self, p0: Adt16) -> R<Result<Adt34, Adt86>> {
        match self {
            Fn2::F41(k0, k1) => f41(k0.clone(), k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F51(Fn1, Adt37),
}

impl Fn3 {
    pub fn apply(&self, p0: Adt23) -> R<Result<Adt34, Adt86>> {
        match self {
            Fn3::F51(k0, k1) => f51(k0.clone(), k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F803(Bytes),
    F828(Bytes, Bytes),
}

impl Fn4 {
    pub fn apply(&self, p0: (u64, u64)) -> R<(u64, u64)> {
        match self {
            Fn4::F803(k0) => f803(k0.clone(), p0),
            Fn4::F828(k0, k1) => f828(k0.clone(), k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F808(Bytes),
}

impl Fn5 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<(u64, Bytes)> {
        match self {
            Fn5::F808(k0) => f808(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F813(Bytes),
}

impl Fn6 {
    pub fn apply(&self, p0: (u64, (Bytes, u64))) -> R<(u64, (Bytes, u64))> {
        match self {
            Fn6::F813(k0) => f813(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F836(Bytes, u64, u64, u64),
    F843(Bytes, u64, u64),
}

impl Fn7 {
    pub fn apply(&self, p0: (u64, Adt159)) -> R<(u64, Adt159)> {
        match self {
            Fn7::F836(k0, k1, k2, k3) => f836(k0.clone(), *k1, *k2, *k3, p0),
            Fn7::F843(k0, k1, k2) => f843(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F838,
}

impl Fn8 {
    pub fn apply(&self, p0: Adt161) -> R<Adt161> {
        match self {
            Fn8::F838 => f838(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F868,
}

impl Fn9 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn9::F868 => f868(p0),
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
    Ok(List::cons(f24(Str::lit("lexlean_draft_write"), Str::lit("Write a file of a draft in the store."), f25(List::cons((Str::lit("path"), f26()?), f27(Str::lit("content"), f28(Str::lit("string")))), List::cons(Str::lit("path"), List::cons(Str::lit("content"), List::<Str>::nil())))), List::cons(f24(Str::lit("lexlean_draft_read"), Str::lit("Read a file of a draft from the store."), f25(f27(Str::lit("path"), f26()?), List::cons(Str::lit("path"), List::<Str>::nil()))), List::cons(f24(Str::lit("lexlean_check"), Str::lit("Check a project with the LexLean engine."), f25(f29()?, List::cons(Str::lit("project"), List::<Str>::nil()))), List::cons(f24(Str::lit("lexlean_snapshot"), Str::lit("Take the semantic snapshot of a project with the LexLean engine."), f25(f29()?, List::cons(Str::lit("project"), List::<Str>::nil()))), List::cons(f24(Str::lit("lexlean_format"), Str::lit("Format a project with the LexLean engine, or check its formatting."), f25({
        let a16 = f29()?;
        let a17 = f27(Str::lit("check_only"), f28(Str::lit("boolean")));
        append_list(a16, a17)
    }, List::cons(Str::lit("project"), List::<Str>::nil()))), List::cons(f24(Str::lit("lexlean_build"), Str::lit("Build a project with the LexLean engine."), f25(f29()?, List::cons(Str::lit("project"), List::<Str>::nil()))), List::cons(f24(Str::lit("lexlean_verify"), Str::lit("Start the verification of a project and return its task."), f25(f29()?, List::cons(Str::lit("project"), List::<Str>::nil()))), List::cons(f24(Str::lit("lexlean_verify_control"), Str::lit("Read the state of a verification task, or cancel it."), f25(List::cons((Str::lit("task"), f26()?), f27(Str::lit("action"), f30(Str::lit("status"), Str::lit("cancel")))), List::cons(Str::lit("task"), List::cons(Str::lit("action"), List::<Str>::nil())))), List::cons(f24(Str::lit("lexlean_lexicon_read"), Str::lit("Read an entry of the lexicon of a profile."), f25(List::cons((Str::lit("profile"), f26()?), f27(Str::lit("entry"), f26()?)), List::cons(Str::lit("profile"), List::cons(Str::lit("entry"), List::<Str>::nil())))), List::cons(f24(Str::lit("lexlean_attestation"), Str::lit("Look an attestation up by its ID or by the digest of its file."), f25(List::cons((Str::lit("by"), f30(Str::lit("id"), Str::lit("digest"))), f27(Str::lit("value"), f26()?)), List::cons(Str::lit("by"), List::cons(Str::lit("value"), List::<Str>::nil())))), List::<Adt37>::nil())))))))))))
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
            Ok(Some(Adt86::C9(f31(v1))))
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
    let m22 = f32(match v2.clone() {
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
            f33(v3, v1, v2)
        }
    }
}

pub fn f15(v0: Adt14, v1: Adt23) -> R<Result<Adt51, Adt86>> {
    f34(match v0 {
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
    f35(v0.clone(), {
        let a46 = v0;
        length_bytes(a46)
    }, List::<u64>::nil())
}

pub fn f19(v0: Bytes) -> R<Result<Adt89, Adt90>> {
    f36(v0.clone(), f37(v0, 0u64)?)
}

pub fn f20() -> Adt88 {
    Adt88::C0(f38(f39()))
}

pub fn f21(v0: Adt0, v1: Fn0, v2: Adt89, v3: Result<Adt91, Adt90>) -> R<Adt88> {
    let m47 = v3;
    match m47 {
        Ok(v5) => {
            f40(v0, Fn2::F41(v1, v2), v5)
        }
        Err(_) => {
            Ok(f20())
        }
    }
}

pub fn f22(v0: Bytes) -> R<Result<Adt91, Adt90>> {
    let m48 = f37(v0.clone(), 0u64)?;
    match m48 {
        None => {
            Ok(Err::<Adt91, Adt90>(Adt90::C0(0u64)))
        }
        Some(v1) => {
            f42(v0.clone(), v1.clone(), f37(v0, match v1 {
                Adt94::C0(_, h49) => {
                    h49
                }
            })?)
        }
    }
}

pub fn f23(v0: List<Adt37>) -> R<Result<List<Adt12>, Adt93>> {
    let m50 = f43(v0.clone());
    match m50 {
        None => {
            f44(v0, List::<Adt12>::nil())
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
    Adt5::C0(List::cons((Str::lit("type"), f45(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(v0)), List::cons((Str::lit("required"), f46(v1, List::<Adt6>::nil())), f27(Str::lit("additionalProperties"), f47(false))))))
}

pub fn f26() -> R<Adt6> {
    Ok(Adt6::C5(List::cons((Str::lit("type"), f45(Str::lit("string"))), f27(Str::lit("minLength"), f48(1u64)?))))
}

pub fn f27(v0: Str, v1: Adt6) -> List<(Str, Adt6)> {
    List::cons((v0, v1), List::<(Str, Adt6)>::nil())
}

pub fn f28(v0: Str) -> Adt6 {
    Adt6::C5(f27(Str::lit("type"), f45(v0)))
}

pub fn f29() -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("project"), f26()?), f27(Str::lit("files"), f49())))
}

pub fn f30(v0: Str, v1: Str) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), f45(Str::lit("string"))), f27(Str::lit("enum"), Adt6::C4(List::cons(f45(v0), List::cons(f45(v1), List::<Adt6>::nil()))))))
}

pub fn f31(v0: Adt93) -> Str {
    let m51 = v0;
    match m51 {
        Adt93::C0(v1) => {
            let a52 = List::cons(Str::lit("tool name is not 1 to 64 characters of letters, digits, underscore, hyphen and dot:"), List::cons(v1, List::<Str>::nil()));
            let a53 = Str::lit(" ");
            join(a52, a53)
        }
        Adt93::C1(v2) => {
            let a54 = List::cons(Str::lit("tool name is used twice:"), List::cons(v2, List::<Str>::nil()));
            let a55 = Str::lit(" ");
            join(a54, a55)
        }
        Adt93::C2(v3) => {
            let a56 = List::cons(Str::lit("the schema dialect of tool"), List::cons(v3, List::cons(Str::lit("is not JSON Schema 2020-12"), List::<Str>::nil())));
            let a57 = Str::lit(" ");
            join(a56, a57)
        }
        Adt93::C3(v4) => {
            let a58 = List::cons(Str::lit("the input schema of tool"), List::cons(v4, List::cons(Str::lit("does not have type object"), List::<Str>::nil())));
            let a59 = Str::lit(" ");
            join(a58, a59)
        }
        Adt93::C4(v5, v6) => {
            let a60 = List::cons(Str::lit("tool"), List::cons(v5, List::cons(Str::lit("uses the unsupported schema keyword"), List::cons(v6, List::<Str>::nil()))));
            let a61 = Str::lit(" ");
            join(a60, a61)
        }
        Adt93::C5(v7, v8) => {
            let a62 = List::cons(Str::lit("tool"), List::cons(v7, List::cons(Str::lit("has a malformed schema keyword"), List::cons(v8, List::<Str>::nil()))));
            let a63 = Str::lit(" ");
            join(a62, a63)
        }
        Adt93::C6(v9, v10) => {
            let a64 = List::cons(Str::lit("tool"), List::cons(v9, List::cons(Str::lit("has an unresolved reference"), List::cons(v10, List::<Str>::nil()))));
            let a65 = Str::lit(" ");
            join(a64, a65)
        }
        Adt93::C7(v11) => {
            let a66 = List::cons(Str::lit("tool"), List::cons(v11, List::cons(Str::lit("has an x-mcp-header that is not text"), List::<Str>::nil())));
            let a67 = Str::lit(" ");
            join(a66, a67)
        }
        Adt93::C8(v12) => {
            let a68 = List::cons(Str::lit("tool"), List::cons(v12, List::cons(Str::lit("has an empty x-mcp-header"), List::<Str>::nil())));
            let a69 = Str::lit(" ");
            join(a68, a69)
        }
        Adt93::C9(v13) => {
            let a70 = List::cons(Str::lit("tool"), List::cons(v13, List::cons(Str::lit("has an x-mcp-header that is not an HTTP field name"), List::<Str>::nil())));
            let a71 = Str::lit(" ");
            join(a70, a71)
        }
        Adt93::C10(v14) => {
            let a72 = List::cons(Str::lit("tool"), List::cons(v14, List::cons(Str::lit("has an x-mcp-header that is not reached through properties alone"), List::<Str>::nil())));
            let a73 = Str::lit(" ");
            join(a72, a73)
        }
        Adt93::C11(v15) => {
            let a74 = List::cons(Str::lit("tool"), List::cons(v15, List::cons(Str::lit("has an x-mcp-header on a parameter that is not a string, integer or boolean"), List::<Str>::nil())));
            let a75 = Str::lit(" ");
            join(a74, a75)
        }
        Adt93::C12(v16, v17) => {
            let a76 = List::cons(Str::lit("tool"), List::cons(v16, List::cons(Str::lit("repeats the x-mcp-header"), List::cons(v17, List::<Str>::nil()))));
            let a77 = Str::lit(" ");
            join(a76, a77)
        }
    }
}

pub fn f32(v0: Str, p83: List<Adt37>) -> Option<Adt37> {
    let m82 = p83;
    {
        let mut m82 = m82;
        loop {
            let step = {
                let v1 = m82;
                let m78 = v1;
                match m78.uncons() {
                    None => {
                        Err::<List<Adt37>, Option<Adt37>>(None::<Adt37>)
                    }
                    Some((v2, v3)) => {
                        let a80 = match v2.clone() {
                            Adt37::C0(h79, _, _, _, _, _, _, _) => {
                                h79
                            }
                        };
                        let a81 = v0.clone();
                        if equal(a80, a81) {
                            Err::<List<Adt37>, Option<Adt37>>(Some(v2))
                        } else {
                            Ok::<List<Adt37>, Option<Adt37>>(v3)
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

pub fn f33(v0: Adt37, v1: Fn1, v2: Adt23) -> R<Result<Adt34, Adt86>> {
    f50(v0.clone(), v2, Fn3::F51(v1, v0))
}

pub fn f34(v0: List<Adt15>, v1: Adt23) -> R<Result<Adt51, Adt86>> {
    let m84 = f52(v1)?;
    match m84 {
        None => {
            Ok(Err::<Adt51, Adt86>(Adt86::C4(Str::lit("a file name holds a line feed, or a draft is over draft.max_bytes"))))
        }
        Some(v2) => {
            let m85 = v0;
            match m85.uncons() {
                None => {
                    Ok(f53(f54(match v2.clone() {
                        Adt96::C0(h86, _) => {
                            h86
                        }
                    }), match v2 {
                        Adt96::C0(_, h87) => {
                            h87
                        }
                    }))
                }
                Some((v3, v4)) => {
                    let m88 = v4;
                    match m88.uncons() {
                        None => {
                            let a91 = match v3.clone() {
                                Adt15::C0(h89, _) => {
                                    h89
                                }
                            };
                            let a92 = f54(match v2.clone() {
                                Adt96::C0(h90, _) => {
                                    h90
                                }
                            });
                            if nat_eq(a91, a92) {
                                Ok(Ok::<Adt51, Adt86>(f55(match v2 {
                                    Adt96::C0(h93, _) => {
                                        h93
                                    }
                                }, match v3 {
                                    Adt15::C0(_, h94) => {
                                        h94
                                    }
                                })?))
                            } else {
                                Ok(f56(Str::lit("the host performed another effect")))
                            }
                        }
                        Some((_, _)) => {
                            Ok(f56(Str::lit("the host performed more than one effect")))
                        }
                    }
                }
            }
        }
    }
}

pub fn f35(v0: Bytes, p97: u64, p98: List<u64>) -> R<List<u64>> {
    let m96 = (p97, p98);
    {
        let mut m96 = m96;
        loop {
            let step = {
                let (v1, v2) = m96;
                let m95 = v1;
                if m95 == 0 {
                    Err::<(u64, List<u64>), R<List<u64>>>(Ok(v2))
                } else {
                    let v3 = m95 - 1;
                    Ok::<(u64, List<u64>), R<List<u64>>>((v3, List::cons(f57(v0.clone(), v3)?, v2)))
                }
            };
            match step {
                Ok(next) => { m96 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f36(v0: Bytes, v1: Option<Adt94>) -> R<Result<Adt89, Adt90>> {
    let m99 = v1;
    match m99 {
        None => {
            Ok(Err::<Adt89, Adt90>(Adt90::C0(0u64)))
        }
        Some(v2) => {
            f58(v0.clone(), v2.clone(), f59(v0, match v2 {
                Adt94::C0(_, h100) => {
                    h100
                }
            })?)
        }
    }
}

pub fn f37(v0: Bytes, v1: u64) -> R<Option<Adt94>> {
    let m101 = f59(v0.clone(), v1)?;
    match m101 {
        None => {
            Ok(None::<Adt94>)
        }
        Some(v2) => {
            let m107 = {
                let a104 = v0;
                let a105 = {
                    let a102 = v1;
                    let a103 = 4u64;
                    nat_add(a102, a103)?
                };
                let a106 = v2;
                slice_bytes(a104, a105, a106)
            };
            match m107 {
                None => {
                    Ok(None::<Adt94>)
                }
                Some(v3) => {
                    Ok(Some(Adt94::C0(v3, {
                        let a110 = {
                            let a108 = v1;
                            let a109 = 4u64;
                            nat_add(a108, a109)?
                        };
                        let a111 = v2;
                        nat_add(a110, a111)?
                    })))
                }
            }
        }
    }
}

pub fn f38(v0: Adt95) -> List<u64> {
    let a132 = f17(match v0.clone() {
        Adt95::C0(h112, _, _, _) => {
            h112
        }
    });
    let a133 = {
        let a130 = f17({
            let a114 = match v0.clone() {
                Adt95::C0(_, h113, _, _) => {
                    h113
                }
            };
            length_list(a114)
        });
        let a131 = {
            let a128 = f60(match v0.clone() {
                Adt95::C0(_, h115, _, _) => {
                    h115
                }
            });
            let a129 = {
                let a126 = f17({
                    let a117 = match v0.clone() {
                        Adt95::C0(_, _, h116, _) => {
                            h116
                        }
                    };
                    length_list(a117)
                });
                let a127 = {
                    let a124 = f61(match v0.clone() {
                        Adt95::C0(_, _, h118, _) => {
                            h118
                        }
                    });
                    let a125 = {
                        let a122 = f17({
                            let a120 = match v0.clone() {
                                Adt95::C0(_, _, _, h119) => {
                                    h119
                                }
                            };
                            length_list(a120)
                        });
                        let a123 = match v0 {
                            Adt95::C0(_, _, _, h121) => {
                                h121
                            }
                        };
                        append_list(a122, a123)
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
}

pub fn f39() -> Adt95 {
    Adt95::C0(f62(), f63(), List::<List<u64>>::nil(), f64())
}

pub fn f40(v0: Adt0, v1: Fn2, v2: Adt91) -> R<Adt88> {
    f65(v0, v1, v2.clone(), f66(match v2.clone() {
        Adt91::C0(_, _, h134, _, _, _, _) => {
            h134
        }
    }, match v2 {
        Adt91::C0(_, _, _, _, _, h135, _) => {
            h135
        }
    })?)
}

pub fn f41(v0: Fn0, v1: Adt89, v2: Adt16) -> R<Result<Adt34, Adt86>> {
    let c136: Fn0 = v0;
    c136.apply(Adt14::C0(match v1 {
        Adt89::C0(_, h137) => {
            h137
        }
    }, v2))
}

pub fn f42(v0: Bytes, v1: Adt94, v2: Option<Adt94>) -> R<Result<Adt91, Adt90>> {
    let m138 = v2;
    match m138 {
        None => {
            Ok(Err::<Adt91, Adt90>(Adt90::C0(match v1 {
                Adt94::C0(_, h139) => {
                    h139
                }
            })))
        }
        Some(v3) => {
            f67(v0.clone(), v1, v3.clone(), f59(v0, match v3 {
                Adt94::C0(_, h140) => {
                    h140
                }
            })?)
        }
    }
}

pub fn f43(v0: List<Adt37>) -> Option<Adt93> {
    let m141 = f68(v0, List::<Str>::nil());
    match m141 {
        None => {
            None::<Adt93>
        }
        Some(v1) => {
            Some(Adt93::C1(v1))
        }
    }
}

pub fn f44(p145: List<Adt37>, p146: List<Adt12>) -> R<Result<List<Adt12>, Adt93>> {
    let m144 = (p145, p146);
    {
        let mut m144 = m144;
        loop {
            let step = {
                let (v0, v1) = m144;
                let m142 = v0;
                match m142.uncons() {
                    None => {
                        Err::<(List<Adt37>, List<Adt12>), R<Result<List<Adt12>, Adt93>>>(Ok(Ok::<List<Adt12>, Adt93>(f69(v1, List::<Adt12>::nil()))))
                    }
                    Some((v2, v3)) => {
                        let m143 = f70(v2)?;
                        match m143 {
                            Ok(v5) => {
                                Ok::<(List<Adt37>, List<Adt12>), R<Result<List<Adt12>, Adt93>>>((v3, f69(v5, v1)))
                            }
                            Err(v4) => {
                                Err::<(List<Adt37>, List<Adt12>), R<Result<List<Adt12>, Adt93>>>(Ok(Err::<List<Adt12>, Adt93>(v4)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m144 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f45(v0: Str) -> Adt6 {
    Adt6::C3(v0)
}

pub fn f46(p151: List<Str>, p152: List<Adt6>) -> Adt6 {
    let m150 = (p151, p152);
    {
        let mut m150 = m150;
        loop {
            let step = {
                let (v0, v1) = m150;
                let m147 = v0;
                match m147.uncons() {
                    None => {
                        Err::<(List<Str>, List<Adt6>), Adt6>(Adt6::C4(v1))
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Str>, List<Adt6>), Adt6>((v3, {
                            let a148 = v1;
                            let a149 = List::cons(f45(v2), List::<Adt6>::nil());
                            append_list(a148, a149)
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m150 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f47(v0: bool) -> Adt6 {
    Adt6::C1(v0)
}

pub fn f48(v0: u64) -> R<Adt6> {
    Ok(Adt6::C2(Adt7::C0(f71(v0, 0i64)?, 0i64)))
}

pub fn f49() -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), f45(Str::lit("array"))), f27(Str::lit("items"), f28(Str::lit("string")))))
}

pub fn f50(v0: Adt37, v1: Adt23, v2: Fn3) -> R<Result<Adt34, Adt86>> {
    let m159 = f72(Adt6::C5({
        let m155 = match v0.clone() {
            Adt37::C0(_, _, _, _, h154, _, _, _) => {
                h154
            }
        };
        match m155 {
            Adt5::C0(h153) => {
                h153
            }
        }
    }), Adt6::C5({
        let m158 = match v0 {
            Adt37::C0(_, _, _, _, h157, _, _, _) => {
                h157
            }
        };
        match m158 {
            Adt5::C0(h156) => {
                h156
            }
        }
    }), f73(v1.clone()))?;
    match m159 {
        Adt99::C0 => {
            let c160: Fn3 = v2;
            c160.apply(v1)
        }
        Adt99::C1(v3, v4) => {
            Ok(Ok::<Adt34, Adt86>(Adt34::C2(f74(f75(v3, v4)?, true))))
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

pub fn f51(v0: Fn1, v1: Adt37, v2: Adt23) -> R<Result<Adt34, Adt86>> {
    f76(v1, v0, v2)
}

pub fn f52(v0: Adt23) -> R<Option<Adt96>> {
    if f77(v0.clone(), Str::lit("lexlean_draft_write")) {
        let m161 = f78(v0.clone(), Str::lit("path"));
        match m161 {
            None => {
                Ok(None::<Adt96>)
            }
            Some(v1) => {
                let m162 = f78(v0, Str::lit("content"));
                match m162 {
                    None => {
                        Ok(None::<Adt96>)
                    }
                    Some(v2) => {
                        let a164 = f79();
                        let a165 = {
                            let a163 = f80(v2.clone())?;
                            length_list(a163)
                        };
                        if nat_lt(a164, a165) {
                            Ok(None::<Adt96>)
                        } else {
                            Ok(Some(Adt96::C0(Adt97::C1, {
                                let a166 = f81(v1)?;
                                let a167 = f82(f80(v2)?);
                                append_list(a166, a167)
                            })))
                        }
                    }
                }
            }
        }
    } else {
        if f77(v0.clone(), Str::lit("lexlean_draft_read")) {
            let m168 = f78(v0, Str::lit("path"));
            match m168 {
                None => {
                    Ok(None::<Adt96>)
                }
                Some(v3) => {
                    Ok(Some(Adt96::C0(Adt97::C0, f81(v3)?)))
                }
            }
        } else {
            if f77(v0.clone(), Str::lit("lexlean_check")) {
                f83(Adt97::C2, v0, List::<u64>::nil())
            } else {
                if f77(v0.clone(), Str::lit("lexlean_snapshot")) {
                    f83(Adt97::C3, v0, List::<u64>::nil())
                } else {
                    if f77(v0.clone(), Str::lit("lexlean_format")) {
                        f83(Adt97::C4, v0.clone(), f84(f85(v0, Str::lit("check_only"))))
                    } else {
                        if f77(v0.clone(), Str::lit("lexlean_build")) {
                            f83(Adt97::C5, v0, List::<u64>::nil())
                        } else {
                            if f77(v0.clone(), Str::lit("lexlean_verify")) {
                                f83(Adt97::C6, v0, List::<u64>::nil())
                            } else {
                                if f77(v0.clone(), Str::lit("lexlean_verify_control")) {
                                    let m169 = f78(v0.clone(), Str::lit("task"));
                                    match m169 {
                                        None => {
                                            Ok(None::<Adt96>)
                                        }
                                        Some(v4) => {
                                            let m170 = f78(v0, Str::lit("action"));
                                            match m170 {
                                                None => {
                                                    Ok(None::<Adt96>)
                                                }
                                                Some(v5) => {
                                                    Ok(Some(Adt96::C0(Adt97::C7, {
                                                        let a173 = f86(v4)?;
                                                        let a174 = f17({
                                                            let a171 = v5;
                                                            let a172 = Str::lit("cancel");
                                                            if equal(a171, a172) {
                                                                1u64
                                                            } else {
                                                                0u64
                                                            }
                                                        });
                                                        append_list(a173, a174)
                                                    })))
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    if f77(v0.clone(), Str::lit("lexlean_lexicon_read")) {
                                        let m175 = f78(v0.clone(), Str::lit("profile"));
                                        match m175 {
                                            None => {
                                                Ok(None::<Adt96>)
                                            }
                                            Some(v6) => {
                                                let m176 = f78(v0, Str::lit("entry"));
                                                match m176 {
                                                    None => {
                                                        Ok(None::<Adt96>)
                                                    }
                                                    Some(v7) => {
                                                        Ok(Some(Adt96::C0(Adt97::C8, {
                                                            let a177 = f86(v6)?;
                                                            let a178 = f86(v7)?;
                                                            append_list(a177, a178)
                                                        })))
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        if f77(v0.clone(), Str::lit("lexlean_attestation")) {
                                            let m179 = f78(v0.clone(), Str::lit("by"));
                                            match m179 {
                                                None => {
                                                    Ok(None::<Adt96>)
                                                }
                                                Some(v8) => {
                                                    let m180 = f78(v0, Str::lit("value"));
                                                    match m180 {
                                                        None => {
                                                            Ok(None::<Adt96>)
                                                        }
                                                        Some(v9) => {
                                                            Ok(Some(Adt96::C0(Adt97::C9, {
                                                                let a183 = f17({
                                                                    let a181 = v8;
                                                                    let a182 = Str::lit("digest");
                                                                    if equal(a181, a182) {
                                                                        1u64
                                                                    } else {
                                                                        0u64
                                                                    }
                                                                });
                                                                let a184 = f86(v9)?;
                                                                append_list(a183, a184)
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

pub fn f53(v0: u64, v1: List<u64>) -> Result<Adt51, Adt86> {
    Err::<Adt51, Adt86>(Adt86::C15(Adt87::C0(v0, f87(f88(), v1))))
}

pub fn f54(v0: Adt97) -> u64 {
    let m185 = v0;
    match m185 {
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

pub fn f55(v0: Adt97, v1: Bytes) -> R<Adt51> {
    let a186 = f89(v1.clone(), 0u64)?;
    let a187 = 1u64;
    if nat_eq(a186, a187) {
        Ok(f90(v0, f89(v1, 4u64)?))
    } else {
        let m188 = v0;
        match m188 {
            Adt97::C0 => {
                f91(v1)
            }
            Adt97::C1 => {
                f92(v1)
            }
            Adt97::C2 => {
                f93(v1)
            }
            Adt97::C3 => {
                f93(v1)
            }
            Adt97::C4 => {
                f93(v1)
            }
            Adt97::C5 => {
                f93(v1)
            }
            Adt97::C6 => {
                f94(v1)
            }
            Adt97::C7 => {
                f95(v1)
            }
            Adt97::C8 => {
                f91(v1)
            }
            Adt97::C9 => {
                f91(v1)
            }
        }
    }
}

pub fn f56(v0: Str) -> Result<Adt51, Adt86> {
    Err::<Adt51, Adt86>(Adt86::C9(v0))
}

pub fn f57(v0: Bytes, v1: u64) -> R<u64> {
    let m191 = {
        let a189 = v0;
        let a190 = v1;
        index_bytes(a189, a190)
    };
    match m191 {
        None => {
            Ok(0u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a236 = {
                let a212 = {
                    let a200 = {
                        let a194 = {
                            let a192 = v3;
                            let a193 = 128u8;
                            fixed_u8::bit_and(a192, a193)
                        };
                        let a195 = 128u8;
                        if equal(a194, a195) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a201 = {
                        let a198 = {
                            let a196 = v3;
                            let a197 = 64u8;
                            fixed_u8::bit_and(a196, a197)
                        };
                        let a199 = 64u8;
                        if equal(a198, a199) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a200, a201)?
                };
                let a213 = {
                    let a210 = {
                        let a204 = {
                            let a202 = v3;
                            let a203 = 32u8;
                            fixed_u8::bit_and(a202, a203)
                        };
                        let a205 = 32u8;
                        if equal(a204, a205) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a211 = {
                        let a208 = {
                            let a206 = v3;
                            let a207 = 16u8;
                            fixed_u8::bit_and(a206, a207)
                        };
                        let a209 = 16u8;
                        if equal(a208, a209) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a210, a211)?
                };
                nat_add(a212, a213)?
            };
            let a237 = {
                let a234 = {
                    let a222 = {
                        let a216 = {
                            let a214 = v3;
                            let a215 = 8u8;
                            fixed_u8::bit_and(a214, a215)
                        };
                        let a217 = 8u8;
                        if equal(a216, a217) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a223 = {
                        let a220 = {
                            let a218 = v3;
                            let a219 = 4u8;
                            fixed_u8::bit_and(a218, a219)
                        };
                        let a221 = 4u8;
                        if equal(a220, a221) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a222, a223)?
                };
                let a235 = {
                    let a232 = {
                        let a226 = {
                            let a224 = v3;
                            let a225 = 2u8;
                            fixed_u8::bit_and(a224, a225)
                        };
                        let a227 = 2u8;
                        if equal(a226, a227) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a233 = {
                        let a230 = {
                            let a228 = v3;
                            let a229 = 1u8;
                            fixed_u8::bit_and(a228, a229)
                        };
                        let a231 = 1u8;
                        if equal(a230, a231) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a232, a233)?
                };
                nat_add(a234, a235)?
            };
            nat_add(a236, a237)
        }
    }
}

pub fn f58(v0: Bytes, v1: Adt94, v2: Option<u64>) -> R<Result<Adt89, Adt90>> {
    let m238 = v2;
    match m238 {
        None => {
            Ok(Err::<Adt89, Adt90>(Adt90::C0(match v1 {
                Adt94::C0(_, h239) => {
                    h239
                }
            })))
        }
        Some(v3) => {
            let m243 = f96(v0.clone(), {
                let a241 = match v1.clone() {
                    Adt94::C0(_, h240) => {
                        h240
                    }
                };
                let a242 = 4u64;
                nat_add(a241, a242)?
            }, v3, List::<Adt15>::nil())?;
            match m243 {
                Ok(v5) => {
                    Ok(f97(v0, match v1 {
                        Adt94::C0(h244, _) => {
                            h244
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

pub fn f59(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a248 = {
        let a245 = v1;
        let a246 = 4u64;
        nat_add(a245, a246)?
    };
    let a249 = {
        let a247 = v0.clone();
        length_bytes(a247)
    };
    if nat_le(a248, a249) {
        Ok(Some({
            let a266 = {
                let a250 = f57(v0.clone(), v1)?;
                let a251 = 16777216u64;
                nat_mul(a250, a251)?
            };
            let a267 = {
                let a264 = {
                    let a254 = f57(v0.clone(), {
                        let a252 = v1;
                        let a253 = 1u64;
                        nat_add(a252, a253)?
                    })?;
                    let a255 = 65536u64;
                    nat_mul(a254, a255)?
                };
                let a265 = {
                    let a262 = {
                        let a258 = f57(v0.clone(), {
                            let a256 = v1;
                            let a257 = 2u64;
                            nat_add(a256, a257)?
                        })?;
                        let a259 = 256u64;
                        nat_mul(a258, a259)?
                    };
                    let a263 = f57(v0, {
                        let a260 = v1;
                        let a261 = 3u64;
                        nat_add(a260, a261)?
                    })?;
                    nat_add(a262, a263)?
                };
                nat_add(a264, a265)?
            };
            nat_add(a266, a267)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f60(v0: List<Adt92>) -> List<u64> {
    f98(f99(v0, List::<Adt92>::nil()), List::<u64>::nil())
}

pub fn f61(v0: List<List<u64>>) -> List<u64> {
    f100(f101(v0, List::<List<u64>>::nil()), List::<u64>::nil())
}

pub fn f62() -> u64 {
    500u64
}

pub fn f63() -> List<Adt92> {
    List::cons(Adt92::C0(f102(), f103()), List::<Adt92>::nil())
}

pub fn f64() -> List<u64> {
    let a270 = f104();
    let a271 = {
        let a268 = f105();
        let a269 = f106();
        append_list(a268, a269)
    };
    append_list(a270, a271)
}

pub fn f65(v0: Adt0, v1: Fn2, v2: Adt91, v3: Option<Adt98>) -> R<Adt88> {
    let m272 = v3;
    match m272 {
        None => {
            f108(v0, v1, v2)
        }
        Some(v4) => {
            Ok(Adt88::C0(f38(f107(v4)?)))
        }
    }
}

pub fn f66(v0: List<Adt92>, v1: List<List<u64>>) -> R<Option<Adt98>> {
    let m273 = f109(v0.clone(), v1.clone())?;
    match m273 {
        None => {
            f110(v0, v1)
        }
        Some(v2) => {
            Ok(Some(v2))
        }
    }
}

pub fn f67(v0: Bytes, v1: Adt94, v2: Adt94, v3: Option<u64>) -> R<Result<Adt91, Adt90>> {
    let m274 = v3;
    match m274 {
        None => {
            Ok(Err::<Adt91, Adt90>(Adt90::C0(match v2 {
                Adt94::C0(_, h275) => {
                    h275
                }
            })))
        }
        Some(v4) => {
            let m279 = f111(v0.clone(), {
                let a277 = match v2.clone() {
                    Adt94::C0(_, h276) => {
                        h276
                    }
                };
                let a278 = 4u64;
                nat_add(a277, a278)?
            }, v4, List::<Adt92>::nil())?;
            match m279 {
                Ok(v6) => {
                    f112(v0, v1, v2, v6)
                }
                Err(v5) => {
                    Ok(Err::<Adt91, Adt90>(v5))
                }
            }
        }
    }
}

pub fn f68(p285: List<Adt37>, p286: List<Str>) -> Option<Str> {
    let m284 = (p285, p286);
    {
        let mut m284 = m284;
        loop {
            let step = {
                let (v0, v1) = m284;
                let m280 = v0;
                match m280.uncons() {
                    None => {
                        Err::<(List<Adt37>, List<Str>), Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        if f113(match v2.clone() {
                            Adt37::C0(h281, _, _, _, _, _, _, _) => {
                                h281
                            }
                        }, v1.clone()) {
                            Err::<(List<Adt37>, List<Str>), Option<Str>>(Some(match v2 {
                                Adt37::C0(h282, _, _, _, _, _, _, _) => {
                                    h282
                                }
                            }))
                        } else {
                            Ok::<(List<Adt37>, List<Str>), Option<Str>>((v3, List::cons(match v2 {
                                Adt37::C0(h283, _, _, _, _, _, _, _) => {
                                    h283
                                }
                            }, v1)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m284 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f69(p289: List<Adt12>, p290: List<Adt12>) -> List<Adt12> {
    let m288 = (p289, p290);
    {
        let mut m288 = m288;
        loop {
            let step = {
                let (v0, v1) = m288;
                let m287 = v0;
                match m287.uncons() {
                    None => {
                        Err::<(List<Adt12>, List<Adt12>), List<Adt12>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt12>, List<Adt12>), List<Adt12>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m288 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f70(v0: Adt37) -> R<Result<List<Adt12>, Adt93>> {
    let m293 = f114(match v0.clone() {
        Adt37::C0(h291, _, _, _, _, _, _, _) => {
            h291
        }
    }, f115(match v0.clone() {
        Adt37::C0(h292, _, _, _, _, _, _, _) => {
            h292
        }
    })?);
    match m293 {
        None => {
            f116(v0)
        }
        Some(v1) => {
            Ok(Err::<List<Adt12>, Adt93>(v1))
        }
    }
}

pub fn f71(p298: u64, p299: i64) -> R<i64> {
    let m297 = (p298, p299);
    {
        let mut m297 = m297;
        loop {
            let step = {
                let (v0, v1) = m297;
                let m294 = v0;
                if m294 == 0 {
                    Err::<(u64, i64), R<i64>>(Ok(v1))
                } else {
                    let v2 = m294 - 1;
                    Ok::<(u64, i64), R<i64>>((v2, {
                        let a295 = v1;
                        let a296 = -1i64;
                        int_sub(a295, a296)?
                    }))
                }
            };
            match step {
                Ok(next) => { m297 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f72(v0: Adt6, v1: Adt6, v2: Adt6) -> R<Adt99> {
    f117(v0, f118(), Adt101::C0, List::cons(Adt102::C0(v1, v2, List::<Adt100>::nil()), List::<Adt102>::nil()))
}

pub fn f73(v0: Adt23) -> Adt6 {
    let m301 = match v0 {
        Adt23::C0(_, _, h300, _, _) => {
            h300
        }
    };
    match m301 {
        None => {
            Adt6::C5(List::<(Str, Adt6)>::nil())
        }
        Some(v1) => {
            Adt6::C5(match v1 {
                Adt5::C0(h302) => {
                    h302
                }
            })
        }
    }
}

pub fn f74(v0: Str, v1: bool) -> Adt39 {
    Adt39::C0(List::cons(Adt40::C0(Adt41::C0(v0, None::<Adt42>, None::<Adt5>)), List::<Adt40>::nil()), None::<Adt6>, if v1 {
        Some(true)
    } else {
        None::<bool>
    })
}

pub fn f75(v0: List<Adt100>, v1: Str) -> R<Str> {
    let v2: Str = f119(v0)?;
    let a305 = List::cons(Str::lit("Invalid arguments:"), List::cons({
        let a303 = v2.clone();
        let a304 = Str::lit("");
        if equal(a303, a304) {
            Str::lit("the arguments object")
        } else {
            v2
        }
    }, List::cons(Str::lit("fails"), List::cons(v1, List::<Str>::nil()))));
    let a306 = Str::lit(" ");
    Ok(join(a305, a306))
}

pub fn f76(v0: Adt37, v1: Fn1, v2: Adt23) -> R<Result<Adt34, Adt86>> {
    let m308 = {
        let c307: Fn1 = v1;
        c307.apply(v2)?
    };
    match m308 {
        Ok(v4) => {
            f120(v0, v4)
        }
        Err(v3) => {
            Ok(Err::<Adt34, Adt86>(v3))
        }
    }
}

pub fn f77(v0: Adt23, v1: Str) -> bool {
    let a310 = match v0 {
        Adt23::C0(_, h309, _, _, _) => {
            h309
        }
    };
    let a311 = v1;
    equal(a310, a311)
}

pub fn f78(v0: Adt23, v1: Str) -> Option<Str> {
    let m312 = f121(v0, v1);
    match m312 {
        None => {
            None::<Str>
        }
        Some(v2) => {
            f122(v2)
        }
    }
}

pub fn f79() -> u64 {
    8388608u64
}

pub fn f80(v0: Str) -> R<List<u64>> {
    f18({
        let a313 = v0;
        utf8_encode(a313)
    })
}

pub fn f81(v0: Str) -> R<List<u64>> {
    let a314 = f17(f123(Str::lit("drafts")));
    let a315 = f86(v0)?;
    Ok(append_list(a314, a315))
}

pub fn f82(v0: List<u64>) -> List<u64> {
    let a317 = f17({
        let a316 = v0.clone();
        length_list(a316)
    });
    let a318 = v0;
    append_list(a317, a318)
}

pub fn f83(v0: Adt97, v1: Adt23, v2: List<u64>) -> R<Option<Adt96>> {
    let m319 = f78(v1.clone(), Str::lit("project"));
    match m319 {
        None => {
            Ok(None::<Adt96>)
        }
        Some(v3) => {
            let m320 = f124(v1);
            match m320 {
                None => {
                    Ok(None::<Adt96>)
                }
                Some(v4) => {
                    if f125(v4.clone())? {
                        Ok(None::<Adt96>)
                    } else {
                        Ok(Some(Adt96::C0(v0, {
                            let a323 = f86(v3)?;
                            let a324 = {
                                let a321 = f126(v4)?;
                                let a322 = v2;
                                append_list(a321, a322)
                            };
                            append_list(a323, a324)
                        })))
                    }
                }
            }
        }
    }
}

pub fn f84(v0: bool) -> List<u64> {
    if v0 {
        List::cons(1u64, List::<u64>::nil())
    } else {
        List::cons(0u64, List::<u64>::nil())
    }
}

pub fn f85(v0: Adt23, v1: Str) -> bool {
    let m325 = f121(v0, v1);
    match m325 {
        None => {
            false
        }
        Some(v2) => {
            let m326 = f127(v2);
            match m326 {
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

pub fn f86(v0: Str) -> R<List<u64>> {
    Ok(f82(f80(v0)?))
}

pub fn f87(v0: Bytes, v1: List<u64>) -> Bytes {
    let a327 = v0;
    let a328 = f128(f88(), f129(List::<Bytes>::nil(), v1));
    append_bytes(a327, a328)
}

pub fn f88() -> Bytes {
    Bytes::lit(&[])
}

pub fn f89(v0: Bytes, v1: u64) -> R<u64> {
    let m329 = f59(v0, v1)?;
    match m329 {
        None => {
            Ok(0u64)
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f90(v0: Adt97, v1: u64) -> Adt51 {
    let v2: Str = f130(f131(v0.clone()), v1);
    f132({
        let a330 = List::cons(f133(v0), List::cons(Str::lit("failed:"), List::cons(v2.clone(), List::<Str>::nil())));
        let a331 = Str::lit(" ");
        join(a330, a331)
    }, Adt6::C5(f27(Str::lit("failure"), f45(v2))), true)
}

pub fn f91(v0: Bytes) -> R<Adt51> {
    let m332 = f37(v0.clone(), 5u64)?;
    match m332 {
        None => {
            Ok(f134())
        }
        Some(v1) => {
            let a333 = f57(v0, 4u64)?;
            let a334 = 1u64;
            if nat_eq(a333, a334) {
                Ok(f132(f135(match v1 {
                    Adt94::C0(h335, _) => {
                        h335
                    }
                }), Adt6::C5(f27(Str::lit("found"), f47(true))), false))
            } else {
                Ok(f132(Str::lit("not found"), Adt6::C5(f27(Str::lit("found"), f47(false))), false))
            }
        }
    }
}

pub fn f92(v0: Bytes) -> R<Adt51> {
    let v1: u64 = {
        let a338 = {
            let a336 = f89(v0.clone(), 4u64)?;
            let a337 = 4294967296u64;
            nat_mul(a336, a337)?
        };
        let a339 = f89(v0, 8u64)?;
        nat_add(a338, a339)?
    };
    Ok(f132({
        let a341 = List::cons(Str::lit("written"), List::cons({
            let a340 = f136(v1, 0i64)?;
            format_i64(a340)
        }, List::cons(Str::lit("bytes"), List::<Str>::nil())));
        let a342 = Str::lit(" ");
        join(a341, a342)
    }, Adt6::C5(f27(Str::lit("length"), f48(v1)?)), false))
}

pub fn f93(v0: Bytes) -> R<Adt51> {
    let m343 = f37(v0.clone(), 8u64)?;
    match m343 {
        None => {
            Ok(f134())
        }
        Some(v1) => {
            let v2: u64 = f89(v0, 4u64)?;
            Ok(f132(f135(match v1 {
                Adt94::C0(h344, _) => {
                    h344
                }
            }), Adt6::C5(f27(Str::lit("class"), f48(v2)?)), {
                let a345 = v2;
                let a346 = 0u64;
                !nat_eq(a345, a346)
            }))
        }
    }
}

pub fn f94(v0: Bytes) -> R<Adt51> {
    let m347 = f37(v0.clone(), 4u64)?;
    match m347 {
        None => {
            Ok(f134())
        }
        Some(v1) => {
            let m350 = {
                let a349 = match v1.clone() {
                    Adt94::C0(h348, _) => {
                        h348
                    }
                };
                utf8_decode(a349)
            };
            match m350 {
                None => {
                    Ok(f134())
                }
                Some(v2) => {
                    Ok(f132(v2.clone(), Adt6::C5(List::cons((Str::lit("task"), f45(v2)), f27(Str::lit("state"), f45(f137(f89(v0, match v1 {
                        Adt94::C0(_, h351) => {
                            h351
                        }
                    })?))))), false))
                }
            }
        }
    }
}

pub fn f95(v0: Bytes) -> R<Adt51> {
    let m352 = f37(v0.clone(), 12u64)?;
    match m352 {
        None => {
            Ok(f134())
        }
        Some(v1) => {
            Ok(f132(f135(match v1.clone() {
                Adt94::C0(h353, _) => {
                    h353
                }
            }), Adt6::C5(List::cons((Str::lit("state"), f45(f137(f89(v0.clone(), 4u64)?))), List::cons((Str::lit("class"), f48(f89(v0.clone(), 8u64)?)?), List::cons((Str::lit("groupsEnded"), f48(f89(v0.clone(), match v1.clone() {
                Adt94::C0(_, h354) => {
                    h354
                }
            })?)?), f27(Str::lit("groupsFailed"), f48(f89(v0.clone(), {
                let a356 = match v1 {
                    Adt94::C0(_, h355) => {
                        h355
                    }
                };
                let a357 = 4u64;
                nat_add(a356, a357)?
            })?)?))))), {
                let a358 = f89(v0, 8u64)?;
                let a359 = 0u64;
                !nat_eq(a358, a359)
            }))
        }
    }
}

pub fn f96(v0: Bytes, p370: u64, p371: u64, p372: List<Adt15>) -> R<Result<(List<Adt15>, u64), Adt90>> {
    let m369 = (p370, (p371, p372));
    {
        let mut m369 = m369;
        loop {
            let step = {
                let (v1, (v2, v3)) = m369;
                let m360 = v2;
                if m360 == 0 {
                    Err::<(u64, (u64, List<Adt15>)), R<Result<(List<Adt15>, u64), Adt90>>>(Ok(Ok::<(List<Adt15>, u64), Adt90>((f138(v3, List::<Adt15>::nil()), v1))))
                } else {
                    let v4 = m360 - 1;
                    let m361 = f59(v0.clone(), v1)?;
                    match m361 {
                        None => {
                            Err::<(u64, (u64, List<Adt15>)), R<Result<(List<Adt15>, u64), Adt90>>>(Ok(Err::<(List<Adt15>, u64), Adt90>(Adt90::C0(v1))))
                        }
                        Some(v5) => {
                            let m364 = f37(v0.clone(), {
                                let a362 = v1;
                                let a363 = 4u64;
                                nat_add(a362, a363)?
                            })?;
                            match m364 {
                                None => {
                                    Err::<(u64, (u64, List<Adt15>)), R<Result<(List<Adt15>, u64), Adt90>>>(Ok(Err::<(List<Adt15>, u64), Adt90>(Adt90::C0({
                                        let a365 = v1;
                                        let a366 = 4u64;
                                        nat_add(a365, a366)?
                                    }))))
                                }
                                Some(v6) => {
                                    Ok::<(u64, (u64, List<Adt15>)), R<Result<(List<Adt15>, u64), Adt90>>>((match v6.clone() {
                                        Adt94::C0(_, h367) => {
                                            h367
                                        }
                                    }, (v4, List::cons(Adt15::C0(v5, match v6 {
                                        Adt94::C0(h368, _) => {
                                            h368
                                        }
                                    }), v3))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m369 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f97(v0: Bytes, v1: Bytes, v2: (List<Adt15>, u64)) -> Result<Adt89, Adt90> {
    f139({
        let a375 = {
            let (_, h373) = v2.clone();
            h373
        };
        let a376 = {
            let a374 = v0;
            length_bytes(a374)
        };
        nat_eq(a375, a376)
    }, v1, {
        let (h377, _) = v2;
        h377
    }, {
        let (_, h378) = v2;
        h378
    })
}

pub fn f98(p383: List<Adt92>, p384: List<u64>) -> List<u64> {
    let m382 = (p383, p384);
    {
        let mut m382 = m382;
        loop {
            let step = {
                let (v0, v1) = m382;
                let m379 = v0;
                match m379.uncons() {
                    None => {
                        Err::<(List<Adt92>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt92>, List<u64>), List<u64>>((v3, {
                            let a380 = f140(v2);
                            let a381 = v1;
                            append_list(a380, a381)
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m382 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f99(p387: List<Adt92>, p388: List<Adt92>) -> List<Adt92> {
    let m386 = (p387, p388);
    {
        let mut m386 = m386;
        loop {
            let step = {
                let (v0, v1) = m386;
                let m385 = v0;
                match m385.uncons() {
                    None => {
                        Err::<(List<Adt92>, List<Adt92>), List<Adt92>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt92>, List<Adt92>), List<Adt92>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m386 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f100(p393: List<List<u64>>, p394: List<u64>) -> List<u64> {
    let m392 = (p393, p394);
    {
        let mut m392 = m392;
        loop {
            let step = {
                let (v0, v1) = m392;
                let m389 = v0;
                match m389.uncons() {
                    None => {
                        Err::<(List<List<u64>>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<u64>>, List<u64>), List<u64>>((v3, {
                            let a390 = f141(v2);
                            let a391 = v1;
                            append_list(a390, a391)
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m392 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f101(p397: List<List<u64>>, p398: List<List<u64>>) -> List<List<u64>> {
    let m396 = (p397, p398);
    {
        let mut m396 = m396;
        loop {
            let step = {
                let (v0, v1) = m396;
                let m395 = v0;
                match m395.uncons() {
                    None => {
                        Err::<(List<List<u64>>, List<List<u64>>), List<List<u64>>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<u64>>, List<List<u64>>), List<List<u64>>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m396 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f102() -> List<u64> {
    List::cons(67u64, List::cons(111u64, List::cons(110u64, List::cons(116u64, List::cons(101u64, List::cons(110u64, List::cons(116u64, List::cons(45u64, List::cons(84u64, List::cons(121u64, List::cons(112u64, List::cons(101u64, List::<u64>::nil()))))))))))))
}

pub fn f103() -> List<u64> {
    List::cons(97u64, List::cons(112u64, List::cons(112u64, List::cons(108u64, List::cons(105u64, List::cons(99u64, List::cons(97u64, List::cons(116u64, List::cons(105u64, List::cons(111u64, List::cons(110u64, List::cons(47u64, List::cons(106u64, List::cons(115u64, List::cons(111u64, List::cons(110u64, List::<u64>::nil()))))))))))))))))
}

pub fn f104() -> List<u64> {
    List::cons(123u64, List::cons(34u64, List::cons(106u64, List::cons(115u64, List::cons(111u64, List::cons(110u64, List::cons(114u64, List::cons(112u64, List::cons(99u64, List::cons(34u64, List::cons(58u64, List::cons(34u64, List::cons(50u64, List::cons(46u64, List::cons(48u64, List::cons(34u64, List::<u64>::nil()))))))))))))))))
}

pub fn f105() -> List<u64> {
    List::cons(44u64, List::cons(34u64, List::cons(101u64, List::cons(114u64, List::cons(114u64, List::cons(111u64, List::cons(114u64, List::cons(34u64, List::cons(58u64, List::cons(123u64, List::cons(34u64, List::cons(99u64, List::cons(111u64, List::cons(100u64, List::cons(101u64, List::cons(34u64, List::cons(58u64, List::cons(45u64, List::cons(51u64, List::cons(50u64, List::cons(54u64, List::cons(48u64, List::cons(51u64, List::cons(44u64, List::cons(34u64, List::cons(109u64, List::<u64>::nil()))))))))))))))))))))))))))
}

pub fn f106() -> List<u64> {
    List::cons(101u64, List::cons(115u64, List::cons(115u64, List::cons(97u64, List::cons(103u64, List::cons(101u64, List::cons(34u64, List::cons(58u64, List::cons(34u64, List::cons(73u64, List::cons(110u64, List::cons(116u64, List::cons(101u64, List::cons(114u64, List::cons(110u64, List::cons(97u64, List::cons(108u64, List::cons(32u64, List::cons(101u64, List::cons(114u64, List::cons(114u64, List::cons(111u64, List::cons(114u64, List::cons(34u64, List::cons(125u64, List::cons(125u64, List::<u64>::nil()))))))))))))))))))))))))))
}

pub fn f107(v0: Adt98) -> R<Adt95> {
    f142(f143(v0.clone()), f144(None::<Adt104>, f145(v0)))
}

pub fn f108(v0: Adt0, v1: Fn2, v2: Adt91) -> R<Adt88> {
    if f146(match v2.clone() {
        Adt91::C0(h399, _, _, _, _, _, _) => {
            h399
        }
    }) {
        f147(f148(v0, match v2.clone() {
            Adt91::C0(_, _, _, _, _, _, h400) => {
                h400
            }
        }), v1, match v2.clone() {
            Adt91::C0(_, _, _, _, h401, _, _) => {
                h401
            }
        }, match v2.clone() {
            Adt91::C0(_, _, h402, _, _, _, _) => {
                h402
            }
        }, f149(match v2 {
            Adt91::C0(_, _, _, h403, _, _, _) => {
                h403
            }
        })?)
    } else {
        Ok(Adt88::C0(f38(f150())))
    }
}

pub fn f109(v0: List<Adt92>, v1: List<List<u64>>) -> R<Option<Adt98>> {
    let m404 = f151(v0, f152())?;
    match m404 {
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
            f153(f154(v2), v1, Adt98::C0)
        }
    }
}

pub fn f110(v0: List<Adt92>, v1: List<List<u64>>) -> R<Option<Adt98>> {
    let m405 = f151(v0, f155())?;
    match m405 {
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
            f153(f156(v2), v1, Adt98::C1)
        }
    }
}

pub fn f111(v0: Bytes, p415: u64, p416: u64, p417: List<Adt92>) -> R<Result<(List<Adt92>, u64), Adt90>> {
    let m414 = (p415, (p416, p417));
    {
        let mut m414 = m414;
        loop {
            let step = {
                let (v1, (v2, v3)) = m414;
                let m406 = v2;
                if m406 == 0 {
                    Err::<(u64, (u64, List<Adt92>)), R<Result<(List<Adt92>, u64), Adt90>>>(Ok(Ok::<(List<Adt92>, u64), Adt90>((f157(v3, List::<Adt92>::nil()), v1))))
                } else {
                    let v4 = m406 - 1;
                    let m407 = f37(v0.clone(), v1)?;
                    match m407 {
                        None => {
                            Err::<(u64, (u64, List<Adt92>)), R<Result<(List<Adt92>, u64), Adt90>>>(Ok(Err::<(List<Adt92>, u64), Adt90>(Adt90::C0(v1))))
                        }
                        Some(v5) => {
                            let m409 = f37(v0.clone(), match v5.clone() {
                                Adt94::C0(_, h408) => {
                                    h408
                                }
                            })?;
                            match m409 {
                                None => {
                                    Err::<(u64, (u64, List<Adt92>)), R<Result<(List<Adt92>, u64), Adt90>>>(Ok(Err::<(List<Adt92>, u64), Adt90>(Adt90::C0(match v5 {
                                        Adt94::C0(_, h410) => {
                                            h410
                                        }
                                    }))))
                                }
                                Some(v6) => {
                                    Ok::<(u64, (u64, List<Adt92>)), R<Result<(List<Adt92>, u64), Adt90>>>((match v6.clone() {
                                        Adt94::C0(_, h411) => {
                                            h411
                                        }
                                    }, (v4, List::cons(Adt92::C0(f18(match v5 {
                                        Adt94::C0(h412, _) => {
                                            h412
                                        }
                                    })?, f18(match v6 {
                                        Adt94::C0(h413, _) => {
                                            h413
                                        }
                                    })?), v3))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m414 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f112(v0: Bytes, v1: Adt94, v2: Adt94, v3: (List<Adt92>, u64)) -> R<Result<Adt91, Adt90>> {
    let m419 = f37(v0.clone(), {
        let (_, h418) = v3.clone();
        h418
    })?;
    match m419 {
        None => {
            Ok(Err::<Adt91, Adt90>(Adt90::C0({
                let (_, h420) = v3;
                h420
            })))
        }
        Some(v4) => {
            let m422 = f59(v0.clone(), match v4.clone() {
                Adt94::C0(_, h421) => {
                    h421
                }
            })?;
            match m422 {
                None => {
                    Ok(Err::<Adt91, Adt90>(Adt90::C0(match v4 {
                        Adt94::C0(_, h423) => {
                            h423
                        }
                    })))
                }
                Some(v5) => {
                    let m427 = f158(v0.clone(), {
                        let a425 = match v4.clone() {
                            Adt94::C0(_, h424) => {
                                h424
                            }
                        };
                        let a426 = 4u64;
                        nat_add(a425, a426)?
                    })?;
                    match m427 {
                        None => {
                            Ok(Err::<Adt91, Adt90>(Adt90::C0({
                                let a429 = match v4 {
                                    Adt94::C0(_, h428) => {
                                        h428
                                    }
                                };
                                let a430 = 4u64;
                                nat_add(a429, a430)?
                            })))
                        }
                        Some(v6) => {
                            f159(v0, Adt91::C0(f18(match v1 {
                                Adt94::C0(h431, _) => {
                                    h431
                                }
                            })?, f18(match v2 {
                                Adt94::C0(h432, _) => {
                                    h432
                                }
                            })?, {
                                let (h433, _) = v3;
                                h433
                            }, match v4 {
                                Adt94::C0(h434, _) => {
                                    h434
                                }
                            }, v5, List::<List<u64>>::nil(), match v6.clone() {
                                Adt94::C0(h435, _) => {
                                    h435
                                }
                            }), match v6 {
                                Adt94::C0(_, h436) => {
                                    h436
                                }
                            })
                        }
                    }
                }
            }
        }
    }
}

pub fn f113(v0: Str, p441: List<Str>) -> bool {
    let m440 = p441;
    {
        let mut m440 = m440;
        loop {
            let step = {
                let v1 = m440;
                let m437 = v1;
                match m437.uncons() {
                    None => {
                        Err::<List<Str>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a438 = v0.clone();
                        let a439 = v2;
                        if equal(a438, a439) {
                            Err::<List<Str>, bool>(true)
                        } else {
                            Ok::<List<Str>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m440 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f114(v0: Str, v1: List<u64>) -> Option<Adt93> {
    if f160(v1) {
        Some(Adt93::C0(v0))
    } else {
        None::<Adt93>
    }
}

pub fn f115(v0: Str) -> R<List<u64>> {
    f18({
        let a442 = v0;
        utf8_encode(a442)
    })
}

pub fn f116(v0: Adt37) -> R<Result<List<Adt12>, Adt93>> {
    let m443 = f161(v0.clone());
    match m443 {
        None => {
            f162(v0.clone(), f163(f164(), match v0.clone() {
                Adt37::C0(h444, _, _, _, _, _, _, _) => {
                    h444
                }
            }, Adt6::C5({
                let m447 = match v0.clone() {
                    Adt37::C0(_, _, _, _, h446, _, _, _) => {
                        h446
                    }
                };
                match m447 {
                    Adt5::C0(h445) => {
                        h445
                    }
                }
            }), List::cons(Adt105::C0(Adt6::C5({
                let m450 = match v0 {
                    Adt37::C0(_, _, _, _, h449, _, _, _) => {
                        h449
                    }
                };
                match m450 {
                    Adt5::C0(h448) => {
                        h448
                    }
                }
            }), true, List::<Str>::nil()), List::<Adt105>::nil()), f165())?)
        }
        Some(v1) => {
            Ok(Err::<List<Adt12>, Adt93>(v1))
        }
    }
}

pub fn f117(v0: Adt6, p469: u64, p470: Adt101, p471: List<Adt102>) -> R<Adt99> {
    let m468 = (p469, (p470, p471));
    {
        let mut m468 = m468;
        loop {
            let step = {
                let (v1, (v2, v3)) = m468;
                let m451 = v1;
                if m451 == 0 {
                    Err::<(u64, (Adt101, List<Adt102>)), R<Adt99>>(Ok(Adt99::C4))
                } else {
                    let v4 = m451 - 1;
                    let m452 = v3;
                    match m452.uncons() {
                        None => {
                            let m453 = v2;
                            match m453 {
                                Adt101::C0 => {
                                    Err::<(u64, (Adt101, List<Adt102>)), R<Adt99>>(Ok(Adt99::C0))
                                }
                                Adt101::C1(v5, v6) => {
                                    Err::<(u64, (Adt101, List<Adt102>)), R<Adt99>>(Ok(Adt99::C1(v5, v6)))
                                }
                            }
                        }
                        Some((v7, v8)) => {
                            let m454 = v2.clone();
                            match m454 {
                                Adt101::C0 => {
                                    let m455 = v7.clone();
                                    match m455 {
                                        Adt102::C0(v9, v10, v11) => {
                                            let m456 = f166(v0.clone(), v9, v10, v11.clone())?;
                                            match m456 {
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
                                                        let a457 = v15;
                                                        let a458 = v8;
                                                        append_list(a457, a458)
                                                    };
                                                    Ok::<(u64, (Adt101, List<Adt102>)), R<Adt99>>((v4, (v2, v16)))
                                                }
                                            }
                                        }
                                        Adt102::C1(_, _, _) => {
                                            Ok::<(u64, (Adt101, List<Adt102>)), R<Adt99>>((v4, (v2, v8)))
                                        }
                                        Adt102::C2(_, _, _, _) => {
                                            let v24: Adt106 = f167(v7, v8)?;
                                            Ok::<(u64, (Adt101, List<Adt102>)), R<Adt99>>((v4, (match v24.clone() {
                                                Adt106::C0(h459, _) => {
                                                    h459
                                                }
                                            }, match v24 {
                                                Adt106::C0(_, h460) => {
                                                    h460
                                                }
                                            })))
                                        }
                                        Adt102::C3(v25) => {
                                            Ok::<(u64, (Adt101, List<Adt102>)), R<Adt99>>((v4, (Adt101::C1(v25, Str::lit("not")), v8)))
                                        }
                                        Adt102::C4(_, _, _, _) => {
                                            let v39: Adt106 = f167(v7, v8)?;
                                            Ok::<(u64, (Adt101, List<Adt102>)), R<Adt99>>((v4, (match v39.clone() {
                                                Adt106::C0(h464, _) => {
                                                    h464
                                                }
                                            }, match v39 {
                                                Adt106::C0(_, h465) => {
                                                    h465
                                                }
                                            })))
                                        }
                                        Adt102::C5(v26, v27, v28, v29) => {
                                            let m461 = f168(v26, v27, v28, v29.clone());
                                            match m461 {
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
                                                        let a462 = v33;
                                                        let a463 = v8;
                                                        append_list(a462, a463)
                                                    };
                                                    Ok::<(u64, (Adt101, List<Adt102>)), R<Adt99>>((v4, (v2, v34)))
                                                }
                                            }
                                        }
                                    }
                                }
                                Adt101::C1(v40, v41) => {
                                    let v42: Adt106 = f169(v7, v8, v40, v41);
                                    Ok::<(u64, (Adt101, List<Adt102>)), R<Adt99>>((v4, (match v42.clone() {
                                        Adt106::C0(h466, _) => {
                                            h466
                                        }
                                    }, match v42 {
                                        Adt106::C0(_, h467) => {
                                            h467
                                        }
                                    })))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m468 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f118() -> u64 {
    67108864u64
}

pub fn f119(v0: List<Adt100>) -> R<Str> {
    let a472 = List::cons(Str::lit(""), f170(v0, List::<Str>::nil())?);
    let a473 = Str::lit("/");
    Ok(join(a472, a473))
}

pub fn f120(v0: Adt37, v1: Adt51) -> R<Result<Adt34, Adt86>> {
    if f171({
        let m476 = match v1.clone() {
            Adt51::C0(h475, _) => {
                h475
            }
        };
        match m476 {
            Adt39::C0(h474, _, _) => {
                h474
            }
        }
    })? {
        f172(v0, v1)
    } else {
        Ok(Err::<Adt34, Adt86>(Adt86::C6(Str::lit("a content payload is not canonical Base64"))))
    }
}

pub fn f121(v0: Adt23, v1: Str) -> Option<Adt6> {
    let m478 = match v0 {
        Adt23::C0(_, _, h477, _, _) => {
            h477
        }
    };
    match m478 {
        None => {
            None::<Adt6>
        }
        Some(v2) => {
            let m479 = f173(v1, v2);
            match m479 {
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

pub fn f122(v0: Adt6) -> Option<Str> {
    let m480 = v0;
    match m480 {
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

pub fn f123(v0: Str) -> u64 {
    let a481 = v0;
    let a482 = Str::lit("drafts");
    if equal(a481, a482) {
        1u64
    } else {
        0u64
    }
}

pub fn f124(v0: Adt23) -> Option<List<Str>> {
    let m483 = f121(v0, Str::lit("files"));
    match m483 {
        None => {
            Some(List::<Str>::nil())
        }
        Some(v1) => {
            let m484 = f174(v1);
            match m484 {
                None => {
                    None::<List<Str>>
                }
                Some(v2) => {
                    let m485 = f175(v2, List::<Str>::nil());
                    match m485 {
                        None => {
                            None::<List<Str>>
                        }
                        Some(v3) => {
                            Some(f176(v3, List::<Str>::nil()))
                        }
                    }
                }
            }
        }
    }
}

pub fn f125(p488: List<Str>) -> R<bool> {
    let m487 = p488;
    {
        let mut m487 = m487;
        loop {
            let step = {
                let v0 = m487;
                let m486 = v0;
                match m486.uncons() {
                    None => {
                        Err::<List<Str>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        if f177(f80(v1)?) {
                            Err::<List<Str>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<Str>, R<bool>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m487 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f126(v0: List<Str>) -> R<List<u64>> {
    let m489 = v0.clone();
    match m489.uncons() {
        None => {
            let a490 = f17(0u64);
            let a491 = f86(Str::lit(""))?;
            Ok(append_list(a490, a491))
        }
        Some((_, _)) => {
            let a494 = f17(2u64);
            let a495 = f86({
                let a492 = v0;
                let a493 = Str::lit("\u{a}");
                join(a492, a493)
            })?;
            Ok(append_list(a494, a495))
        }
    }
}

pub fn f127(v0: Adt6) -> Option<bool> {
    let m496 = v0;
    match m496 {
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

pub fn f128(p501: Bytes, p502: List<Bytes>) -> Bytes {
    let m500 = (p501, p502);
    {
        let mut m500 = m500;
        loop {
            let step = {
                let (v0, v1) = m500;
                let m497 = v1;
                match m497.uncons() {
                    None => {
                        Err::<(Bytes, List<Bytes>), Bytes>(v0)
                    }
                    Some((v2, v3)) => {
                        Ok::<(Bytes, List<Bytes>), Bytes>(({
                            let a498 = v2;
                            let a499 = v0;
                            append_bytes(a498, a499)
                        }, v3))
                    }
                }
            };
            match step {
                Ok(next) => { m500 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f129(p505: List<Bytes>, p506: List<u64>) -> List<Bytes> {
    let m504 = (p505, p506);
    {
        let mut m504 = m504;
        loop {
            let step = {
                let (v0, v1) = m504;
                let m503 = v1;
                match m503.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<u64>), List<Bytes>>(v0)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<u64>), List<Bytes>>((f178(f179(v2), v0), v3))
                    }
                }
            };
            match step {
                Ok(next) => { m504 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f130(p510: List<Str>, p511: u64) -> Str {
    let m509 = (p510, p511);
    {
        let mut m509 = m509;
        loop {
            let step = {
                let (v0, v1) = m509;
                let m507 = v0;
                match m507.uncons() {
                    None => {
                        Err::<(List<Str>, u64), Str>(Str::lit("unknown"))
                    }
                    Some((v2, v3)) => {
                        let m508 = v1;
                        if m508 == 0 {
                            Err::<(List<Str>, u64), Str>(v2)
                        } else {
                            let v4 = m508 - 1;
                            Ok::<(List<Str>, u64), Str>((v3, v4))
                        }
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

pub fn f131(v0: Adt97) -> List<Str> {
    let m512 = v0;
    match m512 {
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

pub fn f132(v0: Str, v1: Adt6, v2: bool) -> Adt51 {
    Adt51::C0(Adt39::C0(List::cons(f180(v0), List::<Adt40>::nil()), Some(v1), if v2 {
        Some(true)
    } else {
        None::<bool>
    }), List::<Adt52>::nil())
}

pub fn f133(v0: Adt97) -> Str {
    let m513 = v0;
    match m513 {
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

pub fn f134() -> Adt51 {
    f181(Str::lit("the host answered with a malformed outcome"))
}

pub fn f135(v0: Bytes) -> Str {
    let m515 = {
        let a514 = v0;
        utf8_decode(a514)
    };
    match m515 {
        None => {
            Str::lit("the content is not UTF-8")
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f136(p518: u64, p519: i64) -> R<i64> {
    let m517 = (p518, p519);
    {
        let mut m517 = m517;
        loop {
            let step = {
                let (v0, v1) = m517;
                let m516 = v0;
                if m516 == 0 {
                    Err::<(u64, i64), R<i64>>(Ok(v1))
                } else {
                    let v2 = m516 - 1;
                    Ok::<(u64, i64), R<i64>>((v2, f182(v1, 1i64)?))
                }
            };
            match step {
                Ok(next) => { m517 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f137(v0: u64) -> Str {
    f130(List::cons(Str::lit("queued"), List::cons(Str::lit("running"), List::cons(Str::lit("verified"), List::cons(Str::lit("failed"), List::cons(Str::lit("cancelled"), List::<Str>::nil()))))), v0)
}

pub fn f138(p522: List<Adt15>, p523: List<Adt15>) -> List<Adt15> {
    let m521 = (p522, p523);
    {
        let mut m521 = m521;
        loop {
            let step = {
                let (v0, v1) = m521;
                let m520 = v0;
                match m520.uncons() {
                    None => {
                        Err::<(List<Adt15>, List<Adt15>), List<Adt15>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt15>, List<Adt15>), List<Adt15>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m521 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f139(v0: bool, v1: Bytes, v2: List<Adt15>, v3: u64) -> Result<Adt89, Adt90> {
    if v0 {
        Ok::<Adt89, Adt90>(Adt89::C0(v1, v2))
    } else {
        Err::<Adt89, Adt90>(Adt90::C1(v3))
    }
}

pub fn f140(v0: Adt92) -> List<u64> {
    let a534 = f17({
        let a525 = match v0.clone() {
            Adt92::C0(h524, _) => {
                h524
            }
        };
        length_list(a525)
    });
    let a535 = {
        let a532 = match v0.clone() {
            Adt92::C0(h526, _) => {
                h526
            }
        };
        let a533 = {
            let a530 = f17({
                let a528 = match v0.clone() {
                    Adt92::C0(_, h527) => {
                        h527
                    }
                };
                length_list(a528)
            });
            let a531 = match v0 {
                Adt92::C0(_, h529) => {
                    h529
                }
            };
            append_list(a530, a531)
        };
        append_list(a532, a533)
    };
    append_list(a534, a535)
}

pub fn f141(v0: List<u64>) -> List<u64> {
    let a537 = f17({
        let a536 = v0.clone();
        length_list(a536)
    });
    let a538 = v0;
    append_list(a537, a538)
}

pub fn f142(v0: u64, v1: Adt6) -> R<Adt95> {
    let m539 = f183(v1)?;
    match m539 {
        Ok(v3) => {
            Ok(Adt95::C0(v0, f63(), List::<List<u64>>::nil(), v3))
        }
        Err(_) => {
            Ok(f39())
        }
    }
}

pub fn f143(v0: Adt98) -> u64 {
    let m540 = v0;
    match m540 {
        Adt98::C0 => {
            f184()
        }
        Adt98::C1 => {
            f184()
        }
    }
}

pub fn f144(v0: Option<Adt104>, v1: Adt82) -> Adt6 {
    Adt6::C5(f185(v0, v1))
}

pub fn f145(v0: Adt98) -> Adt82 {
    Adt82::C0(f186(), f187(v0), None::<Adt6>)
}

pub fn f146(v0: List<u64>) -> bool {
    f188(v0, f189())
}

pub fn f147(v0: Adt0, v1: Fn2, v2: u64, v3: List<Adt92>, v4: Result<Adt6, Adt107>) -> R<Adt88> {
    let m541 = v4;
    match m541 {
        Ok(v6) => {
            f192(v0, v1, v2, v3, f193(v6)?)
        }
        Err(_) => {
            Ok(Adt88::C0(f38(f142(f190(), f144(None::<Adt104>, f191(Adt108::C0)))?)))
        }
    }
}

pub fn f148(v0: Adt0, v1: Bytes) -> Adt0 {
    Adt0::C0(match v0.clone() {
        Adt0::C0(h542, _, _, _, _, _, _) => {
            h542
        }
    }, match v0.clone() {
        Adt0::C0(_, h543, _, _, _, _, _) => {
            h543
        }
    }, match v0.clone() {
        Adt0::C0(_, _, h544, _, _, _, _) => {
            h544
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, h545, _, _, _) => {
            h545
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, h546, _, _) => {
            h546
        }
    }, match v0 {
        Adt0::C0(_, _, _, _, _, h547, _) => {
            h547
        }
    }, v1)
}

pub fn f149(v0: Bytes) -> R<Result<Adt6, Adt107>> {
    f194(f18(v0)?)
}

pub fn f150() -> Adt95 {
    Adt95::C0(f195(), List::cons(Adt92::C0(f196(), f197()), List::<Adt92>::nil()), List::<List<u64>>::nil(), List::<u64>::nil())
}

pub fn f151(v0: List<Adt92>, v1: List<u64>) -> R<Adt110> {
    let m548 = f198(v0, v1, Adt111::C0)?;
    match m548 {
        Adt111::C0 => {
            Ok(Adt110::C0)
        }
        Adt111::C1(v2) => {
            let m549 = f199(v2);
            match m549 {
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

pub fn f152() -> List<u64> {
    List::cons(79u64, List::cons(114u64, List::cons(105u64, List::cons(103u64, List::cons(105u64, List::cons(110u64, List::<u64>::nil()))))))
}

pub fn f153(v0: Option<List<u64>>, v1: List<List<u64>>, v2: Adt98) -> R<Option<Adt98>> {
    let m550 = v0;
    match m550 {
        None => {
            Ok(Some(v2))
        }
        Some(v3) => {
            if f200(v3, v1)? {
                Ok(None::<Adt98>)
            } else {
                Ok(Some(v2))
            }
        }
    }
}

pub fn f154(v0: List<u64>) -> Option<List<u64>> {
    let m551 = f201(v0);
    match m551 {
        None => {
            None::<List<u64>>
        }
        Some(v1) => {
            f156(v1)
        }
    }
}

pub fn f155() -> List<u64> {
    List::cons(72u64, List::cons(111u64, List::cons(115u64, List::cons(116u64, List::<u64>::nil()))))
}

pub fn f156(v0: List<u64>) -> Option<List<u64>> {
    let m552 = v0.clone();
    match m552.uncons() {
        None => {
            None::<List<u64>>
        }
        Some((v1, v2)) => {
            let a553 = v1;
            let a554 = 91u64;
            if nat_eq(a553, a554) {
                f202(f203(v2, List::cons(v1, List::<u64>::nil())))
            } else {
                f202(f204(v0, List::<u64>::nil()))
            }
        }
    }
}

pub fn f157(p557: List<Adt92>, p558: List<Adt92>) -> List<Adt92> {
    let m556 = (p557, p558);
    {
        let mut m556 = m556;
        loop {
            let step = {
                let (v0, v1) = m556;
                let m555 = v0;
                match m555.uncons() {
                    None => {
                        Err::<(List<Adt92>, List<Adt92>), List<Adt92>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt92>, List<Adt92>), List<Adt92>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m556 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f158(v0: Bytes, v1: u64) -> R<Option<Adt94>> {
    let a560 = v1;
    let a561 = {
        let a559 = v0.clone();
        length_bytes(a559)
    };
    if nat_eq(a560, a561) {
        Ok(Some(Adt94::C0(Bytes::lit(&[]), v1)))
    } else {
        f37(v0, v1)
    }
}

pub fn f159(v0: Bytes, v1: Adt91, v2: u64) -> R<Result<Adt91, Adt90>> {
    let a563 = v2;
    let a564 = {
        let a562 = v0.clone();
        length_bytes(a562)
    };
    if nat_eq(a563, a564) {
        Ok(Ok::<Adt91, Adt90>(v1))
    } else {
        let m565 = f59(v0.clone(), v2)?;
        match m565 {
            None => {
                Ok(Err::<Adt91, Adt90>(Adt90::C0(v2)))
            }
            Some(v3) => {
                let m568 = f205(v0.clone(), {
                    let a566 = v2;
                    let a567 = 4u64;
                    nat_add(a566, a567)?
                }, v3, List::<List<u64>>::nil())?;
                match m568 {
                    Ok(v5) => {
                        let a571 = {
                            let (_, h569) = v5.clone();
                            h569
                        };
                        let a572 = {
                            let a570 = v0;
                            length_bytes(a570)
                        };
                        if nat_eq(a571, a572) {
                            Ok(Ok::<Adt91, Adt90>(f206(v1, {
                                let (h573, _) = v5;
                                h573
                            })))
                        } else {
                            Ok(Err::<Adt91, Adt90>(Adt90::C1({
                                let (_, h574) = v5;
                                h574
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

pub fn f160(v0: List<u64>) -> bool {
    let m575 = v0.clone();
    match m575.uncons() {
        None => {
            true
        }
        Some((_, _)) => {
            let a579 = if f207(v0.clone(), false) {
                let a577 = {
                    let a576 = v0;
                    length_list(a576)
                };
                let a578 = f208();
                nat_le(a577, a578)
            } else {
                false
            };
            bool_not(a579)
        }
    }
}

pub fn f161(v0: Adt37) -> Option<Adt93> {
    let m584 = f209(match v0.clone() {
        Adt37::C0(h580, _, _, _, _, _, _, _) => {
            h580
        }
    }, {
        let m583 = match v0.clone() {
            Adt37::C0(_, _, _, _, h582, _, _, _) => {
                h582
            }
        };
        match m583 {
            Adt5::C0(h581) => {
                h581
            }
        }
    });
    match m584 {
        None => {
            f210(match v0.clone() {
                Adt37::C0(h585, _, _, _, _, _, _, _) => {
                    h585
                }
            }, {
                let m588 = match v0 {
                    Adt37::C0(_, _, _, _, h587, _, _, _) => {
                        h587
                    }
                };
                match m588 {
                    Adt5::C0(h586) => {
                        h586
                    }
                }
            })
        }
        Some(v1) => {
            Some(v1)
        }
    }
}

pub fn f162(v0: Adt37, v1: Result<Adt112, Adt93>) -> R<Result<List<Adt12>, Adt93>> {
    let m589 = v1;
    match m589 {
        Ok(v3) => {
            let m590 = f211(v0)?;
            match m590 {
                None => {
                    Ok(Ok::<List<Adt12>, Adt93>(f69(match v3 {
                        Adt112::C0(h591, _) => {
                            h591
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

pub fn f163(p598: u64, v1: Str, v2: Adt6, p599: List<Adt105>, p600: Adt112) -> R<Result<Adt112, Adt93>> {
    let m597 = (p598, (p599, p600));
    {
        let mut m597 = m597;
        loop {
            let step = {
                let (v0, (v3, v4)) = m597;
                let m592 = v0;
                if m592 == 0 {
                    Err::<(u64, (List<Adt105>, Adt112)), R<Result<Adt112, Adt93>>>(Ok(Err::<Adt112, Adt93>(Adt93::C5(v1.clone(), Str::lit("budget")))))
                } else {
                    let v5 = m592 - 1;
                    let m593 = v3;
                    match m593.uncons() {
                        None => {
                            Err::<(u64, (List<Adt105>, Adt112)), R<Result<Adt112, Adt93>>>(Ok(Ok::<Adt112, Adt93>(v4)))
                        }
                        Some((v6, v7)) => {
                            let m594 = f212(v1.clone(), v2.clone(), v6, v4, v7)?;
                            match m594 {
                                Ok(v9) => {
                                    Ok::<(u64, (List<Adt105>, Adt112)), R<Result<Adt112, Adt93>>>((v5, ({
                                        let (h595, _) = v9;
                                        h595
                                    }, {
                                        let (_, h596) = v9;
                                        h596
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
                Ok(next) => { m597 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f164() -> u64 {
    67108864u64
}

pub fn f165() -> Adt112 {
    Adt112::C0(List::<Adt12>::nil(), List::<List<u64>>::nil())
}

pub fn f166(v0: Adt6, v1: Adt6, v2: Adt6, v3: List<Adt100>) -> R<Adt113> {
    let m601 = f127(v1.clone());
    match m601 {
        None => {
            let m602 = f213(v1);
            match m602 {
                None => {
                    Ok(Adt113::C1(Str::lit("schema")))
                }
                Some(v5) => {
                    f214(v0, match v5 {
                        Adt5::C0(h603) => {
                            h603
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

pub fn f167(v0: Adt102, v1: List<Adt102>) -> R<Adt106> {
    let m604 = v0;
    match m604 {
        Adt102::C0(_, _, _) => {
            Ok(Adt106::C0(Adt101::C0, v1))
        }
        Adt102::C1(_, _, _) => {
            Ok(Adt106::C0(Adt101::C0, v1))
        }
        Adt102::C2(v8, v9, v10, v11) => {
            Ok(f215(v8, v9, v10, {
                let a605 = v11;
                let a606 = 1u64;
                nat_add(a605, a606)?
            }, v1))
        }
        Adt102::C3(v12) => {
            Ok(Adt106::C0(Adt101::C1(v12, Str::lit("not")), v1))
        }
        Adt102::C4(v13, _, v15, v16) => {
            Ok(f216(v13, v15, v16, v1))
        }
        Adt102::C5(_, _, _, _) => {
            Ok(Adt106::C0(Adt101::C0, v1))
        }
    }
}

pub fn f168(v0: Adt103, v1: Adt6, v2: Adt6, v3: List<Adt100>) -> Adt113 {
    let m607 = f213(v1);
    match m607 {
        None => {
            Adt113::C1(Str::lit("schema"))
        }
        Some(v4) => {
            let m608 = v0;
            match m608 {
                Adt103::C0 => {
                    f217(match v4 {
                        Adt5::C0(h609) => {
                            h609
                        }
                    }, v2, v3)
                }
                Adt103::C1 => {
                    f218(match v4 {
                        Adt5::C0(h610) => {
                            h610
                        }
                    }, v2, v3)
                }
                Adt103::C2 => {
                    f219(match v4 {
                        Adt5::C0(h611) => {
                            h611
                        }
                    }, v2, v3)
                }
                Adt103::C3 => {
                    f220(match v4 {
                        Adt5::C0(h612) => {
                            h612
                        }
                    }, v2, v3)
                }
            }
        }
    }
}

pub fn f169(v0: Adt102, v1: List<Adt102>, v2: List<Adt100>, v3: Str) -> Adt106 {
    let m613 = v0;
    match m613 {
        Adt102::C0(_, _, _) => {
            Adt106::C0(Adt101::C1(v2, v3), v1)
        }
        Adt102::C1(v7, v8, v9) => {
            f221(v7, v8, v9, v1)
        }
        Adt102::C2(v10, v11, v12, v13) => {
            f215(v10, v11, v12, v13, v1)
        }
        Adt102::C3(_) => {
            Adt106::C0(Adt101::C0, v1)
        }
        Adt102::C4(_, v16, v17, v18) => {
            f216(v16, v17, v18, v1)
        }
        Adt102::C5(_, _, _, _) => {
            Adt106::C0(Adt101::C1(v2, v3), v1)
        }
    }
}

pub fn f170(p616: List<Adt100>, p617: List<Str>) -> R<List<Str>> {
    let m615 = (p616, p617);
    {
        let mut m615 = m615;
        loop {
            let step = {
                let (v0, v1) = m615;
                let m614 = v0;
                match m614.uncons() {
                    None => {
                        Err::<(List<Adt100>, List<Str>), R<List<Str>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt100>, List<Str>), R<List<Str>>>((v3, List::cons(f222(v2)?, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m615 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f171(p626: List<Adt40>) -> R<bool> {
    let m625 = p626;
    {
        let mut m625 = m625;
        loop {
            let step = {
                let v0 = m625;
                let m618 = v0;
                match m618.uncons() {
                    None => {
                        Err::<List<Adt40>, R<bool>>(Ok(true))
                    }
                    Some((v1, v2)) => {
                        let m619 = v1;
                        match m619 {
                            Adt40::C0(_) => {
                                Ok::<List<Adt40>, R<bool>>(v2)
                            }
                            Adt40::C1(v4) => {
                                if f223(match v4 {
                                    Adt44::C0(h620, _, _, _) => {
                                        h620
                                    }
                                })? {
                                    Ok::<List<Adt40>, R<bool>>(v2)
                                } else {
                                    Err::<List<Adt40>, R<bool>>(Ok(false))
                                }
                            }
                            Adt40::C2(v5) => {
                                if f223(match v5 {
                                    Adt45::C0(h621, _, _, _) => {
                                        h621
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
                                let m623 = match v7 {
                                    Adt47::C0(h622, _, _) => {
                                        h622
                                    }
                                };
                                match m623 {
                                    Adt48::C0(_) => {
                                        Ok::<List<Adt40>, R<bool>>(v2)
                                    }
                                    Adt48::C1(v9) => {
                                        if f223(match v9 {
                                            Adt50::C0(_, _, h624, _) => {
                                                h624
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
                Ok(next) => { m625 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f172(v0: Adt37, v1: Adt51) -> R<Result<Adt34, Adt86>> {
    if f224(match v1.clone() {
        Adt51::C0(h627, _) => {
            h627
        }
    }) {
        f225(v1)
    } else {
        let m629 = match v0 {
            Adt37::C0(_, _, _, _, _, h628, _, _) => {
                h628
            }
        };
        match m629 {
            None => {
                f225(v1)
            }
            Some(v2) => {
                f226(v2, v1)
            }
        }
    }
}

pub fn f173(v0: Str, v1: Adt5) -> Adt114 {
    f227(v0, match v1 {
        Adt5::C0(h630) => {
            h630
        }
    })
}

pub fn f174(v0: Adt6) -> Option<List<Adt6>> {
    let m631 = v0;
    match m631 {
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

pub fn f175(p635: List<Adt6>, p636: List<Str>) -> Option<List<Str>> {
    let m634 = (p635, p636);
    {
        let mut m634 = m634;
        loop {
            let step = {
                let (v0, v1) = m634;
                let m632 = v0;
                match m632.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<Str>), Option<List<Str>>>(Some(v1))
                    }
                    Some((v2, v3)) => {
                        let m633 = f122(v2);
                        match m633 {
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
                Ok(next) => { m634 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f176(p639: List<Str>, p640: List<Str>) -> List<Str> {
    let m638 = (p639, p640);
    {
        let mut m638 = m638;
        loop {
            let step = {
                let (v0, v1) = m638;
                let m637 = v0;
                match m637.uncons() {
                    None => {
                        Err::<(List<Str>, List<Str>), List<Str>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Str>, List<Str>), List<Str>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m638 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f177(p645: List<u64>) -> bool {
    let m644 = p645;
    {
        let mut m644 = m644;
        loop {
            let step = {
                let v0 = m644;
                let m641 = v0;
                match m641.uncons() {
                    None => {
                        Err::<List<u64>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        let a642 = v1;
                        let a643 = 10u64;
                        if nat_eq(a642, a643) {
                            Err::<List<u64>, bool>(true)
                        } else {
                            Ok::<List<u64>, bool>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m644 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f178(p654: Bytes, p655: List<Bytes>) -> List<Bytes> {
    let m653 = (p654, p655);
    {
        let mut m653 = m653;
        loop {
            let step = {
                let (v0, v1) = m653;
                let m646 = v1;
                match m646.uncons() {
                    None => {
                        Err::<(Bytes, List<Bytes>), List<Bytes>>(List::cons(v0, List::<Bytes>::nil()))
                    }
                    Some((v2, v3)) => {
                        let a649 = {
                            let a647 = v2.clone();
                            length_bytes(a647)
                        };
                        let a650 = {
                            let a648 = v0.clone();
                            length_bytes(a648)
                        };
                        if nat_eq(a649, a650) {
                            Ok::<(Bytes, List<Bytes>), List<Bytes>>(({
                                let a651 = v2;
                                let a652 = v0;
                                append_bytes(a651, a652)
                            }, v3))
                        } else {
                            Err::<(Bytes, List<Bytes>), List<Bytes>>(List::cons(v0, List::cons(v2, v3)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m653 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f179(v0: u64) -> Bytes {
    let m659 = {
        let a656 = f228();
        let a657 = v0;
        let a658 = 1u64;
        slice_bytes(a656, a657, a658)
    };
    match m659 {
        None => {
            f88()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f180(v0: Str) -> Adt40 {
    Adt40::C0(Adt41::C0(v0, None::<Adt42>, None::<Adt5>))
}

pub fn f181(v0: Str) -> Adt51 {
    Adt51::C0(Adt39::C0(List::cons(f180(v0), List::<Adt40>::nil()), None::<Adt6>, None::<bool>), List::<Adt52>::nil())
}

pub fn f182(v0: i64, v1: i64) -> R<i64> {
    let a661 = v0;
    let a662 = {
        let a660 = v1;
        int_neg(a660)?
    };
    int_sub(a661, a662)
}

pub fn f183(v0: Adt6) -> R<Result<List<u64>, Adt107>> {
    f229(f230(), v0)
}

pub fn f184() -> u64 {
    403u64
}

pub fn f185(v0: Option<Adt104>, v1: Adt82) -> List<(Str, Adt6)> {
    List::cons((Str::lit("jsonrpc"), Adt6::C3(f231())), {
        let a663 = f232(v0);
        let a664 = List::cons((Str::lit("error"), f233(v1)), List::<(Str, Adt6)>::nil());
        append_list(a663, a664)
    })
}

pub fn f186() -> i64 {
    -32000i64
}

pub fn f187(v0: Adt98) -> Str {
    let m665 = v0;
    match m665 {
        Adt98::C0 => {
            Str::lit("Forbidden: invalid Origin")
        }
        Adt98::C1 => {
            Str::lit("Forbidden: invalid Host")
        }
    }
}

pub fn f188(p672: List<u64>, p673: List<u64>) -> bool {
    let m671 = (p672, p673);
    {
        let mut m671 = m671;
        loop {
            let step = {
                let (v0, v1) = m671;
                let m666 = v0;
                match m666.uncons() {
                    None => {
                        let m667 = v1;
                        match m667.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m668 = v1;
                        match m668.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                            Some((v6, v7)) => {
                                let a669 = v4;
                                let a670 = v6;
                                if nat_eq(a669, a670) {
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
                Ok(next) => { m671 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f189() -> List<u64> {
    List::cons(80u64, List::cons(79u64, List::cons(83u64, List::cons(84u64, List::<u64>::nil()))))
}

pub fn f190() -> u64 {
    400u64
}

pub fn f191(v0: Adt108) -> Adt82 {
    Adt82::C0(f234(v0.clone()), f235(v0), None::<Adt6>)
}

pub fn f192(v0: Adt0, v1: Fn2, v2: u64, v3: List<Adt92>, v4: Result<Adt115, Adt108>) -> R<Adt88> {
    let m674 = v4;
    match m674 {
        Ok(v6) => {
            f238(v0, v1, v2, v3, v6)
        }
        Err(v5) => {
            Ok(Adt88::C0(f38(f142(f236(v5.clone()), f144(f237(v5.clone()), f191(v5)))?)))
        }
    }
}

pub fn f193(v0: Adt6) -> R<Result<Adt115, Adt108>> {
    let m675 = v0;
    match m675 {
        Adt6::C0 => {
            Ok(f240(None::<Adt104>, Adt109::C0))
        }
        Adt6::C1(_) => {
            Ok(f240(None::<Adt104>, Adt109::C0))
        }
        Adt6::C2(_) => {
            Ok(f240(None::<Adt104>, Adt109::C0))
        }
        Adt6::C3(_) => {
            Ok(f240(None::<Adt104>, Adt109::C0))
        }
        Adt6::C4(_) => {
            Ok(f240(None::<Adt104>, Adt109::C1))
        }
        Adt6::C5(v1) => {
            f239(Adt5::C0(v1))
        }
    }
}

pub fn f194(v0: List<u64>) -> R<Result<Adt6, Adt107>> {
    let m676 = f241(v0.clone())?;
    match m676 {
        Adt122::C0 => {
            f242(f243(), 0u64, v0)
        }
        Adt122::C1(v1) => {
            Ok(Err::<Adt6, Adt107>(Adt107::C0(v1)))
        }
        Adt122::C2(v2) => {
            Ok(Err::<Adt6, Adt107>(Adt107::C1(v2)))
        }
    }
}

pub fn f195() -> u64 {
    405u64
}

pub fn f196() -> List<u64> {
    List::cons(65u64, List::cons(108u64, List::cons(108u64, List::cons(111u64, List::cons(119u64, List::<u64>::nil())))))
}

pub fn f197() -> List<u64> {
    List::cons(80u64, List::cons(79u64, List::cons(83u64, List::cons(84u64, List::<u64>::nil()))))
}

pub fn f198(p681: List<Adt92>, v1: List<u64>, p682: Adt111) -> R<Adt111> {
    let m680 = (p681, p682);
    {
        let mut m680 = m680;
        loop {
            let step = {
                let (v0, v2) = m680;
                let m677 = v0;
                match m677.uncons() {
                    None => {
                        Err::<(List<Adt92>, Adt111), R<Adt111>>(Ok(v2))
                    }
                    Some((v3, v4)) => {
                        if f244(match v3.clone() {
                            Adt92::C0(h678, _) => {
                                h678
                            }
                        }, v1.clone())? {
                            Ok::<(List<Adt92>, Adt111), R<Adt111>>((v4, f245(v2, match v3 {
                                Adt92::C0(_, h679) => {
                                    h679
                                }
                            })))
                        } else {
                            Ok::<(List<Adt92>, Adt111), R<Adt111>>((v4, v2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m680 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f199(v0: List<u64>) -> Option<List<u64>> {
    let v1: List<u64> = f246(v0);
    if f247(v1.clone()) {
        Some(v1)
    } else {
        None::<List<u64>>
    }
}

pub fn f200(v0: List<u64>, v1: List<List<u64>>) -> R<bool> {
    if f244(v0.clone(), f248())? {
        Ok(true)
    } else {
        if f244(v0.clone(), f249())? {
            Ok(true)
        } else {
            if f244(v0.clone(), f250())? {
                Ok(true)
            } else {
                f251(v0, v1)
            }
        }
    }
}

pub fn f201(p687: List<u64>) -> Option<List<u64>> {
    let m686 = p687;
    {
        let mut m686 = m686;
        loop {
            let step = {
                let v0 = m686;
                let m683 = v0;
                match m683.uncons() {
                    None => {
                        Err::<List<u64>, Option<List<u64>>>(None::<List<u64>>)
                    }
                    Some((v1, v2)) => {
                        let a684 = v1;
                        let a685 = 58u64;
                        if nat_eq(a684, a685) {
                            Err::<List<u64>, Option<List<u64>>>(f252(v2))
                        } else {
                            Ok::<List<u64>, Option<List<u64>>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m686 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f202(v0: Option<(List<u64>, List<u64>)>) -> Option<List<u64>> {
    let m688 = v0;
    match m688 {
        None => {
            None::<List<u64>>
        }
        Some(v1) => {
            if f253({
                let (_, h689) = v1;
                h689
            }) {
                Some({
                    let (h690, _) = v1;
                    h690
                })
            } else {
                None::<List<u64>>
            }
        }
    }
}

pub fn f203(p695: List<u64>, p696: List<u64>) -> Option<(List<u64>, List<u64>)> {
    let m694 = (p695, p696);
    {
        let mut m694 = m694;
        loop {
            let step = {
                let (v0, v1) = m694;
                let m691 = v0;
                match m691.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>(None::<(List<u64>, List<u64>)>)
                    }
                    Some((v2, v3)) => {
                        let a692 = v2;
                        let a693 = 93u64;
                        if nat_eq(a692, a693) {
                            Err::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>(Some((f254(List::cons(v2, v1), List::<u64>::nil()), v3)))
                        } else {
                            Ok::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>((v3, List::cons(v2, v1)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m694 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f204(p701: List<u64>, p702: List<u64>) -> Option<(List<u64>, List<u64>)> {
    let m700 = (p701, p702);
    {
        let mut m700 = m700;
        loop {
            let step = {
                let (v0, v1) = m700;
                let m697 = v0;
                match m697.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>(Some((f254(v1, List::<u64>::nil()), List::<u64>::nil())))
                    }
                    Some((v2, v3)) => {
                        let a698 = v2;
                        let a699 = 58u64;
                        if nat_eq(a698, a699) {
                            Err::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>(Some((f254(v1, List::<u64>::nil()), List::cons(v2, v3))))
                        } else {
                            Ok::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>((v3, List::cons(v2, v1)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m700 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f205(v0: Bytes, p708: u64, p709: u64, p710: List<List<u64>>) -> R<Result<(List<List<u64>>, u64), Adt90>> {
    let m707 = (p708, (p709, p710));
    {
        let mut m707 = m707;
        loop {
            let step = {
                let (v1, (v2, v3)) = m707;
                let m703 = v2;
                if m703 == 0 {
                    Err::<(u64, (u64, List<List<u64>>)), R<Result<(List<List<u64>>, u64), Adt90>>>(Ok(Ok::<(List<List<u64>>, u64), Adt90>((f101(v3, List::<List<u64>>::nil()), v1))))
                } else {
                    let v4 = m703 - 1;
                    let m704 = f37(v0.clone(), v1)?;
                    match m704 {
                        None => {
                            Err::<(u64, (u64, List<List<u64>>)), R<Result<(List<List<u64>>, u64), Adt90>>>(Ok(Err::<(List<List<u64>>, u64), Adt90>(Adt90::C0(v1))))
                        }
                        Some(v5) => {
                            Ok::<(u64, (u64, List<List<u64>>)), R<Result<(List<List<u64>>, u64), Adt90>>>((match v5.clone() {
                                Adt94::C0(_, h705) => {
                                    h705
                                }
                            }, (v4, List::cons(f18(match v5 {
                                Adt94::C0(h706, _) => {
                                    h706
                                }
                            })?, v3))))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m707 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f206(v0: Adt91, v1: List<List<u64>>) -> Adt91 {
    Adt91::C0(match v0.clone() {
        Adt91::C0(h711, _, _, _, _, _, _) => {
            h711
        }
    }, match v0.clone() {
        Adt91::C0(_, h712, _, _, _, _, _) => {
            h712
        }
    }, match v0.clone() {
        Adt91::C0(_, _, h713, _, _, _, _) => {
            h713
        }
    }, match v0.clone() {
        Adt91::C0(_, _, _, h714, _, _, _) => {
            h714
        }
    }, match v0.clone() {
        Adt91::C0(_, _, _, _, h715, _, _) => {
            h715
        }
    }, v1, match v0 {
        Adt91::C0(_, _, _, _, _, _, h716) => {
            h716
        }
    })
}

pub fn f207(p719: List<u64>, v1: bool) -> bool {
    let m718 = p719;
    {
        let mut m718 = m718;
        loop {
            let step = {
                let v0 = m718;
                let m717 = v0;
                match m717.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        if if v1 {
                            f255(v2)
                        } else {
                            f256(v2)
                        } {
                            Ok::<List<u64>, bool>(v3)
                        } else {
                            Err::<List<u64>, bool>(false)
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

pub fn f208() -> u64 {
    64u64
}

pub fn f209(v0: Str, v1: List<(Str, Adt6)>) -> Option<Adt93> {
    let m720 = f227(Str::lit("type"), v1);
    match m720 {
        Adt114::C0 => {
            Some(Adt93::C3(v0))
        }
        Adt114::C1(v2) => {
            let m721 = f122(v2);
            match m721 {
                None => {
                    Some(Adt93::C3(v0))
                }
                Some(v3) => {
                    let a722 = v3;
                    let a723 = Str::lit("object");
                    if equal(a722, a723) {
                        None::<Adt93>
                    } else {
                        Some(Adt93::C3(v0))
                    }
                }
            }
        }
    }
}

pub fn f210(v0: Str, v1: List<(Str, Adt6)>) -> Option<Adt93> {
    let m724 = f227(Str::lit("$schema"), v1);
    match m724 {
        Adt114::C0 => {
            None::<Adt93>
        }
        Adt114::C1(v2) => {
            let m725 = f122(v2);
            match m725 {
                None => {
                    Some(Adt93::C2(v0))
                }
                Some(v3) => {
                    let a726 = v3;
                    let a727 = f257();
                    if equal(a726, a727) {
                        None::<Adt93>
                    } else {
                        Some(Adt93::C2(v0))
                    }
                }
            }
        }
    }
}

pub fn f211(v0: Adt37) -> R<Option<Adt93>> {
    let m729 = match v0.clone() {
        Adt37::C0(_, _, _, _, _, h728, _, _) => {
            h728
        }
    };
    match m729 {
        None => {
            Ok(None::<Adt93>)
        }
        Some(v1) => {
            let m732 = f210(match v0.clone() {
                Adt37::C0(h730, _, _, _, _, _, _, _) => {
                    h730
                }
            }, match v1.clone() {
                Adt5::C0(h731) => {
                    h731
                }
            });
            match m732 {
                None => {
                    let m736 = f163(f164(), match v0 {
                        Adt37::C0(h733, _, _, _, _, _, _, _) => {
                            h733
                        }
                    }, Adt6::C5(match v1.clone() {
                        Adt5::C0(h734) => {
                            h734
                        }
                    }), List::cons(Adt105::C0(Adt6::C5(match v1 {
                        Adt5::C0(h735) => {
                            h735
                        }
                    }), false, List::<Str>::nil()), List::<Adt105>::nil()), f165())?;
                    match m736 {
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

pub fn f212(v0: Str, v1: Adt6, v2: Adt105, v3: Adt112, v4: List<Adt105>) -> R<Result<(List<Adt105>, Adt112), Adt93>> {
    let m737 = v2;
    match m737 {
        Adt105::C0(v5, v6, v7) => {
            let m738 = f127(v5.clone());
            match m738 {
                None => {
                    let m739 = f213(v5);
                    match m739 {
                        None => {
                            Ok(Err::<(List<Adt105>, Adt112), Adt93>(Adt93::C5(v0, Str::lit("schema"))))
                        }
                        Some(v9) => {
                            f258(v0, v1, match v9 {
                                Adt5::C0(h740) => {
                                    h740
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

pub fn f213(v0: Adt6) -> Option<Adt5> {
    let m741 = v0;
    match m741 {
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

pub fn f214(v0: Adt6, v1: List<(Str, Adt6)>, v2: Adt6, v3: List<Adt100>) -> R<Adt113> {
    let m742 = f259(v1.clone(), v2.clone())?;
    match m742 {
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
            f260(v0, v1, v2, v3)
        }
    }
}

pub fn f215(v0: List<Adt6>, v1: Adt6, v2: List<Adt100>, v3: u64, v4: List<Adt102>) -> Adt106 {
    let m743 = v0;
    match m743.uncons() {
        None => {
            let a744 = v3;
            let a745 = 1u64;
            if nat_eq(a744, a745) {
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

pub fn f216(v0: Option<Adt6>, v1: Adt6, v2: List<Adt100>, v3: List<Adt102>) -> Adt106 {
    let m746 = v0;
    match m746 {
        None => {
            Adt106::C0(Adt101::C0, v3)
        }
        Some(v4) => {
            Adt106::C0(Adt101::C0, List::cons(Adt102::C0(v4, v1, v2), v3))
        }
    }
}

pub fn f217(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt100>) -> Adt113 {
    let m747 = f227(Str::lit("anyOf"), v0);
    match m747 {
        Adt114::C0 => {
            Adt113::C3(List::<Adt102>::nil())
        }
        Adt114::C1(v3) => {
            let m748 = f174(v3);
            match m748 {
                None => {
                    Adt113::C1(Str::lit("anyOf"))
                }
                Some(v4) => {
                    let m749 = v4;
                    match m749.uncons() {
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

pub fn f218(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt100>) -> Adt113 {
    let m750 = f227(Str::lit("oneOf"), v0);
    match m750 {
        Adt114::C0 => {
            Adt113::C3(List::<Adt102>::nil())
        }
        Adt114::C1(v3) => {
            let m751 = f174(v3);
            match m751 {
                None => {
                    Adt113::C1(Str::lit("oneOf"))
                }
                Some(v4) => {
                    let m752 = v4;
                    match m752.uncons() {
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

pub fn f219(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt100>) -> Adt113 {
    let m753 = f227(Str::lit("not"), v0);
    match m753 {
        Adt114::C0 => {
            Adt113::C3(List::<Adt102>::nil())
        }
        Adt114::C1(v3) => {
            Adt113::C3(List::cons(Adt102::C0(v3, v1, v2.clone()), List::cons(Adt102::C3(v2), List::<Adt102>::nil())))
        }
    }
}

pub fn f220(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt100>) -> Adt113 {
    let m754 = f227(Str::lit("if"), v0.clone());
    match m754 {
        Adt114::C0 => {
            Adt113::C3(List::<Adt102>::nil())
        }
        Adt114::C1(v3) => {
            Adt113::C3(List::cons(Adt102::C0(v3, v1.clone(), v2.clone()), List::cons(Adt102::C4(f261(Str::lit("then"), v0.clone()), f261(Str::lit("else"), v0), v1, v2), List::<Adt102>::nil())))
        }
    }
}

pub fn f221(v0: List<Adt6>, v1: Adt6, v2: List<Adt100>, v3: List<Adt102>) -> Adt106 {
    let m755 = v0;
    match m755.uncons() {
        None => {
            Adt106::C0(Adt101::C1(v2, Str::lit("anyOf")), v3)
        }
        Some((v4, v5)) => {
            Adt106::C0(Adt101::C0, List::cons(Adt102::C0(v4, v1.clone(), v2.clone()), List::cons(Adt102::C1(v5, v1, v2), v3)))
        }
    }
}

pub fn f222(v0: Adt100) -> R<Str> {
    let m756 = v0;
    match m756 {
        Adt100::C0(v1) => {
            f262(v1)
        }
        Adt100::C1(v2) => {
            let a757 = f136(v2, 0i64)?;
            Ok(format_i64(a757))
        }
    }
}

pub fn f223(v0: Str) -> R<bool> {
    let m759 = f263(f18({
        let a758 = v0;
        utf8_encode(a758)
    })?)?;
    match m759 {
        None => {
            Ok(false)
        }
        Some(_) => {
            Ok(true)
        }
    }
}

pub fn f224(v0: Adt39) -> bool {
    let m761 = match v0 {
        Adt39::C0(_, _, h760) => {
            h760
        }
    };
    match m761 {
        None => {
            false
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f225(v0: Adt51) -> R<Result<Adt34, Adt86>> {
    let m763 = match v0.clone() {
        Adt51::C0(_, h762) => {
            h762
        }
    };
    match m763.uncons() {
        None => {
            Ok(Ok::<Adt34, Adt86>(Adt34::C2(match v0 {
                Adt51::C0(h764, _) => {
                    h764
                }
            })))
        }
        Some((_, _)) => {
            if f264(match v0.clone() {
                Adt51::C0(_, h765) => {
                    h765
                }
            })? {
                Ok(Ok::<Adt34, Adt86>(Adt34::C3(v0)))
            } else {
                Ok(Err::<Adt34, Adt86>(Adt86::C6(Str::lit("progress did not increase"))))
            }
        }
    }
}

pub fn f226(v0: Adt5, v1: Adt51) -> R<Result<Adt34, Adt86>> {
    let m769 = {
        let m768 = match v1.clone() {
            Adt51::C0(h767, _) => {
                h767
            }
        };
        match m768 {
            Adt39::C0(_, h766, _) => {
                h766
            }
        }
    };
    match m769 {
        None => {
            Ok(Err::<Adt34, Adt86>(Adt86::C6(Str::lit("the tool declares an outputSchema and returned no structuredContent"))))
        }
        Some(v2) => {
            let m772 = f72(Adt6::C5(match v0.clone() {
                Adt5::C0(h770) => {
                    h770
                }
            }), Adt6::C5(match v0 {
                Adt5::C0(h771) => {
                    h771
                }
            }), v2)?;
            match m772 {
                Adt99::C0 => {
                    f225(v1)
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

pub fn f227(v0: Str, p778: List<(Str, Adt6)>) -> Adt114 {
    let m777 = p778;
    {
        let mut m777 = m777;
        loop {
            let step = {
                let v1 = m777;
                let m773 = v1;
                match m773.uncons() {
                    None => {
                        Err::<List<(Str, Adt6)>, Adt114>(Adt114::C0)
                    }
                    Some((v2, v3)) => {
                        let m775 = f265({
                            let (h774, _) = v2;
                            h774
                        }, v0.clone());
                        match m775 {
                            Adt131::C0 => {
                                Err::<List<(Str, Adt6)>, Adt114>(Adt114::C1({
                                    let (_, h776) = v2;
                                    h776
                                }))
                            }
                            Adt131::C1 => {
                                Ok::<List<(Str, Adt6)>, Adt114>(v3)
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m777 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f228() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f229(v0: u64, v1: Adt6) -> R<Result<List<u64>, Adt107>> {
    let m779 = f266(v0, List::cons(Adt116::C1(0u64, v1), List::<Adt116>::nil()), List::<u64>::nil())?;
    match m779 {
        Ok(v2) => {
            Ok(Ok::<List<u64>, Adt107>(f267(v2, List::<u64>::nil())))
        }
        Err(v3) => {
            Ok(Err::<List<u64>, Adt107>(v3))
        }
    }
}

pub fn f230() -> u64 {
    67108864u64
}

pub fn f231() -> Str {
    Str::lit("2.0")
}

pub fn f232(v0: Option<Adt104>) -> List<(Str, Adt6)> {
    let m780 = v0;
    match m780 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("id"), f268(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f233(v0: Adt82) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("code"), Adt6::C2(Adt7::C0(match v0.clone() {
        Adt82::C0(h781, _, _) => {
            h781
        }
    }, 0i64))), List::cons((Str::lit("message"), Adt6::C3(match v0.clone() {
        Adt82::C0(_, h782, _) => {
            h782
        }
    })), f269(match v0 {
        Adt82::C0(_, _, h783) => {
            h783
        }
    }))))
}

pub fn f234(v0: Adt108) -> i64 {
    let m784 = v0;
    match m784 {
        Adt108::C0 => {
            f270()
        }
        Adt108::C1(_, _) => {
            f271()
        }
    }
}

pub fn f235(v0: Adt108) -> Str {
    let m785 = v0;
    match m785 {
        Adt108::C0 => {
            Str::lit("Parse error")
        }
        Adt108::C1(_, v2) => {
            f272(v2)
        }
    }
}

pub fn f236(v0: Adt108) -> u64 {
    let m786 = v0;
    match m786 {
        Adt108::C0 => {
            f190()
        }
        Adt108::C1(_, _) => {
            f190()
        }
    }
}

pub fn f237(v0: Adt108) -> Option<Adt104> {
    let m787 = v0;
    match m787 {
        Adt108::C0 => {
            None::<Adt104>
        }
        Adt108::C1(v1, _) => {
            v1
        }
    }
}

pub fn f238(v0: Adt0, v1: Fn2, v2: u64, v3: List<Adt92>, v4: Adt115) -> R<Adt88> {
    f273(f274(v3.clone())?, f275(v0.clone(), v1, v2, Some(f276(v3, match v0 {
        Adt0::C0(_, _, _, _, h788, _, _) => {
            h788
        }
    }, f277(v4.clone()), match v4.clone() {
        Adt115::C0(_, h789, _) => {
            h789
        }
    })?), f278(), v4)?)
}

pub fn f239(v0: Adt5) -> R<Result<Adt115, Adt108>> {
    let m790 = f279(v0.clone())?;
    match m790 {
        Adt139::C0 => {
            let m791 = f173(Str::lit("method"), v0.clone());
            match m791 {
                Adt114::C0 => {
                    Ok(f240(None::<Adt104>, f280(v0)))
                }
                Adt114::C1(_) => {
                    Ok(f240(None::<Adt104>, if f281(v0) {
                        Adt109::C8
                    } else {
                        Adt109::C7
                    }))
                }
            }
        }
        Adt139::C1(v2) => {
            Ok(f282(v2, v0))
        }
        Adt139::C2 => {
            Ok(f240(None::<Adt104>, Adt109::C5))
        }
    }
}

pub fn f240(v0: Option<Adt104>, v1: Adt109) -> Result<Adt115, Adt108> {
    Err::<Adt115, Adt108>(Adt108::C1(v0, v1))
}

pub fn f241(v0: List<u64>) -> R<Adt122> {
    Ok(f283({
        let a792 = v0.clone();
        length_list(a792)
    }, f284(Adt123::C0, 0u64, 0u64, v0)?))
}

pub fn f242(p799: Adt124, p800: u64, p801: List<u64>) -> R<Result<Adt6, Adt107>> {
    let m798 = (p799, (p800, p801));
    {
        let mut m798 = m798;
        loop {
            let step = {
                let (v0, (v1, v2)) = m798;
                let m793 = v2;
                match m793.uncons() {
                    None => {
                        let m794 = f285(v0, v1)?;
                        match m794 {
                            Ok(v3) => {
                                Err::<(Adt124, (u64, List<u64>)), R<Result<Adt6, Adt107>>>(Ok(Ok::<Adt6, Adt107>(v3)))
                            }
                            Err(v4) => {
                                Err::<(Adt124, (u64, List<u64>)), R<Result<Adt6, Adt107>>>(Ok(Err::<Adt6, Adt107>(v4)))
                            }
                        }
                    }
                    Some((v5, v6)) => {
                        let m795 = f286(v0, v1, v5)?;
                        match m795 {
                            Adt140::C0(v7) => {
                                Err::<(Adt124, (u64, List<u64>)), R<Result<Adt6, Adt107>>>(Ok(Err::<Adt6, Adt107>(v7)))
                            }
                            Adt140::C1(v8) => {
                                Ok::<(Adt124, (u64, List<u64>)), R<Result<Adt6, Adt107>>>((v8, ({
                                    let a796 = v1;
                                    let a797 = 1u64;
                                    nat_add(a796, a797)?
                                }, v6)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m798 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f243() -> Adt124 {
    Adt124::C0(Adt125::C0, List::<Adt129>::nil(), 0u64)
}

pub fn f244(p808: List<u64>, p809: List<u64>) -> R<bool> {
    let m807 = (p808, p809);
    {
        let mut m807 = m807;
        loop {
            let step = {
                let (v0, v1) = m807;
                let m802 = v0;
                match m802.uncons() {
                    None => {
                        let m803 = v1;
                        match m803.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), R<bool>>(Ok(true))
                            }
                            Some((_, _)) => {
                                Err::<(List<u64>, List<u64>), R<bool>>(Ok(false))
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m804 = v1;
                        match m804.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), R<bool>>(Ok(false))
                            }
                            Some((v6, v7)) => {
                                let a805 = f287(v4)?;
                                let a806 = f287(v6)?;
                                if nat_eq(a805, a806) {
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
                Ok(next) => { m807 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f245(v0: Adt111, v1: List<u64>) -> Adt111 {
    let m810 = v0;
    match m810 {
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

pub fn f246(v0: List<u64>) -> List<u64> {
    f254(f288(f254(f288(v0), List::<u64>::nil())), List::<u64>::nil())
}

pub fn f247(p813: List<u64>) -> bool {
    let m812 = p813;
    {
        let mut m812 = m812;
        loop {
            let step = {
                let v0 = m812;
                let m811 = v0;
                match m811.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f289(v1) {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m812 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f248() -> List<u64> {
    List::cons(108u64, List::cons(111u64, List::cons(99u64, List::cons(97u64, List::cons(108u64, List::cons(104u64, List::cons(111u64, List::cons(115u64, List::cons(116u64, List::<u64>::nil())))))))))
}

pub fn f249() -> List<u64> {
    List::cons(49u64, List::cons(50u64, List::cons(55u64, List::cons(46u64, List::cons(48u64, List::cons(46u64, List::cons(48u64, List::cons(46u64, List::cons(49u64, List::<u64>::nil())))))))))
}

pub fn f250() -> List<u64> {
    List::cons(91u64, List::cons(58u64, List::cons(58u64, List::cons(49u64, List::cons(93u64, List::<u64>::nil())))))
}

pub fn f251(v0: List<u64>, p816: List<List<u64>>) -> R<bool> {
    let m815 = p816;
    {
        let mut m815 = m815;
        loop {
            let step = {
                let v1 = m815;
                let m814 = v1;
                match m814.uncons() {
                    None => {
                        Err::<List<List<u64>>, R<bool>>(Ok(false))
                    }
                    Some((v2, v3)) => {
                        if f244(v0.clone(), v2)? {
                            Err::<List<List<u64>>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<List<u64>>, R<bool>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m815 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f252(v0: List<u64>) -> Option<List<u64>> {
    let m817 = v0;
    match m817.uncons() {
        None => {
            None::<List<u64>>
        }
        Some((v1, v2)) => {
            let m818 = v2;
            match m818.uncons() {
                None => {
                    None::<List<u64>>
                }
                Some((v3, v4)) => {
                    let a819 = v1;
                    let a820 = 47u64;
                    if if nat_eq(a819, a820) {
                        let a821 = v3;
                        let a822 = 47u64;
                        nat_eq(a821, a822)
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

pub fn f253(v0: List<u64>) -> bool {
    let m823 = v0;
    match m823.uncons() {
        None => {
            true
        }
        Some((v1, v2)) => {
            let a824 = v1;
            let a825 = 58u64;
            if nat_eq(a824, a825) {
                let m826 = v2.clone();
                match m826.uncons() {
                    None => {
                        false
                    }
                    Some((_, _)) => {
                        f290(v2)
                    }
                }
            } else {
                false
            }
        }
    }
}

pub fn f254(p829: List<u64>, p830: List<u64>) -> List<u64> {
    let m828 = (p829, p830);
    {
        let mut m828 = m828;
        loop {
            let step = {
                let (v0, v1) = m828;
                let m827 = v0;
                match m827.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m828 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f255(v0: u64) -> bool {
    let a831 = 48u64;
    let a832 = v0;
    if if nat_le(a831, a832) {
        let a833 = v0;
        let a834 = 57u64;
        nat_le(a833, a834)
    } else {
        false
    } {
        true
    } else {
        let a835 = 65u64;
        let a836 = v0;
        if if nat_le(a835, a836) {
            let a837 = v0;
            let a838 = 90u64;
            nat_le(a837, a838)
        } else {
            false
        } {
            true
        } else {
            let a839 = 97u64;
            let a840 = v0;
            if if nat_le(a839, a840) {
                let a841 = v0;
                let a842 = 122u64;
                nat_le(a841, a842)
            } else {
                false
            } {
                true
            } else {
                let a843 = v0;
                let a844 = 33u64;
                if nat_eq(a843, a844) {
                    true
                } else {
                    let a845 = v0;
                    let a846 = 35u64;
                    if nat_eq(a845, a846) {
                        true
                    } else {
                        let a847 = v0;
                        let a848 = 36u64;
                        if nat_eq(a847, a848) {
                            true
                        } else {
                            let a849 = v0;
                            let a850 = 37u64;
                            if nat_eq(a849, a850) {
                                true
                            } else {
                                let a851 = v0;
                                let a852 = 38u64;
                                if nat_eq(a851, a852) {
                                    true
                                } else {
                                    let a853 = v0;
                                    let a854 = 39u64;
                                    if nat_eq(a853, a854) {
                                        true
                                    } else {
                                        let a855 = v0;
                                        let a856 = 42u64;
                                        if nat_eq(a855, a856) {
                                            true
                                        } else {
                                            let a857 = v0;
                                            let a858 = 43u64;
                                            if nat_eq(a857, a858) {
                                                true
                                            } else {
                                                let a859 = v0;
                                                let a860 = 45u64;
                                                if nat_eq(a859, a860) {
                                                    true
                                                } else {
                                                    let a861 = v0;
                                                    let a862 = 46u64;
                                                    if nat_eq(a861, a862) {
                                                        true
                                                    } else {
                                                        let a863 = v0;
                                                        let a864 = 94u64;
                                                        if nat_eq(a863, a864) {
                                                            true
                                                        } else {
                                                            let a865 = v0;
                                                            let a866 = 95u64;
                                                            if nat_eq(a865, a866) {
                                                                true
                                                            } else {
                                                                let a867 = v0;
                                                                let a868 = 96u64;
                                                                if nat_eq(a867, a868) {
                                                                    true
                                                                } else {
                                                                    let a869 = v0;
                                                                    let a870 = 124u64;
                                                                    if nat_eq(a869, a870) {
                                                                        true
                                                                    } else {
                                                                        let a871 = v0;
                                                                        let a872 = 126u64;
                                                                        nat_eq(a871, a872)
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

pub fn f256(v0: u64) -> bool {
    let a873 = 48u64;
    let a874 = v0;
    if if nat_le(a873, a874) {
        let a875 = v0;
        let a876 = 57u64;
        nat_le(a875, a876)
    } else {
        false
    } {
        true
    } else {
        let a877 = 65u64;
        let a878 = v0;
        if if nat_le(a877, a878) {
            let a879 = v0;
            let a880 = 90u64;
            nat_le(a879, a880)
        } else {
            false
        } {
            true
        } else {
            let a881 = 97u64;
            let a882 = v0;
            if if nat_le(a881, a882) {
                let a883 = v0;
                let a884 = 122u64;
                nat_le(a883, a884)
            } else {
                false
            } {
                true
            } else {
                let a885 = v0;
                let a886 = 95u64;
                if nat_eq(a885, a886) {
                    true
                } else {
                    let a887 = v0;
                    let a888 = 45u64;
                    if nat_eq(a887, a888) {
                        true
                    } else {
                        let a889 = v0;
                        let a890 = 46u64;
                        nat_eq(a889, a890)
                    }
                }
            }
        }
    }
}

pub fn f257() -> Str {
    Str::lit("https://json-schema.org/draft/2020-12/schema")
}

pub fn f258(v0: Str, v1: Adt6, v2: List<(Str, Adt6)>, v3: bool, v4: List<Str>, v5: Adt112, v6: List<Adt105>) -> R<Result<(List<Adt105>, Adt112), Adt93>> {
    let m891 = f291(v0.clone(), v1, v2.clone());
    match m891 {
        None => {
            let m892 = f292(v0, v2.clone(), v3, v4.clone(), v5)?;
            match m892 {
                Adt141::C0(v8) => {
                    Ok(Err::<(List<Adt105>, Adt112), Adt93>(v8))
                }
                Adt141::C1(v9) => {
                    Ok(Ok::<(List<Adt105>, Adt112), Adt93>((f293(v2, v3, v4, v6), v9)))
                }
            }
        }
        Some(v7) => {
            Ok(Err::<(List<Adt105>, Adt112), Adt93>(v7))
        }
    }
}

pub fn f259(v0: List<(Str, Adt6)>, v1: Adt6) -> R<Adt113> {
    Ok(f294(List::cons((Str::lit("type"), f295(v0.clone(), v1.clone())?), List::cons((Str::lit("enum"), f296(v0.clone(), v1.clone())?), List::cons((Str::lit("const"), f297(v0.clone(), v1.clone())?), List::cons((Str::lit("minimum"), f298(Str::lit("minimum"), 0u64, v0.clone(), v1.clone())?), List::cons((Str::lit("maximum"), f298(Str::lit("maximum"), 1u64, v0.clone(), v1.clone())?), List::cons((Str::lit("exclusiveMinimum"), f298(Str::lit("exclusiveMinimum"), 2u64, v0.clone(), v1.clone())?), List::cons((Str::lit("exclusiveMaximum"), f298(Str::lit("exclusiveMaximum"), 3u64, v0.clone(), v1.clone())?), {
        let a893 = f299(v0.clone(), v1.clone())?;
        let a894 = List::cons((Str::lit("required"), f300(v0, v1)), List::<(Str, Adt130)>::nil());
        append_list(a893, a894)
    })))))))))
}

pub fn f260(v0: Adt6, v1: List<(Str, Adt6)>, v2: Adt6, v3: List<Adt100>) -> R<Adt113> {
    Ok(f301(f302(v0, v1.clone(), v2.clone(), v3.clone()), f301(f303(v1.clone(), v2.clone(), v3.clone()), f301(f304(v1.clone(), v2.clone(), v3.clone()), f301(f305(v1.clone(), v2.clone(), v3.clone())?, f301(f306(v1.clone(), v2.clone(), v3.clone()), f301(f307(Adt103::C0, f217(v1.clone(), v2.clone(), v3.clone()), v1.clone(), v2.clone(), v3.clone()), f301(f307(Adt103::C1, f218(v1.clone(), v2.clone(), v3.clone()), v1.clone(), v2.clone(), v3.clone()), f301(f307(Adt103::C2, f219(v1.clone(), v2.clone(), v3.clone()), v1.clone(), v2.clone(), v3.clone()), f307(Adt103::C3, f220(v1.clone(), v2.clone(), v3.clone()), v1, v2, v3))))))))))
}

pub fn f261(v0: Str, v1: List<(Str, Adt6)>) -> Option<Adt6> {
    let m895 = f227(v0, v1);
    match m895 {
        Adt114::C0 => {
            None::<Adt6>
        }
        Adt114::C1(v2) => {
            Some(v2)
        }
    }
}

pub fn f262(v0: Str) -> R<Str> {
    let m898 = {
        let a897 = f87(f88(), f267(f308(f18({
            let a896 = v0.clone();
            utf8_encode(a896)
        })?, List::<u64>::nil()), List::<u64>::nil()));
        utf8_decode(a897)
    };
    match m898 {
        None => {
            Ok(v0)
        }
        Some(v1) => {
            Ok(v1)
        }
    }
}

pub fn f263(v0: List<u64>) -> R<Option<List<u64>>> {
    let m900 = f309({
        let a899 = v0.clone();
        length_list(a899)
    }, v0, List::<u64>::nil())?;
    match m900 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v1) => {
            Ok(Some(f267(v1, List::<u64>::nil())))
        }
    }
}

pub fn f264(v0: List<Adt52>) -> R<bool> {
    let m901 = v0;
    match m901.uncons() {
        None => {
            Ok(true)
        }
        Some((v1, v2)) => {
            f310(v2, match v1 {
                Adt52::C0(h902, _, _) => {
                    h902
                }
            })
        }
    }
}

pub fn f265(v0: Str, v1: Str) -> Adt131 {
    let a903 = v0;
    let a904 = v1;
    if equal(a903, a904) {
        Adt131::C0
    } else {
        Adt131::C1
    }
}

pub fn f266(p909: u64, p910: List<Adt116>, p911: List<u64>) -> R<Result<List<u64>, Adt107>> {
    let m908 = (p909, (p910, p911));
    {
        let mut m908 = m908;
        loop {
            let step = {
                let (v0, (v1, v2)) = m908;
                let m905 = v0;
                if m905 == 0 {
                    Err::<(u64, (List<Adt116>, List<u64>)), R<Result<List<u64>, Adt107>>>(Ok(Err::<List<u64>, Adt107>(Adt107::C13)))
                } else {
                    let v3 = m905 - 1;
                    let m906 = v1;
                    match m906.uncons() {
                        None => {
                            Err::<(u64, (List<Adt116>, List<u64>)), R<Result<List<u64>, Adt107>>>(Ok(Ok::<List<u64>, Adt107>(v2)))
                        }
                        Some((v4, v5)) => {
                            let m907 = f311(v4, v5, v2)?;
                            match m907 {
                                Adt142::C0(v6) => {
                                    Err::<(u64, (List<Adt116>, List<u64>)), R<Result<List<u64>, Adt107>>>(Ok(Err::<List<u64>, Adt107>(v6)))
                                }
                                Adt142::C1(v7, v8) => {
                                    Ok::<(u64, (List<Adt116>, List<u64>)), R<Result<List<u64>, Adt107>>>((v3, (v7, v8)))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m908 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f267(p914: List<u64>, p915: List<u64>) -> List<u64> {
    let m913 = (p914, p915);
    {
        let mut m913 = m913;
        loop {
            let step = {
                let (v0, v1) = m913;
                let m912 = v0;
                match m912.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m913 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f268(v0: Adt104) -> Adt6 {
    let m916 = v0;
    match m916 {
        Adt104::C0(v1) => {
            Adt6::C3(v1)
        }
        Adt104::C1(v2) => {
            Adt6::C2(Adt7::C0(v2, 0i64))
        }
    }
}

pub fn f269(v0: Option<Adt6>) -> List<(Str, Adt6)> {
    let m917 = v0;
    match m917 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("data"), v1), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f270() -> i64 {
    -32700i64
}

pub fn f271() -> i64 {
    -32600i64
}

pub fn f272(v0: Adt109) -> Str {
    let m918 = v0;
    match m918 {
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

pub fn f273(v0: bool, v1: Adt132) -> R<Adt88> {
    let m920 = f312(match v1.clone() {
        Adt132::C0(_, h919) => {
            h919
        }
    });
    match m920 {
        None => {
            Ok(Adt88::C0(f38(f314(v0, v1)?)))
        }
        Some(v2) => {
            f313(v0, v1, v2)
        }
    }
}

pub fn f274(p925: List<Adt92>) -> R<bool> {
    let m924 = p925;
    {
        let mut m924 = m924;
        loop {
            let step = {
                let v0 = m924;
                let m921 = v0;
                match m921.uncons() {
                    None => {
                        Err::<List<Adt92>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        if f244(match v1.clone() {
                            Adt92::C0(h922, _) => {
                                h922
                            }
                        }, f315())? {
                            if f316(f317(44u64, match v1 {
                                Adt92::C0(_, h923) => {
                                    h923
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
                Ok(next) => { m924 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f275(v0: Adt0, v1: Fn2, v2: u64, v3: Option<Adt117>, v4: List<Str>, v5: Adt115) -> R<Adt132> {
    Ok(Adt132::C0(match v5.clone() {
        Adt115::C0(h926, _, _) => {
            h926
        }
    }, f318(v0, v1, v2, f319(v3, v4, v5)?)?))
}

pub fn f276(v0: List<Adt92>, v1: List<Adt12>, v2: Option<Str>, v3: Str) -> R<Adt117> {
    let v4: Adt138 = f320(Adt119::C0, f151(v0.clone(), f321())?);
    let v5: Adt138 = f320(Adt119::C1, f151(v0.clone(), f322())?);
    let v6: Adt138 = if f323(v3) {
        f320(Adt119::C2, f324(f151(v0.clone(), f325())?)?)
    } else {
        f326()
    };
    Ok(Adt117::C0(match v4.clone() {
        Adt138::C0(h927, _) => {
            h927
        }
    }, match v5.clone() {
        Adt138::C0(h928, _) => {
            h928
        }
    }, match v6.clone() {
        Adt138::C0(h929, _) => {
            h929
        }
    }, f327(match v4 {
        Adt138::C0(_, h930) => {
            h930
        }
    }, f327(match v5 {
        Adt138::C0(_, h931) => {
            h931
        }
    }, match v6 {
        Adt138::C0(_, h932) => {
            h932
        }
    })), f328(v0, v1, v2)?))
}

pub fn f277(v0: Adt115) -> Option<Str> {
    let a934 = match v0.clone() {
        Adt115::C0(_, h933, _) => {
            h933
        }
    };
    let a935 = Str::lit("tools/call");
    if equal(a934, a935) {
        let m937 = match v0 {
            Adt115::C0(_, _, h936) => {
                h936
            }
        };
        match m937 {
            None => {
                None::<Str>
            }
            Some(v1) => {
                let m938 = f173(Str::lit("name"), v1);
                match m938 {
                    Adt114::C0 => {
                        None::<Str>
                    }
                    Adt114::C1(v2) => {
                        f122(v2)
                    }
                }
            }
        }
    } else {
        None::<Str>
    }
}

pub fn f278() -> List<Str> {
    List::cons(f329(), List::<Str>::nil())
}

pub fn f279(v0: Adt5) -> R<Adt139> {
    let m939 = f173(Str::lit("id"), v0);
    match m939 {
        Adt114::C0 => {
            Ok(Adt139::C0)
        }
        Adt114::C1(v1) => {
            let m940 = v1;
            match m940 {
                Adt6::C0 => {
                    Ok(Adt139::C2)
                }
                Adt6::C1(_) => {
                    Ok(Adt139::C2)
                }
                Adt6::C2(v3) => {
                    let m941 = f330(v3)?;
                    match m941 {
                        None => {
                            Ok(Adt139::C2)
                        }
                        Some(v4) => {
                            Ok(Adt139::C1(Adt104::C1(v4)))
                        }
                    }
                }
                Adt6::C3(v2) => {
                    Ok(Adt139::C1(Adt104::C0(v2)))
                }
                Adt6::C4(_) => {
                    Ok(Adt139::C2)
                }
                Adt6::C5(_) => {
                    Ok(Adt139::C2)
                }
            }
        }
    }
}

pub fn f280(v0: Adt5) -> Adt109 {
    if f281(v0) {
        Adt109::C8
    } else {
        Adt109::C3
    }
}

pub fn f281(v0: Adt5) -> bool {
    if f331(Str::lit("result"), v0.clone()) {
        true
    } else {
        f331(Str::lit("error"), v0)
    }
}

pub fn f282(v0: Adt104, v1: Adt5) -> Result<Adt115, Adt108> {
    let m942 = f173(Str::lit("jsonrpc"), v1.clone());
    match m942 {
        Adt114::C0 => {
            f240(Some(v0), Adt109::C2)
        }
        Adt114::C1(v2) => {
            let m943 = f122(v2);
            match m943 {
                None => {
                    f240(Some(v0), Adt109::C2)
                }
                Some(v3) => {
                    let a944 = v3;
                    let a945 = f231();
                    if equal(a944, a945) {
                        f332(v0, v1)
                    } else {
                        f240(Some(v0), Adt109::C2)
                    }
                }
            }
        }
    }
}

pub fn f283(v0: u64, v1: u64) -> Adt122 {
    let a946 = v1;
    let a947 = 0u64;
    if nat_eq(a946, a947) {
        Adt122::C0
    } else {
        let a948 = v1;
        let a949 = 2u64;
        if nat_eq(a948, a949) {
            Adt122::C2(v0)
        } else {
            Adt122::C1({
                let a953 = v0;
                let a954 = {
                    let a950 = v1;
                    let a951 = 3u64;
                    let a952 = 0u64;
                    nat_quot(a950, a951, a952)
                };
                nat_sub(a953, a954)
            })
        }
    }
}

pub fn f284(p982: Adt123, p983: u64, p984: u64, p985: List<u64>) -> R<u64> {
    let m981 = (p982, (p983, (p984, p985)));
    {
        let mut m981 = m981;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m981;
                let m955 = v3;
                match m955.uncons() {
                    None => {
                        let m956 = v0;
                        match m956 {
                            Adt123::C0 => {
                                Err::<(Adt123, (u64, (u64, List<u64>))), R<u64>>(Ok(0u64))
                            }
                            Adt123::C1 => {
                                Err::<(Adt123, (u64, (u64, List<u64>))), R<u64>>(Ok(2u64))
                            }
                            Adt123::C2 => {
                                Err::<(Adt123, (u64, (u64, List<u64>))), R<u64>>(Ok(2u64))
                            }
                            Adt123::C3 => {
                                Err::<(Adt123, (u64, (u64, List<u64>))), R<u64>>(Ok(2u64))
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m957 = v0;
                        match m957 {
                            Adt123::C0 => {
                                let a958 = v4;
                                let a959 = 128u64;
                                if nat_lt(a958, a959) {
                                    Ok::<(Adt123, (u64, (u64, List<u64>))), R<u64>>((Adt123::C0, (0u64, (0u64, v5))))
                                } else {
                                    let a960 = v4;
                                    let a961 = 194u64;
                                    if nat_lt(a960, a961) {
                                        let v6: u64 = f333({
                                            let a962 = List::cons(v4, v5);
                                            length_list(a962)
                                        })?;
                                        Err::<(Adt123, (u64, (u64, List<u64>))), R<u64>>(Ok(v6))
                                    } else {
                                        let a963 = v4;
                                        let a964 = 224u64;
                                        if nat_lt(a963, a964) {
                                            Ok::<(Adt123, (u64, (u64, List<u64>))), R<u64>>((Adt123::C1, (128u64, (191u64, v5))))
                                        } else {
                                            let a965 = v4;
                                            let a966 = 224u64;
                                            if nat_eq(a965, a966) {
                                                Ok::<(Adt123, (u64, (u64, List<u64>))), R<u64>>((Adt123::C2, (160u64, (191u64, v5))))
                                            } else {
                                                let a967 = v4;
                                                let a968 = 237u64;
                                                if nat_eq(a967, a968) {
                                                    Ok::<(Adt123, (u64, (u64, List<u64>))), R<u64>>((Adt123::C2, (128u64, (159u64, v5))))
                                                } else {
                                                    let a969 = v4;
                                                    let a970 = 240u64;
                                                    if nat_lt(a969, a970) {
                                                        Ok::<(Adt123, (u64, (u64, List<u64>))), R<u64>>((Adt123::C2, (128u64, (191u64, v5))))
                                                    } else {
                                                        let a971 = v4;
                                                        let a972 = 240u64;
                                                        if nat_eq(a971, a972) {
                                                            Ok::<(Adt123, (u64, (u64, List<u64>))), R<u64>>((Adt123::C3, (144u64, (191u64, v5))))
                                                        } else {
                                                            let a973 = v4;
                                                            let a974 = 244u64;
                                                            if nat_eq(a973, a974) {
                                                                Ok::<(Adt123, (u64, (u64, List<u64>))), R<u64>>((Adt123::C3, (128u64, (143u64, v5))))
                                                            } else {
                                                                let a975 = v4;
                                                                let a976 = 244u64;
                                                                if nat_lt(a975, a976) {
                                                                    Ok::<(Adt123, (u64, (u64, List<u64>))), R<u64>>((Adt123::C3, (128u64, (191u64, v5))))
                                                                } else {
                                                                    let v7: u64 = f333({
                                                                        let a977 = List::cons(v4, v5);
                                                                        length_list(a977)
                                                                    })?;
                                                                    Err::<(Adt123, (u64, (u64, List<u64>))), R<u64>>(Ok(v7))
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
                            Adt123::C1 => {
                                if f334(v4, v1, v2) {
                                    Ok::<(Adt123, (u64, (u64, List<u64>))), R<u64>>((Adt123::C0, (0u64, (0u64, v5))))
                                } else {
                                    let v8: u64 = f333({
                                        let a978 = List::cons(v4, v5);
                                        length_list(a978)
                                    })?;
                                    Err::<(Adt123, (u64, (u64, List<u64>))), R<u64>>(Ok(v8))
                                }
                            }
                            Adt123::C2 => {
                                if f334(v4, v1, v2) {
                                    Ok::<(Adt123, (u64, (u64, List<u64>))), R<u64>>((Adt123::C1, (128u64, (191u64, v5))))
                                } else {
                                    let v9: u64 = f333({
                                        let a979 = List::cons(v4, v5);
                                        length_list(a979)
                                    })?;
                                    Err::<(Adt123, (u64, (u64, List<u64>))), R<u64>>(Ok(v9))
                                }
                            }
                            Adt123::C3 => {
                                if f334(v4, v1, v2) {
                                    Ok::<(Adt123, (u64, (u64, List<u64>))), R<u64>>((Adt123::C2, (128u64, (191u64, v5))))
                                } else {
                                    let v10: u64 = f333({
                                        let a980 = List::cons(v4, v5);
                                        length_list(a980)
                                    })?;
                                    Err::<(Adt123, (u64, (u64, List<u64>))), R<u64>>(Ok(v10))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m981 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f285(v0: Adt124, v1: u64) -> R<Result<Adt6, Adt107>> {
    let m987 = match v0.clone() {
        Adt124::C0(h986, _, _) => {
            h986
        }
    };
    match m987 {
        Adt125::C0 => {
            Ok(Err::<Adt6, Adt107>(Adt107::C3))
        }
        Adt125::C1 => {
            Ok(Err::<Adt6, Adt107>(Adt107::C3))
        }
        Adt125::C2 => {
            Ok(Err::<Adt6, Adt107>(Adt107::C3))
        }
        Adt125::C3 => {
            Ok(Err::<Adt6, Adt107>(Adt107::C3))
        }
        Adt125::C4 => {
            Ok(Err::<Adt6, Adt107>(Adt107::C3))
        }
        Adt125::C5 => {
            Ok(Err::<Adt6, Adt107>(Adt107::C3))
        }
        Adt125::C6(_, _, _) => {
            Ok(Err::<Adt6, Adt107>(Adt107::C3))
        }
        Adt125::C7(_, _) => {
            Ok(Err::<Adt6, Adt107>(Adt107::C3))
        }
        Adt125::C8(v3) => {
            let m988 = f335(v3)?;
            match m988 {
                Adt145::C0 => {
                    Ok(Err::<Adt6, Adt107>(Adt107::C7(v1)))
                }
                Adt145::C1 => {
                    Ok(Err::<Adt6, Adt107>(Adt107::C8(v1)))
                }
                Adt145::C2(v4) => {
                    let m993 = {
                        let m992 = f336(v4, match v0.clone() {
                            Adt124::C0(_, h990, _) => {
                                h990
                            }
                        }, match v0 {
                            Adt124::C0(_, _, h991) => {
                                h991
                            }
                        });
                        match m992 {
                            Adt124::C0(h989, _, _) => {
                                h989
                            }
                        }
                    };
                    match m993 {
                        Adt125::C0 => {
                            Ok(Err::<Adt6, Adt107>(Adt107::C3))
                        }
                        Adt125::C1 => {
                            Ok(Err::<Adt6, Adt107>(Adt107::C3))
                        }
                        Adt125::C2 => {
                            Ok(Err::<Adt6, Adt107>(Adt107::C3))
                        }
                        Adt125::C3 => {
                            Ok(Err::<Adt6, Adt107>(Adt107::C3))
                        }
                        Adt125::C4 => {
                            Ok(Err::<Adt6, Adt107>(Adt107::C3))
                        }
                        Adt125::C5 => {
                            Ok(Err::<Adt6, Adt107>(Adt107::C3))
                        }
                        Adt125::C6(_, _, _) => {
                            Ok(Err::<Adt6, Adt107>(Adt107::C3))
                        }
                        Adt125::C7(_, _) => {
                            Ok(Err::<Adt6, Adt107>(Adt107::C3))
                        }
                        Adt125::C8(_) => {
                            Ok(Err::<Adt6, Adt107>(Adt107::C3))
                        }
                        Adt125::C9(v5) => {
                            Ok(Ok::<Adt6, Adt107>(v5))
                        }
                    }
                }
            }
        }
        Adt125::C9(v2) => {
            Ok(Ok::<Adt6, Adt107>(v2))
        }
    }
}

pub fn f286(v0: Adt124, v1: u64, v2: u64) -> R<Adt140> {
    let m995 = match v0.clone() {
        Adt124::C0(h994, _, _) => {
            h994
        }
    };
    match m995 {
        Adt125::C0 => {
            if f337(v2) {
                Ok(Adt140::C1(v0))
            } else {
                f338(v0, v1, v2)
            }
        }
        Adt125::C1 => {
            if f337(v2) {
                Ok(Adt140::C1(v0))
            } else {
                let a996 = v2;
                let a997 = 93u64;
                if nat_eq(a996, a997) {
                    Ok(f339(v0, v1))
                } else {
                    f338(v0, v1, v2)
                }
            }
        }
        Adt125::C2 => {
            Ok(f340(v0, v1, v2))
        }
        Adt125::C3 => {
            if f337(v2) {
                Ok(Adt140::C1(v0))
            } else {
                let a998 = v2;
                let a999 = 125u64;
                if nat_eq(a998, a999) {
                    Ok(f341(v0, v1))
                } else {
                    let a1000 = v2;
                    let a1001 = 34u64;
                    if nat_eq(a1000, a1001) {
                        Ok(f342(v0, Adt125::C6(Adt126::C0, List::<u64>::nil(), true)))
                    } else {
                        Ok(f343(Adt107::C2(v1)))
                    }
                }
            }
        }
        Adt125::C4 => {
            if f337(v2) {
                Ok(Adt140::C1(v0))
            } else {
                let a1002 = v2;
                let a1003 = 34u64;
                if nat_eq(a1002, a1003) {
                    Ok(f342(v0, Adt125::C6(Adt126::C0, List::<u64>::nil(), true)))
                } else {
                    Ok(f343(Adt107::C2(v1)))
                }
            }
        }
        Adt125::C5 => {
            if f337(v2) {
                Ok(Adt140::C1(v0))
            } else {
                let a1004 = v2;
                let a1005 = 58u64;
                if nat_eq(a1004, a1005) {
                    Ok(f342(v0, Adt125::C0))
                } else {
                    Ok(f343(Adt107::C2(v1)))
                }
            }
        }
        Adt125::C6(v3, v4, v5) => {
            f344(v0, v1, v2, v3, v4, v5)
        }
        Adt125::C7(v6, v7) => {
            f345(v0, v1, v2, v6, v7)
        }
        Adt125::C8(v8) => {
            f346(v0, v1, v2, v8)
        }
        Adt125::C9(_) => {
            Ok(f340(v0, v1, v2))
        }
    }
}

pub fn f287(v0: u64) -> R<u64> {
    let a1006 = 64u64;
    let a1007 = v0;
    if if nat_lt(a1006, a1007) {
        let a1008 = v0;
        let a1009 = 91u64;
        nat_lt(a1008, a1009)
    } else {
        false
    } {
        let a1010 = v0;
        let a1011 = 32u64;
        nat_add(a1010, a1011)
    } else {
        Ok(v0)
    }
}

pub fn f288(p1014: List<u64>) -> List<u64> {
    let m1013 = p1014;
    {
        let mut m1013 = m1013;
        loop {
            let step = {
                let v0 = m1013;
                let m1012 = v0;
                match m1012.uncons() {
                    None => {
                        Err::<List<u64>, List<u64>>(List::<u64>::nil())
                    }
                    Some((v1, v2)) => {
                        if f347(v1) {
                            Ok::<List<u64>, List<u64>>(v2)
                        } else {
                            Err::<List<u64>, List<u64>>(List::cons(v1, v2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1013 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f289(v0: u64) -> bool {
    let a1015 = v0;
    let a1016 = 9u64;
    if nat_eq(a1015, a1016) {
        true
    } else {
        let a1017 = 31u64;
        let a1018 = v0;
        if nat_lt(a1017, a1018) {
            let a1019 = v0;
            let a1020 = 127u64;
            nat_lt(a1019, a1020)
        } else {
            false
        }
    }
}

pub fn f290(p1023: List<u64>) -> bool {
    let m1022 = p1023;
    {
        let mut m1022 = m1022;
        loop {
            let step = {
                let v0 = m1022;
                let m1021 = v0;
                match m1021.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f348(v1) {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1022 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f291(v0: Str, v1: Adt6, v2: List<(Str, Adt6)>) -> Option<Adt93> {
    let m1024 = f349(f350(), v2.clone());
    match m1024 {
        None => {
            let m1025 = f351(f352(), v2.clone());
            match m1025 {
                None => {
                    f353(v0, v1, v2)
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

pub fn f292(v0: Str, v1: List<(Str, Adt6)>, v2: bool, v3: List<Str>, v4: Adt112) -> R<Adt141> {
    let m1026 = f227(Str::lit("x-mcp-header"), v1.clone());
    match m1026 {
        Adt114::C0 => {
            Ok(Adt141::C1(v4))
        }
        Adt114::C1(v5) => {
            f354(v0, v1, v5, v2, v3, v4)
        }
    }
}

pub fn f293(v0: List<(Str, Adt6)>, v1: bool, v2: List<Str>, v3: List<Adt105>) -> List<Adt105> {
    f355(Str::lit("properties"), v0.clone(), v1, v2, f355(Str::lit("$defs"), v0.clone(), false, List::<Str>::nil(), f356(Str::lit("items"), v0.clone(), f356(Str::lit("additionalProperties"), v0.clone(), f357(Str::lit("allOf"), v0.clone(), f357(Str::lit("anyOf"), v0.clone(), f357(Str::lit("oneOf"), v0.clone(), f356(Str::lit("not"), v0.clone(), f356(Str::lit("if"), v0.clone(), f356(Str::lit("then"), v0.clone(), f356(Str::lit("else"), v0, v3)))))))))))
}

pub fn f294(p1033: List<(Str, Adt130)>) -> Adt113 {
    let m1032 = p1033;
    {
        let mut m1032 = m1032;
        loop {
            let step = {
                let v0 = m1032;
                let m1027 = v0;
                match m1027.uncons() {
                    None => {
                        Err::<List<(Str, Adt130)>, Adt113>(Adt113::C3(List::<Adt102>::nil()))
                    }
                    Some((v1, v2)) => {
                        let m1029 = {
                            let (_, h1028) = v1;
                            h1028
                        };
                        match m1029 {
                            Adt130::C0 => {
                                Ok::<List<(Str, Adt130)>, Adt113>(v2)
                            }
                            Adt130::C1 => {
                                Err::<List<(Str, Adt130)>, Adt113>(Adt113::C0({
                                    let (h1030, _) = v1;
                                    h1030
                                }))
                            }
                            Adt130::C2 => {
                                Err::<List<(Str, Adt130)>, Adt113>(Adt113::C1({
                                    let (h1031, _) = v1;
                                    h1031
                                }))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1032 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f295(v0: List<(Str, Adt6)>, v1: Adt6) -> R<Adt130> {
    let m1034 = f227(Str::lit("type"), v0);
    match m1034 {
        Adt114::C0 => {
            Ok(Adt130::C0)
        }
        Adt114::C1(v2) => {
            let m1035 = f122(v2.clone());
            match m1035 {
                None => {
                    let m1036 = f174(v2);
                    match m1036 {
                        None => {
                            Ok(Adt130::C2)
                        }
                        Some(v4) => {
                            f360(v4, v1, false)
                        }
                    }
                }
                Some(v3) => {
                    if f358(v3.clone()) {
                        if f359(v3, v1)? {
                            Ok(Adt130::C0)
                        } else {
                            Ok(Adt130::C1)
                        }
                    } else {
                        Ok(Adt130::C2)
                    }
                }
            }
        }
    }
}

pub fn f296(v0: List<(Str, Adt6)>, v1: Adt6) -> R<Adt130> {
    let m1037 = f227(Str::lit("enum"), v0);
    match m1037 {
        Adt114::C0 => {
            Ok(Adt130::C0)
        }
        Adt114::C1(v2) => {
            let m1038 = f174(v2);
            match m1038 {
                None => {
                    Ok(Adt130::C2)
                }
                Some(v3) => {
                    f361(v3, v1)
                }
            }
        }
    }
}

pub fn f297(v0: List<(Str, Adt6)>, v1: Adt6) -> R<Adt130> {
    let m1039 = f227(Str::lit("const"), v0);
    match m1039 {
        Adt114::C0 => {
            Ok(Adt130::C0)
        }
        Adt114::C1(v2) => {
            let m1040 = f362(v2, v1)?;
            match m1040 {
                Adt146::C0 => {
                    Ok(Adt130::C0)
                }
                Adt146::C1 => {
                    Ok(Adt130::C1)
                }
                Adt146::C2 => {
                    Ok(Adt130::C2)
                }
            }
        }
    }
}

pub fn f298(v0: Str, v1: u64, v2: List<(Str, Adt6)>, v3: Adt6) -> R<Adt130> {
    let m1041 = f227(v0, v2);
    match m1041 {
        Adt114::C0 => {
            Ok(Adt130::C0)
        }
        Adt114::C1(v4) => {
            let m1042 = f363(v4);
            match m1042 {
                None => {
                    Ok(Adt130::C2)
                }
                Some(v5) => {
                    let m1043 = f363(v3);
                    match m1043 {
                        None => {
                            Ok(Adt130::C0)
                        }
                        Some(v6) => {
                            if f364(v1, f365(v6, v5)?) {
                                Ok(Adt130::C0)
                            } else {
                                Ok(Adt130::C1)
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f299(v0: List<(Str, Adt6)>, v1: Adt6) -> R<List<(Str, Adt130)>> {
    let m1044 = v1;
    match m1044 {
        Adt6::C0 => {
            Ok(List::<(Str, Adt130)>::nil())
        }
        Adt6::C1(_) => {
            Ok(List::<(Str, Adt130)>::nil())
        }
        Adt6::C2(_) => {
            Ok(List::<(Str, Adt130)>::nil())
        }
        Adt6::C3(v4) => {
            Ok(List::cons((Str::lit("minLength"), f366(Str::lit("minLength"), true, v0.clone(), f367(v4.clone())?)?), List::cons((Str::lit("maxLength"), f366(Str::lit("maxLength"), false, v0, f367(v4)?)?), List::<(Str, Adt130)>::nil())))
        }
        Adt6::C4(v5) => {
            Ok(List::cons((Str::lit("minItems"), f366(Str::lit("minItems"), true, v0.clone(), f368(v5.clone(), 0u64)?)?), List::cons((Str::lit("maxItems"), f366(Str::lit("maxItems"), false, v0, f368(v5, 0u64)?)?), List::<(Str, Adt130)>::nil())))
        }
        Adt6::C5(v6) => {
            Ok(List::cons((Str::lit("minProperties"), f366(Str::lit("minProperties"), true, v0.clone(), f369(v6.clone(), 0u64)?)?), List::cons((Str::lit("maxProperties"), f366(Str::lit("maxProperties"), false, v0, f369(v6, 0u64)?)?), List::<(Str, Adt130)>::nil())))
        }
    }
}

pub fn f300(v0: List<(Str, Adt6)>, v1: Adt6) -> Adt130 {
    let m1045 = f227(Str::lit("required"), v0);
    match m1045 {
        Adt114::C0 => {
            Adt130::C0
        }
        Adt114::C1(v2) => {
            let m1046 = f174(v2);
            match m1046 {
                None => {
                    Adt130::C2
                }
                Some(v3) => {
                    let m1047 = f213(v1);
                    match m1047 {
                        None => {
                            Adt130::C0
                        }
                        Some(v4) => {
                            f370(v3, v4)
                        }
                    }
                }
            }
        }
    }
}

pub fn f301(v0: Adt113, v1: Adt113) -> Adt113 {
    let m1048 = v0.clone();
    match m1048 {
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
            let m1049 = v1.clone();
            match m1049 {
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
                        let a1050 = v5;
                        let a1051 = v9;
                        append_list(a1050, a1051)
                    })
                }
            }
        }
    }
}

pub fn f302(v0: Adt6, v1: List<(Str, Adt6)>, v2: Adt6, v3: List<Adt100>) -> Adt113 {
    let m1052 = f227(Str::lit("$ref"), v1);
    match m1052 {
        Adt114::C0 => {
            Adt113::C3(List::<Adt102>::nil())
        }
        Adt114::C1(v4) => {
            let m1053 = f122(v4);
            match m1053 {
                None => {
                    Adt113::C1(Str::lit("$ref"))
                }
                Some(v5) => {
                    let m1054 = f371(v0, v5.clone());
                    match m1054 {
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

pub fn f303(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt100>) -> Adt113 {
    let m1055 = f227(Str::lit("properties"), v0);
    match m1055 {
        Adt114::C0 => {
            Adt113::C3(List::<Adt102>::nil())
        }
        Adt114::C1(v3) => {
            let m1056 = f213(v3);
            match m1056 {
                None => {
                    Adt113::C1(Str::lit("properties"))
                }
                Some(v4) => {
                    let m1057 = f213(v1);
                    match m1057 {
                        None => {
                            Adt113::C3(List::<Adt102>::nil())
                        }
                        Some(v5) => {
                            Adt113::C3(f372(match v4 {
                                Adt5::C0(h1058) => {
                                    h1058
                                }
                            }, match v5 {
                                Adt5::C0(h1059) => {
                                    h1059
                                }
                            }, v2, List::<Adt102>::nil()))
                        }
                    }
                }
            }
        }
    }
}

pub fn f304(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt100>) -> Adt113 {
    let m1060 = f227(Str::lit("additionalProperties"), v0.clone());
    match m1060 {
        Adt114::C0 => {
            Adt113::C3(List::<Adt102>::nil())
        }
        Adt114::C1(v3) => {
            let m1061 = f213(v1);
            match m1061 {
                None => {
                    Adt113::C3(List::<Adt102>::nil())
                }
                Some(v4) => {
                    let m1062 = f127(v3.clone());
                    match m1062 {
                        None => {
                            Adt113::C3(f375(match v4 {
                                Adt5::C0(h1064) => {
                                    h1064
                                }
                            }, f374(v0), v3, v2, List::<Adt102>::nil()))
                        }
                        Some(v5) => {
                            if v5 {
                                Adt113::C3(List::<Adt102>::nil())
                            } else {
                                if f373(match v4 {
                                    Adt5::C0(h1063) => {
                                        h1063
                                    }
                                }, f374(v0)) {
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

pub fn f305(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt100>) -> R<Adt113> {
    let m1065 = f227(Str::lit("items"), v0);
    match m1065 {
        Adt114::C0 => {
            Ok(Adt113::C3(List::<Adt102>::nil()))
        }
        Adt114::C1(v3) => {
            let m1066 = f174(v1);
            match m1066 {
                None => {
                    Ok(Adt113::C3(List::<Adt102>::nil()))
                }
                Some(v4) => {
                    Ok(Adt113::C3(f376(v4, 0u64, v3, v2, List::<Adt102>::nil())?))
                }
            }
        }
    }
}

pub fn f306(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt100>) -> Adt113 {
    let m1067 = f227(Str::lit("allOf"), v0);
    match m1067 {
        Adt114::C0 => {
            Adt113::C3(List::<Adt102>::nil())
        }
        Adt114::C1(v3) => {
            let m1068 = f174(v3);
            match m1068 {
                None => {
                    Adt113::C1(Str::lit("allOf"))
                }
                Some(v4) => {
                    let m1069 = v4.clone();
                    match m1069.uncons() {
                        None => {
                            Adt113::C1(Str::lit("allOf"))
                        }
                        Some((_, _)) => {
                            Adt113::C3(f377(v4, v1, v2, List::<Adt102>::nil()))
                        }
                    }
                }
            }
        }
    }
}

pub fn f307(v0: Adt103, v1: Adt113, v2: List<(Str, Adt6)>, v3: Adt6, v4: List<Adt100>) -> Adt113 {
    let m1070 = v1.clone();
    match m1070 {
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
            let m1071 = v8;
            match m1071.uncons() {
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

pub fn f308(p1078: List<u64>, p1079: List<u64>) -> List<u64> {
    let m1077 = (p1078, p1079);
    {
        let mut m1077 = m1077;
        loop {
            let step = {
                let (v0, v1) = m1077;
                let m1072 = v0;
                match m1072.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        let a1073 = v2;
                        let a1074 = 126u64;
                        if nat_eq(a1073, a1074) {
                            Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(48u64, List::cons(126u64, v1))))
                        } else {
                            let a1075 = v2;
                            let a1076 = 47u64;
                            if nat_eq(a1075, a1076) {
                                Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(49u64, List::cons(126u64, v1))))
                            } else {
                                Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1077 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f309(p1088: u64, p1089: List<u64>, p1090: List<u64>) -> R<Option<List<u64>>> {
    let m1087 = (p1088, (p1089, p1090));
    {
        let mut m1087 = m1087;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1087;
                let m1080 = v0;
                if m1080 == 0 {
                    let m1081 = v1;
                    match m1081.uncons() {
                        None => {
                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(Some(v2)))
                        }
                        Some((_, _)) => {
                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                        }
                    }
                } else {
                    let v5 = m1080 - 1;
                    let m1082 = v1;
                    match m1082.uncons() {
                        None => {
                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(Some(v2)))
                        }
                        Some((v6, v7)) => {
                            let m1083 = v7;
                            match m1083.uncons() {
                                None => {
                                    Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                }
                                Some((v8, v9)) => {
                                    let m1084 = v9;
                                    match m1084.uncons() {
                                        None => {
                                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                        }
                                        Some((v10, v11)) => {
                                            let m1085 = v11;
                                            match m1085.uncons() {
                                                None => {
                                                    Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                                }
                                                Some((v12, v13)) => {
                                                    let m1086 = f378(v6, v8, v10, v12, f379(v13.clone()))?;
                                                    match m1086 {
                                                        None => {
                                                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                                        }
                                                        Some(v14) => {
                                                            Ok::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>((v5, (v13, f267(v14, v2))))
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
                Ok(next) => { m1087 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f310(p1095: List<Adt52>, p1096: Adt7) -> R<bool> {
    let m1094 = (p1095, p1096);
    {
        let mut m1094 = m1094;
        loop {
            let step = {
                let (v0, v1) = m1094;
                let m1091 = v0;
                match m1091.uncons() {
                    None => {
                        Err::<(List<Adt52>, Adt7), R<bool>>(Ok(true))
                    }
                    Some((v2, v3)) => {
                        if f380(f365(match v2.clone() {
                            Adt52::C0(h1092, _, _) => {
                                h1092
                            }
                        }, v1)?) {
                            Ok::<(List<Adt52>, Adt7), R<bool>>((v3, match v2 {
                                Adt52::C0(h1093, _, _) => {
                                    h1093
                                }
                            }))
                        } else {
                            Err::<(List<Adt52>, Adt7), R<bool>>(Ok(false))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1094 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f311(v0: Adt116, v1: List<Adt116>, v2: List<u64>) -> R<Adt142> {
    let m1097 = v0;
    match m1097 {
        Adt116::C0(v3) => {
            Ok(Adt142::C1(v1, f267(v3, v2)))
        }
        Adt116::C1(v4, v5) => {
            f381(v4, v5, v1, v2)
        }
        Adt116::C2(v6, v7) => {
            let m1098 = v7;
            match m1098.uncons() {
                None => {
                    Ok(Adt142::C1(List::cons(Adt116::C0(List::cons(93u64, List::<u64>::nil())), v1), v2))
                }
                Some((v8, v9)) => {
                    Ok(Adt142::C1(List::cons(Adt116::C1(v6, v8), List::cons(Adt116::C3(v6, v9), v1)), v2))
                }
            }
        }
        Adt116::C3(v10, v11) => {
            let m1099 = v11.clone();
            match m1099.uncons() {
                None => {
                    Ok(Adt142::C1(List::cons(Adt116::C0(List::cons(93u64, List::<u64>::nil())), v1), v2))
                }
                Some((_, _)) => {
                    Ok(Adt142::C1(List::cons(Adt116::C0(List::cons(44u64, List::<u64>::nil())), List::cons(Adt116::C2(v10, v11), v1)), v2))
                }
            }
        }
        Adt116::C4(v14, v15) => {
            let m1100 = v15;
            match m1100.uncons() {
                None => {
                    Ok(Adt142::C1(List::cons(Adt116::C0(List::cons(125u64, List::<u64>::nil())), v1), v2))
                }
                Some((v16, v17)) => {
                    Ok(Adt142::C1(List::cons(Adt116::C0({
                        let a1102 = f382({
                            let (h1101, _) = v16;
                            h1101
                        })?;
                        let a1103 = List::cons(58u64, List::<u64>::nil());
                        append_list(a1102, a1103)
                    }), List::cons(Adt116::C1(v14, {
                        let (_, h1104) = v16;
                        h1104
                    }), List::cons(Adt116::C5(v14, v17), v1))), v2))
                }
            }
        }
        Adt116::C5(v18, v19) => {
            let m1105 = v19.clone();
            match m1105.uncons() {
                None => {
                    Ok(Adt142::C1(List::cons(Adt116::C0(List::cons(125u64, List::<u64>::nil())), v1), v2))
                }
                Some((_, _)) => {
                    Ok(Adt142::C1(List::cons(Adt116::C0(List::cons(44u64, List::<u64>::nil())), List::cons(Adt116::C4(v18, v19), v1)), v2))
                }
            }
        }
    }
}

pub fn f312(v0: Result<Adt133, Adt136>) -> Option<Adt87> {
    let m1106 = v0;
    match m1106 {
        Ok(_) => {
            None::<Adt87>
        }
        Err(v2) => {
            let m1107 = v2;
            match m1107 {
                Adt136::C0(_) => {
                    None::<Adt87>
                }
                Adt136::C1(_, _) => {
                    None::<Adt87>
                }
                Adt136::C2(_) => {
                    None::<Adt87>
                }
                Adt136::C3(_) => {
                    None::<Adt87>
                }
                Adt136::C4(v8) => {
                    f383(v8)
                }
            }
        }
    }
}

pub fn f313(v0: bool, v1: Adt132, v2: Adt87) -> R<Adt88> {
    if f384(match v2.clone() {
        Adt87::C0(h1108, _) => {
            h1108
        }
    }) {
        Ok(Adt88::C1(v2))
    } else {
        Ok(Adt88::C0(f38(f314(v0, Adt132::C0(match v1 {
            Adt132::C0(h1109, _) => {
                h1109
            }
        }, Err::<Adt133, Adt136>(f385())))?)))
    }
}

pub fn f314(v0: bool, v1: Adt132) -> R<Adt95> {
    let m1111 = match v1.clone() {
        Adt132::C0(_, h1110) => {
            h1110
        }
    };
    match m1111 {
        Ok(v2) => {
            let m1112 = f386(v2.clone())?;
            match m1112 {
                None => {
                    Ok(f39())
                }
                Some(v3) => {
                    Ok(f387(v0, f142(f388(), f389(match v1 {
                        Adt132::C0(h1113, _) => {
                            h1113
                        }
                    }, v3))?, f390(match v2 {
                        Adt133::C0(_, _, h1114) => {
                            h1114
                        }
                    }, List::<List<u64>>::nil())?))
                }
            }
        }
        Err(v4) => {
            f142(f391(v4.clone()), f144(Some(match v1 {
                Adt132::C0(h1115, _) => {
                    h1115
                }
            }), f392(v4)?))
        }
    }
}

pub fn f315() -> List<u64> {
    List::cons(65u64, List::cons(99u64, List::cons(99u64, List::cons(101u64, List::cons(112u64, List::cons(116u64, List::<u64>::nil()))))))
}

pub fn f316(p1118: List<List<u64>>) -> R<bool> {
    let m1117 = p1118;
    {
        let mut m1117 = m1117;
        loop {
            let step = {
                let v0 = m1117;
                let m1116 = v0;
                match m1116.uncons() {
                    None => {
                        Err::<List<List<u64>>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        if f393(v1)? {
                            Err::<List<List<u64>>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<List<u64>>, R<bool>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1117 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f317(v0: u64, p1123: List<u64>, p1124: List<u64>, p1125: List<List<u64>>) -> List<List<u64>> {
    let m1122 = (p1123, (p1124, p1125));
    {
        let mut m1122 = m1122;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1122;
                let m1119 = v1;
                match m1119.uncons() {
                    None => {
                        Err::<(List<u64>, (List<u64>, List<List<u64>>)), List<List<u64>>>(f101(List::cons(f254(v2, List::<u64>::nil()), v3), List::<List<u64>>::nil()))
                    }
                    Some((v4, v5)) => {
                        let a1120 = v4;
                        let a1121 = v0;
                        if nat_eq(a1120, a1121) {
                            Ok::<(List<u64>, (List<u64>, List<List<u64>>)), List<List<u64>>>((v5, (List::<u64>::nil(), List::cons(f254(v2, List::<u64>::nil()), v3))))
                        } else {
                            Ok::<(List<u64>, (List<u64>, List<List<u64>>)), List<List<u64>>>((v5, (List::cons(v4, v2), v3)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1122 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f318(v0: Adt0, v1: Fn2, v2: u64, v3: Result<Adt143, Adt136>) -> R<Result<Adt133, Adt136>> {
    let m1126 = v3;
    match m1126 {
        Ok(v5) => {
            f394(v0, v1, v2, v5)
        }
        Err(v4) => {
            Ok(Err::<Adt133, Adt136>(v4))
        }
    }
}

pub fn f319(v0: Option<Adt117>, v1: List<Str>, v2: Adt115) -> R<Result<Adt143, Adt136>> {
    Ok(f395(f396(match v2.clone() {
        Adt115::C0(_, h1127, _) => {
            h1127
        }
    }), f397(v0, v1, v2)?))
}

pub fn f320(v0: Adt119, v1: Adt110) -> Adt138 {
    let m1128 = v1;
    match m1128 {
        Adt110::C0 => {
            f326()
        }
        Adt110::C1 => {
            Adt138::C0(None::<Str>, Some(Adt118::C6(v0)))
        }
        Adt110::C2 => {
            Adt138::C0(None::<Str>, Some(Adt118::C7(v0)))
        }
        Adt110::C3(v2) => {
            let m1129 = f398(v2);
            match m1129 {
                None => {
                    Adt138::C0(None::<Str>, Some(Adt118::C6(v0)))
                }
                Some(v3) => {
                    Adt138::C0(Some(v3), None::<Adt118>)
                }
            }
        }
    }
}

pub fn f321() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(112u64, List::cons(114u64, List::cons(111u64, List::cons(116u64, List::cons(111u64, List::cons(99u64, List::cons(111u64, List::cons(108u64, List::cons(45u64, List::cons(118u64, List::cons(101u64, List::cons(114u64, List::cons(115u64, List::cons(105u64, List::cons(111u64, List::cons(110u64, List::<u64>::nil()))))))))))))))))))))
}

pub fn f322() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(109u64, List::cons(101u64, List::cons(116u64, List::cons(104u64, List::cons(111u64, List::cons(100u64, List::<u64>::nil()))))))))))
}

pub fn f323(v0: Str) -> bool {
    let m1130 = f399(v0);
    match m1130 {
        Adt151::C0 => {
            false
        }
        Adt151::C1 => {
            true
        }
        Adt151::C2 => {
            true
        }
        Adt151::C3 => {
            true
        }
    }
}

pub fn f324(v0: Adt110) -> R<Adt110> {
    let m1131 = v0.clone();
    match m1131 {
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
            let m1132 = f400(v1);
            match m1132 {
                None => {
                    Ok(v0)
                }
                Some(v2) => {
                    let m1133 = f263(v2)?;
                    match m1133 {
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

pub fn f325() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(110u64, List::cons(97u64, List::cons(109u64, List::cons(101u64, List::<u64>::nil()))))))))
}

pub fn f326() -> Adt138 {
    Adt138::C0(None::<Str>, None::<Adt118>)
}

pub fn f327(v0: Option<Adt118>, v1: Option<Adt118>) -> Option<Adt118> {
    let m1134 = v0.clone();
    match m1134 {
        None => {
            v1
        }
        Some(_) => {
            v0
        }
    }
}

pub fn f328(v0: List<Adt92>, v1: List<Adt12>, v2: Option<Str>) -> R<List<Adt120>> {
    Ok(f401(f402(v0, v1, v2, List::<Adt120>::nil())?, List::<Adt120>::nil()))
}

pub fn f329() -> Str {
    Str::lit("2026-07-28")
}

pub fn f330(v0: Adt7) -> R<Option<i64>> {
    let m1137 = f403(40u64, match v0.clone() {
        Adt7::C0(h1135, _) => {
            h1135
        }
    }, match v0 {
        Adt7::C0(_, h1136) => {
            h1136
        }
    })?;
    match m1137 {
        None => {
            Ok(None::<i64>)
        }
        Some(v1) => {
            Ok(f404(v1))
        }
    }
}

pub fn f331(v0: Str, v1: Adt5) -> bool {
    f405(f173(v0, v1))
}

pub fn f332(v0: Adt104, v1: Adt5) -> Result<Adt115, Adt108> {
    let m1138 = f173(Str::lit("method"), v1.clone());
    match m1138 {
        Adt114::C0 => {
            f240(Some(v0), f280(v1))
        }
        Adt114::C1(v2) => {
            let m1139 = f122(v2);
            match m1139 {
                None => {
                    f240(Some(v0), Adt109::C4)
                }
                Some(v3) => {
                    f406(v0, v3, v1)
                }
            }
        }
    }
}

pub fn f333(v0: u64) -> R<u64> {
    let a1142 = {
        let a1140 = v0;
        let a1141 = 3u64;
        nat_mul(a1140, a1141)?
    };
    let a1143 = 1u64;
    nat_add(a1142, a1143)
}

pub fn f334(v0: u64, v1: u64, v2: u64) -> bool {
    let a1144 = v1;
    let a1145 = v0;
    if nat_le(a1144, a1145) {
        let a1146 = v0;
        let a1147 = v2;
        nat_le(a1146, a1147)
    } else {
        false
    }
}

pub fn f335(v0: Adt127) -> R<Adt145> {
    if f407(match v0.clone() {
        Adt127::C0(_, h1148, _, _, _, _, _) => {
            h1148
        }
    }) {
        let v1: i64 = {
            let a1154 = if match v0.clone() {
                Adt127::C0(_, _, _, _, _, h1149, _) => {
                    h1149
                }
            } {
                let a1151 = match v0.clone() {
                    Adt127::C0(_, _, _, _, _, _, h1150) => {
                        h1150
                    }
                };
                int_neg(a1151)?
            } else {
                match v0.clone() {
                    Adt127::C0(_, _, _, _, _, _, h1152) => {
                        h1152
                    }
                }
            };
            let a1155 = match v0.clone() {
                Adt127::C0(_, _, _, _, h1153, _, _) => {
                    h1153
                }
            };
            int_sub(a1154, a1155)?
        };
        let m1158 = {
            let a1156 = v1;
            let a1157 = -9999i64;
            compare(a1156, a1157)
        };
        if if match m1158 {
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
            let m1161 = {
                let a1159 = 9999i64;
                let a1160 = v1;
                compare(a1159, a1160)
            };
            match m1161 {
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
            Ok(Adt145::C1)
        } else {
            Ok(Adt145::C2(Adt6::C2(Adt7::C0(if match v0.clone() {
                Adt127::C0(h1162, _, _, _, _, _, _) => {
                    h1162
                }
            } {
                let a1164 = match v0 {
                    Adt127::C0(_, _, h1163, _, _, _, _) => {
                        h1163
                    }
                };
                int_neg(a1164)?
            } else {
                match v0 {
                    Adt127::C0(_, _, h1165, _, _, _, _) => {
                        h1165
                    }
                }
            }, v1))))
        }
    } else {
        Ok(Adt145::C0)
    }
}

pub fn f336(v0: Adt6, v1: List<Adt129>, v2: u64) -> Adt124 {
    let m1166 = v1.clone();
    match m1166.uncons() {
        None => {
            Adt124::C0(Adt125::C9(v0), List::<Adt129>::nil(), 0u64)
        }
        Some((v3, v4)) => {
            let m1167 = v3;
            match m1167 {
                Adt129::C0(v5) => {
                    Adt124::C0(Adt125::C2, List::cons(Adt129::C0(List::cons(v0, v5)), v4), v2)
                }
                Adt129::C1(v6, v7) => {
                    let m1168 = v7;
                    match m1168 {
                        None => {
                            Adt124::C0(Adt125::C2, v1, v2)
                        }
                        Some(v8) => {
                            Adt124::C0(Adt125::C2, List::cons(Adt129::C1(List::cons((v8, v0), v6), None::<Str>), v4), v2)
                        }
                    }
                }
            }
        }
    }
}

pub fn f337(v0: u64) -> bool {
    let a1169 = v0;
    let a1170 = 32u64;
    if nat_eq(a1169, a1170) {
        true
    } else {
        let a1171 = v0;
        let a1172 = 10u64;
        if nat_eq(a1171, a1172) {
            true
        } else {
            let a1173 = v0;
            let a1174 = 13u64;
            if nat_eq(a1173, a1174) {
                true
            } else {
                let a1175 = v0;
                let a1176 = 9u64;
                nat_eq(a1175, a1176)
            }
        }
    }
}

pub fn f338(v0: Adt124, v1: u64, v2: u64) -> R<Adt140> {
    let a1177 = v2;
    let a1178 = 34u64;
    if nat_eq(a1177, a1178) {
        Ok(f342(v0, Adt125::C6(Adt126::C0, List::<u64>::nil(), false)))
    } else {
        let a1179 = v2;
        let a1180 = 123u64;
        if nat_eq(a1179, a1180) {
            f408(v0, Adt129::C1(List::<(Str, Adt6)>::nil(), None::<Str>), Adt125::C3, v1)
        } else {
            let a1181 = v2;
            let a1182 = 91u64;
            if nat_eq(a1181, a1182) {
                f408(v0, Adt129::C0(List::<Adt6>::nil()), Adt125::C1, v1)
            } else {
                let a1183 = v2;
                let a1184 = 116u64;
                if nat_eq(a1183, a1184) {
                    Ok(f342(v0, Adt125::C7(0u64, 1u64)))
                } else {
                    let a1185 = v2;
                    let a1186 = 102u64;
                    if nat_eq(a1185, a1186) {
                        Ok(f342(v0, Adt125::C7(1u64, 1u64)))
                    } else {
                        let a1187 = v2;
                        let a1188 = 110u64;
                        if nat_eq(a1187, a1188) {
                            Ok(f342(v0, Adt125::C7(2u64, 1u64)))
                        } else {
                            let a1189 = v2;
                            let a1190 = 45u64;
                            if nat_eq(a1189, a1190) {
                                Ok(f342(v0, f409(true, Adt128::C0, 0i64, 0u64, 0i64, false, 0i64)))
                            } else {
                                let a1191 = v2;
                                let a1192 = 48u64;
                                if nat_eq(a1191, a1192) {
                                    Ok(f342(v0, f409(false, Adt128::C1, 0i64, 0u64, 0i64, false, 0i64)))
                                } else {
                                    if f334(v2, 49u64, 57u64) {
                                        Ok(f342(v0, f409(false, Adt128::C2, f410({
                                            let a1193 = v2;
                                            let a1194 = 48u64;
                                            nat_sub(a1193, a1194)
                                        }), 1u64, 0i64, false, 0i64)))
                                    } else {
                                        Ok(f343(Adt107::C2(v1)))
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

pub fn f339(v0: Adt124, v1: u64) -> Adt140 {
    let m1196 = match v0.clone() {
        Adt124::C0(_, h1195, _) => {
            h1195
        }
    };
    match m1196.uncons() {
        None => {
            f343(Adt107::C2(v1))
        }
        Some((v2, v3)) => {
            let m1197 = v2;
            match m1197 {
                Adt129::C0(v4) => {
                    Adt140::C1(f336(Adt6::C4(f411(v4, List::<Adt6>::nil())), v3, {
                        let a1199 = match v0 {
                            Adt124::C0(_, _, h1198) => {
                                h1198
                            }
                        };
                        let a1200 = 1u64;
                        nat_sub(a1199, a1200)
                    }))
                }
                Adt129::C1(_, _) => {
                    f343(Adt107::C2(v1))
                }
            }
        }
    }
}

pub fn f340(v0: Adt124, v1: u64, v2: u64) -> Adt140 {
    let m1202 = match v0.clone() {
        Adt124::C0(h1201, _, _) => {
            h1201
        }
    };
    match m1202 {
        Adt125::C0 => {
            f343(Adt107::C2(v1))
        }
        Adt125::C1 => {
            f343(Adt107::C2(v1))
        }
        Adt125::C2 => {
            if f337(v2) {
                Adt140::C1(v0)
            } else {
                let a1203 = v2;
                let a1204 = 44u64;
                if nat_eq(a1203, a1204) {
                    f412(v0, v1)
                } else {
                    let a1205 = v2;
                    let a1206 = 93u64;
                    if nat_eq(a1205, a1206) {
                        f339(v0, v1)
                    } else {
                        let a1207 = v2;
                        let a1208 = 125u64;
                        if nat_eq(a1207, a1208) {
                            f341(v0, v1)
                        } else {
                            f343(Adt107::C2(v1))
                        }
                    }
                }
            }
        }
        Adt125::C3 => {
            f343(Adt107::C2(v1))
        }
        Adt125::C4 => {
            f343(Adt107::C2(v1))
        }
        Adt125::C5 => {
            f343(Adt107::C2(v1))
        }
        Adt125::C6(_, _, _) => {
            f343(Adt107::C2(v1))
        }
        Adt125::C7(_, _) => {
            f343(Adt107::C2(v1))
        }
        Adt125::C8(_) => {
            f343(Adt107::C2(v1))
        }
        Adt125::C9(_) => {
            if f337(v2) {
                Adt140::C1(v0)
            } else {
                f343(Adt107::C10(v1))
            }
        }
    }
}

pub fn f341(v0: Adt124, v1: u64) -> Adt140 {
    let m1210 = match v0.clone() {
        Adt124::C0(_, h1209, _) => {
            h1209
        }
    };
    match m1210.uncons() {
        None => {
            f343(Adt107::C2(v1))
        }
        Some((v2, v3)) => {
            let m1211 = v2;
            match m1211 {
                Adt129::C0(_) => {
                    f343(Adt107::C2(v1))
                }
                Adt129::C1(v5, _) => {
                    Adt140::C1(f336(Adt6::C5(f413(v5, List::<(Str, Adt6)>::nil())), v3, {
                        let a1213 = match v0 {
                            Adt124::C0(_, _, h1212) => {
                                h1212
                            }
                        };
                        let a1214 = 1u64;
                        nat_sub(a1213, a1214)
                    }))
                }
            }
        }
    }
}

pub fn f342(v0: Adt124, v1: Adt125) -> Adt140 {
    Adt140::C1(f414(v0, v1))
}

pub fn f343(v0: Adt107) -> Adt140 {
    Adt140::C0(v0)
}

pub fn f344(v0: Adt124, v1: u64, v2: u64, v3: Adt126, v4: List<u64>, v5: bool) -> R<Adt140> {
    let m1215 = v3;
    match m1215 {
        Adt126::C0 => {
            let a1216 = v2;
            let a1217 = 34u64;
            if nat_eq(a1216, a1217) {
                Ok(f415(v0, v1, v4, v5))
            } else {
                let a1218 = v2;
                let a1219 = 92u64;
                if nat_eq(a1218, a1219) {
                    Ok(f342(v0, Adt125::C6(Adt126::C1, v4, v5)))
                } else {
                    let a1220 = v2;
                    let a1221 = 32u64;
                    if nat_lt(a1220, a1221) {
                        Ok(f343(Adt107::C4(v1)))
                    } else {
                        Ok(f342(v0, Adt125::C6(Adt126::C0, List::cons(v2, v4), v5)))
                    }
                }
            }
        }
        Adt126::C1 => {
            let a1222 = v2;
            let a1223 = 117u64;
            if nat_eq(a1222, a1223) {
                Ok(f342(v0, Adt125::C6(Adt126::C2(0u64, 0u64, 0u64), v4, v5)))
            } else {
                let a1224 = f416(v2);
                let a1225 = 0u64;
                if nat_eq(a1224, a1225) {
                    Ok(f343(Adt107::C5(v1)))
                } else {
                    Ok(f342(v0, Adt125::C6(Adt126::C0, List::cons(f416(v2), v4), v5)))
                }
            }
        }
        Adt126::C2(v6, v7, v8) => {
            let a1226 = 15u64;
            let a1227 = f417(v2);
            if nat_lt(a1226, a1227) {
                Ok(f343(Adt107::C5(v1)))
            } else {
                let a1230 = {
                    let a1228 = v6;
                    let a1229 = 1u64;
                    nat_add(a1228, a1229)?
                };
                let a1231 = 4u64;
                if nat_eq(a1230, a1231) {
                    f418(v0, v1, {
                        let a1234 = {
                            let a1232 = v7;
                            let a1233 = 16u64;
                            nat_mul(a1232, a1233)?
                        };
                        let a1235 = f417(v2);
                        nat_add(a1234, a1235)?
                    }, v8, v4, v5)
                } else {
                    Ok(f342(v0, Adt125::C6(Adt126::C2({
                        let a1236 = v6;
                        let a1237 = 1u64;
                        nat_add(a1236, a1237)?
                    }, {
                        let a1240 = {
                            let a1238 = v7;
                            let a1239 = 16u64;
                            nat_mul(a1238, a1239)?
                        };
                        let a1241 = f417(v2);
                        nat_add(a1240, a1241)?
                    }, v8), v4, v5)))
                }
            }
        }
        Adt126::C3(v9) => {
            let a1242 = v2;
            let a1243 = 92u64;
            if nat_eq(a1242, a1243) {
                Ok(f342(v0, Adt125::C6(Adt126::C4(v9), v4, v5)))
            } else {
                Ok(f343(Adt107::C6(v1)))
            }
        }
        Adt126::C4(v10) => {
            let a1244 = v2;
            let a1245 = 117u64;
            if nat_eq(a1244, a1245) {
                Ok(f342(v0, Adt125::C6(Adt126::C2(0u64, 0u64, v10), v4, v5)))
            } else {
                Ok(f343(Adt107::C6(v1)))
            }
        }
    }
}

pub fn f345(v0: Adt124, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Adt140> {
    let a1246 = v2;
    let a1247 = f419(v3, v4);
    if nat_eq(a1246, a1247) {
        let a1251 = {
            let a1248 = v4;
            let a1249 = 1u64;
            nat_add(a1248, a1249)?
        };
        let a1252 = {
            let a1250 = f420(v3);
            length_list(a1250)
        };
        if nat_eq(a1251, a1252) {
            Ok(f421(v0, f422(v3)))
        } else {
            Ok(f342(v0, Adt125::C7(v3, {
                let a1253 = v4;
                let a1254 = 1u64;
                nat_add(a1253, a1254)?
            })))
        }
    } else {
        Ok(f343(Adt107::C2(v1)))
    }
}

pub fn f346(v0: Adt124, v1: u64, v2: u64, v3: Adt127) -> R<Adt140> {
    let m1256 = match v3.clone() {
        Adt127::C0(_, h1255, _, _, _, _, _) => {
            h1255
        }
    };
    match m1256 {
        Adt128::C0 => {
            let a1257 = v2;
            let a1258 = 48u64;
            if nat_eq(a1257, a1258) {
                Ok(f423(v0, v3, Adt128::C1))
            } else {
                if f334(v2, 49u64, 57u64) {
                    f424(v0, v1, v2, v3, false)
                } else {
                    Ok(f343(Adt107::C7(v1)))
                }
            }
        }
        Adt128::C1 => {
            let a1259 = v2;
            let a1260 = 46u64;
            if nat_eq(a1259, a1260) {
                Ok(f423(v0, v3, Adt128::C3))
            } else {
                if f425(v2) {
                    Ok(f423(v0, v3, Adt128::C5))
                } else {
                    if f426(v2) {
                        Ok(f343(Adt107::C7(v1)))
                    } else {
                        f427(v0, v1, v2, v3)
                    }
                }
            }
        }
        Adt128::C2 => {
            if f426(v2) {
                f424(v0, v1, v2, v3, false)
            } else {
                let a1261 = v2;
                let a1262 = 46u64;
                if nat_eq(a1261, a1262) {
                    Ok(f423(v0, v3, Adt128::C3))
                } else {
                    if f425(v2) {
                        Ok(f423(v0, v3, Adt128::C5))
                    } else {
                        f427(v0, v1, v2, v3)
                    }
                }
            }
        }
        Adt128::C3 => {
            if f426(v2) {
                f424(v0, v1, v2, v3, true)
            } else {
                Ok(f343(Adt107::C7(v1)))
            }
        }
        Adt128::C4 => {
            if f426(v2) {
                f424(v0, v1, v2, v3, true)
            } else {
                if f425(v2) {
                    Ok(f423(v0, v3, Adt128::C5))
                } else {
                    f427(v0, v1, v2, v3)
                }
            }
        }
        Adt128::C5 => {
            let a1263 = v2;
            let a1264 = 43u64;
            if nat_eq(a1263, a1264) {
                Ok(f342(v0, f409(match v3.clone() {
                    Adt127::C0(h1265, _, _, _, _, _, _) => {
                        h1265
                    }
                }, Adt128::C6, match v3.clone() {
                    Adt127::C0(_, _, h1266, _, _, _, _) => {
                        h1266
                    }
                }, match v3.clone() {
                    Adt127::C0(_, _, _, h1267, _, _, _) => {
                        h1267
                    }
                }, match v3 {
                    Adt127::C0(_, _, _, _, h1268, _, _) => {
                        h1268
                    }
                }, false, 0i64)))
            } else {
                let a1269 = v2;
                let a1270 = 45u64;
                if nat_eq(a1269, a1270) {
                    Ok(f342(v0, f409(match v3.clone() {
                        Adt127::C0(h1271, _, _, _, _, _, _) => {
                            h1271
                        }
                    }, Adt128::C6, match v3.clone() {
                        Adt127::C0(_, _, h1272, _, _, _, _) => {
                            h1272
                        }
                    }, match v3.clone() {
                        Adt127::C0(_, _, _, h1273, _, _, _) => {
                            h1273
                        }
                    }, match v3 {
                        Adt127::C0(_, _, _, _, h1274, _, _) => {
                            h1274
                        }
                    }, true, 0i64)))
                } else {
                    if f426(v2) {
                        f428(v0, v1, v2, Adt127::C0(match v3.clone() {
                            Adt127::C0(h1275, _, _, _, _, _, _) => {
                                h1275
                            }
                        }, match v3.clone() {
                            Adt127::C0(_, h1276, _, _, _, _, _) => {
                                h1276
                            }
                        }, match v3.clone() {
                            Adt127::C0(_, _, h1277, _, _, _, _) => {
                                h1277
                            }
                        }, match v3.clone() {
                            Adt127::C0(_, _, _, h1278, _, _, _) => {
                                h1278
                            }
                        }, match v3 {
                            Adt127::C0(_, _, _, _, h1279, _, _) => {
                                h1279
                            }
                        }, false, 0i64), false)
                    } else {
                        Ok(f343(Adt107::C7(v1)))
                    }
                }
            }
        }
        Adt128::C6 => {
            if f426(v2) {
                f428(v0, v1, v2, v3.clone(), match v3 {
                    Adt127::C0(_, _, _, _, _, h1280, _) => {
                        h1280
                    }
                })
            } else {
                Ok(f343(Adt107::C7(v1)))
            }
        }
        Adt128::C7 => {
            if f426(v2) {
                f428(v0, v1, v2, v3.clone(), match v3 {
                    Adt127::C0(_, _, _, _, _, h1281, _) => {
                        h1281
                    }
                })
            } else {
                f427(v0, v1, v2, v3)
            }
        }
    }
}

pub fn f347(v0: u64) -> bool {
    let a1282 = v0;
    let a1283 = 9u64;
    if nat_eq(a1282, a1283) {
        true
    } else {
        let a1284 = v0;
        let a1285 = 32u64;
        nat_eq(a1284, a1285)
    }
}

pub fn f348(v0: u64) -> bool {
    let a1286 = 47u64;
    let a1287 = v0;
    if nat_lt(a1286, a1287) {
        let a1288 = v0;
        let a1289 = 58u64;
        nat_lt(a1288, a1289)
    } else {
        false
    }
}

pub fn f349(p1293: List<Str>, v1: List<(Str, Adt6)>) -> Option<Str> {
    let m1292 = p1293;
    {
        let mut m1292 = m1292;
        loop {
            let step = {
                let v0 = m1292;
                let m1290 = v0;
                match m1290.uncons() {
                    None => {
                        Err::<List<Str>, Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        let m1291 = f227(v2.clone(), v1.clone());
                        match m1291 {
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
                Ok(next) => { m1292 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f350() -> List<Str> {
    List::cons(Str::lit("pattern"), List::cons(Str::lit("patternProperties"), List::cons(Str::lit("propertyNames"), List::cons(Str::lit("multipleOf"), List::cons(Str::lit("uniqueItems"), List::cons(Str::lit("contains"), List::cons(Str::lit("minContains"), List::cons(Str::lit("maxContains"), List::cons(Str::lit("prefixItems"), List::cons(Str::lit("dependentRequired"), List::cons(Str::lit("dependentSchemas"), List::cons(Str::lit("dependencies"), List::cons(Str::lit("additionalItems"), List::cons(Str::lit("unevaluatedProperties"), List::cons(Str::lit("unevaluatedItems"), List::cons(Str::lit("$dynamicRef"), List::cons(Str::lit("$dynamicAnchor"), List::cons(Str::lit("$id"), List::cons(Str::lit("$vocabulary"), List::cons(Str::lit("contentEncoding"), List::cons(Str::lit("contentMediaType"), List::cons(Str::lit("contentSchema"), List::<Str>::nil()))))))))))))))))))))))
}

pub fn f351(p1300: List<(Str, u64)>, v1: List<(Str, Adt6)>) -> Option<Str> {
    let m1299 = p1300;
    {
        let mut m1299 = m1299;
        loop {
            let step = {
                let v0 = m1299;
                let m1294 = v0;
                match m1294.uncons() {
                    None => {
                        Err::<List<(Str, u64)>, Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        let m1296 = f227({
                            let (h1295, _) = v2.clone();
                            h1295
                        }, v1.clone());
                        match m1296 {
                            Adt114::C0 => {
                                Ok::<List<(Str, u64)>, Option<Str>>(v3)
                            }
                            Adt114::C1(v4) => {
                                if f429({
                                    let (_, h1297) = v2;
                                    h1297
                                }, v4) {
                                    Ok::<List<(Str, u64)>, Option<Str>>(v3)
                                } else {
                                    Err::<List<(Str, u64)>, Option<Str>>(Some({
                                        let (h1298, _) = v2;
                                        h1298
                                    }))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1299 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f352() -> List<(Str, u64)> {
    List::cons((Str::lit("$schema"), 0u64), List::cons((Str::lit("$ref"), 0u64), List::cons((Str::lit("$anchor"), 0u64), List::cons((Str::lit("$comment"), 0u64), List::cons((Str::lit("title"), 0u64), List::cons((Str::lit("description"), 0u64), List::cons((Str::lit("format"), 0u64), List::cons((Str::lit("type"), 7u64), List::cons((Str::lit("enum"), 3u64), List::cons((Str::lit("required"), 5u64), List::cons((Str::lit("properties"), 4u64), List::cons((Str::lit("$defs"), 4u64), List::cons((Str::lit("allOf"), 8u64), List::cons((Str::lit("anyOf"), 8u64), List::cons((Str::lit("oneOf"), 8u64), List::cons((Str::lit("minimum"), 1u64), List::cons((Str::lit("maximum"), 1u64), List::cons((Str::lit("exclusiveMinimum"), 1u64), List::cons((Str::lit("exclusiveMaximum"), 1u64), List::cons((Str::lit("minItems"), 2u64), List::cons((Str::lit("maxItems"), 2u64), List::cons((Str::lit("minLength"), 2u64), List::cons((Str::lit("maxLength"), 2u64), List::cons((Str::lit("minProperties"), 2u64), List::cons((Str::lit("maxProperties"), 2u64), List::cons((Str::lit("deprecated"), 6u64), List::cons((Str::lit("readOnly"), 6u64), List::cons((Str::lit("writeOnly"), 6u64), List::<(Str, u64)>::nil()))))))))))))))))))))))))))))
}

pub fn f353(v0: Str, v1: Adt6, v2: List<(Str, Adt6)>) -> Option<Adt93> {
    let m1301 = f227(Str::lit("$ref"), v2);
    match m1301 {
        Adt114::C0 => {
            None::<Adt93>
        }
        Adt114::C1(v3) => {
            let m1302 = f122(v3);
            match m1302 {
                None => {
                    None::<Adt93>
                }
                Some(v4) => {
                    let m1303 = f371(v1, v4.clone());
                    match m1303 {
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

pub fn f354(v0: Str, v1: List<(Str, Adt6)>, v2: Adt6, v3: bool, v4: List<Str>, v5: Adt112) -> R<Adt141> {
    if v3 {
        let m1304 = f122(v2);
        match m1304 {
            None => {
                Ok(Adt141::C0(Adt93::C7(v0)))
            }
            Some(v6) => {
                f430(v0, v1, v6.clone(), f115(v6)?, v4, v5)
            }
        }
    } else {
        Ok(Adt141::C0(Adt93::C10(v0)))
    }
}

pub fn f355(v0: Str, v1: List<(Str, Adt6)>, v2: bool, v3: List<Str>, v4: List<Adt105>) -> List<Adt105> {
    let m1305 = f227(v0.clone(), v1);
    match m1305 {
        Adt114::C0 => {
            v4
        }
        Adt114::C1(v5) => {
            let m1306 = f213(v5);
            match m1306 {
                None => {
                    v4
                }
                Some(v6) => {
                    let a1307 = v0;
                    let a1308 = Str::lit("properties");
                    if equal(a1307, a1308) {
                        f431(match v6 {
                            Adt5::C0(h1309) => {
                                h1309
                            }
                        }, v2, v3, v4)
                    } else {
                        f432(match v6 {
                            Adt5::C0(h1310) => {
                                h1310
                            }
                        }, v4)
                    }
                }
            }
        }
    }
}

pub fn f356(v0: Str, v1: List<(Str, Adt6)>, v2: List<Adt105>) -> List<Adt105> {
    let m1311 = f227(v0, v1);
    match m1311 {
        Adt114::C0 => {
            v2
        }
        Adt114::C1(v3) => {
            List::cons(Adt105::C0(v3, false, List::<Str>::nil()), v2)
        }
    }
}

pub fn f357(v0: Str, v1: List<(Str, Adt6)>, v2: List<Adt105>) -> List<Adt105> {
    let m1312 = f227(v0, v1);
    match m1312 {
        Adt114::C0 => {
            v2
        }
        Adt114::C1(v3) => {
            let m1313 = f174(v3);
            match m1313 {
                None => {
                    v2
                }
                Some(v4) => {
                    f433(v4, v2)
                }
            }
        }
    }
}

pub fn f358(v0: Str) -> bool {
    let a1314 = v0.clone();
    let a1315 = Str::lit("null");
    if equal(a1314, a1315) {
        true
    } else {
        let a1316 = v0.clone();
        let a1317 = Str::lit("boolean");
        if equal(a1316, a1317) {
            true
        } else {
            let a1318 = v0.clone();
            let a1319 = Str::lit("object");
            if equal(a1318, a1319) {
                true
            } else {
                let a1320 = v0.clone();
                let a1321 = Str::lit("array");
                if equal(a1320, a1321) {
                    true
                } else {
                    let a1322 = v0.clone();
                    let a1323 = Str::lit("number");
                    if equal(a1322, a1323) {
                        true
                    } else {
                        let a1324 = v0.clone();
                        let a1325 = Str::lit("string");
                        if equal(a1324, a1325) {
                            true
                        } else {
                            let a1326 = v0;
                            let a1327 = Str::lit("integer");
                            equal(a1326, a1327)
                        }
                    }
                }
            }
        }
    }
}

pub fn f359(v0: Str, v1: Adt6) -> R<bool> {
    let a1328 = v0.clone();
    let a1329 = f434(v1.clone());
    if equal(a1328, a1329) {
        Ok(true)
    } else {
        let a1330 = v0;
        let a1331 = Str::lit("integer");
        if equal(a1330, a1331) {
            let m1332 = f363(v1);
            match m1332 {
                None => {
                    Ok(false)
                }
                Some(v2) => {
                    f435(v2)
                }
            }
        } else {
            Ok(false)
        }
    }
}

pub fn f360(p1335: List<Adt6>, v1: Adt6, p1336: bool) -> R<Adt130> {
    let m1334 = (p1335, p1336);
    {
        let mut m1334 = m1334;
        loop {
            let step = {
                let (v0, v2) = m1334;
                let m1333 = v0;
                match m1333.uncons() {
                    None => {
                        if v2 {
                            Err::<(List<Adt6>, bool), R<Adt130>>(Ok(Adt130::C0))
                        } else {
                            Err::<(List<Adt6>, bool), R<Adt130>>(Ok(Adt130::C1))
                        }
                    }
                    Some((v3, v4)) => {
                        if f358(f436(v3.clone())) {
                            Ok::<(List<Adt6>, bool), R<Adt130>>((v4, if v2 {
                                true
                            } else {
                                f359(f436(v3), v1.clone())?
                            }))
                        } else {
                            Err::<(List<Adt6>, bool), R<Adt130>>(Ok(Adt130::C2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1334 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f361(p1340: List<Adt6>, v1: Adt6) -> R<Adt130> {
    let m1339 = p1340;
    {
        let mut m1339 = m1339;
        loop {
            let step = {
                let v0 = m1339;
                let m1337 = v0;
                match m1337.uncons() {
                    None => {
                        Err::<List<Adt6>, R<Adt130>>(Ok(Adt130::C1))
                    }
                    Some((v2, v3)) => {
                        let m1338 = f362(v2, v1.clone())?;
                        match m1338 {
                            Adt146::C0 => {
                                Err::<List<Adt6>, R<Adt130>>(Ok(Adt130::C0))
                            }
                            Adt146::C1 => {
                                Ok::<List<Adt6>, R<Adt130>>(v3)
                            }
                            Adt146::C2 => {
                                Err::<List<Adt6>, R<Adt130>>(Ok(Adt130::C2))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1339 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f362(v0: Adt6, v1: Adt6) -> R<Adt146> {
    f437(f438(), List::cons((v0, v1), List::<(Adt6, Adt6)>::nil()))
}

pub fn f363(v0: Adt6) -> Option<Adt7> {
    let m1341 = v0;
    match m1341 {
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

pub fn f364(v0: u64, v1: Adt147) -> bool {
    let a1342 = v0;
    let a1343 = 0u64;
    if nat_eq(a1342, a1343) {
        let a1344 = f439(v1);
        bool_not(a1344)
    } else {
        let a1345 = v0;
        let a1346 = 1u64;
        if nat_eq(a1345, a1346) {
            let a1347 = f380(v1);
            bool_not(a1347)
        } else {
            let a1348 = v0;
            let a1349 = 2u64;
            if nat_eq(a1348, a1349) {
                f380(v1)
            } else {
                f439(v1)
            }
        }
    }
}

pub fn f365(v0: Adt7, v1: Adt7) -> R<Adt147> {
    f440(v0.clone(), v1.clone(), f441(match v0 {
        Adt7::C0(h1350, _) => {
            h1350
        }
    }), f441(match v1 {
        Adt7::C0(h1351, _) => {
            h1351
        }
    }))
}

pub fn f366(v0: Str, v1: bool, v2: List<(Str, Adt6)>, v3: u64) -> R<Adt130> {
    let m1352 = f227(v0, v2);
    match m1352 {
        Adt114::C0 => {
            Ok(Adt130::C0)
        }
        Adt114::C1(v4) => {
            let m1353 = f442(v4);
            match m1353 {
                None => {
                    Ok(Adt130::C2)
                }
                Some(v5) => {
                    let m1356 = {
                        let a1354 = v5;
                        let a1355 = 0i64;
                        compare(a1354, a1355)
                    };
                    if match m1356 {
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
                        Ok(Adt130::C2)
                    } else {
                        if v1 {
                            if f439(f443(f136(v3, 0i64)?, v5)) {
                                Ok(Adt130::C1)
                            } else {
                                Ok(Adt130::C0)
                            }
                        } else {
                            if f380(f443(f136(v3, 0i64)?, v5)) {
                                Ok(Adt130::C1)
                            } else {
                                Ok(Adt130::C0)
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f367(v0: Str) -> R<u64> {
    f444(f18({
        let a1357 = v0;
        utf8_encode(a1357)
    })?, 0u64)
}

pub fn f368(p1362: List<Adt6>, p1363: u64) -> R<u64> {
    let m1361 = (p1362, p1363);
    {
        let mut m1361 = m1361;
        loop {
            let step = {
                let (v0, v1) = m1361;
                let m1358 = v0;
                match m1358.uncons() {
                    None => {
                        Err::<(List<Adt6>, u64), R<u64>>(Ok(v1))
                    }
                    Some((_, v3)) => {
                        Ok::<(List<Adt6>, u64), R<u64>>((v3, {
                            let a1359 = v1;
                            let a1360 = 1u64;
                            nat_add(a1359, a1360)?
                        }))
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

pub fn f369(p1368: List<(Str, Adt6)>, p1369: u64) -> R<u64> {
    let m1367 = (p1368, p1369);
    {
        let mut m1367 = m1367;
        loop {
            let step = {
                let (v0, v1) = m1367;
                let m1364 = v0;
                match m1364.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, u64), R<u64>>(Ok(v1))
                    }
                    Some((_, v3)) => {
                        Ok::<(List<(Str, Adt6)>, u64), R<u64>>((v3, {
                            let a1365 = v1;
                            let a1366 = 1u64;
                            nat_add(a1365, a1366)?
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m1367 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f370(p1373: List<Adt6>, v1: Adt5) -> Adt130 {
    let m1372 = p1373;
    {
        let mut m1372 = m1372;
        loop {
            let step = {
                let v0 = m1372;
                let m1370 = v0;
                match m1370.uncons() {
                    None => {
                        Err::<List<Adt6>, Adt130>(Adt130::C0)
                    }
                    Some((v2, v3)) => {
                        let m1371 = f122(v2);
                        match m1371 {
                            None => {
                                Err::<List<Adt6>, Adt130>(Adt130::C2)
                            }
                            Some(v4) => {
                                if f331(v4, v1.clone()) {
                                    Ok::<List<Adt6>, Adt130>(v3)
                                } else {
                                    Err::<List<Adt6>, Adt130>(Adt130::C1)
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1372 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f371(v0: Adt6, v1: Str) -> Option<Adt6> {
    let a1374 = v1.clone();
    let a1375 = Str::lit("#");
    if equal(a1374, a1375) {
        Some(v0)
    } else {
        let m1376 = f213(v0);
        match m1376 {
            None => {
                None::<Adt6>
            }
            Some(v2) => {
                let m1377 = f173(Str::lit("$defs"), v2);
                match m1377 {
                    Adt114::C0 => {
                        None::<Adt6>
                    }
                    Adt114::C1(v3) => {
                        let m1378 = f213(v3);
                        match m1378 {
                            None => {
                                None::<Adt6>
                            }
                            Some(v4) => {
                                f445(v1, match v4 {
                                    Adt5::C0(h1379) => {
                                        h1379
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

pub fn f372(p1386: List<(Str, Adt6)>, v1: List<(Str, Adt6)>, v2: List<Adt100>, p1387: List<Adt102>) -> List<Adt102> {
    let m1385 = (p1386, p1387);
    {
        let mut m1385 = m1385;
        loop {
            let step = {
                let (v0, v3) = m1385;
                let m1380 = v0;
                match m1380.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Adt102>), List<Adt102>>(v3)
                    }
                    Some((v4, v5)) => {
                        let m1382 = f227({
                            let (h1381, _) = v4.clone();
                            h1381
                        }, v1.clone());
                        match m1382 {
                            Adt114::C0 => {
                                Ok::<(List<(Str, Adt6)>, List<Adt102>), List<Adt102>>((v5, v3))
                            }
                            Adt114::C1(v6) => {
                                Ok::<(List<(Str, Adt6)>, List<Adt102>), List<Adt102>>((v5, List::cons(Adt102::C0({
                                    let (_, h1383) = v4;
                                    h1383
                                }, v6, List::cons(Adt100::C0({
                                    let (h1384, _) = v4;
                                    h1384
                                }), v2.clone())), v3)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1385 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f373(p1392: List<(Str, Adt6)>, v1: List<(Str, Adt6)>) -> bool {
    let m1391 = p1392;
    {
        let mut m1391 = m1391;
        loop {
            let step = {
                let v0 = m1391;
                let m1388 = v0;
                match m1388.uncons() {
                    None => {
                        Err::<List<(Str, Adt6)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let m1390 = f227({
                            let (h1389, _) = v2;
                            h1389
                        }, v1.clone());
                        match m1390 {
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
                Ok(next) => { m1391 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f374(v0: List<(Str, Adt6)>) -> List<(Str, Adt6)> {
    let m1393 = f227(Str::lit("properties"), v0);
    match m1393 {
        Adt114::C0 => {
            List::<(Str, Adt6)>::nil()
        }
        Adt114::C1(v1) => {
            let m1394 = f213(v1);
            match m1394 {
                None => {
                    List::<(Str, Adt6)>::nil()
                }
                Some(v2) => {
                    match v2 {
                        Adt5::C0(h1395) => {
                            h1395
                        }
                    }
                }
            }
        }
    }
}

pub fn f375(p1402: List<(Str, Adt6)>, v1: List<(Str, Adt6)>, v2: Adt6, v3: List<Adt100>, p1403: List<Adt102>) -> List<Adt102> {
    let m1401 = (p1402, p1403);
    {
        let mut m1401 = m1401;
        loop {
            let step = {
                let (v0, v4) = m1401;
                let m1396 = v0;
                match m1396.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Adt102>), List<Adt102>>(v4)
                    }
                    Some((v5, v6)) => {
                        let m1398 = f227({
                            let (h1397, _) = v5.clone();
                            h1397
                        }, v1.clone());
                        match m1398 {
                            Adt114::C0 => {
                                Ok::<(List<(Str, Adt6)>, List<Adt102>), List<Adt102>>((v6, List::cons(Adt102::C0(v2.clone(), {
                                    let (_, h1399) = v5;
                                    h1399
                                }, List::cons(Adt100::C0({
                                    let (h1400, _) = v5;
                                    h1400
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
                Ok(next) => { m1401 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f376(p1408: List<Adt6>, p1409: u64, v2: Adt6, v3: List<Adt100>, p1410: List<Adt102>) -> R<List<Adt102>> {
    let m1407 = (p1408, (p1409, p1410));
    {
        let mut m1407 = m1407;
        loop {
            let step = {
                let (v0, (v1, v4)) = m1407;
                let m1404 = v0;
                match m1404.uncons() {
                    None => {
                        Err::<(List<Adt6>, (u64, List<Adt102>)), R<List<Adt102>>>(Ok(v4))
                    }
                    Some((v5, v6)) => {
                        Ok::<(List<Adt6>, (u64, List<Adt102>)), R<List<Adt102>>>((v6, ({
                            let a1405 = v1;
                            let a1406 = 1u64;
                            nat_add(a1405, a1406)?
                        }, List::cons(Adt102::C0(v2.clone(), v5, List::cons(Adt100::C1(v1), v3.clone())), v4))))
                    }
                }
            };
            match step {
                Ok(next) => { m1407 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f377(p1413: List<Adt6>, v1: Adt6, v2: List<Adt100>, p1414: List<Adt102>) -> List<Adt102> {
    let m1412 = (p1413, p1414);
    {
        let mut m1412 = m1412;
        loop {
            let step = {
                let (v0, v3) = m1412;
                let m1411 = v0;
                match m1411.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<Adt102>), List<Adt102>>(v3)
                    }
                    Some((v4, v5)) => {
                        Ok::<(List<Adt6>, List<Adt102>), List<Adt102>>((v5, List::cons(Adt102::C0(v4, v1.clone(), v2.clone()), v3)))
                    }
                }
            };
            match step {
                Ok(next) => { m1412 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f378(v0: u64, v1: u64, v2: u64, v3: u64, v4: bool) -> R<Option<List<u64>>> {
    let m1415 = f446(v0)?;
    match m1415 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v5) => {
            let m1416 = f446(v1)?;
            match m1416 {
                None => {
                    Ok(None::<List<u64>>)
                }
                Some(v6) => {
                    let a1417 = v2;
                    let a1418 = 61u64;
                    if nat_eq(a1417, a1418) {
                        f447(v5, v6, v4, v3)
                    } else {
                        let m1419 = f446(v2)?;
                        match m1419 {
                            None => {
                                Ok(None::<List<u64>>)
                            }
                            Some(v7) => {
                                let a1420 = v3;
                                let a1421 = 61u64;
                                if nat_eq(a1420, a1421) {
                                    f448(v5, v6, v7, v4)
                                } else {
                                    let m1422 = f446(v3)?;
                                    match m1422 {
                                        None => {
                                            Ok(None::<List<u64>>)
                                        }
                                        Some(v8) => {
                                            Ok(Some(f449(v5, v6, v7, v8)?))
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

pub fn f379(v0: List<u64>) -> bool {
    let m1423 = v0;
    match m1423.uncons() {
        None => {
            true
        }
        Some((_, _)) => {
            false
        }
    }
}

pub fn f380(v0: Adt147) -> bool {
    let m1424 = v0;
    match m1424 {
        Adt147::C0 => {
            false
        }
        Adt147::C1 => {
            false
        }
        Adt147::C2 => {
            true
        }
    }
}

pub fn f381(v0: u64, v1: Adt6, v2: List<Adt116>, v3: List<u64>) -> R<Adt142> {
    let m1425 = v1;
    match m1425 {
        Adt6::C0 => {
            Ok(Adt142::C1(v2, f267(f450(2u64), v3)))
        }
        Adt6::C1(v4) => {
            Ok(Adt142::C1(v2, f267(if v4 {
                f450(0u64)
            } else {
                f450(1u64)
            }, v3)))
        }
        Adt6::C2(v5) => {
            if f451(v5.clone())? {
                Ok(Adt142::C1(v2, f267(f452(v5)?, v3)))
            } else {
                Ok(Adt142::C0(Adt107::C11))
            }
        }
        Adt6::C3(v6) => {
            Ok(Adt142::C1(v2, f267(f382(v6)?, v3)))
        }
        Adt6::C4(v7) => {
            let a1426 = v0;
            let a1427 = f453();
            if nat_lt(a1426, a1427) {
                Ok(Adt142::C1(f454(91u64, Adt116::C2({
                    let a1428 = v0;
                    let a1429 = 1u64;
                    nat_add(a1428, a1429)?
                }, v7), v2), v3))
            } else {
                Ok(Adt142::C0(Adt107::C12))
            }
        }
        Adt6::C5(v8) => {
            let a1430 = v0;
            let a1431 = f453();
            if nat_lt(a1430, a1431) {
                Ok(Adt142::C1(f454(123u64, Adt116::C4({
                    let a1432 = v0;
                    let a1433 = 1u64;
                    nat_add(a1432, a1433)?
                }, v8), v2), v3))
            } else {
                Ok(Adt142::C0(Adt107::C12))
            }
        }
    }
}

pub fn f382(v0: Str) -> R<List<u64>> {
    Ok(List::cons(34u64, {
        let a1435 = f267(f455(f18({
            let a1434 = v0;
            utf8_encode(a1434)
        })?, List::<u64>::nil())?, List::<u64>::nil());
        let a1436 = List::cons(34u64, List::<u64>::nil());
        append_list(a1435, a1436)
    }))
}

pub fn f383(v0: Adt86) -> Option<Adt87> {
    let m1437 = v0;
    match m1437 {
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

pub fn f384(v0: u64) -> bool {
    let a1438 = v0;
    let a1439 = f456();
    nat_lt(a1438, a1439)
}

pub fn f385() -> Adt136 {
    Adt136::C4(Adt86::C9(Str::lit("the effect tag does not fit thirty-two bits")))
}

pub fn f386(v0: Adt133) -> R<Option<Adt5>> {
    let m1441 = f457(match v0.clone() {
        Adt133::C0(h1440, _, _) => {
            h1440
        }
    })?;
    match m1441 {
        None => {
            Ok(None::<Adt5>)
        }
        Some(v1) => {
            Ok(Some(Adt5::C0({
                let a1444 = v1;
                let a1445 = {
                    let m1443 = f458(v0);
                    match m1443 {
                        Adt5::C0(h1442) => {
                            h1442
                        }
                    }
                };
                append_list(a1444, a1445)
            })))
        }
    }
}

pub fn f387(v0: bool, v1: Adt95, v2: Option<List<List<u64>>>) -> Adt95 {
    let m1446 = v2;
    match m1446 {
        None => {
            f39()
        }
        Some(v3) => {
            f459(v0, v1, f101(v3, List::<List<u64>>::nil()))
        }
    }
}

pub fn f388() -> u64 {
    200u64
}

pub fn f389(v0: Adt104, v1: Adt5) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("jsonrpc"), Adt6::C3(f231())), List::cons((Str::lit("id"), f268(v0)), List::cons((Str::lit("result"), Adt6::C5(match v1 {
        Adt5::C0(h1447) => {
            h1447
        }
    })), List::<(Str, Adt6)>::nil()))))
}

pub fn f390(p1451: List<Adt135>, p1452: List<List<u64>>) -> R<Option<List<List<u64>>>> {
    let m1450 = (p1451, p1452);
    {
        let mut m1450 = m1450;
        loop {
            let step = {
                let (v0, v1) = m1450;
                let m1448 = v0;
                match m1448.uncons() {
                    None => {
                        Err::<(List<Adt135>, List<List<u64>>), R<Option<List<List<u64>>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m1449 = f460(v2)?;
                        match m1449 {
                            None => {
                                Err::<(List<Adt135>, List<List<u64>>), R<Option<List<List<u64>>>>>(Ok(None::<List<List<u64>>>))
                            }
                            Some(v4) => {
                                Ok::<(List<Adt135>, List<List<u64>>), R<Option<List<List<u64>>>>>((v3, List::cons(v4, v1)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1450 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f391(v0: Adt136) -> u64 {
    let m1453 = v0;
    match m1453 {
        Adt136::C0(_) => {
            f190()
        }
        Adt136::C1(_, _) => {
            f190()
        }
        Adt136::C2(_) => {
            f190()
        }
        Adt136::C3(_) => {
            f461()
        }
        Adt136::C4(v6) => {
            f462(v6)
        }
    }
}

pub fn f392(v0: Adt136) -> R<Adt82> {
    Ok(Adt82::C0(f463(v0.clone()), f464(v0.clone()), f465(v0)?))
}

pub fn f393(v0: List<u64>) -> R<bool> {
    let m1454 = f317(59u64, v0, List::<u64>::nil(), List::<List<u64>>::nil());
    match m1454.uncons() {
        None => {
            Ok(false)
        }
        Some((v1, v2)) => {
            if f466(v2)? {
                Ok(false)
            } else {
                f244(f246(v1), f467())
            }
        }
    }
}

pub fn f394(v0: Adt0, v1: Fn2, v2: u64, v3: Adt143) -> R<Result<Adt133, Adt136>> {
    f468(v0.clone(), v1, v2, v3.clone(), f469(match v0 {
        Adt0::C0(_, _, h1455, _, _, _, _) => {
            h1455
        }
    }, match v3 {
        Adt143::C0(_, h1456, _, _) => {
            h1456
        }
    }))
}

pub fn f395(v0: Adt148, v1: Adt149) -> Result<Adt143, Adt136> {
    let m1457 = v1;
    match m1457 {
        Adt149::C0(v3) => {
            f470(v3, v0)
        }
        Adt149::C1(v2) => {
            Err::<Adt143, Adt136>(v2)
        }
    }
}

pub fn f396(v0: Str) -> Adt148 {
    let a1458 = v0.clone();
    let a1459 = Str::lit("server/discover");
    if equal(a1458, a1459) {
        Adt148::C1(Adt144::C0)
    } else {
        let a1460 = v0.clone();
        let a1461 = Str::lit("tools/list");
        if equal(a1460, a1461) {
            Adt148::C1(Adt144::C1)
        } else {
            let a1462 = v0.clone();
            let a1463 = Str::lit("tools/call");
            if equal(a1462, a1463) {
                Adt148::C1(Adt144::C2)
            } else {
                let a1464 = v0.clone();
                let a1465 = Str::lit("resources/list");
                if equal(a1464, a1465) {
                    Adt148::C1(Adt144::C3)
                } else {
                    let a1466 = v0.clone();
                    let a1467 = Str::lit("resources/read");
                    if equal(a1466, a1467) {
                        Adt148::C1(Adt144::C4)
                    } else {
                        let a1468 = v0.clone();
                        let a1469 = Str::lit("resources/templates/list");
                        if equal(a1468, a1469) {
                            Adt148::C1(Adt144::C5)
                        } else {
                            let a1470 = v0.clone();
                            let a1471 = Str::lit("prompts/list");
                            if equal(a1470, a1471) {
                                Adt148::C1(Adt144::C6)
                            } else {
                                let a1472 = v0.clone();
                                let a1473 = Str::lit("prompts/get");
                                if equal(a1472, a1473) {
                                    Adt148::C1(Adt144::C7)
                                } else {
                                    let a1474 = v0.clone();
                                    let a1475 = Str::lit("completion/complete");
                                    if equal(a1474, a1475) {
                                        Adt148::C1(Adt144::C8)
                                    } else {
                                        let a1476 = v0.clone();
                                        let a1477 = Str::lit("tasks/get");
                                        if equal(a1476, a1477) {
                                            Adt148::C1(Adt144::C9)
                                        } else {
                                            let a1478 = v0.clone();
                                            let a1479 = Str::lit("tasks/update");
                                            if equal(a1478, a1479) {
                                                Adt148::C1(Adt144::C10)
                                            } else {
                                                let a1480 = v0;
                                                let a1481 = Str::lit("tasks/cancel");
                                                if equal(a1480, a1481) {
                                                    Adt148::C1(Adt144::C11)
                                                } else {
                                                    Adt148::C0
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

pub fn f397(v0: Option<Adt117>, v1: List<Str>, v2: Adt115) -> R<Adt149> {
    f471(v0, v1, v2.clone(), f472(match v2 {
        Adt115::C0(_, _, h1482) => {
            h1482
        }
    }))
}

pub fn f398(v0: List<u64>) -> Option<Str> {
    let a1483 = f87(f88(), v0);
    utf8_decode(a1483)
}

pub fn f399(v0: Str) -> Adt151 {
    let m1484 = f396(v0);
    match m1484 {
        Adt148::C0 => {
            Adt151::C0
        }
        Adt148::C1(v1) => {
            f473(v1)
        }
    }
}

pub fn f400(v0: List<u64>) -> Option<List<u64>> {
    let m1485 = f474(f475(), v0);
    match m1485 {
        None => {
            None::<List<u64>>
        }
        Some(v1) => {
            let m1486 = f474(f476(), f254(v1, List::<u64>::nil()));
            match m1486 {
                None => {
                    None::<List<u64>>
                }
                Some(v2) => {
                    Some(f254(v2, List::<u64>::nil()))
                }
            }
        }
    }
}

pub fn f401(p1489: List<Adt120>, p1490: List<Adt120>) -> List<Adt120> {
    let m1488 = (p1489, p1490);
    {
        let mut m1488 = m1488;
        loop {
            let step = {
                let (v0, v1) = m1488;
                let m1487 = v0;
                match m1487.uncons() {
                    None => {
                        Err::<(List<Adt120>, List<Adt120>), List<Adt120>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt120>, List<Adt120>), List<Adt120>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1488 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f402(v0: List<Adt92>, p1497: List<Adt12>, v2: Option<Str>, p1498: List<Adt120>) -> R<List<Adt120>> {
    let m1496 = (p1497, p1498);
    {
        let mut m1496 = m1496;
        loop {
            let step = {
                let (v1, v3) = m1496;
                let m1491 = v1;
                match m1491.uncons() {
                    None => {
                        Err::<(List<Adt12>, List<Adt120>), R<List<Adt120>>>(Ok(v3))
                    }
                    Some((v4, v5)) => {
                        if f477(v4.clone(), v2.clone()) {
                            Ok::<(List<Adt12>, List<Adt120>), R<List<Adt120>>>((v5, List::cons(Adt120::C0(match v4.clone() {
                                Adt12::C0(_, h1492, _, _) => {
                                    h1492
                                }
                            }, match v4.clone() {
                                Adt12::C0(_, _, h1493, _) => {
                                    h1493
                                }
                            }, match v4.clone() {
                                Adt12::C0(_, _, _, h1494) => {
                                    h1494
                                }
                            }, f478(v0.clone(), match v4 {
                                Adt12::C0(_, h1495, _, _) => {
                                    h1495
                                }
                            })?), v3)))
                        } else {
                            Ok::<(List<Adt12>, List<Adt120>), R<List<Adt120>>>((v5, v3))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1496 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f403(p1514: u64, p1515: i64, p1516: i64) -> R<Option<i64>> {
    let m1513 = (p1514, (p1515, p1516));
    {
        let mut m1513 = m1513;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1513;
                let m1499 = v0;
                if m1499 == 0 {
                    Err::<(u64, (i64, i64)), R<Option<i64>>>(Ok(None::<i64>))
                } else {
                    let v3 = m1499 - 1;
                    if f479(v2) {
                        Err::<(u64, (i64, i64)), R<Option<i64>>>(Ok(Some(v1)))
                    } else {
                        let m1502 = {
                            let a1500 = v2;
                            let a1501 = 0i64;
                            compare(a1500, a1501)
                        };
                        if match m1502 {
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
                            if f479({
                                let a1503 = v1;
                                let a1504 = 10i64;
                                let a1505 = 0i64;
                                int_rem(a1503, a1504, a1505)
                            }) {
                                Ok::<(u64, (i64, i64)), R<Option<i64>>>((v3, ({
                                    let a1506 = v1;
                                    let a1507 = 10i64;
                                    let a1508 = 0i64;
                                    int_quot(a1506, a1507, a1508)?
                                }, f182(v2, 1i64)?)))
                            } else {
                                Err::<(u64, (i64, i64)), R<Option<i64>>>(Ok(None::<i64>))
                            }
                        } else {
                            Ok::<(u64, (i64, i64)), R<Option<i64>>>((v3, ({
                                let a1509 = v1;
                                let a1510 = 10i64;
                                int_mul(a1509, a1510)?
                            }, {
                                let a1511 = v2;
                                let a1512 = 1i64;
                                int_sub(a1511, a1512)?
                            })))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1513 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f404(v0: i64) -> Option<i64> {
    let m1519 = {
        let a1517 = 9007199254740991i64;
        let a1518 = v0;
        compare(a1517, a1518)
    };
    if if match m1519 {
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
        let m1522 = {
            let a1520 = v0;
            let a1521 = -9007199254740991i64;
            compare(a1520, a1521)
        };
        match m1522 {
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

pub fn f405(v0: Adt114) -> bool {
    let m1523 = v0;
    match m1523 {
        Adt114::C0 => {
            false
        }
        Adt114::C1(_) => {
            true
        }
    }
}

pub fn f406(v0: Adt104, v1: Str, v2: Adt5) -> Result<Adt115, Adt108> {
    let m1524 = f173(Str::lit("params"), v2);
    match m1524 {
        Adt114::C0 => {
            Ok::<Adt115, Adt108>(Adt115::C0(v0, v1, None::<Adt5>))
        }
        Adt114::C1(v3) => {
            let m1525 = f213(v3);
            match m1525 {
                None => {
                    f240(Some(v0), Adt109::C6)
                }
                Some(v4) => {
                    Ok::<Adt115, Adt108>(Adt115::C0(v0, v1, Some(v4)))
                }
            }
        }
    }
}

pub fn f407(v0: Adt128) -> bool {
    let m1526 = v0;
    match m1526 {
        Adt128::C0 => {
            false
        }
        Adt128::C1 => {
            true
        }
        Adt128::C2 => {
            true
        }
        Adt128::C3 => {
            false
        }
        Adt128::C4 => {
            true
        }
        Adt128::C5 => {
            false
        }
        Adt128::C6 => {
            false
        }
        Adt128::C7 => {
            true
        }
    }
}

pub fn f408(v0: Adt124, v1: Adt129, v2: Adt125, v3: u64) -> R<Adt140> {
    let a1528 = match v0.clone() {
        Adt124::C0(_, _, h1527) => {
            h1527
        }
    };
    let a1529 = f453();
    if nat_lt(a1528, a1529) {
        Ok(Adt140::C1(Adt124::C0(v2, List::cons(v1, match v0.clone() {
            Adt124::C0(_, h1530, _) => {
                h1530
            }
        }), {
            let a1532 = match v0 {
                Adt124::C0(_, _, h1531) => {
                    h1531
                }
            };
            let a1533 = 1u64;
            nat_add(a1532, a1533)?
        })))
    } else {
        Ok(f343(Adt107::C9(v3)))
    }
}

pub fn f409(v0: bool, v1: Adt128, v2: i64, v3: u64, v4: i64, v5: bool, v6: i64) -> Adt125 {
    Adt125::C8(Adt127::C0(v0, v1, v2, v3, v4, v5, v6))
}

pub fn f410(v0: u64) -> i64 {
    let a1534 = v0;
    let a1535 = 0u64;
    if nat_eq(a1534, a1535) {
        0i64
    } else {
        let a1536 = v0;
        let a1537 = 1u64;
        if nat_eq(a1536, a1537) {
            1i64
        } else {
            let a1538 = v0;
            let a1539 = 2u64;
            if nat_eq(a1538, a1539) {
                2i64
            } else {
                let a1540 = v0;
                let a1541 = 3u64;
                if nat_eq(a1540, a1541) {
                    3i64
                } else {
                    let a1542 = v0;
                    let a1543 = 4u64;
                    if nat_eq(a1542, a1543) {
                        4i64
                    } else {
                        let a1544 = v0;
                        let a1545 = 5u64;
                        if nat_eq(a1544, a1545) {
                            5i64
                        } else {
                            let a1546 = v0;
                            let a1547 = 6u64;
                            if nat_eq(a1546, a1547) {
                                6i64
                            } else {
                                let a1548 = v0;
                                let a1549 = 7u64;
                                if nat_eq(a1548, a1549) {
                                    7i64
                                } else {
                                    let a1550 = v0;
                                    let a1551 = 8u64;
                                    if nat_eq(a1550, a1551) {
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

pub fn f411(p1554: List<Adt6>, p1555: List<Adt6>) -> List<Adt6> {
    let m1553 = (p1554, p1555);
    {
        let mut m1553 = m1553;
        loop {
            let step = {
                let (v0, v1) = m1553;
                let m1552 = v0;
                match m1552.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt6>, List<Adt6>), List<Adt6>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1553 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f412(v0: Adt124, v1: u64) -> Adt140 {
    let m1557 = match v0.clone() {
        Adt124::C0(_, h1556, _) => {
            h1556
        }
    };
    match m1557.uncons() {
        None => {
            f343(Adt107::C2(v1))
        }
        Some((v2, _)) => {
            let m1558 = v2;
            match m1558 {
                Adt129::C0(_) => {
                    f342(v0, Adt125::C0)
                }
                Adt129::C1(_, _) => {
                    f342(v0, Adt125::C4)
                }
            }
        }
    }
}

pub fn f413(p1561: List<(Str, Adt6)>, p1562: List<(Str, Adt6)>) -> List<(Str, Adt6)> {
    let m1560 = (p1561, p1562);
    {
        let mut m1560 = m1560;
        loop {
            let step = {
                let (v0, v1) = m1560;
                let m1559 = v0;
                match m1559.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<(Str, Adt6)>), List<(Str, Adt6)>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Adt6)>, List<(Str, Adt6)>), List<(Str, Adt6)>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1560 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f414(v0: Adt124, v1: Adt125) -> Adt124 {
    Adt124::C0(v1, match v0.clone() {
        Adt124::C0(_, h1563, _) => {
            h1563
        }
    }, match v0 {
        Adt124::C0(_, _, h1564) => {
            h1564
        }
    })
}

pub fn f415(v0: Adt124, v1: u64, v2: List<u64>, v3: bool) -> Adt140 {
    let m1566 = {
        let a1565 = f87(f88(), f267(v2, List::<u64>::nil()));
        utf8_decode(a1565)
    };
    match m1566 {
        None => {
            f343(Adt107::C14)
        }
        Some(v4) => {
            if v3 {
                f480(v0, v1, v4)
            } else {
                f421(v0, Adt6::C3(v4))
            }
        }
    }
}

pub fn f416(v0: u64) -> u64 {
    let a1567 = v0;
    let a1568 = 34u64;
    if nat_eq(a1567, a1568) {
        34u64
    } else {
        let a1569 = v0;
        let a1570 = 92u64;
        if nat_eq(a1569, a1570) {
            92u64
        } else {
            let a1571 = v0;
            let a1572 = 47u64;
            if nat_eq(a1571, a1572) {
                47u64
            } else {
                let a1573 = v0;
                let a1574 = 98u64;
                if nat_eq(a1573, a1574) {
                    8u64
                } else {
                    let a1575 = v0;
                    let a1576 = 102u64;
                    if nat_eq(a1575, a1576) {
                        12u64
                    } else {
                        let a1577 = v0;
                        let a1578 = 110u64;
                        if nat_eq(a1577, a1578) {
                            10u64
                        } else {
                            let a1579 = v0;
                            let a1580 = 114u64;
                            if nat_eq(a1579, a1580) {
                                13u64
                            } else {
                                let a1581 = v0;
                                let a1582 = 116u64;
                                if nat_eq(a1581, a1582) {
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

pub fn f417(v0: u64) -> u64 {
    let a1583 = v0;
    let a1584 = 48u64;
    if nat_lt(a1583, a1584) {
        16u64
    } else {
        let a1585 = v0;
        let a1586 = 58u64;
        if nat_lt(a1585, a1586) {
            let a1587 = v0;
            let a1588 = 48u64;
            nat_sub(a1587, a1588)
        } else {
            let a1589 = v0;
            let a1590 = 65u64;
            if nat_lt(a1589, a1590) {
                16u64
            } else {
                let a1591 = v0;
                let a1592 = 71u64;
                if nat_lt(a1591, a1592) {
                    let a1593 = v0;
                    let a1594 = 55u64;
                    nat_sub(a1593, a1594)
                } else {
                    let a1595 = v0;
                    let a1596 = 97u64;
                    if nat_lt(a1595, a1596) {
                        16u64
                    } else {
                        let a1597 = v0;
                        let a1598 = 103u64;
                        if nat_lt(a1597, a1598) {
                            let a1599 = v0;
                            let a1600 = 87u64;
                            nat_sub(a1599, a1600)
                        } else {
                            16u64
                        }
                    }
                }
            }
        }
    }
}

pub fn f418(v0: Adt124, v1: u64, v2: u64, v3: u64, v4: List<u64>, v5: bool) -> R<Adt140> {
    let a1601 = v3;
    let a1602 = 0u64;
    if nat_eq(a1601, a1602) {
        if f334(v2, 55296u64, 56319u64) {
            Ok(f342(v0, Adt125::C6(Adt126::C3(v2), v4, v5)))
        } else {
            if f334(v2, 56320u64, 57343u64) {
                Ok(f343(Adt107::C6(v1)))
            } else {
                Ok(f342(v0, Adt125::C6(Adt126::C0, f481(v2, v4)?, v5)))
            }
        }
    } else {
        if f334(v2, 56320u64, 57343u64) {
            Ok(f342(v0, Adt125::C6(Adt126::C0, f481({
                let a1611 = 65536u64;
                let a1612 = {
                    let a1609 = {
                        let a1605 = {
                            let a1603 = v3;
                            let a1604 = 55296u64;
                            nat_sub(a1603, a1604)
                        };
                        let a1606 = 1024u64;
                        nat_mul(a1605, a1606)?
                    };
                    let a1610 = {
                        let a1607 = v2;
                        let a1608 = 56320u64;
                        nat_sub(a1607, a1608)
                    };
                    nat_add(a1609, a1610)?
                };
                nat_add(a1611, a1612)?
            }, v4)?, v5)))
        } else {
            Ok(f343(Adt107::C6(v1)))
        }
    }
}

pub fn f419(v0: u64, v1: u64) -> u64 {
    let m1615 = {
        let a1613 = f420(v0);
        let a1614 = v1;
        index_list(a1613, a1614)
    };
    match m1615 {
        None => {
            0u64
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f420(v0: u64) -> List<u64> {
    let a1616 = v0;
    let a1617 = 0u64;
    if nat_eq(a1616, a1617) {
        List::cons(116u64, List::cons(114u64, List::cons(117u64, List::cons(101u64, List::<u64>::nil()))))
    } else {
        let a1618 = v0;
        let a1619 = 1u64;
        if nat_eq(a1618, a1619) {
            List::cons(102u64, List::cons(97u64, List::cons(108u64, List::cons(115u64, List::cons(101u64, List::<u64>::nil())))))
        } else {
            List::cons(110u64, List::cons(117u64, List::cons(108u64, List::cons(108u64, List::<u64>::nil()))))
        }
    }
}

pub fn f421(v0: Adt124, v1: Adt6) -> Adt140 {
    Adt140::C1(f336(v1, match v0.clone() {
        Adt124::C0(_, h1620, _) => {
            h1620
        }
    }, match v0 {
        Adt124::C0(_, _, h1621) => {
            h1621
        }
    }))
}

pub fn f422(v0: u64) -> Adt6 {
    let a1622 = v0;
    let a1623 = 0u64;
    if nat_eq(a1622, a1623) {
        Adt6::C1(true)
    } else {
        let a1624 = v0;
        let a1625 = 1u64;
        if nat_eq(a1624, a1625) {
            Adt6::C1(false)
        } else {
            Adt6::C0
        }
    }
}

pub fn f423(v0: Adt124, v1: Adt127, v2: Adt128) -> Adt140 {
    f342(v0, Adt125::C8(Adt127::C0(match v1.clone() {
        Adt127::C0(h1626, _, _, _, _, _, _) => {
            h1626
        }
    }, v2, match v1.clone() {
        Adt127::C0(_, _, h1627, _, _, _, _) => {
            h1627
        }
    }, match v1.clone() {
        Adt127::C0(_, _, _, h1628, _, _, _) => {
            h1628
        }
    }, match v1.clone() {
        Adt127::C0(_, _, _, _, h1629, _, _) => {
            h1629
        }
    }, match v1.clone() {
        Adt127::C0(_, _, _, _, _, h1630, _) => {
            h1630
        }
    }, match v1 {
        Adt127::C0(_, _, _, _, _, _, h1631) => {
            h1631
        }
    })))
}

pub fn f424(v0: Adt124, v1: u64, v2: u64, v3: Adt127, v4: bool) -> R<Adt140> {
    let v5: u64 = {
        let a1633 = match v3.clone() {
            Adt127::C0(_, _, _, h1632, _, _, _) => {
                h1632
            }
        };
        let a1634 = 0u64;
        if if nat_eq(a1633, a1634) {
            let a1635 = v2;
            let a1636 = 48u64;
            nat_eq(a1635, a1636)
        } else {
            false
        } {
            0u64
        } else {
            let a1638 = match v3.clone() {
                Adt127::C0(_, _, _, h1637, _, _, _) => {
                    h1637
                }
            };
            let a1639 = 1u64;
            nat_add(a1638, a1639)?
        }
    };
    let v6: i64 = if v4 {
        f182(match v3.clone() {
            Adt127::C0(_, _, _, _, h1640, _, _) => {
                h1640
            }
        }, 1i64)?
    } else {
        match v3.clone() {
            Adt127::C0(_, _, _, _, h1641, _, _) => {
                h1641
            }
        }
    };
    let a1642 = f482();
    let a1643 = v5;
    if nat_lt(a1642, a1643) {
        Ok(f343(Adt107::C8(v1)))
    } else {
        let m1646 = {
            let a1644 = 99999i64;
            let a1645 = v6;
            compare(a1644, a1645)
        };
        if match m1646 {
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
            Ok(f343(Adt107::C8(v1)))
        } else {
            Ok(f342(v0, f409(match v3.clone() {
                Adt127::C0(h1647, _, _, _, _, _, _) => {
                    h1647
                }
            }, if v4 {
                Adt128::C4
            } else {
                Adt128::C2
            }, f182({
                let a1649 = match v3.clone() {
                    Adt127::C0(_, _, h1648, _, _, _, _) => {
                        h1648
                    }
                };
                let a1650 = 10i64;
                int_mul(a1649, a1650)?
            }, f410({
                let a1651 = v2;
                let a1652 = 48u64;
                nat_sub(a1651, a1652)
            }))?, v5, v6, match v3.clone() {
                Adt127::C0(_, _, _, _, _, h1653, _) => {
                    h1653
                }
            }, match v3 {
                Adt127::C0(_, _, _, _, _, _, h1654) => {
                    h1654
                }
            })))
        }
    }
}

pub fn f425(v0: u64) -> bool {
    let a1655 = v0;
    let a1656 = 101u64;
    if nat_eq(a1655, a1656) {
        true
    } else {
        let a1657 = v0;
        let a1658 = 69u64;
        nat_eq(a1657, a1658)
    }
}

pub fn f426(v0: u64) -> bool {
    f334(v0, 48u64, 57u64)
}

pub fn f427(v0: Adt124, v1: u64, v2: u64, v3: Adt127) -> R<Adt140> {
    let m1659 = f335(v3)?;
    match m1659 {
        Adt145::C0 => {
            Ok(f343(Adt107::C7(v1)))
        }
        Adt145::C1 => {
            Ok(f343(Adt107::C8(v1)))
        }
        Adt145::C2(v4) => {
            Ok(f340(f336(v4, match v0.clone() {
                Adt124::C0(_, h1660, _) => {
                    h1660
                }
            }, match v0 {
                Adt124::C0(_, _, h1661) => {
                    h1661
                }
            }), v1, v2))
        }
    }
}

pub fn f428(v0: Adt124, v1: u64, v2: u64, v3: Adt127, v4: bool) -> R<Adt140> {
    let v5: i64 = f182({
        let a1663 = match v3.clone() {
            Adt127::C0(_, _, _, _, _, _, h1662) => {
                h1662
            }
        };
        let a1664 = 10i64;
        int_mul(a1663, a1664)?
    }, f410({
        let a1665 = v2;
        let a1666 = 48u64;
        nat_sub(a1665, a1666)
    }))?;
    let m1669 = {
        let a1667 = 99999i64;
        let a1668 = v5;
        compare(a1667, a1668)
    };
    if match m1669 {
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
        Ok(f343(Adt107::C8(v1)))
    } else {
        Ok(f342(v0, f409(match v3.clone() {
            Adt127::C0(h1670, _, _, _, _, _, _) => {
                h1670
            }
        }, Adt128::C7, match v3.clone() {
            Adt127::C0(_, _, h1671, _, _, _, _) => {
                h1671
            }
        }, match v3.clone() {
            Adt127::C0(_, _, _, h1672, _, _, _) => {
                h1672
            }
        }, match v3 {
            Adt127::C0(_, _, _, _, h1673, _, _) => {
                h1673
            }
        }, v4, v5)))
    }
}

pub fn f429(v0: u64, v1: Adt6) -> bool {
    let a1674 = v0;
    let a1675 = 0u64;
    if nat_eq(a1674, a1675) {
        f483(v1)
    } else {
        let a1676 = v0;
        let a1677 = 1u64;
        if nat_eq(a1676, a1677) {
            f484(v1)
        } else {
            let a1678 = v0;
            let a1679 = 2u64;
            if nat_eq(a1678, a1679) {
                f485(v1)
            } else {
                let a1680 = v0;
                let a1681 = 3u64;
                if nat_eq(a1680, a1681) {
                    f486(v1)
                } else {
                    let a1682 = v0;
                    let a1683 = 4u64;
                    if nat_eq(a1682, a1683) {
                        f487(v1)
                    } else {
                        let a1684 = v0;
                        let a1685 = 5u64;
                        if nat_eq(a1684, a1685) {
                            f488(v1)
                        } else {
                            let a1686 = v0;
                            let a1687 = 6u64;
                            if nat_eq(a1686, a1687) {
                                f489(v1)
                            } else {
                                let a1688 = v0;
                                let a1689 = 7u64;
                                if nat_eq(a1688, a1689) {
                                    f490(v1)
                                } else {
                                    f491(v1)
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f430(v0: Str, v1: List<(Str, Adt6)>, v2: Str, v3: List<u64>, v4: List<Str>, v5: Adt112) -> R<Adt141> {
    let m1690 = v3.clone();
    match m1690.uncons() {
        None => {
            Ok(Adt141::C0(Adt93::C8(v0)))
        }
        Some((_, _)) => {
            if f207(v3.clone(), true) {
                let m1691 = f492(v1);
                match m1691 {
                    None => {
                        Ok(Adt141::C0(Adt93::C11(v0)))
                    }
                    Some(v8) => {
                        let v9: List<u64> = f493(v3)?;
                        if f494(v9.clone(), match v5.clone() {
                            Adt112::C0(_, h1692) => {
                                h1692
                            }
                        }) {
                            Ok(Adt141::C0(Adt93::C12(v0, v2)))
                        } else {
                            Ok(Adt141::C1(Adt112::C0(List::cons(Adt12::C0(v0, v2, f495(v4, List::<Str>::nil()), v8), match v5.clone() {
                                Adt112::C0(h1693, _) => {
                                    h1693
                                }
                            }), List::cons(v9, match v5 {
                                Adt112::C0(_, h1694) => {
                                    h1694
                                }
                            }))))
                        }
                    }
                }
            } else {
                Ok(Adt141::C0(Adt93::C9(v0)))
            }
        }
    }
}

pub fn f431(p1699: List<(Str, Adt6)>, v1: bool, v2: List<Str>, p1700: List<Adt105>) -> List<Adt105> {
    let m1698 = (p1699, p1700);
    {
        let mut m1698 = m1698;
        loop {
            let step = {
                let (v0, v3) = m1698;
                let m1695 = v0;
                match m1695.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Adt105>), List<Adt105>>(v3)
                    }
                    Some((v4, v5)) => {
                        Ok::<(List<(Str, Adt6)>, List<Adt105>), List<Adt105>>((v5, List::cons(Adt105::C0({
                            let (_, h1696) = v4;
                            h1696
                        }, v1, List::cons({
                            let (h1697, _) = v4;
                            h1697
                        }, v2.clone())), v3)))
                    }
                }
            };
            match step {
                Ok(next) => { m1698 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f432(p1704: List<(Str, Adt6)>, p1705: List<Adt105>) -> List<Adt105> {
    let m1703 = (p1704, p1705);
    {
        let mut m1703 = m1703;
        loop {
            let step = {
                let (v0, v1) = m1703;
                let m1701 = v0;
                match m1701.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Adt105>), List<Adt105>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Adt6)>, List<Adt105>), List<Adt105>>((v3, List::cons(Adt105::C0({
                            let (_, h1702) = v2;
                            h1702
                        }, false, List::<Str>::nil()), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1703 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f433(p1708: List<Adt6>, p1709: List<Adt105>) -> List<Adt105> {
    let m1707 = (p1708, p1709);
    {
        let mut m1707 = m1707;
        loop {
            let step = {
                let (v0, v1) = m1707;
                let m1706 = v0;
                match m1706.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<Adt105>), List<Adt105>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt6>, List<Adt105>), List<Adt105>>((v3, List::cons(Adt105::C0(v2, false, List::<Str>::nil()), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1707 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f434(v0: Adt6) -> Str {
    let m1710 = v0;
    match m1710 {
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

pub fn f435(v0: Adt7) -> R<bool> {
    if f479(match v0.clone() {
        Adt7::C0(h1711, _) => {
            h1711
        }
    }) {
        Ok(true)
    } else {
        f496(40u64, match v0.clone() {
            Adt7::C0(h1712, _) => {
                h1712
            }
        }, match v0 {
            Adt7::C0(_, h1713) => {
                h1713
            }
        })
    }
}

pub fn f436(v0: Adt6) -> Str {
    let m1714 = f122(v0);
    match m1714 {
        None => {
            Str::lit("")
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f437(p1721: u64, p1722: List<(Adt6, Adt6)>) -> R<Adt146> {
    let m1720 = (p1721, p1722);
    {
        let mut m1720 = m1720;
        loop {
            let step = {
                let (v0, v1) = m1720;
                let m1715 = v0;
                if m1715 == 0 {
                    Err::<(u64, List<(Adt6, Adt6)>), R<Adt146>>(Ok(Adt146::C2))
                } else {
                    let v2 = m1715 - 1;
                    let m1716 = v1;
                    match m1716.uncons() {
                        None => {
                            Err::<(u64, List<(Adt6, Adt6)>), R<Adt146>>(Ok(Adt146::C0))
                        }
                        Some((v3, v4)) => {
                            let m1719 = f497({
                                let (h1717, _) = v3;
                                h1717
                            }, {
                                let (_, h1718) = v3;
                                h1718
                            }, v4)?;
                            match m1719 {
                                Adt152::C0 => {
                                    Err::<(u64, List<(Adt6, Adt6)>), R<Adt146>>(Ok(Adt146::C1))
                                }
                                Adt152::C1(v5) => {
                                    Ok::<(u64, List<(Adt6, Adt6)>), R<Adt146>>((v2, v5))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1720 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f438() -> u64 {
    67108864u64
}

pub fn f439(v0: Adt147) -> bool {
    let m1723 = v0;
    match m1723 {
        Adt147::C0 => {
            true
        }
        Adt147::C1 => {
            false
        }
        Adt147::C2 => {
            false
        }
    }
}

pub fn f440(v0: Adt7, v1: Adt7, v2: Adt147, v3: Adt147) -> R<Adt147> {
    let m1724 = v2;
    match m1724 {
        Adt147::C0 => {
            let m1726 = v3;
            match m1726 {
                Adt147::C0 => {
                    Ok(f498(Adt147::C0, f499(f500(match v0.clone() {
                        Adt7::C0(h1727, _) => {
                            h1727
                        }
                    })?, match v0 {
                        Adt7::C0(_, h1728) => {
                            h1728
                        }
                    }, f500(match v1.clone() {
                        Adt7::C0(h1729, _) => {
                            h1729
                        }
                    })?, match v1 {
                        Adt7::C0(_, h1730) => {
                            h1730
                        }
                    })?))
                }
                Adt147::C1 => {
                    Ok(Adt147::C0)
                }
                Adt147::C2 => {
                    Ok(Adt147::C0)
                }
            }
        }
        Adt147::C1 => {
            let m1725 = v3;
            match m1725 {
                Adt147::C0 => {
                    Ok(Adt147::C2)
                }
                Adt147::C1 => {
                    Ok(Adt147::C1)
                }
                Adt147::C2 => {
                    Ok(Adt147::C0)
                }
            }
        }
        Adt147::C2 => {
            let m1731 = v3;
            match m1731 {
                Adt147::C0 => {
                    Ok(Adt147::C2)
                }
                Adt147::C1 => {
                    Ok(Adt147::C2)
                }
                Adt147::C2 => {
                    f499(match v0.clone() {
                        Adt7::C0(h1732, _) => {
                            h1732
                        }
                    }, match v0 {
                        Adt7::C0(_, h1733) => {
                            h1733
                        }
                    }, match v1.clone() {
                        Adt7::C0(h1734, _) => {
                            h1734
                        }
                    }, match v1 {
                        Adt7::C0(_, h1735) => {
                            h1735
                        }
                    })
                }
            }
        }
    }
}

pub fn f441(v0: i64) -> Adt147 {
    f443(v0, 0i64)
}

pub fn f442(v0: Adt6) -> Option<i64> {
    let m1736 = v0;
    match m1736 {
        Adt6::C0 => {
            None::<i64>
        }
        Adt6::C1(_) => {
            None::<i64>
        }
        Adt6::C2(v2) => {
            if f479(match v2.clone() {
                Adt7::C0(_, h1737) => {
                    h1737
                }
            }) {
                Some(match v2 {
                    Adt7::C0(h1738, _) => {
                        h1738
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

pub fn f443(v0: i64, v1: i64) -> Adt147 {
    let m1741 = {
        let a1739 = v0;
        let a1740 = v1;
        compare(a1739, a1740)
    };
    if match m1741 {
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
        Adt147::C0
    } else {
        let m1744 = {
            let a1742 = v1;
            let a1743 = v0;
            compare(a1742, a1743)
        };
        if match m1744 {
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
            Adt147::C2
        } else {
            Adt147::C1
        }
    }
}

pub fn f444(p1753: List<u64>, p1754: u64) -> R<u64> {
    let m1752 = (p1753, p1754);
    {
        let mut m1752 = m1752;
        loop {
            let step = {
                let (v0, v1) = m1752;
                let m1745 = v0;
                match m1745.uncons() {
                    None => {
                        Err::<(List<u64>, u64), R<u64>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        let a1746 = v2;
                        let a1747 = 128u64;
                        if if nat_lt(a1746, a1747) {
                            true
                        } else {
                            let a1748 = 191u64;
                            let a1749 = v2;
                            nat_lt(a1748, a1749)
                        } {
                            Ok::<(List<u64>, u64), R<u64>>((v3, {
                                let a1750 = v1;
                                let a1751 = 1u64;
                                nat_add(a1750, a1751)?
                            }))
                        } else {
                            Ok::<(List<u64>, u64), R<u64>>((v3, v1))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1752 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f445(v0: Str, p1772: List<(Str, Adt6)>) -> Option<Adt6> {
    let m1771 = p1772;
    {
        let mut m1771 = m1771;
        loop {
            let step = {
                let v1 = m1771;
                let m1755 = v1;
                match m1755.uncons() {
                    None => {
                        Err::<List<(Str, Adt6)>, Option<Adt6>>(None::<Adt6>)
                    }
                    Some((v2, v3)) => {
                        let a1759 = v0.clone();
                        let a1760 = {
                            let a1757 = List::cons(Str::lit("#/$defs/"), List::cons({
                                let (h1756, _) = v2.clone();
                                h1756
                            }, List::<Str>::nil()));
                            let a1758 = Str::lit("");
                            join(a1757, a1758)
                        };
                        if equal(a1759, a1760) {
                            Err::<List<(Str, Adt6)>, Option<Adt6>>(Some({
                                let (_, h1761) = v2;
                                h1761
                            }))
                        } else {
                            let m1763 = f213({
                                let (_, h1762) = v2.clone();
                                h1762
                            });
                            match m1763 {
                                None => {
                                    Ok::<List<(Str, Adt6)>, Option<Adt6>>(v3)
                                }
                                Some(v4) => {
                                    let m1764 = f173(Str::lit("$anchor"), v4);
                                    match m1764 {
                                        Adt114::C0 => {
                                            Ok::<List<(Str, Adt6)>, Option<Adt6>>(v3)
                                        }
                                        Adt114::C1(v5) => {
                                            let m1765 = f122(v5);
                                            match m1765 {
                                                None => {
                                                    Ok::<List<(Str, Adt6)>, Option<Adt6>>(v3)
                                                }
                                                Some(v6) => {
                                                    let a1768 = v0.clone();
                                                    let a1769 = {
                                                        let a1766 = List::cons(Str::lit("#"), List::cons(v6, List::<Str>::nil()));
                                                        let a1767 = Str::lit("");
                                                        join(a1766, a1767)
                                                    };
                                                    if equal(a1768, a1769) {
                                                        Err::<List<(Str, Adt6)>, Option<Adt6>>(Some({
                                                            let (_, h1770) = v2;
                                                            h1770
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
                Ok(next) => { m1771 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f446(v0: u64) -> R<Option<u64>> {
    let a1773 = 64u64;
    let a1774 = v0;
    if if nat_lt(a1773, a1774) {
        let a1775 = v0;
        let a1776 = 91u64;
        nat_lt(a1775, a1776)
    } else {
        false
    } {
        Ok(Some({
            let a1777 = v0;
            let a1778 = 65u64;
            nat_sub(a1777, a1778)
        }))
    } else {
        let a1779 = 96u64;
        let a1780 = v0;
        if if nat_lt(a1779, a1780) {
            let a1781 = v0;
            let a1782 = 123u64;
            nat_lt(a1781, a1782)
        } else {
            false
        } {
            Ok(Some({
                let a1783 = v0;
                let a1784 = 71u64;
                nat_sub(a1783, a1784)
            }))
        } else {
            let a1785 = 47u64;
            let a1786 = v0;
            if if nat_lt(a1785, a1786) {
                let a1787 = v0;
                let a1788 = 58u64;
                nat_lt(a1787, a1788)
            } else {
                false
            } {
                Ok(Some({
                    let a1789 = v0;
                    let a1790 = 4u64;
                    nat_add(a1789, a1790)?
                }))
            } else {
                let a1791 = v0;
                let a1792 = 43u64;
                if nat_eq(a1791, a1792) {
                    Ok(Some(62u64))
                } else {
                    let a1793 = v0;
                    let a1794 = 47u64;
                    if nat_eq(a1793, a1794) {
                        Ok(Some(63u64))
                    } else {
                        Ok(None::<u64>)
                    }
                }
            }
        }
    }
}

pub fn f447(v0: u64, v1: u64, v2: bool, v3: u64) -> R<Option<List<u64>>> {
    if if v2 {
        let a1795 = v3;
        let a1796 = 61u64;
        if nat_eq(a1795, a1796) {
            let a1797 = f501(v1, 16u64);
            let a1798 = 0u64;
            nat_eq(a1797, a1798)
        } else {
            false
        }
    } else {
        false
    } {
        Ok(Some(f502(v0, v1)?))
    } else {
        Ok(None::<List<u64>>)
    }
}

pub fn f448(v0: u64, v1: u64, v2: u64, v3: bool) -> R<Option<List<u64>>> {
    if if v3 {
        let a1799 = f501(v2, 4u64);
        let a1800 = 0u64;
        nat_eq(a1799, a1800)
    } else {
        false
    } {
        Ok(Some(f503(v0, v1, v2)?))
    } else {
        Ok(None::<List<u64>>)
    }
}

pub fn f449(v0: u64, v1: u64, v2: u64, v3: u64) -> R<List<u64>> {
    Ok(List::cons({
        let a1803 = {
            let a1801 = v0;
            let a1802 = 4u64;
            nat_mul(a1801, a1802)?
        };
        let a1804 = f504(v1, 16u64);
        nat_add(a1803, a1804)?
    }, List::cons({
        let a1807 = {
            let a1805 = f501(v1, 16u64);
            let a1806 = 16u64;
            nat_mul(a1805, a1806)?
        };
        let a1808 = f504(v2, 4u64);
        nat_add(a1807, a1808)?
    }, List::cons({
        let a1811 = {
            let a1809 = f501(v2, 4u64);
            let a1810 = 64u64;
            nat_mul(a1809, a1810)?
        };
        let a1812 = v3;
        nat_add(a1811, a1812)?
    }, List::<u64>::nil()))))
}

pub fn f450(v0: u64) -> List<u64> {
    f420(v0)
}

pub fn f451(v0: Adt7) -> R<bool> {
    let a1830 = {
        let m1816 = {
            let a1814 = f505();
            let a1815 = match v0.clone() {
                Adt7::C0(h1813, _) => {
                    h1813
                }
            };
            compare(a1814, a1815)
        };
        if match m1816 {
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
            let m1821 = {
                let a1819 = match v0.clone() {
                    Adt7::C0(h1817, _) => {
                        h1817
                    }
                };
                let a1820 = {
                    let a1818 = f505();
                    int_neg(a1818)?
                };
                compare(a1819, a1820)
            };
            if match m1821 {
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
                let m1825 = {
                    let a1823 = 9999i64;
                    let a1824 = match v0.clone() {
                        Adt7::C0(_, h1822) => {
                            h1822
                        }
                    };
                    compare(a1823, a1824)
                };
                if match m1825 {
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
                    let m1829 = {
                        let a1827 = match v0 {
                            Adt7::C0(_, h1826) => {
                                h1826
                            }
                        };
                        let a1828 = -9999i64;
                        compare(a1827, a1828)
                    };
                    match m1829 {
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
    Ok(bool_not(a1830))
}

pub fn f452(v0: Adt7) -> R<List<u64>> {
    let m1834 = {
        let a1832 = match v0.clone() {
            Adt7::C0(_, h1831) => {
                h1831
            }
        };
        let a1833 = 0i64;
        compare(a1832, a1833)
    };
    if if match m1834 {
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
        let m1838 = {
            let a1836 = 0i64;
            let a1837 = match v0.clone() {
                Adt7::C0(_, h1835) => {
                    h1835
                }
            };
            compare(a1836, a1837)
        };
        match m1838 {
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
        let a1841 = f506(match v0.clone() {
            Adt7::C0(h1839, _) => {
                h1839
            }
        })?;
        let a1842 = List::cons(101u64, f506(match v0 {
            Adt7::C0(_, h1840) => {
                h1840
            }
        })?);
        Ok(append_list(a1841, a1842))
    } else {
        f506(match v0 {
            Adt7::C0(h1843, _) => {
                h1843
            }
        })
    }
}

pub fn f453() -> u64 {
    512u64
}

pub fn f454(v0: u64, v1: Adt116, v2: List<Adt116>) -> List<Adt116> {
    List::cons(Adt116::C0(List::cons(v0, List::<u64>::nil())), List::cons(v1, v2))
}

pub fn f455(p1846: List<u64>, p1847: List<u64>) -> R<List<u64>> {
    let m1845 = (p1846, p1847);
    {
        let mut m1845 = m1845;
        loop {
            let step = {
                let (v0, v1) = m1845;
                let m1844 = v0;
                match m1844.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), R<List<u64>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), R<List<u64>>>((v3, f267(f507(v2)?, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1845 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f456() -> u64 {
    4294967296u64
}

pub fn f457(v0: Adt34) -> R<Option<List<(Str, Adt6)>>> {
    let m1848 = v0;
    match m1848 {
        Adt34::C0(v1) => {
            Ok(Some(f508(v1)?))
        }
        Adt34::C1(v2) => {
            Ok(Some(f509(v2)?))
        }
        Adt34::C2(v3) => {
            f510(f511(v3)?)
        }
        Adt34::C3(v4) => {
            f510(f511(match v4 {
                Adt51::C0(h1849, _) => {
                    h1849
                }
            })?)
        }
        Adt34::C4(v5) => {
            Ok(Some(f512(v5)?))
        }
        Adt34::C5(v6) => {
            Ok(Some(f513(v6)?))
        }
        Adt34::C6(v7) => {
            Ok(Some(f514(v7)?))
        }
        Adt34::C7(v8) => {
            Ok(Some(f515(v8)?))
        }
        Adt34::C8(v9) => {
            f510(f516(v9)?)
        }
        Adt34::C9(v10) => {
            Ok(Some(f517(v10)?))
        }
        Adt34::C10(_) => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Adt34::C11 => {
            Ok(Some(List::<(Str, Adt6)>::nil()))
        }
        Adt34::C12(v12) => {
            f510(f518(v12)?)
        }
        Adt34::C13(_) => {
            Ok(None::<List<(Str, Adt6)>>)
        }
    }
}

pub fn f458(v0: Adt133) -> Adt5 {
    Adt5::C0(List::cons((Str::lit("resultType"), Adt6::C3(f519(f520(match v0.clone() {
        Adt133::C0(h1850, _, _) => {
            h1850
        }
    })))), List::cons((Str::lit("_meta"), Adt6::C5(f521({
        let m1853 = match v0 {
            Adt133::C0(_, h1852, _) => {
                h1852
            }
        };
        match m1853 {
            Adt134::C0(h1851, _) => {
                h1851
            }
        }
    }))), List::<(Str, Adt6)>::nil())))
}

pub fn f459(v0: bool, v1: Adt95, v2: List<List<u64>>) -> Adt95 {
    if f522(v0, match v1.clone() {
        Adt95::C0(h1854, _, _, _) => {
            h1854
        }
    }, v2.clone()) {
        f523(match v1.clone() {
            Adt95::C0(h1855, _, _, _) => {
                h1855
            }
        }, {
            let a1857 = v2;
            let a1858 = List::cons(match v1 {
                Adt95::C0(_, _, _, h1856) => {
                    h1856
                }
            }, List::<List<u64>>::nil());
            append_list(a1857, a1858)
        })
    } else {
        v1
    }
}

pub fn f460(v0: Adt135) -> R<Option<List<u64>>> {
    let m1859 = f524(f525(v0))?;
    match m1859 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v1) => {
            let m1860 = f183(v1)?;
            match m1860 {
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

pub fn f461() -> u64 {
    404u64
}

pub fn f462(v0: Adt86) -> u64 {
    let m1861 = v0;
    match m1861 {
        Adt86::C0(_) => {
            f190()
        }
        Adt86::C1(_) => {
            f190()
        }
        Adt86::C2(_) => {
            f190()
        }
        Adt86::C3(_) => {
            f190()
        }
        Adt86::C4(_) => {
            f190()
        }
        Adt86::C5(_) => {
            f190()
        }
        Adt86::C6(_) => {
            f62()
        }
        Adt86::C7(_) => {
            f526()
        }
        Adt86::C8(_) => {
            f190()
        }
        Adt86::C9(_) => {
            f62()
        }
        Adt86::C10 => {
            f190()
        }
        Adt86::C11 => {
            f190()
        }
        Adt86::C12(_) => {
            f62()
        }
        Adt86::C13 => {
            f62()
        }
        Adt86::C14 => {
            f62()
        }
        Adt86::C15(_) => {
            f62()
        }
    }
}

pub fn f463(v0: Adt136) -> i64 {
    let m1862 = v0;
    match m1862 {
        Adt136::C0(_) => {
            f527()
        }
        Adt136::C1(_, _) => {
            f528()
        }
        Adt136::C2(_) => {
            f529()
        }
        Adt136::C3(_) => {
            f530()
        }
        Adt136::C4(v6) => {
            f531(v6)
        }
    }
}

pub fn f464(v0: Adt136) -> Str {
    let m1863 = v0;
    match m1863 {
        Adt136::C0(v1) => {
            f532(v1)
        }
        Adt136::C1(_, _) => {
            Str::lit("Unsupported protocol version")
        }
        Adt136::C2(v4) => {
            f533(v4)
        }
        Adt136::C3(_) => {
            Str::lit("Method not found")
        }
        Adt136::C4(v6) => {
            let m1864 = v6;
            match m1864 {
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

pub fn f465(v0: Adt136) -> R<Option<Adt6>> {
    let m1865 = v0;
    match m1865 {
        Adt136::C0(_) => {
            Ok(None::<Adt6>)
        }
        Adt136::C1(v2, v3) => {
            Ok(Some(Adt6::C5(List::cons((Str::lit("supported"), Adt6::C4(f534(v3))), List::cons((Str::lit("requested"), Adt6::C3(v2)), List::<(Str, Adt6)>::nil())))))
        }
        Adt136::C2(_) => {
            Ok(None::<Adt6>)
        }
        Adt136::C3(_) => {
            Ok(None::<Adt6>)
        }
        Adt136::C4(v6) => {
            let m1866 = v6;
            match m1866 {
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
                    Ok(Some(Adt6::C5(List::cons((Str::lit("limit"), Adt6::C2(Adt7::C0(f136(v14, 0i64)?, 0i64))), List::<(Str, Adt6)>::nil()))))
                }
                Adt86::C8(v15) => {
                    Ok(Some(Adt6::C5(List::cons((Str::lit("requiredCapabilities"), f535(v15)), List::<(Str, Adt6)>::nil()))))
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

pub fn f466(p1869: List<List<u64>>) -> R<bool> {
    let m1868 = p1869;
    {
        let mut m1868 = m1868;
        loop {
            let step = {
                let v0 = m1868;
                let m1867 = v0;
                match m1867.uncons() {
                    None => {
                        Err::<List<List<u64>>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        if f536(f246(v1))? {
                            Err::<List<List<u64>>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<List<u64>>, R<bool>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1868 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f467() -> List<u64> {
    List::cons(116u64, List::cons(101u64, List::cons(120u64, List::cons(116u64, List::cons(47u64, List::cons(101u64, List::cons(118u64, List::cons(101u64, List::cons(110u64, List::cons(116u64, List::cons(45u64, List::cons(115u64, List::cons(116u64, List::cons(114u64, List::cons(101u64, List::cons(97u64, List::cons(109u64, List::<u64>::nil())))))))))))))))))
}

pub fn f468(v0: Adt0, v1: Fn2, v2: u64, v3: Adt143, v4: bool) -> R<Result<Adt133, Adt136>> {
    if v4 {
        f537(v0, v1, v2, f538(match v3.clone() {
            Adt143::C0(_, h1870, _, _) => {
                h1870
            }
        }, match v3.clone() {
            Adt143::C0(_, _, h1871, _) => {
                h1871
            }
        }, match v3 {
            Adt143::C0(_, _, _, h1872) => {
                h1872
            }
        }))
    } else {
        Ok(Err::<Adt133, Adt136>(Adt136::C3(f539(match v3 {
            Adt143::C0(_, h1873, _, _) => {
                h1873
            }
        }))))
    }
}

pub fn f469(v0: Adt4, v1: Adt144) -> bool {
    let m1874 = v1;
    match m1874 {
        Adt144::C0 => {
            true
        }
        Adt144::C1 => {
            f540(match v0 {
                Adt4::C0(_, _, _, _, _, h1875, _) => {
                    h1875
                }
            })
        }
        Adt144::C2 => {
            f540(match v0 {
                Adt4::C0(_, _, _, _, _, h1876, _) => {
                    h1876
                }
            })
        }
        Adt144::C3 => {
            f541(match v0 {
                Adt4::C0(_, _, _, _, h1877, _, _) => {
                    h1877
                }
            })
        }
        Adt144::C4 => {
            f541(match v0 {
                Adt4::C0(_, _, _, _, h1878, _, _) => {
                    h1878
                }
            })
        }
        Adt144::C5 => {
            f541(match v0 {
                Adt4::C0(_, _, _, _, h1879, _, _) => {
                    h1879
                }
            })
        }
        Adt144::C6 => {
            f540(match v0 {
                Adt4::C0(_, _, _, h1880, _, _, _) => {
                    h1880
                }
            })
        }
        Adt144::C7 => {
            f540(match v0 {
                Adt4::C0(_, _, _, h1881, _, _, _) => {
                    h1881
                }
            })
        }
        Adt144::C8 => {
            f542(match v0 {
                Adt4::C0(_, _, h1882, _, _, _, _) => {
                    h1882
                }
            })
        }
        Adt144::C9 => {
            f543(match v0 {
                Adt4::C0(_, _, _, _, _, _, h1883) => {
                    h1883
                }
            }, f544())
        }
        Adt144::C10 => {
            f543(match v0 {
                Adt4::C0(_, _, _, _, _, _, h1884) => {
                    h1884
                }
            }, f544())
        }
        Adt144::C11 => {
            f543(match v0 {
                Adt4::C0(_, _, _, _, _, _, h1885) => {
                    h1885
                }
            }, f544())
        }
    }
}

pub fn f470(v0: Adt150, v1: Adt148) -> Result<Adt143, Adt136> {
    let m1886 = v1;
    match m1886 {
        Adt148::C0 => {
            Err::<Adt143, Adt136>(Adt136::C3(match v0 {
                Adt150::C0(_, h1887, _, _) => {
                    h1887
                }
            }))
        }
        Adt148::C1(v2) => {
            Ok::<Adt143, Adt136>(Adt143::C0(match v0.clone() {
                Adt150::C0(h1888, _, _, _) => {
                    h1888
                }
            }, v2, match v0.clone() {
                Adt150::C0(_, _, h1889, _) => {
                    h1889
                }
            }, match v0 {
                Adt150::C0(_, _, _, h1890) => {
                    h1890
                }
            }))
        }
    }
}

pub fn f471(v0: Option<Adt117>, v1: List<Str>, v2: Adt115, v3: Result<Adt18, Adt137>) -> R<Adt149> {
    let m1891 = v3;
    match m1891 {
        Ok(v5) => {
            Ok(f545(v1, v2.clone(), v5.clone(), f546(v0, f399(match v2.clone() {
                Adt115::C0(_, h1892, _) => {
                    h1892
                }
            }), match v2.clone() {
                Adt115::C0(_, h1893, _) => {
                    h1893
                }
            }, match v5 {
                Adt18::C0(h1894, _, _, _, _, _) => {
                    h1894
                }
            }, f547(match v2 {
                Adt115::C0(_, _, h1895) => {
                    h1895
                }
            }))?))
        }
        Err(v4) => {
            Ok(Adt149::C1(Adt136::C0(v4)))
        }
    }
}

pub fn f472(v0: Option<Adt5>) -> Result<Adt18, Adt137> {
    let m1896 = v0;
    match m1896 {
        None => {
            Err::<Adt18, Adt137>(Adt137::C0)
        }
        Some(v1) => {
            let m1897 = f173(Str::lit("_meta"), v1);
            match m1897 {
                Adt114::C0 => {
                    Err::<Adt18, Adt137>(Adt137::C1)
                }
                Adt114::C1(v2) => {
                    let m1898 = f213(v2);
                    match m1898 {
                        None => {
                            Err::<Adt18, Adt137>(Adt137::C4)
                        }
                        Some(v3) => {
                            f548(v3)
                        }
                    }
                }
            }
        }
    }
}

pub fn f473(v0: Adt144) -> Adt151 {
    let m1899 = v0;
    match m1899 {
        Adt144::C0 => {
            Adt151::C0
        }
        Adt144::C1 => {
            Adt151::C0
        }
        Adt144::C2 => {
            Adt151::C1
        }
        Adt144::C3 => {
            Adt151::C0
        }
        Adt144::C4 => {
            Adt151::C2
        }
        Adt144::C5 => {
            Adt151::C0
        }
        Adt144::C6 => {
            Adt151::C0
        }
        Adt144::C7 => {
            Adt151::C1
        }
        Adt144::C8 => {
            Adt151::C0
        }
        Adt144::C9 => {
            Adt151::C3
        }
        Adt144::C10 => {
            Adt151::C3
        }
        Adt144::C11 => {
            Adt151::C3
        }
    }
}

pub fn f474(p1905: List<u64>, p1906: List<u64>) -> Option<List<u64>> {
    let m1904 = (p1905, p1906);
    {
        let mut m1904 = m1904;
        loop {
            let step = {
                let (v0, v1) = m1904;
                let m1900 = v0;
                match m1900.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), Option<List<u64>>>(Some(v1))
                    }
                    Some((v2, v3)) => {
                        let m1901 = v1;
                        match m1901.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), Option<List<u64>>>(None::<List<u64>>)
                            }
                            Some((v4, v5)) => {
                                let a1902 = v4;
                                let a1903 = v2;
                                if nat_eq(a1902, a1903) {
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
                Ok(next) => { m1904 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f475() -> List<u64> {
    List::cons(61u64, List::cons(63u64, List::cons(98u64, List::cons(97u64, List::cons(115u64, List::cons(101u64, List::cons(54u64, List::cons(52u64, List::cons(63u64, List::<u64>::nil())))))))))
}

pub fn f476() -> List<u64> {
    List::cons(61u64, List::cons(63u64, List::<u64>::nil()))
}

pub fn f477(v0: Adt12, v1: Option<Str>) -> bool {
    let m1907 = v1;
    match m1907 {
        None => {
            false
        }
        Some(v2) => {
            let a1909 = match v0 {
                Adt12::C0(h1908, _, _, _) => {
                    h1908
                }
            };
            let a1910 = v2;
            equal(a1909, a1910)
        }
    }
}

pub fn f478(v0: List<Adt92>, v1: Str) -> R<Adt121> {
    let m1913 = f324(f151(v0, {
        let a1911 = f549();
        let a1912 = f80(v1)?;
        append_list(a1911, a1912)
    })?)?;
    match m1913 {
        Adt110::C0 => {
            Ok(Adt121::C0)
        }
        Adt110::C1 => {
            Ok(Adt121::C1)
        }
        Adt110::C2 => {
            Ok(Adt121::C1)
        }
        Adt110::C3(v2) => {
            let m1914 = f398(v2);
            match m1914 {
                None => {
                    Ok(Adt121::C1)
                }
                Some(v3) => {
                    Ok(Adt121::C2(v3))
                }
            }
        }
    }
}

pub fn f479(v0: i64) -> bool {
    let a1921 = {
        let m1917 = {
            let a1915 = v0;
            let a1916 = 0i64;
            compare(a1915, a1916)
        };
        if match m1917 {
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
            let m1920 = {
                let a1918 = 0i64;
                let a1919 = v0;
                compare(a1918, a1919)
            };
            match m1920 {
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
    bool_not(a1921)
}

pub fn f480(v0: Adt124, v1: u64, v2: Str) -> Adt140 {
    let m1923 = match v0.clone() {
        Adt124::C0(_, h1922, _) => {
            h1922
        }
    };
    match m1923.uncons() {
        None => {
            f343(Adt107::C2(v1))
        }
        Some((v3, v4)) => {
            let m1924 = v3;
            match m1924 {
                Adt129::C0(_) => {
                    f343(Adt107::C2(v1))
                }
                Adt129::C1(v6, _) => {
                    Adt140::C1(Adt124::C0(Adt125::C5, List::cons(Adt129::C1(v6, Some(v2)), v4), match v0 {
                        Adt124::C0(_, _, h1925) => {
                            h1925
                        }
                    }))
                }
            }
        }
    }
}

pub fn f481(v0: u64, v1: List<u64>) -> R<List<u64>> {
    let a1926 = v0;
    let a1927 = 128u64;
    if nat_lt(a1926, a1927) {
        Ok(List::cons(v0, v1))
    } else {
        let a1928 = v0;
        let a1929 = 2048u64;
        if nat_lt(a1928, a1929) {
            Ok(List::cons({
                let a1930 = 128u64;
                let a1931 = f550(v0, 64u64);
                nat_add(a1930, a1931)?
            }, List::cons({
                let a1932 = 192u64;
                let a1933 = f551(v0, 64u64);
                nat_add(a1932, a1933)?
            }, v1)))
        } else {
            let a1934 = v0;
            let a1935 = 65536u64;
            if nat_lt(a1934, a1935) {
                Ok(List::cons({
                    let a1936 = 128u64;
                    let a1937 = f550(v0, 64u64);
                    nat_add(a1936, a1937)?
                }, List::cons({
                    let a1938 = 128u64;
                    let a1939 = f550(f551(v0, 64u64), 64u64);
                    nat_add(a1938, a1939)?
                }, List::cons({
                    let a1940 = 224u64;
                    let a1941 = f551(v0, 4096u64);
                    nat_add(a1940, a1941)?
                }, v1))))
            } else {
                Ok(List::cons({
                    let a1942 = 128u64;
                    let a1943 = f550(v0, 64u64);
                    nat_add(a1942, a1943)?
                }, List::cons({
                    let a1944 = 128u64;
                    let a1945 = f550(f551(v0, 64u64), 64u64);
                    nat_add(a1944, a1945)?
                }, List::cons({
                    let a1946 = 128u64;
                    let a1947 = f550(f551(v0, 4096u64), 64u64);
                    nat_add(a1946, a1947)?
                }, List::cons({
                    let a1948 = 240u64;
                    let a1949 = f551(v0, 262144u64);
                    nat_add(a1948, a1949)?
                }, v1)))))
            }
        }
    }
}

pub fn f482() -> u64 {
    18u64
}

pub fn f483(v0: Adt6) -> bool {
    let m1950 = f122(v0);
    match m1950 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f484(v0: Adt6) -> bool {
    let m1951 = f363(v0);
    match m1951 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f485(v0: Adt6) -> bool {
    let m1952 = f442(v0);
    match m1952 {
        None => {
            false
        }
        Some(v1) => {
            let a1956 = {
                let m1955 = {
                    let a1953 = v1;
                    let a1954 = 0i64;
                    compare(a1953, a1954)
                };
                match m1955 {
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
            bool_not(a1956)
        }
    }
}

pub fn f486(v0: Adt6) -> bool {
    let m1957 = f174(v0);
    match m1957 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f487(v0: Adt6) -> bool {
    let m1958 = f213(v0);
    match m1958 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f488(v0: Adt6) -> bool {
    let m1959 = f174(v0);
    match m1959 {
        None => {
            false
        }
        Some(v1) => {
            f552(v1)
        }
    }
}

pub fn f489(v0: Adt6) -> bool {
    let m1960 = f127(v0);
    match m1960 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f490(v0: Adt6) -> bool {
    let m1961 = f122(v0.clone());
    match m1961 {
        None => {
            let m1962 = f174(v0);
            match m1962 {
                None => {
                    false
                }
                Some(v2) => {
                    f553(v2)
                }
            }
        }
        Some(v1) => {
            f358(v1)
        }
    }
}

pub fn f491(v0: Adt6) -> bool {
    let m1963 = f174(v0);
    match m1963 {
        None => {
            false
        }
        Some(v1) => {
            let m1964 = v1;
            match m1964.uncons() {
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

pub fn f492(v0: List<(Str, Adt6)>) -> Option<Adt13> {
    let m1965 = f227(Str::lit("type"), v0);
    match m1965 {
        Adt114::C0 => {
            None::<Adt13>
        }
        Adt114::C1(v1) => {
            let m1966 = f122(v1);
            match m1966 {
                None => {
                    None::<Adt13>
                }
                Some(v2) => {
                    f554(v2)
                }
            }
        }
    }
}

pub fn f493(v0: List<u64>) -> R<List<u64>> {
    Ok(f267(f555(v0, List::<u64>::nil())?, List::<u64>::nil()))
}

pub fn f494(v0: List<u64>, p1969: List<List<u64>>) -> bool {
    let m1968 = p1969;
    {
        let mut m1968 = m1968;
        loop {
            let step = {
                let v1 = m1968;
                let m1967 = v1;
                match m1967.uncons() {
                    None => {
                        Err::<List<List<u64>>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f556(v0.clone(), v2) {
                            Err::<List<List<u64>>, bool>(true)
                        } else {
                            Ok::<List<List<u64>>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1968 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f495(p1972: List<Str>, p1973: List<Str>) -> List<Str> {
    let m1971 = (p1972, p1973);
    {
        let mut m1971 = m1971;
        loop {
            let step = {
                let (v0, v1) = m1971;
                let m1970 = v0;
                match m1970.uncons() {
                    None => {
                        Err::<(List<Str>, List<Str>), List<Str>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Str>, List<Str>), List<Str>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1971 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f496(p1985: u64, p1986: i64, p1987: i64) -> R<bool> {
    let m1984 = (p1985, (p1986, p1987));
    {
        let mut m1984 = m1984;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1984;
                let m1974 = v0;
                if m1974 == 0 {
                    Err::<(u64, (i64, i64)), R<bool>>(Ok(false))
                } else {
                    let v3 = m1974 - 1;
                    let m1977 = {
                        let a1975 = v2;
                        let a1976 = 0i64;
                        compare(a1975, a1976)
                    };
                    if match m1977 {
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
                        if f479({
                            let a1978 = v1;
                            let a1979 = 10i64;
                            let a1980 = 0i64;
                            int_rem(a1978, a1979, a1980)
                        }) {
                            Ok::<(u64, (i64, i64)), R<bool>>((v3, ({
                                let a1981 = v1;
                                let a1982 = 10i64;
                                let a1983 = 0i64;
                                int_quot(a1981, a1982, a1983)?
                            }, f182(v2, 1i64)?)))
                        } else {
                            Err::<(u64, (i64, i64)), R<bool>>(Ok(false))
                        }
                    } else {
                        Err::<(u64, (i64, i64)), R<bool>>(Ok(true))
                    }
                }
            };
            match step {
                Ok(next) => { m1984 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f497(v0: Adt6, v1: Adt6, v2: List<(Adt6, Adt6)>) -> R<Adt152> {
    let a1988 = f557(v0.clone());
    let a1989 = f557(v1.clone());
    if nat_eq(a1988, a1989) {
        let m1990 = v0;
        match m1990 {
            Adt6::C0 => {
                Ok(Adt152::C1(v2))
            }
            Adt6::C1(v3) => {
                let m1991 = f127(v1);
                match m1991 {
                    None => {
                        Ok(Adt152::C0)
                    }
                    Some(v4) => {
                        let a1992 = v3;
                        let a1993 = v4;
                        if equal(a1992, a1993) {
                            Ok(Adt152::C1(v2))
                        } else {
                            Ok(Adt152::C0)
                        }
                    }
                }
            }
            Adt6::C2(v5) => {
                let m1994 = f363(v1);
                match m1994 {
                    None => {
                        Ok(Adt152::C0)
                    }
                    Some(v6) => {
                        if f558(f365(v5, v6)?) {
                            Ok(Adt152::C1(v2))
                        } else {
                            Ok(Adt152::C0)
                        }
                    }
                }
            }
            Adt6::C3(v7) => {
                let m1995 = f122(v1);
                match m1995 {
                    None => {
                        Ok(Adt152::C0)
                    }
                    Some(v8) => {
                        let a1996 = v7;
                        let a1997 = v8;
                        if equal(a1996, a1997) {
                            Ok(Adt152::C1(v2))
                        } else {
                            Ok(Adt152::C0)
                        }
                    }
                }
            }
            Adt6::C4(v9) => {
                let m1998 = f174(v1);
                match m1998 {
                    None => {
                        Ok(Adt152::C0)
                    }
                    Some(v10) => {
                        Ok(f559(f560(v9, v10, List::<(Adt6, Adt6)>::nil()), v2))
                    }
                }
            }
            Adt6::C5(v11) => {
                let m1999 = f213(v1);
                match m1999 {
                    None => {
                        Ok(Adt152::C0)
                    }
                    Some(v12) => {
                        let a2001 = f369(v11.clone(), 0u64)?;
                        let a2002 = f369(match v12.clone() {
                            Adt5::C0(h2000) => {
                                h2000
                            }
                        }, 0u64)?;
                        if nat_eq(a2001, a2002) {
                            Ok(f559(f561(v11, match v12 {
                                Adt5::C0(h2003) => {
                                    h2003
                                }
                            }, List::<(Adt6, Adt6)>::nil()), v2))
                        } else {
                            Ok(Adt152::C0)
                        }
                    }
                }
            }
        }
    } else {
        Ok(Adt152::C0)
    }
}

pub fn f498(v0: Adt147, v1: Adt147) -> Adt147 {
    let m2004 = v0;
    match m2004 {
        Adt147::C0 => {
            f562(v1)
        }
        Adt147::C1 => {
            Adt147::C1
        }
        Adt147::C2 => {
            v1
        }
    }
}

pub fn f499(v0: i64, v1: i64, v2: i64, v3: i64) -> R<Adt147> {
    let m2005 = f443(f563(v0, v1)?, f563(v2, v3)?);
    match m2005 {
        Adt147::C0 => {
            Ok(Adt147::C0)
        }
        Adt147::C1 => {
            f564(v0, f565(v0)?, v2, f565(v2)?)
        }
        Adt147::C2 => {
            Ok(Adt147::C2)
        }
    }
}

pub fn f500(v0: i64) -> R<i64> {
    let m2008 = {
        let a2006 = v0;
        let a2007 = 0i64;
        compare(a2006, a2007)
    };
    if match m2008 {
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
        let a2009 = v0;
        int_neg(a2009)
    } else {
        Ok(v0)
    }
}

pub fn f501(v0: u64, v1: u64) -> u64 {
    let a2010 = v0;
    let a2011 = v1;
    let a2012 = 0u64;
    nat_rem(a2010, a2011, a2012)
}

pub fn f502(v0: u64, v1: u64) -> R<List<u64>> {
    Ok(List::cons({
        let a2015 = {
            let a2013 = v0;
            let a2014 = 4u64;
            nat_mul(a2013, a2014)?
        };
        let a2016 = f504(v1, 16u64);
        nat_add(a2015, a2016)?
    }, List::<u64>::nil()))
}

pub fn f503(v0: u64, v1: u64, v2: u64) -> R<List<u64>> {
    Ok(List::cons({
        let a2019 = {
            let a2017 = v0;
            let a2018 = 4u64;
            nat_mul(a2017, a2018)?
        };
        let a2020 = f504(v1, 16u64);
        nat_add(a2019, a2020)?
    }, List::cons({
        let a2023 = {
            let a2021 = f501(v1, 16u64);
            let a2022 = 16u64;
            nat_mul(a2021, a2022)?
        };
        let a2024 = f504(v2, 4u64);
        nat_add(a2023, a2024)?
    }, List::<u64>::nil())))
}

pub fn f504(v0: u64, v1: u64) -> u64 {
    let a2025 = v0;
    let a2026 = v1;
    let a2027 = 0u64;
    nat_quot(a2025, a2026, a2027)
}

pub fn f505() -> i64 {
    999999999999999999i64
}

pub fn f506(v0: i64) -> R<List<u64>> {
    let m2030 = {
        let a2028 = v0;
        let a2029 = 0i64;
        compare(a2028, a2029)
    };
    if match m2030 {
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
        Ok(List::cons(45u64, f566(20u64, {
            let a2031 = v0;
            int_neg(a2031)?
        }, List::<u64>::nil())?))
    } else {
        f566(20u64, v0, List::<u64>::nil())
    }
}

pub fn f507(v0: u64) -> R<List<u64>> {
    let a2032 = v0;
    let a2033 = 34u64;
    if nat_eq(a2032, a2033) {
        Ok(List::cons(92u64, List::cons(34u64, List::<u64>::nil())))
    } else {
        let a2034 = v0;
        let a2035 = 92u64;
        if nat_eq(a2034, a2035) {
            Ok(List::cons(92u64, List::cons(92u64, List::<u64>::nil())))
        } else {
            let a2036 = v0;
            let a2037 = 8u64;
            if nat_eq(a2036, a2037) {
                Ok(List::cons(92u64, List::cons(98u64, List::<u64>::nil())))
            } else {
                let a2038 = v0;
                let a2039 = 9u64;
                if nat_eq(a2038, a2039) {
                    Ok(List::cons(92u64, List::cons(116u64, List::<u64>::nil())))
                } else {
                    let a2040 = v0;
                    let a2041 = 10u64;
                    if nat_eq(a2040, a2041) {
                        Ok(List::cons(92u64, List::cons(110u64, List::<u64>::nil())))
                    } else {
                        let a2042 = v0;
                        let a2043 = 12u64;
                        if nat_eq(a2042, a2043) {
                            Ok(List::cons(92u64, List::cons(102u64, List::<u64>::nil())))
                        } else {
                            let a2044 = v0;
                            let a2045 = 13u64;
                            if nat_eq(a2044, a2045) {
                                Ok(List::cons(92u64, List::cons(114u64, List::<u64>::nil())))
                            } else {
                                let a2046 = v0;
                                let a2047 = 32u64;
                                if nat_lt(a2046, a2047) {
                                    Ok(List::cons(92u64, List::cons(117u64, List::cons(48u64, List::cons(48u64, List::cons(f567(f551(v0, 16u64))?, List::cons(f567(f550(v0, 16u64))?, List::<u64>::nil())))))))
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

pub fn f508(v0: Adt35) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("supportedVersions"), f568(match v0.clone() {
        Adt35::C0(_, h2048, _, _) => {
            h2048
        }
    })), List::cons((Str::lit("capabilities"), f569(match v0.clone() {
        Adt35::C0(_, _, h2049, _) => {
            h2049
        }
    })), {
        let a2052 = f570(Str::lit("instructions"), match v0.clone() {
            Adt35::C0(_, _, _, h2050) => {
                h2050
            }
        });
        let a2053 = f571(match v0 {
            Adt35::C0(h2051, _, _, _) => {
                h2051
            }
        })?;
        append_list(a2052, a2053)
    })))
}

pub fn f509(v0: Adt36) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("tools"), Adt6::C4(f572(match v0.clone() {
        Adt36::C0(_, _, h2054) => {
            h2054
        }
    }))), {
        let a2057 = f570(Str::lit("nextCursor"), match v0.clone() {
            Adt36::C0(_, h2055, _) => {
                h2055
            }
        });
        let a2058 = f571(match v0 {
            Adt36::C0(h2056, _, _) => {
                h2056
            }
        })?;
        append_list(a2057, a2058)
    }))
}

pub fn f510(v0: Option<List<(Str, Adt6)>>) -> R<Option<List<(Str, Adt6)>>> {
    let m2059 = v0;
    match m2059 {
        None => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Some(v1) => {
            let m2060 = f524(Adt6::C5(v1))?;
            match m2060 {
                None => {
                    Ok(None::<List<(Str, Adt6)>>)
                }
                Some(v2) => {
                    let m2061 = f213(v2);
                    match m2061 {
                        None => {
                            Ok(None::<List<(Str, Adt6)>>)
                        }
                        Some(v3) => {
                            Ok(Some(match v3 {
                                Adt5::C0(h2062) => {
                                    h2062
                                }
                            }))
                        }
                    }
                }
            }
        }
    }
}

pub fn f511(v0: Adt39) -> R<Option<List<(Str, Adt6)>>> {
    let m2064 = f573(match v0.clone() {
        Adt39::C0(h2063, _, _) => {
            h2063
        }
    })?;
    match m2064 {
        None => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Some(v1) => {
            Ok(Some(List::cons((Str::lit("content"), Adt6::C4(v1)), {
                let a2067 = f574(Str::lit("structuredContent"), match v0.clone() {
                    Adt39::C0(_, h2065, _) => {
                        h2065
                    }
                });
                let a2068 = f575(Str::lit("isError"), match v0 {
                    Adt39::C0(_, _, h2066) => {
                        h2066
                    }
                });
                append_list(a2067, a2068)
            })))
        }
    }
}

pub fn f512(v0: Adt53) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("resources"), Adt6::C4(f576(match v0.clone() {
        Adt53::C0(_, _, h2069) => {
            h2069
        }
    })?)), {
        let a2072 = f570(Str::lit("nextCursor"), match v0.clone() {
            Adt53::C0(_, h2070, _) => {
                h2070
            }
        });
        let a2073 = f571(match v0 {
            Adt53::C0(h2071, _, _) => {
                h2071
            }
        })?;
        append_list(a2072, a2073)
    }))
}

pub fn f513(v0: Adt54) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("contents"), Adt6::C4(f577(match v0.clone() {
        Adt54::C0(_, h2074) => {
            h2074
        }
    }))), f571(match v0 {
        Adt54::C0(h2075, _) => {
            h2075
        }
    })?))
}

pub fn f514(v0: Adt55) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("resourceTemplates"), Adt6::C4(f578(match v0.clone() {
        Adt55::C0(_, _, h2076) => {
            h2076
        }
    }))), {
        let a2079 = f570(Str::lit("nextCursor"), match v0.clone() {
            Adt55::C0(_, h2077, _) => {
                h2077
            }
        });
        let a2080 = f571(match v0 {
            Adt55::C0(h2078, _, _) => {
                h2078
            }
        })?;
        append_list(a2079, a2080)
    }))
}

pub fn f515(v0: Adt57) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("prompts"), Adt6::C4(f579(match v0.clone() {
        Adt57::C0(_, _, h2081) => {
            h2081
        }
    }))), {
        let a2084 = f570(Str::lit("nextCursor"), match v0.clone() {
            Adt57::C0(_, h2082, _) => {
                h2082
            }
        });
        let a2085 = f571(match v0 {
            Adt57::C0(h2083, _, _) => {
                h2083
            }
        })?;
        append_list(a2084, a2085)
    }))
}

pub fn f516(v0: Adt60) -> R<Option<List<(Str, Adt6)>>> {
    let m2087 = f580(match v0.clone() {
        Adt60::C0(_, h2086) => {
            h2086
        }
    })?;
    match m2087 {
        None => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Some(v1) => {
            Ok(Some({
                let a2089 = f570(Str::lit("description"), match v0 {
                    Adt60::C0(h2088, _) => {
                        h2088
                    }
                });
                let a2090 = List::cons((Str::lit("messages"), Adt6::C4(v1)), List::<(Str, Adt6)>::nil());
                append_list(a2089, a2090)
            }))
        }
    }
}

pub fn f517(v0: Adt62) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("completion"), Adt6::C5(List::cons((Str::lit("values"), Adt6::C4(f534({
        let m2093 = match v0.clone() {
            Adt62::C0(h2092) => {
                h2092
            }
        };
        match m2093 {
            Adt63::C0(h2091, _, _) => {
                h2091
            }
        }
    }))), {
        let a2100 = f581(Str::lit("total"), {
            let m2096 = match v0.clone() {
                Adt62::C0(h2095) => {
                    h2095
                }
            };
            match m2096 {
                Adt63::C0(_, h2094, _) => {
                    h2094
                }
            }
        })?;
        let a2101 = f575(Str::lit("hasMore"), {
            let m2099 = match v0 {
                Adt62::C0(h2098) => {
                    h2098
                }
            };
            match m2099 {
                Adt63::C0(_, _, h2097) => {
                    h2097
                }
            }
        });
        append_list(a2100, a2101)
    }))), List::<(Str, Adt6)>::nil()))
}

pub fn f518(v0: Adt83) -> R<Option<List<(Str, Adt6)>>> {
    let m2102 = f582(f583(v0.clone()), List::<(Str, Adt6)>::nil())?;
    match m2102 {
        None => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Some(v1) => {
            Ok(Some({
                let a2103 = f584(v1);
                let a2104 = f585(f586(v0));
                append_list(a2103, a2104)
            }))
        }
    }
}

pub fn f519(v0: Adt153) -> Str {
    let m2105 = v0;
    match m2105 {
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

pub fn f520(v0: Adt34) -> Adt153 {
    let m2106 = v0;
    match m2106 {
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

pub fn f521(v0: Option<Adt1>) -> List<(Str, Adt6)> {
    let m2107 = v0;
    match m2107 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((f587(), f588(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f522(v0: bool, v1: u64, v2: List<List<u64>>) -> bool {
    if v0 {
        if f589(v2) {
            let a2108 = v1;
            let a2109 = 200u64;
            nat_eq(a2108, a2109)
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f523(v0: u64, v1: List<List<u64>>) -> Adt95 {
    Adt95::C0(v0, f590(), List::<List<u64>>::nil(), f591(v1))
}

pub fn f524(v0: Adt6) -> R<Option<Adt6>> {
    f592(f593(), Adt154::C0(v0), List::<Adt155>::nil())
}

pub fn f525(v0: Adt135) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("jsonrpc"), Adt6::C3(f231())), List::cons((Str::lit("method"), Adt6::C3(Str::lit("notifications/progress"))), List::cons((Str::lit("params"), Adt6::C5(List::cons((Str::lit("progressToken"), f594(match v0.clone() {
        Adt135::C0(h2110, _) => {
            h2110
        }
    })), List::cons((Str::lit("progress"), Adt6::C2({
        let m2113 = match v0.clone() {
            Adt135::C0(_, h2112) => {
                h2112
            }
        };
        match m2113 {
            Adt52::C0(h2111, _, _) => {
                h2111
            }
        }
    })), {
        let a2120 = f595({
            let m2116 = match v0.clone() {
                Adt135::C0(_, h2115) => {
                    h2115
                }
            };
            match m2116 {
                Adt52::C0(_, h2114, _) => {
                    h2114
                }
            }
        });
        let a2121 = f596({
            let m2119 = match v0 {
                Adt135::C0(_, h2118) => {
                    h2118
                }
            };
            match m2119 {
                Adt52::C0(_, _, h2117) => {
                    h2117
                }
            }
        });
        append_list(a2120, a2121)
    })))), List::<(Str, Adt6)>::nil()))))
}

pub fn f526() -> u64 {
    429u64
}

pub fn f527() -> i64 {
    -32602i64
}

pub fn f528() -> i64 {
    -32022i64
}

pub fn f529() -> i64 {
    -32020i64
}

pub fn f530() -> i64 {
    -32601i64
}

pub fn f531(v0: Adt86) -> i64 {
    let m2122 = v0;
    match m2122 {
        Adt86::C0(_) => {
            f527()
        }
        Adt86::C1(_) => {
            f527()
        }
        Adt86::C2(_) => {
            f527()
        }
        Adt86::C3(_) => {
            f527()
        }
        Adt86::C4(_) => {
            f527()
        }
        Adt86::C5(_) => {
            f527()
        }
        Adt86::C6(_) => {
            f597()
        }
        Adt86::C7(_) => {
            f598()
        }
        Adt86::C8(_) => {
            f599()
        }
        Adt86::C9(_) => {
            f597()
        }
        Adt86::C10 => {
            f527()
        }
        Adt86::C11 => {
            f527()
        }
        Adt86::C12(_) => {
            f597()
        }
        Adt86::C13 => {
            f597()
        }
        Adt86::C14 => {
            f597()
        }
        Adt86::C15(_) => {
            f597()
        }
    }
}

pub fn f532(v0: Adt137) -> Str {
    let m2123 = v0;
    match m2123 {
        Adt137::C0 => {
            Str::lit("Invalid params: params is required")
        }
        Adt137::C1 => {
            Str::lit("Invalid params: _meta is required")
        }
        Adt137::C2 => {
            Str::lit("Invalid params: _meta lacks io.modelcontextprotocol/protocolVersion")
        }
        Adt137::C3 => {
            Str::lit("Invalid params: _meta lacks io.modelcontextprotocol/clientCapabilities")
        }
        Adt137::C4 => {
            Str::lit("Invalid params: _meta is not an object")
        }
        Adt137::C5 => {
            Str::lit("Invalid params: io.modelcontextprotocol/protocolVersion is not a string")
        }
        Adt137::C6 => {
            Str::lit("Invalid params: io.modelcontextprotocol/clientCapabilities is not a valid capabilities object")
        }
        Adt137::C7(_) => {
            Str::lit("Invalid params: a required field is missing")
        }
        Adt137::C8(_) => {
            Str::lit("Invalid params: a field has the wrong type")
        }
    }
}

pub fn f533(v0: Adt118) -> Str {
    let m2124 = v0;
    match m2124 {
        Adt118::C0 => {
            Str::lit("Header mismatch: MCP-Protocol-Version header is missing")
        }
        Adt118::C1 => {
            Str::lit("Header mismatch: Mcp-Method header is missing")
        }
        Adt118::C2 => {
            Str::lit("Header mismatch: Mcp-Name header is missing")
        }
        Adt118::C3 => {
            Str::lit("Header mismatch: MCP-Protocol-Version differs from the request body")
        }
        Adt118::C4 => {
            Str::lit("Header mismatch: Mcp-Method differs from the request body")
        }
        Adt118::C5 => {
            Str::lit("Header mismatch: Mcp-Name differs from the request body")
        }
        Adt118::C6(v1) => {
            let m2125 = v1;
            match m2125 {
                Adt119::C0 => {
                    Str::lit("Header mismatch: MCP-Protocol-Version has a malformed value")
                }
                Adt119::C1 => {
                    Str::lit("Header mismatch: Mcp-Method has a malformed value")
                }
                Adt119::C2 => {
                    Str::lit("Header mismatch: Mcp-Name has a malformed value")
                }
            }
        }
        Adt118::C7(v2) => {
            let m2126 = v2;
            match m2126 {
                Adt119::C0 => {
                    Str::lit("Header mismatch: MCP-Protocol-Version is repeated")
                }
                Adt119::C1 => {
                    Str::lit("Header mismatch: Mcp-Method is repeated")
                }
                Adt119::C2 => {
                    Str::lit("Header mismatch: Mcp-Name is repeated")
                }
            }
        }
        Adt118::C8(_) => {
            Str::lit("Header mismatch: a required Mcp-Param header is missing")
        }
        Adt118::C9(_) => {
            Str::lit("Header mismatch: an Mcp-Param header differs from the request body")
        }
        Adt118::C10(_) => {
            Str::lit("Header mismatch: an Mcp-Param header has a malformed value")
        }
    }
}

pub fn f534(v0: List<Str>) -> List<Adt6> {
    f411(f600(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f535(v0: Adt19) -> Adt6 {
    Adt6::C5({
        let a2138 = f601(Str::lit("experimental"), match v0.clone() {
            Adt19::C0(h2127, _, _, _, _) => {
                h2127
            }
        });
        let a2139 = {
            let a2136 = f601(Str::lit("roots"), match v0.clone() {
                Adt19::C0(_, h2128, _, _, _) => {
                    h2128
                }
            });
            let a2137 = {
                let a2134 = f601(Str::lit("sampling"), match v0.clone() {
                    Adt19::C0(_, _, h2129, _, _) => {
                        h2129
                    }
                });
                let a2135 = {
                    let a2132 = f601(Str::lit("elicitation"), match v0.clone() {
                        Adt19::C0(_, _, _, h2130, _) => {
                            h2130
                        }
                    });
                    let a2133 = f601(Str::lit("extensions"), match v0 {
                        Adt19::C0(_, _, _, _, h2131) => {
                            h2131
                        }
                    });
                    append_list(a2132, a2133)
                };
                append_list(a2134, a2135)
            };
            append_list(a2136, a2137)
        };
        append_list(a2138, a2139)
    })
}

pub fn f536(v0: List<u64>) -> R<bool> {
    let m2140 = v0;
    match m2140.uncons() {
        None => {
            Ok(false)
        }
        Some((v1, v2)) => {
            let m2141 = v2;
            match m2141.uncons() {
                None => {
                    Ok(false)
                }
                Some((v3, v4)) => {
                    let a2142 = f287(v1)?;
                    let a2143 = 113u64;
                    if if nat_eq(a2142, a2143) {
                        let a2144 = v3;
                        let a2145 = 61u64;
                        nat_eq(a2144, a2145)
                    } else {
                        false
                    } {
                        Ok(f602(v4))
                    } else {
                        Ok(false)
                    }
                }
            }
        }
    }
}

pub fn f537(v0: Adt0, v1: Fn2, v2: u64, v3: Result<Adt16, Adt137>) -> R<Result<Adt133, Adt136>> {
    let m2146 = v3;
    match m2146 {
        Ok(v5) => {
            f603(v0, v1, v2, v5)
        }
        Err(v4) => {
            Ok(Err::<Adt133, Adt136>(Adt136::C0(v4)))
        }
    }
}

pub fn f538(v0: Adt144, v1: Adt18, v2: Adt5) -> Result<Adt16, Adt137> {
    let m2147 = v0;
    match m2147 {
        Adt144::C0 => {
            f604(v1, v2)
        }
        Adt144::C1 => {
            f605(v1, v2)
        }
        Adt144::C2 => {
            f606(v1, v2)
        }
        Adt144::C3 => {
            f607(v1, v2)
        }
        Adt144::C4 => {
            f608(v1, v2)
        }
        Adt144::C5 => {
            f609(v1, v2)
        }
        Adt144::C6 => {
            f610(v1, v2)
        }
        Adt144::C7 => {
            f611(v1, v2)
        }
        Adt144::C8 => {
            f612(v1, v2)
        }
        Adt144::C9 => {
            f613(v1, v2)
        }
        Adt144::C10 => {
            f614(v1, v2)
        }
        Adt144::C11 => {
            f615(v1, v2)
        }
    }
}

pub fn f539(v0: Adt144) -> Str {
    let m2148 = v0;
    match m2148 {
        Adt144::C0 => {
            Str::lit("server/discover")
        }
        Adt144::C1 => {
            Str::lit("tools/list")
        }
        Adt144::C2 => {
            Str::lit("tools/call")
        }
        Adt144::C3 => {
            Str::lit("resources/list")
        }
        Adt144::C4 => {
            Str::lit("resources/read")
        }
        Adt144::C5 => {
            Str::lit("resources/templates/list")
        }
        Adt144::C6 => {
            Str::lit("prompts/list")
        }
        Adt144::C7 => {
            Str::lit("prompts/get")
        }
        Adt144::C8 => {
            Str::lit("completion/complete")
        }
        Adt144::C9 => {
            Str::lit("tasks/get")
        }
        Adt144::C10 => {
            Str::lit("tasks/update")
        }
        Adt144::C11 => {
            Str::lit("tasks/cancel")
        }
    }
}

pub fn f540(v0: Option<Adt8>) -> bool {
    let m2149 = v0;
    match m2149 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f541(v0: Option<Adt9>) -> bool {
    let m2150 = v0;
    match m2150 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f542(v0: Option<Adt5>) -> bool {
    let m2151 = v0;
    match m2151 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f543(v0: Option<Adt5>, v1: Str) -> bool {
    let m2152 = v0;
    match m2152 {
        None => {
            false
        }
        Some(v2) => {
            f331(v1, v2)
        }
    }
}

pub fn f544() -> Str {
    Str::lit("io.modelcontextprotocol/tasks")
}

pub fn f545(v0: List<Str>, v1: Adt115, v2: Adt18, v3: Option<Adt118>) -> Adt149 {
    let m2153 = v3;
    match m2153 {
        None => {
            f616(v1, v2.clone(), f617(match v2 {
                Adt18::C0(h2154, _, _, _, _, _) => {
                    h2154
                }
            }, v0))
        }
        Some(v4) => {
            Adt149::C1(Adt136::C2(v4))
        }
    }
}

pub fn f546(v0: Option<Adt117>, v1: Adt151, v2: Str, v3: Str, v4: Adt5) -> R<Option<Adt118>> {
    let m2155 = v0;
    match m2155 {
        None => {
            Ok(None::<Adt118>)
        }
        Some(v5) => {
            let m2157 = match v5.clone() {
                Adt117::C0(_, _, _, h2156, _) => {
                    h2156
                }
            };
            match m2157 {
                None => {
                    f618(v5, v1, v2, v3, v4)
                }
                Some(v6) => {
                    Ok(Some(v6))
                }
            }
        }
    }
}

pub fn f547(v0: Option<Adt5>) -> Adt5 {
    let m2158 = v0;
    match m2158 {
        None => {
            f619()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f548(v0: Adt5) -> Result<Adt18, Adt137> {
    let m2159 = f620(v0.clone());
    match m2159 {
        Adt156::C0 => {
            f621(v0.clone(), f173(f622(), v0.clone()), f173(f623(), v0))
        }
        Adt156::C1 => {
            Err::<Adt18, Adt137>(Adt137::C2)
        }
        Adt156::C2 => {
            Err::<Adt18, Adt137>(Adt137::C3)
        }
    }
}

pub fn f549() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(112u64, List::cons(97u64, List::cons(114u64, List::cons(97u64, List::cons(109u64, List::cons(45u64, List::<u64>::nil()))))))))))
}

pub fn f550(v0: u64, v1: u64) -> u64 {
    let a2160 = v0;
    let a2161 = v1;
    let a2162 = 0u64;
    nat_rem(a2160, a2161, a2162)
}

pub fn f551(v0: u64, v1: u64) -> u64 {
    let a2163 = v0;
    let a2164 = v1;
    let a2165 = 0u64;
    nat_quot(a2163, a2164, a2165)
}

pub fn f552(p2169: List<Adt6>) -> bool {
    let m2168 = p2169;
    {
        let mut m2168 = m2168;
        loop {
            let step = {
                let v0 = m2168;
                let m2166 = v0;
                match m2166.uncons() {
                    None => {
                        Err::<List<Adt6>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let m2167 = f122(v1);
                        match m2167 {
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
                Ok(next) => { m2168 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f553(p2173: List<Adt6>) -> bool {
    let m2172 = p2173;
    {
        let mut m2172 = m2172;
        loop {
            let step = {
                let v0 = m2172;
                let m2170 = v0;
                match m2170.uncons() {
                    None => {
                        Err::<List<Adt6>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let m2171 = f122(v1);
                        match m2171 {
                            None => {
                                Err::<List<Adt6>, bool>(false)
                            }
                            Some(v3) => {
                                if f358(v3) {
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
                Ok(next) => { m2172 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f554(v0: Str) -> Option<Adt13> {
    let a2174 = v0.clone();
    let a2175 = Str::lit("string");
    if equal(a2174, a2175) {
        Some(Adt13::C0)
    } else {
        let a2176 = v0.clone();
        let a2177 = Str::lit("integer");
        if equal(a2176, a2177) {
            Some(Adt13::C1)
        } else {
            let a2178 = v0;
            let a2179 = Str::lit("boolean");
            if equal(a2178, a2179) {
                Some(Adt13::C2)
            } else {
                None::<Adt13>
            }
        }
    }
}

pub fn f555(p2188: List<u64>, p2189: List<u64>) -> R<List<u64>> {
    let m2187 = (p2188, p2189);
    {
        let mut m2187 = m2187;
        loop {
            let step = {
                let (v0, v1) = m2187;
                let m2180 = v0;
                match m2180.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), R<List<u64>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        let a2181 = 65u64;
                        let a2182 = v2;
                        if if nat_le(a2181, a2182) {
                            let a2183 = v2;
                            let a2184 = 90u64;
                            nat_le(a2183, a2184)
                        } else {
                            false
                        } {
                            Ok::<(List<u64>, List<u64>), R<List<u64>>>((v3, List::cons({
                                let a2185 = v2;
                                let a2186 = 32u64;
                                nat_add(a2185, a2186)?
                            }, v1)))
                        } else {
                            Ok::<(List<u64>, List<u64>), R<List<u64>>>((v3, List::cons(v2, v1)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2187 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f556(p2196: List<u64>, p2197: List<u64>) -> bool {
    let m2195 = (p2196, p2197);
    {
        let mut m2195 = m2195;
        loop {
            let step = {
                let (v0, v1) = m2195;
                let m2190 = v0;
                match m2190.uncons() {
                    None => {
                        let m2191 = v1;
                        match m2191.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m2192 = v1;
                        match m2192.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                            Some((v6, v7)) => {
                                let a2193 = v4;
                                let a2194 = v6;
                                if nat_eq(a2193, a2194) {
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
                Ok(next) => { m2195 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f557(v0: Adt6) -> u64 {
    let m2198 = v0;
    match m2198 {
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

pub fn f558(v0: Adt147) -> bool {
    let m2199 = v0;
    match m2199 {
        Adt147::C0 => {
            false
        }
        Adt147::C1 => {
            true
        }
        Adt147::C2 => {
            false
        }
    }
}

pub fn f559(v0: Option<List<(Adt6, Adt6)>>, v1: List<(Adt6, Adt6)>) -> Adt152 {
    let m2200 = v0;
    match m2200 {
        None => {
            Adt152::C0
        }
        Some(v2) => {
            Adt152::C1({
                let a2201 = v2;
                let a2202 = v1;
                append_list(a2201, a2202)
            })
        }
    }
}

pub fn f560(p2207: List<Adt6>, p2208: List<Adt6>, p2209: List<(Adt6, Adt6)>) -> Option<List<(Adt6, Adt6)>> {
    let m2206 = (p2207, (p2208, p2209));
    {
        let mut m2206 = m2206;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2206;
                let m2203 = v0;
                match m2203.uncons() {
                    None => {
                        let m2204 = v1;
                        match m2204.uncons() {
                            None => {
                                Err::<(List<Adt6>, (List<Adt6>, List<(Adt6, Adt6)>)), Option<List<(Adt6, Adt6)>>>(Some(v2))
                            }
                            Some((_, _)) => {
                                Err::<(List<Adt6>, (List<Adt6>, List<(Adt6, Adt6)>)), Option<List<(Adt6, Adt6)>>>(None::<List<(Adt6, Adt6)>>)
                            }
                        }
                    }
                    Some((v5, v6)) => {
                        let m2205 = v1;
                        match m2205.uncons() {
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
                Ok(next) => { m2206 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f561(p2215: List<(Str, Adt6)>, v1: List<(Str, Adt6)>, p2216: List<(Adt6, Adt6)>) -> Option<List<(Adt6, Adt6)>> {
    let m2214 = (p2215, p2216);
    {
        let mut m2214 = m2214;
        loop {
            let step = {
                let (v0, v2) = m2214;
                let m2210 = v0;
                match m2210.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<(Adt6, Adt6)>), Option<List<(Adt6, Adt6)>>>(Some(v2))
                    }
                    Some((v3, v4)) => {
                        let m2212 = f227({
                            let (h2211, _) = v3;
                            h2211
                        }, v1.clone());
                        match m2212 {
                            Adt114::C0 => {
                                Err::<(List<(Str, Adt6)>, List<(Adt6, Adt6)>), Option<List<(Adt6, Adt6)>>>(None::<List<(Adt6, Adt6)>>)
                            }
                            Adt114::C1(v5) => {
                                Ok::<(List<(Str, Adt6)>, List<(Adt6, Adt6)>), Option<List<(Adt6, Adt6)>>>((v4, List::cons(({
                                    let (_, h2213) = v3;
                                    h2213
                                }, v5), v2)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2214 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f562(v0: Adt147) -> Adt147 {
    let m2217 = v0;
    match m2217 {
        Adt147::C0 => {
            Adt147::C2
        }
        Adt147::C1 => {
            Adt147::C1
        }
        Adt147::C2 => {
            Adt147::C0
        }
    }
}

pub fn f563(v0: i64, v1: i64) -> R<i64> {
    f182(f136(f565(v0)?, 0i64)?, v1)
}

pub fn f564(v0: i64, v1: u64, v2: i64, v3: u64) -> R<Adt147> {
    let a2218 = v1;
    let a2219 = v3;
    if nat_lt(a2218, a2219) {
        Ok(f443(f624({
            let a2220 = v3;
            let a2221 = v1;
            nat_sub(a2220, a2221)
        }, v0)?, v2))
    } else {
        let a2222 = v3;
        let a2223 = v1;
        if nat_lt(a2222, a2223) {
            Ok(f443(v0, f624({
                let a2224 = v1;
                let a2225 = v3;
                nat_sub(a2224, a2225)
            }, v2)?))
        } else {
            Ok(f443(v0, v2))
        }
    }
}

pub fn f565(v0: i64) -> R<u64> {
    f625(20u64, v0, 0u64)
}

pub fn f566(p2237: u64, p2238: i64, p2239: List<u64>) -> R<List<u64>> {
    let m2236 = (p2237, (p2238, p2239));
    {
        let mut m2236 = m2236;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2236;
                let m2226 = v0;
                if m2226 == 0 {
                    Err::<(u64, (i64, List<u64>)), R<List<u64>>>(Ok(v2))
                } else {
                    let v3 = m2226 - 1;
                    let v4: i64 = {
                        let a2227 = v1;
                        let a2228 = 10i64;
                        let a2229 = 0i64;
                        int_quot(a2227, a2228, a2229)?
                    };
                    let v5: i64 = {
                        let a2230 = v1;
                        let a2231 = 10i64;
                        let a2232 = 0i64;
                        int_rem(a2230, a2231, a2232)
                    };
                    let m2235 = {
                        let a2233 = v4;
                        let a2234 = 1i64;
                        compare(a2233, a2234)
                    };
                    if match m2235 {
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
                        Err::<(u64, (i64, List<u64>)), R<List<u64>>>(Ok(List::cons(f626(v5), v2)))
                    } else {
                        Ok::<(u64, (i64, List<u64>)), R<List<u64>>>((v3, (v4, List::cons(f626(v5), v2))))
                    }
                }
            };
            match step {
                Ok(next) => { m2236 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f567(v0: u64) -> R<u64> {
    let a2240 = v0;
    let a2241 = 10u64;
    if nat_lt(a2240, a2241) {
        let a2242 = 48u64;
        let a2243 = v0;
        nat_add(a2242, a2243)
    } else {
        let a2244 = 87u64;
        let a2245 = v0;
        nat_add(a2244, a2245)
    }
}

pub fn f568(v0: List<Str>) -> Adt6 {
    Adt6::C4(f534(v0))
}

pub fn f569(v0: Adt4) -> Adt6 {
    Adt6::C5({
        let a2263 = f601(Str::lit("experimental"), match v0.clone() {
            Adt4::C0(h2246, _, _, _, _, _, _) => {
                h2246
            }
        });
        let a2264 = {
            let a2261 = f601(Str::lit("logging"), match v0.clone() {
                Adt4::C0(_, h2247, _, _, _, _, _) => {
                    h2247
                }
            });
            let a2262 = {
                let a2259 = f601(Str::lit("completions"), match v0.clone() {
                    Adt4::C0(_, _, h2248, _, _, _, _) => {
                        h2248
                    }
                });
                let a2260 = {
                    let a2257 = f627(Str::lit("prompts"), match v0.clone() {
                        Adt4::C0(_, _, _, h2249, _, _, _) => {
                            h2249
                        }
                    });
                    let a2258 = {
                        let a2255 = f628(match v0.clone() {
                            Adt4::C0(_, _, _, _, h2250, _, _) => {
                                h2250
                            }
                        });
                        let a2256 = {
                            let a2253 = f627(Str::lit("tools"), match v0.clone() {
                                Adt4::C0(_, _, _, _, _, h2251, _) => {
                                    h2251
                                }
                            });
                            let a2254 = f601(Str::lit("extensions"), match v0 {
                                Adt4::C0(_, _, _, _, _, _, h2252) => {
                                    h2252
                                }
                            });
                            append_list(a2253, a2254)
                        };
                        append_list(a2255, a2256)
                    };
                    append_list(a2257, a2258)
                };
                append_list(a2259, a2260)
            };
            append_list(a2261, a2262)
        };
        append_list(a2263, a2264)
    })
}

pub fn f570(v0: Str, v1: Option<Str>) -> List<(Str, Adt6)> {
    let m2265 = v1;
    match m2265 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C3(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f571(v0: Adt10) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("ttlMs"), f48(match v0.clone() {
        Adt10::C0(h2266, _) => {
            h2266
        }
    })?), List::cons((Str::lit("cacheScope"), Adt6::C3(f629(match v0 {
        Adt10::C0(_, h2267) => {
            h2267
        }
    }))), List::<(Str, Adt6)>::nil())))
}

pub fn f572(v0: List<Adt37>) -> List<Adt6> {
    f411(f630(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f573(v0: List<Adt40>) -> R<Option<List<Adt6>>> {
    let m2268 = f631(v0, List::<Adt6>::nil())?;
    match m2268 {
        None => {
            Ok(None::<List<Adt6>>)
        }
        Some(v1) => {
            Ok(Some(f411(v1, List::<Adt6>::nil())))
        }
    }
}

pub fn f574(v0: Str, v1: Option<Adt6>) -> List<(Str, Adt6)> {
    let m2269 = v1;
    match m2269 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, v2), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f575(v0: Str, v1: Option<bool>) -> List<(Str, Adt6)> {
    let m2270 = v1;
    match m2270 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C1(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f576(v0: List<Adt46>) -> R<List<Adt6>> {
    Ok(f411(f632(v0, List::<Adt6>::nil())?, List::<Adt6>::nil()))
}

pub fn f577(v0: List<Adt48>) -> List<Adt6> {
    f411(f633(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f578(v0: List<Adt56>) -> List<Adt6> {
    f411(f634(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f579(v0: List<Adt58>) -> List<Adt6> {
    f411(f635(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f580(v0: List<Adt61>) -> R<Option<List<Adt6>>> {
    let m2271 = f636(v0, List::<Adt6>::nil())?;
    match m2271 {
        None => {
            Ok(None::<List<Adt6>>)
        }
        Some(v1) => {
            Ok(Some(f411(v1, List::<Adt6>::nil())))
        }
    }
}

pub fn f581(v0: Str, v1: Option<u64>) -> R<List<(Str, Adt6)>> {
    let m2272 = v1;
    match m2272 {
        None => {
            Ok(List::<(Str, Adt6)>::nil())
        }
        Some(v2) => {
            Ok(List::cons((v0, f48(v2)?), List::<(Str, Adt6)>::nil()))
        }
    }
}

pub fn f582(p2278: List<(Str, Adt66)>, p2279: List<(Str, Adt6)>) -> R<Option<List<(Str, Adt6)>>> {
    let m2277 = (p2278, p2279);
    {
        let mut m2277 = m2277;
        loop {
            let step = {
                let (v0, v1) = m2277;
                let m2273 = v0;
                match m2273.uncons() {
                    None => {
                        Err::<(List<(Str, Adt66)>, List<(Str, Adt6)>), R<Option<List<(Str, Adt6)>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m2275 = f637({
                            let (_, h2274) = v2;
                            h2274
                        })?;
                        match m2275 {
                            None => {
                                Err::<(List<(Str, Adt66)>, List<(Str, Adt6)>), R<Option<List<(Str, Adt6)>>>>(Ok(None::<List<(Str, Adt6)>>))
                            }
                            Some(v4) => {
                                Ok::<(List<(Str, Adt66)>, List<(Str, Adt6)>), R<Option<List<(Str, Adt6)>>>>((v3, List::cons(({
                                    let (h2276, _) = v2;
                                    h2276
                                }, v4), v1)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2277 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f583(v0: Adt83) -> List<(Str, Adt66)> {
    let m2280 = v0;
    match m2280 {
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

pub fn f584(v0: List<(Str, Adt6)>) -> List<(Str, Adt6)> {
    let m2281 = v0.clone();
    match m2281.uncons() {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some((_, _)) => {
            List::cons((Str::lit("inputRequests"), Adt6::C5(f413(v0, List::<(Str, Adt6)>::nil()))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f585(v0: Option<Str>) -> List<(Str, Adt6)> {
    let m2282 = v0;
    match m2282 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("requestState"), Adt6::C3(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f586(v0: Adt83) -> Option<Str> {
    let m2283 = v0;
    match m2283 {
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

pub fn f587() -> Str {
    Str::lit("io.modelcontextprotocol/serverInfo")
}

pub fn f588(v0: Adt1) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
        Adt1::C0(h2284, _, _, _, _, _) => {
            h2284
        }
    })), List::cons((Str::lit("version"), Adt6::C3(match v0.clone() {
        Adt1::C0(_, _, h2285, _, _, _) => {
            h2285
        }
    })), {
        let a2294 = f570(Str::lit("title"), match v0.clone() {
            Adt1::C0(_, h2286, _, _, _, _) => {
                h2286
            }
        });
        let a2295 = {
            let a2292 = f570(Str::lit("description"), match v0.clone() {
                Adt1::C0(_, _, _, h2287, _, _) => {
                    h2287
                }
            });
            let a2293 = {
                let a2290 = f570(Str::lit("websiteUrl"), match v0.clone() {
                    Adt1::C0(_, _, _, _, h2288, _) => {
                        h2288
                    }
                });
                let a2291 = f638(match v0 {
                    Adt1::C0(_, _, _, _, _, h2289) => {
                        h2289
                    }
                });
                append_list(a2290, a2291)
            };
            append_list(a2292, a2293)
        };
        append_list(a2294, a2295)
    })))
}

pub fn f589(v0: List<List<u64>>) -> bool {
    let m2296 = v0;
    match m2296.uncons() {
        None => {
            false
        }
        Some((_, _)) => {
            true
        }
    }
}

pub fn f590() -> List<Adt92> {
    List::cons(Adt92::C0(f639(), f467()), List::cons(Adt92::C0(f640(), f641()), List::cons(Adt92::C0(f642(), f643()), List::<Adt92>::nil())))
}

pub fn f591(v0: List<List<u64>>) -> List<u64> {
    f644(f101(v0, List::<List<u64>>::nil()), List::<u64>::nil())
}

pub fn f592(p2311: u64, p2312: Adt154, p2313: List<Adt155>) -> R<Option<Adt6>> {
    let m2310 = (p2311, (p2312, p2313));
    {
        let mut m2310 = m2310;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2310;
                let m2297 = v0;
                if m2297 == 0 {
                    Err::<(u64, (Adt154, List<Adt155>)), R<Option<Adt6>>>(Ok(None::<Adt6>))
                } else {
                    let v3 = m2297 - 1;
                    let m2298 = v1;
                    match m2298 {
                        Adt154::C0(v4) => {
                            let m2299 = v4.clone();
                            match m2299 {
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
                                    let v8: Str = f645(v7)?;
                                    Ok::<(u64, (Adt154, List<Adt155>)), R<Option<Adt6>>>((v3, (Adt154::C1(Adt6::C3(v8)), v2)))
                                }
                                Adt6::C4(v9) => {
                                    let m2300 = v9;
                                    match m2300.uncons() {
                                        None => {
                                            Ok::<(u64, (Adt154, List<Adt155>)), R<Option<Adt6>>>((v3, (Adt154::C1(v4), v2)))
                                        }
                                        Some((v10, v11)) => {
                                            Ok::<(u64, (Adt154, List<Adt155>)), R<Option<Adt6>>>((v3, (Adt154::C0(v10), List::cons(Adt155::C0(List::<Adt6>::nil(), v11), v2))))
                                        }
                                    }
                                }
                                Adt6::C5(v12) => {
                                    let m2301 = v12;
                                    match m2301.uncons() {
                                        None => {
                                            Ok::<(u64, (Adt154, List<Adt155>)), R<Option<Adt6>>>((v3, (Adt154::C1(v4), v2)))
                                        }
                                        Some((v13, v14)) => {
                                            let v15: Str = f645({
                                                let (h2302, _) = v13;
                                                h2302
                                            })?;
                                            Ok::<(u64, (Adt154, List<Adt155>)), R<Option<Adt6>>>((v3, (Adt154::C0({
                                                let (_, h2303) = v13;
                                                h2303
                                            }), List::cons(Adt155::C1(v15, List::<(Str, Adt6)>::nil(), v14), v2))))
                                        }
                                    }
                                }
                            }
                        }
                        Adt154::C1(v16) => {
                            let m2304 = v2;
                            match m2304.uncons() {
                                None => {
                                    Err::<(u64, (Adt154, List<Adt155>)), R<Option<Adt6>>>(Ok(Some(v16)))
                                }
                                Some((v17, v18)) => {
                                    let m2305 = v17;
                                    match m2305 {
                                        Adt155::C0(v19, v20) => {
                                            let v21: List<Adt6> = List::cons(v16, v19);
                                            let m2306 = v20;
                                            match m2306.uncons() {
                                                None => {
                                                    Ok::<(u64, (Adt154, List<Adt155>)), R<Option<Adt6>>>((v3, (Adt154::C1(Adt6::C4(f411(v21, List::<Adt6>::nil()))), v18)))
                                                }
                                                Some((v22, v23)) => {
                                                    Ok::<(u64, (Adt154, List<Adt155>)), R<Option<Adt6>>>((v3, (Adt154::C0(v22), List::cons(Adt155::C0(v21, v23), v18))))
                                                }
                                            }
                                        }
                                        Adt155::C1(v24, v25, v26) => {
                                            let v27: List<(Str, Adt6)> = List::cons((v24, v16), v25);
                                            let m2307 = v26;
                                            match m2307.uncons() {
                                                None => {
                                                    Ok::<(u64, (Adt154, List<Adt155>)), R<Option<Adt6>>>((v3, (Adt154::C1(Adt6::C5(f413(v27, List::<(Str, Adt6)>::nil()))), v18)))
                                                }
                                                Some((v28, v29)) => {
                                                    let v30: Str = f645({
                                                        let (h2308, _) = v28;
                                                        h2308
                                                    })?;
                                                    Ok::<(u64, (Adt154, List<Adt155>)), R<Option<Adt6>>>((v3, (Adt154::C0({
                                                        let (_, h2309) = v28;
                                                        h2309
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
                Ok(next) => { m2310 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f593() -> u64 {
    67108864u64
}

pub fn f594(v0: Adt21) -> Adt6 {
    let m2314 = v0;
    match m2314 {
        Adt21::C0(v1) => {
            Adt6::C3(v1)
        }
        Adt21::C1(v2) => {
            Adt6::C2(Adt7::C0(v2, 0i64))
        }
    }
}

pub fn f595(v0: Option<Adt7>) -> List<(Str, Adt6)> {
    let m2315 = v0;
    match m2315 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("total"), Adt6::C2(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f596(v0: Option<Str>) -> List<(Str, Adt6)> {
    let m2316 = v0;
    match m2316 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("message"), Adt6::C3(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f597() -> i64 {
    -32603i64
}

pub fn f598() -> i64 {
    429i64
}

pub fn f599() -> i64 {
    -32021i64
}

pub fn f600(p2319: List<Str>, p2320: List<Adt6>) -> List<Adt6> {
    let m2318 = (p2319, p2320);
    {
        let mut m2318 = m2318;
        loop {
            let step = {
                let (v0, v1) = m2318;
                let m2317 = v0;
                match m2317.uncons() {
                    None => {
                        Err::<(List<Str>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Str>, List<Adt6>), List<Adt6>>((v3, List::cons(Adt6::C3(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2318 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f601(v0: Str, v1: Option<Adt5>) -> List<(Str, Adt6)> {
    let m2321 = v1;
    match m2321 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C5(match v2 {
                Adt5::C0(h2322) => {
                    h2322
                }
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f602(v0: List<u64>) -> bool {
    let m2323 = v0;
    match m2323.uncons() {
        None => {
            false
        }
        Some((v1, v2)) => {
            let a2324 = v1;
            let a2325 = 48u64;
            if nat_eq(a2324, a2325) {
                f646(v2)
            } else {
                false
            }
        }
    }
}

pub fn f603(v0: Adt0, v1: Fn2, v2: u64, v3: Adt16) -> R<Result<Adt133, Adt136>> {
    if f647(f648(v3.clone())) {
        Ok(Ok::<Adt133, Adt136>(f649(v0.clone(), Adt34::C0(f650(v0)))))
    } else {
        Ok(f651(v0.clone(), v3.clone(), f652(match v0.clone() {
            Adt0::C0(_, _, _, _, _, _, h2326) => {
                h2326
            }
        }, match v0 {
            Adt0::C0(_, _, _, _, _, h2327, _) => {
                h2327
            }
        }, v2, v1, v3)?))
    }
}

pub fn f604(v0: Adt18, _: Adt5) -> Result<Adt16, Adt137> {
    Ok::<Adt16, Adt137>(Adt16::C0(Adt17::C0(v0)))
}

pub fn f605(v0: Adt18, v1: Adt5) -> Result<Adt16, Adt137> {
    let m2328 = f653(Str::lit("cursor"), v1);
    match m2328 {
        Ok(v2) => {
            Ok::<Adt16, Adt137>(Adt16::C1(Adt22::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt16, Adt137>(v3)
        }
    }
}

pub fn f606(v0: Adt18, v1: Adt5) -> Result<Adt16, Adt137> {
    let m2329 = f654(Str::lit("name"), v1.clone());
    match m2329 {
        Ok(v2) => {
            let m2330 = f655(Str::lit("arguments"), v1.clone());
            match m2330 {
                Ok(v3) => {
                    let m2331 = f655(Str::lit("inputResponses"), v1.clone());
                    match m2331 {
                        Ok(v4) => {
                            let m2332 = f653(Str::lit("requestState"), v1);
                            match m2332 {
                                Ok(v5) => {
                                    Ok::<Adt16, Adt137>(Adt16::C2(Adt23::C0(v0, v2, v3, v4, v5)))
                                }
                                Err(v6) => {
                                    Err::<Adt16, Adt137>(v6)
                                }
                            }
                        }
                        Err(v7) => {
                            Err::<Adt16, Adt137>(v7)
                        }
                    }
                }
                Err(v8) => {
                    Err::<Adt16, Adt137>(v8)
                }
            }
        }
        Err(v9) => {
            Err::<Adt16, Adt137>(v9)
        }
    }
}

pub fn f607(v0: Adt18, v1: Adt5) -> Result<Adt16, Adt137> {
    let m2333 = f653(Str::lit("cursor"), v1);
    match m2333 {
        Ok(v2) => {
            Ok::<Adt16, Adt137>(Adt16::C3(Adt22::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt16, Adt137>(v3)
        }
    }
}

pub fn f608(v0: Adt18, v1: Adt5) -> Result<Adt16, Adt137> {
    let m2334 = f654(Str::lit("uri"), v1.clone());
    match m2334 {
        Ok(v2) => {
            let m2335 = f655(Str::lit("inputResponses"), v1.clone());
            match m2335 {
                Ok(v3) => {
                    let m2336 = f653(Str::lit("requestState"), v1);
                    match m2336 {
                        Ok(v4) => {
                            Ok::<Adt16, Adt137>(Adt16::C4(Adt24::C0(v0, v2, v3, v4)))
                        }
                        Err(v5) => {
                            Err::<Adt16, Adt137>(v5)
                        }
                    }
                }
                Err(v6) => {
                    Err::<Adt16, Adt137>(v6)
                }
            }
        }
        Err(v7) => {
            Err::<Adt16, Adt137>(v7)
        }
    }
}

pub fn f609(v0: Adt18, v1: Adt5) -> Result<Adt16, Adt137> {
    let m2337 = f653(Str::lit("cursor"), v1);
    match m2337 {
        Ok(v2) => {
            Ok::<Adt16, Adt137>(Adt16::C5(Adt22::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt16, Adt137>(v3)
        }
    }
}

pub fn f610(v0: Adt18, v1: Adt5) -> Result<Adt16, Adt137> {
    let m2338 = f653(Str::lit("cursor"), v1);
    match m2338 {
        Ok(v2) => {
            Ok::<Adt16, Adt137>(Adt16::C6(Adt22::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt16, Adt137>(v3)
        }
    }
}

pub fn f611(v0: Adt18, v1: Adt5) -> Result<Adt16, Adt137> {
    let m2339 = f654(Str::lit("name"), v1.clone());
    match m2339 {
        Ok(v2) => {
            let m2340 = f656(Str::lit("arguments"), v1.clone());
            match m2340 {
                Ok(v3) => {
                    let m2341 = f655(Str::lit("inputResponses"), v1.clone());
                    match m2341 {
                        Ok(v4) => {
                            let m2342 = f653(Str::lit("requestState"), v1);
                            match m2342 {
                                Ok(v5) => {
                                    Ok::<Adt16, Adt137>(Adt16::C7(Adt25::C0(v0, v2, v3, v4, v5)))
                                }
                                Err(v6) => {
                                    Err::<Adt16, Adt137>(v6)
                                }
                            }
                        }
                        Err(v7) => {
                            Err::<Adt16, Adt137>(v7)
                        }
                    }
                }
                Err(v8) => {
                    Err::<Adt16, Adt137>(v8)
                }
            }
        }
        Err(v9) => {
            Err::<Adt16, Adt137>(v9)
        }
    }
}

pub fn f612(v0: Adt18, v1: Adt5) -> Result<Adt16, Adt137> {
    let m2343 = f657(Str::lit("ref"), v1.clone());
    match m2343 {
        Ok(v2) => {
            let m2344 = f658(v2);
            match m2344 {
                Ok(v3) => {
                    let m2345 = f657(Str::lit("argument"), v1.clone());
                    match m2345 {
                        Ok(v4) => {
                            let m2346 = f659(v4);
                            match m2346 {
                                Ok(v5) => {
                                    let m2347 = f660(v1);
                                    match m2347 {
                                        Ok(v6) => {
                                            Ok::<Adt16, Adt137>(Adt16::C8(Adt26::C0(v0, v3, v5, v6)))
                                        }
                                        Err(v7) => {
                                            Err::<Adt16, Adt137>(v7)
                                        }
                                    }
                                }
                                Err(v8) => {
                                    Err::<Adt16, Adt137>(v8)
                                }
                            }
                        }
                        Err(v9) => {
                            Err::<Adt16, Adt137>(v9)
                        }
                    }
                }
                Err(v10) => {
                    Err::<Adt16, Adt137>(v10)
                }
            }
        }
        Err(v11) => {
            Err::<Adt16, Adt137>(v11)
        }
    }
}

pub fn f613(v0: Adt18, v1: Adt5) -> Result<Adt16, Adt137> {
    let m2348 = f654(Str::lit("taskId"), v1);
    match m2348 {
        Ok(v2) => {
            Ok::<Adt16, Adt137>(Adt16::C9(Adt31::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt16, Adt137>(v3)
        }
    }
}

pub fn f614(v0: Adt18, v1: Adt5) -> Result<Adt16, Adt137> {
    let m2349 = f654(Str::lit("taskId"), v1.clone());
    match m2349 {
        Ok(v2) => {
            let m2350 = f657(Str::lit("inputResponses"), v1);
            match m2350 {
                Ok(v3) => {
                    Ok::<Adt16, Adt137>(Adt16::C10(Adt32::C0(v0, v2, v3)))
                }
                Err(v4) => {
                    Err::<Adt16, Adt137>(v4)
                }
            }
        }
        Err(v5) => {
            Err::<Adt16, Adt137>(v5)
        }
    }
}

pub fn f615(v0: Adt18, v1: Adt5) -> Result<Adt16, Adt137> {
    let m2351 = f654(Str::lit("taskId"), v1);
    match m2351 {
        Ok(v2) => {
            Ok::<Adt16, Adt137>(Adt16::C11(Adt33::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt16, Adt137>(v3)
        }
    }
}

pub fn f616(v0: Adt115, v1: Adt18, v2: Option<Adt136>) -> Adt149 {
    let m2352 = v2;
    match m2352 {
        None => {
            Adt149::C0(Adt150::C0(match v0.clone() {
                Adt115::C0(h2353, _, _) => {
                    h2353
                }
            }, match v0.clone() {
                Adt115::C0(_, h2354, _) => {
                    h2354
                }
            }, v1, f547(match v0 {
                Adt115::C0(_, _, h2355) => {
                    h2355
                }
            })))
        }
        Some(v3) => {
            Adt149::C1(v3)
        }
    }
}

pub fn f617(v0: Str, v1: List<Str>) -> Option<Adt136> {
    f661(f662(v0.clone(), v1.clone()), v0, v1)
}

pub fn f618(v0: Adt117, v1: Adt151, v2: Str, v3: Str, v4: Adt5) -> R<Option<Adt118>> {
    let m2357 = match v0.clone() {
        Adt117::C0(h2356, _, _, _, _) => {
            h2356
        }
    };
    match m2357 {
        None => {
            Ok(Some(Adt118::C0))
        }
        Some(v5) => {
            let a2358 = v5;
            let a2359 = v3;
            if equal(a2358, a2359) {
                let m2361 = match v0.clone() {
                    Adt117::C0(_, h2360, _, _, _) => {
                        h2360
                    }
                };
                match m2361 {
                    None => {
                        Ok(Some(Adt118::C1))
                    }
                    Some(v6) => {
                        let a2362 = v6;
                        let a2363 = v2;
                        if equal(a2362, a2363) {
                            f663(f664(match v0.clone() {
                                Adt117::C0(_, _, h2364, _, _) => {
                                    h2364
                                }
                            }, v1, v4.clone()), match v0 {
                                Adt117::C0(_, _, _, _, h2365) => {
                                    h2365
                                }
                            }, v4)
                        } else {
                            Ok(Some(Adt118::C4))
                        }
                    }
                }
            } else {
                Ok(Some(Adt118::C3))
            }
        }
    }
}

pub fn f619() -> Adt5 {
    Adt5::C0(List::<(Str, Adt6)>::nil())
}

pub fn f620(v0: Adt5) -> Adt156 {
    f665(f173(f622(), v0.clone()), f173(f623(), v0))
}

pub fn f621(v0: Adt5, v1: Adt114, v2: Adt114) -> Result<Adt18, Adt137> {
    let m2366 = v1;
    match m2366 {
        Adt114::C0 => {
            Err::<Adt18, Adt137>(Adt137::C2)
        }
        Adt114::C1(v3) => {
            let m2367 = v2;
            match m2367 {
                Adt114::C0 => {
                    Err::<Adt18, Adt137>(Adt137::C3)
                }
                Adt114::C1(v4) => {
                    f666(v0, v3, v4)
                }
            }
        }
    }
}

pub fn f622() -> Str {
    Str::lit("io.modelcontextprotocol/protocolVersion")
}

pub fn f623() -> Str {
    Str::lit("io.modelcontextprotocol/clientCapabilities")
}

pub fn f624(p2372: u64, p2373: i64) -> R<i64> {
    let m2371 = (p2372, p2373);
    {
        let mut m2371 = m2371;
        loop {
            let step = {
                let (v0, v1) = m2371;
                let m2368 = v0;
                if m2368 == 0 {
                    Err::<(u64, i64), R<i64>>(Ok(v1))
                } else {
                    let v2 = m2368 - 1;
                    Ok::<(u64, i64), R<i64>>((v2, {
                        let a2369 = v1;
                        let a2370 = 10i64;
                        int_mul(a2369, a2370)?
                    }))
                }
            };
            match step {
                Ok(next) => { m2371 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f625(p2384: u64, p2385: i64, p2386: u64) -> R<u64> {
    let m2383 = (p2384, (p2385, p2386));
    {
        let mut m2383 = m2383;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2383;
                let m2374 = v0;
                if m2374 == 0 {
                    Err::<(u64, (i64, u64)), R<u64>>(Ok(v2))
                } else {
                    let v3 = m2374 - 1;
                    let v4: u64 = {
                        let a2375 = v2;
                        let a2376 = 1u64;
                        nat_add(a2375, a2376)?
                    };
                    let m2379 = {
                        let a2377 = v1;
                        let a2378 = 10i64;
                        compare(a2377, a2378)
                    };
                    if match m2379 {
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
                            let a2380 = v1;
                            let a2381 = 10i64;
                            let a2382 = 0i64;
                            int_quot(a2380, a2381, a2382)?
                        }, v4)))
                    }
                }
            };
            match step {
                Ok(next) => { m2383 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f626(v0: i64) -> u64 {
    let m2389 = {
        let a2387 = v0;
        let a2388 = 1i64;
        compare(a2387, a2388)
    };
    if match m2389 {
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
        let m2392 = {
            let a2390 = v0;
            let a2391 = 2i64;
            compare(a2390, a2391)
        };
        if match m2392 {
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
            let m2395 = {
                let a2393 = v0;
                let a2394 = 3i64;
                compare(a2393, a2394)
            };
            if match m2395 {
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
                let m2398 = {
                    let a2396 = v0;
                    let a2397 = 4i64;
                    compare(a2396, a2397)
                };
                if match m2398 {
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
                    let m2401 = {
                        let a2399 = v0;
                        let a2400 = 5i64;
                        compare(a2399, a2400)
                    };
                    if match m2401 {
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
                        let m2404 = {
                            let a2402 = v0;
                            let a2403 = 6i64;
                            compare(a2402, a2403)
                        };
                        if match m2404 {
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
                            let m2407 = {
                                let a2405 = v0;
                                let a2406 = 7i64;
                                compare(a2405, a2406)
                            };
                            if match m2407 {
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
                                let m2410 = {
                                    let a2408 = v0;
                                    let a2409 = 8i64;
                                    compare(a2408, a2409)
                                };
                                if match m2410 {
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
                                    let m2413 = {
                                        let a2411 = v0;
                                        let a2412 = 9i64;
                                        compare(a2411, a2412)
                                    };
                                    if match m2413 {
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

pub fn f627(v0: Str, v1: Option<Adt8>) -> List<(Str, Adt6)> {
    let m2414 = v1;
    match m2414 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, f667(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f628(v0: Option<Adt9>) -> List<(Str, Adt6)> {
    let m2415 = v0;
    match m2415 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("resources"), Adt6::C5({
                let a2418 = f575(Str::lit("subscribe"), match v1.clone() {
                    Adt9::C0(h2416, _) => {
                        h2416
                    }
                });
                let a2419 = f575(Str::lit("listChanged"), match v1 {
                    Adt9::C0(_, h2417) => {
                        h2417
                    }
                });
                append_list(a2418, a2419)
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f629(v0: Adt11) -> Str {
    let m2420 = v0;
    match m2420 {
        Adt11::C0 => {
            Str::lit("public")
        }
        Adt11::C1 => {
            Str::lit("private")
        }
    }
}

pub fn f630(p2423: List<Adt37>, p2424: List<Adt6>) -> List<Adt6> {
    let m2422 = (p2423, p2424);
    {
        let mut m2422 = m2422;
        loop {
            let step = {
                let (v0, v1) = m2422;
                let m2421 = v0;
                match m2421.uncons() {
                    None => {
                        Err::<(List<Adt37>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt37>, List<Adt6>), List<Adt6>>((v3, List::cons(f668(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2422 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f631(p2428: List<Adt40>, p2429: List<Adt6>) -> R<Option<List<Adt6>>> {
    let m2427 = (p2428, p2429);
    {
        let mut m2427 = m2427;
        loop {
            let step = {
                let (v0, v1) = m2427;
                let m2425 = v0;
                match m2425.uncons() {
                    None => {
                        Err::<(List<Adt40>, List<Adt6>), R<Option<List<Adt6>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m2426 = f669(v2)?;
                        match m2426 {
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
                Ok(next) => { m2427 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f632(p2432: List<Adt46>, p2433: List<Adt6>) -> R<List<Adt6>> {
    let m2431 = (p2432, p2433);
    {
        let mut m2431 = m2431;
        loop {
            let step = {
                let (v0, v1) = m2431;
                let m2430 = v0;
                match m2430.uncons() {
                    None => {
                        Err::<(List<Adt46>, List<Adt6>), R<List<Adt6>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt46>, List<Adt6>), R<List<Adt6>>>((v3, List::cons(f670(v2)?, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2431 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f633(p2436: List<Adt48>, p2437: List<Adt6>) -> List<Adt6> {
    let m2435 = (p2436, p2437);
    {
        let mut m2435 = m2435;
        loop {
            let step = {
                let (v0, v1) = m2435;
                let m2434 = v0;
                match m2434.uncons() {
                    None => {
                        Err::<(List<Adt48>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt48>, List<Adt6>), List<Adt6>>((v3, List::cons(f671(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2435 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f634(p2440: List<Adt56>, p2441: List<Adt6>) -> List<Adt6> {
    let m2439 = (p2440, p2441);
    {
        let mut m2439 = m2439;
        loop {
            let step = {
                let (v0, v1) = m2439;
                let m2438 = v0;
                match m2438.uncons() {
                    None => {
                        Err::<(List<Adt56>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt56>, List<Adt6>), List<Adt6>>((v3, List::cons(f672(v2), v1)))
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

pub fn f635(p2444: List<Adt58>, p2445: List<Adt6>) -> List<Adt6> {
    let m2443 = (p2444, p2445);
    {
        let mut m2443 = m2443;
        loop {
            let step = {
                let (v0, v1) = m2443;
                let m2442 = v0;
                match m2442.uncons() {
                    None => {
                        Err::<(List<Adt58>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt58>, List<Adt6>), List<Adt6>>((v3, List::cons(f673(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2443 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f636(p2449: List<Adt61>, p2450: List<Adt6>) -> R<Option<List<Adt6>>> {
    let m2448 = (p2449, p2450);
    {
        let mut m2448 = m2448;
        loop {
            let step = {
                let (v0, v1) = m2448;
                let m2446 = v0;
                match m2446.uncons() {
                    None => {
                        Err::<(List<Adt61>, List<Adt6>), R<Option<List<Adt6>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m2447 = f674(v2)?;
                        match m2447 {
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
                Ok(next) => { m2448 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f637(v0: Adt66) -> R<Option<Adt6>> {
    let m2451 = v0;
    match m2451 {
        Adt66::C0(v1) => {
            Ok(Some(f675(Str::lit("elicitation/create"), Some(f676(v1)))))
        }
        Adt66::C1(v2) => {
            let m2452 = f677(v2)?;
            match m2452 {
                None => {
                    Ok(None::<Adt6>)
                }
                Some(v3) => {
                    Ok(Some(f675(Str::lit("sampling/createMessage"), Some(v3))))
                }
            }
        }
        Adt66::C2(v4) => {
            Ok(Some(f675(Str::lit("roots/list"), {
                let m2453 = v4;
                match m2453 {
                    None => {
                        None::<Adt6>
                    }
                    Some(v5) => {
                        Some(Adt6::C5(match v5 {
                            Adt5::C0(h2454) => {
                                h2454
                            }
                        }))
                    }
                }
            })))
        }
    }
}

pub fn f638(v0: Option<List<Adt2>>) -> List<(Str, Adt6)> {
    let m2455 = v0;
    match m2455 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("icons"), Adt6::C4(f678(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f639() -> List<u64> {
    List::cons(67u64, List::cons(111u64, List::cons(110u64, List::cons(116u64, List::cons(101u64, List::cons(110u64, List::cons(116u64, List::cons(45u64, List::cons(84u64, List::cons(121u64, List::cons(112u64, List::cons(101u64, List::<u64>::nil()))))))))))))
}

pub fn f640() -> List<u64> {
    List::cons(67u64, List::cons(97u64, List::cons(99u64, List::cons(104u64, List::cons(101u64, List::cons(45u64, List::cons(67u64, List::cons(111u64, List::cons(110u64, List::cons(116u64, List::cons(114u64, List::cons(111u64, List::cons(108u64, List::<u64>::nil())))))))))))))
}

pub fn f641() -> List<u64> {
    List::cons(110u64, List::cons(111u64, List::cons(45u64, List::cons(99u64, List::cons(97u64, List::cons(99u64, List::cons(104u64, List::cons(101u64, List::<u64>::nil()))))))))
}

pub fn f642() -> List<u64> {
    List::cons(88u64, List::cons(45u64, List::cons(65u64, List::cons(99u64, List::cons(99u64, List::cons(101u64, List::cons(108u64, List::cons(45u64, List::cons(66u64, List::cons(117u64, List::cons(102u64, List::cons(102u64, List::cons(101u64, List::cons(114u64, List::cons(105u64, List::cons(110u64, List::cons(103u64, List::<u64>::nil())))))))))))))))))
}

pub fn f643() -> List<u64> {
    List::cons(110u64, List::cons(111u64, List::<u64>::nil()))
}

pub fn f644(p2460: List<List<u64>>, p2461: List<u64>) -> List<u64> {
    let m2459 = (p2460, p2461);
    {
        let mut m2459 = m2459;
        loop {
            let step = {
                let (v0, v1) = m2459;
                let m2456 = v0;
                match m2456.uncons() {
                    None => {
                        Err::<(List<List<u64>>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<u64>>, List<u64>), List<u64>>((v3, {
                            let a2457 = f679(v2);
                            let a2458 = v1;
                            append_list(a2457, a2458)
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m2459 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f645(v0: Str) -> R<Str> {
    let m2464 = {
        let a2463 = f87(f88(), f267(f680(f18({
            let a2462 = v0;
            utf8_encode(a2462)
        })?, List::<u64>::nil()), List::<u64>::nil()));
        utf8_decode(a2463)
    };
    match m2464 {
        None => {
            Ok(Str::lit("?"))
        }
        Some(v1) => {
            Ok(v1)
        }
    }
}

pub fn f646(p2471: List<u64>) -> bool {
    let m2470 = p2471;
    {
        let mut m2470 = m2470;
        loop {
            let step = {
                let v0 = m2470;
                let m2465 = v0;
                match m2465.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let a2466 = v1;
                        let a2467 = 48u64;
                        if if nat_eq(a2466, a2467) {
                            true
                        } else {
                            let a2468 = v1;
                            let a2469 = 46u64;
                            nat_eq(a2468, a2469)
                        } {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2470 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f647(v0: Adt144) -> bool {
    let m2472 = v0;
    match m2472 {
        Adt144::C0 => {
            true
        }
        Adt144::C1 => {
            false
        }
        Adt144::C2 => {
            false
        }
        Adt144::C3 => {
            false
        }
        Adt144::C4 => {
            false
        }
        Adt144::C5 => {
            false
        }
        Adt144::C6 => {
            false
        }
        Adt144::C7 => {
            false
        }
        Adt144::C8 => {
            false
        }
        Adt144::C9 => {
            false
        }
        Adt144::C10 => {
            false
        }
        Adt144::C11 => {
            false
        }
    }
}

pub fn f648(v0: Adt16) -> Adt144 {
    let m2473 = v0;
    match m2473 {
        Adt16::C0(_) => {
            Adt144::C0
        }
        Adt16::C1(_) => {
            Adt144::C1
        }
        Adt16::C2(_) => {
            Adt144::C2
        }
        Adt16::C3(_) => {
            Adt144::C3
        }
        Adt16::C4(_) => {
            Adt144::C4
        }
        Adt16::C5(_) => {
            Adt144::C5
        }
        Adt16::C6(_) => {
            Adt144::C6
        }
        Adt16::C7(_) => {
            Adt144::C7
        }
        Adt16::C8(_) => {
            Adt144::C8
        }
        Adt16::C9(_) => {
            Adt144::C9
        }
        Adt16::C10(_) => {
            Adt144::C10
        }
        Adt16::C11(_) => {
            Adt144::C11
        }
    }
}

pub fn f649(v0: Adt0, v1: Adt34) -> Adt133 {
    Adt133::C0(v1, Adt134::C0(Some(match v0 {
        Adt0::C0(h2474, _, _, _, _, _, _) => {
            h2474
        }
    }), None::<Adt5>), List::<Adt135>::nil())
}

pub fn f650(v0: Adt0) -> Adt35 {
    Adt35::C0(match v0.clone() {
        Adt0::C0(_, _, _, h2475, _, _, _) => {
            h2475
        }
    }, f278(), match v0.clone() {
        Adt0::C0(_, _, h2476, _, _, _, _) => {
            h2476
        }
    }, match v0 {
        Adt0::C0(_, h2477, _, _, _, _, _) => {
            h2477
        }
    })
}

pub fn f651(v0: Adt0, v1: Adt16, v2: Result<Adt34, Adt86>) -> Result<Adt133, Adt136> {
    let m2478 = v2;
    match m2478 {
        Ok(v4) => {
            if f681(f648(v1.clone()), v4.clone()) {
                f682(v0, v1, v4)
            } else {
                Err::<Adt133, Adt136>(Adt136::C4(f683()))
            }
        }
        Err(v3) => {
            Err::<Adt133, Adt136>(Adt136::C4(v3))
        }
    }
}

pub fn f652(v0: Bytes, v1: u64, v2: u64, v3: Fn2, v4: Adt16) -> R<Result<Adt34, Adt86>> {
    f684(v0.clone(), v1, v2, v3, v4.clone(), f685(v0, v4)?)
}

pub fn f653(v0: Str, v1: Adt5) -> Result<Option<Str>, Adt137> {
    let m2479 = f173(v0.clone(), v1);
    match m2479 {
        Adt114::C0 => {
            Ok::<Option<Str>, Adt137>(None::<Str>)
        }
        Adt114::C1(v2) => {
            let m2480 = f122(v2);
            match m2480 {
                None => {
                    Err::<Option<Str>, Adt137>(Adt137::C8(v0))
                }
                Some(v3) => {
                    Ok::<Option<Str>, Adt137>(Some(v3))
                }
            }
        }
    }
}

pub fn f654(v0: Str, v1: Adt5) -> Result<Str, Adt137> {
    let m2481 = f173(v0.clone(), v1);
    match m2481 {
        Adt114::C0 => {
            Err::<Str, Adt137>(Adt137::C7(v0))
        }
        Adt114::C1(v2) => {
            let m2482 = f122(v2);
            match m2482 {
                None => {
                    Err::<Str, Adt137>(Adt137::C8(v0))
                }
                Some(v3) => {
                    Ok::<Str, Adt137>(v3)
                }
            }
        }
    }
}

pub fn f655(v0: Str, v1: Adt5) -> Result<Option<Adt5>, Adt137> {
    let m2483 = f173(v0.clone(), v1);
    match m2483 {
        Adt114::C0 => {
            Ok::<Option<Adt5>, Adt137>(None::<Adt5>)
        }
        Adt114::C1(v2) => {
            let m2484 = f213(v2);
            match m2484 {
                None => {
                    Err::<Option<Adt5>, Adt137>(Adt137::C8(v0))
                }
                Some(v3) => {
                    Ok::<Option<Adt5>, Adt137>(Some(v3))
                }
            }
        }
    }
}

pub fn f656(v0: Str, v1: Adt5) -> Result<Option<List<(Str, Str)>>, Adt137> {
    let m2485 = f173(v0.clone(), v1);
    match m2485 {
        Adt114::C0 => {
            Ok::<Option<List<(Str, Str)>>, Adt137>(None::<List<(Str, Str)>>)
        }
        Adt114::C1(v2) => {
            let m2486 = f213(v2);
            match m2486 {
                None => {
                    Err::<Option<List<(Str, Str)>>, Adt137>(Adt137::C8(v0))
                }
                Some(v3) => {
                    let m2488 = f686(v0, match v3 {
                        Adt5::C0(h2487) => {
                            h2487
                        }
                    });
                    match m2488 {
                        Ok(v4) => {
                            Ok::<Option<List<(Str, Str)>>, Adt137>(Some(v4))
                        }
                        Err(v5) => {
                            Err::<Option<List<(Str, Str)>>, Adt137>(v5)
                        }
                    }
                }
            }
        }
    }
}

pub fn f657(v0: Str, v1: Adt5) -> Result<Adt5, Adt137> {
    let m2489 = f173(v0.clone(), v1);
    match m2489 {
        Adt114::C0 => {
            Err::<Adt5, Adt137>(Adt137::C7(v0))
        }
        Adt114::C1(v2) => {
            let m2490 = f213(v2);
            match m2490 {
                None => {
                    Err::<Adt5, Adt137>(Adt137::C8(v0))
                }
                Some(v3) => {
                    Ok::<Adt5, Adt137>(v3)
                }
            }
        }
    }
}

pub fn f658(v0: Adt5) -> Result<Adt27, Adt137> {
    let m2491 = f654(Str::lit("type"), v0.clone());
    match m2491 {
        Ok(v1) => {
            let a2492 = v1.clone();
            let a2493 = Str::lit("ref/prompt");
            if equal(a2492, a2493) {
                let m2494 = f654(Str::lit("name"), v0.clone());
                match m2494 {
                    Ok(v2) => {
                        let m2495 = f653(Str::lit("title"), v0);
                        match m2495 {
                            Ok(v3) => {
                                Ok::<Adt27, Adt137>(Adt27::C0(Adt28::C0(v2, v3)))
                            }
                            Err(v4) => {
                                Err::<Adt27, Adt137>(v4)
                            }
                        }
                    }
                    Err(v5) => {
                        Err::<Adt27, Adt137>(v5)
                    }
                }
            } else {
                let a2496 = v1;
                let a2497 = Str::lit("ref/resource");
                if equal(a2496, a2497) {
                    let m2498 = f654(Str::lit("uri"), v0);
                    match m2498 {
                        Ok(v6) => {
                            Ok::<Adt27, Adt137>(Adt27::C1(v6))
                        }
                        Err(v7) => {
                            Err::<Adt27, Adt137>(v7)
                        }
                    }
                } else {
                    Err::<Adt27, Adt137>(Adt137::C8(Str::lit("type")))
                }
            }
        }
        Err(v8) => {
            Err::<Adt27, Adt137>(v8)
        }
    }
}

pub fn f659(v0: Adt5) -> Result<Adt29, Adt137> {
    let m2499 = f654(Str::lit("name"), v0.clone());
    match m2499 {
        Ok(v1) => {
            let m2500 = f654(Str::lit("value"), v0);
            match m2500 {
                Ok(v2) => {
                    Ok::<Adt29, Adt137>(Adt29::C0(v1, v2))
                }
                Err(v3) => {
                    Err::<Adt29, Adt137>(v3)
                }
            }
        }
        Err(v4) => {
            Err::<Adt29, Adt137>(v4)
        }
    }
}

pub fn f660(v0: Adt5) -> Result<Option<Adt30>, Adt137> {
    let m2501 = f173(Str::lit("context"), v0);
    match m2501 {
        Adt114::C0 => {
            Ok::<Option<Adt30>, Adt137>(None::<Adt30>)
        }
        Adt114::C1(v1) => {
            let m2502 = f213(v1);
            match m2502 {
                None => {
                    Err::<Option<Adt30>, Adt137>(Adt137::C8(Str::lit("context")))
                }
                Some(v2) => {
                    let m2503 = f656(Str::lit("arguments"), v2);
                    match m2503 {
                        Ok(v3) => {
                            Ok::<Option<Adt30>, Adt137>(Some(Adt30::C0(v3)))
                        }
                        Err(v4) => {
                            Err::<Option<Adt30>, Adt137>(v4)
                        }
                    }
                }
            }
        }
    }
}

pub fn f661(v0: bool, v1: Str, v2: List<Str>) -> Option<Adt136> {
    if v0 {
        None::<Adt136>
    } else {
        Some(Adt136::C1(v1, v2))
    }
}

pub fn f662(v0: Str, p2508: List<Str>) -> bool {
    let m2507 = p2508;
    {
        let mut m2507 = m2507;
        loop {
            let step = {
                let v1 = m2507;
                let m2504 = v1;
                match m2504.uncons() {
                    None => {
                        Err::<List<Str>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a2505 = v2;
                        let a2506 = v0.clone();
                        if equal(a2505, a2506) {
                            Err::<List<Str>, bool>(true)
                        } else {
                            Ok::<List<Str>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2507 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f663(v0: Option<Adt118>, v1: List<Adt120>, v2: Adt5) -> R<Option<Adt118>> {
    let m2509 = v0;
    match m2509 {
        None => {
            f687(v1, f688(v2))
        }
        Some(v3) => {
            Ok(Some(v3))
        }
    }
}

pub fn f664(v0: Option<Str>, v1: Adt151, v2: Adt5) -> Option<Adt118> {
    let m2510 = v1.clone();
    match m2510 {
        Adt151::C0 => {
            None::<Adt118>
        }
        Adt151::C1 => {
            f689(v0, f690(v1, v2))
        }
        Adt151::C2 => {
            f689(v0, f690(v1, v2))
        }
        Adt151::C3 => {
            f689(v0, f690(v1, v2))
        }
    }
}

pub fn f665(v0: Adt114, v1: Adt114) -> Adt156 {
    let m2511 = v0;
    match m2511 {
        Adt114::C0 => {
            Adt156::C1
        }
        Adt114::C1(_) => {
            let m2512 = v1;
            match m2512 {
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

pub fn f666(v0: Adt5, v1: Adt6, v2: Adt6) -> Result<Adt18, Adt137> {
    let m2513 = f122(v1);
    match m2513 {
        None => {
            Err::<Adt18, Adt137>(Adt137::C5)
        }
        Some(v3) => {
            let m2514 = f691(v2);
            match m2514 {
                None => {
                    Err::<Adt18, Adt137>(Adt137::C6)
                }
                Some(v4) => {
                    let m2515 = f655(f692(), v0.clone());
                    match m2515 {
                        Ok(v6) => {
                            let m2516 = f693(v0.clone());
                            match m2516 {
                                Ok(v8) => {
                                    let m2517 = f694(v0.clone());
                                    match m2517 {
                                        Ok(v10) => {
                                            Ok::<Adt18, Adt137>(Adt18::C0(v3, v4, v6, v8, v10, v0))
                                        }
                                        Err(v9) => {
                                            Err::<Adt18, Adt137>(v9)
                                        }
                                    }
                                }
                                Err(v7) => {
                                    Err::<Adt18, Adt137>(v7)
                                }
                            }
                        }
                        Err(v5) => {
                            Err::<Adt18, Adt137>(v5)
                        }
                    }
                }
            }
        }
    }
}

pub fn f667(v0: Adt8) -> Adt6 {
    Adt6::C5(f575(Str::lit("listChanged"), match v0 {
        Adt8::C0(h2518) => {
            h2518
        }
    }))
}

pub fn f668(v0: Adt37) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
        Adt37::C0(h2519, _, _, _, _, _, _, _) => {
            h2519
        }
    })), {
        let a2537 = f570(Str::lit("title"), match v0.clone() {
            Adt37::C0(_, h2520, _, _, _, _, _, _) => {
                h2520
            }
        });
        let a2538 = {
            let a2535 = f638(match v0.clone() {
                Adt37::C0(_, _, h2521, _, _, _, _, _) => {
                    h2521
                }
            });
            let a2536 = {
                let a2533 = f570(Str::lit("description"), match v0.clone() {
                    Adt37::C0(_, _, _, h2522, _, _, _, _) => {
                        h2522
                    }
                });
                let a2534 = List::cons((Str::lit("inputSchema"), Adt6::C5({
                    let m2525 = match v0.clone() {
                        Adt37::C0(_, _, _, _, h2524, _, _, _) => {
                            h2524
                        }
                    };
                    match m2525 {
                        Adt5::C0(h2523) => {
                            h2523
                        }
                    }
                })), {
                    let a2531 = f601(Str::lit("outputSchema"), match v0.clone() {
                        Adt37::C0(_, _, _, _, _, h2526, _, _) => {
                            h2526
                        }
                    });
                    let a2532 = {
                        let a2529 = f695(match v0.clone() {
                            Adt37::C0(_, _, _, _, _, _, h2527, _) => {
                                h2527
                            }
                        });
                        let a2530 = f696(match v0 {
                            Adt37::C0(_, _, _, _, _, _, _, h2528) => {
                                h2528
                            }
                        });
                        append_list(a2529, a2530)
                    };
                    append_list(a2531, a2532)
                });
                append_list(a2533, a2534)
            };
            append_list(a2535, a2536)
        };
        append_list(a2537, a2538)
    }))
}

pub fn f669(v0: Adt40) -> R<Option<Adt6>> {
    let m2539 = v0;
    match m2539 {
        Adt40::C0(v1) => {
            Ok(Some(f697(v1)))
        }
        Adt40::C1(v2) => {
            Ok(Some(f698(Str::lit("image"), match v2.clone() {
                Adt44::C0(h2540, _, _, _) => {
                    h2540
                }
            }, match v2.clone() {
                Adt44::C0(_, h2541, _, _) => {
                    h2541
                }
            }, match v2.clone() {
                Adt44::C0(_, _, h2542, _) => {
                    h2542
                }
            }, match v2 {
                Adt44::C0(_, _, _, h2543) => {
                    h2543
                }
            })))
        }
        Adt40::C2(v3) => {
            Ok(Some(f698(Str::lit("audio"), match v3.clone() {
                Adt45::C0(h2544, _, _, _) => {
                    h2544
                }
            }, match v3.clone() {
                Adt45::C0(_, h2545, _, _) => {
                    h2545
                }
            }, match v3.clone() {
                Adt45::C0(_, _, h2546, _) => {
                    h2546
                }
            }, match v3 {
                Adt45::C0(_, _, _, h2547) => {
                    h2547
                }
            })))
        }
        Adt40::C3(v4) => {
            Ok(Some(f699(v4)?))
        }
        Adt40::C4(v5) => {
            Ok(Some(f700(v5)))
        }
    }
}

pub fn f670(v0: Adt46) -> R<Adt6> {
    Ok(Adt6::C5({
        let a2571 = List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt46::C0(h2548, _, _, _, _, _, _, _, _) => {
                h2548
            }
        })), List::<(Str, Adt6)>::nil());
        let a2572 = {
            let a2569 = f570(Str::lit("title"), match v0.clone() {
                Adt46::C0(_, h2549, _, _, _, _, _, _, _) => {
                    h2549
                }
            });
            let a2570 = {
                let a2567 = f638(match v0.clone() {
                    Adt46::C0(_, _, h2550, _, _, _, _, _, _) => {
                        h2550
                    }
                });
                let a2568 = {
                    let a2565 = List::cons((Str::lit("uri"), Adt6::C3(match v0.clone() {
                        Adt46::C0(_, _, _, h2551, _, _, _, _, _) => {
                            h2551
                        }
                    })), List::<(Str, Adt6)>::nil());
                    let a2566 = {
                        let a2563 = f570(Str::lit("description"), match v0.clone() {
                            Adt46::C0(_, _, _, _, h2552, _, _, _, _) => {
                                h2552
                            }
                        });
                        let a2564 = {
                            let a2561 = f570(Str::lit("mimeType"), match v0.clone() {
                                Adt46::C0(_, _, _, _, _, h2553, _, _, _) => {
                                    h2553
                                }
                            });
                            let a2562 = {
                                let a2559 = f701(match v0.clone() {
                                    Adt46::C0(_, _, _, _, _, _, h2554, _, _) => {
                                        h2554
                                    }
                                });
                                let a2560 = {
                                    let a2557 = f702(match v0.clone() {
                                        Adt46::C0(_, _, _, _, _, _, _, h2555, _) => {
                                            h2555
                                        }
                                    })?;
                                    let a2558 = f696(match v0 {
                                        Adt46::C0(_, _, _, _, _, _, _, _, h2556) => {
                                            h2556
                                        }
                                    });
                                    append_list(a2557, a2558)
                                };
                                append_list(a2559, a2560)
                            };
                            append_list(a2561, a2562)
                        };
                        append_list(a2563, a2564)
                    };
                    append_list(a2565, a2566)
                };
                append_list(a2567, a2568)
            };
            append_list(a2569, a2570)
        };
        append_list(a2571, a2572)
    }))
}

pub fn f671(v0: Adt48) -> Adt6 {
    let m2573 = v0;
    match m2573 {
        Adt48::C0(v1) => {
            Adt6::C5(List::cons((Str::lit("uri"), Adt6::C3(match v1.clone() {
                Adt49::C0(h2574, _, _, _) => {
                    h2574
                }
            })), {
                let a2578 = f570(Str::lit("mimeType"), match v1.clone() {
                    Adt49::C0(_, h2575, _, _) => {
                        h2575
                    }
                });
                let a2579 = List::cons((Str::lit("text"), Adt6::C3(match v1.clone() {
                    Adt49::C0(_, _, h2576, _) => {
                        h2576
                    }
                })), f696(match v1 {
                    Adt49::C0(_, _, _, h2577) => {
                        h2577
                    }
                }));
                append_list(a2578, a2579)
            }))
        }
        Adt48::C1(v2) => {
            Adt6::C5(List::cons((Str::lit("uri"), Adt6::C3(match v2.clone() {
                Adt50::C0(h2580, _, _, _) => {
                    h2580
                }
            })), {
                let a2584 = f570(Str::lit("mimeType"), match v2.clone() {
                    Adt50::C0(_, h2581, _, _) => {
                        h2581
                    }
                });
                let a2585 = List::cons((Str::lit("blob"), Adt6::C3(match v2.clone() {
                    Adt50::C0(_, _, h2582, _) => {
                        h2582
                    }
                })), f696(match v2 {
                    Adt50::C0(_, _, _, h2583) => {
                        h2583
                    }
                }));
                append_list(a2584, a2585)
            }))
        }
    }
}

pub fn f672(v0: Adt56) -> Adt6 {
    Adt6::C5({
        let a2606 = List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt56::C0(h2586, _, _, _, _, _, _, _) => {
                h2586
            }
        })), List::<(Str, Adt6)>::nil());
        let a2607 = {
            let a2604 = f570(Str::lit("title"), match v0.clone() {
                Adt56::C0(_, h2587, _, _, _, _, _, _) => {
                    h2587
                }
            });
            let a2605 = {
                let a2602 = f638(match v0.clone() {
                    Adt56::C0(_, _, h2588, _, _, _, _, _) => {
                        h2588
                    }
                });
                let a2603 = {
                    let a2600 = List::cons((Str::lit("uriTemplate"), Adt6::C3(match v0.clone() {
                        Adt56::C0(_, _, _, h2589, _, _, _, _) => {
                            h2589
                        }
                    })), List::<(Str, Adt6)>::nil());
                    let a2601 = {
                        let a2598 = f570(Str::lit("description"), match v0.clone() {
                            Adt56::C0(_, _, _, _, h2590, _, _, _) => {
                                h2590
                            }
                        });
                        let a2599 = {
                            let a2596 = f570(Str::lit("mimeType"), match v0.clone() {
                                Adt56::C0(_, _, _, _, _, h2591, _, _) => {
                                    h2591
                                }
                            });
                            let a2597 = {
                                let a2594 = f701(match v0.clone() {
                                    Adt56::C0(_, _, _, _, _, _, h2592, _) => {
                                        h2592
                                    }
                                });
                                let a2595 = f696(match v0 {
                                    Adt56::C0(_, _, _, _, _, _, _, h2593) => {
                                        h2593
                                    }
                                });
                                append_list(a2594, a2595)
                            };
                            append_list(a2596, a2597)
                        };
                        append_list(a2598, a2599)
                    };
                    append_list(a2600, a2601)
                };
                append_list(a2602, a2603)
            };
            append_list(a2604, a2605)
        };
        append_list(a2606, a2607)
    })
}

pub fn f673(v0: Adt58) -> Adt6 {
    Adt6::C5({
        let a2623 = List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt58::C0(h2608, _, _, _, _, _) => {
                h2608
            }
        })), List::<(Str, Adt6)>::nil());
        let a2624 = {
            let a2621 = f570(Str::lit("title"), match v0.clone() {
                Adt58::C0(_, h2609, _, _, _, _) => {
                    h2609
                }
            });
            let a2622 = {
                let a2619 = f638(match v0.clone() {
                    Adt58::C0(_, _, h2610, _, _, _) => {
                        h2610
                    }
                });
                let a2620 = {
                    let a2617 = f570(Str::lit("description"), match v0.clone() {
                        Adt58::C0(_, _, _, h2611, _, _) => {
                            h2611
                        }
                    });
                    let a2618 = {
                        let a2615 = {
                            let m2613 = match v0.clone() {
                                Adt58::C0(_, _, _, _, h2612, _) => {
                                    h2612
                                }
                            };
                            match m2613 {
                                None => {
                                    List::<(Str, Adt6)>::nil()
                                }
                                Some(v1) => {
                                    List::cons((Str::lit("arguments"), Adt6::C4(f703(v1))), List::<(Str, Adt6)>::nil())
                                }
                            }
                        };
                        let a2616 = f696(match v0 {
                            Adt58::C0(_, _, _, _, _, h2614) => {
                                h2614
                            }
                        });
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

pub fn f674(v0: Adt61) -> R<Option<Adt6>> {
    let m2626 = f669(match v0.clone() {
        Adt61::C0(_, h2625) => {
            h2625
        }
    })?;
    match m2626 {
        None => {
            Ok(None::<Adt6>)
        }
        Some(v1) => {
            Ok(Some(Adt6::C5(List::cons((Str::lit("role"), Adt6::C3(f704(match v0 {
                Adt61::C0(h2627, _) => {
                    h2627
                }
            }))), List::cons((Str::lit("content"), v1), List::<(Str, Adt6)>::nil())))))
        }
    }
}

pub fn f675(v0: Str, v1: Option<Adt6>) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("method"), Adt6::C3(v0)), f574(Str::lit("params"), v1)))
}

pub fn f676(v0: Adt67) -> Adt6 {
    let m2628 = v0;
    match m2628 {
        Adt67::C0(v1) => {
            f705(v1)
        }
        Adt67::C1(v2) => {
            Adt6::C5(List::cons((Str::lit("mode"), Adt6::C3(Str::lit("url"))), List::cons((Str::lit("message"), Adt6::C3(match v2.clone() {
                Adt70::C0(h2629, _) => {
                    h2629
                }
            })), List::cons((Str::lit("url"), Adt6::C3(match v2 {
                Adt70::C0(_, h2630) => {
                    h2630
                }
            })), List::<(Str, Adt6)>::nil()))))
        }
    }
}

pub fn f677(v0: Adt71) -> R<Option<Adt6>> {
    let m2632 = f706(match v0.clone() {
        Adt71::C0(h2631, _, _, _, _, _, _, _, _, _) => {
            h2631
        }
    })?;
    match m2632 {
        None => {
            Ok(None::<Adt6>)
        }
        Some(v1) => {
            Ok(Some(Adt6::C5(List::cons((Str::lit("messages"), Adt6::C4(v1)), {
                let a2634 = f707(v0.clone());
                let a2635 = List::cons((Str::lit("maxTokens"), f48(match v0.clone() {
                    Adt71::C0(_, _, _, _, _, h2633, _, _, _, _) => {
                        h2633
                    }
                })?), f708(v0));
                append_list(a2634, a2635)
            }))))
        }
    }
}

pub fn f678(v0: List<Adt2>) -> List<Adt6> {
    f411(f709(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f679(v0: List<u64>) -> List<u64> {
    let a2638 = f710();
    let a2639 = {
        let a2636 = v0;
        let a2637 = f711();
        append_list(a2636, a2637)
    };
    append_list(a2638, a2639)
}

pub fn f680(v0: List<u64>, v1: List<u64>) -> List<u64> {
    f712(v0, 0u64, v1)
}

pub fn f681(v0: Adt144, v1: Adt34) -> bool {
    let m2640 = v1;
    match m2640 {
        Adt34::C0(_) => {
            f713(v0, Adt144::C0)
        }
        Adt34::C1(_) => {
            f713(v0, Adt144::C1)
        }
        Adt34::C2(_) => {
            f713(v0, Adt144::C2)
        }
        Adt34::C3(_) => {
            f713(v0, Adt144::C2)
        }
        Adt34::C4(_) => {
            f713(v0, Adt144::C3)
        }
        Adt34::C5(_) => {
            f713(v0, Adt144::C4)
        }
        Adt34::C6(_) => {
            f713(v0, Adt144::C5)
        }
        Adt34::C7(_) => {
            f713(v0, Adt144::C6)
        }
        Adt34::C8(_) => {
            f713(v0, Adt144::C7)
        }
        Adt34::C9(_) => {
            f713(v0, Adt144::C8)
        }
        Adt34::C10(_) => {
            f713(v0, Adt144::C9)
        }
        Adt34::C11 => {
            if f713(v0.clone(), Adt144::C10) {
                true
            } else {
                f713(v0, Adt144::C11)
            }
        }
        Adt34::C12(_) => {
            f714(v0)
        }
        Adt34::C13(_) => {
            f715(v0)
        }
    }
}

pub fn f682(v0: Adt0, v1: Adt16, v2: Adt34) -> Result<Adt133, Adt136> {
    let m2641 = v2.clone();
    match m2641 {
        Adt34::C0(_) => {
            Ok::<Adt133, Adt136>(f649(v0, v2))
        }
        Adt34::C1(_) => {
            Ok::<Adt133, Adt136>(f649(v0, v2))
        }
        Adt34::C2(_) => {
            Ok::<Adt133, Adt136>(f649(v0, v2))
        }
        Adt34::C3(v8) => {
            Ok::<Adt133, Adt136>(f719(v0, f717(v1), v8))
        }
        Adt34::C4(_) => {
            Ok::<Adt133, Adt136>(f649(v0, v2))
        }
        Adt34::C5(_) => {
            Ok::<Adt133, Adt136>(f649(v0, v2))
        }
        Adt34::C6(_) => {
            Ok::<Adt133, Adt136>(f649(v0, v2))
        }
        Adt34::C7(_) => {
            Ok::<Adt133, Adt136>(f649(v0, v2))
        }
        Adt34::C8(_) => {
            Ok::<Adt133, Adt136>(f649(v0, v2))
        }
        Adt34::C9(_) => {
            Ok::<Adt133, Adt136>(f649(v0, v2))
        }
        Adt34::C10(_) => {
            Ok::<Adt133, Adt136>(f649(v0, v2))
        }
        Adt34::C11 => {
            Ok::<Adt133, Adt136>(f649(v0, v2))
        }
        Adt34::C12(v3) => {
            f716(v0, f717(v1), v3)
        }
        Adt34::C13(v4) => {
            f718(v0, f717(v1), v4)
        }
    }
}

pub fn f683() -> Adt86 {
    Adt86::C9(Str::lit("the provider answered with a result that does not belong to the method"))
}

pub fn f684(v0: Bytes, v1: u64, v2: u64, v3: Fn2, v4: Adt16, v5: Result<Adt16, Adt86>) -> R<Result<Adt34, Adt86>> {
    let m2642 = v5;
    match m2642 {
        Ok(v7) => {
            f720(v0, v4, f721(v1, v2, v3, v7)?)
        }
        Err(v6) => {
            Ok(Err::<Adt34, Adt86>(v6))
        }
    }
}

pub fn f685(v0: Bytes, v1: Adt16) -> R<Result<Adt16, Adt86>> {
    let m2643 = v1.clone();
    match m2643 {
        Adt16::C0(_) => {
            Ok(Ok::<Adt16, Adt86>(v1))
        }
        Adt16::C1(_) => {
            Ok(Ok::<Adt16, Adt86>(v1))
        }
        Adt16::C2(v2) => {
            f722(v0, v2)
        }
        Adt16::C3(_) => {
            Ok(Ok::<Adt16, Adt86>(v1))
        }
        Adt16::C4(v3) => {
            f723(v0, v3)
        }
        Adt16::C5(_) => {
            Ok(Ok::<Adt16, Adt86>(v1))
        }
        Adt16::C6(_) => {
            Ok(Ok::<Adt16, Adt86>(v1))
        }
        Adt16::C7(v4) => {
            f724(v0, v4)
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

pub fn f686(v0: Str, v1: List<(Str, Adt6)>) -> Result<List<(Str, Str)>, Adt137> {
    let m2644 = f725(v0, v1, List::<(Str, Str)>::nil());
    match m2644 {
        Ok(v2) => {
            Ok::<List<(Str, Str)>, Adt137>(f726(v2, List::<(Str, Str)>::nil()))
        }
        Err(v3) => {
            Err::<List<(Str, Str)>, Adt137>(v3)
        }
    }
}

pub fn f687(p2648: List<Adt120>, v1: Adt5) -> R<Option<Adt118>> {
    let m2647 = p2648;
    {
        let mut m2647 = m2647;
        loop {
            let step = {
                let v0 = m2647;
                let m2645 = v0;
                match m2645.uncons() {
                    None => {
                        Err::<List<Adt120>, R<Option<Adt118>>>(Ok(None::<Adt118>))
                    }
                    Some((v2, v3)) => {
                        let m2646 = f727(v2, v1.clone())?;
                        match m2646 {
                            None => {
                                Ok::<List<Adt120>, R<Option<Adt118>>>(v3)
                            }
                            Some(v4) => {
                                Err::<List<Adt120>, R<Option<Adt118>>>(Ok(Some(v4)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2647 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f688(v0: Adt5) -> Adt5 {
    let m2649 = f173(Str::lit("arguments"), v0);
    match m2649 {
        Adt114::C0 => {
            f619()
        }
        Adt114::C1(v1) => {
            let m2650 = f213(v1);
            match m2650 {
                None => {
                    f619()
                }
                Some(v2) => {
                    v2
                }
            }
        }
    }
}

pub fn f689(v0: Option<Str>, v1: Option<Str>) -> Option<Adt118> {
    let m2651 = v0;
    match m2651 {
        None => {
            Some(Adt118::C2)
        }
        Some(v2) => {
            let m2652 = v1;
            match m2652 {
                None => {
                    None::<Adt118>
                }
                Some(v3) => {
                    let a2653 = v2;
                    let a2654 = v3;
                    if equal(a2653, a2654) {
                        None::<Adt118>
                    } else {
                        Some(Adt118::C5)
                    }
                }
            }
        }
    }
}

pub fn f690(v0: Adt151, v1: Adt5) -> Option<Str> {
    let m2655 = v0;
    match m2655 {
        Adt151::C0 => {
            None::<Str>
        }
        Adt151::C1 => {
            let m2656 = f173(Str::lit("name"), v1);
            match m2656 {
                Adt114::C0 => {
                    None::<Str>
                }
                Adt114::C1(v2) => {
                    f122(v2)
                }
            }
        }
        Adt151::C2 => {
            let m2657 = f173(Str::lit("uri"), v1);
            match m2657 {
                Adt114::C0 => {
                    None::<Str>
                }
                Adt114::C1(v3) => {
                    f122(v3)
                }
            }
        }
        Adt151::C3 => {
            let m2658 = f173(Str::lit("taskId"), v1);
            match m2658 {
                Adt114::C0 => {
                    None::<Str>
                }
                Adt114::C1(v4) => {
                    f122(v4)
                }
            }
        }
    }
}

pub fn f691(v0: Adt6) -> Option<Adt19> {
    let m2659 = f213(v0);
    match m2659 {
        None => {
            None::<Adt19>
        }
        Some(v1) => {
            let m2660 = f728(Str::lit("experimental"), v1.clone());
            match m2660 {
                None => {
                    None::<Adt19>
                }
                Some(v2) => {
                    let m2661 = f728(Str::lit("roots"), v1.clone());
                    match m2661 {
                        None => {
                            None::<Adt19>
                        }
                        Some(v3) => {
                            let m2662 = f728(Str::lit("sampling"), v1.clone());
                            match m2662 {
                                None => {
                                    None::<Adt19>
                                }
                                Some(v4) => {
                                    let m2663 = f728(Str::lit("elicitation"), v1.clone());
                                    match m2663 {
                                        None => {
                                            None::<Adt19>
                                        }
                                        Some(v5) => {
                                            let m2664 = f728(Str::lit("extensions"), v1);
                                            match m2664 {
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

pub fn f692() -> Str {
    Str::lit("io.modelcontextprotocol/clientInfo")
}

pub fn f693(v0: Adt5) -> Result<Option<Adt20>, Adt137> {
    let m2665 = f653(f729(), v0);
    match m2665 {
        Ok(v2) => {
            let m2666 = v2;
            match m2666 {
                None => {
                    Ok::<Option<Adt20>, Adt137>(None::<Adt20>)
                }
                Some(v3) => {
                    let m2667 = f730(v3);
                    match m2667 {
                        Adt158::C0 => {
                            Err::<Option<Adt20>, Adt137>(Adt137::C8(f729()))
                        }
                        Adt158::C1(v4) => {
                            Ok::<Option<Adt20>, Adt137>(Some(v4))
                        }
                    }
                }
            }
        }
        Err(v1) => {
            Err::<Option<Adt20>, Adt137>(v1)
        }
    }
}

pub fn f694(v0: Adt5) -> Result<Option<Adt21>, Adt137> {
    let m2668 = f173(f731(), v0);
    match m2668 {
        Adt114::C0 => {
            Ok::<Option<Adt21>, Adt137>(None::<Adt21>)
        }
        Adt114::C1(v1) => {
            let m2669 = f122(v1.clone());
            match m2669 {
                None => {
                    let m2670 = f442(v1);
                    match m2670 {
                        None => {
                            Err::<Option<Adt21>, Adt137>(Adt137::C8(f731()))
                        }
                        Some(v3) => {
                            Ok::<Option<Adt21>, Adt137>(Some(Adt21::C1(v3)))
                        }
                    }
                }
                Some(v2) => {
                    Ok::<Option<Adt21>, Adt137>(Some(Adt21::C0(v2)))
                }
            }
        }
    }
}

pub fn f695(v0: Option<Adt38>) -> List<(Str, Adt6)> {
    let m2671 = v0;
    match m2671 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("annotations"), Adt6::C5({
                let a2683 = f570(Str::lit("title"), match v1.clone() {
                    Adt38::C0(h2672, _, _, _, _) => {
                        h2672
                    }
                });
                let a2684 = {
                    let a2681 = f575(Str::lit("readOnlyHint"), match v1.clone() {
                        Adt38::C0(_, h2673, _, _, _) => {
                            h2673
                        }
                    });
                    let a2682 = {
                        let a2679 = f575(Str::lit("destructiveHint"), match v1.clone() {
                            Adt38::C0(_, _, h2674, _, _) => {
                                h2674
                            }
                        });
                        let a2680 = {
                            let a2677 = f575(Str::lit("idempotentHint"), match v1.clone() {
                                Adt38::C0(_, _, _, h2675, _) => {
                                    h2675
                                }
                            });
                            let a2678 = f575(Str::lit("openWorldHint"), match v1 {
                                Adt38::C0(_, _, _, _, h2676) => {
                                    h2676
                                }
                            });
                            append_list(a2677, a2678)
                        };
                        append_list(a2679, a2680)
                    };
                    append_list(a2681, a2682)
                };
                append_list(a2683, a2684)
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f696(v0: Option<Adt5>) -> List<(Str, Adt6)> {
    f601(Str::lit("_meta"), v0)
}

pub fn f697(v0: Adt41) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("text"))), List::cons((Str::lit("text"), Adt6::C3(match v0.clone() {
        Adt41::C0(h2685, _, _) => {
            h2685
        }
    })), {
        let a2688 = f701(match v0.clone() {
            Adt41::C0(_, h2686, _) => {
                h2686
            }
        });
        let a2689 = f696(match v0 {
            Adt41::C0(_, _, h2687) => {
                h2687
            }
        });
        append_list(a2688, a2689)
    })))
}

pub fn f698(v0: Str, v1: Str, v2: Str, v3: Option<Adt42>, v4: Option<Adt5>) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(v0)), List::cons((Str::lit("data"), Adt6::C3(v1)), List::cons((Str::lit("mimeType"), Adt6::C3(v2)), {
        let a2690 = f701(v3);
        let a2691 = f696(v4);
        append_list(a2690, a2691)
    }))))
}

pub fn f699(v0: Adt46) -> R<Adt6> {
    Ok(Adt6::C5({
        let a2715 = List::cons((Str::lit("type"), Adt6::C3(Str::lit("resource_link"))), List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt46::C0(h2692, _, _, _, _, _, _, _, _) => {
                h2692
            }
        })), List::<(Str, Adt6)>::nil()));
        let a2716 = {
            let a2713 = f570(Str::lit("title"), match v0.clone() {
                Adt46::C0(_, h2693, _, _, _, _, _, _, _) => {
                    h2693
                }
            });
            let a2714 = {
                let a2711 = f638(match v0.clone() {
                    Adt46::C0(_, _, h2694, _, _, _, _, _, _) => {
                        h2694
                    }
                });
                let a2712 = {
                    let a2709 = List::cons((Str::lit("uri"), Adt6::C3(match v0.clone() {
                        Adt46::C0(_, _, _, h2695, _, _, _, _, _) => {
                            h2695
                        }
                    })), List::<(Str, Adt6)>::nil());
                    let a2710 = {
                        let a2707 = f570(Str::lit("description"), match v0.clone() {
                            Adt46::C0(_, _, _, _, h2696, _, _, _, _) => {
                                h2696
                            }
                        });
                        let a2708 = {
                            let a2705 = f570(Str::lit("mimeType"), match v0.clone() {
                                Adt46::C0(_, _, _, _, _, h2697, _, _, _) => {
                                    h2697
                                }
                            });
                            let a2706 = {
                                let a2703 = f701(match v0.clone() {
                                    Adt46::C0(_, _, _, _, _, _, h2698, _, _) => {
                                        h2698
                                    }
                                });
                                let a2704 = {
                                    let a2701 = f702(match v0.clone() {
                                        Adt46::C0(_, _, _, _, _, _, _, h2699, _) => {
                                            h2699
                                        }
                                    })?;
                                    let a2702 = f696(match v0 {
                                        Adt46::C0(_, _, _, _, _, _, _, _, h2700) => {
                                            h2700
                                        }
                                    });
                                    append_list(a2701, a2702)
                                };
                                append_list(a2703, a2704)
                            };
                            append_list(a2705, a2706)
                        };
                        append_list(a2707, a2708)
                    };
                    append_list(a2709, a2710)
                };
                append_list(a2711, a2712)
            };
            append_list(a2713, a2714)
        };
        append_list(a2715, a2716)
    }))
}

pub fn f700(v0: Adt47) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("resource"))), List::cons((Str::lit("resource"), f671(match v0.clone() {
        Adt47::C0(h2717, _, _) => {
            h2717
        }
    })), {
        let a2720 = f701(match v0.clone() {
            Adt47::C0(_, h2718, _) => {
                h2718
            }
        });
        let a2721 = f696(match v0 {
            Adt47::C0(_, _, h2719) => {
                h2719
            }
        });
        append_list(a2720, a2721)
    })))
}

pub fn f701(v0: Option<Adt42>) -> List<(Str, Adt6)> {
    let m2722 = v0;
    match m2722 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("annotations"), Adt6::C5({
                let a2728 = f732(match v1.clone() {
                    Adt42::C0(h2723, _, _) => {
                        h2723
                    }
                });
                let a2729 = {
                    let a2726 = f733(match v1.clone() {
                        Adt42::C0(_, h2724, _) => {
                            h2724
                        }
                    });
                    let a2727 = f570(Str::lit("lastModified"), match v1 {
                        Adt42::C0(_, _, h2725) => {
                            h2725
                        }
                    });
                    append_list(a2726, a2727)
                };
                append_list(a2728, a2729)
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f702(v0: Option<u64>) -> R<List<(Str, Adt6)>> {
    let m2730 = v0;
    match m2730 {
        None => {
            Ok(List::<(Str, Adt6)>::nil())
        }
        Some(v1) => {
            Ok(List::cons((Str::lit("size"), f48(v1)?), List::<(Str, Adt6)>::nil()))
        }
    }
}

pub fn f703(v0: List<Adt59>) -> List<Adt6> {
    f411(f734(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f704(v0: Adt43) -> Str {
    let m2731 = v0;
    match m2731 {
        Adt43::C0 => {
            Str::lit("user")
        }
        Adt43::C1 => {
            Str::lit("assistant")
        }
    }
}

pub fn f705(v0: Adt68) -> Adt6 {
    Adt6::C5({
        let a2738 = {
            let m2733 = match v0.clone() {
                Adt68::C0(h2732, _, _) => {
                    h2732
                }
            };
            match m2733 {
                None => {
                    List::<(Str, Adt6)>::nil()
                }
                Some(_) => {
                    List::cons((Str::lit("mode"), Adt6::C3(Str::lit("form"))), List::<(Str, Adt6)>::nil())
                }
            }
        };
        let a2739 = List::cons((Str::lit("message"), Adt6::C3(match v0.clone() {
            Adt68::C0(_, h2734, _) => {
                h2734
            }
        })), List::cons((Str::lit("requestedSchema"), Adt6::C5({
            let m2737 = match v0 {
                Adt68::C0(_, _, h2736) => {
                    h2736
                }
            };
            match m2737 {
                Adt5::C0(h2735) => {
                    h2735
                }
            }
        })), List::<(Str, Adt6)>::nil()));
        append_list(a2738, a2739)
    })
}

pub fn f706(v0: List<Adt72>) -> R<Option<List<Adt6>>> {
    let m2740 = f735(v0, List::<Adt6>::nil())?;
    match m2740 {
        None => {
            Ok(None::<List<Adt6>>)
        }
        Some(v1) => {
            Ok(Some(f411(v1, List::<Adt6>::nil())))
        }
    }
}

pub fn f707(v0: Adt71) -> List<(Str, Adt6)> {
    let a2751 = {
        let m2742 = match v0.clone() {
            Adt71::C0(_, h2741, _, _, _, _, _, _, _, _) => {
                h2741
            }
        };
        match m2742 {
            None => {
                List::<(Str, Adt6)>::nil()
            }
            Some(v1) => {
                List::cons((Str::lit("modelPreferences"), f736(v1)), List::<(Str, Adt6)>::nil())
            }
        }
    };
    let a2752 = {
        let a2749 = f570(Str::lit("systemPrompt"), match v0.clone() {
            Adt71::C0(_, _, h2743, _, _, _, _, _, _, _) => {
                h2743
            }
        });
        let a2750 = {
            let a2747 = {
                let m2745 = match v0.clone() {
                    Adt71::C0(_, _, _, h2744, _, _, _, _, _, _) => {
                        h2744
                    }
                };
                match m2745 {
                    None => {
                        List::<(Str, Adt6)>::nil()
                    }
                    Some(v2) => {
                        List::cons((Str::lit("includeContext"), Adt6::C3(f737(v2))), List::<(Str, Adt6)>::nil())
                    }
                }
            };
            let a2748 = f738(Str::lit("temperature"), match v0 {
                Adt71::C0(_, _, _, _, h2746, _, _, _, _, _) => {
                    h2746
                }
            });
            append_list(a2747, a2748)
        };
        append_list(a2749, a2750)
    };
    append_list(a2751, a2752)
}

pub fn f708(v0: Adt71) -> List<(Str, Adt6)> {
    let a2764 = {
        let m2754 = match v0.clone() {
            Adt71::C0(_, _, _, _, _, _, h2753, _, _, _) => {
                h2753
            }
        };
        match m2754 {
            None => {
                List::<(Str, Adt6)>::nil()
            }
            Some(v1) => {
                List::cons((Str::lit("stopSequences"), f568(v1)), List::<(Str, Adt6)>::nil())
            }
        }
    };
    let a2765 = {
        let a2762 = f601(Str::lit("metadata"), match v0.clone() {
            Adt71::C0(_, _, _, _, _, _, _, h2755, _, _) => {
                h2755
            }
        });
        let a2763 = {
            let a2760 = {
                let m2757 = match v0.clone() {
                    Adt71::C0(_, _, _, _, _, _, _, _, h2756, _) => {
                        h2756
                    }
                };
                match m2757 {
                    None => {
                        List::<(Str, Adt6)>::nil()
                    }
                    Some(v2) => {
                        List::cons((Str::lit("tools"), Adt6::C4(f572(v2))), List::<(Str, Adt6)>::nil())
                    }
                }
            };
            let a2761 = {
                let m2759 = match v0 {
                    Adt71::C0(_, _, _, _, _, _, _, _, _, h2758) => {
                        h2758
                    }
                };
                match m2759 {
                    None => {
                        List::<(Str, Adt6)>::nil()
                    }
                    Some(v3) => {
                        List::cons((Str::lit("toolChoice"), Adt6::C5(f739(v3))), List::<(Str, Adt6)>::nil())
                    }
                }
            };
            append_list(a2760, a2761)
        };
        append_list(a2762, a2763)
    };
    append_list(a2764, a2765)
}

pub fn f709(p2768: List<Adt2>, p2769: List<Adt6>) -> List<Adt6> {
    let m2767 = (p2768, p2769);
    {
        let mut m2767 = m2767;
        loop {
            let step = {
                let (v0, v1) = m2767;
                let m2766 = v0;
                match m2766.uncons() {
                    None => {
                        Err::<(List<Adt2>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt2>, List<Adt6>), List<Adt6>>((v3, List::cons(f740(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2767 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f710() -> List<u64> {
    List::cons(100u64, List::cons(97u64, List::cons(116u64, List::cons(97u64, List::cons(58u64, List::cons(32u64, List::<u64>::nil()))))))
}

pub fn f711() -> List<u64> {
    List::cons(10u64, List::cons(10u64, List::<u64>::nil()))
}

pub fn f712(p2774: List<u64>, p2775: u64, p2776: List<u64>) -> List<u64> {
    let m2773 = (p2774, (p2775, p2776));
    {
        let mut m2773 = m2773;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2773;
                let m2770 = v0;
                match m2770.uncons() {
                    None => {
                        Err::<(List<u64>, (u64, List<u64>)), List<u64>>(f741(v1, v2))
                    }
                    Some((v3, v4)) => {
                        let v5: Adt157 = f742(v1, v3, v2);
                        Ok::<(List<u64>, (u64, List<u64>)), List<u64>>((v4, (match v5.clone() {
                            Adt157::C0(h2771, _) => {
                                h2771
                            }
                        }, match v5 {
                            Adt157::C0(_, h2772) => {
                                h2772
                            }
                        })))
                    }
                }
            };
            match step {
                Ok(next) => { m2773 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f713(v0: Adt144, v1: Adt144) -> bool {
    let a2777 = f539(v0);
    let a2778 = f539(v1);
    equal(a2777, a2778)
}

pub fn f714(v0: Adt144) -> bool {
    let m2779 = v0;
    match m2779 {
        Adt144::C0 => {
            false
        }
        Adt144::C1 => {
            false
        }
        Adt144::C2 => {
            true
        }
        Adt144::C3 => {
            false
        }
        Adt144::C4 => {
            true
        }
        Adt144::C5 => {
            false
        }
        Adt144::C6 => {
            false
        }
        Adt144::C7 => {
            true
        }
        Adt144::C8 => {
            false
        }
        Adt144::C9 => {
            false
        }
        Adt144::C10 => {
            false
        }
        Adt144::C11 => {
            false
        }
    }
}

pub fn f715(v0: Adt144) -> bool {
    let m2780 = v0;
    match m2780 {
        Adt144::C0 => {
            false
        }
        Adt144::C1 => {
            false
        }
        Adt144::C2 => {
            true
        }
        Adt144::C3 => {
            false
        }
        Adt144::C4 => {
            false
        }
        Adt144::C5 => {
            false
        }
        Adt144::C6 => {
            false
        }
        Adt144::C7 => {
            false
        }
        Adt144::C8 => {
            false
        }
        Adt144::C9 => {
            false
        }
        Adt144::C10 => {
            false
        }
        Adt144::C11 => {
            false
        }
    }
}

pub fn f716(v0: Adt0, v1: Adt18, v2: Adt83) -> Result<Adt133, Adt136> {
    f743(v0, v2.clone(), f744(match v1 {
        Adt18::C0(_, h2781, _, _, _, _) => {
            h2781
        }
    }, f583(v2)))
}

pub fn f717(v0: Adt16) -> Adt18 {
    let m2782 = v0;
    match m2782 {
        Adt16::C0(v1) => {
            match v1 {
                Adt17::C0(h2783) => {
                    h2783
                }
            }
        }
        Adt16::C1(v2) => {
            match v2 {
                Adt22::C0(h2784, _) => {
                    h2784
                }
            }
        }
        Adt16::C2(v3) => {
            match v3 {
                Adt23::C0(h2785, _, _, _, _) => {
                    h2785
                }
            }
        }
        Adt16::C3(v4) => {
            match v4 {
                Adt22::C0(h2786, _) => {
                    h2786
                }
            }
        }
        Adt16::C4(v5) => {
            match v5 {
                Adt24::C0(h2787, _, _, _) => {
                    h2787
                }
            }
        }
        Adt16::C5(v6) => {
            match v6 {
                Adt22::C0(h2788, _) => {
                    h2788
                }
            }
        }
        Adt16::C6(v7) => {
            match v7 {
                Adt22::C0(h2789, _) => {
                    h2789
                }
            }
        }
        Adt16::C7(v8) => {
            match v8 {
                Adt25::C0(h2790, _, _, _, _) => {
                    h2790
                }
            }
        }
        Adt16::C8(v9) => {
            match v9 {
                Adt26::C0(h2791, _, _, _) => {
                    h2791
                }
            }
        }
        Adt16::C9(v10) => {
            match v10 {
                Adt31::C0(h2792, _) => {
                    h2792
                }
            }
        }
        Adt16::C10(v11) => {
            match v11 {
                Adt32::C0(h2793, _, _) => {
                    h2793
                }
            }
        }
        Adt16::C11(v12) => {
            match v12 {
                Adt33::C0(h2794, _) => {
                    h2794
                }
            }
        }
    }
}

pub fn f718(v0: Adt0, v1: Adt18, v2: Adt84) -> Result<Adt133, Adt136> {
    f745(v0, v2, f746(match v1 {
        Adt18::C0(_, h2795, _, _, _, _) => {
            h2795
        }
    }))
}

pub fn f719(v0: Adt0, v1: Adt18, v2: Adt51) -> Adt133 {
    Adt133::C0(Adt34::C2(match v2.clone() {
        Adt51::C0(h2796, _) => {
            h2796
        }
    }), {
        let m2798 = f649(v0, Adt34::C11);
        match m2798 {
            Adt133::C0(_, h2797, _) => {
                h2797
            }
        }
    }, f747(v1, match v2 {
        Adt51::C0(_, h2799) => {
            h2799
        }
    }))
}

pub fn f720(v0: Bytes, v1: Adt16, v2: Result<Adt34, Adt86>) -> R<Result<Adt34, Adt86>> {
    let m2800 = v2;
    match m2800 {
        Ok(v4) => {
            f748(v0, f648(v1.clone()), f749(v1), v4)
        }
        Err(v3) => {
            Ok(Err::<Adt34, Adt86>(v3))
        }
    }
}

pub fn f721(v0: u64, v1: u64, v2: Fn2, v3: Adt16) -> R<Result<Adt34, Adt86>> {
    let m2801 = v3.clone();
    match m2801 {
        Adt16::C0(_) => {
            let c2803: Fn2 = v2;
            c2803.apply(v3)
        }
        Adt16::C1(_) => {
            let c2804: Fn2 = v2;
            c2804.apply(v3)
        }
        Adt16::C2(_) => {
            if f750(v0, v1) {
                let c2802: Fn2 = v2;
                c2802.apply(v3)
            } else {
                Ok(Err::<Adt34, Adt86>(Adt86::C7(v0)))
            }
        }
        Adt16::C3(_) => {
            let c2805: Fn2 = v2;
            c2805.apply(v3)
        }
        Adt16::C4(_) => {
            let c2806: Fn2 = v2;
            c2806.apply(v3)
        }
        Adt16::C5(_) => {
            let c2807: Fn2 = v2;
            c2807.apply(v3)
        }
        Adt16::C6(_) => {
            let c2808: Fn2 = v2;
            c2808.apply(v3)
        }
        Adt16::C7(_) => {
            let c2809: Fn2 = v2;
            c2809.apply(v3)
        }
        Adt16::C8(_) => {
            let c2810: Fn2 = v2;
            c2810.apply(v3)
        }
        Adt16::C9(_) => {
            let c2811: Fn2 = v2;
            c2811.apply(v3)
        }
        Adt16::C10(_) => {
            let c2812: Fn2 = v2;
            c2812.apply(v3)
        }
        Adt16::C11(_) => {
            let c2813: Fn2 = v2;
            c2813.apply(v3)
        }
    }
}

pub fn f722(v0: Bytes, v1: Adt23) -> R<Result<Adt16, Adt86>> {
    let m2817 = f751(v0, Str::lit("tools/call"), match v1.clone() {
        Adt23::C0(_, h2814, _, _, _) => {
            h2814
        }
    }, match v1.clone() {
        Adt23::C0(_, _, _, _, h2815) => {
            h2815
        }
    }, match v1.clone() {
        Adt23::C0(_, _, _, h2816, _) => {
            h2816
        }
    })?;
    match m2817 {
        Ok(v3) => {
            Ok(Ok::<Adt16, Adt86>(Adt16::C2(Adt23::C0(match v1.clone() {
                Adt23::C0(h2818, _, _, _, _) => {
                    h2818
                }
            }, match v1.clone() {
                Adt23::C0(_, h2819, _, _, _) => {
                    h2819
                }
            }, match v1.clone() {
                Adt23::C0(_, _, h2820, _, _) => {
                    h2820
                }
            }, match v1 {
                Adt23::C0(_, _, _, h2821, _) => {
                    h2821
                }
            }, v3))))
        }
        Err(v2) => {
            Ok(Err::<Adt16, Adt86>(v2))
        }
    }
}

pub fn f723(v0: Bytes, v1: Adt24) -> R<Result<Adt16, Adt86>> {
    let m2825 = f751(v0, Str::lit("resources/read"), match v1.clone() {
        Adt24::C0(_, h2822, _, _) => {
            h2822
        }
    }, match v1.clone() {
        Adt24::C0(_, _, _, h2823) => {
            h2823
        }
    }, match v1.clone() {
        Adt24::C0(_, _, h2824, _) => {
            h2824
        }
    })?;
    match m2825 {
        Ok(v3) => {
            Ok(Ok::<Adt16, Adt86>(Adt16::C4(Adt24::C0(match v1.clone() {
                Adt24::C0(h2826, _, _, _) => {
                    h2826
                }
            }, match v1.clone() {
                Adt24::C0(_, h2827, _, _) => {
                    h2827
                }
            }, match v1 {
                Adt24::C0(_, _, h2828, _) => {
                    h2828
                }
            }, v3))))
        }
        Err(v2) => {
            Ok(Err::<Adt16, Adt86>(v2))
        }
    }
}

pub fn f724(v0: Bytes, v1: Adt25) -> R<Result<Adt16, Adt86>> {
    let m2832 = f751(v0, Str::lit("prompts/get"), match v1.clone() {
        Adt25::C0(_, h2829, _, _, _) => {
            h2829
        }
    }, match v1.clone() {
        Adt25::C0(_, _, _, _, h2830) => {
            h2830
        }
    }, match v1.clone() {
        Adt25::C0(_, _, _, h2831, _) => {
            h2831
        }
    })?;
    match m2832 {
        Ok(v3) => {
            Ok(Ok::<Adt16, Adt86>(Adt16::C7(Adt25::C0(match v1.clone() {
                Adt25::C0(h2833, _, _, _, _) => {
                    h2833
                }
            }, match v1.clone() {
                Adt25::C0(_, h2834, _, _, _) => {
                    h2834
                }
            }, match v1.clone() {
                Adt25::C0(_, _, h2835, _, _) => {
                    h2835
                }
            }, match v1 {
                Adt25::C0(_, _, _, h2836, _) => {
                    h2836
                }
            }, v3))))
        }
        Err(v2) => {
            Ok(Err::<Adt16, Adt86>(v2))
        }
    }
}

pub fn f725(v0: Str, p2842: List<(Str, Adt6)>, p2843: List<(Str, Str)>) -> Result<List<(Str, Str)>, Adt137> {
    let m2841 = (p2842, p2843);
    {
        let mut m2841 = m2841;
        loop {
            let step = {
                let (v1, v2) = m2841;
                let m2837 = v1;
                match m2837.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<(Str, Str)>), Result<List<(Str, Str)>, Adt137>>(Ok::<List<(Str, Str)>, Adt137>(v2))
                    }
                    Some((v3, v4)) => {
                        let m2839 = f122({
                            let (_, h2838) = v3;
                            h2838
                        });
                        match m2839 {
                            None => {
                                Err::<(List<(Str, Adt6)>, List<(Str, Str)>), Result<List<(Str, Str)>, Adt137>>(Err::<List<(Str, Str)>, Adt137>(Adt137::C8(v0.clone())))
                            }
                            Some(v5) => {
                                Ok::<(List<(Str, Adt6)>, List<(Str, Str)>), Result<List<(Str, Str)>, Adt137>>((v4, List::cons(({
                                    let (h2840, _) = v3;
                                    h2840
                                }, v5), v2)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2841 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f726(p2846: List<(Str, Str)>, p2847: List<(Str, Str)>) -> List<(Str, Str)> {
    let m2845 = (p2846, p2847);
    {
        let mut m2845 = m2845;
        loop {
            let step = {
                let (v0, v1) = m2845;
                let m2844 = v0;
                match m2844.uncons() {
                    None => {
                        Err::<(List<(Str, Str)>, List<(Str, Str)>), List<(Str, Str)>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Str)>, List<(Str, Str)>), List<(Str, Str)>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2845 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f727(v0: Adt120, v1: Adt5) -> R<Option<Adt118>> {
    let m2849 = f752(match v0.clone() {
        Adt120::C0(_, h2848, _, _) => {
            h2848
        }
    }, v1);
    match m2849 {
        Adt114::C0 => {
            Ok(f753(v0))
        }
        Adt114::C1(v2) => {
            if f754(v2.clone()) {
                Ok(f753(v0))
            } else {
                f755(v0, v2)
            }
        }
    }
}

pub fn f728(v0: Str, v1: Adt5) -> Option<Option<Adt5>> {
    let m2850 = f173(v0, v1);
    match m2850 {
        Adt114::C0 => {
            Some(None::<Adt5>)
        }
        Adt114::C1(v2) => {
            let m2851 = f213(v2);
            match m2851 {
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

pub fn f729() -> Str {
    Str::lit("io.modelcontextprotocol/logLevel")
}

pub fn f730(v0: Str) -> Adt158 {
    let a2852 = v0.clone();
    let a2853 = Str::lit("debug");
    if equal(a2852, a2853) {
        Adt158::C1(Adt20::C0)
    } else {
        let a2854 = v0.clone();
        let a2855 = Str::lit("info");
        if equal(a2854, a2855) {
            Adt158::C1(Adt20::C1)
        } else {
            let a2856 = v0.clone();
            let a2857 = Str::lit("notice");
            if equal(a2856, a2857) {
                Adt158::C1(Adt20::C2)
            } else {
                let a2858 = v0.clone();
                let a2859 = Str::lit("warning");
                if equal(a2858, a2859) {
                    Adt158::C1(Adt20::C3)
                } else {
                    let a2860 = v0.clone();
                    let a2861 = Str::lit("error");
                    if equal(a2860, a2861) {
                        Adt158::C1(Adt20::C4)
                    } else {
                        let a2862 = v0.clone();
                        let a2863 = Str::lit("critical");
                        if equal(a2862, a2863) {
                            Adt158::C1(Adt20::C5)
                        } else {
                            let a2864 = v0.clone();
                            let a2865 = Str::lit("alert");
                            if equal(a2864, a2865) {
                                Adt158::C1(Adt20::C6)
                            } else {
                                let a2866 = v0;
                                let a2867 = Str::lit("emergency");
                                if equal(a2866, a2867) {
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

pub fn f731() -> Str {
    Str::lit("progressToken")
}

pub fn f732(v0: Option<List<Adt43>>) -> List<(Str, Adt6)> {
    let m2868 = v0;
    match m2868 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("audience"), Adt6::C4(f756(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f733(v0: Option<Adt7>) -> List<(Str, Adt6)> {
    let m2869 = v0;
    match m2869 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("priority"), Adt6::C2(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f734(p2872: List<Adt59>, p2873: List<Adt6>) -> List<Adt6> {
    let m2871 = (p2872, p2873);
    {
        let mut m2871 = m2871;
        loop {
            let step = {
                let (v0, v1) = m2871;
                let m2870 = v0;
                match m2870.uncons() {
                    None => {
                        Err::<(List<Adt59>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt59>, List<Adt6>), List<Adt6>>((v3, List::cons(f757(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2871 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f735(p2877: List<Adt72>, p2878: List<Adt6>) -> R<Option<List<Adt6>>> {
    let m2876 = (p2877, p2878);
    {
        let mut m2876 = m2876;
        loop {
            let step = {
                let (v0, v1) = m2876;
                let m2874 = v0;
                match m2874.uncons() {
                    None => {
                        Err::<(List<Adt72>, List<Adt6>), R<Option<List<Adt6>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m2875 = f758(v2)?;
                        match m2875 {
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
                Ok(next) => { m2876 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f736(v0: Adt77) -> Adt6 {
    Adt6::C5({
        let a2888 = {
            let m2880 = match v0.clone() {
                Adt77::C0(h2879, _, _, _) => {
                    h2879
                }
            };
            match m2880 {
                None => {
                    List::<(Str, Adt6)>::nil()
                }
                Some(v1) => {
                    List::cons((Str::lit("hints"), Adt6::C4(f411(f759(v1, List::<Adt6>::nil()), List::<Adt6>::nil()))), List::<(Str, Adt6)>::nil())
                }
            }
        };
        let a2889 = {
            let a2886 = f738(Str::lit("costPriority"), match v0.clone() {
                Adt77::C0(_, h2881, _, _) => {
                    h2881
                }
            });
            let a2887 = {
                let a2884 = f738(Str::lit("speedPriority"), match v0.clone() {
                    Adt77::C0(_, _, h2882, _) => {
                        h2882
                    }
                });
                let a2885 = f738(Str::lit("intelligencePriority"), match v0 {
                    Adt77::C0(_, _, _, h2883) => {
                        h2883
                    }
                });
                append_list(a2884, a2885)
            };
            append_list(a2886, a2887)
        };
        append_list(a2888, a2889)
    })
}

pub fn f737(v0: Adt79) -> Str {
    let m2890 = v0;
    match m2890 {
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

pub fn f738(v0: Str, v1: Option<Adt7>) -> List<(Str, Adt6)> {
    let m2891 = v1;
    match m2891 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C2(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f739(v0: Adt80) -> List<(Str, Adt6)> {
    let m2893 = match v0 {
        Adt80::C0(h2892) => {
            h2892
        }
    };
    match m2893 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("mode"), Adt6::C3(f760(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f740(v0: Adt2) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("src"), Adt6::C3(match v0.clone() {
        Adt2::C0(h2894, _, _, _) => {
            h2894
        }
    })), {
        let a2900 = f570(Str::lit("mimeType"), match v0.clone() {
            Adt2::C0(_, h2895, _, _) => {
                h2895
            }
        });
        let a2901 = {
            let a2898 = f761(match v0.clone() {
                Adt2::C0(_, _, h2896, _) => {
                    h2896
                }
            });
            let a2899 = f762(match v0 {
                Adt2::C0(_, _, _, h2897) => {
                    h2897
                }
            });
            append_list(a2898, a2899)
        };
        append_list(a2900, a2901)
    }))
}

pub fn f741(v0: u64, v1: List<u64>) -> List<u64> {
    let a2902 = v0;
    let a2903 = 0u64;
    if nat_eq(a2902, a2903) {
        v1
    } else {
        let a2904 = v0;
        let a2905 = 1u64;
        if nat_eq(a2904, a2905) {
            List::cons(194u64, v1)
        } else {
            let a2906 = v0;
            let a2907 = 2u64;
            if nat_eq(a2906, a2907) {
                List::cons(226u64, v1)
            } else {
                let a2908 = v0;
                let a2909 = 3u64;
                if nat_eq(a2908, a2909) {
                    List::cons(128u64, List::cons(226u64, v1))
                } else {
                    List::cons(129u64, List::cons(226u64, v1))
                }
            }
        }
    }
}

pub fn f742(v0: u64, v1: u64, v2: List<u64>) -> Adt157 {
    let a2910 = v0;
    let a2911 = 0u64;
    if nat_eq(a2910, a2911) {
        f763(v1, v2)
    } else {
        let a2912 = v0;
        let a2913 = 1u64;
        if nat_eq(a2912, a2913) {
            let a2914 = 128u64;
            let a2915 = v1;
            if if nat_le(a2914, a2915) {
                let a2916 = v1;
                let a2917 = 159u64;
                nat_le(a2916, a2917)
            } else {
                false
            } {
                Adt157::C0(0u64, f764(v2))
            } else {
                f763(v1, List::cons(194u64, v2))
            }
        } else {
            let a2918 = v0;
            let a2919 = 2u64;
            if nat_eq(a2918, a2919) {
                let a2920 = v1;
                let a2921 = 128u64;
                if nat_eq(a2920, a2921) {
                    Adt157::C0(3u64, v2)
                } else {
                    let a2922 = v1;
                    let a2923 = 129u64;
                    if nat_eq(a2922, a2923) {
                        Adt157::C0(4u64, v2)
                    } else {
                        f763(v1, List::cons(226u64, v2))
                    }
                }
            } else {
                let a2924 = v0;
                let a2925 = 3u64;
                if nat_eq(a2924, a2925) {
                    let a2926 = 170u64;
                    let a2927 = v1;
                    if if nat_le(a2926, a2927) {
                        let a2928 = v1;
                        let a2929 = 174u64;
                        nat_le(a2928, a2929)
                    } else {
                        false
                    } {
                        Adt157::C0(0u64, f764(v2))
                    } else {
                        f763(v1, List::cons(128u64, List::cons(226u64, v2)))
                    }
                } else {
                    let a2930 = 166u64;
                    let a2931 = v1;
                    if if nat_le(a2930, a2931) {
                        let a2932 = v1;
                        let a2933 = 169u64;
                        nat_le(a2932, a2933)
                    } else {
                        false
                    } {
                        Adt157::C0(0u64, f764(v2))
                    } else {
                        f763(v1, List::cons(129u64, List::cons(226u64, v2)))
                    }
                }
            }
        }
    }
}

pub fn f743(v0: Adt0, v1: Adt83, v2: bool) -> Result<Adt133, Adt136> {
    if v2 {
        Ok::<Adt133, Adt136>(f649(v0, Adt34::C12(v1)))
    } else {
        Err::<Adt133, Adt136>(Adt136::C4(Adt86::C8(f765(f583(v1)))))
    }
}

pub fn f744(v0: Adt19, v1: List<(Str, Adt66)>) -> bool {
    let a2934 = f766(v1.clone());
    if if if bool_not(a2934) {
        true
    } else {
        f542(match v0.clone() {
            Adt19::C0(_, _, _, h2935, _) => {
                h2935
            }
        })
    } {
        let a2936 = f767(v1.clone());
        if bool_not(a2936) {
            true
        } else {
            f542(match v0.clone() {
                Adt19::C0(_, _, h2937, _, _) => {
                    h2937
                }
            })
        }
    } else {
        false
    } {
        let a2938 = f768(v1);
        if bool_not(a2938) {
            true
        } else {
            f542(match v0 {
                Adt19::C0(_, h2939, _, _, _) => {
                    h2939
                }
            })
        }
    } else {
        false
    }
}

pub fn f745(v0: Adt0, v1: Adt84, v2: bool) -> Result<Adt133, Adt136> {
    if v2 {
        Ok::<Adt133, Adt136>(f649(v0, Adt34::C13(v1)))
    } else {
        Err::<Adt133, Adt136>(Adt136::C4(Adt86::C8(f769())))
    }
}

pub fn f746(v0: Adt19) -> bool {
    f543(match v0 {
        Adt19::C0(_, _, _, _, h2940) => {
            h2940
        }
    }, f544())
}

pub fn f747(v0: Adt18, v1: List<Adt52>) -> List<Adt135> {
    let m2942 = match v0 {
        Adt18::C0(_, _, _, _, h2941, _) => {
            h2941
        }
    };
    match m2942 {
        None => {
            List::<Adt135>::nil()
        }
        Some(v2) => {
            f770(f771(v2, v1, List::<Adt135>::nil()), List::<Adt135>::nil())
        }
    }
}

pub fn f748(v0: Bytes, v1: Adt144, v2: Str, v3: Adt34) -> R<Result<Adt34, Adt86>> {
    let m2943 = v3.clone();
    match m2943 {
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
            f772(v0, v1, v2, v4)
        }
        Adt34::C13(_) => {
            Ok(Ok::<Adt34, Adt86>(v3))
        }
    }
}

pub fn f749(v0: Adt16) -> Str {
    let m2944 = v0;
    match m2944 {
        Adt16::C0(_) => {
            Str::lit("")
        }
        Adt16::C1(_) => {
            Str::lit("")
        }
        Adt16::C2(v1) => {
            match v1 {
                Adt23::C0(_, h2945, _, _, _) => {
                    h2945
                }
            }
        }
        Adt16::C3(_) => {
            Str::lit("")
        }
        Adt16::C4(v2) => {
            match v2 {
                Adt24::C0(_, h2946, _, _) => {
                    h2946
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
                Adt25::C0(_, h2947, _, _, _) => {
                    h2947
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

pub fn f750(v0: u64, v1: u64) -> bool {
    let a2948 = v1;
    let a2949 = v0;
    nat_lt(a2948, a2949)
}

pub fn f751(v0: Bytes, v1: Str, v2: Str, v3: Option<Str>, v4: Option<Adt5>) -> R<Result<Option<Str>, Adt86>> {
    Ok(f773(f774(v0, v1, v2, v3)?, f775(v4)))
}

pub fn f752(p2955: List<Str>, p2956: Adt5) -> Adt114 {
    let m2954 = (p2955, p2956);
    {
        let mut m2954 = m2954;
        loop {
            let step = {
                let (v0, v1) = m2954;
                let m2950 = v0;
                match m2950.uncons() {
                    None => {
                        Err::<(List<Str>, Adt5), Adt114>(Adt114::C0)
                    }
                    Some((v2, v3)) => {
                        let m2951 = f173(v2, v1);
                        match m2951 {
                            Adt114::C0 => {
                                Err::<(List<Str>, Adt5), Adt114>(Adt114::C0)
                            }
                            Adt114::C1(v4) => {
                                let m2952 = v3.clone();
                                match m2952.uncons() {
                                    None => {
                                        Err::<(List<Str>, Adt5), Adt114>(Adt114::C1(v4))
                                    }
                                    Some((_, _)) => {
                                        let m2953 = f213(v4);
                                        match m2953 {
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
                Ok(next) => { m2954 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f753(v0: Adt120) -> Option<Adt118> {
    let m2958 = match v0.clone() {
        Adt120::C0(_, _, _, h2957) => {
            h2957
        }
    };
    match m2958 {
        Adt121::C0 => {
            None::<Adt118>
        }
        Adt121::C1 => {
            Some(Adt118::C10(match v0 {
                Adt120::C0(h2959, _, _, _) => {
                    h2959
                }
            }))
        }
        Adt121::C2(_) => {
            None::<Adt118>
        }
    }
}

pub fn f754(v0: Adt6) -> bool {
    let m2960 = v0;
    match m2960 {
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

pub fn f755(v0: Adt120, v1: Adt6) -> R<Option<Adt118>> {
    let m2962 = match v0.clone() {
        Adt120::C0(_, _, _, h2961) => {
            h2961
        }
    };
    match m2962 {
        Adt121::C0 => {
            Ok(Some(Adt118::C8(match v0 {
                Adt120::C0(h2963, _, _, _) => {
                    h2963
                }
            })))
        }
        Adt121::C1 => {
            Ok(Some(Adt118::C10(match v0 {
                Adt120::C0(h2964, _, _, _) => {
                    h2964
                }
            })))
        }
        Adt121::C2(v2) => {
            if f776(match v0.clone() {
                Adt120::C0(_, _, h2965, _) => {
                    h2965
                }
            }, v2, v1)? {
                Ok(None::<Adt118>)
            } else {
                Ok(Some(Adt118::C9(match v0 {
                    Adt120::C0(h2966, _, _, _) => {
                        h2966
                    }
                })))
            }
        }
    }
}

pub fn f756(v0: List<Adt43>) -> List<Adt6> {
    f411(f777(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f757(v0: Adt59) -> Adt6 {
    Adt6::C5({
        let a2975 = List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt59::C0(h2967, _, _, _) => {
                h2967
            }
        })), List::<(Str, Adt6)>::nil());
        let a2976 = {
            let a2973 = f570(Str::lit("title"), match v0.clone() {
                Adt59::C0(_, h2968, _, _) => {
                    h2968
                }
            });
            let a2974 = {
                let a2971 = f570(Str::lit("description"), match v0.clone() {
                    Adt59::C0(_, _, h2969, _) => {
                        h2969
                    }
                });
                let a2972 = f575(Str::lit("required"), match v0 {
                    Adt59::C0(_, _, _, h2970) => {
                        h2970
                    }
                });
                append_list(a2971, a2972)
            };
            append_list(a2973, a2974)
        };
        append_list(a2975, a2976)
    })
}

pub fn f758(v0: Adt72) -> R<Option<Adt6>> {
    let m2978 = f778(match v0.clone() {
        Adt72::C0(_, h2977, _) => {
            h2977
        }
    })?;
    match m2978 {
        None => {
            Ok(None::<Adt6>)
        }
        Some(v1) => {
            Ok(Some(Adt6::C5(List::cons((Str::lit("role"), Adt6::C3(f704(match v0.clone() {
                Adt72::C0(h2979, _, _) => {
                    h2979
                }
            }))), List::cons((Str::lit("content"), v1), f696(match v0 {
                Adt72::C0(_, _, h2980) => {
                    h2980
                }
            }))))))
        }
    }
}

pub fn f759(p2984: List<Adt78>, p2985: List<Adt6>) -> List<Adt6> {
    let m2983 = (p2984, p2985);
    {
        let mut m2983 = m2983;
        loop {
            let step = {
                let (v0, v1) = m2983;
                let m2981 = v0;
                match m2981.uncons() {
                    None => {
                        Err::<(List<Adt78>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt78>, List<Adt6>), List<Adt6>>((v3, List::cons(Adt6::C5(f570(Str::lit("name"), match v2 {
                            Adt78::C0(h2982) => {
                                h2982
                            }
                        })), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2983 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f760(v0: Adt81) -> Str {
    let m2986 = v0;
    match m2986 {
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

pub fn f761(v0: Option<List<Str>>) -> List<(Str, Adt6)> {
    let m2987 = v0;
    match m2987 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("sizes"), Adt6::C4(f534(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f762(v0: Option<Adt3>) -> List<(Str, Adt6)> {
    let m2988 = v0;
    match m2988 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("theme"), Adt6::C3(f779(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f763(v0: u64, v1: List<u64>) -> Adt157 {
    let a2989 = v0;
    let a2990 = 32u64;
    if nat_lt(a2989, a2990) {
        if f780(v0) {
            Adt157::C0(0u64, List::cons(v0, v1))
        } else {
            Adt157::C0(0u64, f764(v1))
        }
    } else {
        let a2991 = v0;
        let a2992 = 127u64;
        if nat_eq(a2991, a2992) {
            Adt157::C0(0u64, f764(v1))
        } else {
            let a2993 = v0;
            let a2994 = 194u64;
            if nat_eq(a2993, a2994) {
                Adt157::C0(1u64, v1)
            } else {
                let a2995 = v0;
                let a2996 = 226u64;
                if nat_eq(a2995, a2996) {
                    Adt157::C0(2u64, v1)
                } else {
                    Adt157::C0(0u64, List::cons(v0, v1))
                }
            }
        }
    }
}

pub fn f764(v0: List<u64>) -> List<u64> {
    List::cons(189u64, List::cons(191u64, List::cons(239u64, v0)))
}

pub fn f765(v0: List<(Str, Adt66)>) -> Adt19 {
    Adt19::C0(None::<Adt5>, if f768(v0.clone()) {
        Some(f619())
    } else {
        None::<Adt5>
    }, if f767(v0.clone()) {
        Some(f619())
    } else {
        None::<Adt5>
    }, if f766(v0) {
        Some(f619())
    } else {
        None::<Adt5>
    }, None::<Adt5>)
}

pub fn f766(p3001: List<(Str, Adt66)>) -> bool {
    let m3000 = p3001;
    {
        let mut m3000 = m3000;
        loop {
            let step = {
                let v0 = m3000;
                let m2997 = v0;
                match m2997.uncons() {
                    None => {
                        Err::<List<(Str, Adt66)>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        let m2999 = {
                            let (_, h2998) = v1;
                            h2998
                        };
                        match m2999 {
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
                Ok(next) => { m3000 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f767(p3006: List<(Str, Adt66)>) -> bool {
    let m3005 = p3006;
    {
        let mut m3005 = m3005;
        loop {
            let step = {
                let v0 = m3005;
                let m3002 = v0;
                match m3002.uncons() {
                    None => {
                        Err::<List<(Str, Adt66)>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        let m3004 = {
                            let (_, h3003) = v1;
                            h3003
                        };
                        match m3004 {
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
                Ok(next) => { m3005 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f768(p3011: List<(Str, Adt66)>) -> bool {
    let m3010 = p3011;
    {
        let mut m3010 = m3010;
        loop {
            let step = {
                let v0 = m3010;
                let m3007 = v0;
                match m3007.uncons() {
                    None => {
                        Err::<List<(Str, Adt66)>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        let m3009 = {
                            let (_, h3008) = v1;
                            h3008
                        };
                        match m3009 {
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
                Ok(next) => { m3010 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f769() -> Adt19 {
    Adt19::C0(None::<Adt5>, None::<Adt5>, None::<Adt5>, None::<Adt5>, Some(Adt5::C0(List::cons((f544(), Adt6::C5(List::<(Str, Adt6)>::nil())), List::<(Str, Adt6)>::nil()))))
}

pub fn f770(p3014: List<Adt135>, p3015: List<Adt135>) -> List<Adt135> {
    let m3013 = (p3014, p3015);
    {
        let mut m3013 = m3013;
        loop {
            let step = {
                let (v0, v1) = m3013;
                let m3012 = v0;
                match m3012.uncons() {
                    None => {
                        Err::<(List<Adt135>, List<Adt135>), List<Adt135>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt135>, List<Adt135>), List<Adt135>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m3013 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f771(v0: Adt21, p3018: List<Adt52>, p3019: List<Adt135>) -> List<Adt135> {
    let m3017 = (p3018, p3019);
    {
        let mut m3017 = m3017;
        loop {
            let step = {
                let (v1, v2) = m3017;
                let m3016 = v1;
                match m3016.uncons() {
                    None => {
                        Err::<(List<Adt52>, List<Adt135>), List<Adt135>>(v2)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt52>, List<Adt135>), List<Adt135>>((v4, List::cons(Adt135::C0(v0.clone(), v3), v2)))
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

pub fn f772(v0: Bytes, v1: Adt144, v2: Str, v3: Adt83) -> R<Result<Adt34, Adt86>> {
    if f714(v1.clone()) {
        if f781(v3.clone()) {
            let m3020 = f782(v0, f539(v1), v2, v3)?;
            match m3020 {
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
        Ok(Err::<Adt34, Adt86>(Adt86::C12(f539(v1))))
    }
}

pub fn f773(v0: Result<Option<Str>, Adt86>, v1: bool) -> Result<Option<Str>, Adt86> {
    let m3021 = v0;
    match m3021 {
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

pub fn f774(v0: Bytes, v1: Str, v2: Str, v3: Option<Str>) -> R<Result<Option<Str>, Adt86>> {
    let m3022 = v3;
    match m3022 {
        None => {
            Ok(Ok::<Option<Str>, Adt86>(None::<Str>))
        }
        Some(v4) => {
            Ok(f783(f784(v0, v1, v2, v4)?))
        }
    }
}

pub fn f775(v0: Option<Adt5>) -> bool {
    let m3023 = v0;
    match m3023 {
        None => {
            true
        }
        Some(v1) => {
            f785(match v1 {
                Adt5::C0(h3024) => {
                    h3024
                }
            })
        }
    }
}

pub fn f776(v0: Adt13, v1: Str, v2: Adt6) -> R<bool> {
    let m3025 = v0;
    match m3025 {
        Adt13::C0 => {
            let m3026 = f122(v2);
            match m3026 {
                None => {
                    Ok(false)
                }
                Some(v3) => {
                    let a3027 = v1;
                    let a3028 = v3;
                    Ok(equal(a3027, a3028))
                }
            }
        }
        Adt13::C1 => {
            f786(v1, v2)
        }
        Adt13::C2 => {
            let m3029 = f127(v2);
            match m3029 {
                None => {
                    Ok(false)
                }
                Some(v4) => {
                    if v4 {
                        let a3030 = v1;
                        let a3031 = Str::lit("true");
                        Ok(equal(a3030, a3031))
                    } else {
                        let a3032 = v1;
                        let a3033 = Str::lit("false");
                        Ok(equal(a3032, a3033))
                    }
                }
            }
        }
    }
}

pub fn f777(p3036: List<Adt43>, p3037: List<Adt6>) -> List<Adt6> {
    let m3035 = (p3036, p3037);
    {
        let mut m3035 = m3035;
        loop {
            let step = {
                let (v0, v1) = m3035;
                let m3034 = v0;
                match m3034.uncons() {
                    None => {
                        Err::<(List<Adt43>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt43>, List<Adt6>), List<Adt6>>((v3, List::cons(Adt6::C3(f704(v2)), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m3035 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f778(v0: Adt73) -> R<Option<Adt6>> {
    let m3038 = v0;
    match m3038 {
        Adt73::C0(v1) => {
            f787(v1)
        }
        Adt73::C1(v2) => {
            let m3039 = f788(v2, List::<Adt6>::nil())?;
            match m3039 {
                None => {
                    Ok(None::<Adt6>)
                }
                Some(v3) => {
                    Ok(Some(Adt6::C4(f411(v3, List::<Adt6>::nil()))))
                }
            }
        }
    }
}

pub fn f779(v0: Adt3) -> Str {
    let m3040 = v0;
    match m3040 {
        Adt3::C0 => {
            Str::lit("light")
        }
        Adt3::C1 => {
            Str::lit("dark")
        }
    }
}

pub fn f780(v0: u64) -> bool {
    let a3041 = v0;
    let a3042 = 9u64;
    if nat_eq(a3041, a3042) {
        true
    } else {
        let a3043 = v0;
        let a3044 = 10u64;
        if nat_eq(a3043, a3044) {
            true
        } else {
            let a3045 = v0;
            let a3046 = 13u64;
            nat_eq(a3045, a3046)
        }
    }
}

pub fn f781(v0: Adt83) -> bool {
    let m3047 = v0;
    match m3047 {
        Adt83::C0(v1) => {
            let m3048 = v1;
            match m3048.uncons() {
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

pub fn f782(v0: Bytes, v1: Str, v2: Str, v3: Adt83) -> R<Result<Adt83, Adt86>> {
    let m3049 = v3.clone();
    match m3049 {
        Adt83::C0(_) => {
            Ok(Ok::<Adt83, Adt86>(v3))
        }
        Adt83::C1(v5) => {
            Ok(f789(f790(v0, v1, v2, v5)?, List::<(Str, Adt66)>::nil()))
        }
        Adt83::C2(v6, v7) => {
            Ok(f789(f790(v0, v1, v2, v7)?, v6))
        }
    }
}

pub fn f783(v0: Option<Str>) -> Result<Option<Str>, Adt86> {
    let m3050 = v0;
    match m3050 {
        None => {
            Err::<Option<Str>, Adt86>(Adt86::C10)
        }
        Some(v1) => {
            Ok::<Option<Str>, Adt86>(Some(v1))
        }
    }
}

pub fn f784(v0: Bytes, v1: Str, v2: Str, v3: Str) -> R<Option<Str>> {
    let m3054 = f791(v0, {
        let a3051 = v1;
        utf8_encode(a3051)
    }, {
        let a3052 = v2;
        utf8_encode(a3052)
    }, {
        let a3053 = v3;
        utf8_encode(a3053)
    })?;
    match m3054 {
        None => {
            Ok(None::<Str>)
        }
        Some(v4) => {
            let a3055 = v4;
            Ok(utf8_decode(a3055))
        }
    }
}

pub fn f785(p3060: List<(Str, Adt6)>) -> bool {
    let m3059 = p3060;
    {
        let mut m3059 = m3059;
        loop {
            let step = {
                let v0 = m3059;
                let m3056 = v0;
                match m3056.uncons() {
                    None => {
                        Err::<List<(Str, Adt6)>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let m3058 = f213({
                            let (_, h3057) = v1;
                            h3057
                        });
                        match m3058 {
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
                Ok(next) => { m3059 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f786(v0: Str, v1: Adt6) -> R<bool> {
    let m3061 = f363(v1);
    match m3061 {
        None => {
            Ok(false)
        }
        Some(v2) => {
            let m3062 = f330(v2)?;
            match m3062 {
                None => {
                    Ok(false)
                }
                Some(v3) => {
                    let m3064 = f792(f18({
                        let a3063 = v0;
                        utf8_encode(a3063)
                    })?)?;
                    match m3064 {
                        None => {
                            Ok(false)
                        }
                        Some(v4) => {
                            Ok(f479({
                                let a3065 = v3;
                                let a3066 = v4;
                                int_sub(a3065, a3066)?
                            }))
                        }
                    }
                }
            }
        }
    }
}

pub fn f787(v0: Adt74) -> R<Option<Adt6>> {
    let m3067 = v0;
    match m3067 {
        Adt74::C0(v1) => {
            Ok(Some(f697(v1)))
        }
        Adt74::C1(v2) => {
            Ok(Some(f698(Str::lit("image"), match v2.clone() {
                Adt44::C0(h3068, _, _, _) => {
                    h3068
                }
            }, match v2.clone() {
                Adt44::C0(_, h3069, _, _) => {
                    h3069
                }
            }, match v2.clone() {
                Adt44::C0(_, _, h3070, _) => {
                    h3070
                }
            }, match v2 {
                Adt44::C0(_, _, _, h3071) => {
                    h3071
                }
            })))
        }
        Adt74::C2(v3) => {
            Ok(Some(f698(Str::lit("audio"), match v3.clone() {
                Adt45::C0(h3072, _, _, _) => {
                    h3072
                }
            }, match v3.clone() {
                Adt45::C0(_, h3073, _, _) => {
                    h3073
                }
            }, match v3.clone() {
                Adt45::C0(_, _, h3074, _) => {
                    h3074
                }
            }, match v3 {
                Adt45::C0(_, _, _, h3075) => {
                    h3075
                }
            })))
        }
        Adt74::C3(v4) => {
            Ok(Some(f793(v4)))
        }
        Adt74::C4(v5) => {
            f794(v5)
        }
    }
}

pub fn f788(p3079: List<Adt74>, p3080: List<Adt6>) -> R<Option<List<Adt6>>> {
    let m3078 = (p3079, p3080);
    {
        let mut m3078 = m3078;
        loop {
            let step = {
                let (v0, v1) = m3078;
                let m3076 = v0;
                match m3076.uncons() {
                    None => {
                        Err::<(List<Adt74>, List<Adt6>), R<Option<List<Adt6>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m3077 = f787(v2)?;
                        match m3077 {
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
                Ok(next) => { m3078 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f789(v0: Option<Str>, v1: List<(Str, Adt66)>) -> Result<Adt83, Adt86> {
    let m3081 = v0;
    match m3081 {
        None => {
            Err::<Adt83, Adt86>(Adt86::C14)
        }
        Some(v2) => {
            let m3082 = v1.clone();
            match m3082.uncons() {
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

pub fn f790(v0: Bytes, v1: Str, v2: Str, v3: Str) -> R<Option<Str>> {
    let m3086 = f795(v0, {
        let a3083 = v1;
        utf8_encode(a3083)
    }, {
        let a3084 = v2;
        utf8_encode(a3084)
    }, {
        let a3085 = v3;
        utf8_encode(a3085)
    })?;
    match m3086 {
        None => {
            Ok(None::<Str>)
        }
        Some(v4) => {
            let a3087 = v4;
            Ok(utf8_decode(a3087))
        }
    }
}

pub fn f791(v0: Bytes, v1: Bytes, v2: Bytes, v3: Bytes) -> R<Option<Bytes>> {
    let a3089 = f796();
    let a3090 = {
        let a3088 = v0.clone();
        length_bytes(a3088)
    };
    if nat_le(a3089, a3090) {
        let a3092 = f797(v3.clone())?;
        let a3093 = {
            let a3091 = v3.clone();
            length_bytes(a3091)
        };
        if nat_eq(a3092, a3093) {
            Ok(None::<Bytes>)
        } else {
            f798(v0, v1, v2, v3.clone(), f797(v3)?)
        }
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f792(v0: List<u64>) -> R<Option<i64>> {
    let m3094 = v0.clone();
    match m3094.uncons() {
        None => {
            Ok(None::<i64>)
        }
        Some((v1, v2)) => {
            let a3095 = v1;
            let a3096 = 45u64;
            if nat_eq(a3095, a3096) {
                let m3097 = v2.clone();
                match m3097.uncons() {
                    None => {
                        Ok(None::<i64>)
                    }
                    Some((_, _)) => {
                        let m3098 = f799(v2, 0i64)?;
                        match m3098 {
                            None => {
                                Ok(None::<i64>)
                            }
                            Some(v5) => {
                                Ok(Some({
                                    let a3099 = v5;
                                    int_neg(a3099)?
                                }))
                            }
                        }
                    }
                }
            } else {
                f799(v0, 0i64)
            }
        }
    }
}

pub fn f793(v0: Adt75) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("tool_use"))), List::cons((Str::lit("id"), Adt6::C3(match v0.clone() {
        Adt75::C0(h3100, _, _, _) => {
            h3100
        }
    })), List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
        Adt75::C0(_, h3101, _, _) => {
            h3101
        }
    })), List::cons((Str::lit("input"), Adt6::C5({
        let m3104 = match v0.clone() {
            Adt75::C0(_, _, h3103, _) => {
                h3103
            }
        };
        match m3104 {
            Adt5::C0(h3102) => {
                h3102
            }
        }
    })), f696(match v0 {
        Adt75::C0(_, _, _, h3105) => {
            h3105
        }
    }))))))
}

pub fn f794(v0: Adt76) -> R<Option<Adt6>> {
    let m3107 = f573(match v0.clone() {
        Adt76::C0(_, h3106, _, _, _) => {
            h3106
        }
    })?;
    match m3107 {
        None => {
            Ok(None::<Adt6>)
        }
        Some(v1) => {
            Ok(Some(Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("tool_result"))), List::cons((Str::lit("toolUseId"), Adt6::C3(match v0.clone() {
                Adt76::C0(h3108, _, _, _, _) => {
                    h3108
                }
            })), List::cons((Str::lit("content"), Adt6::C4(v1)), {
                let a3114 = f574(Str::lit("structuredContent"), match v0.clone() {
                    Adt76::C0(_, _, h3109, _, _) => {
                        h3109
                    }
                });
                let a3115 = {
                    let a3112 = f575(Str::lit("isError"), match v0.clone() {
                        Adt76::C0(_, _, _, h3110, _) => {
                            h3110
                        }
                    });
                    let a3113 = f696(match v0 {
                        Adt76::C0(_, _, _, _, h3111) => {
                            h3111
                        }
                    });
                    append_list(a3112, a3113)
                };
                append_list(a3114, a3115)
            }))))))
        }
    }
}

pub fn f795(v0: Bytes, v1: Bytes, v2: Bytes, v3: Bytes) -> R<Option<Bytes>> {
    let a3117 = f796();
    let a3118 = {
        let a3116 = v0.clone();
        length_bytes(a3116)
    };
    if nat_le(a3117, a3118) {
        Ok(Some({
            let a3121 = f800(v3.clone())?;
            let a3122 = {
                let a3119 = Bytes::lit(&[0x2e]);
                let a3120 = f800(f801(v0, v1, v2, v3)?)?;
                append_bytes(a3119, a3120)
            };
            append_bytes(a3121, a3122)
        }))
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f796() -> u64 {
    32u64
}

pub fn f797(v0: Bytes) -> R<u64> {
    let (_, h3123) = f802(Fn4::F803(v0.clone()), {
        let a3124 = v0.clone();
        length_bytes(a3124)
    }, (0u64, {
        let a3125 = v0;
        length_bytes(a3125)
    }))?;
    Ok(h3123)
}

pub fn f798(v0: Bytes, v1: Bytes, v2: Bytes, v3: Bytes, v4: u64) -> R<Option<Bytes>> {
    let m3129 = {
        let a3126 = v3.clone();
        let a3127 = 0u64;
        let a3128 = v4;
        slice_bytes(a3126, a3127, a3128)
    };
    match m3129 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v5) => {
            let m3140 = {
                let a3137 = v3.clone();
                let a3138 = {
                    let a3130 = v4;
                    let a3131 = 1u64;
                    nat_add(a3130, a3131)?
                };
                let a3139 = {
                    let a3135 = {
                        let a3132 = v3;
                        length_bytes(a3132)
                    };
                    let a3136 = {
                        let a3133 = v4;
                        let a3134 = 1u64;
                        nat_add(a3133, a3134)?
                    };
                    nat_sub(a3135, a3136)
                };
                slice_bytes(a3137, a3138, a3139)
            };
            match m3140 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v6) => {
                    f804(v0, v1, v2, f805(v5)?, f805(v6)?)
                }
            }
        }
    }
}

pub fn f799(p3146: List<u64>, p3147: i64) -> R<Option<i64>> {
    let m3145 = (p3146, p3147);
    {
        let mut m3145 = m3145;
        loop {
            let step = {
                let (v0, v1) = m3145;
                let m3141 = v0;
                match m3141.uncons() {
                    None => {
                        Err::<(List<u64>, i64), R<Option<i64>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m3142 = f806(v2);
                        match m3142 {
                            None => {
                                Err::<(List<u64>, i64), R<Option<i64>>>(Ok(None::<i64>))
                            }
                            Some(v4) => {
                                Ok::<(List<u64>, i64), R<Option<i64>>>((v3, f182({
                                    let a3143 = v1;
                                    let a3144 = 10i64;
                                    int_mul(a3143, a3144)?
                                }, f410(v4))?))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3145 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f800(v0: Bytes) -> R<Bytes> {
    let (_, h3148) = f807(Fn5::F808(v0.clone()), {
        let a3149 = v0;
        length_bytes(a3149)
    }, (0u64, f88()))?;
    Ok(h3148)
}

pub fn f801(v0: Bytes, v1: Bytes, v2: Bytes, v3: Bytes) -> R<Bytes> {
    f809(v0, {
        let a3150 = f810(v1, v2);
        let a3151 = v3;
        append_bytes(a3150, a3151)
    })
}

pub fn f802(v0: Fn4, p3155: u64, p3156: (u64, u64)) -> R<(u64, u64)> {
    let m3154 = (p3155, p3156);
    {
        let mut m3154 = m3154;
        loop {
            let step = {
                let (v1, v2) = m3154;
                let m3152 = v1;
                if m3152 == 0 {
                    Err::<(u64, (u64, u64)), R<(u64, u64)>>(Ok(v2))
                } else {
                    let v3 = m3152 - 1;
                    Ok::<(u64, (u64, u64)), R<(u64, u64)>>((v3, {
                        let c3153: Fn4 = v0.clone();
                        c3153.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m3154 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f803(v0: Bytes, v1: (u64, u64)) -> R<(u64, u64)> {
    Ok(({
        let a3158 = {
            let (h3157, _) = v1;
            h3157
        };
        let a3159 = 1u64;
        nat_add(a3158, a3159)?
    }, {
        let a3162 = {
            let (_, h3160) = v1;
            h3160
        };
        let a3163 = {
            let a3161 = v0.clone();
            length_bytes(a3161)
        };
        if nat_eq(a3162, a3163) {
            let a3165 = f57(v0, {
                let (h3164, _) = v1;
                h3164
            })?;
            let a3166 = 46u64;
            if nat_eq(a3165, a3166) {
                let (h3167, _) = v1;
                h3167
            } else {
                let (_, h3168) = v1;
                h3168
            }
        } else {
            let (_, h3169) = v1;
            h3169
        }
    }))
}

pub fn f804(v0: Bytes, v1: Bytes, v2: Bytes, v3: Option<Bytes>, v4: Option<Bytes>) -> R<Option<Bytes>> {
    let m3170 = v3;
    match m3170 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v5) => {
            let m3171 = v4;
            match m3171 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v6) => {
                    if f811(f801(v0, v1, v2, v5.clone())?, v6)? {
                        Ok(Some(v5))
                    } else {
                        Ok(None::<Bytes>)
                    }
                }
            }
        }
    }
}

pub fn f805(v0: Bytes) -> R<Option<Bytes>> {
    let a3176 = {
        let a3173 = {
            let a3172 = v0.clone();
            length_bytes(a3172)
        };
        let a3174 = 2u64;
        let a3175 = 0u64;
        nat_rem(a3173, a3174, a3175)
    };
    let a3177 = 0u64;
    if nat_eq(a3176, a3177) {
        let v1: (u64, (Bytes, u64)) = f812(Fn6::F813(v0.clone()), {
            let a3179 = {
                let a3178 = v0;
                length_bytes(a3178)
            };
            let a3180 = 2u64;
            let a3181 = 0u64;
            nat_quot(a3179, a3180, a3181)
        }, (0u64, (f88(), 0u64)))?;
        let a3184 = {
            let (_, h3182) = {
                let (_, h3183) = v1.clone();
                h3183
            };
            h3182
        };
        let a3185 = 0u64;
        if nat_eq(a3184, a3185) {
            Ok(Some({
                let (h3186, _) = {
                    let (_, h3187) = v1;
                    h3187
                };
                h3186
            }))
        } else {
            Ok(None::<Bytes>)
        }
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f806(v0: u64) -> Option<u64> {
    let a3188 = v0;
    let a3189 = 48u64;
    if if nat_lt(a3188, a3189) {
        true
    } else {
        let a3190 = 57u64;
        let a3191 = v0;
        nat_lt(a3190, a3191)
    } {
        None::<u64>
    } else {
        Some({
            let a3192 = v0;
            let a3193 = 48u64;
            nat_sub(a3192, a3193)
        })
    }
}

pub fn f807(v0: Fn5, p3197: u64, p3198: (u64, Bytes)) -> R<(u64, Bytes)> {
    let m3196 = (p3197, p3198);
    {
        let mut m3196 = m3196;
        loop {
            let step = {
                let (v1, v2) = m3196;
                let m3194 = v1;
                if m3194 == 0 {
                    Err::<(u64, (u64, Bytes)), R<(u64, Bytes)>>(Ok(v2))
                } else {
                    let v3 = m3194 - 1;
                    Ok::<(u64, (u64, Bytes)), R<(u64, Bytes)>>((v3, {
                        let c3195: Fn5 = v0.clone();
                        c3195.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m3196 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f808(v0: Bytes, v1: (u64, Bytes)) -> R<(u64, Bytes)> {
    Ok(({
        let a3200 = {
            let (h3199, _) = v1.clone();
            h3199
        };
        let a3201 = 1u64;
        nat_add(a3200, a3201)?
    }, {
        let a3204 = {
            let (_, h3202) = v1;
            h3202
        };
        let a3205 = f814(f57(v0, {
            let (h3203, _) = v1;
            h3203
        })?)?;
        append_bytes(a3204, a3205)
    }))
}

pub fn f809(v0: Bytes, v1: Bytes) -> R<Bytes> {
    f815(f816(v0, v1)?)
}

pub fn f810(v0: Bytes, v1: Bytes) -> Bytes {
    let a3212 = f817({
        let a3206 = v0.clone();
        length_bytes(a3206)
    });
    let a3213 = {
        let a3210 = v0;
        let a3211 = {
            let a3208 = f817({
                let a3207 = v1.clone();
                length_bytes(a3207)
            });
            let a3209 = v1;
            append_bytes(a3208, a3209)
        };
        append_bytes(a3210, a3211)
    };
    append_bytes(a3212, a3213)
}

pub fn f811(v0: Bytes, v1: Bytes) -> R<bool> {
    let a3216 = {
        let a3214 = v0.clone();
        length_bytes(a3214)
    };
    let a3217 = {
        let a3215 = v1.clone();
        length_bytes(a3215)
    };
    if nat_eq(a3216, a3217) {
        let a3218 = f818(v0, v1)?;
        let a3219 = 0u64;
        Ok(nat_eq(a3218, a3219))
    } else {
        Ok(false)
    }
}

pub fn f812(v0: Fn6, p3223: u64, p3224: (u64, (Bytes, u64))) -> R<(u64, (Bytes, u64))> {
    let m3222 = (p3223, p3224);
    {
        let mut m3222 = m3222;
        loop {
            let step = {
                let (v1, v2) = m3222;
                let m3220 = v1;
                if m3220 == 0 {
                    Err::<(u64, (u64, (Bytes, u64))), R<(u64, (Bytes, u64))>>(Ok(v2))
                } else {
                    let v3 = m3220 - 1;
                    Ok::<(u64, (u64, (Bytes, u64))), R<(u64, (Bytes, u64))>>((v3, {
                        let c3221: Fn6 = v0.clone();
                        c3221.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m3222 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f813(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<(u64, (Bytes, u64))> {
    f819(v0, v1)
}

pub fn f814(v0: u64) -> R<Bytes> {
    let a3231 = f820({
        let a3225 = v0;
        let a3226 = 16u64;
        let a3227 = 0u64;
        nat_quot(a3225, a3226, a3227)
    })?;
    let a3232 = f820({
        let a3228 = v0;
        let a3229 = 16u64;
        let a3230 = 0u64;
        nat_rem(a3228, a3229, a3230)
    })?;
    Ok(append_bytes(a3231, a3232))
}

pub fn f815(v0: Adt159) -> R<Bytes> {
    let a3253 = f821(match v0.clone() {
        Adt159::C0(h3233, _, _, _, _, _, _, _) => {
            h3233
        }
    })?;
    let a3254 = {
        let a3251 = f821(match v0.clone() {
            Adt159::C0(_, h3234, _, _, _, _, _, _) => {
                h3234
            }
        })?;
        let a3252 = {
            let a3249 = f821(match v0.clone() {
                Adt159::C0(_, _, h3235, _, _, _, _, _) => {
                    h3235
                }
            })?;
            let a3250 = {
                let a3247 = f821(match v0.clone() {
                    Adt159::C0(_, _, _, h3236, _, _, _, _) => {
                        h3236
                    }
                })?;
                let a3248 = {
                    let a3245 = f821(match v0.clone() {
                        Adt159::C0(_, _, _, _, h3237, _, _, _) => {
                            h3237
                        }
                    })?;
                    let a3246 = {
                        let a3243 = f821(match v0.clone() {
                            Adt159::C0(_, _, _, _, _, h3238, _, _) => {
                                h3238
                            }
                        })?;
                        let a3244 = {
                            let a3241 = f821(match v0.clone() {
                                Adt159::C0(_, _, _, _, _, _, h3239, _) => {
                                    h3239
                                }
                            })?;
                            let a3242 = f821(match v0 {
                                Adt159::C0(_, _, _, _, _, _, _, h3240) => {
                                    h3240
                                }
                            })?;
                            append_bytes(a3241, a3242)
                        };
                        append_bytes(a3243, a3244)
                    };
                    append_bytes(a3245, a3246)
                };
                append_bytes(a3247, a3248)
            };
            append_bytes(a3249, a3250)
        };
        append_bytes(a3251, a3252)
    };
    Ok(append_bytes(a3253, a3254))
}

pub fn f816(v0: Bytes, v1: Bytes) -> R<Adt159> {
    let v2: Adt160 = f822(v0)?;
    let v3: Adt159 = f823(f824(f825(), f826(v2.clone(), 909522486u32))?, 64u64, v1)?;
    f824(f824(f825(), f826(v2, 1549556828u32))?, f827(v3))
}

pub fn f817(v0: u64) -> Bytes {
    let a3280 = f179({
        let a3258 = {
            let a3255 = v0;
            let a3256 = 16777216u64;
            let a3257 = 0u64;
            nat_quot(a3255, a3256, a3257)
        };
        let a3259 = 256u64;
        let a3260 = 0u64;
        nat_rem(a3258, a3259, a3260)
    });
    let a3281 = {
        let a3278 = f179({
            let a3264 = {
                let a3261 = v0;
                let a3262 = 65536u64;
                let a3263 = 0u64;
                nat_quot(a3261, a3262, a3263)
            };
            let a3265 = 256u64;
            let a3266 = 0u64;
            nat_rem(a3264, a3265, a3266)
        });
        let a3279 = {
            let a3276 = f179({
                let a3270 = {
                    let a3267 = v0;
                    let a3268 = 256u64;
                    let a3269 = 0u64;
                    nat_quot(a3267, a3268, a3269)
                };
                let a3271 = 256u64;
                let a3272 = 0u64;
                nat_rem(a3270, a3271, a3272)
            });
            let a3277 = f179({
                let a3273 = v0;
                let a3274 = 256u64;
                let a3275 = 0u64;
                nat_rem(a3273, a3274, a3275)
            });
            append_bytes(a3276, a3277)
        };
        append_bytes(a3278, a3279)
    };
    append_bytes(a3280, a3281)
}

pub fn f818(v0: Bytes, v1: Bytes) -> R<u64> {
    let (_, h3282) = f802(Fn4::F828(v0.clone(), v1), {
        let a3283 = v0;
        length_bytes(a3283)
    }, (0u64, 0u64))?;
    Ok(h3282)
}

pub fn f819(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<(u64, (Bytes, u64))> {
    let v2: u64 = f829(f57(v0.clone(), {
        let (h3284, _) = v1.clone();
        h3284
    })?);
    let v3: u64 = f829(f57(v0, {
        let a3286 = {
            let (h3285, _) = v1.clone();
            h3285
        };
        let a3287 = 1u64;
        nat_add(a3286, a3287)?
    })?);
    let a3288 = v2;
    let a3289 = 16u64;
    if if nat_lt(a3288, a3289) {
        let a3290 = v3;
        let a3291 = 16u64;
        nat_lt(a3290, a3291)
    } else {
        false
    } {
        Ok(({
            let a3293 = {
                let (h3292, _) = v1.clone();
                h3292
            };
            let a3294 = 2u64;
            nat_add(a3293, a3294)?
        }, ({
            let a3301 = {
                let (h3295, _) = {
                    let (_, h3296) = v1.clone();
                    h3296
                };
                h3295
            };
            let a3302 = f179({
                let a3299 = {
                    let a3297 = v2;
                    let a3298 = 16u64;
                    nat_mul(a3297, a3298)?
                };
                let a3300 = v3;
                nat_add(a3299, a3300)?
            });
            append_bytes(a3301, a3302)
        }, {
            let (_, h3303) = {
                let (_, h3304) = v1;
                h3304
            };
            h3303
        })))
    } else {
        Ok(({
            let a3306 = {
                let (h3305, _) = v1.clone();
                h3305
            };
            let a3307 = 2u64;
            nat_add(a3306, a3307)?
        }, ({
            let (h3308, _) = {
                let (_, h3309) = v1.clone();
                h3309
            };
            h3308
        }, {
            let a3312 = {
                let (_, h3310) = {
                    let (_, h3311) = v1;
                    h3311
                };
                h3310
            };
            let a3313 = 1u64;
            nat_add(a3312, a3313)?
        })))
    }
}

pub fn f820(v0: u64) -> R<Bytes> {
    Ok(f179({
        let a3314 = v0;
        let a3315 = 10u64;
        if nat_lt(a3314, a3315) {
            let a3316 = 48u64;
            let a3317 = v0;
            nat_add(a3316, a3317)?
        } else {
            let a3318 = 87u64;
            let a3319 = v0;
            nat_add(a3318, a3319)?
        }
    }))
}

pub fn f821(v0: u32) -> R<Bytes> {
    let a3324 = f830(v0, 24u32)?;
    let a3325 = {
        let a3322 = f830(v0, 16u32)?;
        let a3323 = {
            let a3320 = f830(v0, 8u32)?;
            let a3321 = f830(v0, 0u32)?;
            append_bytes(a3320, a3321)
        };
        append_bytes(a3322, a3323)
    };
    Ok(append_bytes(a3324, a3325))
}

pub fn f822(v0: Bytes) -> R<Adt160> {
    let a3327 = f831();
    let a3328 = {
        let a3326 = v0.clone();
        length_bytes(a3326)
    };
    if nat_lt(a3327, a3328) {
        Ok(f832(f833(v0)?))
    } else {
        f834(v0)
    }
}

pub fn f823(v0: Adt159, v1: u64, v2: Bytes) -> R<Adt159> {
    let v3: u64 = {
        let a3329 = v2.clone();
        length_bytes(a3329)
    };
    let v4: u64 = {
        let a3335 = {
            let a3332 = {
                let a3330 = v3;
                let a3331 = 72u64;
                nat_add(a3330, a3331)?
            };
            let a3333 = 64u64;
            let a3334 = 0u64;
            nat_quot(a3332, a3333, a3334)
        };
        let a3336 = 64u64;
        nat_mul(a3335, a3336)?
    };
    let (_, h3337) = f835(Fn7::F836(v2, v1, v3, v4), {
        let a3338 = v4;
        let a3339 = 64u64;
        let a3340 = 0u64;
        nat_quot(a3338, a3339, a3340)
    }, (0u64, v0))?;
    Ok(h3337)
}

pub fn f824(v0: Adt159, v1: Adt160) -> R<Adt159> {
    let v2: Adt161 = f837(Fn8::F838, 64u64, Adt161::C0(match v0.clone() {
        Adt159::C0(h3341, _, _, _, _, _, _, _) => {
            h3341
        }
    }, match v0.clone() {
        Adt159::C0(_, h3342, _, _, _, _, _, _) => {
            h3342
        }
    }, match v0.clone() {
        Adt159::C0(_, _, h3343, _, _, _, _, _) => {
            h3343
        }
    }, match v0.clone() {
        Adt159::C0(_, _, _, h3344, _, _, _, _) => {
            h3344
        }
    }, match v0.clone() {
        Adt159::C0(_, _, _, _, h3345, _, _, _) => {
            h3345
        }
    }, match v0.clone() {
        Adt159::C0(_, _, _, _, _, h3346, _, _) => {
            h3346
        }
    }, match v0.clone() {
        Adt159::C0(_, _, _, _, _, _, h3347, _) => {
            h3347
        }
    }, match v0.clone() {
        Adt159::C0(_, _, _, _, _, _, _, h3348) => {
            h3348
        }
    }, v1, 0u64))?;
    Ok(Adt159::C0(f839(match v0.clone() {
        Adt159::C0(h3349, _, _, _, _, _, _, _) => {
            h3349
        }
    }, match v2.clone() {
        Adt161::C0(h3350, _, _, _, _, _, _, _, _, _) => {
            h3350
        }
    }), f839(match v0.clone() {
        Adt159::C0(_, h3351, _, _, _, _, _, _) => {
            h3351
        }
    }, match v2.clone() {
        Adt161::C0(_, h3352, _, _, _, _, _, _, _, _) => {
            h3352
        }
    }), f839(match v0.clone() {
        Adt159::C0(_, _, h3353, _, _, _, _, _) => {
            h3353
        }
    }, match v2.clone() {
        Adt161::C0(_, _, h3354, _, _, _, _, _, _, _) => {
            h3354
        }
    }), f839(match v0.clone() {
        Adt159::C0(_, _, _, h3355, _, _, _, _) => {
            h3355
        }
    }, match v2.clone() {
        Adt161::C0(_, _, _, h3356, _, _, _, _, _, _) => {
            h3356
        }
    }), f839(match v0.clone() {
        Adt159::C0(_, _, _, _, h3357, _, _, _) => {
            h3357
        }
    }, match v2.clone() {
        Adt161::C0(_, _, _, _, h3358, _, _, _, _, _) => {
            h3358
        }
    }), f839(match v0.clone() {
        Adt159::C0(_, _, _, _, _, h3359, _, _) => {
            h3359
        }
    }, match v2.clone() {
        Adt161::C0(_, _, _, _, _, h3360, _, _, _, _) => {
            h3360
        }
    }), f839(match v0.clone() {
        Adt159::C0(_, _, _, _, _, _, h3361, _) => {
            h3361
        }
    }, match v2.clone() {
        Adt161::C0(_, _, _, _, _, _, h3362, _, _, _) => {
            h3362
        }
    }), f839(match v0 {
        Adt159::C0(_, _, _, _, _, _, _, h3363) => {
            h3363
        }
    }, match v2 {
        Adt161::C0(_, _, _, _, _, _, _, h3364, _, _) => {
            h3364
        }
    })))
}

pub fn f825() -> Adt159 {
    Adt159::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f826(v0: Adt160, v1: u32) -> Adt160 {
    Adt160::C0({
        let a3366 = match v0.clone() {
            Adt160::C0(h3365, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h3365
            }
        };
        let a3367 = v1;
        fixed_u32::bit_xor(a3366, a3367)
    }, {
        let a3369 = match v0.clone() {
            Adt160::C0(_, h3368, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h3368
            }
        };
        let a3370 = v1;
        fixed_u32::bit_xor(a3369, a3370)
    }, {
        let a3372 = match v0.clone() {
            Adt160::C0(_, _, h3371, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h3371
            }
        };
        let a3373 = v1;
        fixed_u32::bit_xor(a3372, a3373)
    }, {
        let a3375 = match v0.clone() {
            Adt160::C0(_, _, _, h3374, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h3374
            }
        };
        let a3376 = v1;
        fixed_u32::bit_xor(a3375, a3376)
    }, {
        let a3378 = match v0.clone() {
            Adt160::C0(_, _, _, _, h3377, _, _, _, _, _, _, _, _, _, _, _) => {
                h3377
            }
        };
        let a3379 = v1;
        fixed_u32::bit_xor(a3378, a3379)
    }, {
        let a3381 = match v0.clone() {
            Adt160::C0(_, _, _, _, _, h3380, _, _, _, _, _, _, _, _, _, _) => {
                h3380
            }
        };
        let a3382 = v1;
        fixed_u32::bit_xor(a3381, a3382)
    }, {
        let a3384 = match v0.clone() {
            Adt160::C0(_, _, _, _, _, _, h3383, _, _, _, _, _, _, _, _, _) => {
                h3383
            }
        };
        let a3385 = v1;
        fixed_u32::bit_xor(a3384, a3385)
    }, {
        let a3387 = match v0.clone() {
            Adt160::C0(_, _, _, _, _, _, _, h3386, _, _, _, _, _, _, _, _) => {
                h3386
            }
        };
        let a3388 = v1;
        fixed_u32::bit_xor(a3387, a3388)
    }, {
        let a3390 = match v0.clone() {
            Adt160::C0(_, _, _, _, _, _, _, _, h3389, _, _, _, _, _, _, _) => {
                h3389
            }
        };
        let a3391 = v1;
        fixed_u32::bit_xor(a3390, a3391)
    }, {
        let a3393 = match v0.clone() {
            Adt160::C0(_, _, _, _, _, _, _, _, _, h3392, _, _, _, _, _, _) => {
                h3392
            }
        };
        let a3394 = v1;
        fixed_u32::bit_xor(a3393, a3394)
    }, {
        let a3396 = match v0.clone() {
            Adt160::C0(_, _, _, _, _, _, _, _, _, _, h3395, _, _, _, _, _) => {
                h3395
            }
        };
        let a3397 = v1;
        fixed_u32::bit_xor(a3396, a3397)
    }, {
        let a3399 = match v0.clone() {
            Adt160::C0(_, _, _, _, _, _, _, _, _, _, _, h3398, _, _, _, _) => {
                h3398
            }
        };
        let a3400 = v1;
        fixed_u32::bit_xor(a3399, a3400)
    }, {
        let a3402 = match v0.clone() {
            Adt160::C0(_, _, _, _, _, _, _, _, _, _, _, _, h3401, _, _, _) => {
                h3401
            }
        };
        let a3403 = v1;
        fixed_u32::bit_xor(a3402, a3403)
    }, {
        let a3405 = match v0.clone() {
            Adt160::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h3404, _, _) => {
                h3404
            }
        };
        let a3406 = v1;
        fixed_u32::bit_xor(a3405, a3406)
    }, {
        let a3408 = match v0.clone() {
            Adt160::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h3407, _) => {
                h3407
            }
        };
        let a3409 = v1;
        fixed_u32::bit_xor(a3408, a3409)
    }, {
        let a3411 = match v0 {
            Adt160::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h3410) => {
                h3410
            }
        };
        let a3412 = v1;
        fixed_u32::bit_xor(a3411, a3412)
    })
}

pub fn f827(v0: Adt159) -> Adt160 {
    Adt160::C0(match v0.clone() {
        Adt159::C0(h3413, _, _, _, _, _, _, _) => {
            h3413
        }
    }, match v0.clone() {
        Adt159::C0(_, h3414, _, _, _, _, _, _) => {
            h3414
        }
    }, match v0.clone() {
        Adt159::C0(_, _, h3415, _, _, _, _, _) => {
            h3415
        }
    }, match v0.clone() {
        Adt159::C0(_, _, _, h3416, _, _, _, _) => {
            h3416
        }
    }, match v0.clone() {
        Adt159::C0(_, _, _, _, h3417, _, _, _) => {
            h3417
        }
    }, match v0.clone() {
        Adt159::C0(_, _, _, _, _, h3418, _, _) => {
            h3418
        }
    }, match v0.clone() {
        Adt159::C0(_, _, _, _, _, _, h3419, _) => {
            h3419
        }
    }, match v0 {
        Adt159::C0(_, _, _, _, _, _, _, h3420) => {
            h3420
        }
    }, 2147483648u32, f840(), f840(), f840(), f840(), f840(), f840(), 768u32)
}

pub fn f828(v0: Bytes, v1: Bytes, v2: (u64, u64)) -> R<(u64, u64)> {
    Ok(({
        let a3422 = {
            let (h3421, _) = v2;
            h3421
        };
        let a3423 = 1u64;
        nat_add(a3422, a3423)?
    }, {
        let a3475 = {
            let (_, h3424) = v2;
            h3424
        };
        let a3476 = {
            let v3: u8 = {
                let a3427 = f841(v0, {
                    let (h3425, _) = v2;
                    h3425
                });
                let a3428 = f841(v1, {
                    let (h3426, _) = v2;
                    h3426
                });
                fixed_u8::bit_xor(a3427, a3428)
            };
            let a3473 = {
                let a3449 = {
                    let a3437 = {
                        let a3431 = {
                            let a3429 = v3;
                            let a3430 = 128u8;
                            fixed_u8::bit_and(a3429, a3430)
                        };
                        let a3432 = 128u8;
                        if equal(a3431, a3432) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a3438 = {
                        let a3435 = {
                            let a3433 = v3;
                            let a3434 = 64u8;
                            fixed_u8::bit_and(a3433, a3434)
                        };
                        let a3436 = 64u8;
                        if equal(a3435, a3436) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a3437, a3438)?
                };
                let a3450 = {
                    let a3447 = {
                        let a3441 = {
                            let a3439 = v3;
                            let a3440 = 32u8;
                            fixed_u8::bit_and(a3439, a3440)
                        };
                        let a3442 = 32u8;
                        if equal(a3441, a3442) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a3448 = {
                        let a3445 = {
                            let a3443 = v3;
                            let a3444 = 16u8;
                            fixed_u8::bit_and(a3443, a3444)
                        };
                        let a3446 = 16u8;
                        if equal(a3445, a3446) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a3447, a3448)?
                };
                nat_add(a3449, a3450)?
            };
            let a3474 = {
                let a3471 = {
                    let a3459 = {
                        let a3453 = {
                            let a3451 = v3;
                            let a3452 = 8u8;
                            fixed_u8::bit_and(a3451, a3452)
                        };
                        let a3454 = 8u8;
                        if equal(a3453, a3454) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a3460 = {
                        let a3457 = {
                            let a3455 = v3;
                            let a3456 = 4u8;
                            fixed_u8::bit_and(a3455, a3456)
                        };
                        let a3458 = 4u8;
                        if equal(a3457, a3458) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a3459, a3460)?
                };
                let a3472 = {
                    let a3469 = {
                        let a3463 = {
                            let a3461 = v3;
                            let a3462 = 2u8;
                            fixed_u8::bit_and(a3461, a3462)
                        };
                        let a3464 = 2u8;
                        if equal(a3463, a3464) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a3470 = {
                        let a3467 = {
                            let a3465 = v3;
                            let a3466 = 1u8;
                            fixed_u8::bit_and(a3465, a3466)
                        };
                        let a3468 = 1u8;
                        if equal(a3467, a3468) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a3469, a3470)?
                };
                nat_add(a3471, a3472)?
            };
            nat_add(a3473, a3474)?
        };
        nat_add(a3475, a3476)?
    }))
}

pub fn f829(v0: u64) -> u64 {
    let a3477 = v0;
    let a3478 = 48u64;
    if nat_lt(a3477, a3478) {
        16u64
    } else {
        let a3479 = v0;
        let a3480 = 58u64;
        if nat_lt(a3479, a3480) {
            let a3481 = v0;
            let a3482 = 48u64;
            nat_sub(a3481, a3482)
        } else {
            let a3483 = v0;
            let a3484 = 97u64;
            if nat_lt(a3483, a3484) {
                16u64
            } else {
                let a3485 = v0;
                let a3486 = 103u64;
                if nat_lt(a3485, a3486) {
                    let a3487 = v0;
                    let a3488 = 87u64;
                    nat_sub(a3487, a3488)
                } else {
                    16u64
                }
            }
        }
    }
}

pub fn f830(v0: u32, v1: u32) -> R<Bytes> {
    Ok(f179({
        let v2: u32 = {
            let a3489 = f842(v0, v1);
            let a3490 = 255u32;
            fixed_u32::bit_and(a3489, a3490)
        };
        let a3679 = {
            let a3583 = {
                let a3535 = {
                    let a3511 = {
                        let a3499 = {
                            let a3493 = {
                                let a3491 = v2;
                                let a3492 = 2147483648u32;
                                fixed_u32::bit_and(a3491, a3492)
                            };
                            let a3494 = 2147483648u32;
                            if equal(a3493, a3494) {
                                2147483648u64
                            } else {
                                0u64
                            }
                        };
                        let a3500 = {
                            let a3497 = {
                                let a3495 = v2;
                                let a3496 = 1073741824u32;
                                fixed_u32::bit_and(a3495, a3496)
                            };
                            let a3498 = 1073741824u32;
                            if equal(a3497, a3498) {
                                1073741824u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3499, a3500)?
                    };
                    let a3512 = {
                        let a3509 = {
                            let a3503 = {
                                let a3501 = v2;
                                let a3502 = 536870912u32;
                                fixed_u32::bit_and(a3501, a3502)
                            };
                            let a3504 = 536870912u32;
                            if equal(a3503, a3504) {
                                536870912u64
                            } else {
                                0u64
                            }
                        };
                        let a3510 = {
                            let a3507 = {
                                let a3505 = v2;
                                let a3506 = 268435456u32;
                                fixed_u32::bit_and(a3505, a3506)
                            };
                            let a3508 = 268435456u32;
                            if equal(a3507, a3508) {
                                268435456u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3509, a3510)?
                    };
                    nat_add(a3511, a3512)?
                };
                let a3536 = {
                    let a3533 = {
                        let a3521 = {
                            let a3515 = {
                                let a3513 = v2;
                                let a3514 = 134217728u32;
                                fixed_u32::bit_and(a3513, a3514)
                            };
                            let a3516 = 134217728u32;
                            if equal(a3515, a3516) {
                                134217728u64
                            } else {
                                0u64
                            }
                        };
                        let a3522 = {
                            let a3519 = {
                                let a3517 = v2;
                                let a3518 = 67108864u32;
                                fixed_u32::bit_and(a3517, a3518)
                            };
                            let a3520 = 67108864u32;
                            if equal(a3519, a3520) {
                                67108864u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3521, a3522)?
                    };
                    let a3534 = {
                        let a3531 = {
                            let a3525 = {
                                let a3523 = v2;
                                let a3524 = 33554432u32;
                                fixed_u32::bit_and(a3523, a3524)
                            };
                            let a3526 = 33554432u32;
                            if equal(a3525, a3526) {
                                33554432u64
                            } else {
                                0u64
                            }
                        };
                        let a3532 = {
                            let a3529 = {
                                let a3527 = v2;
                                let a3528 = 16777216u32;
                                fixed_u32::bit_and(a3527, a3528)
                            };
                            let a3530 = 16777216u32;
                            if equal(a3529, a3530) {
                                16777216u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3531, a3532)?
                    };
                    nat_add(a3533, a3534)?
                };
                nat_add(a3535, a3536)?
            };
            let a3584 = {
                let a3581 = {
                    let a3557 = {
                        let a3545 = {
                            let a3539 = {
                                let a3537 = v2;
                                let a3538 = 8388608u32;
                                fixed_u32::bit_and(a3537, a3538)
                            };
                            let a3540 = 8388608u32;
                            if equal(a3539, a3540) {
                                8388608u64
                            } else {
                                0u64
                            }
                        };
                        let a3546 = {
                            let a3543 = {
                                let a3541 = v2;
                                let a3542 = 4194304u32;
                                fixed_u32::bit_and(a3541, a3542)
                            };
                            let a3544 = 4194304u32;
                            if equal(a3543, a3544) {
                                4194304u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3545, a3546)?
                    };
                    let a3558 = {
                        let a3555 = {
                            let a3549 = {
                                let a3547 = v2;
                                let a3548 = 2097152u32;
                                fixed_u32::bit_and(a3547, a3548)
                            };
                            let a3550 = 2097152u32;
                            if equal(a3549, a3550) {
                                2097152u64
                            } else {
                                0u64
                            }
                        };
                        let a3556 = {
                            let a3553 = {
                                let a3551 = v2;
                                let a3552 = 1048576u32;
                                fixed_u32::bit_and(a3551, a3552)
                            };
                            let a3554 = 1048576u32;
                            if equal(a3553, a3554) {
                                1048576u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3555, a3556)?
                    };
                    nat_add(a3557, a3558)?
                };
                let a3582 = {
                    let a3579 = {
                        let a3567 = {
                            let a3561 = {
                                let a3559 = v2;
                                let a3560 = 524288u32;
                                fixed_u32::bit_and(a3559, a3560)
                            };
                            let a3562 = 524288u32;
                            if equal(a3561, a3562) {
                                524288u64
                            } else {
                                0u64
                            }
                        };
                        let a3568 = {
                            let a3565 = {
                                let a3563 = v2;
                                let a3564 = 262144u32;
                                fixed_u32::bit_and(a3563, a3564)
                            };
                            let a3566 = 262144u32;
                            if equal(a3565, a3566) {
                                262144u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3567, a3568)?
                    };
                    let a3580 = {
                        let a3577 = {
                            let a3571 = {
                                let a3569 = v2;
                                let a3570 = 131072u32;
                                fixed_u32::bit_and(a3569, a3570)
                            };
                            let a3572 = 131072u32;
                            if equal(a3571, a3572) {
                                131072u64
                            } else {
                                0u64
                            }
                        };
                        let a3578 = {
                            let a3575 = {
                                let a3573 = v2;
                                let a3574 = 65536u32;
                                fixed_u32::bit_and(a3573, a3574)
                            };
                            let a3576 = 65536u32;
                            if equal(a3575, a3576) {
                                65536u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3577, a3578)?
                    };
                    nat_add(a3579, a3580)?
                };
                nat_add(a3581, a3582)?
            };
            nat_add(a3583, a3584)?
        };
        let a3680 = {
            let a3677 = {
                let a3629 = {
                    let a3605 = {
                        let a3593 = {
                            let a3587 = {
                                let a3585 = v2;
                                let a3586 = 32768u32;
                                fixed_u32::bit_and(a3585, a3586)
                            };
                            let a3588 = 32768u32;
                            if equal(a3587, a3588) {
                                32768u64
                            } else {
                                0u64
                            }
                        };
                        let a3594 = {
                            let a3591 = {
                                let a3589 = v2;
                                let a3590 = 16384u32;
                                fixed_u32::bit_and(a3589, a3590)
                            };
                            let a3592 = 16384u32;
                            if equal(a3591, a3592) {
                                16384u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3593, a3594)?
                    };
                    let a3606 = {
                        let a3603 = {
                            let a3597 = {
                                let a3595 = v2;
                                let a3596 = 8192u32;
                                fixed_u32::bit_and(a3595, a3596)
                            };
                            let a3598 = 8192u32;
                            if equal(a3597, a3598) {
                                8192u64
                            } else {
                                0u64
                            }
                        };
                        let a3604 = {
                            let a3601 = {
                                let a3599 = v2;
                                let a3600 = 4096u32;
                                fixed_u32::bit_and(a3599, a3600)
                            };
                            let a3602 = 4096u32;
                            if equal(a3601, a3602) {
                                4096u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3603, a3604)?
                    };
                    nat_add(a3605, a3606)?
                };
                let a3630 = {
                    let a3627 = {
                        let a3615 = {
                            let a3609 = {
                                let a3607 = v2;
                                let a3608 = 2048u32;
                                fixed_u32::bit_and(a3607, a3608)
                            };
                            let a3610 = 2048u32;
                            if equal(a3609, a3610) {
                                2048u64
                            } else {
                                0u64
                            }
                        };
                        let a3616 = {
                            let a3613 = {
                                let a3611 = v2;
                                let a3612 = 1024u32;
                                fixed_u32::bit_and(a3611, a3612)
                            };
                            let a3614 = 1024u32;
                            if equal(a3613, a3614) {
                                1024u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3615, a3616)?
                    };
                    let a3628 = {
                        let a3625 = {
                            let a3619 = {
                                let a3617 = v2;
                                let a3618 = 512u32;
                                fixed_u32::bit_and(a3617, a3618)
                            };
                            let a3620 = 512u32;
                            if equal(a3619, a3620) {
                                512u64
                            } else {
                                0u64
                            }
                        };
                        let a3626 = {
                            let a3623 = {
                                let a3621 = v2;
                                let a3622 = 256u32;
                                fixed_u32::bit_and(a3621, a3622)
                            };
                            let a3624 = 256u32;
                            if equal(a3623, a3624) {
                                256u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3625, a3626)?
                    };
                    nat_add(a3627, a3628)?
                };
                nat_add(a3629, a3630)?
            };
            let a3678 = {
                let a3675 = {
                    let a3651 = {
                        let a3639 = {
                            let a3633 = {
                                let a3631 = v2;
                                let a3632 = 128u32;
                                fixed_u32::bit_and(a3631, a3632)
                            };
                            let a3634 = 128u32;
                            if equal(a3633, a3634) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a3640 = {
                            let a3637 = {
                                let a3635 = v2;
                                let a3636 = 64u32;
                                fixed_u32::bit_and(a3635, a3636)
                            };
                            let a3638 = 64u32;
                            if equal(a3637, a3638) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3639, a3640)?
                    };
                    let a3652 = {
                        let a3649 = {
                            let a3643 = {
                                let a3641 = v2;
                                let a3642 = 32u32;
                                fixed_u32::bit_and(a3641, a3642)
                            };
                            let a3644 = 32u32;
                            if equal(a3643, a3644) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a3650 = {
                            let a3647 = {
                                let a3645 = v2;
                                let a3646 = 16u32;
                                fixed_u32::bit_and(a3645, a3646)
                            };
                            let a3648 = 16u32;
                            if equal(a3647, a3648) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3649, a3650)?
                    };
                    nat_add(a3651, a3652)?
                };
                let a3676 = {
                    let a3673 = {
                        let a3661 = {
                            let a3655 = {
                                let a3653 = v2;
                                let a3654 = 8u32;
                                fixed_u32::bit_and(a3653, a3654)
                            };
                            let a3656 = 8u32;
                            if equal(a3655, a3656) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a3662 = {
                            let a3659 = {
                                let a3657 = v2;
                                let a3658 = 4u32;
                                fixed_u32::bit_and(a3657, a3658)
                            };
                            let a3660 = 4u32;
                            if equal(a3659, a3660) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3661, a3662)?
                    };
                    let a3674 = {
                        let a3671 = {
                            let a3665 = {
                                let a3663 = v2;
                                let a3664 = 2u32;
                                fixed_u32::bit_and(a3663, a3664)
                            };
                            let a3666 = 2u32;
                            if equal(a3665, a3666) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a3672 = {
                            let a3669 = {
                                let a3667 = v2;
                                let a3668 = 1u32;
                                fixed_u32::bit_and(a3667, a3668)
                            };
                            let a3670 = 1u32;
                            if equal(a3669, a3670) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3671, a3672)?
                    };
                    nat_add(a3673, a3674)?
                };
                nat_add(a3675, a3676)?
            };
            nat_add(a3677, a3678)?
        };
        nat_add(a3679, a3680)?
    }))
}

pub fn f831() -> u64 {
    64u64
}

pub fn f832(v0: Adt159) -> Adt160 {
    Adt160::C0(match v0.clone() {
        Adt159::C0(h3681, _, _, _, _, _, _, _) => {
            h3681
        }
    }, match v0.clone() {
        Adt159::C0(_, h3682, _, _, _, _, _, _) => {
            h3682
        }
    }, match v0.clone() {
        Adt159::C0(_, _, h3683, _, _, _, _, _) => {
            h3683
        }
    }, match v0.clone() {
        Adt159::C0(_, _, _, h3684, _, _, _, _) => {
            h3684
        }
    }, match v0.clone() {
        Adt159::C0(_, _, _, _, h3685, _, _, _) => {
            h3685
        }
    }, match v0.clone() {
        Adt159::C0(_, _, _, _, _, h3686, _, _) => {
            h3686
        }
    }, match v0.clone() {
        Adt159::C0(_, _, _, _, _, _, h3687, _) => {
            h3687
        }
    }, match v0 {
        Adt159::C0(_, _, _, _, _, _, _, h3688) => {
            h3688
        }
    }, f840(), f840(), f840(), f840(), f840(), f840(), f840(), f840())
}

pub fn f833(v0: Bytes) -> R<Adt159> {
    let v1: u64 = {
        let a3689 = v0.clone();
        length_bytes(a3689)
    };
    let v2: u64 = {
        let a3695 = {
            let a3692 = {
                let a3690 = v1;
                let a3691 = 72u64;
                nat_add(a3690, a3691)?
            };
            let a3693 = 64u64;
            let a3694 = 0u64;
            nat_quot(a3692, a3693, a3694)
        };
        let a3696 = 64u64;
        nat_mul(a3695, a3696)?
    };
    let (_, h3697) = f835(Fn7::F843(v0, v1, v2), {
        let a3698 = v2;
        let a3699 = 64u64;
        let a3700 = 0u64;
        nat_quot(a3698, a3699, a3700)
    }, (0u64, f825()))?;
    Ok(h3697)
}

pub fn f834(v0: Bytes) -> R<Adt160> {
    Ok(Adt160::C0(f844(v0.clone(), 0u64)?, f844(v0.clone(), 4u64)?, f844(v0.clone(), 8u64)?, f844(v0.clone(), 12u64)?, f844(v0.clone(), 16u64)?, f844(v0.clone(), 20u64)?, f844(v0.clone(), 24u64)?, f844(v0.clone(), 28u64)?, f844(v0.clone(), 32u64)?, f844(v0.clone(), 36u64)?, f844(v0.clone(), 40u64)?, f844(v0.clone(), 44u64)?, f844(v0.clone(), 48u64)?, f844(v0.clone(), 52u64)?, f844(v0.clone(), 56u64)?, f844(v0, 60u64)?))
}

pub fn f835(v0: Fn7, p3704: u64, p3705: (u64, Adt159)) -> R<(u64, Adt159)> {
    let m3703 = (p3704, p3705);
    {
        let mut m3703 = m3703;
        loop {
            let step = {
                let (v1, v2) = m3703;
                let m3701 = v1;
                if m3701 == 0 {
                    Err::<(u64, (u64, Adt159)), R<(u64, Adt159)>>(Ok(v2))
                } else {
                    let v3 = m3701 - 1;
                    Ok::<(u64, (u64, Adt159)), R<(u64, Adt159)>>((v3, {
                        let c3702: Fn7 = v0.clone();
                        c3702.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m3703 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f836(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: (u64, Adt159)) -> R<(u64, Adt159)> {
    Ok(({
        let a3707 = {
            let (h3706, _) = v4.clone();
            h3706
        };
        let a3708 = 64u64;
        nat_add(a3707, a3708)?
    }, f824({
        let (_, h3709) = v4;
        h3709
    }, f845(v0, v2, v3, {
        let a3712 = {
            let a3710 = v1;
            let a3711 = v2;
            nat_add(a3710, a3711)?
        };
        let a3713 = 8u64;
        nat_mul(a3712, a3713)?
    }, {
        let (h3714, _) = v4;
        h3714
    })?)?))
}

pub fn f837(v0: Fn8, p3718: u64, p3719: Adt161) -> R<Adt161> {
    let m3717 = (p3718, p3719);
    {
        let mut m3717 = m3717;
        loop {
            let step = {
                let (v1, v2) = m3717;
                let m3715 = v1;
                if m3715 == 0 {
                    Err::<(u64, Adt161), R<Adt161>>(Ok(v2))
                } else {
                    let v3 = m3715 - 1;
                    Ok::<(u64, Adt161), R<Adt161>>((v3, {
                        let c3716: Fn8 = v0.clone();
                        c3716.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m3717 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f838(v0: Adt161) -> R<Adt161> {
    f846(v0)
}

pub fn f839(v0: u32, v1: u32) -> u32 {
    let m3722 = {
        let a3720 = f847(v0);
        let a3721 = f847(v1);
        fixed_u64::checked_add(a3720, a3721)
    };
    match m3722 {
        None => {
            0u32
        }
        Some(v2) => {
            f848(v2)
        }
    }
}

pub fn f840() -> u32 {
    0u32
}

pub fn f841(v0: Bytes, v1: u64) -> u8 {
    let m3725 = {
        let a3723 = v0;
        let a3724 = v1;
        index_bytes(a3723, a3724)
    };
    match m3725 {
        None => {
            0u8
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f842(v0: u32, v1: u32) -> u32 {
    let m3728 = {
        let a3726 = v0;
        let a3727 = v1;
        fixed_u32::shift_right(a3726, a3727)
    };
    match m3728 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f843(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt159)) -> R<(u64, Adt159)> {
    Ok(({
        let a3730 = {
            let (h3729, _) = v3.clone();
            h3729
        };
        let a3731 = 64u64;
        nat_add(a3730, a3731)?
    }, f824({
        let (_, h3732) = v3;
        h3732
    }, f849(v0, v1, v2, {
        let (h3733, _) = v3;
        h3733
    })?)?))
}

pub fn f844(v0: Bytes, v1: u64) -> R<u32> {
    Ok(f850(f851(v0.clone(), v1), f851(v0.clone(), {
        let a3734 = v1;
        let a3735 = 1u64;
        nat_add(a3734, a3735)?
    }), f851(v0.clone(), {
        let a3736 = v1;
        let a3737 = 2u64;
        nat_add(a3736, a3737)?
    }), f851(v0, {
        let a3738 = v1;
        let a3739 = 3u64;
        nat_add(a3738, a3739)?
    })))
}

pub fn f845(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Adt160> {
    Ok(Adt160::C0(f852(v0.clone(), v1, v2, v3, v4)?, f852(v0.clone(), v1, v2, v3, {
        let a3740 = v4;
        let a3741 = 4u64;
        nat_add(a3740, a3741)?
    })?, f852(v0.clone(), v1, v2, v3, {
        let a3742 = v4;
        let a3743 = 8u64;
        nat_add(a3742, a3743)?
    })?, f852(v0.clone(), v1, v2, v3, {
        let a3744 = v4;
        let a3745 = 12u64;
        nat_add(a3744, a3745)?
    })?, f852(v0.clone(), v1, v2, v3, {
        let a3746 = v4;
        let a3747 = 16u64;
        nat_add(a3746, a3747)?
    })?, f852(v0.clone(), v1, v2, v3, {
        let a3748 = v4;
        let a3749 = 20u64;
        nat_add(a3748, a3749)?
    })?, f852(v0.clone(), v1, v2, v3, {
        let a3750 = v4;
        let a3751 = 24u64;
        nat_add(a3750, a3751)?
    })?, f852(v0.clone(), v1, v2, v3, {
        let a3752 = v4;
        let a3753 = 28u64;
        nat_add(a3752, a3753)?
    })?, f852(v0.clone(), v1, v2, v3, {
        let a3754 = v4;
        let a3755 = 32u64;
        nat_add(a3754, a3755)?
    })?, f852(v0.clone(), v1, v2, v3, {
        let a3756 = v4;
        let a3757 = 36u64;
        nat_add(a3756, a3757)?
    })?, f852(v0.clone(), v1, v2, v3, {
        let a3758 = v4;
        let a3759 = 40u64;
        nat_add(a3758, a3759)?
    })?, f852(v0.clone(), v1, v2, v3, {
        let a3760 = v4;
        let a3761 = 44u64;
        nat_add(a3760, a3761)?
    })?, f852(v0.clone(), v1, v2, v3, {
        let a3762 = v4;
        let a3763 = 48u64;
        nat_add(a3762, a3763)?
    })?, f852(v0.clone(), v1, v2, v3, {
        let a3764 = v4;
        let a3765 = 52u64;
        nat_add(a3764, a3765)?
    })?, f852(v0.clone(), v1, v2, v3, {
        let a3766 = v4;
        let a3767 = 56u64;
        nat_add(a3766, a3767)?
    })?, f852(v0, v1, v2, v3, {
        let a3768 = v4;
        let a3769 = 60u64;
        nat_add(a3768, a3769)?
    })?))
}

pub fn f846(v0: Adt161) -> R<Adt161> {
    let v1: u32 = f853(match v0.clone() {
        Adt161::C0(_, _, _, _, _, _, _, _, _, h3770) => {
            h3770
        }
    });
    let v2: u32 = f839(match v0.clone() {
        Adt161::C0(_, _, _, _, _, _, _, h3771, _, _) => {
            h3771
        }
    }, f839(f854(match v0.clone() {
        Adt161::C0(_, _, _, _, h3772, _, _, _, _, _) => {
            h3772
        }
    }), f839(f855(match v0.clone() {
        Adt161::C0(_, _, _, _, h3773, _, _, _, _, _) => {
            h3773
        }
    }, match v0.clone() {
        Adt161::C0(_, _, _, _, _, h3774, _, _, _, _) => {
            h3774
        }
    }, match v0.clone() {
        Adt161::C0(_, _, _, _, _, _, h3775, _, _, _) => {
            h3775
        }
    }), f839(v1, {
        let m3778 = match v0.clone() {
            Adt161::C0(_, _, _, _, _, _, _, _, h3777, _) => {
                h3777
            }
        };
        match m3778 {
            Adt160::C0(h3776, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h3776
            }
        }
    }))));
    let v3: u32 = f839(f856(match v0.clone() {
        Adt161::C0(h3779, _, _, _, _, _, _, _, _, _) => {
            h3779
        }
    }), f857(match v0.clone() {
        Adt161::C0(h3780, _, _, _, _, _, _, _, _, _) => {
            h3780
        }
    }, match v0.clone() {
        Adt161::C0(_, h3781, _, _, _, _, _, _, _, _) => {
            h3781
        }
    }, match v0.clone() {
        Adt161::C0(_, _, h3782, _, _, _, _, _, _, _) => {
            h3782
        }
    }));
    Ok(Adt161::C0(f839(v2, v3), match v0.clone() {
        Adt161::C0(h3783, _, _, _, _, _, _, _, _, _) => {
            h3783
        }
    }, match v0.clone() {
        Adt161::C0(_, h3784, _, _, _, _, _, _, _, _) => {
            h3784
        }
    }, match v0.clone() {
        Adt161::C0(_, _, h3785, _, _, _, _, _, _, _) => {
            h3785
        }
    }, f839(match v0.clone() {
        Adt161::C0(_, _, _, h3786, _, _, _, _, _, _) => {
            h3786
        }
    }, v2), match v0.clone() {
        Adt161::C0(_, _, _, _, h3787, _, _, _, _, _) => {
            h3787
        }
    }, match v0.clone() {
        Adt161::C0(_, _, _, _, _, h3788, _, _, _, _) => {
            h3788
        }
    }, match v0.clone() {
        Adt161::C0(_, _, _, _, _, _, h3789, _, _, _) => {
            h3789
        }
    }, f858(match v0.clone() {
        Adt161::C0(_, _, _, _, _, _, _, _, h3790, _) => {
            h3790
        }
    }), {
        let a3792 = match v0 {
            Adt161::C0(_, _, _, _, _, _, _, _, _, h3791) => {
                h3791
            }
        };
        let a3793 = 1u64;
        nat_add(a3792, a3793)?
    }))
}

pub fn f847(v0: u32) -> u64 {
    let m3795 = {
        let a3794 = v0;
        fixed_u64::convert(i128::from(a3794))
    };
    match m3795 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f848(v0: u64) -> u32 {
    let m3799 = {
        let a3798 = {
            let a3796 = v0;
            let a3797 = 4294967295u64;
            fixed_u64::bit_and(a3796, a3797)
        };
        fixed_u32::convert(i128::from(a3798))
    };
    match m3799 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f849(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt160> {
    Ok(Adt160::C0(f859(v0.clone(), v1, v2, v3)?, f859(v0.clone(), v1, v2, {
        let a3800 = v3;
        let a3801 = 4u64;
        nat_add(a3800, a3801)?
    })?, f859(v0.clone(), v1, v2, {
        let a3802 = v3;
        let a3803 = 8u64;
        nat_add(a3802, a3803)?
    })?, f859(v0.clone(), v1, v2, {
        let a3804 = v3;
        let a3805 = 12u64;
        nat_add(a3804, a3805)?
    })?, f859(v0.clone(), v1, v2, {
        let a3806 = v3;
        let a3807 = 16u64;
        nat_add(a3806, a3807)?
    })?, f859(v0.clone(), v1, v2, {
        let a3808 = v3;
        let a3809 = 20u64;
        nat_add(a3808, a3809)?
    })?, f859(v0.clone(), v1, v2, {
        let a3810 = v3;
        let a3811 = 24u64;
        nat_add(a3810, a3811)?
    })?, f859(v0.clone(), v1, v2, {
        let a3812 = v3;
        let a3813 = 28u64;
        nat_add(a3812, a3813)?
    })?, f859(v0.clone(), v1, v2, {
        let a3814 = v3;
        let a3815 = 32u64;
        nat_add(a3814, a3815)?
    })?, f859(v0.clone(), v1, v2, {
        let a3816 = v3;
        let a3817 = 36u64;
        nat_add(a3816, a3817)?
    })?, f859(v0.clone(), v1, v2, {
        let a3818 = v3;
        let a3819 = 40u64;
        nat_add(a3818, a3819)?
    })?, f859(v0.clone(), v1, v2, {
        let a3820 = v3;
        let a3821 = 44u64;
        nat_add(a3820, a3821)?
    })?, f859(v0.clone(), v1, v2, {
        let a3822 = v3;
        let a3823 = 48u64;
        nat_add(a3822, a3823)?
    })?, f859(v0.clone(), v1, v2, {
        let a3824 = v3;
        let a3825 = 52u64;
        nat_add(a3824, a3825)?
    })?, f859(v0.clone(), v1, v2, {
        let a3826 = v3;
        let a3827 = 56u64;
        nat_add(a3826, a3827)?
    })?, f859(v0, v1, v2, {
        let a3828 = v3;
        let a3829 = 60u64;
        nat_add(a3828, a3829)?
    })?))
}

pub fn f850(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a3834 = f860(v0, 24u32);
    let a3835 = {
        let a3832 = f860(v1, 16u32);
        let a3833 = {
            let a3830 = f860(v2, 8u32);
            let a3831 = v3;
            fixed_u32::bit_or(a3830, a3831)
        };
        fixed_u32::bit_or(a3832, a3833)
    };
    fixed_u32::bit_or(a3834, a3835)
}

pub fn f851(v0: Bytes, v1: u64) -> u32 {
    let m3838 = {
        let a3836 = v0;
        let a3837 = v1;
        index_bytes(a3836, a3837)
    };
    match m3838 {
        None => {
            0u32
        }
        Some(v2) => {
            let m3840 = {
                let a3839 = v2;
                fixed_u32::convert(i128::from(a3839))
            };
            match m3840 {
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

pub fn f852(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u32> {
    let a3843 = {
        let a3841 = v4;
        let a3842 = 8u64;
        nat_add(a3841, a3842)?
    };
    let a3844 = v2;
    if nat_eq(a3843, a3844) {
        Ok(f861(v3, true))
    } else {
        let a3847 = {
            let a3845 = v4;
            let a3846 = 4u64;
            nat_add(a3845, a3846)?
        };
        let a3848 = v2;
        if nat_eq(a3847, a3848) {
            Ok(f861(v3, false))
        } else {
            Ok(f850(f862(v0.clone(), v1, v4), f862(v0.clone(), v1, {
                let a3849 = v4;
                let a3850 = 1u64;
                nat_add(a3849, a3850)?
            }), f862(v0.clone(), v1, {
                let a3851 = v4;
                let a3852 = 2u64;
                nat_add(a3851, a3852)?
            }), f862(v0, v1, {
                let a3853 = v4;
                let a3854 = 3u64;
                nat_add(a3853, a3854)?
            })))
        }
    }
}

pub fn f853(v0: u64) -> u32 {
    let a3855 = v0;
    let a3856 = 32u64;
    if nat_lt(a3855, a3856) {
        let a3857 = v0;
        let a3858 = 16u64;
        if nat_lt(a3857, a3858) {
            let a3859 = v0;
            let a3860 = 8u64;
            if nat_lt(a3859, a3860) {
                let a3861 = v0;
                let a3862 = 4u64;
                if nat_lt(a3861, a3862) {
                    let a3863 = v0;
                    let a3864 = 2u64;
                    if nat_lt(a3863, a3864) {
                        let a3865 = v0;
                        let a3866 = 1u64;
                        if nat_lt(a3865, a3866) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a3867 = v0;
                        let a3868 = 3u64;
                        if nat_lt(a3867, a3868) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a3869 = v0;
                    let a3870 = 6u64;
                    if nat_lt(a3869, a3870) {
                        let a3871 = v0;
                        let a3872 = 5u64;
                        if nat_lt(a3871, a3872) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a3873 = v0;
                        let a3874 = 7u64;
                        if nat_lt(a3873, a3874) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a3875 = v0;
                let a3876 = 12u64;
                if nat_lt(a3875, a3876) {
                    let a3877 = v0;
                    let a3878 = 10u64;
                    if nat_lt(a3877, a3878) {
                        let a3879 = v0;
                        let a3880 = 9u64;
                        if nat_lt(a3879, a3880) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a3881 = v0;
                        let a3882 = 11u64;
                        if nat_lt(a3881, a3882) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a3883 = v0;
                    let a3884 = 14u64;
                    if nat_lt(a3883, a3884) {
                        let a3885 = v0;
                        let a3886 = 13u64;
                        if nat_lt(a3885, a3886) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a3887 = v0;
                        let a3888 = 15u64;
                        if nat_lt(a3887, a3888) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a3889 = v0;
            let a3890 = 24u64;
            if nat_lt(a3889, a3890) {
                let a3891 = v0;
                let a3892 = 20u64;
                if nat_lt(a3891, a3892) {
                    let a3893 = v0;
                    let a3894 = 18u64;
                    if nat_lt(a3893, a3894) {
                        let a3895 = v0;
                        let a3896 = 17u64;
                        if nat_lt(a3895, a3896) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a3897 = v0;
                        let a3898 = 19u64;
                        if nat_lt(a3897, a3898) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a3899 = v0;
                    let a3900 = 22u64;
                    if nat_lt(a3899, a3900) {
                        let a3901 = v0;
                        let a3902 = 21u64;
                        if nat_lt(a3901, a3902) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a3903 = v0;
                        let a3904 = 23u64;
                        if nat_lt(a3903, a3904) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a3905 = v0;
                let a3906 = 28u64;
                if nat_lt(a3905, a3906) {
                    let a3907 = v0;
                    let a3908 = 26u64;
                    if nat_lt(a3907, a3908) {
                        let a3909 = v0;
                        let a3910 = 25u64;
                        if nat_lt(a3909, a3910) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a3911 = v0;
                        let a3912 = 27u64;
                        if nat_lt(a3911, a3912) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a3913 = v0;
                    let a3914 = 30u64;
                    if nat_lt(a3913, a3914) {
                        let a3915 = v0;
                        let a3916 = 29u64;
                        if nat_lt(a3915, a3916) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a3917 = v0;
                        let a3918 = 31u64;
                        if nat_lt(a3917, a3918) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a3919 = v0;
        let a3920 = 48u64;
        if nat_lt(a3919, a3920) {
            let a3921 = v0;
            let a3922 = 40u64;
            if nat_lt(a3921, a3922) {
                let a3923 = v0;
                let a3924 = 36u64;
                if nat_lt(a3923, a3924) {
                    let a3925 = v0;
                    let a3926 = 34u64;
                    if nat_lt(a3925, a3926) {
                        let a3927 = v0;
                        let a3928 = 33u64;
                        if nat_lt(a3927, a3928) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a3929 = v0;
                        let a3930 = 35u64;
                        if nat_lt(a3929, a3930) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a3931 = v0;
                    let a3932 = 38u64;
                    if nat_lt(a3931, a3932) {
                        let a3933 = v0;
                        let a3934 = 37u64;
                        if nat_lt(a3933, a3934) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a3935 = v0;
                        let a3936 = 39u64;
                        if nat_lt(a3935, a3936) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a3937 = v0;
                let a3938 = 44u64;
                if nat_lt(a3937, a3938) {
                    let a3939 = v0;
                    let a3940 = 42u64;
                    if nat_lt(a3939, a3940) {
                        let a3941 = v0;
                        let a3942 = 41u64;
                        if nat_lt(a3941, a3942) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a3943 = v0;
                        let a3944 = 43u64;
                        if nat_lt(a3943, a3944) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a3945 = v0;
                    let a3946 = 46u64;
                    if nat_lt(a3945, a3946) {
                        let a3947 = v0;
                        let a3948 = 45u64;
                        if nat_lt(a3947, a3948) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a3949 = v0;
                        let a3950 = 47u64;
                        if nat_lt(a3949, a3950) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a3951 = v0;
            let a3952 = 56u64;
            if nat_lt(a3951, a3952) {
                let a3953 = v0;
                let a3954 = 52u64;
                if nat_lt(a3953, a3954) {
                    let a3955 = v0;
                    let a3956 = 50u64;
                    if nat_lt(a3955, a3956) {
                        let a3957 = v0;
                        let a3958 = 49u64;
                        if nat_lt(a3957, a3958) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a3959 = v0;
                        let a3960 = 51u64;
                        if nat_lt(a3959, a3960) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a3961 = v0;
                    let a3962 = 54u64;
                    if nat_lt(a3961, a3962) {
                        let a3963 = v0;
                        let a3964 = 53u64;
                        if nat_lt(a3963, a3964) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a3965 = v0;
                        let a3966 = 55u64;
                        if nat_lt(a3965, a3966) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a3967 = v0;
                let a3968 = 60u64;
                if nat_lt(a3967, a3968) {
                    let a3969 = v0;
                    let a3970 = 58u64;
                    if nat_lt(a3969, a3970) {
                        let a3971 = v0;
                        let a3972 = 57u64;
                        if nat_lt(a3971, a3972) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a3973 = v0;
                        let a3974 = 59u64;
                        if nat_lt(a3973, a3974) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a3975 = v0;
                    let a3976 = 62u64;
                    if nat_lt(a3975, a3976) {
                        let a3977 = v0;
                        let a3978 = 61u64;
                        if nat_lt(a3977, a3978) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a3979 = v0;
                        let a3980 = 63u64;
                        if nat_lt(a3979, a3980) {
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

pub fn f854(v0: u32) -> u32 {
    let a3983 = {
        let a3981 = f863(v0, 6u32, 26u32);
        let a3982 = f863(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a3981, a3982)
    };
    let a3984 = f863(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a3983, a3984)
}

pub fn f855(v0: u32, v1: u32, v2: u32) -> u32 {
    let a3990 = {
        let a3985 = v0;
        let a3986 = v1;
        fixed_u32::bit_and(a3985, a3986)
    };
    let a3991 = {
        let a3988 = {
            let a3987 = v0;
            fixed_u32::bit_not(a3987)
        };
        let a3989 = v2;
        fixed_u32::bit_and(a3988, a3989)
    };
    fixed_u32::bit_xor(a3990, a3991)
}

pub fn f856(v0: u32) -> u32 {
    let a3994 = {
        let a3992 = f863(v0, 2u32, 30u32);
        let a3993 = f863(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a3992, a3993)
    };
    let a3995 = f863(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a3994, a3995)
}

pub fn f857(v0: u32, v1: u32, v2: u32) -> u32 {
    let a4004 = {
        let a4000 = {
            let a3996 = v0;
            let a3997 = v1;
            fixed_u32::bit_and(a3996, a3997)
        };
        let a4001 = {
            let a3998 = v0;
            let a3999 = v2;
            fixed_u32::bit_and(a3998, a3999)
        };
        fixed_u32::bit_xor(a4000, a4001)
    };
    let a4005 = {
        let a4002 = v1;
        let a4003 = v2;
        fixed_u32::bit_and(a4002, a4003)
    };
    fixed_u32::bit_xor(a4004, a4005)
}

pub fn f858(v0: Adt160) -> Adt160 {
    Adt160::C0(match v0.clone() {
        Adt160::C0(_, h4006, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h4006
        }
    }, match v0.clone() {
        Adt160::C0(_, _, h4007, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h4007
        }
    }, match v0.clone() {
        Adt160::C0(_, _, _, h4008, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h4008
        }
    }, match v0.clone() {
        Adt160::C0(_, _, _, _, h4009, _, _, _, _, _, _, _, _, _, _, _) => {
            h4009
        }
    }, match v0.clone() {
        Adt160::C0(_, _, _, _, _, h4010, _, _, _, _, _, _, _, _, _, _) => {
            h4010
        }
    }, match v0.clone() {
        Adt160::C0(_, _, _, _, _, _, h4011, _, _, _, _, _, _, _, _, _) => {
            h4011
        }
    }, match v0.clone() {
        Adt160::C0(_, _, _, _, _, _, _, h4012, _, _, _, _, _, _, _, _) => {
            h4012
        }
    }, match v0.clone() {
        Adt160::C0(_, _, _, _, _, _, _, _, h4013, _, _, _, _, _, _, _) => {
            h4013
        }
    }, match v0.clone() {
        Adt160::C0(_, _, _, _, _, _, _, _, _, h4014, _, _, _, _, _, _) => {
            h4014
        }
    }, match v0.clone() {
        Adt160::C0(_, _, _, _, _, _, _, _, _, _, h4015, _, _, _, _, _) => {
            h4015
        }
    }, match v0.clone() {
        Adt160::C0(_, _, _, _, _, _, _, _, _, _, _, h4016, _, _, _, _) => {
            h4016
        }
    }, match v0.clone() {
        Adt160::C0(_, _, _, _, _, _, _, _, _, _, _, _, h4017, _, _, _) => {
            h4017
        }
    }, match v0.clone() {
        Adt160::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h4018, _, _) => {
            h4018
        }
    }, match v0.clone() {
        Adt160::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h4019, _) => {
            h4019
        }
    }, match v0.clone() {
        Adt160::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h4020) => {
            h4020
        }
    }, f839(f864(match v0.clone() {
        Adt160::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h4021, _) => {
            h4021
        }
    }), f839(match v0.clone() {
        Adt160::C0(_, _, _, _, _, _, _, _, _, h4022, _, _, _, _, _, _) => {
            h4022
        }
    }, f839(f865(match v0.clone() {
        Adt160::C0(_, h4023, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h4023
        }
    }), match v0 {
        Adt160::C0(h4024, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h4024
        }
    }))))
}

pub fn f859(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a4027 = {
        let a4025 = v3;
        let a4026 = 8u64;
        nat_add(a4025, a4026)?
    };
    let a4028 = v2;
    if nat_eq(a4027, a4028) {
        Ok(f866({
            let a4031 = {
                let a4029 = v1;
                let a4030 = 8u64;
                nat_mul(a4029, a4030)?
            };
            let a4032 = 4294967296u64;
            let a4033 = 0u64;
            nat_quot(a4031, a4032, a4033)
        }))
    } else {
        let a4036 = {
            let a4034 = v3;
            let a4035 = 4u64;
            nat_add(a4034, a4035)?
        };
        let a4037 = v2;
        if nat_eq(a4036, a4037) {
            Ok(f866({
                let a4040 = {
                    let a4038 = v1;
                    let a4039 = 8u64;
                    nat_mul(a4038, a4039)?
                };
                let a4041 = 4294967296u64;
                let a4042 = 0u64;
                nat_rem(a4040, a4041, a4042)
            }))
        } else {
            Ok(f850(f862(v0.clone(), v1, v3), f862(v0.clone(), v1, {
                let a4043 = v3;
                let a4044 = 1u64;
                nat_add(a4043, a4044)?
            }), f862(v0.clone(), v1, {
                let a4045 = v3;
                let a4046 = 2u64;
                nat_add(a4045, a4046)?
            }), f862(v0, v1, {
                let a4047 = v3;
                let a4048 = 3u64;
                nat_add(a4047, a4048)?
            })))
        }
    }
}

pub fn f860(v0: u32, v1: u32) -> u32 {
    let m4051 = {
        let a4049 = v0;
        let a4050 = v1;
        fixed_u32::shift_left(a4049, a4050)
    };
    match m4051 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f861(v0: u64, v1: bool) -> u32 {
    if v1 {
        f866({
            let a4052 = v0;
            let a4053 = 4294967296u64;
            let a4054 = 0u64;
            nat_quot(a4052, a4053, a4054)
        })
    } else {
        f866({
            let a4055 = v0;
            let a4056 = 4294967296u64;
            let a4057 = 0u64;
            nat_rem(a4055, a4056, a4057)
        })
    }
}

pub fn f862(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a4058 = v2;
    let a4059 = v1;
    if nat_lt(a4058, a4059) {
        let m4062 = {
            let a4060 = v0;
            let a4061 = v2;
            index_bytes(a4060, a4061)
        };
        match m4062 {
            None => {
                0u32
            }
            Some(v3) => {
                let m4064 = {
                    let a4063 = v3;
                    fixed_u32::convert(i128::from(a4063))
                };
                match m4064 {
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
        let a4065 = v2;
        let a4066 = v1;
        if nat_eq(a4065, a4066) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f863(v0: u32, v1: u32, v2: u32) -> u32 {
    let a4067 = f842(v0, v1);
    let a4068 = f860(v0, v2);
    fixed_u32::bit_or(a4067, a4068)
}

pub fn f864(v0: u32) -> u32 {
    let a4071 = {
        let a4069 = f863(v0, 17u32, 15u32);
        let a4070 = f863(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a4069, a4070)
    };
    let a4072 = f842(v0, 10u32);
    fixed_u32::bit_xor(a4071, a4072)
}

pub fn f865(v0: u32) -> u32 {
    let a4075 = {
        let a4073 = f863(v0, 7u32, 25u32);
        let a4074 = f863(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a4073, a4074)
    };
    let a4076 = f842(v0, 3u32);
    fixed_u32::bit_xor(a4075, a4076)
}

pub fn f866(v0: u64) -> u32 {
    let (_, h4077) = {
        let (_, h4078) = f867(Fn9::F868, 32u64, (v0, (1u32, 0u32)));
        h4078
    };
    h4077
}

pub fn f867(v0: Fn9, p4082: u64, p4083: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m4081 = (p4082, p4083);
    {
        let mut m4081 = m4081;
        loop {
            let step = {
                let (v1, v2) = m4081;
                let m4079 = v1;
                if m4079 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m4079 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c4080: Fn9 = v0.clone();
                        c4080.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m4081 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f868(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h4084, _) = v0;
        h4084
    };
    let v2: u32 = {
        let (h4085, _) = {
            let (_, h4086) = v0;
            h4086
        };
        h4085
    };
    let v3: u32 = {
        let (_, h4087) = {
            let (_, h4088) = v0;
            h4088
        };
        h4087
    };
    ({
        let a4089 = v1;
        let a4090 = 2u64;
        let a4091 = 0u64;
        nat_quot(a4089, a4090, a4091)
    }, (f860(v2, 1u32), {
        let a4095 = {
            let a4092 = v1;
            let a4093 = 2u64;
            let a4094 = 0u64;
            nat_rem(a4092, a4093, a4094)
        };
        let a4096 = 1u64;
        if nat_eq(a4095, a4096) {
            let a4097 = v3;
            let a4098 = v2;
            fixed_u32::bit_or(a4097, a4098)
        } else {
            v3
        }
    }))
}

pub fn run(p0: Bytes) -> R<List<u64>> {
    f0(p0)
}
