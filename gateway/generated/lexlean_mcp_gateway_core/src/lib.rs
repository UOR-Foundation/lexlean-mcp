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
pub enum Adt105 {
    C0(Str),
    C1(i64),
}

#[derive(Clone)]
pub enum Adt106 {
    C0(Adt6, bool, List<Str>),
}

#[derive(Clone)]
pub enum Adt107 {
    C0(Adt101, List<Adt102>),
}

#[derive(Clone)]
pub enum Adt108 {
    C0,
    C1(Adt6),
}

#[derive(Clone)]
pub enum Adt109 {
    C0,
    C1(Option<Adt105>, Adt110),
}

#[derive(Clone)]
pub enum Adt110 {
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
pub enum Adt111 {
    C0,
    C1,
    C2,
    C3(List<u64>),
}

#[derive(Clone)]
pub enum Adt112 {
    C0,
    C1(List<u64>),
    C2,
}

#[derive(Clone)]
pub enum Adt113 {
    C0(List<Adt12>, List<List<u64>>),
}

#[derive(Clone)]
pub enum Adt114 {
    C0(Str),
    C1(Str),
    C2(Str),
    C3(List<Adt102>),
}

#[derive(Clone)]
pub enum Adt115 {
    C0(List<u64>),
    C1(u64, Adt6),
    C2(u64, List<Adt6>),
    C3(u64, List<Adt6>),
    C4(u64, List<(Str, Adt6)>),
    C5(u64, List<(Str, Adt6)>),
}

#[derive(Clone)]
pub enum Adt116 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt117 {
    C0(Adt105, Str, Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt118 {
    C0,
    C1,
}

#[derive(Clone)]
pub enum Adt119 {
    C0,
    C1(u64),
    C2(u64),
}

#[derive(Clone)]
pub enum Adt120 {
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt121 {
    C0(Adt122, List<Adt126>, u64),
}

#[derive(Clone)]
pub enum Adt122 {
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6(Adt123, List<u64>, bool),
    C7(u64, u64),
    C8(Adt124),
    C9(Adt6),
}

#[derive(Clone)]
pub enum Adt123 {
    C0,
    C1,
    C2(u64, u64, u64),
    C3(u64),
    C4(u64),
}

#[derive(Clone)]
pub enum Adt124 {
    C0(bool, Adt125, i64, u64, i64, bool, i64),
}

#[derive(Clone)]
pub enum Adt125 {
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
pub enum Adt126 {
    C0(List<Adt6>),
    C1(List<(Str, Adt6)>, Option<Str>),
}

#[derive(Clone)]
pub enum Adt127 {
    C0(Option<Str>, Option<Str>, Option<Str>, Option<Adt128>, List<Adt130>),
}

#[derive(Clone)]
pub enum Adt128 {
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6(Adt129),
    C7(Adt129),
    C8(Str),
    C9(Str),
    C10(Str),
}

#[derive(Clone)]
pub enum Adt129 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt130 {
    C0(Str, List<Str>, Adt13, Adt131),
}

#[derive(Clone)]
pub enum Adt131 {
    C0,
    C1,
    C2(Str),
}

#[derive(Clone)]
pub enum Adt132 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt133 {
    C0(Adt104),
    C1(List<Adt115>, List<u64>),
}

#[derive(Clone)]
pub enum Adt134 {
    C0(Adt104),
    C1(Adt121),
}

#[derive(Clone)]
pub enum Adt135 {
    C0,
    C1(List<(Adt6, Adt6)>),
}

#[derive(Clone)]
pub enum Adt136 {
    C0(Adt105, Result<Adt137, Adt140>),
}

#[derive(Clone)]
pub enum Adt137 {
    C0(Adt34, Adt138, List<Adt139>),
}

#[derive(Clone)]
pub enum Adt138 {
    C0(Option<Adt1>, Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt139 {
    C0(Adt21, Adt52),
}

#[derive(Clone)]
pub enum Adt140 {
    C0(Adt141),
    C1(Str, List<Str>),
    C2(Adt128),
    C3(Str),
    C4(Adt86),
}

#[derive(Clone)]
pub enum Adt141 {
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
pub enum Adt142 {
    C0(Option<Str>, Option<Adt128>),
}

#[derive(Clone)]
pub enum Adt143 {
    C0,
    C1(Adt105),
    C2,
}

#[derive(Clone)]
pub enum Adt144 {
    C0(Adt93),
    C1(Adt113),
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
    C0(Adt105, Adt148, Adt18, Adt5),
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
    C1(Adt140),
}

#[derive(Clone)]
pub enum Adt151 {
    C0(Adt105, Str, Adt18, Adt5),
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
    F46(Fn0, Adt89),
}

impl Fn2 {
    pub fn apply(&self, p0: Adt16) -> R<Result<Adt34, Adt86>> {
        match self {
            Fn2::F46(k0, k1) => f46(k0.clone(), k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F58(Fn1, Adt37),
}

impl Fn3 {
    pub fn apply(&self, p0: Adt23) -> R<Result<Adt34, Adt86>> {
        match self {
            Fn3::F58(k0, k1) => f58(k0.clone(), k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F833(Bytes),
    F858(Bytes, Bytes),
}

impl Fn4 {
    pub fn apply(&self, p0: (u64, u64)) -> R<(u64, u64)> {
        match self {
            Fn4::F833(k0) => f833(k0.clone(), p0),
            Fn4::F858(k0, k1) => f858(k0.clone(), k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F838(Bytes),
}

impl Fn5 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<(u64, Bytes)> {
        match self {
            Fn5::F838(k0) => f838(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F843(Bytes),
}

impl Fn6 {
    pub fn apply(&self, p0: (u64, (Bytes, u64))) -> R<(u64, (Bytes, u64))> {
        match self {
            Fn6::F843(k0) => f843(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F866(Bytes, u64, u64, u64),
    F873(Bytes, u64, u64),
}

impl Fn7 {
    pub fn apply(&self, p0: (u64, Adt159)) -> R<(u64, Adt159)> {
        match self {
            Fn7::F866(k0, k1, k2, k3) => f866(k0.clone(), *k1, *k2, *k3, p0),
            Fn7::F873(k0, k1, k2) => f873(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F868,
}

impl Fn8 {
    pub fn apply(&self, p0: Adt161) -> R<Adt161> {
        match self {
            Fn8::F868 => f868(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F898,
}

impl Fn9 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn9::F898 => f898(p0),
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
    }, List::cons(Str::lit("project"), List::<Str>::nil())), f31(Str::lit("fmt"))), List::cons(f29(Str::lit("lexlean_build"), Str::lit("Build a project with the LexLean engine as lexlean build does."), f25(f30()?, List::cons(Str::lit("project"), List::<Str>::nil())), f31(Str::lit("build"))), List::cons(f29(Str::lit("lexlean_explain"), Str::lit("Explain a diagnostic code of LexLean: the result is what lexlean explain prints with --diagnostic-format json, and a code the registry does not hold is an error result that carries the refusal of the CLI."), f25(f27(Str::lit("code"), f28(Str::lit("string"))), List::cons(Str::lit("code"), List::<Str>::nil())), f33()), List::cons(f24(Str::lit("lexlean_verify"), Str::lit("Start the verification of a project and return its task."), f25(f34()?, List::cons(Str::lit("project"), List::<Str>::nil()))), List::cons(f24(Str::lit("lexlean_verify_control"), Str::lit("Read the state of a verification task, or cancel it."), f25(List::cons((Str::lit("task"), f26()?), f27(Str::lit("action"), f35(Str::lit("status"), Str::lit("cancel")))), List::cons(Str::lit("task"), List::cons(Str::lit("action"), List::<Str>::nil())))), List::cons(f24(Str::lit("lexlean_lexicon_read"), Str::lit("Read an entry of the lexicon of a profile."), f25(List::cons((Str::lit("profile"), f26()?), f27(Str::lit("entry"), f26()?)), List::cons(Str::lit("profile"), List::cons(Str::lit("entry"), List::<Str>::nil())))), List::cons(f24(Str::lit("lexlean_attestation"), Str::lit("Look an attestation up by its ID or by the digest of its file."), f25(List::cons((Str::lit("by"), f35(Str::lit("id"), Str::lit("digest"))), f27(Str::lit("value"), f26()?)), List::cons(Str::lit("by"), List::cons(Str::lit("value"), List::<Str>::nil())))), List::<Adt37>::nil()))))))))))))
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
            Ok(Some(Adt86::C9(f36(v1))))
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
    let m22 = f37(match v2.clone() {
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
            f38(v3, v1, v2)
        }
    }
}

pub fn f15(v0: Adt14, v1: Adt23) -> R<Result<Adt51, Adt86>> {
    f39(match v0 {
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
    f40(v0.clone(), {
        let a46 = v0;
        length_bytes(a46)
    }, List::<u64>::nil())
}

pub fn f19(v0: Bytes) -> R<Result<Adt89, Adt90>> {
    f41(v0.clone(), f42(v0, 0u64)?)
}

pub fn f20() -> Adt88 {
    Adt88::C0(f43(f44()))
}

pub fn f21(v0: Adt0, v1: Fn0, v2: Adt89, v3: Result<Adt91, Adt90>) -> R<Adt88> {
    let m47 = v3;
    match m47 {
        Ok(v5) => {
            f45(v0, Fn2::F46(v1, v2), v5)
        }
        Err(_) => {
            Ok(f20())
        }
    }
}

pub fn f22(v0: Bytes) -> R<Result<Adt91, Adt90>> {
    let m48 = f42(v0.clone(), 0u64)?;
    match m48 {
        None => {
            Ok(Err::<Adt91, Adt90>(Adt90::C0(0u64)))
        }
        Some(v1) => {
            f47(v0.clone(), v1.clone(), f42(v0, match v1 {
                Adt94::C0(_, h49) => {
                    h49
                }
            })?)
        }
    }
}

pub fn f23(v0: List<Adt37>) -> R<Result<List<Adt12>, Adt93>> {
    let m50 = f48(v0.clone());
    match m50 {
        None => {
            f49(v0, List::<Adt12>::nil())
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
    Adt5::C0(List::cons((Str::lit("type"), f50(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(v0)), List::cons((Str::lit("required"), f51(v1, List::<Adt6>::nil())), f27(Str::lit("additionalProperties"), f52(false))))))
}

pub fn f26() -> R<Adt6> {
    Ok(Adt6::C5(List::cons((Str::lit("type"), f50(Str::lit("string"))), f27(Str::lit("minLength"), f53(1u64)?))))
}

pub fn f27(v0: Str, v1: Adt6) -> List<(Str, Adt6)> {
    List::cons((v0, v1), List::<(Str, Adt6)>::nil())
}

pub fn f28(v0: Str) -> Adt6 {
    Adt6::C5(f27(Str::lit("type"), f50(v0)))
}

pub fn f29(v0: Str, v1: Str, v2: Adt5, v3: Adt5) -> Adt37 {
    Adt37::C0(v0, None::<Str>, None::<List<Adt2>>, Some(v1), v2, Some(v3), None::<Adt38>, None::<Adt5>)
}

pub fn f30() -> R<List<(Str, Adt6)>> {
    let a51 = f34()?;
    let a52 = f27(Str::lit("all"), f28(Str::lit("boolean")));
    Ok(append_list(a51, a52))
}

pub fn f31(v0: Str) -> Adt5 {
    f25(List::cons((Str::lit("spec"), f54(Str::lit("lexlean/command-result/1"))), List::cons((Str::lit("command"), f54(v0)), List::cons((Str::lit("success"), f28(Str::lit("boolean"))), List::cons((Str::lit("exit_code"), f28(Str::lit("integer"))), List::cons((Str::lit("modules"), f55()), List::cons((Str::lit("artifacts"), f55()), List::cons((Str::lit("diagnostics"), f56()), List::cons((Str::lit("source_id"), f28(Str::lit("string"))), List::cons((Str::lit("semantic_id"), f28(Str::lit("string"))), List::cons((Str::lit("build_id"), f28(Str::lit("string"))), List::<(Str, Adt6)>::nil())))))))))), List::cons(Str::lit("spec"), List::cons(Str::lit("command"), List::cons(Str::lit("success"), List::cons(Str::lit("exit_code"), List::cons(Str::lit("modules"), List::cons(Str::lit("artifacts"), List::cons(Str::lit("diagnostics"), List::<Str>::nil()))))))))
}

pub fn f32() -> Adt5 {
    Adt5::C0(List::cons((Str::lit("type"), f50(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(f27(Str::lit("spec"), f28(Str::lit("string"))))), f27(Str::lit("required"), f51(List::cons(Str::lit("spec"), List::<Str>::nil()), List::<Adt6>::nil())))))
}

pub fn f33() -> Adt5 {
    f25(List::cons((Str::lit("spec"), f54(Str::lit("lexlean/command-result/1"))), List::cons((Str::lit("command"), f54(Str::lit("explain"))), List::cons((Str::lit("success"), f28(Str::lit("boolean"))), List::cons((Str::lit("exit_code"), f28(Str::lit("integer"))), List::cons((Str::lit("modules"), f55()), List::cons((Str::lit("artifacts"), f55()), List::cons((Str::lit("diagnostics"), f56()), f27(Str::lit("explanation"), f28(Str::lit("string")))))))))), List::cons(Str::lit("spec"), List::cons(Str::lit("command"), List::cons(Str::lit("success"), List::cons(Str::lit("exit_code"), List::cons(Str::lit("modules"), List::cons(Str::lit("artifacts"), List::cons(Str::lit("diagnostics"), List::<Str>::nil()))))))))
}

pub fn f34() -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("project"), f26()?), f27(Str::lit("files"), f55())))
}

pub fn f35(v0: Str, v1: Str) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), f50(Str::lit("string"))), f27(Str::lit("enum"), Adt6::C4(List::cons(f50(v0), List::cons(f50(v1), List::<Adt6>::nil()))))))
}

pub fn f36(v0: Adt93) -> Str {
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

pub fn f37(v0: Str, p85: List<Adt37>) -> Option<Adt37> {
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

pub fn f38(v0: Adt37, v1: Fn1, v2: Adt23) -> R<Result<Adt34, Adt86>> {
    f57(v0.clone(), v2, Fn3::F58(v1, v0))
}

pub fn f39(v0: List<Adt15>, v1: Adt23) -> R<Result<Adt51, Adt86>> {
    if f59(v1.clone(), Str::lit("lexlean_explain")) {
        let m86 = v0;
        match m86.uncons() {
            None => {
                let m87 = f60(v1, Str::lit("code"));
                match m87 {
                    None => {
                        Ok(Err::<Adt51, Adt86>(Adt86::C4(Str::lit("code is not a string"))))
                    }
                    Some(v2) => {
                        f61(v2)
                    }
                }
            }
            Some((_, _)) => {
                Ok(f62(Str::lit("the host performed an effect that nothing asked for")))
            }
        }
    } else {
        let m88 = f63(v1)?;
        match m88 {
            None => {
                Ok(Err::<Adt51, Adt86>(Adt86::C4(Str::lit("a file name holds a line feed, a draft is over draft.max_bytes, or all is true with files"))))
            }
            Some(v5) => {
                let m89 = v0;
                match m89.uncons() {
                    None => {
                        Ok(f64(f65(match v5.clone() {
                            Adt96::C0(h90, _) => {
                                h90
                            }
                        }), match v5 {
                            Adt96::C0(_, h91) => {
                                h91
                            }
                        }))
                    }
                    Some((v6, v7)) => {
                        let m92 = v7;
                        match m92.uncons() {
                            None => {
                                let a95 = match v6.clone() {
                                    Adt15::C0(h93, _) => {
                                        h93
                                    }
                                };
                                let a96 = f65(match v5.clone() {
                                    Adt96::C0(h94, _) => {
                                        h94
                                    }
                                });
                                if nat_eq(a95, a96) {
                                    f66(match v5 {
                                        Adt96::C0(h97, _) => {
                                            h97
                                        }
                                    }, match v6 {
                                        Adt15::C0(_, h98) => {
                                            h98
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
}

pub fn f40(v0: Bytes, p101: u64, p102: List<u64>) -> R<List<u64>> {
    let m100 = (p101, p102);
    {
        let mut m100 = m100;
        loop {
            let step = {
                let (v1, v2) = m100;
                let m99 = v1;
                if m99 == 0 {
                    Err::<(u64, List<u64>), R<List<u64>>>(Ok(v2))
                } else {
                    let v3 = m99 - 1;
                    Ok::<(u64, List<u64>), R<List<u64>>>((v3, List::cons(f67(v0.clone(), v3)?, v2)))
                }
            };
            match step {
                Ok(next) => { m100 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f41(v0: Bytes, v1: Option<Adt94>) -> R<Result<Adt89, Adt90>> {
    let m103 = v1;
    match m103 {
        None => {
            Ok(Err::<Adt89, Adt90>(Adt90::C0(0u64)))
        }
        Some(v2) => {
            f68(v0.clone(), v2.clone(), f69(v0, match v2 {
                Adt94::C0(_, h104) => {
                    h104
                }
            })?)
        }
    }
}

pub fn f42(v0: Bytes, v1: u64) -> R<Option<Adt94>> {
    let m105 = f69(v0.clone(), v1)?;
    match m105 {
        None => {
            Ok(None::<Adt94>)
        }
        Some(v2) => {
            let m111 = {
                let a108 = v0;
                let a109 = {
                    let a106 = v1;
                    let a107 = 4u64;
                    nat_add(a106, a107)?
                };
                let a110 = v2;
                slice_bytes(a108, a109, a110)
            };
            match m111 {
                None => {
                    Ok(None::<Adt94>)
                }
                Some(v3) => {
                    Ok(Some(Adt94::C0(v3, {
                        let a114 = {
                            let a112 = v1;
                            let a113 = 4u64;
                            nat_add(a112, a113)?
                        };
                        let a115 = v2;
                        nat_add(a114, a115)?
                    })))
                }
            }
        }
    }
}

pub fn f43(v0: Adt95) -> List<u64> {
    let a136 = f17(match v0.clone() {
        Adt95::C0(h116, _, _, _) => {
            h116
        }
    });
    let a137 = {
        let a134 = f17({
            let a118 = match v0.clone() {
                Adt95::C0(_, h117, _, _) => {
                    h117
                }
            };
            length_list(a118)
        });
        let a135 = {
            let a132 = f70(match v0.clone() {
                Adt95::C0(_, h119, _, _) => {
                    h119
                }
            });
            let a133 = {
                let a130 = f17({
                    let a121 = match v0.clone() {
                        Adt95::C0(_, _, h120, _) => {
                            h120
                        }
                    };
                    length_list(a121)
                });
                let a131 = {
                    let a128 = f71(match v0.clone() {
                        Adt95::C0(_, _, h122, _) => {
                            h122
                        }
                    });
                    let a129 = {
                        let a126 = f17({
                            let a124 = match v0.clone() {
                                Adt95::C0(_, _, _, h123) => {
                                    h123
                                }
                            };
                            length_list(a124)
                        });
                        let a127 = match v0 {
                            Adt95::C0(_, _, _, h125) => {
                                h125
                            }
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
    };
    append_list(a136, a137)
}

pub fn f44() -> Adt95 {
    Adt95::C0(f72(), f73(), List::<List<u64>>::nil(), f74())
}

pub fn f45(v0: Adt0, v1: Fn2, v2: Adt91) -> R<Adt88> {
    f75(v0, v1, v2.clone(), f76(match v2.clone() {
        Adt91::C0(_, _, h138, _, _, _, _) => {
            h138
        }
    }, match v2 {
        Adt91::C0(_, _, _, _, _, h139, _) => {
            h139
        }
    })?)
}

pub fn f46(v0: Fn0, v1: Adt89, v2: Adt16) -> R<Result<Adt34, Adt86>> {
    let c140: Fn0 = v0;
    c140.apply(Adt14::C0(match v1 {
        Adt89::C0(_, h141) => {
            h141
        }
    }, v2))
}

pub fn f47(v0: Bytes, v1: Adt94, v2: Option<Adt94>) -> R<Result<Adt91, Adt90>> {
    let m142 = v2;
    match m142 {
        None => {
            Ok(Err::<Adt91, Adt90>(Adt90::C0(match v1 {
                Adt94::C0(_, h143) => {
                    h143
                }
            })))
        }
        Some(v3) => {
            f77(v0.clone(), v1, v3.clone(), f69(v0, match v3 {
                Adt94::C0(_, h144) => {
                    h144
                }
            })?)
        }
    }
}

pub fn f48(v0: List<Adt37>) -> Option<Adt93> {
    let m145 = f78(v0, List::<Str>::nil());
    match m145 {
        None => {
            None::<Adt93>
        }
        Some(v1) => {
            Some(Adt93::C1(v1))
        }
    }
}

pub fn f49(p149: List<Adt37>, p150: List<Adt12>) -> R<Result<List<Adt12>, Adt93>> {
    let m148 = (p149, p150);
    {
        let mut m148 = m148;
        loop {
            let step = {
                let (v0, v1) = m148;
                let m146 = v0;
                match m146.uncons() {
                    None => {
                        Err::<(List<Adt37>, List<Adt12>), R<Result<List<Adt12>, Adt93>>>(Ok(Ok::<List<Adt12>, Adt93>(f79(v1, List::<Adt12>::nil()))))
                    }
                    Some((v2, v3)) => {
                        let m147 = f80(v2)?;
                        match m147 {
                            Ok(v5) => {
                                Ok::<(List<Adt37>, List<Adt12>), R<Result<List<Adt12>, Adt93>>>((v3, f79(v5, v1)))
                            }
                            Err(v4) => {
                                Err::<(List<Adt37>, List<Adt12>), R<Result<List<Adt12>, Adt93>>>(Ok(Err::<List<Adt12>, Adt93>(v4)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m148 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f50(v0: Str) -> Adt6 {
    Adt6::C3(v0)
}

pub fn f51(p155: List<Str>, p156: List<Adt6>) -> Adt6 {
    let m154 = (p155, p156);
    {
        let mut m154 = m154;
        loop {
            let step = {
                let (v0, v1) = m154;
                let m151 = v0;
                match m151.uncons() {
                    None => {
                        Err::<(List<Str>, List<Adt6>), Adt6>(Adt6::C4(v1))
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Str>, List<Adt6>), Adt6>((v3, {
                            let a152 = v1;
                            let a153 = List::cons(f50(v2), List::<Adt6>::nil());
                            append_list(a152, a153)
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m154 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f52(v0: bool) -> Adt6 {
    Adt6::C1(v0)
}

pub fn f53(v0: u64) -> R<Adt6> {
    Ok(Adt6::C2(Adt7::C0(f81(v0, 0i64)?, 0i64)))
}

pub fn f54(v0: Str) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), f50(Str::lit("string"))), f27(Str::lit("const"), f50(v0))))
}

pub fn f55() -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), f50(Str::lit("array"))), f27(Str::lit("items"), f28(Str::lit("string")))))
}

pub fn f56() -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), f50(Str::lit("array"))), f27(Str::lit("items"), f82())))
}

pub fn f57(v0: Adt37, v1: Adt23, v2: Fn3) -> R<Result<Adt34, Adt86>> {
    let m163 = f83(Adt6::C5({
        let m159 = match v0.clone() {
            Adt37::C0(_, _, _, _, h158, _, _, _) => {
                h158
            }
        };
        match m159 {
            Adt5::C0(h157) => {
                h157
            }
        }
    }), Adt6::C5({
        let m162 = match v0 {
            Adt37::C0(_, _, _, _, h161, _, _, _) => {
                h161
            }
        };
        match m162 {
            Adt5::C0(h160) => {
                h160
            }
        }
    }), f84(v1.clone()))?;
    match m163 {
        Adt99::C0 => {
            let c164: Fn3 = v2;
            c164.apply(v1)
        }
        Adt99::C1(v3, v4) => {
            Ok(Ok::<Adt34, Adt86>(Adt34::C2(f85(f86(v3, v4)?, true))))
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

pub fn f58(v0: Fn1, v1: Adt37, v2: Adt23) -> R<Result<Adt34, Adt86>> {
    f87(v1, v0, v2)
}

pub fn f59(v0: Adt23, v1: Str) -> bool {
    let a166 = match v0 {
        Adt23::C0(_, h165, _, _, _) => {
            h165
        }
    };
    let a167 = v1;
    equal(a166, a167)
}

pub fn f60(v0: Adt23, v1: Str) -> Option<Str> {
    let m168 = f88(v0, v1);
    match m168 {
        None => {
            None::<Str>
        }
        Some(v2) => {
            f89(v2)
        }
    }
}

pub fn f61(v0: Str) -> R<Result<Adt51, Adt86>> {
    let v1: Adt6 = f90(v0.clone())?;
    let m169 = f91(v1.clone())?;
    match m169 {
        Ok(v3) => {
            Ok(f92(f93({
                let a170 = List::cons(f94(v3), List::cons(Str::lit("\u{a}"), List::<Str>::nil()));
                let a171 = Str::lit("");
                join(a170, a171)
            }, v1, {
                let m172 = f95(v0);
                match m172 {
                    None => {
                        true
                    }
                    Some(_) => {
                        false
                    }
                }
            })))
        }
        Err(_) => {
            Ok(f62(Str::lit("the command result of explain cannot be written")))
        }
    }
}

pub fn f62(v0: Str) -> Result<Adt51, Adt86> {
    Err::<Adt51, Adt86>(Adt86::C9(v0))
}

pub fn f63(v0: Adt23) -> R<Option<Adt96>> {
    if f59(v0.clone(), Str::lit("lexlean_draft_write")) {
        let m173 = f60(v0.clone(), Str::lit("path"));
        match m173 {
            None => {
                Ok(None::<Adt96>)
            }
            Some(v1) => {
                let m174 = f60(v0, Str::lit("content"));
                match m174 {
                    None => {
                        Ok(None::<Adt96>)
                    }
                    Some(v2) => {
                        let a176 = f96();
                        let a177 = {
                            let a175 = f97(v2.clone())?;
                            length_list(a175)
                        };
                        if nat_lt(a176, a177) {
                            Ok(None::<Adt96>)
                        } else {
                            Ok(Some(Adt96::C0(Adt97::C1, {
                                let a178 = f98(v1)?;
                                let a179 = f99(f97(v2)?);
                                append_list(a178, a179)
                            })))
                        }
                    }
                }
            }
        }
    } else {
        if f59(v0.clone(), Str::lit("lexlean_draft_read")) {
            let m180 = f60(v0, Str::lit("path"));
            match m180 {
                None => {
                    Ok(None::<Adt96>)
                }
                Some(v3) => {
                    Ok(Some(Adt96::C0(Adt97::C0, f98(v3)?)))
                }
            }
        } else {
            if f59(v0.clone(), Str::lit("lexlean_check")) {
                f100(Adt97::C2, v0, List::<u64>::nil())
            } else {
                if f59(v0.clone(), Str::lit("lexlean_snapshot")) {
                    f100(Adt97::C3, v0, List::<u64>::nil())
                } else {
                    if f59(v0.clone(), Str::lit("lexlean_format")) {
                        f100(Adt97::C4, v0.clone(), f101(f102(v0, Str::lit("check_only"))))
                    } else {
                        if f59(v0.clone(), Str::lit("lexlean_build")) {
                            f100(Adt97::C5, v0, List::<u64>::nil())
                        } else {
                            if f59(v0.clone(), Str::lit("lexlean_verify")) {
                                f100(Adt97::C6, v0, List::<u64>::nil())
                            } else {
                                if f59(v0.clone(), Str::lit("lexlean_verify_control")) {
                                    let m181 = f60(v0.clone(), Str::lit("task"));
                                    match m181 {
                                        None => {
                                            Ok(None::<Adt96>)
                                        }
                                        Some(v4) => {
                                            let m182 = f60(v0, Str::lit("action"));
                                            match m182 {
                                                None => {
                                                    Ok(None::<Adt96>)
                                                }
                                                Some(v5) => {
                                                    Ok(Some(Adt96::C0(Adt97::C7, {
                                                        let a185 = f103(v4)?;
                                                        let a186 = f17({
                                                            let a183 = v5;
                                                            let a184 = Str::lit("cancel");
                                                            if equal(a183, a184) {
                                                                1u64
                                                            } else {
                                                                0u64
                                                            }
                                                        });
                                                        append_list(a185, a186)
                                                    })))
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    if f59(v0.clone(), Str::lit("lexlean_lexicon_read")) {
                                        let m187 = f60(v0.clone(), Str::lit("profile"));
                                        match m187 {
                                            None => {
                                                Ok(None::<Adt96>)
                                            }
                                            Some(v6) => {
                                                let m188 = f60(v0, Str::lit("entry"));
                                                match m188 {
                                                    None => {
                                                        Ok(None::<Adt96>)
                                                    }
                                                    Some(v7) => {
                                                        Ok(Some(Adt96::C0(Adt97::C8, {
                                                            let a189 = f103(v6)?;
                                                            let a190 = f103(v7)?;
                                                            append_list(a189, a190)
                                                        })))
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        if f59(v0.clone(), Str::lit("lexlean_attestation")) {
                                            let m191 = f60(v0.clone(), Str::lit("by"));
                                            match m191 {
                                                None => {
                                                    Ok(None::<Adt96>)
                                                }
                                                Some(v8) => {
                                                    let m192 = f60(v0, Str::lit("value"));
                                                    match m192 {
                                                        None => {
                                                            Ok(None::<Adt96>)
                                                        }
                                                        Some(v9) => {
                                                            Ok(Some(Adt96::C0(Adt97::C9, {
                                                                let a195 = f17({
                                                                    let a193 = v8;
                                                                    let a194 = Str::lit("digest");
                                                                    if equal(a193, a194) {
                                                                        1u64
                                                                    } else {
                                                                        0u64
                                                                    }
                                                                });
                                                                let a196 = f103(v9)?;
                                                                append_list(a195, a196)
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

pub fn f64(v0: u64, v1: List<u64>) -> Result<Adt51, Adt86> {
    Err::<Adt51, Adt86>(Adt86::C15(Adt87::C0(v0, f104(f105(), v1))))
}

pub fn f65(v0: Adt97) -> u64 {
    let m197 = v0;
    match m197 {
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

pub fn f66(v0: Adt97, v1: Bytes) -> R<Result<Adt51, Adt86>> {
    let a198 = f106(v1.clone(), 0u64)?;
    let a199 = 1u64;
    if nat_eq(a198, a199) {
        Ok(f92(f107(v0, f106(v1, 4u64)?)))
    } else {
        let m200 = v0.clone();
        match m200 {
            Adt97::C0 => {
                Ok(f92(f108(v1)?))
            }
            Adt97::C1 => {
                Ok(f92(f109(v1)?))
            }
            Adt97::C2 => {
                f110(v0, v1)
            }
            Adt97::C3 => {
                f110(v0, v1)
            }
            Adt97::C4 => {
                f110(v0, v1)
            }
            Adt97::C5 => {
                f110(v0, v1)
            }
            Adt97::C6 => {
                Ok(f92(f111(v1)?))
            }
            Adt97::C7 => {
                Ok(f92(f112(v1)?))
            }
            Adt97::C8 => {
                Ok(f92(f108(v1)?))
            }
            Adt97::C9 => {
                Ok(f92(f108(v1)?))
            }
        }
    }
}

pub fn f67(v0: Bytes, v1: u64) -> R<u64> {
    let m203 = {
        let a201 = v0;
        let a202 = v1;
        index_bytes(a201, a202)
    };
    match m203 {
        None => {
            Ok(0u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a248 = {
                let a224 = {
                    let a212 = {
                        let a206 = {
                            let a204 = v3;
                            let a205 = 128u8;
                            fixed_u8::bit_and(a204, a205)
                        };
                        let a207 = 128u8;
                        if equal(a206, a207) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a213 = {
                        let a210 = {
                            let a208 = v3;
                            let a209 = 64u8;
                            fixed_u8::bit_and(a208, a209)
                        };
                        let a211 = 64u8;
                        if equal(a210, a211) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a212, a213)?
                };
                let a225 = {
                    let a222 = {
                        let a216 = {
                            let a214 = v3;
                            let a215 = 32u8;
                            fixed_u8::bit_and(a214, a215)
                        };
                        let a217 = 32u8;
                        if equal(a216, a217) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a223 = {
                        let a220 = {
                            let a218 = v3;
                            let a219 = 16u8;
                            fixed_u8::bit_and(a218, a219)
                        };
                        let a221 = 16u8;
                        if equal(a220, a221) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a222, a223)?
                };
                nat_add(a224, a225)?
            };
            let a249 = {
                let a246 = {
                    let a234 = {
                        let a228 = {
                            let a226 = v3;
                            let a227 = 8u8;
                            fixed_u8::bit_and(a226, a227)
                        };
                        let a229 = 8u8;
                        if equal(a228, a229) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a235 = {
                        let a232 = {
                            let a230 = v3;
                            let a231 = 4u8;
                            fixed_u8::bit_and(a230, a231)
                        };
                        let a233 = 4u8;
                        if equal(a232, a233) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a234, a235)?
                };
                let a247 = {
                    let a244 = {
                        let a238 = {
                            let a236 = v3;
                            let a237 = 2u8;
                            fixed_u8::bit_and(a236, a237)
                        };
                        let a239 = 2u8;
                        if equal(a238, a239) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a245 = {
                        let a242 = {
                            let a240 = v3;
                            let a241 = 1u8;
                            fixed_u8::bit_and(a240, a241)
                        };
                        let a243 = 1u8;
                        if equal(a242, a243) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a244, a245)?
                };
                nat_add(a246, a247)?
            };
            nat_add(a248, a249)
        }
    }
}

pub fn f68(v0: Bytes, v1: Adt94, v2: Option<u64>) -> R<Result<Adt89, Adt90>> {
    let m250 = v2;
    match m250 {
        None => {
            Ok(Err::<Adt89, Adt90>(Adt90::C0(match v1 {
                Adt94::C0(_, h251) => {
                    h251
                }
            })))
        }
        Some(v3) => {
            let m255 = f113(v0.clone(), {
                let a253 = match v1.clone() {
                    Adt94::C0(_, h252) => {
                        h252
                    }
                };
                let a254 = 4u64;
                nat_add(a253, a254)?
            }, v3, List::<Adt15>::nil())?;
            match m255 {
                Ok(v5) => {
                    Ok(f114(v0, match v1 {
                        Adt94::C0(h256, _) => {
                            h256
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

pub fn f69(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a260 = {
        let a257 = v1;
        let a258 = 4u64;
        nat_add(a257, a258)?
    };
    let a261 = {
        let a259 = v0.clone();
        length_bytes(a259)
    };
    if nat_le(a260, a261) {
        Ok(Some({
            let a278 = {
                let a262 = f67(v0.clone(), v1)?;
                let a263 = 16777216u64;
                nat_mul(a262, a263)?
            };
            let a279 = {
                let a276 = {
                    let a266 = f67(v0.clone(), {
                        let a264 = v1;
                        let a265 = 1u64;
                        nat_add(a264, a265)?
                    })?;
                    let a267 = 65536u64;
                    nat_mul(a266, a267)?
                };
                let a277 = {
                    let a274 = {
                        let a270 = f67(v0.clone(), {
                            let a268 = v1;
                            let a269 = 2u64;
                            nat_add(a268, a269)?
                        })?;
                        let a271 = 256u64;
                        nat_mul(a270, a271)?
                    };
                    let a275 = f67(v0, {
                        let a272 = v1;
                        let a273 = 3u64;
                        nat_add(a272, a273)?
                    })?;
                    nat_add(a274, a275)?
                };
                nat_add(a276, a277)?
            };
            nat_add(a278, a279)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f70(v0: List<Adt92>) -> List<u64> {
    f115(f116(v0, List::<Adt92>::nil()), List::<u64>::nil())
}

pub fn f71(v0: List<List<u64>>) -> List<u64> {
    f117(f118(v0, List::<List<u64>>::nil()), List::<u64>::nil())
}

pub fn f72() -> u64 {
    500u64
}

pub fn f73() -> List<Adt92> {
    List::cons(Adt92::C0(f119(), f120()), List::<Adt92>::nil())
}

pub fn f74() -> List<u64> {
    let a282 = f121();
    let a283 = {
        let a280 = f122();
        let a281 = f123();
        append_list(a280, a281)
    };
    append_list(a282, a283)
}

pub fn f75(v0: Adt0, v1: Fn2, v2: Adt91, v3: Option<Adt98>) -> R<Adt88> {
    let m284 = v3;
    match m284 {
        None => {
            f125(v0, v1, v2)
        }
        Some(v4) => {
            Ok(Adt88::C0(f43(f124(v4)?)))
        }
    }
}

pub fn f76(v0: List<Adt92>, v1: List<List<u64>>) -> R<Option<Adt98>> {
    let m285 = f126(v0.clone(), v1.clone())?;
    match m285 {
        None => {
            f127(v0, v1)
        }
        Some(v2) => {
            Ok(Some(v2))
        }
    }
}

pub fn f77(v0: Bytes, v1: Adt94, v2: Adt94, v3: Option<u64>) -> R<Result<Adt91, Adt90>> {
    let m286 = v3;
    match m286 {
        None => {
            Ok(Err::<Adt91, Adt90>(Adt90::C0(match v2 {
                Adt94::C0(_, h287) => {
                    h287
                }
            })))
        }
        Some(v4) => {
            let m291 = f128(v0.clone(), {
                let a289 = match v2.clone() {
                    Adt94::C0(_, h288) => {
                        h288
                    }
                };
                let a290 = 4u64;
                nat_add(a289, a290)?
            }, v4, List::<Adt92>::nil())?;
            match m291 {
                Ok(v6) => {
                    f129(v0, v1, v2, v6)
                }
                Err(v5) => {
                    Ok(Err::<Adt91, Adt90>(v5))
                }
            }
        }
    }
}

pub fn f78(p297: List<Adt37>, p298: List<Str>) -> Option<Str> {
    let m296 = (p297, p298);
    {
        let mut m296 = m296;
        loop {
            let step = {
                let (v0, v1) = m296;
                let m292 = v0;
                match m292.uncons() {
                    None => {
                        Err::<(List<Adt37>, List<Str>), Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        if f130(match v2.clone() {
                            Adt37::C0(h293, _, _, _, _, _, _, _) => {
                                h293
                            }
                        }, v1.clone()) {
                            Err::<(List<Adt37>, List<Str>), Option<Str>>(Some(match v2 {
                                Adt37::C0(h294, _, _, _, _, _, _, _) => {
                                    h294
                                }
                            }))
                        } else {
                            Ok::<(List<Adt37>, List<Str>), Option<Str>>((v3, List::cons(match v2 {
                                Adt37::C0(h295, _, _, _, _, _, _, _) => {
                                    h295
                                }
                            }, v1)))
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

pub fn f79(p301: List<Adt12>, p302: List<Adt12>) -> List<Adt12> {
    let m300 = (p301, p302);
    {
        let mut m300 = m300;
        loop {
            let step = {
                let (v0, v1) = m300;
                let m299 = v0;
                match m299.uncons() {
                    None => {
                        Err::<(List<Adt12>, List<Adt12>), List<Adt12>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt12>, List<Adt12>), List<Adt12>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m300 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f80(v0: Adt37) -> R<Result<List<Adt12>, Adt93>> {
    let m305 = f131(match v0.clone() {
        Adt37::C0(h303, _, _, _, _, _, _, _) => {
            h303
        }
    }, f132(match v0.clone() {
        Adt37::C0(h304, _, _, _, _, _, _, _) => {
            h304
        }
    })?);
    match m305 {
        None => {
            f133(v0)
        }
        Some(v1) => {
            Ok(Err::<List<Adt12>, Adt93>(v1))
        }
    }
}

pub fn f81(p310: u64, p311: i64) -> R<i64> {
    let m309 = (p310, p311);
    {
        let mut m309 = m309;
        loop {
            let step = {
                let (v0, v1) = m309;
                let m306 = v0;
                if m306 == 0 {
                    Err::<(u64, i64), R<i64>>(Ok(v1))
                } else {
                    let v2 = m306 - 1;
                    Ok::<(u64, i64), R<i64>>((v2, {
                        let a307 = v1;
                        let a308 = -1i64;
                        int_sub(a307, a308)?
                    }))
                }
            };
            match step {
                Ok(next) => { m309 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f82() -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), f50(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(List::cons((Str::lit("code"), f28(Str::lit("string"))), f27(Str::lit("message"), f28(Str::lit("string")))))), f27(Str::lit("required"), f51(List::cons(Str::lit("code"), List::cons(Str::lit("message"), List::<Str>::nil())), List::<Adt6>::nil())))))
}

pub fn f83(v0: Adt6, v1: Adt6, v2: Adt6) -> R<Adt99> {
    f134(v0, f135(), Adt101::C0, List::cons(Adt102::C0(v1, v2, List::<Adt100>::nil()), List::<Adt102>::nil()))
}

pub fn f84(v0: Adt23) -> Adt6 {
    let m313 = match v0 {
        Adt23::C0(_, _, h312, _, _) => {
            h312
        }
    };
    match m313 {
        None => {
            Adt6::C5(List::<(Str, Adt6)>::nil())
        }
        Some(v1) => {
            Adt6::C5(match v1 {
                Adt5::C0(h314) => {
                    h314
                }
            })
        }
    }
}

pub fn f85(v0: Str, v1: bool) -> Adt39 {
    Adt39::C0(List::cons(Adt40::C0(Adt41::C0(v0, None::<Adt42>, None::<Adt5>)), List::<Adt40>::nil()), None::<Adt6>, if v1 {
        Some(true)
    } else {
        None::<bool>
    })
}

pub fn f86(v0: List<Adt100>, v1: Str) -> R<Str> {
    let v2: Str = f136(v0)?;
    let a317 = List::cons(Str::lit("Invalid arguments:"), List::cons({
        let a315 = v2.clone();
        let a316 = Str::lit("");
        if equal(a315, a316) {
            Str::lit("the arguments object")
        } else {
            v2
        }
    }, List::cons(Str::lit("fails"), List::cons(v1, List::<Str>::nil()))));
    let a318 = Str::lit(" ");
    Ok(join(a317, a318))
}

pub fn f87(v0: Adt37, v1: Fn1, v2: Adt23) -> R<Result<Adt34, Adt86>> {
    let m320 = {
        let c319: Fn1 = v1;
        c319.apply(v2)?
    };
    match m320 {
        Ok(v4) => {
            f137(v0, v4)
        }
        Err(v3) => {
            Ok(Err::<Adt34, Adt86>(v3))
        }
    }
}

pub fn f88(v0: Adt23, v1: Str) -> Option<Adt6> {
    let m322 = match v0 {
        Adt23::C0(_, _, h321, _, _) => {
            h321
        }
    };
    match m322 {
        None => {
            None::<Adt6>
        }
        Some(v2) => {
            let m323 = f138(v1, v2);
            match m323 {
                Adt108::C0 => {
                    None::<Adt6>
                }
                Adt108::C1(v3) => {
                    Some(v3)
                }
            }
        }
    }
}

pub fn f89(v0: Adt6) -> Option<Str> {
    let m324 = v0;
    match m324 {
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

pub fn f90(v0: Str) -> R<Adt6> {
    let m325 = f95(v0.clone());
    match m325 {
        None => {
            Ok(Adt6::C5(List::cons((Str::lit("artifacts"), f139()), List::cons((Str::lit("command"), f50(Str::lit("explain"))), List::cons((Str::lit("diagnostics"), Adt6::C4(List::cons(f140(v0), List::<Adt6>::nil()))), List::cons((Str::lit("exit_code"), f53(2u64)?), List::cons((Str::lit("modules"), f139()), List::cons((Str::lit("spec"), f50(Str::lit("lexlean/command-result/1"))), f27(Str::lit("success"), f52(false))))))))))
        }
        Some(v1) => {
            Ok(Adt6::C5(List::cons((Str::lit("artifacts"), f139()), List::cons((Str::lit("command"), f50(Str::lit("explain"))), List::cons((Str::lit("diagnostics"), f139()), List::cons((Str::lit("exit_code"), f53(0u64)?), List::cons((Str::lit("explanation"), f50(v1)), List::cons((Str::lit("modules"), f139()), List::cons((Str::lit("spec"), f50(Str::lit("lexlean/command-result/1"))), f27(Str::lit("success"), f52(true)))))))))))
        }
    }
}

pub fn f91(v0: Adt6) -> R<Result<Bytes, Adt104>> {
    let m326 = f141(v0)?;
    match m326 {
        Ok(v1) => {
            Ok(Ok::<Bytes, Adt104>(f104(f105(), v1)))
        }
        Err(v2) => {
            Ok(Err::<Bytes, Adt104>(v2))
        }
    }
}

pub fn f92(v0: Adt51) -> Result<Adt51, Adt86> {
    Ok::<Adt51, Adt86>(v0)
}

pub fn f93(v0: Str, v1: Adt6, v2: bool) -> Adt51 {
    Adt51::C0(Adt39::C0(List::cons(f142(v0), List::<Adt40>::nil()), Some(v1), if v2 {
        Some(true)
    } else {
        None::<bool>
    }), List::<Adt52>::nil())
}

pub fn f94(v0: Bytes) -> Str {
    let m328 = {
        let a327 = v0;
        utf8_decode(a327)
    };
    match m328 {
        None => {
            Str::lit("the content is not UTF-8")
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f95(v0: Str) -> Option<Str> {
    let m329 = f143(v0.clone(), f144());
    match m329 {
        None => {
            let m330 = f143(v0.clone(), f145());
            match m330 {
                None => {
                    let m331 = f143(v0.clone(), f146());
                    match m331 {
                        None => {
                            let m332 = f143(v0.clone(), f147());
                            match m332 {
                                None => {
                                    let m333 = f143(v0.clone(), f148());
                                    match m333 {
                                        None => {
                                            let m334 = f143(v0.clone(), f149());
                                            match m334 {
                                                None => {
                                                    let m335 = f143(v0.clone(), f150());
                                                    match m335 {
                                                        None => {
                                                            let m336 = f143(v0.clone(), f151());
                                                            match m336 {
                                                                None => {
                                                                    let m337 = f143(v0.clone(), f152());
                                                                    match m337 {
                                                                        None => {
                                                                            let m338 = f143(v0.clone(), f153());
                                                                            match m338 {
                                                                                None => {
                                                                                    let m339 = f143(v0, f154());
                                                                                    match m339 {
                                                                                        None => {
                                                                                            None::<Str>
                                                                                        }
                                                                                        Some(v11) => {
                                                                                            Some(v11)
                                                                                        }
                                                                                    }
                                                                                }
                                                                                Some(v10) => {
                                                                                    Some(v10)
                                                                                }
                                                                            }
                                                                        }
                                                                        Some(v9) => {
                                                                            Some(v9)
                                                                        }
                                                                    }
                                                                }
                                                                Some(v8) => {
                                                                    Some(v8)
                                                                }
                                                            }
                                                        }
                                                        Some(v7) => {
                                                            Some(v7)
                                                        }
                                                    }
                                                }
                                                Some(v6) => {
                                                    Some(v6)
                                                }
                                            }
                                        }
                                        Some(v5) => {
                                            Some(v5)
                                        }
                                    }
                                }
                                Some(v4) => {
                                    Some(v4)
                                }
                            }
                        }
                        Some(v3) => {
                            Some(v3)
                        }
                    }
                }
                Some(v2) => {
                    Some(v2)
                }
            }
        }
        Some(v1) => {
            Some(v1)
        }
    }
}

pub fn f96() -> u64 {
    8388608u64
}

pub fn f97(v0: Str) -> R<List<u64>> {
    f18({
        let a340 = v0;
        utf8_encode(a340)
    })
}

pub fn f98(v0: Str) -> R<List<u64>> {
    let a341 = f17(f155(Str::lit("drafts")));
    let a342 = f103(v0)?;
    Ok(append_list(a341, a342))
}

pub fn f99(v0: List<u64>) -> List<u64> {
    let a344 = f17({
        let a343 = v0.clone();
        length_list(a343)
    });
    let a345 = v0;
    append_list(a344, a345)
}

pub fn f100(v0: Adt97, v1: Adt23, v2: List<u64>) -> R<Option<Adt96>> {
    let m346 = f60(v1.clone(), Str::lit("project"));
    match m346 {
        None => {
            Ok(None::<Adt96>)
        }
        Some(v3) => {
            let m347 = f156(v1.clone());
            match m347 {
                None => {
                    Ok(None::<Adt96>)
                }
                Some(v4) => {
                    if f157(v4.clone())? {
                        Ok(None::<Adt96>)
                    } else {
                        if f158(f102(v1.clone(), Str::lit("all")), v4.clone()) {
                            Ok(None::<Adt96>)
                        } else {
                            Ok(Some(Adt96::C0(v0, {
                                let a350 = f103(v3)?;
                                let a351 = {
                                    let a348 = f159(f102(v1, Str::lit("all")), v4)?;
                                    let a349 = v2;
                                    append_list(a348, a349)
                                };
                                append_list(a350, a351)
                            })))
                        }
                    }
                }
            }
        }
    }
}

pub fn f101(v0: bool) -> List<u64> {
    if v0 {
        List::cons(1u64, List::<u64>::nil())
    } else {
        List::cons(0u64, List::<u64>::nil())
    }
}

pub fn f102(v0: Adt23, v1: Str) -> bool {
    let m352 = f88(v0, v1);
    match m352 {
        None => {
            false
        }
        Some(v2) => {
            let m353 = f160(v2);
            match m353 {
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

pub fn f103(v0: Str) -> R<List<u64>> {
    Ok(f99(f97(v0)?))
}

pub fn f104(v0: Bytes, v1: List<u64>) -> Bytes {
    let a354 = v0;
    let a355 = f161(f105(), f162(List::<Bytes>::nil(), v1));
    append_bytes(a354, a355)
}

pub fn f105() -> Bytes {
    Bytes::lit(&[])
}

pub fn f106(v0: Bytes, v1: u64) -> R<u64> {
    let m356 = f69(v0, v1)?;
    match m356 {
        None => {
            Ok(0u64)
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f107(v0: Adt97, v1: u64) -> Adt51 {
    let v2: Str = f163(f164(v0.clone()), v1);
    f93({
        let a357 = List::cons(f165(v0), List::cons(Str::lit("failed:"), List::cons(v2.clone(), List::<Str>::nil())));
        let a358 = Str::lit(" ");
        join(a357, a358)
    }, Adt6::C5(f27(Str::lit("failure"), f50(v2))), true)
}

pub fn f108(v0: Bytes) -> R<Adt51> {
    let m359 = f42(v0.clone(), 5u64)?;
    match m359 {
        None => {
            Ok(f166())
        }
        Some(v1) => {
            let a360 = f67(v0, 4u64)?;
            let a361 = 1u64;
            if nat_eq(a360, a361) {
                Ok(f93(f94(match v1 {
                    Adt94::C0(h362, _) => {
                        h362
                    }
                }), Adt6::C5(f27(Str::lit("found"), f52(true))), false))
            } else {
                Ok(f93(Str::lit("not found"), Adt6::C5(f27(Str::lit("found"), f52(false))), false))
            }
        }
    }
}

pub fn f109(v0: Bytes) -> R<Adt51> {
    let v1: u64 = {
        let a365 = {
            let a363 = f106(v0.clone(), 4u64)?;
            let a364 = 4294967296u64;
            nat_mul(a363, a364)?
        };
        let a366 = f106(v0, 8u64)?;
        nat_add(a365, a366)?
    };
    Ok(f93({
        let a368 = List::cons(Str::lit("written"), List::cons({
            let a367 = f167(v1, 0i64)?;
            format_i64(a367)
        }, List::cons(Str::lit("bytes"), List::<Str>::nil())));
        let a369 = Str::lit(" ");
        join(a368, a369)
    }, Adt6::C5(f27(Str::lit("length"), f53(v1)?)), false))
}

pub fn f110(v0: Adt97, v1: Bytes) -> R<Result<Adt51, Adt86>> {
    let m370 = f42(v1.clone(), 8u64)?;
    match m370 {
        None => {
            Ok(f92(f166()))
        }
        Some(v2) => {
            let m372 = f168(match v2.clone() {
                Adt94::C0(h371, _) => {
                    h371
                }
            })?;
            match m372 {
                Ok(v4) => {
                    let v5: u64 = f106(v1, 4u64)?;
                    let a373 = f165(v0);
                    let a374 = Str::lit("engine.snapshot");
                    if if equal(a373, a374) {
                        let a375 = v5;
                        let a376 = 0u64;
                        if nat_eq(a375, a376) {
                            f169(v4.clone())
                        } else {
                            f170(v4.clone(), v5)?
                        }
                    } else {
                        f170(v4.clone(), v5)?
                    } {
                        Ok(f92(f93(f94(match v2 {
                            Adt94::C0(h377, _) => {
                                h377
                            }
                        }), v4, {
                            let a378 = v5;
                            let a379 = 0u64;
                            !nat_eq(a378, a379)
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

pub fn f111(v0: Bytes) -> R<Adt51> {
    let m380 = f42(v0.clone(), 4u64)?;
    match m380 {
        None => {
            Ok(f166())
        }
        Some(v1) => {
            let m383 = {
                let a382 = match v1.clone() {
                    Adt94::C0(h381, _) => {
                        h381
                    }
                };
                utf8_decode(a382)
            };
            match m383 {
                None => {
                    Ok(f166())
                }
                Some(v2) => {
                    Ok(f93(v2.clone(), Adt6::C5(List::cons((Str::lit("task"), f50(v2)), f27(Str::lit("state"), f50(f171(f106(v0, match v1 {
                        Adt94::C0(_, h384) => {
                            h384
                        }
                    })?))))), false))
                }
            }
        }
    }
}

pub fn f112(v0: Bytes) -> R<Adt51> {
    let m385 = f42(v0.clone(), 12u64)?;
    match m385 {
        None => {
            Ok(f166())
        }
        Some(v1) => {
            Ok(f93(f94(match v1.clone() {
                Adt94::C0(h386, _) => {
                    h386
                }
            }), Adt6::C5(List::cons((Str::lit("state"), f50(f171(f106(v0.clone(), 4u64)?))), List::cons((Str::lit("class"), f53(f106(v0.clone(), 8u64)?)?), List::cons((Str::lit("groupsEnded"), f53(f106(v0.clone(), match v1.clone() {
                Adt94::C0(_, h387) => {
                    h387
                }
            })?)?), f27(Str::lit("groupsFailed"), f53(f106(v0.clone(), {
                let a389 = match v1 {
                    Adt94::C0(_, h388) => {
                        h388
                    }
                };
                let a390 = 4u64;
                nat_add(a389, a390)?
            })?)?))))), {
                let a391 = f106(v0, 8u64)?;
                let a392 = 0u64;
                !nat_eq(a391, a392)
            }))
        }
    }
}

pub fn f113(v0: Bytes, p403: u64, p404: u64, p405: List<Adt15>) -> R<Result<(List<Adt15>, u64), Adt90>> {
    let m402 = (p403, (p404, p405));
    {
        let mut m402 = m402;
        loop {
            let step = {
                let (v1, (v2, v3)) = m402;
                let m393 = v2;
                if m393 == 0 {
                    Err::<(u64, (u64, List<Adt15>)), R<Result<(List<Adt15>, u64), Adt90>>>(Ok(Ok::<(List<Adt15>, u64), Adt90>((f172(v3, List::<Adt15>::nil()), v1))))
                } else {
                    let v4 = m393 - 1;
                    let m394 = f69(v0.clone(), v1)?;
                    match m394 {
                        None => {
                            Err::<(u64, (u64, List<Adt15>)), R<Result<(List<Adt15>, u64), Adt90>>>(Ok(Err::<(List<Adt15>, u64), Adt90>(Adt90::C0(v1))))
                        }
                        Some(v5) => {
                            let m397 = f42(v0.clone(), {
                                let a395 = v1;
                                let a396 = 4u64;
                                nat_add(a395, a396)?
                            })?;
                            match m397 {
                                None => {
                                    Err::<(u64, (u64, List<Adt15>)), R<Result<(List<Adt15>, u64), Adt90>>>(Ok(Err::<(List<Adt15>, u64), Adt90>(Adt90::C0({
                                        let a398 = v1;
                                        let a399 = 4u64;
                                        nat_add(a398, a399)?
                                    }))))
                                }
                                Some(v6) => {
                                    Ok::<(u64, (u64, List<Adt15>)), R<Result<(List<Adt15>, u64), Adt90>>>((match v6.clone() {
                                        Adt94::C0(_, h400) => {
                                            h400
                                        }
                                    }, (v4, List::cons(Adt15::C0(v5, match v6 {
                                        Adt94::C0(h401, _) => {
                                            h401
                                        }
                                    }), v3))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m402 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f114(v0: Bytes, v1: Bytes, v2: (List<Adt15>, u64)) -> Result<Adt89, Adt90> {
    f173({
        let a408 = {
            let (_, h406) = v2.clone();
            h406
        };
        let a409 = {
            let a407 = v0;
            length_bytes(a407)
        };
        nat_eq(a408, a409)
    }, v1, {
        let (h410, _) = v2;
        h410
    }, {
        let (_, h411) = v2;
        h411
    })
}

pub fn f115(p416: List<Adt92>, p417: List<u64>) -> List<u64> {
    let m415 = (p416, p417);
    {
        let mut m415 = m415;
        loop {
            let step = {
                let (v0, v1) = m415;
                let m412 = v0;
                match m412.uncons() {
                    None => {
                        Err::<(List<Adt92>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt92>, List<u64>), List<u64>>((v3, {
                            let a413 = f174(v2);
                            let a414 = v1;
                            append_list(a413, a414)
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m415 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f116(p420: List<Adt92>, p421: List<Adt92>) -> List<Adt92> {
    let m419 = (p420, p421);
    {
        let mut m419 = m419;
        loop {
            let step = {
                let (v0, v1) = m419;
                let m418 = v0;
                match m418.uncons() {
                    None => {
                        Err::<(List<Adt92>, List<Adt92>), List<Adt92>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt92>, List<Adt92>), List<Adt92>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m419 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f117(p426: List<List<u64>>, p427: List<u64>) -> List<u64> {
    let m425 = (p426, p427);
    {
        let mut m425 = m425;
        loop {
            let step = {
                let (v0, v1) = m425;
                let m422 = v0;
                match m422.uncons() {
                    None => {
                        Err::<(List<List<u64>>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<u64>>, List<u64>), List<u64>>((v3, {
                            let a423 = f175(v2);
                            let a424 = v1;
                            append_list(a423, a424)
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m425 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f118(p430: List<List<u64>>, p431: List<List<u64>>) -> List<List<u64>> {
    let m429 = (p430, p431);
    {
        let mut m429 = m429;
        loop {
            let step = {
                let (v0, v1) = m429;
                let m428 = v0;
                match m428.uncons() {
                    None => {
                        Err::<(List<List<u64>>, List<List<u64>>), List<List<u64>>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<u64>>, List<List<u64>>), List<List<u64>>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m429 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f119() -> List<u64> {
    List::cons(67u64, List::cons(111u64, List::cons(110u64, List::cons(116u64, List::cons(101u64, List::cons(110u64, List::cons(116u64, List::cons(45u64, List::cons(84u64, List::cons(121u64, List::cons(112u64, List::cons(101u64, List::<u64>::nil()))))))))))))
}

pub fn f120() -> List<u64> {
    List::cons(97u64, List::cons(112u64, List::cons(112u64, List::cons(108u64, List::cons(105u64, List::cons(99u64, List::cons(97u64, List::cons(116u64, List::cons(105u64, List::cons(111u64, List::cons(110u64, List::cons(47u64, List::cons(106u64, List::cons(115u64, List::cons(111u64, List::cons(110u64, List::<u64>::nil()))))))))))))))))
}

pub fn f121() -> List<u64> {
    List::cons(123u64, List::cons(34u64, List::cons(106u64, List::cons(115u64, List::cons(111u64, List::cons(110u64, List::cons(114u64, List::cons(112u64, List::cons(99u64, List::cons(34u64, List::cons(58u64, List::cons(34u64, List::cons(50u64, List::cons(46u64, List::cons(48u64, List::cons(34u64, List::<u64>::nil()))))))))))))))))
}

pub fn f122() -> List<u64> {
    List::cons(44u64, List::cons(34u64, List::cons(101u64, List::cons(114u64, List::cons(114u64, List::cons(111u64, List::cons(114u64, List::cons(34u64, List::cons(58u64, List::cons(123u64, List::cons(34u64, List::cons(99u64, List::cons(111u64, List::cons(100u64, List::cons(101u64, List::cons(34u64, List::cons(58u64, List::cons(45u64, List::cons(51u64, List::cons(50u64, List::cons(54u64, List::cons(48u64, List::cons(51u64, List::cons(44u64, List::cons(34u64, List::cons(109u64, List::<u64>::nil()))))))))))))))))))))))))))
}

pub fn f123() -> List<u64> {
    List::cons(101u64, List::cons(115u64, List::cons(115u64, List::cons(97u64, List::cons(103u64, List::cons(101u64, List::cons(34u64, List::cons(58u64, List::cons(34u64, List::cons(73u64, List::cons(110u64, List::cons(116u64, List::cons(101u64, List::cons(114u64, List::cons(110u64, List::cons(97u64, List::cons(108u64, List::cons(32u64, List::cons(101u64, List::cons(114u64, List::cons(114u64, List::cons(111u64, List::cons(114u64, List::cons(34u64, List::cons(125u64, List::cons(125u64, List::<u64>::nil()))))))))))))))))))))))))))
}

pub fn f124(v0: Adt98) -> R<Adt95> {
    f176(f177(v0.clone()), f178(None::<Adt105>, f179(v0)))
}

pub fn f125(v0: Adt0, v1: Fn2, v2: Adt91) -> R<Adt88> {
    if f180(match v2.clone() {
        Adt91::C0(h432, _, _, _, _, _, _) => {
            h432
        }
    }) {
        f181(f182(v0, match v2.clone() {
            Adt91::C0(_, _, _, _, _, _, h433) => {
                h433
            }
        }), v1, match v2.clone() {
            Adt91::C0(_, _, _, _, h434, _, _) => {
                h434
            }
        }, match v2.clone() {
            Adt91::C0(_, _, h435, _, _, _, _) => {
                h435
            }
        }, f168(match v2 {
            Adt91::C0(_, _, _, h436, _, _, _) => {
                h436
            }
        })?)
    } else {
        Ok(Adt88::C0(f43(f183())))
    }
}

pub fn f126(v0: List<Adt92>, v1: List<List<u64>>) -> R<Option<Adt98>> {
    let m437 = f184(v0, f185())?;
    match m437 {
        Adt111::C0 => {
            Ok(None::<Adt98>)
        }
        Adt111::C1 => {
            Ok(Some(Adt98::C0))
        }
        Adt111::C2 => {
            Ok(Some(Adt98::C0))
        }
        Adt111::C3(v2) => {
            f186(f187(v2), v1, Adt98::C0)
        }
    }
}

pub fn f127(v0: List<Adt92>, v1: List<List<u64>>) -> R<Option<Adt98>> {
    let m438 = f184(v0, f188())?;
    match m438 {
        Adt111::C0 => {
            Ok(None::<Adt98>)
        }
        Adt111::C1 => {
            Ok(Some(Adt98::C1))
        }
        Adt111::C2 => {
            Ok(Some(Adt98::C1))
        }
        Adt111::C3(v2) => {
            f186(f189(v2), v1, Adt98::C1)
        }
    }
}

pub fn f128(v0: Bytes, p448: u64, p449: u64, p450: List<Adt92>) -> R<Result<(List<Adt92>, u64), Adt90>> {
    let m447 = (p448, (p449, p450));
    {
        let mut m447 = m447;
        loop {
            let step = {
                let (v1, (v2, v3)) = m447;
                let m439 = v2;
                if m439 == 0 {
                    Err::<(u64, (u64, List<Adt92>)), R<Result<(List<Adt92>, u64), Adt90>>>(Ok(Ok::<(List<Adt92>, u64), Adt90>((f190(v3, List::<Adt92>::nil()), v1))))
                } else {
                    let v4 = m439 - 1;
                    let m440 = f42(v0.clone(), v1)?;
                    match m440 {
                        None => {
                            Err::<(u64, (u64, List<Adt92>)), R<Result<(List<Adt92>, u64), Adt90>>>(Ok(Err::<(List<Adt92>, u64), Adt90>(Adt90::C0(v1))))
                        }
                        Some(v5) => {
                            let m442 = f42(v0.clone(), match v5.clone() {
                                Adt94::C0(_, h441) => {
                                    h441
                                }
                            })?;
                            match m442 {
                                None => {
                                    Err::<(u64, (u64, List<Adt92>)), R<Result<(List<Adt92>, u64), Adt90>>>(Ok(Err::<(List<Adt92>, u64), Adt90>(Adt90::C0(match v5 {
                                        Adt94::C0(_, h443) => {
                                            h443
                                        }
                                    }))))
                                }
                                Some(v6) => {
                                    Ok::<(u64, (u64, List<Adt92>)), R<Result<(List<Adt92>, u64), Adt90>>>((match v6.clone() {
                                        Adt94::C0(_, h444) => {
                                            h444
                                        }
                                    }, (v4, List::cons(Adt92::C0(f18(match v5 {
                                        Adt94::C0(h445, _) => {
                                            h445
                                        }
                                    })?, f18(match v6 {
                                        Adt94::C0(h446, _) => {
                                            h446
                                        }
                                    })?), v3))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m447 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f129(v0: Bytes, v1: Adt94, v2: Adt94, v3: (List<Adt92>, u64)) -> R<Result<Adt91, Adt90>> {
    let m452 = f42(v0.clone(), {
        let (_, h451) = v3.clone();
        h451
    })?;
    match m452 {
        None => {
            Ok(Err::<Adt91, Adt90>(Adt90::C0({
                let (_, h453) = v3;
                h453
            })))
        }
        Some(v4) => {
            let m455 = f69(v0.clone(), match v4.clone() {
                Adt94::C0(_, h454) => {
                    h454
                }
            })?;
            match m455 {
                None => {
                    Ok(Err::<Adt91, Adt90>(Adt90::C0(match v4 {
                        Adt94::C0(_, h456) => {
                            h456
                        }
                    })))
                }
                Some(v5) => {
                    let m460 = f191(v0.clone(), {
                        let a458 = match v4.clone() {
                            Adt94::C0(_, h457) => {
                                h457
                            }
                        };
                        let a459 = 4u64;
                        nat_add(a458, a459)?
                    })?;
                    match m460 {
                        None => {
                            Ok(Err::<Adt91, Adt90>(Adt90::C0({
                                let a462 = match v4 {
                                    Adt94::C0(_, h461) => {
                                        h461
                                    }
                                };
                                let a463 = 4u64;
                                nat_add(a462, a463)?
                            })))
                        }
                        Some(v6) => {
                            f192(v0, Adt91::C0(f18(match v1 {
                                Adt94::C0(h464, _) => {
                                    h464
                                }
                            })?, f18(match v2 {
                                Adt94::C0(h465, _) => {
                                    h465
                                }
                            })?, {
                                let (h466, _) = v3;
                                h466
                            }, match v4 {
                                Adt94::C0(h467, _) => {
                                    h467
                                }
                            }, v5, List::<List<u64>>::nil(), match v6.clone() {
                                Adt94::C0(h468, _) => {
                                    h468
                                }
                            }), match v6 {
                                Adt94::C0(_, h469) => {
                                    h469
                                }
                            })
                        }
                    }
                }
            }
        }
    }
}

pub fn f130(v0: Str, p474: List<Str>) -> bool {
    let m473 = p474;
    {
        let mut m473 = m473;
        loop {
            let step = {
                let v1 = m473;
                let m470 = v1;
                match m470.uncons() {
                    None => {
                        Err::<List<Str>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a471 = v0.clone();
                        let a472 = v2;
                        if equal(a471, a472) {
                            Err::<List<Str>, bool>(true)
                        } else {
                            Ok::<List<Str>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m473 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f131(v0: Str, v1: List<u64>) -> Option<Adt93> {
    if f193(v1) {
        Some(Adt93::C0(v0))
    } else {
        None::<Adt93>
    }
}

pub fn f132(v0: Str) -> R<List<u64>> {
    f18({
        let a475 = v0;
        utf8_encode(a475)
    })
}

pub fn f133(v0: Adt37) -> R<Result<List<Adt12>, Adt93>> {
    let m476 = f194(v0.clone());
    match m476 {
        None => {
            f195(v0.clone(), f196(f197(), match v0.clone() {
                Adt37::C0(h477, _, _, _, _, _, _, _) => {
                    h477
                }
            }, Adt6::C5({
                let m480 = match v0.clone() {
                    Adt37::C0(_, _, _, _, h479, _, _, _) => {
                        h479
                    }
                };
                match m480 {
                    Adt5::C0(h478) => {
                        h478
                    }
                }
            }), List::cons(Adt106::C0(Adt6::C5({
                let m483 = match v0 {
                    Adt37::C0(_, _, _, _, h482, _, _, _) => {
                        h482
                    }
                };
                match m483 {
                    Adt5::C0(h481) => {
                        h481
                    }
                }
            }), true, List::<Str>::nil()), List::<Adt106>::nil()), f198())?)
        }
        Some(v1) => {
            Ok(Err::<List<Adt12>, Adt93>(v1))
        }
    }
}

pub fn f134(v0: Adt6, p502: u64, p503: Adt101, p504: List<Adt102>) -> R<Adt99> {
    let m501 = (p502, (p503, p504));
    {
        let mut m501 = m501;
        loop {
            let step = {
                let (v1, (v2, v3)) = m501;
                let m484 = v1;
                if m484 == 0 {
                    Err::<(u64, (Adt101, List<Adt102>)), R<Adt99>>(Ok(Adt99::C4))
                } else {
                    let v4 = m484 - 1;
                    let m485 = v3;
                    match m485.uncons() {
                        None => {
                            let m486 = v2;
                            match m486 {
                                Adt101::C0 => {
                                    Err::<(u64, (Adt101, List<Adt102>)), R<Adt99>>(Ok(Adt99::C0))
                                }
                                Adt101::C1(v5, v6) => {
                                    Err::<(u64, (Adt101, List<Adt102>)), R<Adt99>>(Ok(Adt99::C1(v5, v6)))
                                }
                            }
                        }
                        Some((v7, v8)) => {
                            let m487 = v2.clone();
                            match m487 {
                                Adt101::C0 => {
                                    let m488 = v7.clone();
                                    match m488 {
                                        Adt102::C0(v9, v10, v11) => {
                                            let m489 = f199(v0.clone(), v9, v10, v11.clone())?;
                                            match m489 {
                                                Adt114::C0(v12) => {
                                                    Ok::<(u64, (Adt101, List<Adt102>)), R<Adt99>>((v4, (Adt101::C1(v11, v12), v8)))
                                                }
                                                Adt114::C1(v13) => {
                                                    Err::<(u64, (Adt101, List<Adt102>)), R<Adt99>>(Ok(Adt99::C2(v13)))
                                                }
                                                Adt114::C2(v14) => {
                                                    Err::<(u64, (Adt101, List<Adt102>)), R<Adt99>>(Ok(Adt99::C3(v14)))
                                                }
                                                Adt114::C3(v15) => {
                                                    let v16: List<Adt102> = {
                                                        let a490 = v15;
                                                        let a491 = v8;
                                                        append_list(a490, a491)
                                                    };
                                                    Ok::<(u64, (Adt101, List<Adt102>)), R<Adt99>>((v4, (v2, v16)))
                                                }
                                            }
                                        }
                                        Adt102::C1(_, _, _) => {
                                            Ok::<(u64, (Adt101, List<Adt102>)), R<Adt99>>((v4, (v2, v8)))
                                        }
                                        Adt102::C2(_, _, _, _) => {
                                            let v24: Adt107 = f200(v7, v8)?;
                                            Ok::<(u64, (Adt101, List<Adt102>)), R<Adt99>>((v4, (match v24.clone() {
                                                Adt107::C0(h492, _) => {
                                                    h492
                                                }
                                            }, match v24 {
                                                Adt107::C0(_, h493) => {
                                                    h493
                                                }
                                            })))
                                        }
                                        Adt102::C3(v25) => {
                                            Ok::<(u64, (Adt101, List<Adt102>)), R<Adt99>>((v4, (Adt101::C1(v25, Str::lit("not")), v8)))
                                        }
                                        Adt102::C4(_, _, _, _) => {
                                            let v39: Adt107 = f200(v7, v8)?;
                                            Ok::<(u64, (Adt101, List<Adt102>)), R<Adt99>>((v4, (match v39.clone() {
                                                Adt107::C0(h497, _) => {
                                                    h497
                                                }
                                            }, match v39 {
                                                Adt107::C0(_, h498) => {
                                                    h498
                                                }
                                            })))
                                        }
                                        Adt102::C5(v26, v27, v28, v29) => {
                                            let m494 = f201(v26, v27, v28, v29.clone());
                                            match m494 {
                                                Adt114::C0(v30) => {
                                                    Ok::<(u64, (Adt101, List<Adt102>)), R<Adt99>>((v4, (Adt101::C1(v29, v30), v8)))
                                                }
                                                Adt114::C1(v31) => {
                                                    Err::<(u64, (Adt101, List<Adt102>)), R<Adt99>>(Ok(Adt99::C2(v31)))
                                                }
                                                Adt114::C2(v32) => {
                                                    Err::<(u64, (Adt101, List<Adt102>)), R<Adt99>>(Ok(Adt99::C3(v32)))
                                                }
                                                Adt114::C3(v33) => {
                                                    let v34: List<Adt102> = {
                                                        let a495 = v33;
                                                        let a496 = v8;
                                                        append_list(a495, a496)
                                                    };
                                                    Ok::<(u64, (Adt101, List<Adt102>)), R<Adt99>>((v4, (v2, v34)))
                                                }
                                            }
                                        }
                                    }
                                }
                                Adt101::C1(v40, v41) => {
                                    let v42: Adt107 = f202(v7, v8, v40, v41);
                                    Ok::<(u64, (Adt101, List<Adt102>)), R<Adt99>>((v4, (match v42.clone() {
                                        Adt107::C0(h499, _) => {
                                            h499
                                        }
                                    }, match v42 {
                                        Adt107::C0(_, h500) => {
                                            h500
                                        }
                                    })))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m501 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f135() -> u64 {
    67108864u64
}

pub fn f136(v0: List<Adt100>) -> R<Str> {
    let a505 = List::cons(Str::lit(""), f203(v0, List::<Str>::nil())?);
    let a506 = Str::lit("/");
    Ok(join(a505, a506))
}

pub fn f137(v0: Adt37, v1: Adt51) -> R<Result<Adt34, Adt86>> {
    if f204({
        let m509 = match v1.clone() {
            Adt51::C0(h508, _) => {
                h508
            }
        };
        match m509 {
            Adt39::C0(h507, _, _) => {
                h507
            }
        }
    })? {
        f205(v0, v1)
    } else {
        Ok(Err::<Adt34, Adt86>(Adt86::C6(Str::lit("a content payload is not canonical Base64"))))
    }
}

pub fn f138(v0: Str, v1: Adt5) -> Adt108 {
    f206(v0, match v1 {
        Adt5::C0(h510) => {
            h510
        }
    })
}

pub fn f139() -> Adt6 {
    Adt6::C4(List::<Adt6>::nil())
}

pub fn f140(v0: Str) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("causes"), f139()), List::cons((Str::lit("code"), f50(Str::lit("LLC0001"))), List::cons((Str::lit("help"), f139()), List::cons((Str::lit("labels"), f139()), List::cons((Str::lit("message"), f50({
        let a511 = List::cons(Str::lit("`"), List::cons(v0, List::cons(Str::lit("` is not a registered diagnostic code"), List::<Str>::nil())));
        let a512 = Str::lit("");
        join(a511, a512)
    })), List::cons((Str::lit("notes"), f139()), List::cons((Str::lit("severity"), f50(Str::lit("error"))), f27(Str::lit("spec"), f50(Str::lit("lexlean/diagnostic/1")))))))))))
}

pub fn f141(v0: Adt6) -> R<Result<List<u64>, Adt104>> {
    f207(f208(), v0)
}

pub fn f142(v0: Str) -> Adt40 {
    Adt40::C0(Adt41::C0(v0, None::<Adt42>, None::<Adt5>))
}

pub fn f143(v0: Str, p519: List<(Str, Str)>) -> Option<Str> {
    let m518 = p519;
    {
        let mut m518 = m518;
        loop {
            let step = {
                let v1 = m518;
                let m513 = v1;
                match m513.uncons() {
                    None => {
                        Err::<List<(Str, Str)>, Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        let a515 = v0.clone();
                        let a516 = {
                            let (h514, _) = v2;
                            h514
                        };
                        if equal(a515, a516) {
                            Err::<List<(Str, Str)>, Option<Str>>(Some({
                                let (_, h517) = v2;
                                h517
                            }))
                        } else {
                            Ok::<List<(Str, Str)>, Option<Str>>(v3)
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

pub fn f144() -> List<(Str, Str)> {
    List::cons((Str::lit("LLB6001"), Str::lit("## `LLB6001` --- Lean lowering has no defined form\u{a}\u{a}Lean lowering has no defined form.\u{a}\u{a}Class: `language`. Exit code: 1.\u{a}")), List::cons((Str::lit("LLB6002"), Str::lit("## `LLB6002` --- LaTeX lowering or renderer-token coverage failure\u{a}\u{a}LaTeX lowering or renderer-token coverage failure.\u{a}\u{a}Class: `language`. Exit code: 1.\u{a}")), List::cons((Str::lit("LLB6003"), Str::lit("## `LLB6003` --- Artifact hash, schema, or atomic publication failure\u{a}\u{a}Artifact hash, schema, or atomic publication failure.\u{a}\u{a}Class: `language`. Exit code: 1.\u{a}")), List::cons((Str::lit("LLB6004"), Str::lit("## `LLB6004` --- PDF-provider protocol failure\u{a}\u{a}PDF-provider protocol failure.\u{a}\u{a}Class: `environment`. Exit code: 3.\u{a}")), List::cons((Str::lit("LLB6005"), Str::lit("## `LLB6005` --- Target realization program or Rust package invalid or unrenderable\u{a}\u{a}A production-realization target program is malformed, violates the calculus's static rules, or cannot be rendered faithfully to the Rust profile, or a Rust package manifest is malformed or misstates its crate name, version, sources, exports, or the interface of the program it packages.\u{a}\u{a}Class: `language`. Exit code: 1.\u{a}")), List::cons((Str::lit("LLB6006"), Str::lit("## `LLB6006` --- GNAF request malformed\u{a}\u{a}A GNAF request is not a closed `lexlean/gnaf-request/1` document, its reference program or a system its universe realizes is not a valid target program, its reference returns no value on a domain argument, its grammar repeats a threshold or misdeclares a plan's preparation, or its stated universe identity is not the one its components determine.\u{a}\u{a}Class: `language`. Exit code: 1.\u{a}")), List::cons((Str::lit("LLB6007"), Str::lit("## `LLB6007` --- GNAF certificate malformed\u{a}\u{a}A GNAF certificate is not a closed `lexlean/gnaf-certificate/1` document, or a certificate fixture states an identity other than its certificate's.\u{a}\u{a}Class: `language`. Exit code: 1.\u{a}")), List::cons((Str::lit("LLC0001"), Str::lit("## `LLC0001` --- Invalid command-line usage\u{a}\u{a}Invalid command-line usage.\u{a}\u{a}Class: `cli-configuration`. Exit code: 2.\u{a}")), List::<(Str, Str)>::nil()))))))))
}

pub fn f145() -> List<(Str, Str)> {
    List::cons((Str::lit("LLC0002"), Str::lit("## `LLC0002` --- Mutually exclusive or empty selection\u{a}\u{a}Mutually exclusive or empty selection.\u{a}\u{a}Class: `cli-configuration`. Exit code: 2.\u{a}")), List::cons((Str::lit("LLC0101"), Str::lit("## `LLC0101` --- Invalid or unknown project configuration field\u{a}\u{a}Invalid or unknown project configuration field.\u{a}\u{a}Class: `cli-configuration`. Exit code: 2.\u{a}")), List::cons((Str::lit("LLC0102"), Str::lit("## `LLC0102` --- Missing, stale, or noncanonical lock file\u{a}\u{a}Missing, stale, or noncanonical lock file.\u{a}\u{a}Class: `cli-configuration`. Exit code: 2.\u{a}")), List::cons((Str::lit("LLC0103"), Str::lit("## `LLC0103` --- Unsupported schema or language version\u{a}\u{a}Unsupported schema or language version.\u{a}\u{a}Class: `cli-configuration`. Exit code: 2.\u{a}")), List::cons((Str::lit("LLC0104"), Str::lit("## `LLC0104` --- Duplicate module or case-folded path collision\u{a}\u{a}Duplicate module or case-folded path collision.\u{a}\u{a}Class: `cli-configuration`. Exit code: 2.\u{a}")), List::cons((Str::lit("LLC0105"), Str::lit("## `LLC0105` --- Source package is not the package the lock pins\u{a}\u{a}A source package's version, source, module prefix, manifest digest, tree digest, or declared packages differ from the lock's row, naming the package and both digests.\u{a}\u{a}Class: `cli-configuration`. Exit code: 2.\u{a}")), List::cons((Str::lit("LLC0106"), Str::lit("## `LLC0106` --- Source package undeclared or declared with two sources\u{a}\u{a}The lock pins a source package the project does not declare, or one identifier names two sources in the closure.\u{a}\u{a}Class: `cli-configuration`. Exit code: 2.\u{a}")), List::cons((Str::lit("LLC0107"), Str::lit("## `LLC0107` --- Module prefix claimed by two sources\u{a}\u{a}A module prefix is claimed by the project and a source package, or by two source packages, equal or one beneath the other, or an import names a module both the project and a package provide.\u{a}\u{a}Class: `cli-configuration`. Exit code: 2.\u{a}")), List::<(Str, Str)>::nil()))))))))
}

pub fn f146() -> List<(Str, Str)> {
    List::cons((Str::lit("LLC0108"), Str::lit("## `LLC0108` --- Source package outside the workspace rule\u{a}\u{a}A project that declares a source package is outside any workspace or is not a member of it, a package is not a member of the workspace, or the workspace file or a package is malformed.\u{a}\u{a}Class: `cli-configuration`. Exit code: 2.\u{a}")), List::cons((Str::lit("LLC0201"), Str::lit("## `LLC0201` --- Production request malformed\u{a}\u{a}A production request is not a closed `lexlean/production-request/1` document: an unknown or missing field, another spec, roots, targets, or evidence that are empty, unsorted, or repeated, a root, package, or path that is not of its form, or a package name used twice for one target.\u{a}\u{a}Class: `cli-configuration`. Exit code: 2.\u{a}")), List::cons((Str::lit("LLC0202"), Str::lit("## `LLC0202` --- Requested root is not a production root\u{a}\u{a}A production request names a root that is not a linked production root of the project's entrypoint closure.\u{a}\u{a}Class: `cli-configuration`. Exit code: 2.\u{a}")), List::cons((Str::lit("LLC0203"), Str::lit("## `LLC0203` --- Unknown or mismatched production target\u{a}\u{a}A production request names a target that is not registered, that its root does not declare, or whose descriptor digest is not the registered target's.\u{a}\u{a}Class: `cli-configuration`. Exit code: 2.\u{a}")), List::cons((Str::lit("LLC0204"), Str::lit("## `LLC0204` --- Stale production expectation\u{a}\u{a}The source ID, semantic ID, or lock digest a production request pins differs from the project's, or the project changed while it was being exported.\u{a}\u{a}Class: `cli-configuration`. Exit code: 2.\u{a}")), List::cons((Str::lit("LLF5001"), Str::lit("## `LLF5001` --- Invalid definition form, self head, or recursion\u{a}\u{a}Invalid definition form, self head, or recursion.\u{a}\u{a}Class: `language`. Exit code: 1.\u{a}")), List::cons((Str::lit("LLF5002"), Str::lit("## `LLF5002` --- Invalid proof step for the current goal\u{a}\u{a}Invalid proof step for the current goal.\u{a}\u{a}Class: `language`. Exit code: 1.\u{a}")), List::cons((Str::lit("LLF5003"), Str::lit("## `LLF5003` --- Missing, duplicate, or malformed proof branch\u{a}\u{a}Missing, duplicate, or malformed proof branch.\u{a}\u{a}Class: `language`. Exit code: 1.\u{a}")), List::<(Str, Str)>::nil()))))))))
}

pub fn f147() -> List<(Str, Str)> {
    List::cons((Str::lit("LLF5004"), Str::lit("## `LLF5004` --- Proof does not close all goals\u{a}\u{a}Proof does not close all goals.\u{a}\u{a}Class: `language`. Exit code: 1.\u{a}")), List::cons((Str::lit("LLF5005"), Str::lit("## `LLF5005` --- Forbidden proof form or proof hole\u{a}\u{a}Forbidden proof form or proof hole.\u{a}\u{a}Class: `language`. Exit code: 1.\u{a}")), List::cons((Str::lit("LLI9001"), Str::lit("## `LLI9001` --- Internal invariant failure\u{a}\u{a}Internal invariant failure.\u{a}\u{a}Class: `internal`. Exit code: 70.\u{a}")), List::cons((Str::lit("LLL1001"), Str::lit("## `LLL1001` --- Invalid UTF-8, forbidden scalar, or line-ending form\u{a}\u{a}Invalid UTF-8, forbidden scalar, or line-ending form.\u{a}\u{a}Class: `language`. Exit code: 1.\u{a}")), List::cons((Str::lit("LLL1002"), Str::lit("## `LLL1002` --- Forbidden comment, raw percent, tab, or TeX escape\u{a}\u{a}Forbidden comment, raw percent, tab, or TeX escape.\u{a}\u{a}Class: `language`. Exit code: 1.\u{a}")), List::cons((Str::lit("LLL1003"), Str::lit("## `LLL1003` --- Non-NFC or noncanonical source requiring formatting\u{a}\u{a}Non-NFC or noncanonical source requiring formatting.\u{a}\u{a}Class: `language`. Exit code: 1.\u{a}")), List::cons((Str::lit("LLL1004"), Str::lit("## `LLL1004` --- Unknown non-whitespace primitive atom\u{a}\u{a}Unknown non-whitespace primitive atom.\u{a}\u{a}Class: `language`. Exit code: 1.\u{a}")), List::cons((Str::lit("LLL1005"), Str::lit("## `LLL1005` --- Source token-coverage gap or overlap\u{a}\u{a}Source token-coverage gap or overlap.\u{a}\u{a}Class: `language`. Exit code: 1.\u{a}")), List::<(Str, Str)>::nil()))))))))
}

pub fn f148() -> List<(Str, Str)> {
    List::cons((Str::lit("LLL1006"), Str::lit("## `LLL1006` --- Token-lattice construction failure\u{a}\u{a}Token-lattice construction failure.\u{a}\u{a}Class: `language`. Exit code: 1.\u{a}")), List::cons((Str::lit("LLP2001"), Str::lit("## `LLP2001` --- No grammar parse\u{a}\u{a}No grammar parse.\u{a}\u{a}Class: `language`. Exit code: 1.\u{a}")), List::cons((Str::lit("LLP2002"), Str::lit("## `LLP2002` --- More than one distinct linked interpretation\u{a}\u{a}More than one distinct linked interpretation.\u{a}\u{a}Class: `language`. Exit code: 1.\u{a}")), List::cons((Str::lit("LLP2003"), Str::lit("## `LLP2003` --- Invalid structural order or component cardinality\u{a}\u{a}Invalid structural order or component cardinality.\u{a}\u{a}Class: `language`. Exit code: 1.\u{a}")), List::cons((Str::lit("LLP2004"), Str::lit("## `LLP2004` --- Invalid math precedence, associativity, or grouping\u{a}\u{a}Invalid math precedence, associativity, or grouping.\u{a}\u{a}Class: `language`. Exit code: 1.\u{a}")), List::cons((Str::lit("LLR3001"), Str::lit("## `LLR3001` --- Missing package, version, source, or digest mismatch\u{a}\u{a}Missing package, version, source, or digest mismatch.\u{a}\u{a}Class: `language`. Exit code: 1.\u{a}")), List::cons((Str::lit("LLR3002"), Str::lit("## `LLR3002` --- Duplicate package, entry, form, or qualified ID\u{a}\u{a}Duplicate package, entry, form, or qualified ID.\u{a}\u{a}Class: `language`. Exit code: 1.\u{a}")), List::cons((Str::lit("LLR3003"), Str::lit("## `LLR3003` --- Import, denotation, module, or definition cycle\u{a}\u{a}Import, denotation, module, or definition cycle.\u{a}\u{a}Class: `language`. Exit code: 1.\u{a}")), List::<(Str, Str)>::nil()))))))))
}

pub fn f149() -> List<(Str, Str)> {
    List::cons((Str::lit("LLR3004"), Str::lit("## `LLR3004` --- Invalid LSE, LRE, frame, form, or renderer token\u{a}\u{a}Invalid LSE, LRE, frame, form, or renderer token.\u{a}\u{a}Class: `language`. Exit code: 1.\u{a}")), List::cons((Str::lit("LLR3005"), Str::lit("## `LLR3005` --- Unavailable or invalid document/external reference\u{a}\u{a}Unavailable or invalid document/external reference.\u{a}\u{a}Class: `language`. Exit code: 1.\u{a}")), List::cons((Str::lit("LLR3006"), Str::lit("## `LLR3006` --- Unsafe canonical form or raw renderer output\u{a}\u{a}Unsafe canonical form or raw renderer output.\u{a}\u{a}Class: `language`. Exit code: 1.\u{a}")), List::cons((Str::lit("LLR3007"), Str::lit("## `LLR3007` --- Model artifact unavailable or its digest or length mismatched\u{a}\u{a}Model artifact unavailable or its digest or length mismatched.\u{a}\u{a}Class: `language`. Exit code: 1.\u{a}")), List::cons((Str::lit("LLR3008"), Str::lit("## `LLR3008` --- Model artifact bytes violate the declared schema, type, or role\u{a}\u{a}Model artifact bytes violate the declared schema, type, or role.\u{a}\u{a}Class: `language`. Exit code: 1.\u{a}")), List::cons((Str::lit("LLR3009"), Str::lit("## `LLR3009` --- Import of a module of a source package the importing scope does not declare\u{a}\u{a}A module imports a module of a source package that the project, or the package the module belongs to, does not declare.\u{a}\u{a}Class: `language`. Exit code: 1.\u{a}")), List::cons((Str::lit("LLS8001"), Str::lit("## `LLS8001` --- Path escape, symlink, special file, or filesystem identity conflict\u{a}\u{a}Path escape, symlink, special file, or filesystem identity conflict.\u{a}\u{a}Class: `security-limit`. Exit code: 4.\u{a}")), List::cons((Str::lit("LLS8002"), Str::lit("## `LLS8002` --- Explicit resource limit exceeded\u{a}\u{a}An explicit project resource limit, or the declared capacity of an evaluator such as the GNAF host's (\u{a7}17.15), is exceeded.\u{a}\u{a}Class: `security-limit`. Exit code: 4.\u{a}")), List::<(Str, Str)>::nil()))))))))
}

pub fn f150() -> List<(Str, Str)> {
    List::cons((Str::lit("LLS8003"), Str::lit("## `LLS8003` --- Network operation attempted outside permitted lock acquisition\u{a}\u{a}Network operation attempted outside permitted lock acquisition.\u{a}\u{a}Class: `security-limit`. Exit code: 4.\u{a}")), List::cons((Str::lit("LLS8004"), Str::lit("## `LLS8004` --- Child environment, executable, hash, or shell-policy violation\u{a}\u{a}Child environment, executable, hash, or shell-policy violation.\u{a}\u{a}Class: `security-limit`. Exit code: 4.\u{a}")), List::cons((Str::lit("LLT4001"), Str::lit("## `LLT4001` --- Arity, binder, category, or conservative type mismatch\u{a}\u{a}Arity, binder, category, or conservative type mismatch.\u{a}\u{a}Class: `language`. Exit code: 1.\u{a}")), List::cons((Str::lit("LLT4002"), Str::lit("## `LLT4002` --- Unresolved overloaded entry\u{a}\u{a}Unresolved overloaded entry.\u{a}\u{a}Class: `language`. Exit code: 1.\u{a}")), List::cons((Str::lit("LLT4003"), Str::lit("## `LLT4003` --- External-interface probe mismatch\u{a}\u{a}External-interface probe mismatch.\u{a}\u{a}Class: `language`. Exit code: 1.\u{a}")), List::cons((Str::lit("LLT4004"), Str::lit("## `LLT4004` --- Document-entry signature mismatch\u{a}\u{a}Document-entry signature mismatch.\u{a}\u{a}Class: `language`. Exit code: 1.\u{a}")), List::cons((Str::lit("LLT4005"), Str::lit("## `LLT4005` --- Production root not eligible for a declared target\u{a}\u{a}Production root not eligible for a declared target.\u{a}\u{a}Class: `language`. Exit code: 1.\u{a}")), List::cons((Str::lit("LLT4006"), Str::lit("## `LLT4006` --- Model contract, realization, or artifact interface mismatch\u{a}\u{a}Model contract, realization, or artifact interface mismatch.\u{a}\u{a}Class: `language`. Exit code: 1.\u{a}")), List::<(Str, Str)>::nil()))))))))
}

pub fn f151() -> List<(Str, Str)> {
    List::cons((Str::lit("LLT4007"), Str::lit("## `LLT4007` --- Invalid model composition or entry obligation\u{a}\u{a}Invalid model composition or entry obligation.\u{a}\u{a}Class: `language`. Exit code: 1.\u{a}")), List::cons((Str::lit("LLT4008"), Str::lit("## `LLT4008` --- Unvalidated model runtime boundary\u{a}\u{a}Unvalidated model runtime boundary.\u{a}\u{a}Class: `language`. Exit code: 1.\u{a}")), List::cons((Str::lit("LLT4009"), Str::lit("## `LLT4009` --- Model evidence not established\u{a}\u{a}Model evidence not established.\u{a}\u{a}Class: `language`. Exit code: 1.\u{a}")), List::cons((Str::lit("LLT4010"), Str::lit("## `LLT4010` --- Reasoning interface or obligation mismatch\u{a}\u{a}Reasoning interface or obligation mismatch.\u{a}\u{a}Class: `language`. Exit code: 1.\u{a}")), List::cons((Str::lit("LLT4011"), Str::lit("## `LLT4011` --- Unbounded reasoning or missing termination evidence\u{a}\u{a}Unbounded reasoning or missing termination evidence.\u{a}\u{a}Class: `language`. Exit code: 1.\u{a}")), List::cons((Str::lit("LLT4012"), Str::lit("## `LLT4012` --- Unvalidated reasoning runtime boundary\u{a}\u{a}Unvalidated reasoning runtime boundary.\u{a}\u{a}Class: `language`. Exit code: 1.\u{a}")), List::cons((Str::lit("LLT4013"), Str::lit("## `LLT4013` --- Tooling module malformed or out of context\u{a}\u{a}A tooling module is not canonical JSON of the closed schema lexlean/tooling-module/1, has a member the schema does not close, nests deeper than the decoder allows, or is not in a language-1.2 project the compiler embeds.\u{a}\u{a}Class: `language`. Exit code: 1.\u{a}")), List::cons((Str::lit("LLT4014"), Str::lit("## `LLT4014` --- Tooling authority registry invalid\u{a}\u{a}A tooling module pins an authority other than the compiler's Lean release, or its registry of types, calls, and plumbing is not closed: a name declared twice, a signature or field naming an unregistered constant or an unbound variable, a type whose parameters, constructors, index requirements, or abbreviation are inconsistent, a plumbing row that is empty, unsorted, or repeats a constant, or a source pin that is not a toolchain path with a SHA-256.\u{a}\u{a}Class: `language`. Exit code: 1.\u{a}")), List::<(Str, Str)>::nil()))))))))
}

pub fn f152() -> List<(Str, Str)> {
    List::cons((Str::lit("LLT4015"), Str::lit("## `LLT4015` --- Tooling term ill-typed or unlicensed\u{a}\u{a}A tooling term is not well typed: a call with the wrong type arguments or arguments, a match that is not exhaustive at its scrutinee's index, or a binder that shadows another; a function recurses without a licence (partial, or its contiguous mutual group); a command, invocation, or evaluation does not match its grammar; or a traversal does not recurse into every field of its recursive family.\u{a}\u{a}Class: `language`. Exit code: 1.\u{a}")), List::cons((Str::lit("LLT4016"), Str::lit("## `LLT4016` --- Unbounded or unspecified text generation\u{a}\u{a}A text declaration does not state the bound it must: a context window with capacity 0, or a text generator with max_tokens 0.\u{a}\u{a}Class: `language`. Exit code: 1.\u{a}")), List::cons((Str::lit("LLV7001"), Str::lit("## `LLV7001` --- Lean/Lake/leanchecker version or executable mismatch\u{a}\u{a}Lean/Lake/leanchecker version or executable mismatch.\u{a}\u{a}Class: `environment`. Exit code: 3.\u{a}")), List::cons((Str::lit("LLV7002"), Str::lit("## `LLV7002` --- Generated Lean elaboration or compilation failure\u{a}\u{a}Generated Lean elaboration or compilation failure.\u{a}\u{a}Class: `language`. Exit code: 1.\u{a}")), List::cons((Str::lit("LLV7003"), Str::lit("## `LLV7003` --- `leanchecker` replay failure\u{a}\u{a}`leanchecker` replay failure.\u{a}\u{a}Class: `language`. Exit code: 1.\u{a}")), List::cons((Str::lit("LLV7004"), Str::lit("## `LLV7004` --- Missing, malformed, or unexpected axiom-audit output\u{a}\u{a}Missing, malformed, or unexpected axiom-audit output.\u{a}\u{a}Class: `language`. Exit code: 1.\u{a}")), List::cons((Str::lit("LLV7005"), Str::lit("## `LLV7005` --- Axiom policy violation\u{a}\u{a}Axiom policy violation.\u{a}\u{a}Class: `language`. Exit code: 1.\u{a}")), List::cons((Str::lit("LLV7006"), Str::lit("## `LLV7006` --- Lean warning or unexpected successful-process output\u{a}\u{a}Lean warning or unexpected successful-process output.\u{a}\u{a}Class: `language`. Exit code: 1.\u{a}")), List::<(Str, Str)>::nil()))))))))
}

pub fn f153() -> List<(Str, Str)> {
    List::cons((Str::lit("LLV7007"), Str::lit("## `LLV7007` --- Lake workspace lock or dependency availability mismatch\u{a}\u{a}Lake workspace lock or dependency availability mismatch.\u{a}\u{a}Class: `environment`. Exit code: 3.\u{a}")), List::cons((Str::lit("LLV7008"), Str::lit("## `LLV7008` --- Non-UTF-8 path or argument in the environment\u{a}\u{a}A host path, working directory, or command-line argument is not valid UTF-8; non-UTF-8 project paths are outside language 1.0 (\u{a7}8.3).\u{a}\u{a}Class: `environment`. Exit code: 3.\u{a}")), List::cons((Str::lit("LLV7009"), Str::lit("## `LLV7009` --- Git acquisition environment failure\u{a}\u{a}During `lock --allow-network`, the git executable could not be resolved or run, or the exact configured commit could not be fetched or checked out from the HTTPS remote.\u{a}\u{a}Class: `environment`. Exit code: 3.\u{a}")), List::cons((Str::lit("LLV7010"), Str::lit("## `LLV7010` --- Host filesystem or platform failure\u{a}\u{a}A host filesystem or platform operation under the project (creating, writing, renaming, or locking a file) failed for a reason outside the project's control.\u{a}\u{a}Class: `environment`. Exit code: 3.\u{a}")), List::cons((Str::lit("LLV7011"), Str::lit("## `LLV7011` --- Named-root extraction rejected\u{a}\u{a}Lean's compiler front end could not extract a production root's computational closure, or the extracted closure names an unknown root, an opaque, unsafe, partial, or noncomputable dependency, an unresolved or unsupported constant, an unsupported compiler form, or disagrees with the production-eligibility closure.\u{a}\u{a}Class: `language`. Exit code: 1.\u{a}")), List::cons((Str::lit("LLV7012"), Str::lit("## `LLV7012` --- Lean compiler-front-end authority drift\u{a}\u{a}The pinned Lean compiler-front-end interface no longer matches its registry: a signature probe or the pinned extraction adapter fails to elaborate, or the running Lean reports another version or source commit.\u{a}\u{a}Class: `environment`. Exit code: 3.\u{a}")), List::cons((Str::lit("LLV7013"), Str::lit("## `LLV7013` --- Certificate A rejected\u{a}\u{a}A production root's preservation certificate does not compile silently under the pinned Lean, its leanchecker replay fails, or its root theorem depends on axioms other than exactly Classical.choice, Quot.sound, and propext.\u{a}\u{a}Class: `language`. Exit code: 1.\u{a}")), List::cons((Str::lit("LLV7014"), Str::lit("## `LLV7014` --- Preservation environment drift\u{a}\u{a}The shipped preservation library or calculus modules fail their token audit, do not compile silently under the pinned Lean, or a library declaration depends on axioms other than those its registry row states.\u{a}\u{a}Class: `environment`. Exit code: 3.\u{a}")), List::<(Str, Str)>::nil()))))))))
}

pub fn f154() -> List<(Str, Str)> {
    List::cons((Str::lit("LLV7015"), Str::lit("## `LLV7015` --- Certificate B rejected\u{a}\u{a}A production root's rendering in one of its targets admits no derivation of the correspondence with its lowered program, its certificate does not compile silently under the pinned Lean, its leanchecker replay fails, or its simulation theorem depends on axioms other than exactly Classical.choice, Quot.sound, and propext.\u{a}\u{a}Class: `language`. Exit code: 1.\u{a}")), List::cons((Str::lit("LLV7016"), Str::lit("## `LLV7016` --- Certificate E rejected\u{a}\u{a}The composition of a production root's certificates A and B for one of its targets does not compile silently under the pinned Lean, its leanchecker replay fails, or its end-to-end theorem depends on axioms other than exactly Classical.choice, Quot.sound, and propext.\u{a}\u{a}Class: `language`. Exit code: 1.\u{a}")), List::cons((Str::lit("LLV7017"), Str::lit("## `LLV7017` --- Invalid verification resource profile\u{a}\u{a}The operational verification process width the environment selects is not a decimal integer from 1 to 64.\u{a}\u{a}Class: `environment`. Exit code: 3.\u{a}")), List::cons((Str::lit("LLV7018"), Str::lit("## `LLV7018` --- Child start refused after the host ended the live children\u{a}\u{a}A child process could not be started because the host ended the live children of this process through `lexlean::verify::end_live_children` and has not re-enabled starting children.\u{a}\u{a}Class: `environment`. Exit code: 3.\u{a}")), List::cons((Str::lit("LLV7101"), Str::lit("## `LLV7101` --- Certificate evidence does not certify the root\u{a}\u{a}Certificate evidence a production request supplies is not the certificate whose identity it pins, earns a refused verdict for its request, or is for a request whose reference program is not the root's lowered program.\u{a}\u{a}Class: `language`. Exit code: 1.\u{a}")), List::cons((Str::lit("LLV7102"), Str::lit("## `LLV7102` --- Production artifact replay rejected\u{a}\u{a}A production bundle does not replay: its result identity, a file digest, a binding between the result, the attestation, the preservation record, and its packages, a re-rendered package, a target descriptor, the producing compiler's semantics, or a recomputed certificate verdict disagrees with what the bundle states, or the bundle holds a file it does not state.\u{a}\u{a}Class: `language`. Exit code: 1.\u{a}")), List::<(Str, Str)>::nil()))))))
}

pub fn f155(v0: Str) -> u64 {
    let a520 = v0;
    let a521 = Str::lit("drafts");
    if equal(a520, a521) {
        1u64
    } else {
        0u64
    }
}

pub fn f156(v0: Adt23) -> Option<List<Str>> {
    let m522 = f88(v0, Str::lit("files"));
    match m522 {
        None => {
            Some(List::<Str>::nil())
        }
        Some(v1) => {
            let m523 = f209(v1);
            match m523 {
                None => {
                    None::<List<Str>>
                }
                Some(v2) => {
                    let m524 = f210(v2, List::<Str>::nil());
                    match m524 {
                        None => {
                            None::<List<Str>>
                        }
                        Some(v3) => {
                            Some(f211(v3, List::<Str>::nil()))
                        }
                    }
                }
            }
        }
    }
}

pub fn f157(p527: List<Str>) -> R<bool> {
    let m526 = p527;
    {
        let mut m526 = m526;
        loop {
            let step = {
                let v0 = m526;
                let m525 = v0;
                match m525.uncons() {
                    None => {
                        Err::<List<Str>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        if f212(f97(v1)?) {
                            Err::<List<Str>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<Str>, R<bool>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m526 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f158(v0: bool, v1: List<Str>) -> bool {
    if v0 {
        let m528 = v1;
        match m528.uncons() {
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

pub fn f159(v0: bool, v1: List<Str>) -> R<List<u64>> {
    let m529 = v1.clone();
    match m529.uncons() {
        None => {
            let a530 = f17(if v0 {
                1u64
            } else {
                0u64
            });
            let a531 = f103(Str::lit(""))?;
            Ok(append_list(a530, a531))
        }
        Some((_, _)) => {
            let a534 = f17(2u64);
            let a535 = f103({
                let a532 = v1;
                let a533 = Str::lit("\u{a}");
                join(a532, a533)
            })?;
            Ok(append_list(a534, a535))
        }
    }
}

pub fn f160(v0: Adt6) -> Option<bool> {
    let m536 = v0;
    match m536 {
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

pub fn f161(p541: Bytes, p542: List<Bytes>) -> Bytes {
    let m540 = (p541, p542);
    {
        let mut m540 = m540;
        loop {
            let step = {
                let (v0, v1) = m540;
                let m537 = v1;
                match m537.uncons() {
                    None => {
                        Err::<(Bytes, List<Bytes>), Bytes>(v0)
                    }
                    Some((v2, v3)) => {
                        Ok::<(Bytes, List<Bytes>), Bytes>(({
                            let a538 = v2;
                            let a539 = v0;
                            append_bytes(a538, a539)
                        }, v3))
                    }
                }
            };
            match step {
                Ok(next) => { m540 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f162(p545: List<Bytes>, p546: List<u64>) -> List<Bytes> {
    let m544 = (p545, p546);
    {
        let mut m544 = m544;
        loop {
            let step = {
                let (v0, v1) = m544;
                let m543 = v1;
                match m543.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<u64>), List<Bytes>>(v0)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<u64>), List<Bytes>>((f213(f214(v2), v0), v3))
                    }
                }
            };
            match step {
                Ok(next) => { m544 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f163(p550: List<Str>, p551: u64) -> Str {
    let m549 = (p550, p551);
    {
        let mut m549 = m549;
        loop {
            let step = {
                let (v0, v1) = m549;
                let m547 = v0;
                match m547.uncons() {
                    None => {
                        Err::<(List<Str>, u64), Str>(Str::lit("unknown"))
                    }
                    Some((v2, v3)) => {
                        let m548 = v1;
                        if m548 == 0 {
                            Err::<(List<Str>, u64), Str>(v2)
                        } else {
                            let v4 = m548 - 1;
                            Ok::<(List<Str>, u64), Str>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m549 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f164(v0: Adt97) -> List<Str> {
    let m552 = v0;
    match m552 {
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

pub fn f165(v0: Adt97) -> Str {
    let m553 = v0;
    match m553 {
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

pub fn f166() -> Adt51 {
    f215(Str::lit("the host answered with a malformed outcome"))
}

pub fn f167(p556: u64, p557: i64) -> R<i64> {
    let m555 = (p556, p557);
    {
        let mut m555 = m555;
        loop {
            let step = {
                let (v0, v1) = m555;
                let m554 = v0;
                if m554 == 0 {
                    Err::<(u64, i64), R<i64>>(Ok(v1))
                } else {
                    let v2 = m554 - 1;
                    Ok::<(u64, i64), R<i64>>((v2, f216(v1, 1i64)?))
                }
            };
            match step {
                Ok(next) => { m555 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f168(v0: Bytes) -> R<Result<Adt6, Adt104>> {
    f217(f18(v0)?)
}

pub fn f169(v0: Adt6) -> bool {
    let m558 = f218(v0);
    match m558 {
        None => {
            false
        }
        Some(v1) => {
            let m559 = f138(Str::lit("spec"), v1);
            match m559 {
                Adt108::C0 => {
                    false
                }
                Adt108::C1(v2) => {
                    let m560 = f89(v2);
                    match m560 {
                        None => {
                            false
                        }
                        Some(v3) => {
                            let a561 = v3;
                            let a562 = Str::lit("lexlean/command-result/1");
                            !equal(a561, a562)
                        }
                    }
                }
            }
        }
    }
}

pub fn f170(v0: Adt6, v1: u64) -> R<bool> {
    let m563 = f218(v0);
    match m563 {
        None => {
            Ok(false)
        }
        Some(v2) => {
            let m564 = f138(Str::lit("exit_code"), v2);
            match m564 {
                Adt108::C0 => {
                    Ok(false)
                }
                Adt108::C1(v3) => {
                    let m565 = f219(v3, f53(v1)?)?;
                    match m565 {
                        Adt116::C0 => {
                            Ok(true)
                        }
                        Adt116::C1 => {
                            Ok(false)
                        }
                        Adt116::C2 => {
                            Ok(false)
                        }
                    }
                }
            }
        }
    }
}

pub fn f171(v0: u64) -> Str {
    f163(List::cons(Str::lit("queued"), List::cons(Str::lit("running"), List::cons(Str::lit("verified"), List::cons(Str::lit("failed"), List::cons(Str::lit("cancelled"), List::<Str>::nil()))))), v0)
}

pub fn f172(p568: List<Adt15>, p569: List<Adt15>) -> List<Adt15> {
    let m567 = (p568, p569);
    {
        let mut m567 = m567;
        loop {
            let step = {
                let (v0, v1) = m567;
                let m566 = v0;
                match m566.uncons() {
                    None => {
                        Err::<(List<Adt15>, List<Adt15>), List<Adt15>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt15>, List<Adt15>), List<Adt15>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m567 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f173(v0: bool, v1: Bytes, v2: List<Adt15>, v3: u64) -> Result<Adt89, Adt90> {
    if v0 {
        Ok::<Adt89, Adt90>(Adt89::C0(v1, v2))
    } else {
        Err::<Adt89, Adt90>(Adt90::C1(v3))
    }
}

pub fn f174(v0: Adt92) -> List<u64> {
    let a580 = f17({
        let a571 = match v0.clone() {
            Adt92::C0(h570, _) => {
                h570
            }
        };
        length_list(a571)
    });
    let a581 = {
        let a578 = match v0.clone() {
            Adt92::C0(h572, _) => {
                h572
            }
        };
        let a579 = {
            let a576 = f17({
                let a574 = match v0.clone() {
                    Adt92::C0(_, h573) => {
                        h573
                    }
                };
                length_list(a574)
            });
            let a577 = match v0 {
                Adt92::C0(_, h575) => {
                    h575
                }
            };
            append_list(a576, a577)
        };
        append_list(a578, a579)
    };
    append_list(a580, a581)
}

pub fn f175(v0: List<u64>) -> List<u64> {
    let a583 = f17({
        let a582 = v0.clone();
        length_list(a582)
    });
    let a584 = v0;
    append_list(a583, a584)
}

pub fn f176(v0: u64, v1: Adt6) -> R<Adt95> {
    let m585 = f141(v1)?;
    match m585 {
        Ok(v3) => {
            Ok(Adt95::C0(v0, f73(), List::<List<u64>>::nil(), v3))
        }
        Err(_) => {
            Ok(f44())
        }
    }
}

pub fn f177(v0: Adt98) -> u64 {
    let m586 = v0;
    match m586 {
        Adt98::C0 => {
            f220()
        }
        Adt98::C1 => {
            f220()
        }
    }
}

pub fn f178(v0: Option<Adt105>, v1: Adt82) -> Adt6 {
    Adt6::C5(f221(v0, v1))
}

pub fn f179(v0: Adt98) -> Adt82 {
    Adt82::C0(f222(), f223(v0), None::<Adt6>)
}

pub fn f180(v0: List<u64>) -> bool {
    f224(v0, f225())
}

pub fn f181(v0: Adt0, v1: Fn2, v2: u64, v3: List<Adt92>, v4: Result<Adt6, Adt104>) -> R<Adt88> {
    let m587 = v4;
    match m587 {
        Ok(v6) => {
            f228(v0, v1, v2, v3, f229(v6)?)
        }
        Err(_) => {
            Ok(Adt88::C0(f43(f176(f226(), f178(None::<Adt105>, f227(Adt109::C0)))?)))
        }
    }
}

pub fn f182(v0: Adt0, v1: Bytes) -> Adt0 {
    Adt0::C0(match v0.clone() {
        Adt0::C0(h588, _, _, _, _, _, _) => {
            h588
        }
    }, match v0.clone() {
        Adt0::C0(_, h589, _, _, _, _, _) => {
            h589
        }
    }, match v0.clone() {
        Adt0::C0(_, _, h590, _, _, _, _) => {
            h590
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, h591, _, _, _) => {
            h591
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, h592, _, _) => {
            h592
        }
    }, match v0 {
        Adt0::C0(_, _, _, _, _, h593, _) => {
            h593
        }
    }, v1)
}

pub fn f183() -> Adt95 {
    Adt95::C0(f230(), List::cons(Adt92::C0(f231(), f232()), List::<Adt92>::nil()), List::<List<u64>>::nil(), List::<u64>::nil())
}

pub fn f184(v0: List<Adt92>, v1: List<u64>) -> R<Adt111> {
    let m594 = f233(v0, v1, Adt112::C0)?;
    match m594 {
        Adt112::C0 => {
            Ok(Adt111::C0)
        }
        Adt112::C1(v2) => {
            let m595 = f234(v2);
            match m595 {
                None => {
                    Ok(Adt111::C1)
                }
                Some(v3) => {
                    Ok(Adt111::C3(v3))
                }
            }
        }
        Adt112::C2 => {
            Ok(Adt111::C2)
        }
    }
}

pub fn f185() -> List<u64> {
    List::cons(79u64, List::cons(114u64, List::cons(105u64, List::cons(103u64, List::cons(105u64, List::cons(110u64, List::<u64>::nil()))))))
}

pub fn f186(v0: Option<List<u64>>, v1: List<List<u64>>, v2: Adt98) -> R<Option<Adt98>> {
    let m596 = v0;
    match m596 {
        None => {
            Ok(Some(v2))
        }
        Some(v3) => {
            if f235(v3, v1)? {
                Ok(None::<Adt98>)
            } else {
                Ok(Some(v2))
            }
        }
    }
}

pub fn f187(v0: List<u64>) -> Option<List<u64>> {
    let m597 = f236(v0);
    match m597 {
        None => {
            None::<List<u64>>
        }
        Some(v1) => {
            f189(v1)
        }
    }
}

pub fn f188() -> List<u64> {
    List::cons(72u64, List::cons(111u64, List::cons(115u64, List::cons(116u64, List::<u64>::nil()))))
}

pub fn f189(v0: List<u64>) -> Option<List<u64>> {
    let m598 = v0.clone();
    match m598.uncons() {
        None => {
            None::<List<u64>>
        }
        Some((v1, v2)) => {
            let a599 = v1;
            let a600 = 91u64;
            if nat_eq(a599, a600) {
                f237(f238(v2, List::cons(v1, List::<u64>::nil())))
            } else {
                f237(f239(v0, List::<u64>::nil()))
            }
        }
    }
}

pub fn f190(p603: List<Adt92>, p604: List<Adt92>) -> List<Adt92> {
    let m602 = (p603, p604);
    {
        let mut m602 = m602;
        loop {
            let step = {
                let (v0, v1) = m602;
                let m601 = v0;
                match m601.uncons() {
                    None => {
                        Err::<(List<Adt92>, List<Adt92>), List<Adt92>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt92>, List<Adt92>), List<Adt92>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m602 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f191(v0: Bytes, v1: u64) -> R<Option<Adt94>> {
    let a606 = v1;
    let a607 = {
        let a605 = v0.clone();
        length_bytes(a605)
    };
    if nat_eq(a606, a607) {
        Ok(Some(Adt94::C0(Bytes::lit(&[]), v1)))
    } else {
        f42(v0, v1)
    }
}

pub fn f192(v0: Bytes, v1: Adt91, v2: u64) -> R<Result<Adt91, Adt90>> {
    let a609 = v2;
    let a610 = {
        let a608 = v0.clone();
        length_bytes(a608)
    };
    if nat_eq(a609, a610) {
        Ok(Ok::<Adt91, Adt90>(v1))
    } else {
        let m611 = f69(v0.clone(), v2)?;
        match m611 {
            None => {
                Ok(Err::<Adt91, Adt90>(Adt90::C0(v2)))
            }
            Some(v3) => {
                let m614 = f240(v0.clone(), {
                    let a612 = v2;
                    let a613 = 4u64;
                    nat_add(a612, a613)?
                }, v3, List::<List<u64>>::nil())?;
                match m614 {
                    Ok(v5) => {
                        let a617 = {
                            let (_, h615) = v5.clone();
                            h615
                        };
                        let a618 = {
                            let a616 = v0;
                            length_bytes(a616)
                        };
                        if nat_eq(a617, a618) {
                            Ok(Ok::<Adt91, Adt90>(f241(v1, {
                                let (h619, _) = v5;
                                h619
                            })))
                        } else {
                            Ok(Err::<Adt91, Adt90>(Adt90::C1({
                                let (_, h620) = v5;
                                h620
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

pub fn f193(v0: List<u64>) -> bool {
    let m621 = v0.clone();
    match m621.uncons() {
        None => {
            true
        }
        Some((_, _)) => {
            let a625 = if f242(v0.clone(), false) {
                let a623 = {
                    let a622 = v0;
                    length_list(a622)
                };
                let a624 = f243();
                nat_le(a623, a624)
            } else {
                false
            };
            bool_not(a625)
        }
    }
}

pub fn f194(v0: Adt37) -> Option<Adt93> {
    let m630 = f244(match v0.clone() {
        Adt37::C0(h626, _, _, _, _, _, _, _) => {
            h626
        }
    }, {
        let m629 = match v0.clone() {
            Adt37::C0(_, _, _, _, h628, _, _, _) => {
                h628
            }
        };
        match m629 {
            Adt5::C0(h627) => {
                h627
            }
        }
    });
    match m630 {
        None => {
            f245(match v0.clone() {
                Adt37::C0(h631, _, _, _, _, _, _, _) => {
                    h631
                }
            }, {
                let m634 = match v0 {
                    Adt37::C0(_, _, _, _, h633, _, _, _) => {
                        h633
                    }
                };
                match m634 {
                    Adt5::C0(h632) => {
                        h632
                    }
                }
            })
        }
        Some(v1) => {
            Some(v1)
        }
    }
}

pub fn f195(v0: Adt37, v1: Result<Adt113, Adt93>) -> R<Result<List<Adt12>, Adt93>> {
    let m635 = v1;
    match m635 {
        Ok(v3) => {
            let m636 = f246(v0)?;
            match m636 {
                None => {
                    Ok(Ok::<List<Adt12>, Adt93>(f79(match v3 {
                        Adt113::C0(h637, _) => {
                            h637
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

pub fn f196(p644: u64, v1: Str, v2: Adt6, p645: List<Adt106>, p646: Adt113) -> R<Result<Adt113, Adt93>> {
    let m643 = (p644, (p645, p646));
    {
        let mut m643 = m643;
        loop {
            let step = {
                let (v0, (v3, v4)) = m643;
                let m638 = v0;
                if m638 == 0 {
                    Err::<(u64, (List<Adt106>, Adt113)), R<Result<Adt113, Adt93>>>(Ok(Err::<Adt113, Adt93>(Adt93::C5(v1.clone(), Str::lit("budget")))))
                } else {
                    let v5 = m638 - 1;
                    let m639 = v3;
                    match m639.uncons() {
                        None => {
                            Err::<(u64, (List<Adt106>, Adt113)), R<Result<Adt113, Adt93>>>(Ok(Ok::<Adt113, Adt93>(v4)))
                        }
                        Some((v6, v7)) => {
                            let m640 = f247(v1.clone(), v2.clone(), v6, v4, v7)?;
                            match m640 {
                                Ok(v9) => {
                                    Ok::<(u64, (List<Adt106>, Adt113)), R<Result<Adt113, Adt93>>>((v5, ({
                                        let (h641, _) = v9;
                                        h641
                                    }, {
                                        let (_, h642) = v9;
                                        h642
                                    })))
                                }
                                Err(v8) => {
                                    Err::<(u64, (List<Adt106>, Adt113)), R<Result<Adt113, Adt93>>>(Ok(Err::<Adt113, Adt93>(v8)))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m643 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f197() -> u64 {
    67108864u64
}

pub fn f198() -> Adt113 {
    Adt113::C0(List::<Adt12>::nil(), List::<List<u64>>::nil())
}

pub fn f199(v0: Adt6, v1: Adt6, v2: Adt6, v3: List<Adt100>) -> R<Adt114> {
    let m647 = f160(v1.clone());
    match m647 {
        None => {
            let m648 = f218(v1);
            match m648 {
                None => {
                    Ok(Adt114::C1(Str::lit("schema")))
                }
                Some(v5) => {
                    f248(v0, match v5 {
                        Adt5::C0(h649) => {
                            h649
                        }
                    }, v2, v3)
                }
            }
        }
        Some(v4) => {
            if v4 {
                Ok(Adt114::C3(List::<Adt102>::nil()))
            } else {
                Ok(Adt114::C0(Str::lit("false")))
            }
        }
    }
}

pub fn f200(v0: Adt102, v1: List<Adt102>) -> R<Adt107> {
    let m650 = v0;
    match m650 {
        Adt102::C0(_, _, _) => {
            Ok(Adt107::C0(Adt101::C0, v1))
        }
        Adt102::C1(_, _, _) => {
            Ok(Adt107::C0(Adt101::C0, v1))
        }
        Adt102::C2(v8, v9, v10, v11) => {
            Ok(f249(v8, v9, v10, {
                let a651 = v11;
                let a652 = 1u64;
                nat_add(a651, a652)?
            }, v1))
        }
        Adt102::C3(v12) => {
            Ok(Adt107::C0(Adt101::C1(v12, Str::lit("not")), v1))
        }
        Adt102::C4(v13, _, v15, v16) => {
            Ok(f250(v13, v15, v16, v1))
        }
        Adt102::C5(_, _, _, _) => {
            Ok(Adt107::C0(Adt101::C0, v1))
        }
    }
}

pub fn f201(v0: Adt103, v1: Adt6, v2: Adt6, v3: List<Adt100>) -> Adt114 {
    let m653 = f218(v1);
    match m653 {
        None => {
            Adt114::C1(Str::lit("schema"))
        }
        Some(v4) => {
            let m654 = v0;
            match m654 {
                Adt103::C0 => {
                    f251(match v4 {
                        Adt5::C0(h655) => {
                            h655
                        }
                    }, v2, v3)
                }
                Adt103::C1 => {
                    f252(match v4 {
                        Adt5::C0(h656) => {
                            h656
                        }
                    }, v2, v3)
                }
                Adt103::C2 => {
                    f253(match v4 {
                        Adt5::C0(h657) => {
                            h657
                        }
                    }, v2, v3)
                }
                Adt103::C3 => {
                    f254(match v4 {
                        Adt5::C0(h658) => {
                            h658
                        }
                    }, v2, v3)
                }
            }
        }
    }
}

pub fn f202(v0: Adt102, v1: List<Adt102>, v2: List<Adt100>, v3: Str) -> Adt107 {
    let m659 = v0;
    match m659 {
        Adt102::C0(_, _, _) => {
            Adt107::C0(Adt101::C1(v2, v3), v1)
        }
        Adt102::C1(v7, v8, v9) => {
            f255(v7, v8, v9, v1)
        }
        Adt102::C2(v10, v11, v12, v13) => {
            f249(v10, v11, v12, v13, v1)
        }
        Adt102::C3(_) => {
            Adt107::C0(Adt101::C0, v1)
        }
        Adt102::C4(_, v16, v17, v18) => {
            f250(v16, v17, v18, v1)
        }
        Adt102::C5(_, _, _, _) => {
            Adt107::C0(Adt101::C1(v2, v3), v1)
        }
    }
}

pub fn f203(p662: List<Adt100>, p663: List<Str>) -> R<List<Str>> {
    let m661 = (p662, p663);
    {
        let mut m661 = m661;
        loop {
            let step = {
                let (v0, v1) = m661;
                let m660 = v0;
                match m660.uncons() {
                    None => {
                        Err::<(List<Adt100>, List<Str>), R<List<Str>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt100>, List<Str>), R<List<Str>>>((v3, List::cons(f256(v2)?, v1)))
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

pub fn f204(p672: List<Adt40>) -> R<bool> {
    let m671 = p672;
    {
        let mut m671 = m671;
        loop {
            let step = {
                let v0 = m671;
                let m664 = v0;
                match m664.uncons() {
                    None => {
                        Err::<List<Adt40>, R<bool>>(Ok(true))
                    }
                    Some((v1, v2)) => {
                        let m665 = v1;
                        match m665 {
                            Adt40::C0(_) => {
                                Ok::<List<Adt40>, R<bool>>(v2)
                            }
                            Adt40::C1(v4) => {
                                if f257(match v4 {
                                    Adt44::C0(h666, _, _, _) => {
                                        h666
                                    }
                                })? {
                                    Ok::<List<Adt40>, R<bool>>(v2)
                                } else {
                                    Err::<List<Adt40>, R<bool>>(Ok(false))
                                }
                            }
                            Adt40::C2(v5) => {
                                if f257(match v5 {
                                    Adt45::C0(h667, _, _, _) => {
                                        h667
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
                                let m669 = match v7 {
                                    Adt47::C0(h668, _, _) => {
                                        h668
                                    }
                                };
                                match m669 {
                                    Adt48::C0(_) => {
                                        Ok::<List<Adt40>, R<bool>>(v2)
                                    }
                                    Adt48::C1(v9) => {
                                        if f257(match v9 {
                                            Adt50::C0(_, _, h670, _) => {
                                                h670
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
                Ok(next) => { m671 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f205(v0: Adt37, v1: Adt51) -> R<Result<Adt34, Adt86>> {
    if f258(match v1.clone() {
        Adt51::C0(h673, _) => {
            h673
        }
    }) {
        f259(v1)
    } else {
        let m675 = match v0 {
            Adt37::C0(_, _, _, _, _, h674, _, _) => {
                h674
            }
        };
        match m675 {
            None => {
                f259(v1)
            }
            Some(v2) => {
                f260(v2, v1)
            }
        }
    }
}

pub fn f206(v0: Str, p681: List<(Str, Adt6)>) -> Adt108 {
    let m680 = p681;
    {
        let mut m680 = m680;
        loop {
            let step = {
                let v1 = m680;
                let m676 = v1;
                match m676.uncons() {
                    None => {
                        Err::<List<(Str, Adt6)>, Adt108>(Adt108::C0)
                    }
                    Some((v2, v3)) => {
                        let m678 = f261({
                            let (h677, _) = v2;
                            h677
                        }, v0.clone());
                        match m678 {
                            Adt118::C0 => {
                                Err::<List<(Str, Adt6)>, Adt108>(Adt108::C1({
                                    let (_, h679) = v2;
                                    h679
                                }))
                            }
                            Adt118::C1 => {
                                Ok::<List<(Str, Adt6)>, Adt108>(v3)
                            }
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

pub fn f207(v0: u64, v1: Adt6) -> R<Result<List<u64>, Adt104>> {
    let m682 = f262(v0, List::cons(Adt115::C1(0u64, v1), List::<Adt115>::nil()), List::<u64>::nil())?;
    match m682 {
        Ok(v2) => {
            Ok(Ok::<List<u64>, Adt104>(f263(v2, List::<u64>::nil())))
        }
        Err(v3) => {
            Ok(Err::<List<u64>, Adt104>(v3))
        }
    }
}

pub fn f208() -> u64 {
    67108864u64
}

pub fn f209(v0: Adt6) -> Option<List<Adt6>> {
    let m683 = v0;
    match m683 {
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

pub fn f210(p687: List<Adt6>, p688: List<Str>) -> Option<List<Str>> {
    let m686 = (p687, p688);
    {
        let mut m686 = m686;
        loop {
            let step = {
                let (v0, v1) = m686;
                let m684 = v0;
                match m684.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<Str>), Option<List<Str>>>(Some(v1))
                    }
                    Some((v2, v3)) => {
                        let m685 = f89(v2);
                        match m685 {
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
                Ok(next) => { m686 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f211(p691: List<Str>, p692: List<Str>) -> List<Str> {
    let m690 = (p691, p692);
    {
        let mut m690 = m690;
        loop {
            let step = {
                let (v0, v1) = m690;
                let m689 = v0;
                match m689.uncons() {
                    None => {
                        Err::<(List<Str>, List<Str>), List<Str>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Str>, List<Str>), List<Str>>((v3, List::cons(v2, v1)))
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

pub fn f212(p697: List<u64>) -> bool {
    let m696 = p697;
    {
        let mut m696 = m696;
        loop {
            let step = {
                let v0 = m696;
                let m693 = v0;
                match m693.uncons() {
                    None => {
                        Err::<List<u64>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        let a694 = v1;
                        let a695 = 10u64;
                        if nat_eq(a694, a695) {
                            Err::<List<u64>, bool>(true)
                        } else {
                            Ok::<List<u64>, bool>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m696 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f213(p706: Bytes, p707: List<Bytes>) -> List<Bytes> {
    let m705 = (p706, p707);
    {
        let mut m705 = m705;
        loop {
            let step = {
                let (v0, v1) = m705;
                let m698 = v1;
                match m698.uncons() {
                    None => {
                        Err::<(Bytes, List<Bytes>), List<Bytes>>(List::cons(v0, List::<Bytes>::nil()))
                    }
                    Some((v2, v3)) => {
                        let a701 = {
                            let a699 = v2.clone();
                            length_bytes(a699)
                        };
                        let a702 = {
                            let a700 = v0.clone();
                            length_bytes(a700)
                        };
                        if nat_eq(a701, a702) {
                            Ok::<(Bytes, List<Bytes>), List<Bytes>>(({
                                let a703 = v2;
                                let a704 = v0;
                                append_bytes(a703, a704)
                            }, v3))
                        } else {
                            Err::<(Bytes, List<Bytes>), List<Bytes>>(List::cons(v0, List::cons(v2, v3)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m705 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f214(v0: u64) -> Bytes {
    let m711 = {
        let a708 = f264();
        let a709 = v0;
        let a710 = 1u64;
        slice_bytes(a708, a709, a710)
    };
    match m711 {
        None => {
            f105()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f215(v0: Str) -> Adt51 {
    Adt51::C0(Adt39::C0(List::cons(f142(v0), List::<Adt40>::nil()), None::<Adt6>, None::<bool>), List::<Adt52>::nil())
}

pub fn f216(v0: i64, v1: i64) -> R<i64> {
    let a713 = v0;
    let a714 = {
        let a712 = v1;
        int_neg(a712)?
    };
    int_sub(a713, a714)
}

pub fn f217(v0: List<u64>) -> R<Result<Adt6, Adt104>> {
    let m715 = f265(v0.clone())?;
    match m715 {
        Adt119::C0 => {
            f266(f267(), 0u64, v0)
        }
        Adt119::C1(v1) => {
            Ok(Err::<Adt6, Adt104>(Adt104::C0(v1)))
        }
        Adt119::C2(v2) => {
            Ok(Err::<Adt6, Adt104>(Adt104::C1(v2)))
        }
    }
}

pub fn f218(v0: Adt6) -> Option<Adt5> {
    let m716 = v0;
    match m716 {
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

pub fn f219(v0: Adt6, v1: Adt6) -> R<Adt116> {
    f268(f269(), List::cons((v0, v1), List::<(Adt6, Adt6)>::nil()))
}

pub fn f220() -> u64 {
    403u64
}

pub fn f221(v0: Option<Adt105>, v1: Adt82) -> List<(Str, Adt6)> {
    List::cons((Str::lit("jsonrpc"), Adt6::C3(f270())), {
        let a717 = f271(v0);
        let a718 = List::cons((Str::lit("error"), f272(v1)), List::<(Str, Adt6)>::nil());
        append_list(a717, a718)
    })
}

pub fn f222() -> i64 {
    -32000i64
}

pub fn f223(v0: Adt98) -> Str {
    let m719 = v0;
    match m719 {
        Adt98::C0 => {
            Str::lit("Forbidden: invalid Origin")
        }
        Adt98::C1 => {
            Str::lit("Forbidden: invalid Host")
        }
    }
}

pub fn f224(p726: List<u64>, p727: List<u64>) -> bool {
    let m725 = (p726, p727);
    {
        let mut m725 = m725;
        loop {
            let step = {
                let (v0, v1) = m725;
                let m720 = v0;
                match m720.uncons() {
                    None => {
                        let m721 = v1;
                        match m721.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m722 = v1;
                        match m722.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                            Some((v6, v7)) => {
                                let a723 = v4;
                                let a724 = v6;
                                if nat_eq(a723, a724) {
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
                Ok(next) => { m725 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f225() -> List<u64> {
    List::cons(80u64, List::cons(79u64, List::cons(83u64, List::cons(84u64, List::<u64>::nil()))))
}

pub fn f226() -> u64 {
    400u64
}

pub fn f227(v0: Adt109) -> Adt82 {
    Adt82::C0(f273(v0.clone()), f274(v0), None::<Adt6>)
}

pub fn f228(v0: Adt0, v1: Fn2, v2: u64, v3: List<Adt92>, v4: Result<Adt117, Adt109>) -> R<Adt88> {
    let m728 = v4;
    match m728 {
        Ok(v6) => {
            f277(v0, v1, v2, v3, v6)
        }
        Err(v5) => {
            Ok(Adt88::C0(f43(f176(f275(v5.clone()), f178(f276(v5.clone()), f227(v5)))?)))
        }
    }
}

pub fn f229(v0: Adt6) -> R<Result<Adt117, Adt109>> {
    let m729 = v0;
    match m729 {
        Adt6::C0 => {
            Ok(f279(None::<Adt105>, Adt110::C0))
        }
        Adt6::C1(_) => {
            Ok(f279(None::<Adt105>, Adt110::C0))
        }
        Adt6::C2(_) => {
            Ok(f279(None::<Adt105>, Adt110::C0))
        }
        Adt6::C3(_) => {
            Ok(f279(None::<Adt105>, Adt110::C0))
        }
        Adt6::C4(_) => {
            Ok(f279(None::<Adt105>, Adt110::C1))
        }
        Adt6::C5(v1) => {
            f278(Adt5::C0(v1))
        }
    }
}

pub fn f230() -> u64 {
    405u64
}

pub fn f231() -> List<u64> {
    List::cons(65u64, List::cons(108u64, List::cons(108u64, List::cons(111u64, List::cons(119u64, List::<u64>::nil())))))
}

pub fn f232() -> List<u64> {
    List::cons(80u64, List::cons(79u64, List::cons(83u64, List::cons(84u64, List::<u64>::nil()))))
}

pub fn f233(p734: List<Adt92>, v1: List<u64>, p735: Adt112) -> R<Adt112> {
    let m733 = (p734, p735);
    {
        let mut m733 = m733;
        loop {
            let step = {
                let (v0, v2) = m733;
                let m730 = v0;
                match m730.uncons() {
                    None => {
                        Err::<(List<Adt92>, Adt112), R<Adt112>>(Ok(v2))
                    }
                    Some((v3, v4)) => {
                        if f280(match v3.clone() {
                            Adt92::C0(h731, _) => {
                                h731
                            }
                        }, v1.clone())? {
                            Ok::<(List<Adt92>, Adt112), R<Adt112>>((v4, f281(v2, match v3 {
                                Adt92::C0(_, h732) => {
                                    h732
                                }
                            })))
                        } else {
                            Ok::<(List<Adt92>, Adt112), R<Adt112>>((v4, v2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m733 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f234(v0: List<u64>) -> Option<List<u64>> {
    let v1: List<u64> = f282(v0);
    if f283(v1.clone()) {
        Some(v1)
    } else {
        None::<List<u64>>
    }
}

pub fn f235(v0: List<u64>, v1: List<List<u64>>) -> R<bool> {
    if f280(v0.clone(), f284())? {
        Ok(true)
    } else {
        if f280(v0.clone(), f285())? {
            Ok(true)
        } else {
            if f280(v0.clone(), f286())? {
                Ok(true)
            } else {
                f287(v0, v1)
            }
        }
    }
}

pub fn f236(p740: List<u64>) -> Option<List<u64>> {
    let m739 = p740;
    {
        let mut m739 = m739;
        loop {
            let step = {
                let v0 = m739;
                let m736 = v0;
                match m736.uncons() {
                    None => {
                        Err::<List<u64>, Option<List<u64>>>(None::<List<u64>>)
                    }
                    Some((v1, v2)) => {
                        let a737 = v1;
                        let a738 = 58u64;
                        if nat_eq(a737, a738) {
                            Err::<List<u64>, Option<List<u64>>>(f288(v2))
                        } else {
                            Ok::<List<u64>, Option<List<u64>>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m739 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f237(v0: Option<(List<u64>, List<u64>)>) -> Option<List<u64>> {
    let m741 = v0;
    match m741 {
        None => {
            None::<List<u64>>
        }
        Some(v1) => {
            if f289({
                let (_, h742) = v1;
                h742
            }) {
                Some({
                    let (h743, _) = v1;
                    h743
                })
            } else {
                None::<List<u64>>
            }
        }
    }
}

pub fn f238(p748: List<u64>, p749: List<u64>) -> Option<(List<u64>, List<u64>)> {
    let m747 = (p748, p749);
    {
        let mut m747 = m747;
        loop {
            let step = {
                let (v0, v1) = m747;
                let m744 = v0;
                match m744.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>(None::<(List<u64>, List<u64>)>)
                    }
                    Some((v2, v3)) => {
                        let a745 = v2;
                        let a746 = 93u64;
                        if nat_eq(a745, a746) {
                            Err::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>(Some((f290(List::cons(v2, v1), List::<u64>::nil()), v3)))
                        } else {
                            Ok::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>((v3, List::cons(v2, v1)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m747 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f239(p754: List<u64>, p755: List<u64>) -> Option<(List<u64>, List<u64>)> {
    let m753 = (p754, p755);
    {
        let mut m753 = m753;
        loop {
            let step = {
                let (v0, v1) = m753;
                let m750 = v0;
                match m750.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>(Some((f290(v1, List::<u64>::nil()), List::<u64>::nil())))
                    }
                    Some((v2, v3)) => {
                        let a751 = v2;
                        let a752 = 58u64;
                        if nat_eq(a751, a752) {
                            Err::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>(Some((f290(v1, List::<u64>::nil()), List::cons(v2, v3))))
                        } else {
                            Ok::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>((v3, List::cons(v2, v1)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m753 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f240(v0: Bytes, p761: u64, p762: u64, p763: List<List<u64>>) -> R<Result<(List<List<u64>>, u64), Adt90>> {
    let m760 = (p761, (p762, p763));
    {
        let mut m760 = m760;
        loop {
            let step = {
                let (v1, (v2, v3)) = m760;
                let m756 = v2;
                if m756 == 0 {
                    Err::<(u64, (u64, List<List<u64>>)), R<Result<(List<List<u64>>, u64), Adt90>>>(Ok(Ok::<(List<List<u64>>, u64), Adt90>((f118(v3, List::<List<u64>>::nil()), v1))))
                } else {
                    let v4 = m756 - 1;
                    let m757 = f42(v0.clone(), v1)?;
                    match m757 {
                        None => {
                            Err::<(u64, (u64, List<List<u64>>)), R<Result<(List<List<u64>>, u64), Adt90>>>(Ok(Err::<(List<List<u64>>, u64), Adt90>(Adt90::C0(v1))))
                        }
                        Some(v5) => {
                            Ok::<(u64, (u64, List<List<u64>>)), R<Result<(List<List<u64>>, u64), Adt90>>>((match v5.clone() {
                                Adt94::C0(_, h758) => {
                                    h758
                                }
                            }, (v4, List::cons(f18(match v5 {
                                Adt94::C0(h759, _) => {
                                    h759
                                }
                            })?, v3))))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m760 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f241(v0: Adt91, v1: List<List<u64>>) -> Adt91 {
    Adt91::C0(match v0.clone() {
        Adt91::C0(h764, _, _, _, _, _, _) => {
            h764
        }
    }, match v0.clone() {
        Adt91::C0(_, h765, _, _, _, _, _) => {
            h765
        }
    }, match v0.clone() {
        Adt91::C0(_, _, h766, _, _, _, _) => {
            h766
        }
    }, match v0.clone() {
        Adt91::C0(_, _, _, h767, _, _, _) => {
            h767
        }
    }, match v0.clone() {
        Adt91::C0(_, _, _, _, h768, _, _) => {
            h768
        }
    }, v1, match v0 {
        Adt91::C0(_, _, _, _, _, _, h769) => {
            h769
        }
    })
}

pub fn f242(p772: List<u64>, v1: bool) -> bool {
    let m771 = p772;
    {
        let mut m771 = m771;
        loop {
            let step = {
                let v0 = m771;
                let m770 = v0;
                match m770.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        if if v1 {
                            f291(v2)
                        } else {
                            f292(v2)
                        } {
                            Ok::<List<u64>, bool>(v3)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m771 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f243() -> u64 {
    64u64
}

pub fn f244(v0: Str, v1: List<(Str, Adt6)>) -> Option<Adt93> {
    let m773 = f206(Str::lit("type"), v1);
    match m773 {
        Adt108::C0 => {
            Some(Adt93::C3(v0))
        }
        Adt108::C1(v2) => {
            let m774 = f89(v2);
            match m774 {
                None => {
                    Some(Adt93::C3(v0))
                }
                Some(v3) => {
                    let a775 = v3;
                    let a776 = Str::lit("object");
                    if equal(a775, a776) {
                        None::<Adt93>
                    } else {
                        Some(Adt93::C3(v0))
                    }
                }
            }
        }
    }
}

pub fn f245(v0: Str, v1: List<(Str, Adt6)>) -> Option<Adt93> {
    let m777 = f206(Str::lit("$schema"), v1);
    match m777 {
        Adt108::C0 => {
            None::<Adt93>
        }
        Adt108::C1(v2) => {
            let m778 = f89(v2);
            match m778 {
                None => {
                    Some(Adt93::C2(v0))
                }
                Some(v3) => {
                    let a779 = v3;
                    let a780 = f293();
                    if equal(a779, a780) {
                        None::<Adt93>
                    } else {
                        Some(Adt93::C2(v0))
                    }
                }
            }
        }
    }
}

pub fn f246(v0: Adt37) -> R<Option<Adt93>> {
    let m782 = match v0.clone() {
        Adt37::C0(_, _, _, _, _, h781, _, _) => {
            h781
        }
    };
    match m782 {
        None => {
            Ok(None::<Adt93>)
        }
        Some(v1) => {
            let m785 = f245(match v0.clone() {
                Adt37::C0(h783, _, _, _, _, _, _, _) => {
                    h783
                }
            }, match v1.clone() {
                Adt5::C0(h784) => {
                    h784
                }
            });
            match m785 {
                None => {
                    let m789 = f196(f197(), match v0 {
                        Adt37::C0(h786, _, _, _, _, _, _, _) => {
                            h786
                        }
                    }, Adt6::C5(match v1.clone() {
                        Adt5::C0(h787) => {
                            h787
                        }
                    }), List::cons(Adt106::C0(Adt6::C5(match v1 {
                        Adt5::C0(h788) => {
                            h788
                        }
                    }), false, List::<Str>::nil()), List::<Adt106>::nil()), f198())?;
                    match m789 {
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

pub fn f247(v0: Str, v1: Adt6, v2: Adt106, v3: Adt113, v4: List<Adt106>) -> R<Result<(List<Adt106>, Adt113), Adt93>> {
    let m790 = v2;
    match m790 {
        Adt106::C0(v5, v6, v7) => {
            let m791 = f160(v5.clone());
            match m791 {
                None => {
                    let m792 = f218(v5);
                    match m792 {
                        None => {
                            Ok(Err::<(List<Adt106>, Adt113), Adt93>(Adt93::C5(v0, Str::lit("schema"))))
                        }
                        Some(v9) => {
                            f294(v0, v1, match v9 {
                                Adt5::C0(h793) => {
                                    h793
                                }
                            }, v6, v7, v3, v4)
                        }
                    }
                }
                Some(_) => {
                    Ok(Ok::<(List<Adt106>, Adt113), Adt93>((v4, v3)))
                }
            }
        }
    }
}

pub fn f248(v0: Adt6, v1: List<(Str, Adt6)>, v2: Adt6, v3: List<Adt100>) -> R<Adt114> {
    let m794 = f295(v1.clone(), v2.clone())?;
    match m794 {
        Adt114::C0(v4) => {
            Ok(Adt114::C0(v4))
        }
        Adt114::C1(v5) => {
            Ok(Adt114::C1(v5))
        }
        Adt114::C2(v6) => {
            Ok(Adt114::C2(v6))
        }
        Adt114::C3(_) => {
            f296(v0, v1, v2, v3)
        }
    }
}

pub fn f249(v0: List<Adt6>, v1: Adt6, v2: List<Adt100>, v3: u64, v4: List<Adt102>) -> Adt107 {
    let m795 = v0;
    match m795.uncons() {
        None => {
            let a796 = v3;
            let a797 = 1u64;
            if nat_eq(a796, a797) {
                Adt107::C0(Adt101::C0, v4)
            } else {
                Adt107::C0(Adt101::C1(v2, Str::lit("oneOf")), v4)
            }
        }
        Some((v5, v6)) => {
            Adt107::C0(Adt101::C0, List::cons(Adt102::C0(v5, v1.clone(), v2.clone()), List::cons(Adt102::C2(v6, v1, v2, v3), v4)))
        }
    }
}

pub fn f250(v0: Option<Adt6>, v1: Adt6, v2: List<Adt100>, v3: List<Adt102>) -> Adt107 {
    let m798 = v0;
    match m798 {
        None => {
            Adt107::C0(Adt101::C0, v3)
        }
        Some(v4) => {
            Adt107::C0(Adt101::C0, List::cons(Adt102::C0(v4, v1, v2), v3))
        }
    }
}

pub fn f251(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt100>) -> Adt114 {
    let m799 = f206(Str::lit("anyOf"), v0);
    match m799 {
        Adt108::C0 => {
            Adt114::C3(List::<Adt102>::nil())
        }
        Adt108::C1(v3) => {
            let m800 = f209(v3);
            match m800 {
                None => {
                    Adt114::C1(Str::lit("anyOf"))
                }
                Some(v4) => {
                    let m801 = v4;
                    match m801.uncons() {
                        None => {
                            Adt114::C1(Str::lit("anyOf"))
                        }
                        Some((v5, v6)) => {
                            Adt114::C3(List::cons(Adt102::C0(v5, v1.clone(), v2.clone()), List::cons(Adt102::C1(v6, v1, v2), List::<Adt102>::nil())))
                        }
                    }
                }
            }
        }
    }
}

pub fn f252(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt100>) -> Adt114 {
    let m802 = f206(Str::lit("oneOf"), v0);
    match m802 {
        Adt108::C0 => {
            Adt114::C3(List::<Adt102>::nil())
        }
        Adt108::C1(v3) => {
            let m803 = f209(v3);
            match m803 {
                None => {
                    Adt114::C1(Str::lit("oneOf"))
                }
                Some(v4) => {
                    let m804 = v4;
                    match m804.uncons() {
                        None => {
                            Adt114::C1(Str::lit("oneOf"))
                        }
                        Some((v5, v6)) => {
                            Adt114::C3(List::cons(Adt102::C0(v5, v1.clone(), v2.clone()), List::cons(Adt102::C2(v6, v1, v2, 0u64), List::<Adt102>::nil())))
                        }
                    }
                }
            }
        }
    }
}

pub fn f253(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt100>) -> Adt114 {
    let m805 = f206(Str::lit("not"), v0);
    match m805 {
        Adt108::C0 => {
            Adt114::C3(List::<Adt102>::nil())
        }
        Adt108::C1(v3) => {
            Adt114::C3(List::cons(Adt102::C0(v3, v1, v2.clone()), List::cons(Adt102::C3(v2), List::<Adt102>::nil())))
        }
    }
}

pub fn f254(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt100>) -> Adt114 {
    let m806 = f206(Str::lit("if"), v0.clone());
    match m806 {
        Adt108::C0 => {
            Adt114::C3(List::<Adt102>::nil())
        }
        Adt108::C1(v3) => {
            Adt114::C3(List::cons(Adt102::C0(v3, v1.clone(), v2.clone()), List::cons(Adt102::C4(f297(Str::lit("then"), v0.clone()), f297(Str::lit("else"), v0), v1, v2), List::<Adt102>::nil())))
        }
    }
}

pub fn f255(v0: List<Adt6>, v1: Adt6, v2: List<Adt100>, v3: List<Adt102>) -> Adt107 {
    let m807 = v0;
    match m807.uncons() {
        None => {
            Adt107::C0(Adt101::C1(v2, Str::lit("anyOf")), v3)
        }
        Some((v4, v5)) => {
            Adt107::C0(Adt101::C0, List::cons(Adt102::C0(v4, v1.clone(), v2.clone()), List::cons(Adt102::C1(v5, v1, v2), v3)))
        }
    }
}

pub fn f256(v0: Adt100) -> R<Str> {
    let m808 = v0;
    match m808 {
        Adt100::C0(v1) => {
            f298(v1)
        }
        Adt100::C1(v2) => {
            let a809 = f167(v2, 0i64)?;
            Ok(format_i64(a809))
        }
    }
}

pub fn f257(v0: Str) -> R<bool> {
    let m811 = f299(f18({
        let a810 = v0;
        utf8_encode(a810)
    })?)?;
    match m811 {
        None => {
            Ok(false)
        }
        Some(_) => {
            Ok(true)
        }
    }
}

pub fn f258(v0: Adt39) -> bool {
    let m813 = match v0 {
        Adt39::C0(_, _, h812) => {
            h812
        }
    };
    match m813 {
        None => {
            false
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f259(v0: Adt51) -> R<Result<Adt34, Adt86>> {
    let m815 = match v0.clone() {
        Adt51::C0(_, h814) => {
            h814
        }
    };
    match m815.uncons() {
        None => {
            Ok(Ok::<Adt34, Adt86>(Adt34::C2(match v0 {
                Adt51::C0(h816, _) => {
                    h816
                }
            })))
        }
        Some((_, _)) => {
            if f300(match v0.clone() {
                Adt51::C0(_, h817) => {
                    h817
                }
            })? {
                Ok(Ok::<Adt34, Adt86>(Adt34::C3(v0)))
            } else {
                Ok(Err::<Adt34, Adt86>(Adt86::C6(Str::lit("progress did not increase"))))
            }
        }
    }
}

pub fn f260(v0: Adt5, v1: Adt51) -> R<Result<Adt34, Adt86>> {
    let m821 = {
        let m820 = match v1.clone() {
            Adt51::C0(h819, _) => {
                h819
            }
        };
        match m820 {
            Adt39::C0(_, h818, _) => {
                h818
            }
        }
    };
    match m821 {
        None => {
            Ok(Err::<Adt34, Adt86>(Adt86::C6(Str::lit("the tool declares an outputSchema and returned no structuredContent"))))
        }
        Some(v2) => {
            let m824 = f83(Adt6::C5(match v0.clone() {
                Adt5::C0(h822) => {
                    h822
                }
            }), Adt6::C5(match v0 {
                Adt5::C0(h823) => {
                    h823
                }
            }), v2)?;
            match m824 {
                Adt99::C0 => {
                    f259(v1)
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

pub fn f261(v0: Str, v1: Str) -> Adt118 {
    let a825 = v0;
    let a826 = v1;
    if equal(a825, a826) {
        Adt118::C0
    } else {
        Adt118::C1
    }
}

pub fn f262(p831: u64, p832: List<Adt115>, p833: List<u64>) -> R<Result<List<u64>, Adt104>> {
    let m830 = (p831, (p832, p833));
    {
        let mut m830 = m830;
        loop {
            let step = {
                let (v0, (v1, v2)) = m830;
                let m827 = v0;
                if m827 == 0 {
                    Err::<(u64, (List<Adt115>, List<u64>)), R<Result<List<u64>, Adt104>>>(Ok(Err::<List<u64>, Adt104>(Adt104::C13)))
                } else {
                    let v3 = m827 - 1;
                    let m828 = v1;
                    match m828.uncons() {
                        None => {
                            Err::<(u64, (List<Adt115>, List<u64>)), R<Result<List<u64>, Adt104>>>(Ok(Ok::<List<u64>, Adt104>(v2)))
                        }
                        Some((v4, v5)) => {
                            let m829 = f301(v4, v5, v2)?;
                            match m829 {
                                Adt133::C0(v6) => {
                                    Err::<(u64, (List<Adt115>, List<u64>)), R<Result<List<u64>, Adt104>>>(Ok(Err::<List<u64>, Adt104>(v6)))
                                }
                                Adt133::C1(v7, v8) => {
                                    Ok::<(u64, (List<Adt115>, List<u64>)), R<Result<List<u64>, Adt104>>>((v3, (v7, v8)))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m830 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f263(p836: List<u64>, p837: List<u64>) -> List<u64> {
    let m835 = (p836, p837);
    {
        let mut m835 = m835;
        loop {
            let step = {
                let (v0, v1) = m835;
                let m834 = v0;
                match m834.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m835 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f264() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f265(v0: List<u64>) -> R<Adt119> {
    Ok(f302({
        let a838 = v0.clone();
        length_list(a838)
    }, f303(Adt120::C0, 0u64, 0u64, v0)?))
}

pub fn f266(p845: Adt121, p846: u64, p847: List<u64>) -> R<Result<Adt6, Adt104>> {
    let m844 = (p845, (p846, p847));
    {
        let mut m844 = m844;
        loop {
            let step = {
                let (v0, (v1, v2)) = m844;
                let m839 = v2;
                match m839.uncons() {
                    None => {
                        let m840 = f304(v0, v1)?;
                        match m840 {
                            Ok(v3) => {
                                Err::<(Adt121, (u64, List<u64>)), R<Result<Adt6, Adt104>>>(Ok(Ok::<Adt6, Adt104>(v3)))
                            }
                            Err(v4) => {
                                Err::<(Adt121, (u64, List<u64>)), R<Result<Adt6, Adt104>>>(Ok(Err::<Adt6, Adt104>(v4)))
                            }
                        }
                    }
                    Some((v5, v6)) => {
                        let m841 = f305(v0, v1, v5)?;
                        match m841 {
                            Adt134::C0(v7) => {
                                Err::<(Adt121, (u64, List<u64>)), R<Result<Adt6, Adt104>>>(Ok(Err::<Adt6, Adt104>(v7)))
                            }
                            Adt134::C1(v8) => {
                                Ok::<(Adt121, (u64, List<u64>)), R<Result<Adt6, Adt104>>>((v8, ({
                                    let a842 = v1;
                                    let a843 = 1u64;
                                    nat_add(a842, a843)?
                                }, v6)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m844 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f267() -> Adt121 {
    Adt121::C0(Adt122::C0, List::<Adt126>::nil(), 0u64)
}

pub fn f268(p854: u64, p855: List<(Adt6, Adt6)>) -> R<Adt116> {
    let m853 = (p854, p855);
    {
        let mut m853 = m853;
        loop {
            let step = {
                let (v0, v1) = m853;
                let m848 = v0;
                if m848 == 0 {
                    Err::<(u64, List<(Adt6, Adt6)>), R<Adt116>>(Ok(Adt116::C2))
                } else {
                    let v2 = m848 - 1;
                    let m849 = v1;
                    match m849.uncons() {
                        None => {
                            Err::<(u64, List<(Adt6, Adt6)>), R<Adt116>>(Ok(Adt116::C0))
                        }
                        Some((v3, v4)) => {
                            let m852 = f306({
                                let (h850, _) = v3;
                                h850
                            }, {
                                let (_, h851) = v3;
                                h851
                            }, v4)?;
                            match m852 {
                                Adt135::C0 => {
                                    Err::<(u64, List<(Adt6, Adt6)>), R<Adt116>>(Ok(Adt116::C1))
                                }
                                Adt135::C1(v5) => {
                                    Ok::<(u64, List<(Adt6, Adt6)>), R<Adt116>>((v2, v5))
                                }
                            }
                        }
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

pub fn f269() -> u64 {
    67108864u64
}

pub fn f270() -> Str {
    Str::lit("2.0")
}

pub fn f271(v0: Option<Adt105>) -> List<(Str, Adt6)> {
    let m856 = v0;
    match m856 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("id"), f307(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f272(v0: Adt82) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("code"), Adt6::C2(Adt7::C0(match v0.clone() {
        Adt82::C0(h857, _, _) => {
            h857
        }
    }, 0i64))), List::cons((Str::lit("message"), Adt6::C3(match v0.clone() {
        Adt82::C0(_, h858, _) => {
            h858
        }
    })), f308(match v0 {
        Adt82::C0(_, _, h859) => {
            h859
        }
    }))))
}

pub fn f273(v0: Adt109) -> i64 {
    let m860 = v0;
    match m860 {
        Adt109::C0 => {
            f309()
        }
        Adt109::C1(_, _) => {
            f310()
        }
    }
}

pub fn f274(v0: Adt109) -> Str {
    let m861 = v0;
    match m861 {
        Adt109::C0 => {
            Str::lit("Parse error")
        }
        Adt109::C1(_, v2) => {
            f311(v2)
        }
    }
}

pub fn f275(v0: Adt109) -> u64 {
    let m862 = v0;
    match m862 {
        Adt109::C0 => {
            f226()
        }
        Adt109::C1(_, _) => {
            f226()
        }
    }
}

pub fn f276(v0: Adt109) -> Option<Adt105> {
    let m863 = v0;
    match m863 {
        Adt109::C0 => {
            None::<Adt105>
        }
        Adt109::C1(v1, _) => {
            v1
        }
    }
}

pub fn f277(v0: Adt0, v1: Fn2, v2: u64, v3: List<Adt92>, v4: Adt117) -> R<Adt88> {
    f312(f313(v3.clone())?, f314(v0.clone(), v1, v2, Some(f315(v3, match v0 {
        Adt0::C0(_, _, _, _, h864, _, _) => {
            h864
        }
    }, f316(v4.clone()), match v4.clone() {
        Adt117::C0(_, h865, _) => {
            h865
        }
    })?), f317(), v4)?)
}

pub fn f278(v0: Adt5) -> R<Result<Adt117, Adt109>> {
    let m866 = f318(v0.clone())?;
    match m866 {
        Adt143::C0 => {
            let m867 = f138(Str::lit("method"), v0.clone());
            match m867 {
                Adt108::C0 => {
                    Ok(f279(None::<Adt105>, f319(v0)))
                }
                Adt108::C1(_) => {
                    Ok(f279(None::<Adt105>, if f320(v0) {
                        Adt110::C8
                    } else {
                        Adt110::C7
                    }))
                }
            }
        }
        Adt143::C1(v2) => {
            Ok(f321(v2, v0))
        }
        Adt143::C2 => {
            Ok(f279(None::<Adt105>, Adt110::C5))
        }
    }
}

pub fn f279(v0: Option<Adt105>, v1: Adt110) -> Result<Adt117, Adt109> {
    Err::<Adt117, Adt109>(Adt109::C1(v0, v1))
}

pub fn f280(p874: List<u64>, p875: List<u64>) -> R<bool> {
    let m873 = (p874, p875);
    {
        let mut m873 = m873;
        loop {
            let step = {
                let (v0, v1) = m873;
                let m868 = v0;
                match m868.uncons() {
                    None => {
                        let m869 = v1;
                        match m869.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), R<bool>>(Ok(true))
                            }
                            Some((_, _)) => {
                                Err::<(List<u64>, List<u64>), R<bool>>(Ok(false))
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m870 = v1;
                        match m870.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), R<bool>>(Ok(false))
                            }
                            Some((v6, v7)) => {
                                let a871 = f322(v4)?;
                                let a872 = f322(v6)?;
                                if nat_eq(a871, a872) {
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
                Ok(next) => { m873 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f281(v0: Adt112, v1: List<u64>) -> Adt112 {
    let m876 = v0;
    match m876 {
        Adt112::C0 => {
            Adt112::C1(v1)
        }
        Adt112::C1(_) => {
            Adt112::C2
        }
        Adt112::C2 => {
            Adt112::C2
        }
    }
}

pub fn f282(v0: List<u64>) -> List<u64> {
    f290(f323(f290(f323(v0), List::<u64>::nil())), List::<u64>::nil())
}

pub fn f283(p879: List<u64>) -> bool {
    let m878 = p879;
    {
        let mut m878 = m878;
        loop {
            let step = {
                let v0 = m878;
                let m877 = v0;
                match m877.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f324(v1) {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m878 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f284() -> List<u64> {
    List::cons(108u64, List::cons(111u64, List::cons(99u64, List::cons(97u64, List::cons(108u64, List::cons(104u64, List::cons(111u64, List::cons(115u64, List::cons(116u64, List::<u64>::nil())))))))))
}

pub fn f285() -> List<u64> {
    List::cons(49u64, List::cons(50u64, List::cons(55u64, List::cons(46u64, List::cons(48u64, List::cons(46u64, List::cons(48u64, List::cons(46u64, List::cons(49u64, List::<u64>::nil())))))))))
}

pub fn f286() -> List<u64> {
    List::cons(91u64, List::cons(58u64, List::cons(58u64, List::cons(49u64, List::cons(93u64, List::<u64>::nil())))))
}

pub fn f287(v0: List<u64>, p882: List<List<u64>>) -> R<bool> {
    let m881 = p882;
    {
        let mut m881 = m881;
        loop {
            let step = {
                let v1 = m881;
                let m880 = v1;
                match m880.uncons() {
                    None => {
                        Err::<List<List<u64>>, R<bool>>(Ok(false))
                    }
                    Some((v2, v3)) => {
                        if f280(v0.clone(), v2)? {
                            Err::<List<List<u64>>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<List<u64>>, R<bool>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m881 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f288(v0: List<u64>) -> Option<List<u64>> {
    let m883 = v0;
    match m883.uncons() {
        None => {
            None::<List<u64>>
        }
        Some((v1, v2)) => {
            let m884 = v2;
            match m884.uncons() {
                None => {
                    None::<List<u64>>
                }
                Some((v3, v4)) => {
                    let a885 = v1;
                    let a886 = 47u64;
                    if if nat_eq(a885, a886) {
                        let a887 = v3;
                        let a888 = 47u64;
                        nat_eq(a887, a888)
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

pub fn f289(v0: List<u64>) -> bool {
    let m889 = v0;
    match m889.uncons() {
        None => {
            true
        }
        Some((v1, v2)) => {
            let a890 = v1;
            let a891 = 58u64;
            if nat_eq(a890, a891) {
                let m892 = v2.clone();
                match m892.uncons() {
                    None => {
                        false
                    }
                    Some((_, _)) => {
                        f325(v2)
                    }
                }
            } else {
                false
            }
        }
    }
}

pub fn f290(p895: List<u64>, p896: List<u64>) -> List<u64> {
    let m894 = (p895, p896);
    {
        let mut m894 = m894;
        loop {
            let step = {
                let (v0, v1) = m894;
                let m893 = v0;
                match m893.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m894 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f291(v0: u64) -> bool {
    let a897 = 48u64;
    let a898 = v0;
    if if nat_le(a897, a898) {
        let a899 = v0;
        let a900 = 57u64;
        nat_le(a899, a900)
    } else {
        false
    } {
        true
    } else {
        let a901 = 65u64;
        let a902 = v0;
        if if nat_le(a901, a902) {
            let a903 = v0;
            let a904 = 90u64;
            nat_le(a903, a904)
        } else {
            false
        } {
            true
        } else {
            let a905 = 97u64;
            let a906 = v0;
            if if nat_le(a905, a906) {
                let a907 = v0;
                let a908 = 122u64;
                nat_le(a907, a908)
            } else {
                false
            } {
                true
            } else {
                let a909 = v0;
                let a910 = 33u64;
                if nat_eq(a909, a910) {
                    true
                } else {
                    let a911 = v0;
                    let a912 = 35u64;
                    if nat_eq(a911, a912) {
                        true
                    } else {
                        let a913 = v0;
                        let a914 = 36u64;
                        if nat_eq(a913, a914) {
                            true
                        } else {
                            let a915 = v0;
                            let a916 = 37u64;
                            if nat_eq(a915, a916) {
                                true
                            } else {
                                let a917 = v0;
                                let a918 = 38u64;
                                if nat_eq(a917, a918) {
                                    true
                                } else {
                                    let a919 = v0;
                                    let a920 = 39u64;
                                    if nat_eq(a919, a920) {
                                        true
                                    } else {
                                        let a921 = v0;
                                        let a922 = 42u64;
                                        if nat_eq(a921, a922) {
                                            true
                                        } else {
                                            let a923 = v0;
                                            let a924 = 43u64;
                                            if nat_eq(a923, a924) {
                                                true
                                            } else {
                                                let a925 = v0;
                                                let a926 = 45u64;
                                                if nat_eq(a925, a926) {
                                                    true
                                                } else {
                                                    let a927 = v0;
                                                    let a928 = 46u64;
                                                    if nat_eq(a927, a928) {
                                                        true
                                                    } else {
                                                        let a929 = v0;
                                                        let a930 = 94u64;
                                                        if nat_eq(a929, a930) {
                                                            true
                                                        } else {
                                                            let a931 = v0;
                                                            let a932 = 95u64;
                                                            if nat_eq(a931, a932) {
                                                                true
                                                            } else {
                                                                let a933 = v0;
                                                                let a934 = 96u64;
                                                                if nat_eq(a933, a934) {
                                                                    true
                                                                } else {
                                                                    let a935 = v0;
                                                                    let a936 = 124u64;
                                                                    if nat_eq(a935, a936) {
                                                                        true
                                                                    } else {
                                                                        let a937 = v0;
                                                                        let a938 = 126u64;
                                                                        nat_eq(a937, a938)
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

pub fn f292(v0: u64) -> bool {
    let a939 = 48u64;
    let a940 = v0;
    if if nat_le(a939, a940) {
        let a941 = v0;
        let a942 = 57u64;
        nat_le(a941, a942)
    } else {
        false
    } {
        true
    } else {
        let a943 = 65u64;
        let a944 = v0;
        if if nat_le(a943, a944) {
            let a945 = v0;
            let a946 = 90u64;
            nat_le(a945, a946)
        } else {
            false
        } {
            true
        } else {
            let a947 = 97u64;
            let a948 = v0;
            if if nat_le(a947, a948) {
                let a949 = v0;
                let a950 = 122u64;
                nat_le(a949, a950)
            } else {
                false
            } {
                true
            } else {
                let a951 = v0;
                let a952 = 95u64;
                if nat_eq(a951, a952) {
                    true
                } else {
                    let a953 = v0;
                    let a954 = 45u64;
                    if nat_eq(a953, a954) {
                        true
                    } else {
                        let a955 = v0;
                        let a956 = 46u64;
                        nat_eq(a955, a956)
                    }
                }
            }
        }
    }
}

pub fn f293() -> Str {
    Str::lit("https://json-schema.org/draft/2020-12/schema")
}

pub fn f294(v0: Str, v1: Adt6, v2: List<(Str, Adt6)>, v3: bool, v4: List<Str>, v5: Adt113, v6: List<Adt106>) -> R<Result<(List<Adt106>, Adt113), Adt93>> {
    let m957 = f326(v0.clone(), v1, v2.clone());
    match m957 {
        None => {
            let m958 = f327(v0, v2.clone(), v3, v4.clone(), v5)?;
            match m958 {
                Adt144::C0(v8) => {
                    Ok(Err::<(List<Adt106>, Adt113), Adt93>(v8))
                }
                Adt144::C1(v9) => {
                    Ok(Ok::<(List<Adt106>, Adt113), Adt93>((f328(v2, v3, v4, v6), v9)))
                }
            }
        }
        Some(v7) => {
            Ok(Err::<(List<Adt106>, Adt113), Adt93>(v7))
        }
    }
}

pub fn f295(v0: List<(Str, Adt6)>, v1: Adt6) -> R<Adt114> {
    Ok(f329(List::cons((Str::lit("type"), f330(v0.clone(), v1.clone())?), List::cons((Str::lit("enum"), f331(v0.clone(), v1.clone())?), List::cons((Str::lit("const"), f332(v0.clone(), v1.clone())?), List::cons((Str::lit("minimum"), f333(Str::lit("minimum"), 0u64, v0.clone(), v1.clone())?), List::cons((Str::lit("maximum"), f333(Str::lit("maximum"), 1u64, v0.clone(), v1.clone())?), List::cons((Str::lit("exclusiveMinimum"), f333(Str::lit("exclusiveMinimum"), 2u64, v0.clone(), v1.clone())?), List::cons((Str::lit("exclusiveMaximum"), f333(Str::lit("exclusiveMaximum"), 3u64, v0.clone(), v1.clone())?), {
        let a959 = f334(v0.clone(), v1.clone())?;
        let a960 = List::cons((Str::lit("required"), f335(v0, v1)), List::<(Str, Adt132)>::nil());
        append_list(a959, a960)
    })))))))))
}

pub fn f296(v0: Adt6, v1: List<(Str, Adt6)>, v2: Adt6, v3: List<Adt100>) -> R<Adt114> {
    Ok(f336(f337(v0, v1.clone(), v2.clone(), v3.clone()), f336(f338(v1.clone(), v2.clone(), v3.clone()), f336(f339(v1.clone(), v2.clone(), v3.clone()), f336(f340(v1.clone(), v2.clone(), v3.clone())?, f336(f341(v1.clone(), v2.clone(), v3.clone()), f336(f342(Adt103::C0, f251(v1.clone(), v2.clone(), v3.clone()), v1.clone(), v2.clone(), v3.clone()), f336(f342(Adt103::C1, f252(v1.clone(), v2.clone(), v3.clone()), v1.clone(), v2.clone(), v3.clone()), f336(f342(Adt103::C2, f253(v1.clone(), v2.clone(), v3.clone()), v1.clone(), v2.clone(), v3.clone()), f342(Adt103::C3, f254(v1.clone(), v2.clone(), v3.clone()), v1, v2, v3))))))))))
}

pub fn f297(v0: Str, v1: List<(Str, Adt6)>) -> Option<Adt6> {
    let m961 = f206(v0, v1);
    match m961 {
        Adt108::C0 => {
            None::<Adt6>
        }
        Adt108::C1(v2) => {
            Some(v2)
        }
    }
}

pub fn f298(v0: Str) -> R<Str> {
    let m964 = {
        let a963 = f104(f105(), f263(f343(f18({
            let a962 = v0.clone();
            utf8_encode(a962)
        })?, List::<u64>::nil()), List::<u64>::nil()));
        utf8_decode(a963)
    };
    match m964 {
        None => {
            Ok(v0)
        }
        Some(v1) => {
            Ok(v1)
        }
    }
}

pub fn f299(v0: List<u64>) -> R<Option<List<u64>>> {
    let m966 = f344({
        let a965 = v0.clone();
        length_list(a965)
    }, v0, List::<u64>::nil())?;
    match m966 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v1) => {
            Ok(Some(f263(v1, List::<u64>::nil())))
        }
    }
}

pub fn f300(v0: List<Adt52>) -> R<bool> {
    let m967 = v0;
    match m967.uncons() {
        None => {
            Ok(true)
        }
        Some((v1, v2)) => {
            f345(v2, match v1 {
                Adt52::C0(h968, _, _) => {
                    h968
                }
            })
        }
    }
}

pub fn f301(v0: Adt115, v1: List<Adt115>, v2: List<u64>) -> R<Adt133> {
    let m969 = v0;
    match m969 {
        Adt115::C0(v3) => {
            Ok(Adt133::C1(v1, f263(v3, v2)))
        }
        Adt115::C1(v4, v5) => {
            f346(v4, v5, v1, v2)
        }
        Adt115::C2(v6, v7) => {
            let m970 = v7;
            match m970.uncons() {
                None => {
                    Ok(Adt133::C1(List::cons(Adt115::C0(List::cons(93u64, List::<u64>::nil())), v1), v2))
                }
                Some((v8, v9)) => {
                    Ok(Adt133::C1(List::cons(Adt115::C1(v6, v8), List::cons(Adt115::C3(v6, v9), v1)), v2))
                }
            }
        }
        Adt115::C3(v10, v11) => {
            let m971 = v11.clone();
            match m971.uncons() {
                None => {
                    Ok(Adt133::C1(List::cons(Adt115::C0(List::cons(93u64, List::<u64>::nil())), v1), v2))
                }
                Some((_, _)) => {
                    Ok(Adt133::C1(List::cons(Adt115::C0(List::cons(44u64, List::<u64>::nil())), List::cons(Adt115::C2(v10, v11), v1)), v2))
                }
            }
        }
        Adt115::C4(v14, v15) => {
            let m972 = v15;
            match m972.uncons() {
                None => {
                    Ok(Adt133::C1(List::cons(Adt115::C0(List::cons(125u64, List::<u64>::nil())), v1), v2))
                }
                Some((v16, v17)) => {
                    Ok(Adt133::C1(List::cons(Adt115::C0({
                        let a974 = f347({
                            let (h973, _) = v16;
                            h973
                        })?;
                        let a975 = List::cons(58u64, List::<u64>::nil());
                        append_list(a974, a975)
                    }), List::cons(Adt115::C1(v14, {
                        let (_, h976) = v16;
                        h976
                    }), List::cons(Adt115::C5(v14, v17), v1))), v2))
                }
            }
        }
        Adt115::C5(v18, v19) => {
            let m977 = v19.clone();
            match m977.uncons() {
                None => {
                    Ok(Adt133::C1(List::cons(Adt115::C0(List::cons(125u64, List::<u64>::nil())), v1), v2))
                }
                Some((_, _)) => {
                    Ok(Adt133::C1(List::cons(Adt115::C0(List::cons(44u64, List::<u64>::nil())), List::cons(Adt115::C4(v18, v19), v1)), v2))
                }
            }
        }
    }
}

pub fn f302(v0: u64, v1: u64) -> Adt119 {
    let a978 = v1;
    let a979 = 0u64;
    if nat_eq(a978, a979) {
        Adt119::C0
    } else {
        let a980 = v1;
        let a981 = 2u64;
        if nat_eq(a980, a981) {
            Adt119::C2(v0)
        } else {
            Adt119::C1({
                let a985 = v0;
                let a986 = {
                    let a982 = v1;
                    let a983 = 3u64;
                    let a984 = 0u64;
                    nat_quot(a982, a983, a984)
                };
                nat_sub(a985, a986)
            })
        }
    }
}

pub fn f303(p1014: Adt120, p1015: u64, p1016: u64, p1017: List<u64>) -> R<u64> {
    let m1013 = (p1014, (p1015, (p1016, p1017)));
    {
        let mut m1013 = m1013;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m1013;
                let m987 = v3;
                match m987.uncons() {
                    None => {
                        let m988 = v0;
                        match m988 {
                            Adt120::C0 => {
                                Err::<(Adt120, (u64, (u64, List<u64>))), R<u64>>(Ok(0u64))
                            }
                            Adt120::C1 => {
                                Err::<(Adt120, (u64, (u64, List<u64>))), R<u64>>(Ok(2u64))
                            }
                            Adt120::C2 => {
                                Err::<(Adt120, (u64, (u64, List<u64>))), R<u64>>(Ok(2u64))
                            }
                            Adt120::C3 => {
                                Err::<(Adt120, (u64, (u64, List<u64>))), R<u64>>(Ok(2u64))
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m989 = v0;
                        match m989 {
                            Adt120::C0 => {
                                let a990 = v4;
                                let a991 = 128u64;
                                if nat_lt(a990, a991) {
                                    Ok::<(Adt120, (u64, (u64, List<u64>))), R<u64>>((Adt120::C0, (0u64, (0u64, v5))))
                                } else {
                                    let a992 = v4;
                                    let a993 = 194u64;
                                    if nat_lt(a992, a993) {
                                        let v6: u64 = f348({
                                            let a994 = List::cons(v4, v5);
                                            length_list(a994)
                                        })?;
                                        Err::<(Adt120, (u64, (u64, List<u64>))), R<u64>>(Ok(v6))
                                    } else {
                                        let a995 = v4;
                                        let a996 = 224u64;
                                        if nat_lt(a995, a996) {
                                            Ok::<(Adt120, (u64, (u64, List<u64>))), R<u64>>((Adt120::C1, (128u64, (191u64, v5))))
                                        } else {
                                            let a997 = v4;
                                            let a998 = 224u64;
                                            if nat_eq(a997, a998) {
                                                Ok::<(Adt120, (u64, (u64, List<u64>))), R<u64>>((Adt120::C2, (160u64, (191u64, v5))))
                                            } else {
                                                let a999 = v4;
                                                let a1000 = 237u64;
                                                if nat_eq(a999, a1000) {
                                                    Ok::<(Adt120, (u64, (u64, List<u64>))), R<u64>>((Adt120::C2, (128u64, (159u64, v5))))
                                                } else {
                                                    let a1001 = v4;
                                                    let a1002 = 240u64;
                                                    if nat_lt(a1001, a1002) {
                                                        Ok::<(Adt120, (u64, (u64, List<u64>))), R<u64>>((Adt120::C2, (128u64, (191u64, v5))))
                                                    } else {
                                                        let a1003 = v4;
                                                        let a1004 = 240u64;
                                                        if nat_eq(a1003, a1004) {
                                                            Ok::<(Adt120, (u64, (u64, List<u64>))), R<u64>>((Adt120::C3, (144u64, (191u64, v5))))
                                                        } else {
                                                            let a1005 = v4;
                                                            let a1006 = 244u64;
                                                            if nat_eq(a1005, a1006) {
                                                                Ok::<(Adt120, (u64, (u64, List<u64>))), R<u64>>((Adt120::C3, (128u64, (143u64, v5))))
                                                            } else {
                                                                let a1007 = v4;
                                                                let a1008 = 244u64;
                                                                if nat_lt(a1007, a1008) {
                                                                    Ok::<(Adt120, (u64, (u64, List<u64>))), R<u64>>((Adt120::C3, (128u64, (191u64, v5))))
                                                                } else {
                                                                    let v7: u64 = f348({
                                                                        let a1009 = List::cons(v4, v5);
                                                                        length_list(a1009)
                                                                    })?;
                                                                    Err::<(Adt120, (u64, (u64, List<u64>))), R<u64>>(Ok(v7))
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
                            Adt120::C1 => {
                                if f349(v4, v1, v2) {
                                    Ok::<(Adt120, (u64, (u64, List<u64>))), R<u64>>((Adt120::C0, (0u64, (0u64, v5))))
                                } else {
                                    let v8: u64 = f348({
                                        let a1010 = List::cons(v4, v5);
                                        length_list(a1010)
                                    })?;
                                    Err::<(Adt120, (u64, (u64, List<u64>))), R<u64>>(Ok(v8))
                                }
                            }
                            Adt120::C2 => {
                                if f349(v4, v1, v2) {
                                    Ok::<(Adt120, (u64, (u64, List<u64>))), R<u64>>((Adt120::C1, (128u64, (191u64, v5))))
                                } else {
                                    let v9: u64 = f348({
                                        let a1011 = List::cons(v4, v5);
                                        length_list(a1011)
                                    })?;
                                    Err::<(Adt120, (u64, (u64, List<u64>))), R<u64>>(Ok(v9))
                                }
                            }
                            Adt120::C3 => {
                                if f349(v4, v1, v2) {
                                    Ok::<(Adt120, (u64, (u64, List<u64>))), R<u64>>((Adt120::C2, (128u64, (191u64, v5))))
                                } else {
                                    let v10: u64 = f348({
                                        let a1012 = List::cons(v4, v5);
                                        length_list(a1012)
                                    })?;
                                    Err::<(Adt120, (u64, (u64, List<u64>))), R<u64>>(Ok(v10))
                                }
                            }
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

pub fn f304(v0: Adt121, v1: u64) -> R<Result<Adt6, Adt104>> {
    let m1019 = match v0.clone() {
        Adt121::C0(h1018, _, _) => {
            h1018
        }
    };
    match m1019 {
        Adt122::C0 => {
            Ok(Err::<Adt6, Adt104>(Adt104::C3))
        }
        Adt122::C1 => {
            Ok(Err::<Adt6, Adt104>(Adt104::C3))
        }
        Adt122::C2 => {
            Ok(Err::<Adt6, Adt104>(Adt104::C3))
        }
        Adt122::C3 => {
            Ok(Err::<Adt6, Adt104>(Adt104::C3))
        }
        Adt122::C4 => {
            Ok(Err::<Adt6, Adt104>(Adt104::C3))
        }
        Adt122::C5 => {
            Ok(Err::<Adt6, Adt104>(Adt104::C3))
        }
        Adt122::C6(_, _, _) => {
            Ok(Err::<Adt6, Adt104>(Adt104::C3))
        }
        Adt122::C7(_, _) => {
            Ok(Err::<Adt6, Adt104>(Adt104::C3))
        }
        Adt122::C8(v3) => {
            let m1020 = f350(v3)?;
            match m1020 {
                Adt145::C0 => {
                    Ok(Err::<Adt6, Adt104>(Adt104::C7(v1)))
                }
                Adt145::C1 => {
                    Ok(Err::<Adt6, Adt104>(Adt104::C8(v1)))
                }
                Adt145::C2(v4) => {
                    let m1025 = {
                        let m1024 = f351(v4, match v0.clone() {
                            Adt121::C0(_, h1022, _) => {
                                h1022
                            }
                        }, match v0 {
                            Adt121::C0(_, _, h1023) => {
                                h1023
                            }
                        });
                        match m1024 {
                            Adt121::C0(h1021, _, _) => {
                                h1021
                            }
                        }
                    };
                    match m1025 {
                        Adt122::C0 => {
                            Ok(Err::<Adt6, Adt104>(Adt104::C3))
                        }
                        Adt122::C1 => {
                            Ok(Err::<Adt6, Adt104>(Adt104::C3))
                        }
                        Adt122::C2 => {
                            Ok(Err::<Adt6, Adt104>(Adt104::C3))
                        }
                        Adt122::C3 => {
                            Ok(Err::<Adt6, Adt104>(Adt104::C3))
                        }
                        Adt122::C4 => {
                            Ok(Err::<Adt6, Adt104>(Adt104::C3))
                        }
                        Adt122::C5 => {
                            Ok(Err::<Adt6, Adt104>(Adt104::C3))
                        }
                        Adt122::C6(_, _, _) => {
                            Ok(Err::<Adt6, Adt104>(Adt104::C3))
                        }
                        Adt122::C7(_, _) => {
                            Ok(Err::<Adt6, Adt104>(Adt104::C3))
                        }
                        Adt122::C8(_) => {
                            Ok(Err::<Adt6, Adt104>(Adt104::C3))
                        }
                        Adt122::C9(v5) => {
                            Ok(Ok::<Adt6, Adt104>(v5))
                        }
                    }
                }
            }
        }
        Adt122::C9(v2) => {
            Ok(Ok::<Adt6, Adt104>(v2))
        }
    }
}

pub fn f305(v0: Adt121, v1: u64, v2: u64) -> R<Adt134> {
    let m1027 = match v0.clone() {
        Adt121::C0(h1026, _, _) => {
            h1026
        }
    };
    match m1027 {
        Adt122::C0 => {
            if f352(v2) {
                Ok(Adt134::C1(v0))
            } else {
                f353(v0, v1, v2)
            }
        }
        Adt122::C1 => {
            if f352(v2) {
                Ok(Adt134::C1(v0))
            } else {
                let a1028 = v2;
                let a1029 = 93u64;
                if nat_eq(a1028, a1029) {
                    Ok(f354(v0, v1))
                } else {
                    f353(v0, v1, v2)
                }
            }
        }
        Adt122::C2 => {
            Ok(f355(v0, v1, v2))
        }
        Adt122::C3 => {
            if f352(v2) {
                Ok(Adt134::C1(v0))
            } else {
                let a1030 = v2;
                let a1031 = 125u64;
                if nat_eq(a1030, a1031) {
                    Ok(f356(v0, v1))
                } else {
                    let a1032 = v2;
                    let a1033 = 34u64;
                    if nat_eq(a1032, a1033) {
                        Ok(f357(v0, Adt122::C6(Adt123::C0, List::<u64>::nil(), true)))
                    } else {
                        Ok(f358(Adt104::C2(v1)))
                    }
                }
            }
        }
        Adt122::C4 => {
            if f352(v2) {
                Ok(Adt134::C1(v0))
            } else {
                let a1034 = v2;
                let a1035 = 34u64;
                if nat_eq(a1034, a1035) {
                    Ok(f357(v0, Adt122::C6(Adt123::C0, List::<u64>::nil(), true)))
                } else {
                    Ok(f358(Adt104::C2(v1)))
                }
            }
        }
        Adt122::C5 => {
            if f352(v2) {
                Ok(Adt134::C1(v0))
            } else {
                let a1036 = v2;
                let a1037 = 58u64;
                if nat_eq(a1036, a1037) {
                    Ok(f357(v0, Adt122::C0))
                } else {
                    Ok(f358(Adt104::C2(v1)))
                }
            }
        }
        Adt122::C6(v3, v4, v5) => {
            f359(v0, v1, v2, v3, v4, v5)
        }
        Adt122::C7(v6, v7) => {
            f360(v0, v1, v2, v6, v7)
        }
        Adt122::C8(v8) => {
            f361(v0, v1, v2, v8)
        }
        Adt122::C9(_) => {
            Ok(f355(v0, v1, v2))
        }
    }
}

pub fn f306(v0: Adt6, v1: Adt6, v2: List<(Adt6, Adt6)>) -> R<Adt135> {
    let a1038 = f362(v0.clone());
    let a1039 = f362(v1.clone());
    if nat_eq(a1038, a1039) {
        let m1040 = v0;
        match m1040 {
            Adt6::C0 => {
                Ok(Adt135::C1(v2))
            }
            Adt6::C1(v3) => {
                let m1041 = f160(v1);
                match m1041 {
                    None => {
                        Ok(Adt135::C0)
                    }
                    Some(v4) => {
                        let a1042 = v3;
                        let a1043 = v4;
                        if equal(a1042, a1043) {
                            Ok(Adt135::C1(v2))
                        } else {
                            Ok(Adt135::C0)
                        }
                    }
                }
            }
            Adt6::C2(v5) => {
                let m1044 = f363(v1);
                match m1044 {
                    None => {
                        Ok(Adt135::C0)
                    }
                    Some(v6) => {
                        if f364(f365(v5, v6)?) {
                            Ok(Adt135::C1(v2))
                        } else {
                            Ok(Adt135::C0)
                        }
                    }
                }
            }
            Adt6::C3(v7) => {
                let m1045 = f89(v1);
                match m1045 {
                    None => {
                        Ok(Adt135::C0)
                    }
                    Some(v8) => {
                        let a1046 = v7;
                        let a1047 = v8;
                        if equal(a1046, a1047) {
                            Ok(Adt135::C1(v2))
                        } else {
                            Ok(Adt135::C0)
                        }
                    }
                }
            }
            Adt6::C4(v9) => {
                let m1048 = f209(v1);
                match m1048 {
                    None => {
                        Ok(Adt135::C0)
                    }
                    Some(v10) => {
                        Ok(f366(f367(v9, v10, List::<(Adt6, Adt6)>::nil()), v2))
                    }
                }
            }
            Adt6::C5(v11) => {
                let m1049 = f218(v1);
                match m1049 {
                    None => {
                        Ok(Adt135::C0)
                    }
                    Some(v12) => {
                        let a1051 = f368(v11.clone(), 0u64)?;
                        let a1052 = f368(match v12.clone() {
                            Adt5::C0(h1050) => {
                                h1050
                            }
                        }, 0u64)?;
                        if nat_eq(a1051, a1052) {
                            Ok(f366(f369(v11, match v12 {
                                Adt5::C0(h1053) => {
                                    h1053
                                }
                            }, List::<(Adt6, Adt6)>::nil()), v2))
                        } else {
                            Ok(Adt135::C0)
                        }
                    }
                }
            }
        }
    } else {
        Ok(Adt135::C0)
    }
}

pub fn f307(v0: Adt105) -> Adt6 {
    let m1054 = v0;
    match m1054 {
        Adt105::C0(v1) => {
            Adt6::C3(v1)
        }
        Adt105::C1(v2) => {
            Adt6::C2(Adt7::C0(v2, 0i64))
        }
    }
}

pub fn f308(v0: Option<Adt6>) -> List<(Str, Adt6)> {
    let m1055 = v0;
    match m1055 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("data"), v1), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f309() -> i64 {
    -32700i64
}

pub fn f310() -> i64 {
    -32600i64
}

pub fn f311(v0: Adt110) -> Str {
    let m1056 = v0;
    match m1056 {
        Adt110::C0 => {
            Str::lit("Invalid Request: the body is not a JSON object")
        }
        Adt110::C1 => {
            Str::lit("Invalid Request: batches are not supported")
        }
        Adt110::C2 => {
            Str::lit("Invalid Request: jsonrpc must be 2.0")
        }
        Adt110::C3 => {
            Str::lit("Invalid Request: method is required")
        }
        Adt110::C4 => {
            Str::lit("Invalid Request: method must be a string")
        }
        Adt110::C5 => {
            Str::lit("Invalid Request: ident must be a string or an integer")
        }
        Adt110::C6 => {
            Str::lit("Invalid Request: params must be an object")
        }
        Adt110::C7 => {
            Str::lit("Invalid Request: notifications are not accepted")
        }
        Adt110::C8 => {
            Str::lit("Invalid Request: the client must not send responses")
        }
    }
}

pub fn f312(v0: bool, v1: Adt136) -> R<Adt88> {
    let m1058 = f370(match v1.clone() {
        Adt136::C0(_, h1057) => {
            h1057
        }
    });
    match m1058 {
        None => {
            Ok(Adt88::C0(f43(f372(v0, v1)?)))
        }
        Some(v2) => {
            f371(v0, v1, v2)
        }
    }
}

pub fn f313(p1063: List<Adt92>) -> R<bool> {
    let m1062 = p1063;
    {
        let mut m1062 = m1062;
        loop {
            let step = {
                let v0 = m1062;
                let m1059 = v0;
                match m1059.uncons() {
                    None => {
                        Err::<List<Adt92>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        if f280(match v1.clone() {
                            Adt92::C0(h1060, _) => {
                                h1060
                            }
                        }, f373())? {
                            if f374(f375(44u64, match v1 {
                                Adt92::C0(_, h1061) => {
                                    h1061
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
                Ok(next) => { m1062 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f314(v0: Adt0, v1: Fn2, v2: u64, v3: Option<Adt127>, v4: List<Str>, v5: Adt117) -> R<Adt136> {
    Ok(Adt136::C0(match v5.clone() {
        Adt117::C0(h1064, _, _) => {
            h1064
        }
    }, f376(v0, v1, v2, f377(v3, v4, v5)?)?))
}

pub fn f315(v0: List<Adt92>, v1: List<Adt12>, v2: Option<Str>, v3: Str) -> R<Adt127> {
    let v4: Adt142 = f378(Adt129::C0, f184(v0.clone(), f379())?);
    let v5: Adt142 = f378(Adt129::C1, f184(v0.clone(), f380())?);
    let v6: Adt142 = if f381(v3) {
        f378(Adt129::C2, f382(f184(v0.clone(), f383())?)?)
    } else {
        f384()
    };
    Ok(Adt127::C0(match v4.clone() {
        Adt142::C0(h1065, _) => {
            h1065
        }
    }, match v5.clone() {
        Adt142::C0(h1066, _) => {
            h1066
        }
    }, match v6.clone() {
        Adt142::C0(h1067, _) => {
            h1067
        }
    }, f385(match v4 {
        Adt142::C0(_, h1068) => {
            h1068
        }
    }, f385(match v5 {
        Adt142::C0(_, h1069) => {
            h1069
        }
    }, match v6 {
        Adt142::C0(_, h1070) => {
            h1070
        }
    })), f386(v0, v1, v2)?))
}

pub fn f316(v0: Adt117) -> Option<Str> {
    let a1072 = match v0.clone() {
        Adt117::C0(_, h1071, _) => {
            h1071
        }
    };
    let a1073 = Str::lit("tools/call");
    if equal(a1072, a1073) {
        let m1075 = match v0 {
            Adt117::C0(_, _, h1074) => {
                h1074
            }
        };
        match m1075 {
            None => {
                None::<Str>
            }
            Some(v1) => {
                let m1076 = f138(Str::lit("name"), v1);
                match m1076 {
                    Adt108::C0 => {
                        None::<Str>
                    }
                    Adt108::C1(v2) => {
                        f89(v2)
                    }
                }
            }
        }
    } else {
        None::<Str>
    }
}

pub fn f317() -> List<Str> {
    List::cons(f387(), List::<Str>::nil())
}

pub fn f318(v0: Adt5) -> R<Adt143> {
    let m1077 = f138(Str::lit("id"), v0);
    match m1077 {
        Adt108::C0 => {
            Ok(Adt143::C0)
        }
        Adt108::C1(v1) => {
            let m1078 = v1;
            match m1078 {
                Adt6::C0 => {
                    Ok(Adt143::C2)
                }
                Adt6::C1(_) => {
                    Ok(Adt143::C2)
                }
                Adt6::C2(v3) => {
                    let m1079 = f388(v3)?;
                    match m1079 {
                        None => {
                            Ok(Adt143::C2)
                        }
                        Some(v4) => {
                            Ok(Adt143::C1(Adt105::C1(v4)))
                        }
                    }
                }
                Adt6::C3(v2) => {
                    Ok(Adt143::C1(Adt105::C0(v2)))
                }
                Adt6::C4(_) => {
                    Ok(Adt143::C2)
                }
                Adt6::C5(_) => {
                    Ok(Adt143::C2)
                }
            }
        }
    }
}

pub fn f319(v0: Adt5) -> Adt110 {
    if f320(v0) {
        Adt110::C8
    } else {
        Adt110::C3
    }
}

pub fn f320(v0: Adt5) -> bool {
    if f389(Str::lit("result"), v0.clone()) {
        true
    } else {
        f389(Str::lit("error"), v0)
    }
}

pub fn f321(v0: Adt105, v1: Adt5) -> Result<Adt117, Adt109> {
    let m1080 = f138(Str::lit("jsonrpc"), v1.clone());
    match m1080 {
        Adt108::C0 => {
            f279(Some(v0), Adt110::C2)
        }
        Adt108::C1(v2) => {
            let m1081 = f89(v2);
            match m1081 {
                None => {
                    f279(Some(v0), Adt110::C2)
                }
                Some(v3) => {
                    let a1082 = v3;
                    let a1083 = f270();
                    if equal(a1082, a1083) {
                        f390(v0, v1)
                    } else {
                        f279(Some(v0), Adt110::C2)
                    }
                }
            }
        }
    }
}

pub fn f322(v0: u64) -> R<u64> {
    let a1084 = 64u64;
    let a1085 = v0;
    if if nat_lt(a1084, a1085) {
        let a1086 = v0;
        let a1087 = 91u64;
        nat_lt(a1086, a1087)
    } else {
        false
    } {
        let a1088 = v0;
        let a1089 = 32u64;
        nat_add(a1088, a1089)
    } else {
        Ok(v0)
    }
}

pub fn f323(p1092: List<u64>) -> List<u64> {
    let m1091 = p1092;
    {
        let mut m1091 = m1091;
        loop {
            let step = {
                let v0 = m1091;
                let m1090 = v0;
                match m1090.uncons() {
                    None => {
                        Err::<List<u64>, List<u64>>(List::<u64>::nil())
                    }
                    Some((v1, v2)) => {
                        if f391(v1) {
                            Ok::<List<u64>, List<u64>>(v2)
                        } else {
                            Err::<List<u64>, List<u64>>(List::cons(v1, v2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1091 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f324(v0: u64) -> bool {
    let a1093 = v0;
    let a1094 = 9u64;
    if nat_eq(a1093, a1094) {
        true
    } else {
        let a1095 = 31u64;
        let a1096 = v0;
        if nat_lt(a1095, a1096) {
            let a1097 = v0;
            let a1098 = 127u64;
            nat_lt(a1097, a1098)
        } else {
            false
        }
    }
}

pub fn f325(p1101: List<u64>) -> bool {
    let m1100 = p1101;
    {
        let mut m1100 = m1100;
        loop {
            let step = {
                let v0 = m1100;
                let m1099 = v0;
                match m1099.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f392(v1) {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1100 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f326(v0: Str, v1: Adt6, v2: List<(Str, Adt6)>) -> Option<Adt93> {
    let m1102 = f393(f394(), v2.clone());
    match m1102 {
        None => {
            let m1103 = f395(f396(), v2.clone());
            match m1103 {
                None => {
                    f397(v0, v1, v2)
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

pub fn f327(v0: Str, v1: List<(Str, Adt6)>, v2: bool, v3: List<Str>, v4: Adt113) -> R<Adt144> {
    let m1104 = f206(Str::lit("x-mcp-header"), v1.clone());
    match m1104 {
        Adt108::C0 => {
            Ok(Adt144::C1(v4))
        }
        Adt108::C1(v5) => {
            f398(v0, v1, v5, v2, v3, v4)
        }
    }
}

pub fn f328(v0: List<(Str, Adt6)>, v1: bool, v2: List<Str>, v3: List<Adt106>) -> List<Adt106> {
    f399(Str::lit("properties"), v0.clone(), v1, v2, f399(Str::lit("$defs"), v0.clone(), false, List::<Str>::nil(), f400(Str::lit("items"), v0.clone(), f400(Str::lit("additionalProperties"), v0.clone(), f401(Str::lit("allOf"), v0.clone(), f401(Str::lit("anyOf"), v0.clone(), f401(Str::lit("oneOf"), v0.clone(), f400(Str::lit("not"), v0.clone(), f400(Str::lit("if"), v0.clone(), f400(Str::lit("then"), v0.clone(), f400(Str::lit("else"), v0, v3)))))))))))
}

pub fn f329(p1111: List<(Str, Adt132)>) -> Adt114 {
    let m1110 = p1111;
    {
        let mut m1110 = m1110;
        loop {
            let step = {
                let v0 = m1110;
                let m1105 = v0;
                match m1105.uncons() {
                    None => {
                        Err::<List<(Str, Adt132)>, Adt114>(Adt114::C3(List::<Adt102>::nil()))
                    }
                    Some((v1, v2)) => {
                        let m1107 = {
                            let (_, h1106) = v1;
                            h1106
                        };
                        match m1107 {
                            Adt132::C0 => {
                                Ok::<List<(Str, Adt132)>, Adt114>(v2)
                            }
                            Adt132::C1 => {
                                Err::<List<(Str, Adt132)>, Adt114>(Adt114::C0({
                                    let (h1108, _) = v1;
                                    h1108
                                }))
                            }
                            Adt132::C2 => {
                                Err::<List<(Str, Adt132)>, Adt114>(Adt114::C1({
                                    let (h1109, _) = v1;
                                    h1109
                                }))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1110 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f330(v0: List<(Str, Adt6)>, v1: Adt6) -> R<Adt132> {
    let m1112 = f206(Str::lit("type"), v0);
    match m1112 {
        Adt108::C0 => {
            Ok(Adt132::C0)
        }
        Adt108::C1(v2) => {
            let m1113 = f89(v2.clone());
            match m1113 {
                None => {
                    let m1114 = f209(v2);
                    match m1114 {
                        None => {
                            Ok(Adt132::C2)
                        }
                        Some(v4) => {
                            f404(v4, v1, false)
                        }
                    }
                }
                Some(v3) => {
                    if f402(v3.clone()) {
                        if f403(v3, v1)? {
                            Ok(Adt132::C0)
                        } else {
                            Ok(Adt132::C1)
                        }
                    } else {
                        Ok(Adt132::C2)
                    }
                }
            }
        }
    }
}

pub fn f331(v0: List<(Str, Adt6)>, v1: Adt6) -> R<Adt132> {
    let m1115 = f206(Str::lit("enum"), v0);
    match m1115 {
        Adt108::C0 => {
            Ok(Adt132::C0)
        }
        Adt108::C1(v2) => {
            let m1116 = f209(v2);
            match m1116 {
                None => {
                    Ok(Adt132::C2)
                }
                Some(v3) => {
                    f405(v3, v1)
                }
            }
        }
    }
}

pub fn f332(v0: List<(Str, Adt6)>, v1: Adt6) -> R<Adt132> {
    let m1117 = f206(Str::lit("const"), v0);
    match m1117 {
        Adt108::C0 => {
            Ok(Adt132::C0)
        }
        Adt108::C1(v2) => {
            let m1118 = f219(v2, v1)?;
            match m1118 {
                Adt116::C0 => {
                    Ok(Adt132::C0)
                }
                Adt116::C1 => {
                    Ok(Adt132::C1)
                }
                Adt116::C2 => {
                    Ok(Adt132::C2)
                }
            }
        }
    }
}

pub fn f333(v0: Str, v1: u64, v2: List<(Str, Adt6)>, v3: Adt6) -> R<Adt132> {
    let m1119 = f206(v0, v2);
    match m1119 {
        Adt108::C0 => {
            Ok(Adt132::C0)
        }
        Adt108::C1(v4) => {
            let m1120 = f363(v4);
            match m1120 {
                None => {
                    Ok(Adt132::C2)
                }
                Some(v5) => {
                    let m1121 = f363(v3);
                    match m1121 {
                        None => {
                            Ok(Adt132::C0)
                        }
                        Some(v6) => {
                            if f406(v1, f365(v6, v5)?) {
                                Ok(Adt132::C0)
                            } else {
                                Ok(Adt132::C1)
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f334(v0: List<(Str, Adt6)>, v1: Adt6) -> R<List<(Str, Adt132)>> {
    let m1122 = v1;
    match m1122 {
        Adt6::C0 => {
            Ok(List::<(Str, Adt132)>::nil())
        }
        Adt6::C1(_) => {
            Ok(List::<(Str, Adt132)>::nil())
        }
        Adt6::C2(_) => {
            Ok(List::<(Str, Adt132)>::nil())
        }
        Adt6::C3(v4) => {
            Ok(List::cons((Str::lit("minLength"), f407(Str::lit("minLength"), true, v0.clone(), f408(v4.clone())?)?), List::cons((Str::lit("maxLength"), f407(Str::lit("maxLength"), false, v0, f408(v4)?)?), List::<(Str, Adt132)>::nil())))
        }
        Adt6::C4(v5) => {
            Ok(List::cons((Str::lit("minItems"), f407(Str::lit("minItems"), true, v0.clone(), f409(v5.clone(), 0u64)?)?), List::cons((Str::lit("maxItems"), f407(Str::lit("maxItems"), false, v0, f409(v5, 0u64)?)?), List::<(Str, Adt132)>::nil())))
        }
        Adt6::C5(v6) => {
            Ok(List::cons((Str::lit("minProperties"), f407(Str::lit("minProperties"), true, v0.clone(), f368(v6.clone(), 0u64)?)?), List::cons((Str::lit("maxProperties"), f407(Str::lit("maxProperties"), false, v0, f368(v6, 0u64)?)?), List::<(Str, Adt132)>::nil())))
        }
    }
}

pub fn f335(v0: List<(Str, Adt6)>, v1: Adt6) -> Adt132 {
    let m1123 = f206(Str::lit("required"), v0);
    match m1123 {
        Adt108::C0 => {
            Adt132::C0
        }
        Adt108::C1(v2) => {
            let m1124 = f209(v2);
            match m1124 {
                None => {
                    Adt132::C2
                }
                Some(v3) => {
                    let m1125 = f218(v1);
                    match m1125 {
                        None => {
                            Adt132::C0
                        }
                        Some(v4) => {
                            f410(v3, v4)
                        }
                    }
                }
            }
        }
    }
}

pub fn f336(v0: Adt114, v1: Adt114) -> Adt114 {
    let m1126 = v0.clone();
    match m1126 {
        Adt114::C0(_) => {
            v0
        }
        Adt114::C1(_) => {
            v0
        }
        Adt114::C2(_) => {
            v0
        }
        Adt114::C3(v5) => {
            let m1127 = v1.clone();
            match m1127 {
                Adt114::C0(_) => {
                    v1
                }
                Adt114::C1(_) => {
                    v1
                }
                Adt114::C2(_) => {
                    v1
                }
                Adt114::C3(v9) => {
                    Adt114::C3({
                        let a1128 = v5;
                        let a1129 = v9;
                        append_list(a1128, a1129)
                    })
                }
            }
        }
    }
}

pub fn f337(v0: Adt6, v1: List<(Str, Adt6)>, v2: Adt6, v3: List<Adt100>) -> Adt114 {
    let m1130 = f206(Str::lit("$ref"), v1);
    match m1130 {
        Adt108::C0 => {
            Adt114::C3(List::<Adt102>::nil())
        }
        Adt108::C1(v4) => {
            let m1131 = f89(v4);
            match m1131 {
                None => {
                    Adt114::C1(Str::lit("$ref"))
                }
                Some(v5) => {
                    let m1132 = f411(v0, v5.clone());
                    match m1132 {
                        None => {
                            Adt114::C2(v5)
                        }
                        Some(v6) => {
                            Adt114::C3(List::cons(Adt102::C0(v6, v2, v3), List::<Adt102>::nil()))
                        }
                    }
                }
            }
        }
    }
}

pub fn f338(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt100>) -> Adt114 {
    let m1133 = f206(Str::lit("properties"), v0);
    match m1133 {
        Adt108::C0 => {
            Adt114::C3(List::<Adt102>::nil())
        }
        Adt108::C1(v3) => {
            let m1134 = f218(v3);
            match m1134 {
                None => {
                    Adt114::C1(Str::lit("properties"))
                }
                Some(v4) => {
                    let m1135 = f218(v1);
                    match m1135 {
                        None => {
                            Adt114::C3(List::<Adt102>::nil())
                        }
                        Some(v5) => {
                            Adt114::C3(f412(match v4 {
                                Adt5::C0(h1136) => {
                                    h1136
                                }
                            }, match v5 {
                                Adt5::C0(h1137) => {
                                    h1137
                                }
                            }, v2, List::<Adt102>::nil()))
                        }
                    }
                }
            }
        }
    }
}

pub fn f339(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt100>) -> Adt114 {
    let m1138 = f206(Str::lit("additionalProperties"), v0.clone());
    match m1138 {
        Adt108::C0 => {
            Adt114::C3(List::<Adt102>::nil())
        }
        Adt108::C1(v3) => {
            let m1139 = f218(v1);
            match m1139 {
                None => {
                    Adt114::C3(List::<Adt102>::nil())
                }
                Some(v4) => {
                    let m1140 = f160(v3.clone());
                    match m1140 {
                        None => {
                            Adt114::C3(f415(match v4 {
                                Adt5::C0(h1142) => {
                                    h1142
                                }
                            }, f414(v0), v3, v2, List::<Adt102>::nil()))
                        }
                        Some(v5) => {
                            if v5 {
                                Adt114::C3(List::<Adt102>::nil())
                            } else {
                                if f413(match v4 {
                                    Adt5::C0(h1141) => {
                                        h1141
                                    }
                                }, f414(v0)) {
                                    Adt114::C0(Str::lit("additionalProperties"))
                                } else {
                                    Adt114::C3(List::<Adt102>::nil())
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f340(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt100>) -> R<Adt114> {
    let m1143 = f206(Str::lit("items"), v0);
    match m1143 {
        Adt108::C0 => {
            Ok(Adt114::C3(List::<Adt102>::nil()))
        }
        Adt108::C1(v3) => {
            let m1144 = f209(v1);
            match m1144 {
                None => {
                    Ok(Adt114::C3(List::<Adt102>::nil()))
                }
                Some(v4) => {
                    Ok(Adt114::C3(f416(v4, 0u64, v3, v2, List::<Adt102>::nil())?))
                }
            }
        }
    }
}

pub fn f341(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt100>) -> Adt114 {
    let m1145 = f206(Str::lit("allOf"), v0);
    match m1145 {
        Adt108::C0 => {
            Adt114::C3(List::<Adt102>::nil())
        }
        Adt108::C1(v3) => {
            let m1146 = f209(v3);
            match m1146 {
                None => {
                    Adt114::C1(Str::lit("allOf"))
                }
                Some(v4) => {
                    let m1147 = v4.clone();
                    match m1147.uncons() {
                        None => {
                            Adt114::C1(Str::lit("allOf"))
                        }
                        Some((_, _)) => {
                            Adt114::C3(f417(v4, v1, v2, List::<Adt102>::nil()))
                        }
                    }
                }
            }
        }
    }
}

pub fn f342(v0: Adt103, v1: Adt114, v2: List<(Str, Adt6)>, v3: Adt6, v4: List<Adt100>) -> Adt114 {
    let m1148 = v1.clone();
    match m1148 {
        Adt114::C0(_) => {
            v1
        }
        Adt114::C1(_) => {
            v1
        }
        Adt114::C2(_) => {
            v1
        }
        Adt114::C3(v8) => {
            let m1149 = v8;
            match m1149.uncons() {
                None => {
                    v1
                }
                Some((_, _)) => {
                    Adt114::C3(List::cons(Adt102::C5(v0, Adt6::C5(v2), v3, v4), List::<Adt102>::nil()))
                }
            }
        }
    }
}

pub fn f343(p1156: List<u64>, p1157: List<u64>) -> List<u64> {
    let m1155 = (p1156, p1157);
    {
        let mut m1155 = m1155;
        loop {
            let step = {
                let (v0, v1) = m1155;
                let m1150 = v0;
                match m1150.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        let a1151 = v2;
                        let a1152 = 126u64;
                        if nat_eq(a1151, a1152) {
                            Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(48u64, List::cons(126u64, v1))))
                        } else {
                            let a1153 = v2;
                            let a1154 = 47u64;
                            if nat_eq(a1153, a1154) {
                                Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(49u64, List::cons(126u64, v1))))
                            } else {
                                Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1155 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f344(p1166: u64, p1167: List<u64>, p1168: List<u64>) -> R<Option<List<u64>>> {
    let m1165 = (p1166, (p1167, p1168));
    {
        let mut m1165 = m1165;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1165;
                let m1158 = v0;
                if m1158 == 0 {
                    let m1159 = v1;
                    match m1159.uncons() {
                        None => {
                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(Some(v2)))
                        }
                        Some((_, _)) => {
                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                        }
                    }
                } else {
                    let v5 = m1158 - 1;
                    let m1160 = v1;
                    match m1160.uncons() {
                        None => {
                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(Some(v2)))
                        }
                        Some((v6, v7)) => {
                            let m1161 = v7;
                            match m1161.uncons() {
                                None => {
                                    Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                }
                                Some((v8, v9)) => {
                                    let m1162 = v9;
                                    match m1162.uncons() {
                                        None => {
                                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                        }
                                        Some((v10, v11)) => {
                                            let m1163 = v11;
                                            match m1163.uncons() {
                                                None => {
                                                    Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                                }
                                                Some((v12, v13)) => {
                                                    let m1164 = f418(v6, v8, v10, v12, f419(v13.clone()))?;
                                                    match m1164 {
                                                        None => {
                                                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                                        }
                                                        Some(v14) => {
                                                            Ok::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>((v5, (v13, f263(v14, v2))))
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
                Ok(next) => { m1165 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f345(p1173: List<Adt52>, p1174: Adt7) -> R<bool> {
    let m1172 = (p1173, p1174);
    {
        let mut m1172 = m1172;
        loop {
            let step = {
                let (v0, v1) = m1172;
                let m1169 = v0;
                match m1169.uncons() {
                    None => {
                        Err::<(List<Adt52>, Adt7), R<bool>>(Ok(true))
                    }
                    Some((v2, v3)) => {
                        if f420(f365(match v2.clone() {
                            Adt52::C0(h1170, _, _) => {
                                h1170
                            }
                        }, v1)?) {
                            Ok::<(List<Adt52>, Adt7), R<bool>>((v3, match v2 {
                                Adt52::C0(h1171, _, _) => {
                                    h1171
                                }
                            }))
                        } else {
                            Err::<(List<Adt52>, Adt7), R<bool>>(Ok(false))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1172 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f346(v0: u64, v1: Adt6, v2: List<Adt115>, v3: List<u64>) -> R<Adt133> {
    let m1175 = v1;
    match m1175 {
        Adt6::C0 => {
            Ok(Adt133::C1(v2, f263(f421(2u64), v3)))
        }
        Adt6::C1(v4) => {
            Ok(Adt133::C1(v2, f263(if v4 {
                f421(0u64)
            } else {
                f421(1u64)
            }, v3)))
        }
        Adt6::C2(v5) => {
            if f422(v5.clone())? {
                Ok(Adt133::C1(v2, f263(f423(v5)?, v3)))
            } else {
                Ok(Adt133::C0(Adt104::C11))
            }
        }
        Adt6::C3(v6) => {
            Ok(Adt133::C1(v2, f263(f347(v6)?, v3)))
        }
        Adt6::C4(v7) => {
            let a1176 = v0;
            let a1177 = f424();
            if nat_lt(a1176, a1177) {
                Ok(Adt133::C1(f425(91u64, Adt115::C2({
                    let a1178 = v0;
                    let a1179 = 1u64;
                    nat_add(a1178, a1179)?
                }, v7), v2), v3))
            } else {
                Ok(Adt133::C0(Adt104::C12))
            }
        }
        Adt6::C5(v8) => {
            let a1180 = v0;
            let a1181 = f424();
            if nat_lt(a1180, a1181) {
                Ok(Adt133::C1(f425(123u64, Adt115::C4({
                    let a1182 = v0;
                    let a1183 = 1u64;
                    nat_add(a1182, a1183)?
                }, v8), v2), v3))
            } else {
                Ok(Adt133::C0(Adt104::C12))
            }
        }
    }
}

pub fn f347(v0: Str) -> R<List<u64>> {
    Ok(List::cons(34u64, {
        let a1185 = f263(f426(f18({
            let a1184 = v0;
            utf8_encode(a1184)
        })?, List::<u64>::nil())?, List::<u64>::nil());
        let a1186 = List::cons(34u64, List::<u64>::nil());
        append_list(a1185, a1186)
    }))
}

pub fn f348(v0: u64) -> R<u64> {
    let a1189 = {
        let a1187 = v0;
        let a1188 = 3u64;
        nat_mul(a1187, a1188)?
    };
    let a1190 = 1u64;
    nat_add(a1189, a1190)
}

pub fn f349(v0: u64, v1: u64, v2: u64) -> bool {
    let a1191 = v1;
    let a1192 = v0;
    if nat_le(a1191, a1192) {
        let a1193 = v0;
        let a1194 = v2;
        nat_le(a1193, a1194)
    } else {
        false
    }
}

pub fn f350(v0: Adt124) -> R<Adt145> {
    if f427(match v0.clone() {
        Adt124::C0(_, h1195, _, _, _, _, _) => {
            h1195
        }
    }) {
        let v1: i64 = {
            let a1201 = if match v0.clone() {
                Adt124::C0(_, _, _, _, _, h1196, _) => {
                    h1196
                }
            } {
                let a1198 = match v0.clone() {
                    Adt124::C0(_, _, _, _, _, _, h1197) => {
                        h1197
                    }
                };
                int_neg(a1198)?
            } else {
                match v0.clone() {
                    Adt124::C0(_, _, _, _, _, _, h1199) => {
                        h1199
                    }
                }
            };
            let a1202 = match v0.clone() {
                Adt124::C0(_, _, _, _, h1200, _, _) => {
                    h1200
                }
            };
            int_sub(a1201, a1202)?
        };
        let m1205 = {
            let a1203 = v1;
            let a1204 = -9999i64;
            compare(a1203, a1204)
        };
        if if match m1205 {
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
            let m1208 = {
                let a1206 = 9999i64;
                let a1207 = v1;
                compare(a1206, a1207)
            };
            match m1208 {
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
                Adt124::C0(h1209, _, _, _, _, _, _) => {
                    h1209
                }
            } {
                let a1211 = match v0 {
                    Adt124::C0(_, _, h1210, _, _, _, _) => {
                        h1210
                    }
                };
                int_neg(a1211)?
            } else {
                match v0 {
                    Adt124::C0(_, _, h1212, _, _, _, _) => {
                        h1212
                    }
                }
            }, v1))))
        }
    } else {
        Ok(Adt145::C0)
    }
}

pub fn f351(v0: Adt6, v1: List<Adt126>, v2: u64) -> Adt121 {
    let m1213 = v1.clone();
    match m1213.uncons() {
        None => {
            Adt121::C0(Adt122::C9(v0), List::<Adt126>::nil(), 0u64)
        }
        Some((v3, v4)) => {
            let m1214 = v3;
            match m1214 {
                Adt126::C0(v5) => {
                    Adt121::C0(Adt122::C2, List::cons(Adt126::C0(List::cons(v0, v5)), v4), v2)
                }
                Adt126::C1(v6, v7) => {
                    let m1215 = v7;
                    match m1215 {
                        None => {
                            Adt121::C0(Adt122::C2, v1, v2)
                        }
                        Some(v8) => {
                            Adt121::C0(Adt122::C2, List::cons(Adt126::C1(List::cons((v8, v0), v6), None::<Str>), v4), v2)
                        }
                    }
                }
            }
        }
    }
}

pub fn f352(v0: u64) -> bool {
    let a1216 = v0;
    let a1217 = 32u64;
    if nat_eq(a1216, a1217) {
        true
    } else {
        let a1218 = v0;
        let a1219 = 10u64;
        if nat_eq(a1218, a1219) {
            true
        } else {
            let a1220 = v0;
            let a1221 = 13u64;
            if nat_eq(a1220, a1221) {
                true
            } else {
                let a1222 = v0;
                let a1223 = 9u64;
                nat_eq(a1222, a1223)
            }
        }
    }
}

pub fn f353(v0: Adt121, v1: u64, v2: u64) -> R<Adt134> {
    let a1224 = v2;
    let a1225 = 34u64;
    if nat_eq(a1224, a1225) {
        Ok(f357(v0, Adt122::C6(Adt123::C0, List::<u64>::nil(), false)))
    } else {
        let a1226 = v2;
        let a1227 = 123u64;
        if nat_eq(a1226, a1227) {
            f428(v0, Adt126::C1(List::<(Str, Adt6)>::nil(), None::<Str>), Adt122::C3, v1)
        } else {
            let a1228 = v2;
            let a1229 = 91u64;
            if nat_eq(a1228, a1229) {
                f428(v0, Adt126::C0(List::<Adt6>::nil()), Adt122::C1, v1)
            } else {
                let a1230 = v2;
                let a1231 = 116u64;
                if nat_eq(a1230, a1231) {
                    Ok(f357(v0, Adt122::C7(0u64, 1u64)))
                } else {
                    let a1232 = v2;
                    let a1233 = 102u64;
                    if nat_eq(a1232, a1233) {
                        Ok(f357(v0, Adt122::C7(1u64, 1u64)))
                    } else {
                        let a1234 = v2;
                        let a1235 = 110u64;
                        if nat_eq(a1234, a1235) {
                            Ok(f357(v0, Adt122::C7(2u64, 1u64)))
                        } else {
                            let a1236 = v2;
                            let a1237 = 45u64;
                            if nat_eq(a1236, a1237) {
                                Ok(f357(v0, f429(true, Adt125::C0, 0i64, 0u64, 0i64, false, 0i64)))
                            } else {
                                let a1238 = v2;
                                let a1239 = 48u64;
                                if nat_eq(a1238, a1239) {
                                    Ok(f357(v0, f429(false, Adt125::C1, 0i64, 0u64, 0i64, false, 0i64)))
                                } else {
                                    if f349(v2, 49u64, 57u64) {
                                        Ok(f357(v0, f429(false, Adt125::C2, f430({
                                            let a1240 = v2;
                                            let a1241 = 48u64;
                                            nat_sub(a1240, a1241)
                                        }), 1u64, 0i64, false, 0i64)))
                                    } else {
                                        Ok(f358(Adt104::C2(v1)))
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

pub fn f354(v0: Adt121, v1: u64) -> Adt134 {
    let m1243 = match v0.clone() {
        Adt121::C0(_, h1242, _) => {
            h1242
        }
    };
    match m1243.uncons() {
        None => {
            f358(Adt104::C2(v1))
        }
        Some((v2, v3)) => {
            let m1244 = v2;
            match m1244 {
                Adt126::C0(v4) => {
                    Adt134::C1(f351(Adt6::C4(f431(v4, List::<Adt6>::nil())), v3, {
                        let a1246 = match v0 {
                            Adt121::C0(_, _, h1245) => {
                                h1245
                            }
                        };
                        let a1247 = 1u64;
                        nat_sub(a1246, a1247)
                    }))
                }
                Adt126::C1(_, _) => {
                    f358(Adt104::C2(v1))
                }
            }
        }
    }
}

pub fn f355(v0: Adt121, v1: u64, v2: u64) -> Adt134 {
    let m1249 = match v0.clone() {
        Adt121::C0(h1248, _, _) => {
            h1248
        }
    };
    match m1249 {
        Adt122::C0 => {
            f358(Adt104::C2(v1))
        }
        Adt122::C1 => {
            f358(Adt104::C2(v1))
        }
        Adt122::C2 => {
            if f352(v2) {
                Adt134::C1(v0)
            } else {
                let a1250 = v2;
                let a1251 = 44u64;
                if nat_eq(a1250, a1251) {
                    f432(v0, v1)
                } else {
                    let a1252 = v2;
                    let a1253 = 93u64;
                    if nat_eq(a1252, a1253) {
                        f354(v0, v1)
                    } else {
                        let a1254 = v2;
                        let a1255 = 125u64;
                        if nat_eq(a1254, a1255) {
                            f356(v0, v1)
                        } else {
                            f358(Adt104::C2(v1))
                        }
                    }
                }
            }
        }
        Adt122::C3 => {
            f358(Adt104::C2(v1))
        }
        Adt122::C4 => {
            f358(Adt104::C2(v1))
        }
        Adt122::C5 => {
            f358(Adt104::C2(v1))
        }
        Adt122::C6(_, _, _) => {
            f358(Adt104::C2(v1))
        }
        Adt122::C7(_, _) => {
            f358(Adt104::C2(v1))
        }
        Adt122::C8(_) => {
            f358(Adt104::C2(v1))
        }
        Adt122::C9(_) => {
            if f352(v2) {
                Adt134::C1(v0)
            } else {
                f358(Adt104::C10(v1))
            }
        }
    }
}

pub fn f356(v0: Adt121, v1: u64) -> Adt134 {
    let m1257 = match v0.clone() {
        Adt121::C0(_, h1256, _) => {
            h1256
        }
    };
    match m1257.uncons() {
        None => {
            f358(Adt104::C2(v1))
        }
        Some((v2, v3)) => {
            let m1258 = v2;
            match m1258 {
                Adt126::C0(_) => {
                    f358(Adt104::C2(v1))
                }
                Adt126::C1(v5, _) => {
                    Adt134::C1(f351(Adt6::C5(f433(v5, List::<(Str, Adt6)>::nil())), v3, {
                        let a1260 = match v0 {
                            Adt121::C0(_, _, h1259) => {
                                h1259
                            }
                        };
                        let a1261 = 1u64;
                        nat_sub(a1260, a1261)
                    }))
                }
            }
        }
    }
}

pub fn f357(v0: Adt121, v1: Adt122) -> Adt134 {
    Adt134::C1(f434(v0, v1))
}

pub fn f358(v0: Adt104) -> Adt134 {
    Adt134::C0(v0)
}

pub fn f359(v0: Adt121, v1: u64, v2: u64, v3: Adt123, v4: List<u64>, v5: bool) -> R<Adt134> {
    let m1262 = v3;
    match m1262 {
        Adt123::C0 => {
            let a1263 = v2;
            let a1264 = 34u64;
            if nat_eq(a1263, a1264) {
                Ok(f435(v0, v1, v4, v5))
            } else {
                let a1265 = v2;
                let a1266 = 92u64;
                if nat_eq(a1265, a1266) {
                    Ok(f357(v0, Adt122::C6(Adt123::C1, v4, v5)))
                } else {
                    let a1267 = v2;
                    let a1268 = 32u64;
                    if nat_lt(a1267, a1268) {
                        Ok(f358(Adt104::C4(v1)))
                    } else {
                        Ok(f357(v0, Adt122::C6(Adt123::C0, List::cons(v2, v4), v5)))
                    }
                }
            }
        }
        Adt123::C1 => {
            let a1269 = v2;
            let a1270 = 117u64;
            if nat_eq(a1269, a1270) {
                Ok(f357(v0, Adt122::C6(Adt123::C2(0u64, 0u64, 0u64), v4, v5)))
            } else {
                let a1271 = f436(v2);
                let a1272 = 0u64;
                if nat_eq(a1271, a1272) {
                    Ok(f358(Adt104::C5(v1)))
                } else {
                    Ok(f357(v0, Adt122::C6(Adt123::C0, List::cons(f436(v2), v4), v5)))
                }
            }
        }
        Adt123::C2(v6, v7, v8) => {
            let a1273 = 15u64;
            let a1274 = f437(v2);
            if nat_lt(a1273, a1274) {
                Ok(f358(Adt104::C5(v1)))
            } else {
                let a1277 = {
                    let a1275 = v6;
                    let a1276 = 1u64;
                    nat_add(a1275, a1276)?
                };
                let a1278 = 4u64;
                if nat_eq(a1277, a1278) {
                    f438(v0, v1, {
                        let a1281 = {
                            let a1279 = v7;
                            let a1280 = 16u64;
                            nat_mul(a1279, a1280)?
                        };
                        let a1282 = f437(v2);
                        nat_add(a1281, a1282)?
                    }, v8, v4, v5)
                } else {
                    Ok(f357(v0, Adt122::C6(Adt123::C2({
                        let a1283 = v6;
                        let a1284 = 1u64;
                        nat_add(a1283, a1284)?
                    }, {
                        let a1287 = {
                            let a1285 = v7;
                            let a1286 = 16u64;
                            nat_mul(a1285, a1286)?
                        };
                        let a1288 = f437(v2);
                        nat_add(a1287, a1288)?
                    }, v8), v4, v5)))
                }
            }
        }
        Adt123::C3(v9) => {
            let a1289 = v2;
            let a1290 = 92u64;
            if nat_eq(a1289, a1290) {
                Ok(f357(v0, Adt122::C6(Adt123::C4(v9), v4, v5)))
            } else {
                Ok(f358(Adt104::C6(v1)))
            }
        }
        Adt123::C4(v10) => {
            let a1291 = v2;
            let a1292 = 117u64;
            if nat_eq(a1291, a1292) {
                Ok(f357(v0, Adt122::C6(Adt123::C2(0u64, 0u64, v10), v4, v5)))
            } else {
                Ok(f358(Adt104::C6(v1)))
            }
        }
    }
}

pub fn f360(v0: Adt121, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Adt134> {
    let a1293 = v2;
    let a1294 = f439(v3, v4);
    if nat_eq(a1293, a1294) {
        let a1298 = {
            let a1295 = v4;
            let a1296 = 1u64;
            nat_add(a1295, a1296)?
        };
        let a1299 = {
            let a1297 = f440(v3);
            length_list(a1297)
        };
        if nat_eq(a1298, a1299) {
            Ok(f441(v0, f442(v3)))
        } else {
            Ok(f357(v0, Adt122::C7(v3, {
                let a1300 = v4;
                let a1301 = 1u64;
                nat_add(a1300, a1301)?
            })))
        }
    } else {
        Ok(f358(Adt104::C2(v1)))
    }
}

pub fn f361(v0: Adt121, v1: u64, v2: u64, v3: Adt124) -> R<Adt134> {
    let m1303 = match v3.clone() {
        Adt124::C0(_, h1302, _, _, _, _, _) => {
            h1302
        }
    };
    match m1303 {
        Adt125::C0 => {
            let a1304 = v2;
            let a1305 = 48u64;
            if nat_eq(a1304, a1305) {
                Ok(f443(v0, v3, Adt125::C1))
            } else {
                if f349(v2, 49u64, 57u64) {
                    f444(v0, v1, v2, v3, false)
                } else {
                    Ok(f358(Adt104::C7(v1)))
                }
            }
        }
        Adt125::C1 => {
            let a1306 = v2;
            let a1307 = 46u64;
            if nat_eq(a1306, a1307) {
                Ok(f443(v0, v3, Adt125::C3))
            } else {
                if f445(v2) {
                    Ok(f443(v0, v3, Adt125::C5))
                } else {
                    if f446(v2) {
                        Ok(f358(Adt104::C7(v1)))
                    } else {
                        f447(v0, v1, v2, v3)
                    }
                }
            }
        }
        Adt125::C2 => {
            if f446(v2) {
                f444(v0, v1, v2, v3, false)
            } else {
                let a1308 = v2;
                let a1309 = 46u64;
                if nat_eq(a1308, a1309) {
                    Ok(f443(v0, v3, Adt125::C3))
                } else {
                    if f445(v2) {
                        Ok(f443(v0, v3, Adt125::C5))
                    } else {
                        f447(v0, v1, v2, v3)
                    }
                }
            }
        }
        Adt125::C3 => {
            if f446(v2) {
                f444(v0, v1, v2, v3, true)
            } else {
                Ok(f358(Adt104::C7(v1)))
            }
        }
        Adt125::C4 => {
            if f446(v2) {
                f444(v0, v1, v2, v3, true)
            } else {
                if f445(v2) {
                    Ok(f443(v0, v3, Adt125::C5))
                } else {
                    f447(v0, v1, v2, v3)
                }
            }
        }
        Adt125::C5 => {
            let a1310 = v2;
            let a1311 = 43u64;
            if nat_eq(a1310, a1311) {
                Ok(f357(v0, f429(match v3.clone() {
                    Adt124::C0(h1312, _, _, _, _, _, _) => {
                        h1312
                    }
                }, Adt125::C6, match v3.clone() {
                    Adt124::C0(_, _, h1313, _, _, _, _) => {
                        h1313
                    }
                }, match v3.clone() {
                    Adt124::C0(_, _, _, h1314, _, _, _) => {
                        h1314
                    }
                }, match v3 {
                    Adt124::C0(_, _, _, _, h1315, _, _) => {
                        h1315
                    }
                }, false, 0i64)))
            } else {
                let a1316 = v2;
                let a1317 = 45u64;
                if nat_eq(a1316, a1317) {
                    Ok(f357(v0, f429(match v3.clone() {
                        Adt124::C0(h1318, _, _, _, _, _, _) => {
                            h1318
                        }
                    }, Adt125::C6, match v3.clone() {
                        Adt124::C0(_, _, h1319, _, _, _, _) => {
                            h1319
                        }
                    }, match v3.clone() {
                        Adt124::C0(_, _, _, h1320, _, _, _) => {
                            h1320
                        }
                    }, match v3 {
                        Adt124::C0(_, _, _, _, h1321, _, _) => {
                            h1321
                        }
                    }, true, 0i64)))
                } else {
                    if f446(v2) {
                        f448(v0, v1, v2, Adt124::C0(match v3.clone() {
                            Adt124::C0(h1322, _, _, _, _, _, _) => {
                                h1322
                            }
                        }, match v3.clone() {
                            Adt124::C0(_, h1323, _, _, _, _, _) => {
                                h1323
                            }
                        }, match v3.clone() {
                            Adt124::C0(_, _, h1324, _, _, _, _) => {
                                h1324
                            }
                        }, match v3.clone() {
                            Adt124::C0(_, _, _, h1325, _, _, _) => {
                                h1325
                            }
                        }, match v3 {
                            Adt124::C0(_, _, _, _, h1326, _, _) => {
                                h1326
                            }
                        }, false, 0i64), false)
                    } else {
                        Ok(f358(Adt104::C7(v1)))
                    }
                }
            }
        }
        Adt125::C6 => {
            if f446(v2) {
                f448(v0, v1, v2, v3.clone(), match v3 {
                    Adt124::C0(_, _, _, _, _, h1327, _) => {
                        h1327
                    }
                })
            } else {
                Ok(f358(Adt104::C7(v1)))
            }
        }
        Adt125::C7 => {
            if f446(v2) {
                f448(v0, v1, v2, v3.clone(), match v3 {
                    Adt124::C0(_, _, _, _, _, h1328, _) => {
                        h1328
                    }
                })
            } else {
                f447(v0, v1, v2, v3)
            }
        }
    }
}

pub fn f362(v0: Adt6) -> u64 {
    let m1329 = v0;
    match m1329 {
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

pub fn f363(v0: Adt6) -> Option<Adt7> {
    let m1330 = v0;
    match m1330 {
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

pub fn f364(v0: Adt146) -> bool {
    let m1331 = v0;
    match m1331 {
        Adt146::C0 => {
            false
        }
        Adt146::C1 => {
            true
        }
        Adt146::C2 => {
            false
        }
    }
}

pub fn f365(v0: Adt7, v1: Adt7) -> R<Adt146> {
    f449(v0.clone(), v1.clone(), f450(match v0 {
        Adt7::C0(h1332, _) => {
            h1332
        }
    }), f450(match v1 {
        Adt7::C0(h1333, _) => {
            h1333
        }
    }))
}

pub fn f366(v0: Option<List<(Adt6, Adt6)>>, v1: List<(Adt6, Adt6)>) -> Adt135 {
    let m1334 = v0;
    match m1334 {
        None => {
            Adt135::C0
        }
        Some(v2) => {
            Adt135::C1({
                let a1335 = v2;
                let a1336 = v1;
                append_list(a1335, a1336)
            })
        }
    }
}

pub fn f367(p1341: List<Adt6>, p1342: List<Adt6>, p1343: List<(Adt6, Adt6)>) -> Option<List<(Adt6, Adt6)>> {
    let m1340 = (p1341, (p1342, p1343));
    {
        let mut m1340 = m1340;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1340;
                let m1337 = v0;
                match m1337.uncons() {
                    None => {
                        let m1338 = v1;
                        match m1338.uncons() {
                            None => {
                                Err::<(List<Adt6>, (List<Adt6>, List<(Adt6, Adt6)>)), Option<List<(Adt6, Adt6)>>>(Some(v2))
                            }
                            Some((_, _)) => {
                                Err::<(List<Adt6>, (List<Adt6>, List<(Adt6, Adt6)>)), Option<List<(Adt6, Adt6)>>>(None::<List<(Adt6, Adt6)>>)
                            }
                        }
                    }
                    Some((v5, v6)) => {
                        let m1339 = v1;
                        match m1339.uncons() {
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
                Ok(next) => { m1340 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f368(p1348: List<(Str, Adt6)>, p1349: u64) -> R<u64> {
    let m1347 = (p1348, p1349);
    {
        let mut m1347 = m1347;
        loop {
            let step = {
                let (v0, v1) = m1347;
                let m1344 = v0;
                match m1344.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, u64), R<u64>>(Ok(v1))
                    }
                    Some((_, v3)) => {
                        Ok::<(List<(Str, Adt6)>, u64), R<u64>>((v3, {
                            let a1345 = v1;
                            let a1346 = 1u64;
                            nat_add(a1345, a1346)?
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m1347 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f369(p1355: List<(Str, Adt6)>, v1: List<(Str, Adt6)>, p1356: List<(Adt6, Adt6)>) -> Option<List<(Adt6, Adt6)>> {
    let m1354 = (p1355, p1356);
    {
        let mut m1354 = m1354;
        loop {
            let step = {
                let (v0, v2) = m1354;
                let m1350 = v0;
                match m1350.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<(Adt6, Adt6)>), Option<List<(Adt6, Adt6)>>>(Some(v2))
                    }
                    Some((v3, v4)) => {
                        let m1352 = f206({
                            let (h1351, _) = v3;
                            h1351
                        }, v1.clone());
                        match m1352 {
                            Adt108::C0 => {
                                Err::<(List<(Str, Adt6)>, List<(Adt6, Adt6)>), Option<List<(Adt6, Adt6)>>>(None::<List<(Adt6, Adt6)>>)
                            }
                            Adt108::C1(v5) => {
                                Ok::<(List<(Str, Adt6)>, List<(Adt6, Adt6)>), Option<List<(Adt6, Adt6)>>>((v4, List::cons(({
                                    let (_, h1353) = v3;
                                    h1353
                                }, v5), v2)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1354 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f370(v0: Result<Adt137, Adt140>) -> Option<Adt87> {
    let m1357 = v0;
    match m1357 {
        Ok(_) => {
            None::<Adt87>
        }
        Err(v2) => {
            let m1358 = v2;
            match m1358 {
                Adt140::C0(_) => {
                    None::<Adt87>
                }
                Adt140::C1(_, _) => {
                    None::<Adt87>
                }
                Adt140::C2(_) => {
                    None::<Adt87>
                }
                Adt140::C3(_) => {
                    None::<Adt87>
                }
                Adt140::C4(v8) => {
                    f451(v8)
                }
            }
        }
    }
}

pub fn f371(v0: bool, v1: Adt136, v2: Adt87) -> R<Adt88> {
    if f452(match v2.clone() {
        Adt87::C0(h1359, _) => {
            h1359
        }
    }) {
        Ok(Adt88::C1(v2))
    } else {
        Ok(Adt88::C0(f43(f372(v0, Adt136::C0(match v1 {
            Adt136::C0(h1360, _) => {
                h1360
            }
        }, Err::<Adt137, Adt140>(f453())))?)))
    }
}

pub fn f372(v0: bool, v1: Adt136) -> R<Adt95> {
    let m1362 = match v1.clone() {
        Adt136::C0(_, h1361) => {
            h1361
        }
    };
    match m1362 {
        Ok(v2) => {
            let m1363 = f454(v2.clone())?;
            match m1363 {
                None => {
                    Ok(f44())
                }
                Some(v3) => {
                    Ok(f455(v0, f176(f456(), f457(match v1 {
                        Adt136::C0(h1364, _) => {
                            h1364
                        }
                    }, v3))?, f458(match v2 {
                        Adt137::C0(_, _, h1365) => {
                            h1365
                        }
                    }, List::<List<u64>>::nil())?))
                }
            }
        }
        Err(v4) => {
            f176(f459(v4.clone()), f178(Some(match v1 {
                Adt136::C0(h1366, _) => {
                    h1366
                }
            }), f460(v4)?))
        }
    }
}

pub fn f373() -> List<u64> {
    List::cons(65u64, List::cons(99u64, List::cons(99u64, List::cons(101u64, List::cons(112u64, List::cons(116u64, List::<u64>::nil()))))))
}

pub fn f374(p1369: List<List<u64>>) -> R<bool> {
    let m1368 = p1369;
    {
        let mut m1368 = m1368;
        loop {
            let step = {
                let v0 = m1368;
                let m1367 = v0;
                match m1367.uncons() {
                    None => {
                        Err::<List<List<u64>>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        if f461(v1)? {
                            Err::<List<List<u64>>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<List<u64>>, R<bool>>(v2)
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

pub fn f375(v0: u64, p1374: List<u64>, p1375: List<u64>, p1376: List<List<u64>>) -> List<List<u64>> {
    let m1373 = (p1374, (p1375, p1376));
    {
        let mut m1373 = m1373;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1373;
                let m1370 = v1;
                match m1370.uncons() {
                    None => {
                        Err::<(List<u64>, (List<u64>, List<List<u64>>)), List<List<u64>>>(f118(List::cons(f290(v2, List::<u64>::nil()), v3), List::<List<u64>>::nil()))
                    }
                    Some((v4, v5)) => {
                        let a1371 = v4;
                        let a1372 = v0;
                        if nat_eq(a1371, a1372) {
                            Ok::<(List<u64>, (List<u64>, List<List<u64>>)), List<List<u64>>>((v5, (List::<u64>::nil(), List::cons(f290(v2, List::<u64>::nil()), v3))))
                        } else {
                            Ok::<(List<u64>, (List<u64>, List<List<u64>>)), List<List<u64>>>((v5, (List::cons(v4, v2), v3)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1373 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f376(v0: Adt0, v1: Fn2, v2: u64, v3: Result<Adt147, Adt140>) -> R<Result<Adt137, Adt140>> {
    let m1377 = v3;
    match m1377 {
        Ok(v5) => {
            f462(v0, v1, v2, v5)
        }
        Err(v4) => {
            Ok(Err::<Adt137, Adt140>(v4))
        }
    }
}

pub fn f377(v0: Option<Adt127>, v1: List<Str>, v2: Adt117) -> R<Result<Adt147, Adt140>> {
    Ok(f463(f464(match v2.clone() {
        Adt117::C0(_, h1378, _) => {
            h1378
        }
    }), f465(v0, v1, v2)?))
}

pub fn f378(v0: Adt129, v1: Adt111) -> Adt142 {
    let m1379 = v1;
    match m1379 {
        Adt111::C0 => {
            f384()
        }
        Adt111::C1 => {
            Adt142::C0(None::<Str>, Some(Adt128::C6(v0)))
        }
        Adt111::C2 => {
            Adt142::C0(None::<Str>, Some(Adt128::C7(v0)))
        }
        Adt111::C3(v2) => {
            let m1380 = f466(v2);
            match m1380 {
                None => {
                    Adt142::C0(None::<Str>, Some(Adt128::C6(v0)))
                }
                Some(v3) => {
                    Adt142::C0(Some(v3), None::<Adt128>)
                }
            }
        }
    }
}

pub fn f379() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(112u64, List::cons(114u64, List::cons(111u64, List::cons(116u64, List::cons(111u64, List::cons(99u64, List::cons(111u64, List::cons(108u64, List::cons(45u64, List::cons(118u64, List::cons(101u64, List::cons(114u64, List::cons(115u64, List::cons(105u64, List::cons(111u64, List::cons(110u64, List::<u64>::nil()))))))))))))))))))))
}

pub fn f380() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(109u64, List::cons(101u64, List::cons(116u64, List::cons(104u64, List::cons(111u64, List::cons(100u64, List::<u64>::nil()))))))))))
}

pub fn f381(v0: Str) -> bool {
    let m1381 = f467(v0);
    match m1381 {
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

pub fn f382(v0: Adt111) -> R<Adt111> {
    let m1382 = v0.clone();
    match m1382 {
        Adt111::C0 => {
            Ok(v0)
        }
        Adt111::C1 => {
            Ok(v0)
        }
        Adt111::C2 => {
            Ok(v0)
        }
        Adt111::C3(v1) => {
            let m1383 = f468(v1);
            match m1383 {
                None => {
                    Ok(v0)
                }
                Some(v2) => {
                    let m1384 = f299(v2)?;
                    match m1384 {
                        None => {
                            Ok(Adt111::C1)
                        }
                        Some(v3) => {
                            Ok(Adt111::C3(v3))
                        }
                    }
                }
            }
        }
    }
}

pub fn f383() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(110u64, List::cons(97u64, List::cons(109u64, List::cons(101u64, List::<u64>::nil()))))))))
}

pub fn f384() -> Adt142 {
    Adt142::C0(None::<Str>, None::<Adt128>)
}

pub fn f385(v0: Option<Adt128>, v1: Option<Adt128>) -> Option<Adt128> {
    let m1385 = v0.clone();
    match m1385 {
        None => {
            v1
        }
        Some(_) => {
            v0
        }
    }
}

pub fn f386(v0: List<Adt92>, v1: List<Adt12>, v2: Option<Str>) -> R<List<Adt130>> {
    Ok(f469(f470(v0, v1, v2, List::<Adt130>::nil())?, List::<Adt130>::nil()))
}

pub fn f387() -> Str {
    Str::lit("2026-07-28")
}

pub fn f388(v0: Adt7) -> R<Option<i64>> {
    let m1388 = f471(40u64, match v0.clone() {
        Adt7::C0(h1386, _) => {
            h1386
        }
    }, match v0 {
        Adt7::C0(_, h1387) => {
            h1387
        }
    })?;
    match m1388 {
        None => {
            Ok(None::<i64>)
        }
        Some(v1) => {
            Ok(f472(v1))
        }
    }
}

pub fn f389(v0: Str, v1: Adt5) -> bool {
    f473(f138(v0, v1))
}

pub fn f390(v0: Adt105, v1: Adt5) -> Result<Adt117, Adt109> {
    let m1389 = f138(Str::lit("method"), v1.clone());
    match m1389 {
        Adt108::C0 => {
            f279(Some(v0), f319(v1))
        }
        Adt108::C1(v2) => {
            let m1390 = f89(v2);
            match m1390 {
                None => {
                    f279(Some(v0), Adt110::C4)
                }
                Some(v3) => {
                    f474(v0, v3, v1)
                }
            }
        }
    }
}

pub fn f391(v0: u64) -> bool {
    let a1391 = v0;
    let a1392 = 9u64;
    if nat_eq(a1391, a1392) {
        true
    } else {
        let a1393 = v0;
        let a1394 = 32u64;
        nat_eq(a1393, a1394)
    }
}

pub fn f392(v0: u64) -> bool {
    let a1395 = 47u64;
    let a1396 = v0;
    if nat_lt(a1395, a1396) {
        let a1397 = v0;
        let a1398 = 58u64;
        nat_lt(a1397, a1398)
    } else {
        false
    }
}

pub fn f393(p1402: List<Str>, v1: List<(Str, Adt6)>) -> Option<Str> {
    let m1401 = p1402;
    {
        let mut m1401 = m1401;
        loop {
            let step = {
                let v0 = m1401;
                let m1399 = v0;
                match m1399.uncons() {
                    None => {
                        Err::<List<Str>, Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        let m1400 = f206(v2.clone(), v1.clone());
                        match m1400 {
                            Adt108::C0 => {
                                Ok::<List<Str>, Option<Str>>(v3)
                            }
                            Adt108::C1(_) => {
                                Err::<List<Str>, Option<Str>>(Some(v2))
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

pub fn f394() -> List<Str> {
    List::cons(Str::lit("pattern"), List::cons(Str::lit("patternProperties"), List::cons(Str::lit("propertyNames"), List::cons(Str::lit("multipleOf"), List::cons(Str::lit("uniqueItems"), List::cons(Str::lit("contains"), List::cons(Str::lit("minContains"), List::cons(Str::lit("maxContains"), List::cons(Str::lit("prefixItems"), List::cons(Str::lit("dependentRequired"), List::cons(Str::lit("dependentSchemas"), List::cons(Str::lit("dependencies"), List::cons(Str::lit("additionalItems"), List::cons(Str::lit("unevaluatedProperties"), List::cons(Str::lit("unevaluatedItems"), List::cons(Str::lit("$dynamicRef"), List::cons(Str::lit("$dynamicAnchor"), List::cons(Str::lit("$id"), List::cons(Str::lit("$vocabulary"), List::cons(Str::lit("contentEncoding"), List::cons(Str::lit("contentMediaType"), List::cons(Str::lit("contentSchema"), List::<Str>::nil()))))))))))))))))))))))
}

pub fn f395(p1409: List<(Str, u64)>, v1: List<(Str, Adt6)>) -> Option<Str> {
    let m1408 = p1409;
    {
        let mut m1408 = m1408;
        loop {
            let step = {
                let v0 = m1408;
                let m1403 = v0;
                match m1403.uncons() {
                    None => {
                        Err::<List<(Str, u64)>, Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        let m1405 = f206({
                            let (h1404, _) = v2.clone();
                            h1404
                        }, v1.clone());
                        match m1405 {
                            Adt108::C0 => {
                                Ok::<List<(Str, u64)>, Option<Str>>(v3)
                            }
                            Adt108::C1(v4) => {
                                if f475({
                                    let (_, h1406) = v2;
                                    h1406
                                }, v4) {
                                    Ok::<List<(Str, u64)>, Option<Str>>(v3)
                                } else {
                                    Err::<List<(Str, u64)>, Option<Str>>(Some({
                                        let (h1407, _) = v2;
                                        h1407
                                    }))
                                }
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

pub fn f396() -> List<(Str, u64)> {
    List::cons((Str::lit("$schema"), 0u64), List::cons((Str::lit("$ref"), 0u64), List::cons((Str::lit("$anchor"), 0u64), List::cons((Str::lit("$comment"), 0u64), List::cons((Str::lit("title"), 0u64), List::cons((Str::lit("description"), 0u64), List::cons((Str::lit("format"), 0u64), List::cons((Str::lit("type"), 7u64), List::cons((Str::lit("enum"), 3u64), List::cons((Str::lit("required"), 5u64), List::cons((Str::lit("properties"), 4u64), List::cons((Str::lit("$defs"), 4u64), List::cons((Str::lit("allOf"), 8u64), List::cons((Str::lit("anyOf"), 8u64), List::cons((Str::lit("oneOf"), 8u64), List::cons((Str::lit("minimum"), 1u64), List::cons((Str::lit("maximum"), 1u64), List::cons((Str::lit("exclusiveMinimum"), 1u64), List::cons((Str::lit("exclusiveMaximum"), 1u64), List::cons((Str::lit("minItems"), 2u64), List::cons((Str::lit("maxItems"), 2u64), List::cons((Str::lit("minLength"), 2u64), List::cons((Str::lit("maxLength"), 2u64), List::cons((Str::lit("minProperties"), 2u64), List::cons((Str::lit("maxProperties"), 2u64), List::cons((Str::lit("deprecated"), 6u64), List::cons((Str::lit("readOnly"), 6u64), List::cons((Str::lit("writeOnly"), 6u64), List::<(Str, u64)>::nil()))))))))))))))))))))))))))))
}

pub fn f397(v0: Str, v1: Adt6, v2: List<(Str, Adt6)>) -> Option<Adt93> {
    let m1410 = f206(Str::lit("$ref"), v2);
    match m1410 {
        Adt108::C0 => {
            None::<Adt93>
        }
        Adt108::C1(v3) => {
            let m1411 = f89(v3);
            match m1411 {
                None => {
                    None::<Adt93>
                }
                Some(v4) => {
                    let m1412 = f411(v1, v4.clone());
                    match m1412 {
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

pub fn f398(v0: Str, v1: List<(Str, Adt6)>, v2: Adt6, v3: bool, v4: List<Str>, v5: Adt113) -> R<Adt144> {
    if v3 {
        let m1413 = f89(v2);
        match m1413 {
            None => {
                Ok(Adt144::C0(Adt93::C7(v0)))
            }
            Some(v6) => {
                f476(v0, v1, v6.clone(), f132(v6)?, v4, v5)
            }
        }
    } else {
        Ok(Adt144::C0(Adt93::C10(v0)))
    }
}

pub fn f399(v0: Str, v1: List<(Str, Adt6)>, v2: bool, v3: List<Str>, v4: List<Adt106>) -> List<Adt106> {
    let m1414 = f206(v0.clone(), v1);
    match m1414 {
        Adt108::C0 => {
            v4
        }
        Adt108::C1(v5) => {
            let m1415 = f218(v5);
            match m1415 {
                None => {
                    v4
                }
                Some(v6) => {
                    let a1416 = v0;
                    let a1417 = Str::lit("properties");
                    if equal(a1416, a1417) {
                        f477(match v6 {
                            Adt5::C0(h1418) => {
                                h1418
                            }
                        }, v2, v3, v4)
                    } else {
                        f478(match v6 {
                            Adt5::C0(h1419) => {
                                h1419
                            }
                        }, v4)
                    }
                }
            }
        }
    }
}

pub fn f400(v0: Str, v1: List<(Str, Adt6)>, v2: List<Adt106>) -> List<Adt106> {
    let m1420 = f206(v0, v1);
    match m1420 {
        Adt108::C0 => {
            v2
        }
        Adt108::C1(v3) => {
            List::cons(Adt106::C0(v3, false, List::<Str>::nil()), v2)
        }
    }
}

pub fn f401(v0: Str, v1: List<(Str, Adt6)>, v2: List<Adt106>) -> List<Adt106> {
    let m1421 = f206(v0, v1);
    match m1421 {
        Adt108::C0 => {
            v2
        }
        Adt108::C1(v3) => {
            let m1422 = f209(v3);
            match m1422 {
                None => {
                    v2
                }
                Some(v4) => {
                    f479(v4, v2)
                }
            }
        }
    }
}

pub fn f402(v0: Str) -> bool {
    let a1423 = v0.clone();
    let a1424 = Str::lit("null");
    if equal(a1423, a1424) {
        true
    } else {
        let a1425 = v0.clone();
        let a1426 = Str::lit("boolean");
        if equal(a1425, a1426) {
            true
        } else {
            let a1427 = v0.clone();
            let a1428 = Str::lit("object");
            if equal(a1427, a1428) {
                true
            } else {
                let a1429 = v0.clone();
                let a1430 = Str::lit("array");
                if equal(a1429, a1430) {
                    true
                } else {
                    let a1431 = v0.clone();
                    let a1432 = Str::lit("number");
                    if equal(a1431, a1432) {
                        true
                    } else {
                        let a1433 = v0.clone();
                        let a1434 = Str::lit("string");
                        if equal(a1433, a1434) {
                            true
                        } else {
                            let a1435 = v0;
                            let a1436 = Str::lit("integer");
                            equal(a1435, a1436)
                        }
                    }
                }
            }
        }
    }
}

pub fn f403(v0: Str, v1: Adt6) -> R<bool> {
    let a1437 = v0.clone();
    let a1438 = f480(v1.clone());
    if equal(a1437, a1438) {
        Ok(true)
    } else {
        let a1439 = v0;
        let a1440 = Str::lit("integer");
        if equal(a1439, a1440) {
            let m1441 = f363(v1);
            match m1441 {
                None => {
                    Ok(false)
                }
                Some(v2) => {
                    f481(v2)
                }
            }
        } else {
            Ok(false)
        }
    }
}

pub fn f404(p1444: List<Adt6>, v1: Adt6, p1445: bool) -> R<Adt132> {
    let m1443 = (p1444, p1445);
    {
        let mut m1443 = m1443;
        loop {
            let step = {
                let (v0, v2) = m1443;
                let m1442 = v0;
                match m1442.uncons() {
                    None => {
                        if v2 {
                            Err::<(List<Adt6>, bool), R<Adt132>>(Ok(Adt132::C0))
                        } else {
                            Err::<(List<Adt6>, bool), R<Adt132>>(Ok(Adt132::C1))
                        }
                    }
                    Some((v3, v4)) => {
                        if f402(f482(v3.clone())) {
                            Ok::<(List<Adt6>, bool), R<Adt132>>((v4, if v2 {
                                true
                            } else {
                                f403(f482(v3), v1.clone())?
                            }))
                        } else {
                            Err::<(List<Adt6>, bool), R<Adt132>>(Ok(Adt132::C2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1443 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f405(p1449: List<Adt6>, v1: Adt6) -> R<Adt132> {
    let m1448 = p1449;
    {
        let mut m1448 = m1448;
        loop {
            let step = {
                let v0 = m1448;
                let m1446 = v0;
                match m1446.uncons() {
                    None => {
                        Err::<List<Adt6>, R<Adt132>>(Ok(Adt132::C1))
                    }
                    Some((v2, v3)) => {
                        let m1447 = f219(v2, v1.clone())?;
                        match m1447 {
                            Adt116::C0 => {
                                Err::<List<Adt6>, R<Adt132>>(Ok(Adt132::C0))
                            }
                            Adt116::C1 => {
                                Ok::<List<Adt6>, R<Adt132>>(v3)
                            }
                            Adt116::C2 => {
                                Err::<List<Adt6>, R<Adt132>>(Ok(Adt132::C2))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1448 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f406(v0: u64, v1: Adt146) -> bool {
    let a1450 = v0;
    let a1451 = 0u64;
    if nat_eq(a1450, a1451) {
        let a1452 = f483(v1);
        bool_not(a1452)
    } else {
        let a1453 = v0;
        let a1454 = 1u64;
        if nat_eq(a1453, a1454) {
            let a1455 = f420(v1);
            bool_not(a1455)
        } else {
            let a1456 = v0;
            let a1457 = 2u64;
            if nat_eq(a1456, a1457) {
                f420(v1)
            } else {
                f483(v1)
            }
        }
    }
}

pub fn f407(v0: Str, v1: bool, v2: List<(Str, Adt6)>, v3: u64) -> R<Adt132> {
    let m1458 = f206(v0, v2);
    match m1458 {
        Adt108::C0 => {
            Ok(Adt132::C0)
        }
        Adt108::C1(v4) => {
            let m1459 = f484(v4);
            match m1459 {
                None => {
                    Ok(Adt132::C2)
                }
                Some(v5) => {
                    let m1462 = {
                        let a1460 = v5;
                        let a1461 = 0i64;
                        compare(a1460, a1461)
                    };
                    if match m1462 {
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
                        Ok(Adt132::C2)
                    } else {
                        if v1 {
                            if f483(f485(f167(v3, 0i64)?, v5)) {
                                Ok(Adt132::C1)
                            } else {
                                Ok(Adt132::C0)
                            }
                        } else {
                            if f420(f485(f167(v3, 0i64)?, v5)) {
                                Ok(Adt132::C1)
                            } else {
                                Ok(Adt132::C0)
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f408(v0: Str) -> R<u64> {
    f486(f18({
        let a1463 = v0;
        utf8_encode(a1463)
    })?, 0u64)
}

pub fn f409(p1468: List<Adt6>, p1469: u64) -> R<u64> {
    let m1467 = (p1468, p1469);
    {
        let mut m1467 = m1467;
        loop {
            let step = {
                let (v0, v1) = m1467;
                let m1464 = v0;
                match m1464.uncons() {
                    None => {
                        Err::<(List<Adt6>, u64), R<u64>>(Ok(v1))
                    }
                    Some((_, v3)) => {
                        Ok::<(List<Adt6>, u64), R<u64>>((v3, {
                            let a1465 = v1;
                            let a1466 = 1u64;
                            nat_add(a1465, a1466)?
                        }))
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

pub fn f410(p1473: List<Adt6>, v1: Adt5) -> Adt132 {
    let m1472 = p1473;
    {
        let mut m1472 = m1472;
        loop {
            let step = {
                let v0 = m1472;
                let m1470 = v0;
                match m1470.uncons() {
                    None => {
                        Err::<List<Adt6>, Adt132>(Adt132::C0)
                    }
                    Some((v2, v3)) => {
                        let m1471 = f89(v2);
                        match m1471 {
                            None => {
                                Err::<List<Adt6>, Adt132>(Adt132::C2)
                            }
                            Some(v4) => {
                                if f389(v4, v1.clone()) {
                                    Ok::<List<Adt6>, Adt132>(v3)
                                } else {
                                    Err::<List<Adt6>, Adt132>(Adt132::C1)
                                }
                            }
                        }
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

pub fn f411(v0: Adt6, v1: Str) -> Option<Adt6> {
    let a1474 = v1.clone();
    let a1475 = Str::lit("#");
    if equal(a1474, a1475) {
        Some(v0)
    } else {
        let m1476 = f218(v0);
        match m1476 {
            None => {
                None::<Adt6>
            }
            Some(v2) => {
                let m1477 = f138(Str::lit("$defs"), v2);
                match m1477 {
                    Adt108::C0 => {
                        None::<Adt6>
                    }
                    Adt108::C1(v3) => {
                        let m1478 = f218(v3);
                        match m1478 {
                            None => {
                                None::<Adt6>
                            }
                            Some(v4) => {
                                f487(v1, match v4 {
                                    Adt5::C0(h1479) => {
                                        h1479
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

pub fn f412(p1486: List<(Str, Adt6)>, v1: List<(Str, Adt6)>, v2: List<Adt100>, p1487: List<Adt102>) -> List<Adt102> {
    let m1485 = (p1486, p1487);
    {
        let mut m1485 = m1485;
        loop {
            let step = {
                let (v0, v3) = m1485;
                let m1480 = v0;
                match m1480.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Adt102>), List<Adt102>>(v3)
                    }
                    Some((v4, v5)) => {
                        let m1482 = f206({
                            let (h1481, _) = v4.clone();
                            h1481
                        }, v1.clone());
                        match m1482 {
                            Adt108::C0 => {
                                Ok::<(List<(Str, Adt6)>, List<Adt102>), List<Adt102>>((v5, v3))
                            }
                            Adt108::C1(v6) => {
                                Ok::<(List<(Str, Adt6)>, List<Adt102>), List<Adt102>>((v5, List::cons(Adt102::C0({
                                    let (_, h1483) = v4;
                                    h1483
                                }, v6, List::cons(Adt100::C0({
                                    let (h1484, _) = v4;
                                    h1484
                                }), v2.clone())), v3)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1485 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f413(p1492: List<(Str, Adt6)>, v1: List<(Str, Adt6)>) -> bool {
    let m1491 = p1492;
    {
        let mut m1491 = m1491;
        loop {
            let step = {
                let v0 = m1491;
                let m1488 = v0;
                match m1488.uncons() {
                    None => {
                        Err::<List<(Str, Adt6)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let m1490 = f206({
                            let (h1489, _) = v2;
                            h1489
                        }, v1.clone());
                        match m1490 {
                            Adt108::C0 => {
                                Err::<List<(Str, Adt6)>, bool>(true)
                            }
                            Adt108::C1(_) => {
                                Ok::<List<(Str, Adt6)>, bool>(v3)
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1491 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f414(v0: List<(Str, Adt6)>) -> List<(Str, Adt6)> {
    let m1493 = f206(Str::lit("properties"), v0);
    match m1493 {
        Adt108::C0 => {
            List::<(Str, Adt6)>::nil()
        }
        Adt108::C1(v1) => {
            let m1494 = f218(v1);
            match m1494 {
                None => {
                    List::<(Str, Adt6)>::nil()
                }
                Some(v2) => {
                    match v2 {
                        Adt5::C0(h1495) => {
                            h1495
                        }
                    }
                }
            }
        }
    }
}

pub fn f415(p1502: List<(Str, Adt6)>, v1: List<(Str, Adt6)>, v2: Adt6, v3: List<Adt100>, p1503: List<Adt102>) -> List<Adt102> {
    let m1501 = (p1502, p1503);
    {
        let mut m1501 = m1501;
        loop {
            let step = {
                let (v0, v4) = m1501;
                let m1496 = v0;
                match m1496.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Adt102>), List<Adt102>>(v4)
                    }
                    Some((v5, v6)) => {
                        let m1498 = f206({
                            let (h1497, _) = v5.clone();
                            h1497
                        }, v1.clone());
                        match m1498 {
                            Adt108::C0 => {
                                Ok::<(List<(Str, Adt6)>, List<Adt102>), List<Adt102>>((v6, List::cons(Adt102::C0(v2.clone(), {
                                    let (_, h1499) = v5;
                                    h1499
                                }, List::cons(Adt100::C0({
                                    let (h1500, _) = v5;
                                    h1500
                                }), v3.clone())), v4)))
                            }
                            Adt108::C1(_) => {
                                Ok::<(List<(Str, Adt6)>, List<Adt102>), List<Adt102>>((v6, v4))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1501 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f416(p1508: List<Adt6>, p1509: u64, v2: Adt6, v3: List<Adt100>, p1510: List<Adt102>) -> R<List<Adt102>> {
    let m1507 = (p1508, (p1509, p1510));
    {
        let mut m1507 = m1507;
        loop {
            let step = {
                let (v0, (v1, v4)) = m1507;
                let m1504 = v0;
                match m1504.uncons() {
                    None => {
                        Err::<(List<Adt6>, (u64, List<Adt102>)), R<List<Adt102>>>(Ok(v4))
                    }
                    Some((v5, v6)) => {
                        Ok::<(List<Adt6>, (u64, List<Adt102>)), R<List<Adt102>>>((v6, ({
                            let a1505 = v1;
                            let a1506 = 1u64;
                            nat_add(a1505, a1506)?
                        }, List::cons(Adt102::C0(v2.clone(), v5, List::cons(Adt100::C1(v1), v3.clone())), v4))))
                    }
                }
            };
            match step {
                Ok(next) => { m1507 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f417(p1513: List<Adt6>, v1: Adt6, v2: List<Adt100>, p1514: List<Adt102>) -> List<Adt102> {
    let m1512 = (p1513, p1514);
    {
        let mut m1512 = m1512;
        loop {
            let step = {
                let (v0, v3) = m1512;
                let m1511 = v0;
                match m1511.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<Adt102>), List<Adt102>>(v3)
                    }
                    Some((v4, v5)) => {
                        Ok::<(List<Adt6>, List<Adt102>), List<Adt102>>((v5, List::cons(Adt102::C0(v4, v1.clone(), v2.clone()), v3)))
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

pub fn f418(v0: u64, v1: u64, v2: u64, v3: u64, v4: bool) -> R<Option<List<u64>>> {
    let m1515 = f488(v0)?;
    match m1515 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v5) => {
            let m1516 = f488(v1)?;
            match m1516 {
                None => {
                    Ok(None::<List<u64>>)
                }
                Some(v6) => {
                    let a1517 = v2;
                    let a1518 = 61u64;
                    if nat_eq(a1517, a1518) {
                        f489(v5, v6, v4, v3)
                    } else {
                        let m1519 = f488(v2)?;
                        match m1519 {
                            None => {
                                Ok(None::<List<u64>>)
                            }
                            Some(v7) => {
                                let a1520 = v3;
                                let a1521 = 61u64;
                                if nat_eq(a1520, a1521) {
                                    f490(v5, v6, v7, v4)
                                } else {
                                    let m1522 = f488(v3)?;
                                    match m1522 {
                                        None => {
                                            Ok(None::<List<u64>>)
                                        }
                                        Some(v8) => {
                                            Ok(Some(f491(v5, v6, v7, v8)?))
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

pub fn f419(v0: List<u64>) -> bool {
    let m1523 = v0;
    match m1523.uncons() {
        None => {
            true
        }
        Some((_, _)) => {
            false
        }
    }
}

pub fn f420(v0: Adt146) -> bool {
    let m1524 = v0;
    match m1524 {
        Adt146::C0 => {
            false
        }
        Adt146::C1 => {
            false
        }
        Adt146::C2 => {
            true
        }
    }
}

pub fn f421(v0: u64) -> List<u64> {
    f440(v0)
}

pub fn f422(v0: Adt7) -> R<bool> {
    let a1542 = {
        let m1528 = {
            let a1526 = f492();
            let a1527 = match v0.clone() {
                Adt7::C0(h1525, _) => {
                    h1525
                }
            };
            compare(a1526, a1527)
        };
        if match m1528 {
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
            let m1533 = {
                let a1531 = match v0.clone() {
                    Adt7::C0(h1529, _) => {
                        h1529
                    }
                };
                let a1532 = {
                    let a1530 = f492();
                    int_neg(a1530)?
                };
                compare(a1531, a1532)
            };
            if match m1533 {
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
                let m1537 = {
                    let a1535 = 9999i64;
                    let a1536 = match v0.clone() {
                        Adt7::C0(_, h1534) => {
                            h1534
                        }
                    };
                    compare(a1535, a1536)
                };
                if match m1537 {
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
                    let m1541 = {
                        let a1539 = match v0 {
                            Adt7::C0(_, h1538) => {
                                h1538
                            }
                        };
                        let a1540 = -9999i64;
                        compare(a1539, a1540)
                    };
                    match m1541 {
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
    Ok(bool_not(a1542))
}

pub fn f423(v0: Adt7) -> R<List<u64>> {
    let m1546 = {
        let a1544 = match v0.clone() {
            Adt7::C0(_, h1543) => {
                h1543
            }
        };
        let a1545 = 0i64;
        compare(a1544, a1545)
    };
    if if match m1546 {
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
        let m1550 = {
            let a1548 = 0i64;
            let a1549 = match v0.clone() {
                Adt7::C0(_, h1547) => {
                    h1547
                }
            };
            compare(a1548, a1549)
        };
        match m1550 {
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
        let a1553 = f493(match v0.clone() {
            Adt7::C0(h1551, _) => {
                h1551
            }
        })?;
        let a1554 = List::cons(101u64, f493(match v0 {
            Adt7::C0(_, h1552) => {
                h1552
            }
        })?);
        Ok(append_list(a1553, a1554))
    } else {
        f493(match v0 {
            Adt7::C0(h1555, _) => {
                h1555
            }
        })
    }
}

pub fn f424() -> u64 {
    512u64
}

pub fn f425(v0: u64, v1: Adt115, v2: List<Adt115>) -> List<Adt115> {
    List::cons(Adt115::C0(List::cons(v0, List::<u64>::nil())), List::cons(v1, v2))
}

pub fn f426(p1558: List<u64>, p1559: List<u64>) -> R<List<u64>> {
    let m1557 = (p1558, p1559);
    {
        let mut m1557 = m1557;
        loop {
            let step = {
                let (v0, v1) = m1557;
                let m1556 = v0;
                match m1556.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), R<List<u64>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), R<List<u64>>>((v3, f263(f494(v2)?, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1557 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f427(v0: Adt125) -> bool {
    let m1560 = v0;
    match m1560 {
        Adt125::C0 => {
            false
        }
        Adt125::C1 => {
            true
        }
        Adt125::C2 => {
            true
        }
        Adt125::C3 => {
            false
        }
        Adt125::C4 => {
            true
        }
        Adt125::C5 => {
            false
        }
        Adt125::C6 => {
            false
        }
        Adt125::C7 => {
            true
        }
    }
}

pub fn f428(v0: Adt121, v1: Adt126, v2: Adt122, v3: u64) -> R<Adt134> {
    let a1562 = match v0.clone() {
        Adt121::C0(_, _, h1561) => {
            h1561
        }
    };
    let a1563 = f424();
    if nat_lt(a1562, a1563) {
        Ok(Adt134::C1(Adt121::C0(v2, List::cons(v1, match v0.clone() {
            Adt121::C0(_, h1564, _) => {
                h1564
            }
        }), {
            let a1566 = match v0 {
                Adt121::C0(_, _, h1565) => {
                    h1565
                }
            };
            let a1567 = 1u64;
            nat_add(a1566, a1567)?
        })))
    } else {
        Ok(f358(Adt104::C9(v3)))
    }
}

pub fn f429(v0: bool, v1: Adt125, v2: i64, v3: u64, v4: i64, v5: bool, v6: i64) -> Adt122 {
    Adt122::C8(Adt124::C0(v0, v1, v2, v3, v4, v5, v6))
}

pub fn f430(v0: u64) -> i64 {
    let a1568 = v0;
    let a1569 = 0u64;
    if nat_eq(a1568, a1569) {
        0i64
    } else {
        let a1570 = v0;
        let a1571 = 1u64;
        if nat_eq(a1570, a1571) {
            1i64
        } else {
            let a1572 = v0;
            let a1573 = 2u64;
            if nat_eq(a1572, a1573) {
                2i64
            } else {
                let a1574 = v0;
                let a1575 = 3u64;
                if nat_eq(a1574, a1575) {
                    3i64
                } else {
                    let a1576 = v0;
                    let a1577 = 4u64;
                    if nat_eq(a1576, a1577) {
                        4i64
                    } else {
                        let a1578 = v0;
                        let a1579 = 5u64;
                        if nat_eq(a1578, a1579) {
                            5i64
                        } else {
                            let a1580 = v0;
                            let a1581 = 6u64;
                            if nat_eq(a1580, a1581) {
                                6i64
                            } else {
                                let a1582 = v0;
                                let a1583 = 7u64;
                                if nat_eq(a1582, a1583) {
                                    7i64
                                } else {
                                    let a1584 = v0;
                                    let a1585 = 8u64;
                                    if nat_eq(a1584, a1585) {
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

pub fn f431(p1588: List<Adt6>, p1589: List<Adt6>) -> List<Adt6> {
    let m1587 = (p1588, p1589);
    {
        let mut m1587 = m1587;
        loop {
            let step = {
                let (v0, v1) = m1587;
                let m1586 = v0;
                match m1586.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt6>, List<Adt6>), List<Adt6>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1587 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f432(v0: Adt121, v1: u64) -> Adt134 {
    let m1591 = match v0.clone() {
        Adt121::C0(_, h1590, _) => {
            h1590
        }
    };
    match m1591.uncons() {
        None => {
            f358(Adt104::C2(v1))
        }
        Some((v2, _)) => {
            let m1592 = v2;
            match m1592 {
                Adt126::C0(_) => {
                    f357(v0, Adt122::C0)
                }
                Adt126::C1(_, _) => {
                    f357(v0, Adt122::C4)
                }
            }
        }
    }
}

pub fn f433(p1595: List<(Str, Adt6)>, p1596: List<(Str, Adt6)>) -> List<(Str, Adt6)> {
    let m1594 = (p1595, p1596);
    {
        let mut m1594 = m1594;
        loop {
            let step = {
                let (v0, v1) = m1594;
                let m1593 = v0;
                match m1593.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<(Str, Adt6)>), List<(Str, Adt6)>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Adt6)>, List<(Str, Adt6)>), List<(Str, Adt6)>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1594 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f434(v0: Adt121, v1: Adt122) -> Adt121 {
    Adt121::C0(v1, match v0.clone() {
        Adt121::C0(_, h1597, _) => {
            h1597
        }
    }, match v0 {
        Adt121::C0(_, _, h1598) => {
            h1598
        }
    })
}

pub fn f435(v0: Adt121, v1: u64, v2: List<u64>, v3: bool) -> Adt134 {
    let m1600 = {
        let a1599 = f104(f105(), f263(v2, List::<u64>::nil()));
        utf8_decode(a1599)
    };
    match m1600 {
        None => {
            f358(Adt104::C14)
        }
        Some(v4) => {
            if v3 {
                f495(v0, v1, v4)
            } else {
                f441(v0, Adt6::C3(v4))
            }
        }
    }
}

pub fn f436(v0: u64) -> u64 {
    let a1601 = v0;
    let a1602 = 34u64;
    if nat_eq(a1601, a1602) {
        34u64
    } else {
        let a1603 = v0;
        let a1604 = 92u64;
        if nat_eq(a1603, a1604) {
            92u64
        } else {
            let a1605 = v0;
            let a1606 = 47u64;
            if nat_eq(a1605, a1606) {
                47u64
            } else {
                let a1607 = v0;
                let a1608 = 98u64;
                if nat_eq(a1607, a1608) {
                    8u64
                } else {
                    let a1609 = v0;
                    let a1610 = 102u64;
                    if nat_eq(a1609, a1610) {
                        12u64
                    } else {
                        let a1611 = v0;
                        let a1612 = 110u64;
                        if nat_eq(a1611, a1612) {
                            10u64
                        } else {
                            let a1613 = v0;
                            let a1614 = 114u64;
                            if nat_eq(a1613, a1614) {
                                13u64
                            } else {
                                let a1615 = v0;
                                let a1616 = 116u64;
                                if nat_eq(a1615, a1616) {
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

pub fn f437(v0: u64) -> u64 {
    let a1617 = v0;
    let a1618 = 48u64;
    if nat_lt(a1617, a1618) {
        16u64
    } else {
        let a1619 = v0;
        let a1620 = 58u64;
        if nat_lt(a1619, a1620) {
            let a1621 = v0;
            let a1622 = 48u64;
            nat_sub(a1621, a1622)
        } else {
            let a1623 = v0;
            let a1624 = 65u64;
            if nat_lt(a1623, a1624) {
                16u64
            } else {
                let a1625 = v0;
                let a1626 = 71u64;
                if nat_lt(a1625, a1626) {
                    let a1627 = v0;
                    let a1628 = 55u64;
                    nat_sub(a1627, a1628)
                } else {
                    let a1629 = v0;
                    let a1630 = 97u64;
                    if nat_lt(a1629, a1630) {
                        16u64
                    } else {
                        let a1631 = v0;
                        let a1632 = 103u64;
                        if nat_lt(a1631, a1632) {
                            let a1633 = v0;
                            let a1634 = 87u64;
                            nat_sub(a1633, a1634)
                        } else {
                            16u64
                        }
                    }
                }
            }
        }
    }
}

pub fn f438(v0: Adt121, v1: u64, v2: u64, v3: u64, v4: List<u64>, v5: bool) -> R<Adt134> {
    let a1635 = v3;
    let a1636 = 0u64;
    if nat_eq(a1635, a1636) {
        if f349(v2, 55296u64, 56319u64) {
            Ok(f357(v0, Adt122::C6(Adt123::C3(v2), v4, v5)))
        } else {
            if f349(v2, 56320u64, 57343u64) {
                Ok(f358(Adt104::C6(v1)))
            } else {
                Ok(f357(v0, Adt122::C6(Adt123::C0, f496(v2, v4)?, v5)))
            }
        }
    } else {
        if f349(v2, 56320u64, 57343u64) {
            Ok(f357(v0, Adt122::C6(Adt123::C0, f496({
                let a1645 = 65536u64;
                let a1646 = {
                    let a1643 = {
                        let a1639 = {
                            let a1637 = v3;
                            let a1638 = 55296u64;
                            nat_sub(a1637, a1638)
                        };
                        let a1640 = 1024u64;
                        nat_mul(a1639, a1640)?
                    };
                    let a1644 = {
                        let a1641 = v2;
                        let a1642 = 56320u64;
                        nat_sub(a1641, a1642)
                    };
                    nat_add(a1643, a1644)?
                };
                nat_add(a1645, a1646)?
            }, v4)?, v5)))
        } else {
            Ok(f358(Adt104::C6(v1)))
        }
    }
}

pub fn f439(v0: u64, v1: u64) -> u64 {
    let m1649 = {
        let a1647 = f440(v0);
        let a1648 = v1;
        index_list(a1647, a1648)
    };
    match m1649 {
        None => {
            0u64
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f440(v0: u64) -> List<u64> {
    let a1650 = v0;
    let a1651 = 0u64;
    if nat_eq(a1650, a1651) {
        List::cons(116u64, List::cons(114u64, List::cons(117u64, List::cons(101u64, List::<u64>::nil()))))
    } else {
        let a1652 = v0;
        let a1653 = 1u64;
        if nat_eq(a1652, a1653) {
            List::cons(102u64, List::cons(97u64, List::cons(108u64, List::cons(115u64, List::cons(101u64, List::<u64>::nil())))))
        } else {
            List::cons(110u64, List::cons(117u64, List::cons(108u64, List::cons(108u64, List::<u64>::nil()))))
        }
    }
}

pub fn f441(v0: Adt121, v1: Adt6) -> Adt134 {
    Adt134::C1(f351(v1, match v0.clone() {
        Adt121::C0(_, h1654, _) => {
            h1654
        }
    }, match v0 {
        Adt121::C0(_, _, h1655) => {
            h1655
        }
    }))
}

pub fn f442(v0: u64) -> Adt6 {
    let a1656 = v0;
    let a1657 = 0u64;
    if nat_eq(a1656, a1657) {
        Adt6::C1(true)
    } else {
        let a1658 = v0;
        let a1659 = 1u64;
        if nat_eq(a1658, a1659) {
            Adt6::C1(false)
        } else {
            Adt6::C0
        }
    }
}

pub fn f443(v0: Adt121, v1: Adt124, v2: Adt125) -> Adt134 {
    f357(v0, Adt122::C8(Adt124::C0(match v1.clone() {
        Adt124::C0(h1660, _, _, _, _, _, _) => {
            h1660
        }
    }, v2, match v1.clone() {
        Adt124::C0(_, _, h1661, _, _, _, _) => {
            h1661
        }
    }, match v1.clone() {
        Adt124::C0(_, _, _, h1662, _, _, _) => {
            h1662
        }
    }, match v1.clone() {
        Adt124::C0(_, _, _, _, h1663, _, _) => {
            h1663
        }
    }, match v1.clone() {
        Adt124::C0(_, _, _, _, _, h1664, _) => {
            h1664
        }
    }, match v1 {
        Adt124::C0(_, _, _, _, _, _, h1665) => {
            h1665
        }
    })))
}

pub fn f444(v0: Adt121, v1: u64, v2: u64, v3: Adt124, v4: bool) -> R<Adt134> {
    let v5: u64 = {
        let a1667 = match v3.clone() {
            Adt124::C0(_, _, _, h1666, _, _, _) => {
                h1666
            }
        };
        let a1668 = 0u64;
        if if nat_eq(a1667, a1668) {
            let a1669 = v2;
            let a1670 = 48u64;
            nat_eq(a1669, a1670)
        } else {
            false
        } {
            0u64
        } else {
            let a1672 = match v3.clone() {
                Adt124::C0(_, _, _, h1671, _, _, _) => {
                    h1671
                }
            };
            let a1673 = 1u64;
            nat_add(a1672, a1673)?
        }
    };
    let v6: i64 = if v4 {
        f216(match v3.clone() {
            Adt124::C0(_, _, _, _, h1674, _, _) => {
                h1674
            }
        }, 1i64)?
    } else {
        match v3.clone() {
            Adt124::C0(_, _, _, _, h1675, _, _) => {
                h1675
            }
        }
    };
    let a1676 = f497();
    let a1677 = v5;
    if nat_lt(a1676, a1677) {
        Ok(f358(Adt104::C8(v1)))
    } else {
        let m1680 = {
            let a1678 = 99999i64;
            let a1679 = v6;
            compare(a1678, a1679)
        };
        if match m1680 {
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
            Ok(f358(Adt104::C8(v1)))
        } else {
            Ok(f357(v0, f429(match v3.clone() {
                Adt124::C0(h1681, _, _, _, _, _, _) => {
                    h1681
                }
            }, if v4 {
                Adt125::C4
            } else {
                Adt125::C2
            }, f216({
                let a1683 = match v3.clone() {
                    Adt124::C0(_, _, h1682, _, _, _, _) => {
                        h1682
                    }
                };
                let a1684 = 10i64;
                int_mul(a1683, a1684)?
            }, f430({
                let a1685 = v2;
                let a1686 = 48u64;
                nat_sub(a1685, a1686)
            }))?, v5, v6, match v3.clone() {
                Adt124::C0(_, _, _, _, _, h1687, _) => {
                    h1687
                }
            }, match v3 {
                Adt124::C0(_, _, _, _, _, _, h1688) => {
                    h1688
                }
            })))
        }
    }
}

pub fn f445(v0: u64) -> bool {
    let a1689 = v0;
    let a1690 = 101u64;
    if nat_eq(a1689, a1690) {
        true
    } else {
        let a1691 = v0;
        let a1692 = 69u64;
        nat_eq(a1691, a1692)
    }
}

pub fn f446(v0: u64) -> bool {
    f349(v0, 48u64, 57u64)
}

pub fn f447(v0: Adt121, v1: u64, v2: u64, v3: Adt124) -> R<Adt134> {
    let m1693 = f350(v3)?;
    match m1693 {
        Adt145::C0 => {
            Ok(f358(Adt104::C7(v1)))
        }
        Adt145::C1 => {
            Ok(f358(Adt104::C8(v1)))
        }
        Adt145::C2(v4) => {
            Ok(f355(f351(v4, match v0.clone() {
                Adt121::C0(_, h1694, _) => {
                    h1694
                }
            }, match v0 {
                Adt121::C0(_, _, h1695) => {
                    h1695
                }
            }), v1, v2))
        }
    }
}

pub fn f448(v0: Adt121, v1: u64, v2: u64, v3: Adt124, v4: bool) -> R<Adt134> {
    let v5: i64 = f216({
        let a1697 = match v3.clone() {
            Adt124::C0(_, _, _, _, _, _, h1696) => {
                h1696
            }
        };
        let a1698 = 10i64;
        int_mul(a1697, a1698)?
    }, f430({
        let a1699 = v2;
        let a1700 = 48u64;
        nat_sub(a1699, a1700)
    }))?;
    let m1703 = {
        let a1701 = 99999i64;
        let a1702 = v5;
        compare(a1701, a1702)
    };
    if match m1703 {
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
        Ok(f358(Adt104::C8(v1)))
    } else {
        Ok(f357(v0, f429(match v3.clone() {
            Adt124::C0(h1704, _, _, _, _, _, _) => {
                h1704
            }
        }, Adt125::C7, match v3.clone() {
            Adt124::C0(_, _, h1705, _, _, _, _) => {
                h1705
            }
        }, match v3.clone() {
            Adt124::C0(_, _, _, h1706, _, _, _) => {
                h1706
            }
        }, match v3 {
            Adt124::C0(_, _, _, _, h1707, _, _) => {
                h1707
            }
        }, v4, v5)))
    }
}

pub fn f449(v0: Adt7, v1: Adt7, v2: Adt146, v3: Adt146) -> R<Adt146> {
    let m1708 = v2;
    match m1708 {
        Adt146::C0 => {
            let m1710 = v3;
            match m1710 {
                Adt146::C0 => {
                    Ok(f498(Adt146::C0, f499(f500(match v0.clone() {
                        Adt7::C0(h1711, _) => {
                            h1711
                        }
                    })?, match v0 {
                        Adt7::C0(_, h1712) => {
                            h1712
                        }
                    }, f500(match v1.clone() {
                        Adt7::C0(h1713, _) => {
                            h1713
                        }
                    })?, match v1 {
                        Adt7::C0(_, h1714) => {
                            h1714
                        }
                    })?))
                }
                Adt146::C1 => {
                    Ok(Adt146::C0)
                }
                Adt146::C2 => {
                    Ok(Adt146::C0)
                }
            }
        }
        Adt146::C1 => {
            let m1709 = v3;
            match m1709 {
                Adt146::C0 => {
                    Ok(Adt146::C2)
                }
                Adt146::C1 => {
                    Ok(Adt146::C1)
                }
                Adt146::C2 => {
                    Ok(Adt146::C0)
                }
            }
        }
        Adt146::C2 => {
            let m1715 = v3;
            match m1715 {
                Adt146::C0 => {
                    Ok(Adt146::C2)
                }
                Adt146::C1 => {
                    Ok(Adt146::C2)
                }
                Adt146::C2 => {
                    f499(match v0.clone() {
                        Adt7::C0(h1716, _) => {
                            h1716
                        }
                    }, match v0 {
                        Adt7::C0(_, h1717) => {
                            h1717
                        }
                    }, match v1.clone() {
                        Adt7::C0(h1718, _) => {
                            h1718
                        }
                    }, match v1 {
                        Adt7::C0(_, h1719) => {
                            h1719
                        }
                    })
                }
            }
        }
    }
}

pub fn f450(v0: i64) -> Adt146 {
    f485(v0, 0i64)
}

pub fn f451(v0: Adt86) -> Option<Adt87> {
    let m1720 = v0;
    match m1720 {
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

pub fn f452(v0: u64) -> bool {
    let a1721 = v0;
    let a1722 = f501();
    nat_lt(a1721, a1722)
}

pub fn f453() -> Adt140 {
    Adt140::C4(Adt86::C9(Str::lit("the effect tag does not fit thirty-two bits")))
}

pub fn f454(v0: Adt137) -> R<Option<Adt5>> {
    let m1724 = f502(match v0.clone() {
        Adt137::C0(h1723, _, _) => {
            h1723
        }
    })?;
    match m1724 {
        None => {
            Ok(None::<Adt5>)
        }
        Some(v1) => {
            Ok(Some(Adt5::C0({
                let a1727 = v1;
                let a1728 = {
                    let m1726 = f503(v0);
                    match m1726 {
                        Adt5::C0(h1725) => {
                            h1725
                        }
                    }
                };
                append_list(a1727, a1728)
            })))
        }
    }
}

pub fn f455(v0: bool, v1: Adt95, v2: Option<List<List<u64>>>) -> Adt95 {
    let m1729 = v2;
    match m1729 {
        None => {
            f44()
        }
        Some(v3) => {
            f504(v0, v1, f118(v3, List::<List<u64>>::nil()))
        }
    }
}

pub fn f456() -> u64 {
    200u64
}

pub fn f457(v0: Adt105, v1: Adt5) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("jsonrpc"), Adt6::C3(f270())), List::cons((Str::lit("id"), f307(v0)), List::cons((Str::lit("result"), Adt6::C5(match v1 {
        Adt5::C0(h1730) => {
            h1730
        }
    })), List::<(Str, Adt6)>::nil()))))
}

pub fn f458(p1734: List<Adt139>, p1735: List<List<u64>>) -> R<Option<List<List<u64>>>> {
    let m1733 = (p1734, p1735);
    {
        let mut m1733 = m1733;
        loop {
            let step = {
                let (v0, v1) = m1733;
                let m1731 = v0;
                match m1731.uncons() {
                    None => {
                        Err::<(List<Adt139>, List<List<u64>>), R<Option<List<List<u64>>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m1732 = f505(v2)?;
                        match m1732 {
                            None => {
                                Err::<(List<Adt139>, List<List<u64>>), R<Option<List<List<u64>>>>>(Ok(None::<List<List<u64>>>))
                            }
                            Some(v4) => {
                                Ok::<(List<Adt139>, List<List<u64>>), R<Option<List<List<u64>>>>>((v3, List::cons(v4, v1)))
                            }
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

pub fn f459(v0: Adt140) -> u64 {
    let m1736 = v0;
    match m1736 {
        Adt140::C0(_) => {
            f226()
        }
        Adt140::C1(_, _) => {
            f226()
        }
        Adt140::C2(_) => {
            f226()
        }
        Adt140::C3(_) => {
            f506()
        }
        Adt140::C4(v6) => {
            f507(v6)
        }
    }
}

pub fn f460(v0: Adt140) -> R<Adt82> {
    Ok(Adt82::C0(f508(v0.clone()), f509(v0.clone()), f510(v0)?))
}

pub fn f461(v0: List<u64>) -> R<bool> {
    let m1737 = f375(59u64, v0, List::<u64>::nil(), List::<List<u64>>::nil());
    match m1737.uncons() {
        None => {
            Ok(false)
        }
        Some((v1, v2)) => {
            if f511(v2)? {
                Ok(false)
            } else {
                f280(f282(v1), f512())
            }
        }
    }
}

pub fn f462(v0: Adt0, v1: Fn2, v2: u64, v3: Adt147) -> R<Result<Adt137, Adt140>> {
    f513(v0.clone(), v1, v2, v3.clone(), f514(match v0 {
        Adt0::C0(_, _, h1738, _, _, _, _) => {
            h1738
        }
    }, match v3 {
        Adt147::C0(_, h1739, _, _) => {
            h1739
        }
    }))
}

pub fn f463(v0: Adt149, v1: Adt150) -> Result<Adt147, Adt140> {
    let m1740 = v1;
    match m1740 {
        Adt150::C0(v3) => {
            f515(v3, v0)
        }
        Adt150::C1(v2) => {
            Err::<Adt147, Adt140>(v2)
        }
    }
}

pub fn f464(v0: Str) -> Adt149 {
    let a1741 = v0.clone();
    let a1742 = Str::lit("server/discover");
    if equal(a1741, a1742) {
        Adt149::C1(Adt148::C0)
    } else {
        let a1743 = v0.clone();
        let a1744 = Str::lit("tools/list");
        if equal(a1743, a1744) {
            Adt149::C1(Adt148::C1)
        } else {
            let a1745 = v0.clone();
            let a1746 = Str::lit("tools/call");
            if equal(a1745, a1746) {
                Adt149::C1(Adt148::C2)
            } else {
                let a1747 = v0.clone();
                let a1748 = Str::lit("resources/list");
                if equal(a1747, a1748) {
                    Adt149::C1(Adt148::C3)
                } else {
                    let a1749 = v0.clone();
                    let a1750 = Str::lit("resources/read");
                    if equal(a1749, a1750) {
                        Adt149::C1(Adt148::C4)
                    } else {
                        let a1751 = v0.clone();
                        let a1752 = Str::lit("resources/templates/list");
                        if equal(a1751, a1752) {
                            Adt149::C1(Adt148::C5)
                        } else {
                            let a1753 = v0.clone();
                            let a1754 = Str::lit("prompts/list");
                            if equal(a1753, a1754) {
                                Adt149::C1(Adt148::C6)
                            } else {
                                let a1755 = v0.clone();
                                let a1756 = Str::lit("prompts/get");
                                if equal(a1755, a1756) {
                                    Adt149::C1(Adt148::C7)
                                } else {
                                    let a1757 = v0.clone();
                                    let a1758 = Str::lit("completion/complete");
                                    if equal(a1757, a1758) {
                                        Adt149::C1(Adt148::C8)
                                    } else {
                                        let a1759 = v0.clone();
                                        let a1760 = Str::lit("tasks/get");
                                        if equal(a1759, a1760) {
                                            Adt149::C1(Adt148::C9)
                                        } else {
                                            let a1761 = v0.clone();
                                            let a1762 = Str::lit("tasks/update");
                                            if equal(a1761, a1762) {
                                                Adt149::C1(Adt148::C10)
                                            } else {
                                                let a1763 = v0;
                                                let a1764 = Str::lit("tasks/cancel");
                                                if equal(a1763, a1764) {
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

pub fn f465(v0: Option<Adt127>, v1: List<Str>, v2: Adt117) -> R<Adt150> {
    f516(v0, v1, v2.clone(), f517(match v2 {
        Adt117::C0(_, _, h1765) => {
            h1765
        }
    }))
}

pub fn f466(v0: List<u64>) -> Option<Str> {
    let a1766 = f104(f105(), v0);
    utf8_decode(a1766)
}

pub fn f467(v0: Str) -> Adt152 {
    let m1767 = f464(v0);
    match m1767 {
        Adt149::C0 => {
            Adt152::C0
        }
        Adt149::C1(v1) => {
            f518(v1)
        }
    }
}

pub fn f468(v0: List<u64>) -> Option<List<u64>> {
    let m1768 = f519(f520(), v0);
    match m1768 {
        None => {
            None::<List<u64>>
        }
        Some(v1) => {
            let m1769 = f519(f521(), f290(v1, List::<u64>::nil()));
            match m1769 {
                None => {
                    None::<List<u64>>
                }
                Some(v2) => {
                    Some(f290(v2, List::<u64>::nil()))
                }
            }
        }
    }
}

pub fn f469(p1772: List<Adt130>, p1773: List<Adt130>) -> List<Adt130> {
    let m1771 = (p1772, p1773);
    {
        let mut m1771 = m1771;
        loop {
            let step = {
                let (v0, v1) = m1771;
                let m1770 = v0;
                match m1770.uncons() {
                    None => {
                        Err::<(List<Adt130>, List<Adt130>), List<Adt130>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt130>, List<Adt130>), List<Adt130>>((v3, List::cons(v2, v1)))
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

pub fn f470(v0: List<Adt92>, p1780: List<Adt12>, v2: Option<Str>, p1781: List<Adt130>) -> R<List<Adt130>> {
    let m1779 = (p1780, p1781);
    {
        let mut m1779 = m1779;
        loop {
            let step = {
                let (v1, v3) = m1779;
                let m1774 = v1;
                match m1774.uncons() {
                    None => {
                        Err::<(List<Adt12>, List<Adt130>), R<List<Adt130>>>(Ok(v3))
                    }
                    Some((v4, v5)) => {
                        if f522(v4.clone(), v2.clone()) {
                            Ok::<(List<Adt12>, List<Adt130>), R<List<Adt130>>>((v5, List::cons(Adt130::C0(match v4.clone() {
                                Adt12::C0(_, h1775, _, _) => {
                                    h1775
                                }
                            }, match v4.clone() {
                                Adt12::C0(_, _, h1776, _) => {
                                    h1776
                                }
                            }, match v4.clone() {
                                Adt12::C0(_, _, _, h1777) => {
                                    h1777
                                }
                            }, f523(v0.clone(), match v4 {
                                Adt12::C0(_, h1778, _, _) => {
                                    h1778
                                }
                            })?), v3)))
                        } else {
                            Ok::<(List<Adt12>, List<Adt130>), R<List<Adt130>>>((v5, v3))
                        }
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

pub fn f471(p1797: u64, p1798: i64, p1799: i64) -> R<Option<i64>> {
    let m1796 = (p1797, (p1798, p1799));
    {
        let mut m1796 = m1796;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1796;
                let m1782 = v0;
                if m1782 == 0 {
                    Err::<(u64, (i64, i64)), R<Option<i64>>>(Ok(None::<i64>))
                } else {
                    let v3 = m1782 - 1;
                    if f524(v2) {
                        Err::<(u64, (i64, i64)), R<Option<i64>>>(Ok(Some(v1)))
                    } else {
                        let m1785 = {
                            let a1783 = v2;
                            let a1784 = 0i64;
                            compare(a1783, a1784)
                        };
                        if match m1785 {
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
                            if f524({
                                let a1786 = v1;
                                let a1787 = 10i64;
                                let a1788 = 0i64;
                                int_rem(a1786, a1787, a1788)
                            }) {
                                Ok::<(u64, (i64, i64)), R<Option<i64>>>((v3, ({
                                    let a1789 = v1;
                                    let a1790 = 10i64;
                                    let a1791 = 0i64;
                                    int_quot(a1789, a1790, a1791)?
                                }, f216(v2, 1i64)?)))
                            } else {
                                Err::<(u64, (i64, i64)), R<Option<i64>>>(Ok(None::<i64>))
                            }
                        } else {
                            Ok::<(u64, (i64, i64)), R<Option<i64>>>((v3, ({
                                let a1792 = v1;
                                let a1793 = 10i64;
                                int_mul(a1792, a1793)?
                            }, {
                                let a1794 = v2;
                                let a1795 = 1i64;
                                int_sub(a1794, a1795)?
                            })))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1796 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f472(v0: i64) -> Option<i64> {
    let m1802 = {
        let a1800 = 9007199254740991i64;
        let a1801 = v0;
        compare(a1800, a1801)
    };
    if if match m1802 {
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
        let m1805 = {
            let a1803 = v0;
            let a1804 = -9007199254740991i64;
            compare(a1803, a1804)
        };
        match m1805 {
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

pub fn f473(v0: Adt108) -> bool {
    let m1806 = v0;
    match m1806 {
        Adt108::C0 => {
            false
        }
        Adt108::C1(_) => {
            true
        }
    }
}

pub fn f474(v0: Adt105, v1: Str, v2: Adt5) -> Result<Adt117, Adt109> {
    let m1807 = f138(Str::lit("params"), v2);
    match m1807 {
        Adt108::C0 => {
            Ok::<Adt117, Adt109>(Adt117::C0(v0, v1, None::<Adt5>))
        }
        Adt108::C1(v3) => {
            let m1808 = f218(v3);
            match m1808 {
                None => {
                    f279(Some(v0), Adt110::C6)
                }
                Some(v4) => {
                    Ok::<Adt117, Adt109>(Adt117::C0(v0, v1, Some(v4)))
                }
            }
        }
    }
}

pub fn f475(v0: u64, v1: Adt6) -> bool {
    let a1809 = v0;
    let a1810 = 0u64;
    if nat_eq(a1809, a1810) {
        f525(v1)
    } else {
        let a1811 = v0;
        let a1812 = 1u64;
        if nat_eq(a1811, a1812) {
            f526(v1)
        } else {
            let a1813 = v0;
            let a1814 = 2u64;
            if nat_eq(a1813, a1814) {
                f527(v1)
            } else {
                let a1815 = v0;
                let a1816 = 3u64;
                if nat_eq(a1815, a1816) {
                    f528(v1)
                } else {
                    let a1817 = v0;
                    let a1818 = 4u64;
                    if nat_eq(a1817, a1818) {
                        f529(v1)
                    } else {
                        let a1819 = v0;
                        let a1820 = 5u64;
                        if nat_eq(a1819, a1820) {
                            f530(v1)
                        } else {
                            let a1821 = v0;
                            let a1822 = 6u64;
                            if nat_eq(a1821, a1822) {
                                f531(v1)
                            } else {
                                let a1823 = v0;
                                let a1824 = 7u64;
                                if nat_eq(a1823, a1824) {
                                    f532(v1)
                                } else {
                                    f533(v1)
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f476(v0: Str, v1: List<(Str, Adt6)>, v2: Str, v3: List<u64>, v4: List<Str>, v5: Adt113) -> R<Adt144> {
    let m1825 = v3.clone();
    match m1825.uncons() {
        None => {
            Ok(Adt144::C0(Adt93::C8(v0)))
        }
        Some((_, _)) => {
            if f242(v3.clone(), true) {
                let m1826 = f534(v1);
                match m1826 {
                    None => {
                        Ok(Adt144::C0(Adt93::C11(v0)))
                    }
                    Some(v8) => {
                        let v9: List<u64> = f535(v3)?;
                        if f536(v9.clone(), match v5.clone() {
                            Adt113::C0(_, h1827) => {
                                h1827
                            }
                        }) {
                            Ok(Adt144::C0(Adt93::C12(v0, v2)))
                        } else {
                            Ok(Adt144::C1(Adt113::C0(List::cons(Adt12::C0(v0, v2, f537(v4, List::<Str>::nil()), v8), match v5.clone() {
                                Adt113::C0(h1828, _) => {
                                    h1828
                                }
                            }), List::cons(v9, match v5 {
                                Adt113::C0(_, h1829) => {
                                    h1829
                                }
                            }))))
                        }
                    }
                }
            } else {
                Ok(Adt144::C0(Adt93::C9(v0)))
            }
        }
    }
}

pub fn f477(p1834: List<(Str, Adt6)>, v1: bool, v2: List<Str>, p1835: List<Adt106>) -> List<Adt106> {
    let m1833 = (p1834, p1835);
    {
        let mut m1833 = m1833;
        loop {
            let step = {
                let (v0, v3) = m1833;
                let m1830 = v0;
                match m1830.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Adt106>), List<Adt106>>(v3)
                    }
                    Some((v4, v5)) => {
                        Ok::<(List<(Str, Adt6)>, List<Adt106>), List<Adt106>>((v5, List::cons(Adt106::C0({
                            let (_, h1831) = v4;
                            h1831
                        }, v1, List::cons({
                            let (h1832, _) = v4;
                            h1832
                        }, v2.clone())), v3)))
                    }
                }
            };
            match step {
                Ok(next) => { m1833 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f478(p1839: List<(Str, Adt6)>, p1840: List<Adt106>) -> List<Adt106> {
    let m1838 = (p1839, p1840);
    {
        let mut m1838 = m1838;
        loop {
            let step = {
                let (v0, v1) = m1838;
                let m1836 = v0;
                match m1836.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Adt106>), List<Adt106>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Adt6)>, List<Adt106>), List<Adt106>>((v3, List::cons(Adt106::C0({
                            let (_, h1837) = v2;
                            h1837
                        }, false, List::<Str>::nil()), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1838 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f479(p1843: List<Adt6>, p1844: List<Adt106>) -> List<Adt106> {
    let m1842 = (p1843, p1844);
    {
        let mut m1842 = m1842;
        loop {
            let step = {
                let (v0, v1) = m1842;
                let m1841 = v0;
                match m1841.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<Adt106>), List<Adt106>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt6>, List<Adt106>), List<Adt106>>((v3, List::cons(Adt106::C0(v2, false, List::<Str>::nil()), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1842 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f480(v0: Adt6) -> Str {
    let m1845 = v0;
    match m1845 {
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

pub fn f481(v0: Adt7) -> R<bool> {
    if f524(match v0.clone() {
        Adt7::C0(h1846, _) => {
            h1846
        }
    }) {
        Ok(true)
    } else {
        f538(40u64, match v0.clone() {
            Adt7::C0(h1847, _) => {
                h1847
            }
        }, match v0 {
            Adt7::C0(_, h1848) => {
                h1848
            }
        })
    }
}

pub fn f482(v0: Adt6) -> Str {
    let m1849 = f89(v0);
    match m1849 {
        None => {
            Str::lit("")
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f483(v0: Adt146) -> bool {
    let m1850 = v0;
    match m1850 {
        Adt146::C0 => {
            true
        }
        Adt146::C1 => {
            false
        }
        Adt146::C2 => {
            false
        }
    }
}

pub fn f484(v0: Adt6) -> Option<i64> {
    let m1851 = v0;
    match m1851 {
        Adt6::C0 => {
            None::<i64>
        }
        Adt6::C1(_) => {
            None::<i64>
        }
        Adt6::C2(v2) => {
            if f524(match v2.clone() {
                Adt7::C0(_, h1852) => {
                    h1852
                }
            }) {
                Some(match v2 {
                    Adt7::C0(h1853, _) => {
                        h1853
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

pub fn f485(v0: i64, v1: i64) -> Adt146 {
    let m1856 = {
        let a1854 = v0;
        let a1855 = v1;
        compare(a1854, a1855)
    };
    if match m1856 {
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
        Adt146::C0
    } else {
        let m1859 = {
            let a1857 = v1;
            let a1858 = v0;
            compare(a1857, a1858)
        };
        if match m1859 {
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
            Adt146::C2
        } else {
            Adt146::C1
        }
    }
}

pub fn f486(p1868: List<u64>, p1869: u64) -> R<u64> {
    let m1867 = (p1868, p1869);
    {
        let mut m1867 = m1867;
        loop {
            let step = {
                let (v0, v1) = m1867;
                let m1860 = v0;
                match m1860.uncons() {
                    None => {
                        Err::<(List<u64>, u64), R<u64>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        let a1861 = v2;
                        let a1862 = 128u64;
                        if if nat_lt(a1861, a1862) {
                            true
                        } else {
                            let a1863 = 191u64;
                            let a1864 = v2;
                            nat_lt(a1863, a1864)
                        } {
                            Ok::<(List<u64>, u64), R<u64>>((v3, {
                                let a1865 = v1;
                                let a1866 = 1u64;
                                nat_add(a1865, a1866)?
                            }))
                        } else {
                            Ok::<(List<u64>, u64), R<u64>>((v3, v1))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1867 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f487(v0: Str, p1887: List<(Str, Adt6)>) -> Option<Adt6> {
    let m1886 = p1887;
    {
        let mut m1886 = m1886;
        loop {
            let step = {
                let v1 = m1886;
                let m1870 = v1;
                match m1870.uncons() {
                    None => {
                        Err::<List<(Str, Adt6)>, Option<Adt6>>(None::<Adt6>)
                    }
                    Some((v2, v3)) => {
                        let a1874 = v0.clone();
                        let a1875 = {
                            let a1872 = List::cons(Str::lit("#/$defs/"), List::cons({
                                let (h1871, _) = v2.clone();
                                h1871
                            }, List::<Str>::nil()));
                            let a1873 = Str::lit("");
                            join(a1872, a1873)
                        };
                        if equal(a1874, a1875) {
                            Err::<List<(Str, Adt6)>, Option<Adt6>>(Some({
                                let (_, h1876) = v2;
                                h1876
                            }))
                        } else {
                            let m1878 = f218({
                                let (_, h1877) = v2.clone();
                                h1877
                            });
                            match m1878 {
                                None => {
                                    Ok::<List<(Str, Adt6)>, Option<Adt6>>(v3)
                                }
                                Some(v4) => {
                                    let m1879 = f138(Str::lit("$anchor"), v4);
                                    match m1879 {
                                        Adt108::C0 => {
                                            Ok::<List<(Str, Adt6)>, Option<Adt6>>(v3)
                                        }
                                        Adt108::C1(v5) => {
                                            let m1880 = f89(v5);
                                            match m1880 {
                                                None => {
                                                    Ok::<List<(Str, Adt6)>, Option<Adt6>>(v3)
                                                }
                                                Some(v6) => {
                                                    let a1883 = v0.clone();
                                                    let a1884 = {
                                                        let a1881 = List::cons(Str::lit("#"), List::cons(v6, List::<Str>::nil()));
                                                        let a1882 = Str::lit("");
                                                        join(a1881, a1882)
                                                    };
                                                    if equal(a1883, a1884) {
                                                        Err::<List<(Str, Adt6)>, Option<Adt6>>(Some({
                                                            let (_, h1885) = v2;
                                                            h1885
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
                Ok(next) => { m1886 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f488(v0: u64) -> R<Option<u64>> {
    let a1888 = 64u64;
    let a1889 = v0;
    if if nat_lt(a1888, a1889) {
        let a1890 = v0;
        let a1891 = 91u64;
        nat_lt(a1890, a1891)
    } else {
        false
    } {
        Ok(Some({
            let a1892 = v0;
            let a1893 = 65u64;
            nat_sub(a1892, a1893)
        }))
    } else {
        let a1894 = 96u64;
        let a1895 = v0;
        if if nat_lt(a1894, a1895) {
            let a1896 = v0;
            let a1897 = 123u64;
            nat_lt(a1896, a1897)
        } else {
            false
        } {
            Ok(Some({
                let a1898 = v0;
                let a1899 = 71u64;
                nat_sub(a1898, a1899)
            }))
        } else {
            let a1900 = 47u64;
            let a1901 = v0;
            if if nat_lt(a1900, a1901) {
                let a1902 = v0;
                let a1903 = 58u64;
                nat_lt(a1902, a1903)
            } else {
                false
            } {
                Ok(Some({
                    let a1904 = v0;
                    let a1905 = 4u64;
                    nat_add(a1904, a1905)?
                }))
            } else {
                let a1906 = v0;
                let a1907 = 43u64;
                if nat_eq(a1906, a1907) {
                    Ok(Some(62u64))
                } else {
                    let a1908 = v0;
                    let a1909 = 47u64;
                    if nat_eq(a1908, a1909) {
                        Ok(Some(63u64))
                    } else {
                        Ok(None::<u64>)
                    }
                }
            }
        }
    }
}

pub fn f489(v0: u64, v1: u64, v2: bool, v3: u64) -> R<Option<List<u64>>> {
    if if v2 {
        let a1910 = v3;
        let a1911 = 61u64;
        if nat_eq(a1910, a1911) {
            let a1912 = f539(v1, 16u64);
            let a1913 = 0u64;
            nat_eq(a1912, a1913)
        } else {
            false
        }
    } else {
        false
    } {
        Ok(Some(f540(v0, v1)?))
    } else {
        Ok(None::<List<u64>>)
    }
}

pub fn f490(v0: u64, v1: u64, v2: u64, v3: bool) -> R<Option<List<u64>>> {
    if if v3 {
        let a1914 = f539(v2, 4u64);
        let a1915 = 0u64;
        nat_eq(a1914, a1915)
    } else {
        false
    } {
        Ok(Some(f541(v0, v1, v2)?))
    } else {
        Ok(None::<List<u64>>)
    }
}

pub fn f491(v0: u64, v1: u64, v2: u64, v3: u64) -> R<List<u64>> {
    Ok(List::cons({
        let a1918 = {
            let a1916 = v0;
            let a1917 = 4u64;
            nat_mul(a1916, a1917)?
        };
        let a1919 = f542(v1, 16u64);
        nat_add(a1918, a1919)?
    }, List::cons({
        let a1922 = {
            let a1920 = f539(v1, 16u64);
            let a1921 = 16u64;
            nat_mul(a1920, a1921)?
        };
        let a1923 = f542(v2, 4u64);
        nat_add(a1922, a1923)?
    }, List::cons({
        let a1926 = {
            let a1924 = f539(v2, 4u64);
            let a1925 = 64u64;
            nat_mul(a1924, a1925)?
        };
        let a1927 = v3;
        nat_add(a1926, a1927)?
    }, List::<u64>::nil()))))
}

pub fn f492() -> i64 {
    999999999999999999i64
}

pub fn f493(v0: i64) -> R<List<u64>> {
    let m1930 = {
        let a1928 = v0;
        let a1929 = 0i64;
        compare(a1928, a1929)
    };
    if match m1930 {
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
        Ok(List::cons(45u64, f543(20u64, {
            let a1931 = v0;
            int_neg(a1931)?
        }, List::<u64>::nil())?))
    } else {
        f543(20u64, v0, List::<u64>::nil())
    }
}

pub fn f494(v0: u64) -> R<List<u64>> {
    let a1932 = v0;
    let a1933 = 34u64;
    if nat_eq(a1932, a1933) {
        Ok(List::cons(92u64, List::cons(34u64, List::<u64>::nil())))
    } else {
        let a1934 = v0;
        let a1935 = 92u64;
        if nat_eq(a1934, a1935) {
            Ok(List::cons(92u64, List::cons(92u64, List::<u64>::nil())))
        } else {
            let a1936 = v0;
            let a1937 = 8u64;
            if nat_eq(a1936, a1937) {
                Ok(List::cons(92u64, List::cons(98u64, List::<u64>::nil())))
            } else {
                let a1938 = v0;
                let a1939 = 9u64;
                if nat_eq(a1938, a1939) {
                    Ok(List::cons(92u64, List::cons(116u64, List::<u64>::nil())))
                } else {
                    let a1940 = v0;
                    let a1941 = 10u64;
                    if nat_eq(a1940, a1941) {
                        Ok(List::cons(92u64, List::cons(110u64, List::<u64>::nil())))
                    } else {
                        let a1942 = v0;
                        let a1943 = 12u64;
                        if nat_eq(a1942, a1943) {
                            Ok(List::cons(92u64, List::cons(102u64, List::<u64>::nil())))
                        } else {
                            let a1944 = v0;
                            let a1945 = 13u64;
                            if nat_eq(a1944, a1945) {
                                Ok(List::cons(92u64, List::cons(114u64, List::<u64>::nil())))
                            } else {
                                let a1946 = v0;
                                let a1947 = 32u64;
                                if nat_lt(a1946, a1947) {
                                    Ok(List::cons(92u64, List::cons(117u64, List::cons(48u64, List::cons(48u64, List::cons(f544(f545(v0, 16u64))?, List::cons(f544(f546(v0, 16u64))?, List::<u64>::nil())))))))
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

pub fn f495(v0: Adt121, v1: u64, v2: Str) -> Adt134 {
    let m1949 = match v0.clone() {
        Adt121::C0(_, h1948, _) => {
            h1948
        }
    };
    match m1949.uncons() {
        None => {
            f358(Adt104::C2(v1))
        }
        Some((v3, v4)) => {
            let m1950 = v3;
            match m1950 {
                Adt126::C0(_) => {
                    f358(Adt104::C2(v1))
                }
                Adt126::C1(v6, _) => {
                    Adt134::C1(Adt121::C0(Adt122::C5, List::cons(Adt126::C1(v6, Some(v2)), v4), match v0 {
                        Adt121::C0(_, _, h1951) => {
                            h1951
                        }
                    }))
                }
            }
        }
    }
}

pub fn f496(v0: u64, v1: List<u64>) -> R<List<u64>> {
    let a1952 = v0;
    let a1953 = 128u64;
    if nat_lt(a1952, a1953) {
        Ok(List::cons(v0, v1))
    } else {
        let a1954 = v0;
        let a1955 = 2048u64;
        if nat_lt(a1954, a1955) {
            Ok(List::cons({
                let a1956 = 128u64;
                let a1957 = f546(v0, 64u64);
                nat_add(a1956, a1957)?
            }, List::cons({
                let a1958 = 192u64;
                let a1959 = f545(v0, 64u64);
                nat_add(a1958, a1959)?
            }, v1)))
        } else {
            let a1960 = v0;
            let a1961 = 65536u64;
            if nat_lt(a1960, a1961) {
                Ok(List::cons({
                    let a1962 = 128u64;
                    let a1963 = f546(v0, 64u64);
                    nat_add(a1962, a1963)?
                }, List::cons({
                    let a1964 = 128u64;
                    let a1965 = f546(f545(v0, 64u64), 64u64);
                    nat_add(a1964, a1965)?
                }, List::cons({
                    let a1966 = 224u64;
                    let a1967 = f545(v0, 4096u64);
                    nat_add(a1966, a1967)?
                }, v1))))
            } else {
                Ok(List::cons({
                    let a1968 = 128u64;
                    let a1969 = f546(v0, 64u64);
                    nat_add(a1968, a1969)?
                }, List::cons({
                    let a1970 = 128u64;
                    let a1971 = f546(f545(v0, 64u64), 64u64);
                    nat_add(a1970, a1971)?
                }, List::cons({
                    let a1972 = 128u64;
                    let a1973 = f546(f545(v0, 4096u64), 64u64);
                    nat_add(a1972, a1973)?
                }, List::cons({
                    let a1974 = 240u64;
                    let a1975 = f545(v0, 262144u64);
                    nat_add(a1974, a1975)?
                }, v1)))))
            }
        }
    }
}

pub fn f497() -> u64 {
    18u64
}

pub fn f498(v0: Adt146, v1: Adt146) -> Adt146 {
    let m1976 = v0;
    match m1976 {
        Adt146::C0 => {
            f547(v1)
        }
        Adt146::C1 => {
            Adt146::C1
        }
        Adt146::C2 => {
            v1
        }
    }
}

pub fn f499(v0: i64, v1: i64, v2: i64, v3: i64) -> R<Adt146> {
    let m1977 = f485(f548(v0, v1)?, f548(v2, v3)?);
    match m1977 {
        Adt146::C0 => {
            Ok(Adt146::C0)
        }
        Adt146::C1 => {
            f549(v0, f550(v0)?, v2, f550(v2)?)
        }
        Adt146::C2 => {
            Ok(Adt146::C2)
        }
    }
}

pub fn f500(v0: i64) -> R<i64> {
    let m1980 = {
        let a1978 = v0;
        let a1979 = 0i64;
        compare(a1978, a1979)
    };
    if match m1980 {
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
        let a1981 = v0;
        int_neg(a1981)
    } else {
        Ok(v0)
    }
}

pub fn f501() -> u64 {
    4294967296u64
}

pub fn f502(v0: Adt34) -> R<Option<List<(Str, Adt6)>>> {
    let m1982 = v0;
    match m1982 {
        Adt34::C0(v1) => {
            Ok(Some(f551(v1)?))
        }
        Adt34::C1(v2) => {
            Ok(Some(f552(v2)?))
        }
        Adt34::C2(v3) => {
            f553(f554(v3)?)
        }
        Adt34::C3(v4) => {
            f553(f554(match v4 {
                Adt51::C0(h1983, _) => {
                    h1983
                }
            })?)
        }
        Adt34::C4(v5) => {
            Ok(Some(f555(v5)?))
        }
        Adt34::C5(v6) => {
            Ok(Some(f556(v6)?))
        }
        Adt34::C6(v7) => {
            Ok(Some(f557(v7)?))
        }
        Adt34::C7(v8) => {
            Ok(Some(f558(v8)?))
        }
        Adt34::C8(v9) => {
            f553(f559(v9)?)
        }
        Adt34::C9(v10) => {
            Ok(Some(f560(v10)?))
        }
        Adt34::C10(_) => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Adt34::C11 => {
            Ok(Some(List::<(Str, Adt6)>::nil()))
        }
        Adt34::C12(v12) => {
            f553(f561(v12)?)
        }
        Adt34::C13(_) => {
            Ok(None::<List<(Str, Adt6)>>)
        }
    }
}

pub fn f503(v0: Adt137) -> Adt5 {
    Adt5::C0(List::cons((Str::lit("resultType"), Adt6::C3(f562(f563(match v0.clone() {
        Adt137::C0(h1984, _, _) => {
            h1984
        }
    })))), List::cons((Str::lit("_meta"), Adt6::C5(f564({
        let m1987 = match v0 {
            Adt137::C0(_, h1986, _) => {
                h1986
            }
        };
        match m1987 {
            Adt138::C0(h1985, _) => {
                h1985
            }
        }
    }))), List::<(Str, Adt6)>::nil())))
}

pub fn f504(v0: bool, v1: Adt95, v2: List<List<u64>>) -> Adt95 {
    if f565(v0, match v1.clone() {
        Adt95::C0(h1988, _, _, _) => {
            h1988
        }
    }, v2.clone()) {
        f566(match v1.clone() {
            Adt95::C0(h1989, _, _, _) => {
                h1989
            }
        }, {
            let a1991 = v2;
            let a1992 = List::cons(match v1 {
                Adt95::C0(_, _, _, h1990) => {
                    h1990
                }
            }, List::<List<u64>>::nil());
            append_list(a1991, a1992)
        })
    } else {
        v1
    }
}

pub fn f505(v0: Adt139) -> R<Option<List<u64>>> {
    let m1993 = f567(f568(v0))?;
    match m1993 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v1) => {
            let m1994 = f141(v1)?;
            match m1994 {
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

pub fn f506() -> u64 {
    404u64
}

pub fn f507(v0: Adt86) -> u64 {
    let m1995 = v0;
    match m1995 {
        Adt86::C0(_) => {
            f226()
        }
        Adt86::C1(_) => {
            f226()
        }
        Adt86::C2(_) => {
            f226()
        }
        Adt86::C3(_) => {
            f226()
        }
        Adt86::C4(_) => {
            f226()
        }
        Adt86::C5(_) => {
            f226()
        }
        Adt86::C6(_) => {
            f72()
        }
        Adt86::C7(_) => {
            f569()
        }
        Adt86::C8(_) => {
            f226()
        }
        Adt86::C9(_) => {
            f72()
        }
        Adt86::C10 => {
            f226()
        }
        Adt86::C11 => {
            f226()
        }
        Adt86::C12(_) => {
            f72()
        }
        Adt86::C13 => {
            f72()
        }
        Adt86::C14 => {
            f72()
        }
        Adt86::C15(_) => {
            f72()
        }
    }
}

pub fn f508(v0: Adt140) -> i64 {
    let m1996 = v0;
    match m1996 {
        Adt140::C0(_) => {
            f570()
        }
        Adt140::C1(_, _) => {
            f571()
        }
        Adt140::C2(_) => {
            f572()
        }
        Adt140::C3(_) => {
            f573()
        }
        Adt140::C4(v6) => {
            f574(v6)
        }
    }
}

pub fn f509(v0: Adt140) -> Str {
    let m1997 = v0;
    match m1997 {
        Adt140::C0(v1) => {
            f575(v1)
        }
        Adt140::C1(_, _) => {
            Str::lit("Unsupported protocol version")
        }
        Adt140::C2(v4) => {
            f576(v4)
        }
        Adt140::C3(_) => {
            Str::lit("Method not found")
        }
        Adt140::C4(v6) => {
            let m1998 = v6;
            match m1998 {
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

pub fn f510(v0: Adt140) -> R<Option<Adt6>> {
    let m1999 = v0;
    match m1999 {
        Adt140::C0(_) => {
            Ok(None::<Adt6>)
        }
        Adt140::C1(v2, v3) => {
            Ok(Some(Adt6::C5(List::cons((Str::lit("supported"), Adt6::C4(f577(v3))), List::cons((Str::lit("requested"), Adt6::C3(v2)), List::<(Str, Adt6)>::nil())))))
        }
        Adt140::C2(_) => {
            Ok(None::<Adt6>)
        }
        Adt140::C3(_) => {
            Ok(None::<Adt6>)
        }
        Adt140::C4(v6) => {
            let m2000 = v6;
            match m2000 {
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
                    Ok(Some(Adt6::C5(List::cons((Str::lit("limit"), Adt6::C2(Adt7::C0(f167(v14, 0i64)?, 0i64))), List::<(Str, Adt6)>::nil()))))
                }
                Adt86::C8(v15) => {
                    Ok(Some(Adt6::C5(List::cons((Str::lit("requiredCapabilities"), f578(v15)), List::<(Str, Adt6)>::nil()))))
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

pub fn f511(p2003: List<List<u64>>) -> R<bool> {
    let m2002 = p2003;
    {
        let mut m2002 = m2002;
        loop {
            let step = {
                let v0 = m2002;
                let m2001 = v0;
                match m2001.uncons() {
                    None => {
                        Err::<List<List<u64>>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        if f579(f282(v1))? {
                            Err::<List<List<u64>>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<List<u64>>, R<bool>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2002 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f512() -> List<u64> {
    List::cons(116u64, List::cons(101u64, List::cons(120u64, List::cons(116u64, List::cons(47u64, List::cons(101u64, List::cons(118u64, List::cons(101u64, List::cons(110u64, List::cons(116u64, List::cons(45u64, List::cons(115u64, List::cons(116u64, List::cons(114u64, List::cons(101u64, List::cons(97u64, List::cons(109u64, List::<u64>::nil())))))))))))))))))
}

pub fn f513(v0: Adt0, v1: Fn2, v2: u64, v3: Adt147, v4: bool) -> R<Result<Adt137, Adt140>> {
    if v4 {
        f580(v0, v1, v2, f581(match v3.clone() {
            Adt147::C0(_, h2004, _, _) => {
                h2004
            }
        }, match v3.clone() {
            Adt147::C0(_, _, h2005, _) => {
                h2005
            }
        }, match v3 {
            Adt147::C0(_, _, _, h2006) => {
                h2006
            }
        }))
    } else {
        Ok(Err::<Adt137, Adt140>(Adt140::C3(f582(match v3 {
            Adt147::C0(_, h2007, _, _) => {
                h2007
            }
        }))))
    }
}

pub fn f514(v0: Adt4, v1: Adt148) -> bool {
    let m2008 = v1;
    match m2008 {
        Adt148::C0 => {
            true
        }
        Adt148::C1 => {
            f583(match v0 {
                Adt4::C0(_, _, _, _, _, h2009, _) => {
                    h2009
                }
            })
        }
        Adt148::C2 => {
            f583(match v0 {
                Adt4::C0(_, _, _, _, _, h2010, _) => {
                    h2010
                }
            })
        }
        Adt148::C3 => {
            f584(match v0 {
                Adt4::C0(_, _, _, _, h2011, _, _) => {
                    h2011
                }
            })
        }
        Adt148::C4 => {
            f584(match v0 {
                Adt4::C0(_, _, _, _, h2012, _, _) => {
                    h2012
                }
            })
        }
        Adt148::C5 => {
            f584(match v0 {
                Adt4::C0(_, _, _, _, h2013, _, _) => {
                    h2013
                }
            })
        }
        Adt148::C6 => {
            f583(match v0 {
                Adt4::C0(_, _, _, h2014, _, _, _) => {
                    h2014
                }
            })
        }
        Adt148::C7 => {
            f583(match v0 {
                Adt4::C0(_, _, _, h2015, _, _, _) => {
                    h2015
                }
            })
        }
        Adt148::C8 => {
            f585(match v0 {
                Adt4::C0(_, _, h2016, _, _, _, _) => {
                    h2016
                }
            })
        }
        Adt148::C9 => {
            f586(match v0 {
                Adt4::C0(_, _, _, _, _, _, h2017) => {
                    h2017
                }
            }, f587())
        }
        Adt148::C10 => {
            f586(match v0 {
                Adt4::C0(_, _, _, _, _, _, h2018) => {
                    h2018
                }
            }, f587())
        }
        Adt148::C11 => {
            f586(match v0 {
                Adt4::C0(_, _, _, _, _, _, h2019) => {
                    h2019
                }
            }, f587())
        }
    }
}

pub fn f515(v0: Adt151, v1: Adt149) -> Result<Adt147, Adt140> {
    let m2020 = v1;
    match m2020 {
        Adt149::C0 => {
            Err::<Adt147, Adt140>(Adt140::C3(match v0 {
                Adt151::C0(_, h2021, _, _) => {
                    h2021
                }
            }))
        }
        Adt149::C1(v2) => {
            Ok::<Adt147, Adt140>(Adt147::C0(match v0.clone() {
                Adt151::C0(h2022, _, _, _) => {
                    h2022
                }
            }, v2, match v0.clone() {
                Adt151::C0(_, _, h2023, _) => {
                    h2023
                }
            }, match v0 {
                Adt151::C0(_, _, _, h2024) => {
                    h2024
                }
            }))
        }
    }
}

pub fn f516(v0: Option<Adt127>, v1: List<Str>, v2: Adt117, v3: Result<Adt18, Adt141>) -> R<Adt150> {
    let m2025 = v3;
    match m2025 {
        Ok(v5) => {
            Ok(f588(v1, v2.clone(), v5.clone(), f589(v0, f467(match v2.clone() {
                Adt117::C0(_, h2026, _) => {
                    h2026
                }
            }), match v2.clone() {
                Adt117::C0(_, h2027, _) => {
                    h2027
                }
            }, match v5 {
                Adt18::C0(h2028, _, _, _, _, _) => {
                    h2028
                }
            }, f590(match v2 {
                Adt117::C0(_, _, h2029) => {
                    h2029
                }
            }))?))
        }
        Err(v4) => {
            Ok(Adt150::C1(Adt140::C0(v4)))
        }
    }
}

pub fn f517(v0: Option<Adt5>) -> Result<Adt18, Adt141> {
    let m2030 = v0;
    match m2030 {
        None => {
            Err::<Adt18, Adt141>(Adt141::C0)
        }
        Some(v1) => {
            let m2031 = f138(Str::lit("_meta"), v1);
            match m2031 {
                Adt108::C0 => {
                    Err::<Adt18, Adt141>(Adt141::C1)
                }
                Adt108::C1(v2) => {
                    let m2032 = f218(v2);
                    match m2032 {
                        None => {
                            Err::<Adt18, Adt141>(Adt141::C4)
                        }
                        Some(v3) => {
                            f591(v3)
                        }
                    }
                }
            }
        }
    }
}

pub fn f518(v0: Adt148) -> Adt152 {
    let m2033 = v0;
    match m2033 {
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

pub fn f519(p2039: List<u64>, p2040: List<u64>) -> Option<List<u64>> {
    let m2038 = (p2039, p2040);
    {
        let mut m2038 = m2038;
        loop {
            let step = {
                let (v0, v1) = m2038;
                let m2034 = v0;
                match m2034.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), Option<List<u64>>>(Some(v1))
                    }
                    Some((v2, v3)) => {
                        let m2035 = v1;
                        match m2035.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), Option<List<u64>>>(None::<List<u64>>)
                            }
                            Some((v4, v5)) => {
                                let a2036 = v4;
                                let a2037 = v2;
                                if nat_eq(a2036, a2037) {
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
                Ok(next) => { m2038 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f520() -> List<u64> {
    List::cons(61u64, List::cons(63u64, List::cons(98u64, List::cons(97u64, List::cons(115u64, List::cons(101u64, List::cons(54u64, List::cons(52u64, List::cons(63u64, List::<u64>::nil())))))))))
}

pub fn f521() -> List<u64> {
    List::cons(61u64, List::cons(63u64, List::<u64>::nil()))
}

pub fn f522(v0: Adt12, v1: Option<Str>) -> bool {
    let m2041 = v1;
    match m2041 {
        None => {
            false
        }
        Some(v2) => {
            let a2043 = match v0 {
                Adt12::C0(h2042, _, _, _) => {
                    h2042
                }
            };
            let a2044 = v2;
            equal(a2043, a2044)
        }
    }
}

pub fn f523(v0: List<Adt92>, v1: Str) -> R<Adt131> {
    let m2047 = f382(f184(v0, {
        let a2045 = f592();
        let a2046 = f97(v1)?;
        append_list(a2045, a2046)
    })?)?;
    match m2047 {
        Adt111::C0 => {
            Ok(Adt131::C0)
        }
        Adt111::C1 => {
            Ok(Adt131::C1)
        }
        Adt111::C2 => {
            Ok(Adt131::C1)
        }
        Adt111::C3(v2) => {
            let m2048 = f466(v2);
            match m2048 {
                None => {
                    Ok(Adt131::C1)
                }
                Some(v3) => {
                    Ok(Adt131::C2(v3))
                }
            }
        }
    }
}

pub fn f524(v0: i64) -> bool {
    let a2055 = {
        let m2051 = {
            let a2049 = v0;
            let a2050 = 0i64;
            compare(a2049, a2050)
        };
        if match m2051 {
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
            let m2054 = {
                let a2052 = 0i64;
                let a2053 = v0;
                compare(a2052, a2053)
            };
            match m2054 {
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
    bool_not(a2055)
}

pub fn f525(v0: Adt6) -> bool {
    let m2056 = f89(v0);
    match m2056 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f526(v0: Adt6) -> bool {
    let m2057 = f363(v0);
    match m2057 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f527(v0: Adt6) -> bool {
    let m2058 = f484(v0);
    match m2058 {
        None => {
            false
        }
        Some(v1) => {
            let a2062 = {
                let m2061 = {
                    let a2059 = v1;
                    let a2060 = 0i64;
                    compare(a2059, a2060)
                };
                match m2061 {
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
            bool_not(a2062)
        }
    }
}

pub fn f528(v0: Adt6) -> bool {
    let m2063 = f209(v0);
    match m2063 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f529(v0: Adt6) -> bool {
    let m2064 = f218(v0);
    match m2064 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f530(v0: Adt6) -> bool {
    let m2065 = f209(v0);
    match m2065 {
        None => {
            false
        }
        Some(v1) => {
            f593(v1)
        }
    }
}

pub fn f531(v0: Adt6) -> bool {
    let m2066 = f160(v0);
    match m2066 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f532(v0: Adt6) -> bool {
    let m2067 = f89(v0.clone());
    match m2067 {
        None => {
            let m2068 = f209(v0);
            match m2068 {
                None => {
                    false
                }
                Some(v2) => {
                    f594(v2)
                }
            }
        }
        Some(v1) => {
            f402(v1)
        }
    }
}

pub fn f533(v0: Adt6) -> bool {
    let m2069 = f209(v0);
    match m2069 {
        None => {
            false
        }
        Some(v1) => {
            let m2070 = v1;
            match m2070.uncons() {
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

pub fn f534(v0: List<(Str, Adt6)>) -> Option<Adt13> {
    let m2071 = f206(Str::lit("type"), v0);
    match m2071 {
        Adt108::C0 => {
            None::<Adt13>
        }
        Adt108::C1(v1) => {
            let m2072 = f89(v1);
            match m2072 {
                None => {
                    None::<Adt13>
                }
                Some(v2) => {
                    f595(v2)
                }
            }
        }
    }
}

pub fn f535(v0: List<u64>) -> R<List<u64>> {
    Ok(f263(f596(v0, List::<u64>::nil())?, List::<u64>::nil()))
}

pub fn f536(v0: List<u64>, p2075: List<List<u64>>) -> bool {
    let m2074 = p2075;
    {
        let mut m2074 = m2074;
        loop {
            let step = {
                let v1 = m2074;
                let m2073 = v1;
                match m2073.uncons() {
                    None => {
                        Err::<List<List<u64>>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f597(v0.clone(), v2) {
                            Err::<List<List<u64>>, bool>(true)
                        } else {
                            Ok::<List<List<u64>>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2074 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f537(p2078: List<Str>, p2079: List<Str>) -> List<Str> {
    let m2077 = (p2078, p2079);
    {
        let mut m2077 = m2077;
        loop {
            let step = {
                let (v0, v1) = m2077;
                let m2076 = v0;
                match m2076.uncons() {
                    None => {
                        Err::<(List<Str>, List<Str>), List<Str>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Str>, List<Str>), List<Str>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2077 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f538(p2091: u64, p2092: i64, p2093: i64) -> R<bool> {
    let m2090 = (p2091, (p2092, p2093));
    {
        let mut m2090 = m2090;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2090;
                let m2080 = v0;
                if m2080 == 0 {
                    Err::<(u64, (i64, i64)), R<bool>>(Ok(false))
                } else {
                    let v3 = m2080 - 1;
                    let m2083 = {
                        let a2081 = v2;
                        let a2082 = 0i64;
                        compare(a2081, a2082)
                    };
                    if match m2083 {
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
                        if f524({
                            let a2084 = v1;
                            let a2085 = 10i64;
                            let a2086 = 0i64;
                            int_rem(a2084, a2085, a2086)
                        }) {
                            Ok::<(u64, (i64, i64)), R<bool>>((v3, ({
                                let a2087 = v1;
                                let a2088 = 10i64;
                                let a2089 = 0i64;
                                int_quot(a2087, a2088, a2089)?
                            }, f216(v2, 1i64)?)))
                        } else {
                            Err::<(u64, (i64, i64)), R<bool>>(Ok(false))
                        }
                    } else {
                        Err::<(u64, (i64, i64)), R<bool>>(Ok(true))
                    }
                }
            };
            match step {
                Ok(next) => { m2090 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f539(v0: u64, v1: u64) -> u64 {
    let a2094 = v0;
    let a2095 = v1;
    let a2096 = 0u64;
    nat_rem(a2094, a2095, a2096)
}

pub fn f540(v0: u64, v1: u64) -> R<List<u64>> {
    Ok(List::cons({
        let a2099 = {
            let a2097 = v0;
            let a2098 = 4u64;
            nat_mul(a2097, a2098)?
        };
        let a2100 = f542(v1, 16u64);
        nat_add(a2099, a2100)?
    }, List::<u64>::nil()))
}

pub fn f541(v0: u64, v1: u64, v2: u64) -> R<List<u64>> {
    Ok(List::cons({
        let a2103 = {
            let a2101 = v0;
            let a2102 = 4u64;
            nat_mul(a2101, a2102)?
        };
        let a2104 = f542(v1, 16u64);
        nat_add(a2103, a2104)?
    }, List::cons({
        let a2107 = {
            let a2105 = f539(v1, 16u64);
            let a2106 = 16u64;
            nat_mul(a2105, a2106)?
        };
        let a2108 = f542(v2, 4u64);
        nat_add(a2107, a2108)?
    }, List::<u64>::nil())))
}

pub fn f542(v0: u64, v1: u64) -> u64 {
    let a2109 = v0;
    let a2110 = v1;
    let a2111 = 0u64;
    nat_quot(a2109, a2110, a2111)
}

pub fn f543(p2123: u64, p2124: i64, p2125: List<u64>) -> R<List<u64>> {
    let m2122 = (p2123, (p2124, p2125));
    {
        let mut m2122 = m2122;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2122;
                let m2112 = v0;
                if m2112 == 0 {
                    Err::<(u64, (i64, List<u64>)), R<List<u64>>>(Ok(v2))
                } else {
                    let v3 = m2112 - 1;
                    let v4: i64 = {
                        let a2113 = v1;
                        let a2114 = 10i64;
                        let a2115 = 0i64;
                        int_quot(a2113, a2114, a2115)?
                    };
                    let v5: i64 = {
                        let a2116 = v1;
                        let a2117 = 10i64;
                        let a2118 = 0i64;
                        int_rem(a2116, a2117, a2118)
                    };
                    let m2121 = {
                        let a2119 = v4;
                        let a2120 = 1i64;
                        compare(a2119, a2120)
                    };
                    if match m2121 {
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
                        Err::<(u64, (i64, List<u64>)), R<List<u64>>>(Ok(List::cons(f598(v5), v2)))
                    } else {
                        Ok::<(u64, (i64, List<u64>)), R<List<u64>>>((v3, (v4, List::cons(f598(v5), v2))))
                    }
                }
            };
            match step {
                Ok(next) => { m2122 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f544(v0: u64) -> R<u64> {
    let a2126 = v0;
    let a2127 = 10u64;
    if nat_lt(a2126, a2127) {
        let a2128 = 48u64;
        let a2129 = v0;
        nat_add(a2128, a2129)
    } else {
        let a2130 = 87u64;
        let a2131 = v0;
        nat_add(a2130, a2131)
    }
}

pub fn f545(v0: u64, v1: u64) -> u64 {
    let a2132 = v0;
    let a2133 = v1;
    let a2134 = 0u64;
    nat_quot(a2132, a2133, a2134)
}

pub fn f546(v0: u64, v1: u64) -> u64 {
    let a2135 = v0;
    let a2136 = v1;
    let a2137 = 0u64;
    nat_rem(a2135, a2136, a2137)
}

pub fn f547(v0: Adt146) -> Adt146 {
    let m2138 = v0;
    match m2138 {
        Adt146::C0 => {
            Adt146::C2
        }
        Adt146::C1 => {
            Adt146::C1
        }
        Adt146::C2 => {
            Adt146::C0
        }
    }
}

pub fn f548(v0: i64, v1: i64) -> R<i64> {
    f216(f167(f550(v0)?, 0i64)?, v1)
}

pub fn f549(v0: i64, v1: u64, v2: i64, v3: u64) -> R<Adt146> {
    let a2139 = v1;
    let a2140 = v3;
    if nat_lt(a2139, a2140) {
        Ok(f485(f599({
            let a2141 = v3;
            let a2142 = v1;
            nat_sub(a2141, a2142)
        }, v0)?, v2))
    } else {
        let a2143 = v3;
        let a2144 = v1;
        if nat_lt(a2143, a2144) {
            Ok(f485(v0, f599({
                let a2145 = v1;
                let a2146 = v3;
                nat_sub(a2145, a2146)
            }, v2)?))
        } else {
            Ok(f485(v0, v2))
        }
    }
}

pub fn f550(v0: i64) -> R<u64> {
    f600(20u64, v0, 0u64)
}

pub fn f551(v0: Adt35) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("supportedVersions"), f601(match v0.clone() {
        Adt35::C0(_, h2147, _, _) => {
            h2147
        }
    })), List::cons((Str::lit("capabilities"), f602(match v0.clone() {
        Adt35::C0(_, _, h2148, _) => {
            h2148
        }
    })), {
        let a2151 = f603(Str::lit("instructions"), match v0.clone() {
            Adt35::C0(_, _, _, h2149) => {
                h2149
            }
        });
        let a2152 = f604(match v0 {
            Adt35::C0(h2150, _, _, _) => {
                h2150
            }
        })?;
        append_list(a2151, a2152)
    })))
}

pub fn f552(v0: Adt36) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("tools"), Adt6::C4(f605(match v0.clone() {
        Adt36::C0(_, _, h2153) => {
            h2153
        }
    }))), {
        let a2156 = f603(Str::lit("nextCursor"), match v0.clone() {
            Adt36::C0(_, h2154, _) => {
                h2154
            }
        });
        let a2157 = f604(match v0 {
            Adt36::C0(h2155, _, _) => {
                h2155
            }
        })?;
        append_list(a2156, a2157)
    }))
}

pub fn f553(v0: Option<List<(Str, Adt6)>>) -> R<Option<List<(Str, Adt6)>>> {
    let m2158 = v0;
    match m2158 {
        None => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Some(v1) => {
            let m2159 = f567(Adt6::C5(v1))?;
            match m2159 {
                None => {
                    Ok(None::<List<(Str, Adt6)>>)
                }
                Some(v2) => {
                    let m2160 = f218(v2);
                    match m2160 {
                        None => {
                            Ok(None::<List<(Str, Adt6)>>)
                        }
                        Some(v3) => {
                            Ok(Some(match v3 {
                                Adt5::C0(h2161) => {
                                    h2161
                                }
                            }))
                        }
                    }
                }
            }
        }
    }
}

pub fn f554(v0: Adt39) -> R<Option<List<(Str, Adt6)>>> {
    let m2163 = f606(match v0.clone() {
        Adt39::C0(h2162, _, _) => {
            h2162
        }
    })?;
    match m2163 {
        None => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Some(v1) => {
            Ok(Some(List::cons((Str::lit("content"), Adt6::C4(v1)), {
                let a2166 = f607(Str::lit("structuredContent"), match v0.clone() {
                    Adt39::C0(_, h2164, _) => {
                        h2164
                    }
                });
                let a2167 = f608(Str::lit("isError"), match v0 {
                    Adt39::C0(_, _, h2165) => {
                        h2165
                    }
                });
                append_list(a2166, a2167)
            })))
        }
    }
}

pub fn f555(v0: Adt53) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("resources"), Adt6::C4(f609(match v0.clone() {
        Adt53::C0(_, _, h2168) => {
            h2168
        }
    })?)), {
        let a2171 = f603(Str::lit("nextCursor"), match v0.clone() {
            Adt53::C0(_, h2169, _) => {
                h2169
            }
        });
        let a2172 = f604(match v0 {
            Adt53::C0(h2170, _, _) => {
                h2170
            }
        })?;
        append_list(a2171, a2172)
    }))
}

pub fn f556(v0: Adt54) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("contents"), Adt6::C4(f610(match v0.clone() {
        Adt54::C0(_, h2173) => {
            h2173
        }
    }))), f604(match v0 {
        Adt54::C0(h2174, _) => {
            h2174
        }
    })?))
}

pub fn f557(v0: Adt55) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("resourceTemplates"), Adt6::C4(f611(match v0.clone() {
        Adt55::C0(_, _, h2175) => {
            h2175
        }
    }))), {
        let a2178 = f603(Str::lit("nextCursor"), match v0.clone() {
            Adt55::C0(_, h2176, _) => {
                h2176
            }
        });
        let a2179 = f604(match v0 {
            Adt55::C0(h2177, _, _) => {
                h2177
            }
        })?;
        append_list(a2178, a2179)
    }))
}

pub fn f558(v0: Adt57) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("prompts"), Adt6::C4(f612(match v0.clone() {
        Adt57::C0(_, _, h2180) => {
            h2180
        }
    }))), {
        let a2183 = f603(Str::lit("nextCursor"), match v0.clone() {
            Adt57::C0(_, h2181, _) => {
                h2181
            }
        });
        let a2184 = f604(match v0 {
            Adt57::C0(h2182, _, _) => {
                h2182
            }
        })?;
        append_list(a2183, a2184)
    }))
}

pub fn f559(v0: Adt60) -> R<Option<List<(Str, Adt6)>>> {
    let m2186 = f613(match v0.clone() {
        Adt60::C0(_, h2185) => {
            h2185
        }
    })?;
    match m2186 {
        None => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Some(v1) => {
            Ok(Some({
                let a2188 = f603(Str::lit("description"), match v0 {
                    Adt60::C0(h2187, _) => {
                        h2187
                    }
                });
                let a2189 = List::cons((Str::lit("messages"), Adt6::C4(v1)), List::<(Str, Adt6)>::nil());
                append_list(a2188, a2189)
            }))
        }
    }
}

pub fn f560(v0: Adt62) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("completion"), Adt6::C5(List::cons((Str::lit("values"), Adt6::C4(f577({
        let m2192 = match v0.clone() {
            Adt62::C0(h2191) => {
                h2191
            }
        };
        match m2192 {
            Adt63::C0(h2190, _, _) => {
                h2190
            }
        }
    }))), {
        let a2199 = f614(Str::lit("total"), {
            let m2195 = match v0.clone() {
                Adt62::C0(h2194) => {
                    h2194
                }
            };
            match m2195 {
                Adt63::C0(_, h2193, _) => {
                    h2193
                }
            }
        })?;
        let a2200 = f608(Str::lit("hasMore"), {
            let m2198 = match v0 {
                Adt62::C0(h2197) => {
                    h2197
                }
            };
            match m2198 {
                Adt63::C0(_, _, h2196) => {
                    h2196
                }
            }
        });
        append_list(a2199, a2200)
    }))), List::<(Str, Adt6)>::nil()))
}

pub fn f561(v0: Adt83) -> R<Option<List<(Str, Adt6)>>> {
    let m2201 = f615(f616(v0.clone()), List::<(Str, Adt6)>::nil())?;
    match m2201 {
        None => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Some(v1) => {
            Ok(Some({
                let a2202 = f617(v1);
                let a2203 = f618(f619(v0));
                append_list(a2202, a2203)
            }))
        }
    }
}

pub fn f562(v0: Adt153) -> Str {
    let m2204 = v0;
    match m2204 {
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

pub fn f563(v0: Adt34) -> Adt153 {
    let m2205 = v0;
    match m2205 {
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

pub fn f564(v0: Option<Adt1>) -> List<(Str, Adt6)> {
    let m2206 = v0;
    match m2206 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((f620(), f621(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f565(v0: bool, v1: u64, v2: List<List<u64>>) -> bool {
    if v0 {
        if f622(v2) {
            let a2207 = v1;
            let a2208 = 200u64;
            nat_eq(a2207, a2208)
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f566(v0: u64, v1: List<List<u64>>) -> Adt95 {
    Adt95::C0(v0, f623(), List::<List<u64>>::nil(), f624(v1))
}

pub fn f567(v0: Adt6) -> R<Option<Adt6>> {
    f625(f626(), Adt154::C0(v0), List::<Adt155>::nil())
}

pub fn f568(v0: Adt139) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("jsonrpc"), Adt6::C3(f270())), List::cons((Str::lit("method"), Adt6::C3(Str::lit("notifications/progress"))), List::cons((Str::lit("params"), Adt6::C5(List::cons((Str::lit("progressToken"), f627(match v0.clone() {
        Adt139::C0(h2209, _) => {
            h2209
        }
    })), List::cons((Str::lit("progress"), Adt6::C2({
        let m2212 = match v0.clone() {
            Adt139::C0(_, h2211) => {
                h2211
            }
        };
        match m2212 {
            Adt52::C0(h2210, _, _) => {
                h2210
            }
        }
    })), {
        let a2219 = f628({
            let m2215 = match v0.clone() {
                Adt139::C0(_, h2214) => {
                    h2214
                }
            };
            match m2215 {
                Adt52::C0(_, h2213, _) => {
                    h2213
                }
            }
        });
        let a2220 = f629({
            let m2218 = match v0 {
                Adt139::C0(_, h2217) => {
                    h2217
                }
            };
            match m2218 {
                Adt52::C0(_, _, h2216) => {
                    h2216
                }
            }
        });
        append_list(a2219, a2220)
    })))), List::<(Str, Adt6)>::nil()))))
}

pub fn f569() -> u64 {
    429u64
}

pub fn f570() -> i64 {
    -32602i64
}

pub fn f571() -> i64 {
    -32022i64
}

pub fn f572() -> i64 {
    -32020i64
}

pub fn f573() -> i64 {
    -32601i64
}

pub fn f574(v0: Adt86) -> i64 {
    let m2221 = v0;
    match m2221 {
        Adt86::C0(_) => {
            f570()
        }
        Adt86::C1(_) => {
            f570()
        }
        Adt86::C2(_) => {
            f570()
        }
        Adt86::C3(_) => {
            f570()
        }
        Adt86::C4(_) => {
            f570()
        }
        Adt86::C5(_) => {
            f570()
        }
        Adt86::C6(_) => {
            f630()
        }
        Adt86::C7(_) => {
            f631()
        }
        Adt86::C8(_) => {
            f632()
        }
        Adt86::C9(_) => {
            f630()
        }
        Adt86::C10 => {
            f570()
        }
        Adt86::C11 => {
            f570()
        }
        Adt86::C12(_) => {
            f630()
        }
        Adt86::C13 => {
            f630()
        }
        Adt86::C14 => {
            f630()
        }
        Adt86::C15(_) => {
            f630()
        }
    }
}

pub fn f575(v0: Adt141) -> Str {
    let m2222 = v0;
    match m2222 {
        Adt141::C0 => {
            Str::lit("Invalid params: params is required")
        }
        Adt141::C1 => {
            Str::lit("Invalid params: _meta is required")
        }
        Adt141::C2 => {
            Str::lit("Invalid params: _meta lacks io.modelcontextprotocol/protocolVersion")
        }
        Adt141::C3 => {
            Str::lit("Invalid params: _meta lacks io.modelcontextprotocol/clientCapabilities")
        }
        Adt141::C4 => {
            Str::lit("Invalid params: _meta is not an object")
        }
        Adt141::C5 => {
            Str::lit("Invalid params: io.modelcontextprotocol/protocolVersion is not a string")
        }
        Adt141::C6 => {
            Str::lit("Invalid params: io.modelcontextprotocol/clientCapabilities is not a valid capabilities object")
        }
        Adt141::C7(_) => {
            Str::lit("Invalid params: a required field is missing")
        }
        Adt141::C8(_) => {
            Str::lit("Invalid params: a field has the wrong type")
        }
    }
}

pub fn f576(v0: Adt128) -> Str {
    let m2223 = v0;
    match m2223 {
        Adt128::C0 => {
            Str::lit("Header mismatch: MCP-Protocol-Version header is missing")
        }
        Adt128::C1 => {
            Str::lit("Header mismatch: Mcp-Method header is missing")
        }
        Adt128::C2 => {
            Str::lit("Header mismatch: Mcp-Name header is missing")
        }
        Adt128::C3 => {
            Str::lit("Header mismatch: MCP-Protocol-Version differs from the request body")
        }
        Adt128::C4 => {
            Str::lit("Header mismatch: Mcp-Method differs from the request body")
        }
        Adt128::C5 => {
            Str::lit("Header mismatch: Mcp-Name differs from the request body")
        }
        Adt128::C6(v1) => {
            let m2224 = v1;
            match m2224 {
                Adt129::C0 => {
                    Str::lit("Header mismatch: MCP-Protocol-Version has a malformed value")
                }
                Adt129::C1 => {
                    Str::lit("Header mismatch: Mcp-Method has a malformed value")
                }
                Adt129::C2 => {
                    Str::lit("Header mismatch: Mcp-Name has a malformed value")
                }
            }
        }
        Adt128::C7(v2) => {
            let m2225 = v2;
            match m2225 {
                Adt129::C0 => {
                    Str::lit("Header mismatch: MCP-Protocol-Version is repeated")
                }
                Adt129::C1 => {
                    Str::lit("Header mismatch: Mcp-Method is repeated")
                }
                Adt129::C2 => {
                    Str::lit("Header mismatch: Mcp-Name is repeated")
                }
            }
        }
        Adt128::C8(_) => {
            Str::lit("Header mismatch: a required Mcp-Param header is missing")
        }
        Adt128::C9(_) => {
            Str::lit("Header mismatch: an Mcp-Param header differs from the request body")
        }
        Adt128::C10(_) => {
            Str::lit("Header mismatch: an Mcp-Param header has a malformed value")
        }
    }
}

pub fn f577(v0: List<Str>) -> List<Adt6> {
    f431(f633(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f578(v0: Adt19) -> Adt6 {
    Adt6::C5({
        let a2237 = f634(Str::lit("experimental"), match v0.clone() {
            Adt19::C0(h2226, _, _, _, _) => {
                h2226
            }
        });
        let a2238 = {
            let a2235 = f634(Str::lit("roots"), match v0.clone() {
                Adt19::C0(_, h2227, _, _, _) => {
                    h2227
                }
            });
            let a2236 = {
                let a2233 = f634(Str::lit("sampling"), match v0.clone() {
                    Adt19::C0(_, _, h2228, _, _) => {
                        h2228
                    }
                });
                let a2234 = {
                    let a2231 = f634(Str::lit("elicitation"), match v0.clone() {
                        Adt19::C0(_, _, _, h2229, _) => {
                            h2229
                        }
                    });
                    let a2232 = f634(Str::lit("extensions"), match v0 {
                        Adt19::C0(_, _, _, _, h2230) => {
                            h2230
                        }
                    });
                    append_list(a2231, a2232)
                };
                append_list(a2233, a2234)
            };
            append_list(a2235, a2236)
        };
        append_list(a2237, a2238)
    })
}

pub fn f579(v0: List<u64>) -> R<bool> {
    let m2239 = v0;
    match m2239.uncons() {
        None => {
            Ok(false)
        }
        Some((v1, v2)) => {
            let m2240 = v2;
            match m2240.uncons() {
                None => {
                    Ok(false)
                }
                Some((v3, v4)) => {
                    let a2241 = f322(v1)?;
                    let a2242 = 113u64;
                    if if nat_eq(a2241, a2242) {
                        let a2243 = v3;
                        let a2244 = 61u64;
                        nat_eq(a2243, a2244)
                    } else {
                        false
                    } {
                        Ok(f635(v4))
                    } else {
                        Ok(false)
                    }
                }
            }
        }
    }
}

pub fn f580(v0: Adt0, v1: Fn2, v2: u64, v3: Result<Adt16, Adt141>) -> R<Result<Adt137, Adt140>> {
    let m2245 = v3;
    match m2245 {
        Ok(v5) => {
            f636(v0, v1, v2, v5)
        }
        Err(v4) => {
            Ok(Err::<Adt137, Adt140>(Adt140::C0(v4)))
        }
    }
}

pub fn f581(v0: Adt148, v1: Adt18, v2: Adt5) -> Result<Adt16, Adt141> {
    let m2246 = v0;
    match m2246 {
        Adt148::C0 => {
            f637(v1, v2)
        }
        Adt148::C1 => {
            f638(v1, v2)
        }
        Adt148::C2 => {
            f639(v1, v2)
        }
        Adt148::C3 => {
            f640(v1, v2)
        }
        Adt148::C4 => {
            f641(v1, v2)
        }
        Adt148::C5 => {
            f642(v1, v2)
        }
        Adt148::C6 => {
            f643(v1, v2)
        }
        Adt148::C7 => {
            f644(v1, v2)
        }
        Adt148::C8 => {
            f645(v1, v2)
        }
        Adt148::C9 => {
            f646(v1, v2)
        }
        Adt148::C10 => {
            f647(v1, v2)
        }
        Adt148::C11 => {
            f648(v1, v2)
        }
    }
}

pub fn f582(v0: Adt148) -> Str {
    let m2247 = v0;
    match m2247 {
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

pub fn f583(v0: Option<Adt8>) -> bool {
    let m2248 = v0;
    match m2248 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f584(v0: Option<Adt9>) -> bool {
    let m2249 = v0;
    match m2249 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f585(v0: Option<Adt5>) -> bool {
    let m2250 = v0;
    match m2250 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f586(v0: Option<Adt5>, v1: Str) -> bool {
    let m2251 = v0;
    match m2251 {
        None => {
            false
        }
        Some(v2) => {
            f389(v1, v2)
        }
    }
}

pub fn f587() -> Str {
    Str::lit("io.modelcontextprotocol/tasks")
}

pub fn f588(v0: List<Str>, v1: Adt117, v2: Adt18, v3: Option<Adt128>) -> Adt150 {
    let m2252 = v3;
    match m2252 {
        None => {
            f649(v1, v2.clone(), f650(match v2 {
                Adt18::C0(h2253, _, _, _, _, _) => {
                    h2253
                }
            }, v0))
        }
        Some(v4) => {
            Adt150::C1(Adt140::C2(v4))
        }
    }
}

pub fn f589(v0: Option<Adt127>, v1: Adt152, v2: Str, v3: Str, v4: Adt5) -> R<Option<Adt128>> {
    let m2254 = v0;
    match m2254 {
        None => {
            Ok(None::<Adt128>)
        }
        Some(v5) => {
            let m2256 = match v5.clone() {
                Adt127::C0(_, _, _, h2255, _) => {
                    h2255
                }
            };
            match m2256 {
                None => {
                    f651(v5, v1, v2, v3, v4)
                }
                Some(v6) => {
                    Ok(Some(v6))
                }
            }
        }
    }
}

pub fn f590(v0: Option<Adt5>) -> Adt5 {
    let m2257 = v0;
    match m2257 {
        None => {
            f652()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f591(v0: Adt5) -> Result<Adt18, Adt141> {
    let m2258 = f653(v0.clone());
    match m2258 {
        Adt156::C0 => {
            f654(v0.clone(), f138(f655(), v0.clone()), f138(f656(), v0))
        }
        Adt156::C1 => {
            Err::<Adt18, Adt141>(Adt141::C2)
        }
        Adt156::C2 => {
            Err::<Adt18, Adt141>(Adt141::C3)
        }
    }
}

pub fn f592() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(112u64, List::cons(97u64, List::cons(114u64, List::cons(97u64, List::cons(109u64, List::cons(45u64, List::<u64>::nil()))))))))))
}

pub fn f593(p2262: List<Adt6>) -> bool {
    let m2261 = p2262;
    {
        let mut m2261 = m2261;
        loop {
            let step = {
                let v0 = m2261;
                let m2259 = v0;
                match m2259.uncons() {
                    None => {
                        Err::<List<Adt6>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let m2260 = f89(v1);
                        match m2260 {
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
                Ok(next) => { m2261 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f594(p2266: List<Adt6>) -> bool {
    let m2265 = p2266;
    {
        let mut m2265 = m2265;
        loop {
            let step = {
                let v0 = m2265;
                let m2263 = v0;
                match m2263.uncons() {
                    None => {
                        Err::<List<Adt6>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let m2264 = f89(v1);
                        match m2264 {
                            None => {
                                Err::<List<Adt6>, bool>(false)
                            }
                            Some(v3) => {
                                if f402(v3) {
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
                Ok(next) => { m2265 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f595(v0: Str) -> Option<Adt13> {
    let a2267 = v0.clone();
    let a2268 = Str::lit("string");
    if equal(a2267, a2268) {
        Some(Adt13::C0)
    } else {
        let a2269 = v0.clone();
        let a2270 = Str::lit("integer");
        if equal(a2269, a2270) {
            Some(Adt13::C1)
        } else {
            let a2271 = v0;
            let a2272 = Str::lit("boolean");
            if equal(a2271, a2272) {
                Some(Adt13::C2)
            } else {
                None::<Adt13>
            }
        }
    }
}

pub fn f596(p2281: List<u64>, p2282: List<u64>) -> R<List<u64>> {
    let m2280 = (p2281, p2282);
    {
        let mut m2280 = m2280;
        loop {
            let step = {
                let (v0, v1) = m2280;
                let m2273 = v0;
                match m2273.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), R<List<u64>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        let a2274 = 65u64;
                        let a2275 = v2;
                        if if nat_le(a2274, a2275) {
                            let a2276 = v2;
                            let a2277 = 90u64;
                            nat_le(a2276, a2277)
                        } else {
                            false
                        } {
                            Ok::<(List<u64>, List<u64>), R<List<u64>>>((v3, List::cons({
                                let a2278 = v2;
                                let a2279 = 32u64;
                                nat_add(a2278, a2279)?
                            }, v1)))
                        } else {
                            Ok::<(List<u64>, List<u64>), R<List<u64>>>((v3, List::cons(v2, v1)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2280 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f597(p2289: List<u64>, p2290: List<u64>) -> bool {
    let m2288 = (p2289, p2290);
    {
        let mut m2288 = m2288;
        loop {
            let step = {
                let (v0, v1) = m2288;
                let m2283 = v0;
                match m2283.uncons() {
                    None => {
                        let m2284 = v1;
                        match m2284.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m2285 = v1;
                        match m2285.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                            Some((v6, v7)) => {
                                let a2286 = v4;
                                let a2287 = v6;
                                if nat_eq(a2286, a2287) {
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
                Ok(next) => { m2288 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f598(v0: i64) -> u64 {
    let m2293 = {
        let a2291 = v0;
        let a2292 = 1i64;
        compare(a2291, a2292)
    };
    if match m2293 {
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
        let m2296 = {
            let a2294 = v0;
            let a2295 = 2i64;
            compare(a2294, a2295)
        };
        if match m2296 {
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
            let m2299 = {
                let a2297 = v0;
                let a2298 = 3i64;
                compare(a2297, a2298)
            };
            if match m2299 {
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
                let m2302 = {
                    let a2300 = v0;
                    let a2301 = 4i64;
                    compare(a2300, a2301)
                };
                if match m2302 {
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
                    let m2305 = {
                        let a2303 = v0;
                        let a2304 = 5i64;
                        compare(a2303, a2304)
                    };
                    if match m2305 {
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
                        let m2308 = {
                            let a2306 = v0;
                            let a2307 = 6i64;
                            compare(a2306, a2307)
                        };
                        if match m2308 {
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
                            let m2311 = {
                                let a2309 = v0;
                                let a2310 = 7i64;
                                compare(a2309, a2310)
                            };
                            if match m2311 {
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
                                let m2314 = {
                                    let a2312 = v0;
                                    let a2313 = 8i64;
                                    compare(a2312, a2313)
                                };
                                if match m2314 {
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
                                    let m2317 = {
                                        let a2315 = v0;
                                        let a2316 = 9i64;
                                        compare(a2315, a2316)
                                    };
                                    if match m2317 {
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

pub fn f599(p2322: u64, p2323: i64) -> R<i64> {
    let m2321 = (p2322, p2323);
    {
        let mut m2321 = m2321;
        loop {
            let step = {
                let (v0, v1) = m2321;
                let m2318 = v0;
                if m2318 == 0 {
                    Err::<(u64, i64), R<i64>>(Ok(v1))
                } else {
                    let v2 = m2318 - 1;
                    Ok::<(u64, i64), R<i64>>((v2, {
                        let a2319 = v1;
                        let a2320 = 10i64;
                        int_mul(a2319, a2320)?
                    }))
                }
            };
            match step {
                Ok(next) => { m2321 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f600(p2334: u64, p2335: i64, p2336: u64) -> R<u64> {
    let m2333 = (p2334, (p2335, p2336));
    {
        let mut m2333 = m2333;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2333;
                let m2324 = v0;
                if m2324 == 0 {
                    Err::<(u64, (i64, u64)), R<u64>>(Ok(v2))
                } else {
                    let v3 = m2324 - 1;
                    let v4: u64 = {
                        let a2325 = v2;
                        let a2326 = 1u64;
                        nat_add(a2325, a2326)?
                    };
                    let m2329 = {
                        let a2327 = v1;
                        let a2328 = 10i64;
                        compare(a2327, a2328)
                    };
                    if match m2329 {
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
                            let a2330 = v1;
                            let a2331 = 10i64;
                            let a2332 = 0i64;
                            int_quot(a2330, a2331, a2332)?
                        }, v4)))
                    }
                }
            };
            match step {
                Ok(next) => { m2333 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f601(v0: List<Str>) -> Adt6 {
    Adt6::C4(f577(v0))
}

pub fn f602(v0: Adt4) -> Adt6 {
    Adt6::C5({
        let a2354 = f634(Str::lit("experimental"), match v0.clone() {
            Adt4::C0(h2337, _, _, _, _, _, _) => {
                h2337
            }
        });
        let a2355 = {
            let a2352 = f634(Str::lit("logging"), match v0.clone() {
                Adt4::C0(_, h2338, _, _, _, _, _) => {
                    h2338
                }
            });
            let a2353 = {
                let a2350 = f634(Str::lit("completions"), match v0.clone() {
                    Adt4::C0(_, _, h2339, _, _, _, _) => {
                        h2339
                    }
                });
                let a2351 = {
                    let a2348 = f657(Str::lit("prompts"), match v0.clone() {
                        Adt4::C0(_, _, _, h2340, _, _, _) => {
                            h2340
                        }
                    });
                    let a2349 = {
                        let a2346 = f658(match v0.clone() {
                            Adt4::C0(_, _, _, _, h2341, _, _) => {
                                h2341
                            }
                        });
                        let a2347 = {
                            let a2344 = f657(Str::lit("tools"), match v0.clone() {
                                Adt4::C0(_, _, _, _, _, h2342, _) => {
                                    h2342
                                }
                            });
                            let a2345 = f634(Str::lit("extensions"), match v0 {
                                Adt4::C0(_, _, _, _, _, _, h2343) => {
                                    h2343
                                }
                            });
                            append_list(a2344, a2345)
                        };
                        append_list(a2346, a2347)
                    };
                    append_list(a2348, a2349)
                };
                append_list(a2350, a2351)
            };
            append_list(a2352, a2353)
        };
        append_list(a2354, a2355)
    })
}

pub fn f603(v0: Str, v1: Option<Str>) -> List<(Str, Adt6)> {
    let m2356 = v1;
    match m2356 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C3(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f604(v0: Adt10) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("ttlMs"), f53(match v0.clone() {
        Adt10::C0(h2357, _) => {
            h2357
        }
    })?), List::cons((Str::lit("cacheScope"), Adt6::C3(f659(match v0 {
        Adt10::C0(_, h2358) => {
            h2358
        }
    }))), List::<(Str, Adt6)>::nil())))
}

pub fn f605(v0: List<Adt37>) -> List<Adt6> {
    f431(f660(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f606(v0: List<Adt40>) -> R<Option<List<Adt6>>> {
    let m2359 = f661(v0, List::<Adt6>::nil())?;
    match m2359 {
        None => {
            Ok(None::<List<Adt6>>)
        }
        Some(v1) => {
            Ok(Some(f431(v1, List::<Adt6>::nil())))
        }
    }
}

pub fn f607(v0: Str, v1: Option<Adt6>) -> List<(Str, Adt6)> {
    let m2360 = v1;
    match m2360 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, v2), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f608(v0: Str, v1: Option<bool>) -> List<(Str, Adt6)> {
    let m2361 = v1;
    match m2361 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C1(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f609(v0: List<Adt46>) -> R<List<Adt6>> {
    Ok(f431(f662(v0, List::<Adt6>::nil())?, List::<Adt6>::nil()))
}

pub fn f610(v0: List<Adt48>) -> List<Adt6> {
    f431(f663(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f611(v0: List<Adt56>) -> List<Adt6> {
    f431(f664(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f612(v0: List<Adt58>) -> List<Adt6> {
    f431(f665(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f613(v0: List<Adt61>) -> R<Option<List<Adt6>>> {
    let m2362 = f666(v0, List::<Adt6>::nil())?;
    match m2362 {
        None => {
            Ok(None::<List<Adt6>>)
        }
        Some(v1) => {
            Ok(Some(f431(v1, List::<Adt6>::nil())))
        }
    }
}

pub fn f614(v0: Str, v1: Option<u64>) -> R<List<(Str, Adt6)>> {
    let m2363 = v1;
    match m2363 {
        None => {
            Ok(List::<(Str, Adt6)>::nil())
        }
        Some(v2) => {
            Ok(List::cons((v0, f53(v2)?), List::<(Str, Adt6)>::nil()))
        }
    }
}

pub fn f615(p2369: List<(Str, Adt66)>, p2370: List<(Str, Adt6)>) -> R<Option<List<(Str, Adt6)>>> {
    let m2368 = (p2369, p2370);
    {
        let mut m2368 = m2368;
        loop {
            let step = {
                let (v0, v1) = m2368;
                let m2364 = v0;
                match m2364.uncons() {
                    None => {
                        Err::<(List<(Str, Adt66)>, List<(Str, Adt6)>), R<Option<List<(Str, Adt6)>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m2366 = f667({
                            let (_, h2365) = v2;
                            h2365
                        })?;
                        match m2366 {
                            None => {
                                Err::<(List<(Str, Adt66)>, List<(Str, Adt6)>), R<Option<List<(Str, Adt6)>>>>(Ok(None::<List<(Str, Adt6)>>))
                            }
                            Some(v4) => {
                                Ok::<(List<(Str, Adt66)>, List<(Str, Adt6)>), R<Option<List<(Str, Adt6)>>>>((v3, List::cons(({
                                    let (h2367, _) = v2;
                                    h2367
                                }, v4), v1)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2368 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f616(v0: Adt83) -> List<(Str, Adt66)> {
    let m2371 = v0;
    match m2371 {
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

pub fn f617(v0: List<(Str, Adt6)>) -> List<(Str, Adt6)> {
    let m2372 = v0.clone();
    match m2372.uncons() {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some((_, _)) => {
            List::cons((Str::lit("inputRequests"), Adt6::C5(f433(v0, List::<(Str, Adt6)>::nil()))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f618(v0: Option<Str>) -> List<(Str, Adt6)> {
    let m2373 = v0;
    match m2373 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("requestState"), Adt6::C3(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f619(v0: Adt83) -> Option<Str> {
    let m2374 = v0;
    match m2374 {
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

pub fn f620() -> Str {
    Str::lit("io.modelcontextprotocol/serverInfo")
}

pub fn f621(v0: Adt1) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
        Adt1::C0(h2375, _, _, _, _, _) => {
            h2375
        }
    })), List::cons((Str::lit("version"), Adt6::C3(match v0.clone() {
        Adt1::C0(_, _, h2376, _, _, _) => {
            h2376
        }
    })), {
        let a2385 = f603(Str::lit("title"), match v0.clone() {
            Adt1::C0(_, h2377, _, _, _, _) => {
                h2377
            }
        });
        let a2386 = {
            let a2383 = f603(Str::lit("description"), match v0.clone() {
                Adt1::C0(_, _, _, h2378, _, _) => {
                    h2378
                }
            });
            let a2384 = {
                let a2381 = f603(Str::lit("websiteUrl"), match v0.clone() {
                    Adt1::C0(_, _, _, _, h2379, _) => {
                        h2379
                    }
                });
                let a2382 = f668(match v0 {
                    Adt1::C0(_, _, _, _, _, h2380) => {
                        h2380
                    }
                });
                append_list(a2381, a2382)
            };
            append_list(a2383, a2384)
        };
        append_list(a2385, a2386)
    })))
}

pub fn f622(v0: List<List<u64>>) -> bool {
    let m2387 = v0;
    match m2387.uncons() {
        None => {
            false
        }
        Some((_, _)) => {
            true
        }
    }
}

pub fn f623() -> List<Adt92> {
    List::cons(Adt92::C0(f669(), f512()), List::cons(Adt92::C0(f670(), f671()), List::cons(Adt92::C0(f672(), f673()), List::<Adt92>::nil())))
}

pub fn f624(v0: List<List<u64>>) -> List<u64> {
    f674(f118(v0, List::<List<u64>>::nil()), List::<u64>::nil())
}

pub fn f625(p2402: u64, p2403: Adt154, p2404: List<Adt155>) -> R<Option<Adt6>> {
    let m2401 = (p2402, (p2403, p2404));
    {
        let mut m2401 = m2401;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2401;
                let m2388 = v0;
                if m2388 == 0 {
                    Err::<(u64, (Adt154, List<Adt155>)), R<Option<Adt6>>>(Ok(None::<Adt6>))
                } else {
                    let v3 = m2388 - 1;
                    let m2389 = v1;
                    match m2389 {
                        Adt154::C0(v4) => {
                            let m2390 = v4.clone();
                            match m2390 {
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
                                    let v8: Str = f675(v7)?;
                                    Ok::<(u64, (Adt154, List<Adt155>)), R<Option<Adt6>>>((v3, (Adt154::C1(Adt6::C3(v8)), v2)))
                                }
                                Adt6::C4(v9) => {
                                    let m2391 = v9;
                                    match m2391.uncons() {
                                        None => {
                                            Ok::<(u64, (Adt154, List<Adt155>)), R<Option<Adt6>>>((v3, (Adt154::C1(v4), v2)))
                                        }
                                        Some((v10, v11)) => {
                                            Ok::<(u64, (Adt154, List<Adt155>)), R<Option<Adt6>>>((v3, (Adt154::C0(v10), List::cons(Adt155::C0(List::<Adt6>::nil(), v11), v2))))
                                        }
                                    }
                                }
                                Adt6::C5(v12) => {
                                    let m2392 = v12;
                                    match m2392.uncons() {
                                        None => {
                                            Ok::<(u64, (Adt154, List<Adt155>)), R<Option<Adt6>>>((v3, (Adt154::C1(v4), v2)))
                                        }
                                        Some((v13, v14)) => {
                                            let v15: Str = f675({
                                                let (h2393, _) = v13;
                                                h2393
                                            })?;
                                            Ok::<(u64, (Adt154, List<Adt155>)), R<Option<Adt6>>>((v3, (Adt154::C0({
                                                let (_, h2394) = v13;
                                                h2394
                                            }), List::cons(Adt155::C1(v15, List::<(Str, Adt6)>::nil(), v14), v2))))
                                        }
                                    }
                                }
                            }
                        }
                        Adt154::C1(v16) => {
                            let m2395 = v2;
                            match m2395.uncons() {
                                None => {
                                    Err::<(u64, (Adt154, List<Adt155>)), R<Option<Adt6>>>(Ok(Some(v16)))
                                }
                                Some((v17, v18)) => {
                                    let m2396 = v17;
                                    match m2396 {
                                        Adt155::C0(v19, v20) => {
                                            let v21: List<Adt6> = List::cons(v16, v19);
                                            let m2397 = v20;
                                            match m2397.uncons() {
                                                None => {
                                                    Ok::<(u64, (Adt154, List<Adt155>)), R<Option<Adt6>>>((v3, (Adt154::C1(Adt6::C4(f431(v21, List::<Adt6>::nil()))), v18)))
                                                }
                                                Some((v22, v23)) => {
                                                    Ok::<(u64, (Adt154, List<Adt155>)), R<Option<Adt6>>>((v3, (Adt154::C0(v22), List::cons(Adt155::C0(v21, v23), v18))))
                                                }
                                            }
                                        }
                                        Adt155::C1(v24, v25, v26) => {
                                            let v27: List<(Str, Adt6)> = List::cons((v24, v16), v25);
                                            let m2398 = v26;
                                            match m2398.uncons() {
                                                None => {
                                                    Ok::<(u64, (Adt154, List<Adt155>)), R<Option<Adt6>>>((v3, (Adt154::C1(Adt6::C5(f433(v27, List::<(Str, Adt6)>::nil()))), v18)))
                                                }
                                                Some((v28, v29)) => {
                                                    let v30: Str = f675({
                                                        let (h2399, _) = v28;
                                                        h2399
                                                    })?;
                                                    Ok::<(u64, (Adt154, List<Adt155>)), R<Option<Adt6>>>((v3, (Adt154::C0({
                                                        let (_, h2400) = v28;
                                                        h2400
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
                Ok(next) => { m2401 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f626() -> u64 {
    67108864u64
}

pub fn f627(v0: Adt21) -> Adt6 {
    let m2405 = v0;
    match m2405 {
        Adt21::C0(v1) => {
            Adt6::C3(v1)
        }
        Adt21::C1(v2) => {
            Adt6::C2(Adt7::C0(v2, 0i64))
        }
    }
}

pub fn f628(v0: Option<Adt7>) -> List<(Str, Adt6)> {
    let m2406 = v0;
    match m2406 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("total"), Adt6::C2(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f629(v0: Option<Str>) -> List<(Str, Adt6)> {
    let m2407 = v0;
    match m2407 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("message"), Adt6::C3(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f630() -> i64 {
    -32603i64
}

pub fn f631() -> i64 {
    429i64
}

pub fn f632() -> i64 {
    -32021i64
}

pub fn f633(p2410: List<Str>, p2411: List<Adt6>) -> List<Adt6> {
    let m2409 = (p2410, p2411);
    {
        let mut m2409 = m2409;
        loop {
            let step = {
                let (v0, v1) = m2409;
                let m2408 = v0;
                match m2408.uncons() {
                    None => {
                        Err::<(List<Str>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Str>, List<Adt6>), List<Adt6>>((v3, List::cons(Adt6::C3(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2409 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f634(v0: Str, v1: Option<Adt5>) -> List<(Str, Adt6)> {
    let m2412 = v1;
    match m2412 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C5(match v2 {
                Adt5::C0(h2413) => {
                    h2413
                }
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f635(v0: List<u64>) -> bool {
    let m2414 = v0;
    match m2414.uncons() {
        None => {
            false
        }
        Some((v1, v2)) => {
            let a2415 = v1;
            let a2416 = 48u64;
            if nat_eq(a2415, a2416) {
                f676(v2)
            } else {
                false
            }
        }
    }
}

pub fn f636(v0: Adt0, v1: Fn2, v2: u64, v3: Adt16) -> R<Result<Adt137, Adt140>> {
    if f677(f678(v3.clone())) {
        Ok(Ok::<Adt137, Adt140>(f679(v0.clone(), Adt34::C0(f680(v0)))))
    } else {
        Ok(f681(v0.clone(), v3.clone(), f682(match v0.clone() {
            Adt0::C0(_, _, _, _, _, _, h2417) => {
                h2417
            }
        }, match v0 {
            Adt0::C0(_, _, _, _, _, h2418, _) => {
                h2418
            }
        }, v2, v1, v3)?))
    }
}

pub fn f637(v0: Adt18, _: Adt5) -> Result<Adt16, Adt141> {
    Ok::<Adt16, Adt141>(Adt16::C0(Adt17::C0(v0)))
}

pub fn f638(v0: Adt18, v1: Adt5) -> Result<Adt16, Adt141> {
    let m2419 = f683(Str::lit("cursor"), v1);
    match m2419 {
        Ok(v2) => {
            Ok::<Adt16, Adt141>(Adt16::C1(Adt22::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt16, Adt141>(v3)
        }
    }
}

pub fn f639(v0: Adt18, v1: Adt5) -> Result<Adt16, Adt141> {
    let m2420 = f684(Str::lit("name"), v1.clone());
    match m2420 {
        Ok(v2) => {
            let m2421 = f685(Str::lit("arguments"), v1.clone());
            match m2421 {
                Ok(v3) => {
                    let m2422 = f685(Str::lit("inputResponses"), v1.clone());
                    match m2422 {
                        Ok(v4) => {
                            let m2423 = f683(Str::lit("requestState"), v1);
                            match m2423 {
                                Ok(v5) => {
                                    Ok::<Adt16, Adt141>(Adt16::C2(Adt23::C0(v0, v2, v3, v4, v5)))
                                }
                                Err(v6) => {
                                    Err::<Adt16, Adt141>(v6)
                                }
                            }
                        }
                        Err(v7) => {
                            Err::<Adt16, Adt141>(v7)
                        }
                    }
                }
                Err(v8) => {
                    Err::<Adt16, Adt141>(v8)
                }
            }
        }
        Err(v9) => {
            Err::<Adt16, Adt141>(v9)
        }
    }
}

pub fn f640(v0: Adt18, v1: Adt5) -> Result<Adt16, Adt141> {
    let m2424 = f683(Str::lit("cursor"), v1);
    match m2424 {
        Ok(v2) => {
            Ok::<Adt16, Adt141>(Adt16::C3(Adt22::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt16, Adt141>(v3)
        }
    }
}

pub fn f641(v0: Adt18, v1: Adt5) -> Result<Adt16, Adt141> {
    let m2425 = f684(Str::lit("uri"), v1.clone());
    match m2425 {
        Ok(v2) => {
            let m2426 = f685(Str::lit("inputResponses"), v1.clone());
            match m2426 {
                Ok(v3) => {
                    let m2427 = f683(Str::lit("requestState"), v1);
                    match m2427 {
                        Ok(v4) => {
                            Ok::<Adt16, Adt141>(Adt16::C4(Adt24::C0(v0, v2, v3, v4)))
                        }
                        Err(v5) => {
                            Err::<Adt16, Adt141>(v5)
                        }
                    }
                }
                Err(v6) => {
                    Err::<Adt16, Adt141>(v6)
                }
            }
        }
        Err(v7) => {
            Err::<Adt16, Adt141>(v7)
        }
    }
}

pub fn f642(v0: Adt18, v1: Adt5) -> Result<Adt16, Adt141> {
    let m2428 = f683(Str::lit("cursor"), v1);
    match m2428 {
        Ok(v2) => {
            Ok::<Adt16, Adt141>(Adt16::C5(Adt22::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt16, Adt141>(v3)
        }
    }
}

pub fn f643(v0: Adt18, v1: Adt5) -> Result<Adt16, Adt141> {
    let m2429 = f683(Str::lit("cursor"), v1);
    match m2429 {
        Ok(v2) => {
            Ok::<Adt16, Adt141>(Adt16::C6(Adt22::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt16, Adt141>(v3)
        }
    }
}

pub fn f644(v0: Adt18, v1: Adt5) -> Result<Adt16, Adt141> {
    let m2430 = f684(Str::lit("name"), v1.clone());
    match m2430 {
        Ok(v2) => {
            let m2431 = f686(Str::lit("arguments"), v1.clone());
            match m2431 {
                Ok(v3) => {
                    let m2432 = f685(Str::lit("inputResponses"), v1.clone());
                    match m2432 {
                        Ok(v4) => {
                            let m2433 = f683(Str::lit("requestState"), v1);
                            match m2433 {
                                Ok(v5) => {
                                    Ok::<Adt16, Adt141>(Adt16::C7(Adt25::C0(v0, v2, v3, v4, v5)))
                                }
                                Err(v6) => {
                                    Err::<Adt16, Adt141>(v6)
                                }
                            }
                        }
                        Err(v7) => {
                            Err::<Adt16, Adt141>(v7)
                        }
                    }
                }
                Err(v8) => {
                    Err::<Adt16, Adt141>(v8)
                }
            }
        }
        Err(v9) => {
            Err::<Adt16, Adt141>(v9)
        }
    }
}

pub fn f645(v0: Adt18, v1: Adt5) -> Result<Adt16, Adt141> {
    let m2434 = f687(Str::lit("ref"), v1.clone());
    match m2434 {
        Ok(v2) => {
            let m2435 = f688(v2);
            match m2435 {
                Ok(v3) => {
                    let m2436 = f687(Str::lit("argument"), v1.clone());
                    match m2436 {
                        Ok(v4) => {
                            let m2437 = f689(v4);
                            match m2437 {
                                Ok(v5) => {
                                    let m2438 = f690(v1);
                                    match m2438 {
                                        Ok(v6) => {
                                            Ok::<Adt16, Adt141>(Adt16::C8(Adt26::C0(v0, v3, v5, v6)))
                                        }
                                        Err(v7) => {
                                            Err::<Adt16, Adt141>(v7)
                                        }
                                    }
                                }
                                Err(v8) => {
                                    Err::<Adt16, Adt141>(v8)
                                }
                            }
                        }
                        Err(v9) => {
                            Err::<Adt16, Adt141>(v9)
                        }
                    }
                }
                Err(v10) => {
                    Err::<Adt16, Adt141>(v10)
                }
            }
        }
        Err(v11) => {
            Err::<Adt16, Adt141>(v11)
        }
    }
}

pub fn f646(v0: Adt18, v1: Adt5) -> Result<Adt16, Adt141> {
    let m2439 = f684(Str::lit("taskId"), v1);
    match m2439 {
        Ok(v2) => {
            Ok::<Adt16, Adt141>(Adt16::C9(Adt31::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt16, Adt141>(v3)
        }
    }
}

pub fn f647(v0: Adt18, v1: Adt5) -> Result<Adt16, Adt141> {
    let m2440 = f684(Str::lit("taskId"), v1.clone());
    match m2440 {
        Ok(v2) => {
            let m2441 = f687(Str::lit("inputResponses"), v1);
            match m2441 {
                Ok(v3) => {
                    Ok::<Adt16, Adt141>(Adt16::C10(Adt32::C0(v0, v2, v3)))
                }
                Err(v4) => {
                    Err::<Adt16, Adt141>(v4)
                }
            }
        }
        Err(v5) => {
            Err::<Adt16, Adt141>(v5)
        }
    }
}

pub fn f648(v0: Adt18, v1: Adt5) -> Result<Adt16, Adt141> {
    let m2442 = f684(Str::lit("taskId"), v1);
    match m2442 {
        Ok(v2) => {
            Ok::<Adt16, Adt141>(Adt16::C11(Adt33::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt16, Adt141>(v3)
        }
    }
}

pub fn f649(v0: Adt117, v1: Adt18, v2: Option<Adt140>) -> Adt150 {
    let m2443 = v2;
    match m2443 {
        None => {
            Adt150::C0(Adt151::C0(match v0.clone() {
                Adt117::C0(h2444, _, _) => {
                    h2444
                }
            }, match v0.clone() {
                Adt117::C0(_, h2445, _) => {
                    h2445
                }
            }, v1, f590(match v0 {
                Adt117::C0(_, _, h2446) => {
                    h2446
                }
            })))
        }
        Some(v3) => {
            Adt150::C1(v3)
        }
    }
}

pub fn f650(v0: Str, v1: List<Str>) -> Option<Adt140> {
    f691(f692(v0.clone(), v1.clone()), v0, v1)
}

pub fn f651(v0: Adt127, v1: Adt152, v2: Str, v3: Str, v4: Adt5) -> R<Option<Adt128>> {
    let m2448 = match v0.clone() {
        Adt127::C0(h2447, _, _, _, _) => {
            h2447
        }
    };
    match m2448 {
        None => {
            Ok(Some(Adt128::C0))
        }
        Some(v5) => {
            let a2449 = v5;
            let a2450 = v3;
            if equal(a2449, a2450) {
                let m2452 = match v0.clone() {
                    Adt127::C0(_, h2451, _, _, _) => {
                        h2451
                    }
                };
                match m2452 {
                    None => {
                        Ok(Some(Adt128::C1))
                    }
                    Some(v6) => {
                        let a2453 = v6;
                        let a2454 = v2;
                        if equal(a2453, a2454) {
                            f693(f694(match v0.clone() {
                                Adt127::C0(_, _, h2455, _, _) => {
                                    h2455
                                }
                            }, v1, v4.clone()), match v0 {
                                Adt127::C0(_, _, _, _, h2456) => {
                                    h2456
                                }
                            }, v4)
                        } else {
                            Ok(Some(Adt128::C4))
                        }
                    }
                }
            } else {
                Ok(Some(Adt128::C3))
            }
        }
    }
}

pub fn f652() -> Adt5 {
    Adt5::C0(List::<(Str, Adt6)>::nil())
}

pub fn f653(v0: Adt5) -> Adt156 {
    f695(f138(f655(), v0.clone()), f138(f656(), v0))
}

pub fn f654(v0: Adt5, v1: Adt108, v2: Adt108) -> Result<Adt18, Adt141> {
    let m2457 = v1;
    match m2457 {
        Adt108::C0 => {
            Err::<Adt18, Adt141>(Adt141::C2)
        }
        Adt108::C1(v3) => {
            let m2458 = v2;
            match m2458 {
                Adt108::C0 => {
                    Err::<Adt18, Adt141>(Adt141::C3)
                }
                Adt108::C1(v4) => {
                    f696(v0, v3, v4)
                }
            }
        }
    }
}

pub fn f655() -> Str {
    Str::lit("io.modelcontextprotocol/protocolVersion")
}

pub fn f656() -> Str {
    Str::lit("io.modelcontextprotocol/clientCapabilities")
}

pub fn f657(v0: Str, v1: Option<Adt8>) -> List<(Str, Adt6)> {
    let m2459 = v1;
    match m2459 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, f697(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f658(v0: Option<Adt9>) -> List<(Str, Adt6)> {
    let m2460 = v0;
    match m2460 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("resources"), Adt6::C5({
                let a2463 = f608(Str::lit("subscribe"), match v1.clone() {
                    Adt9::C0(h2461, _) => {
                        h2461
                    }
                });
                let a2464 = f608(Str::lit("listChanged"), match v1 {
                    Adt9::C0(_, h2462) => {
                        h2462
                    }
                });
                append_list(a2463, a2464)
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f659(v0: Adt11) -> Str {
    let m2465 = v0;
    match m2465 {
        Adt11::C0 => {
            Str::lit("public")
        }
        Adt11::C1 => {
            Str::lit("private")
        }
    }
}

pub fn f660(p2468: List<Adt37>, p2469: List<Adt6>) -> List<Adt6> {
    let m2467 = (p2468, p2469);
    {
        let mut m2467 = m2467;
        loop {
            let step = {
                let (v0, v1) = m2467;
                let m2466 = v0;
                match m2466.uncons() {
                    None => {
                        Err::<(List<Adt37>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt37>, List<Adt6>), List<Adt6>>((v3, List::cons(f698(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2467 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f661(p2473: List<Adt40>, p2474: List<Adt6>) -> R<Option<List<Adt6>>> {
    let m2472 = (p2473, p2474);
    {
        let mut m2472 = m2472;
        loop {
            let step = {
                let (v0, v1) = m2472;
                let m2470 = v0;
                match m2470.uncons() {
                    None => {
                        Err::<(List<Adt40>, List<Adt6>), R<Option<List<Adt6>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m2471 = f699(v2)?;
                        match m2471 {
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
                Ok(next) => { m2472 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f662(p2477: List<Adt46>, p2478: List<Adt6>) -> R<List<Adt6>> {
    let m2476 = (p2477, p2478);
    {
        let mut m2476 = m2476;
        loop {
            let step = {
                let (v0, v1) = m2476;
                let m2475 = v0;
                match m2475.uncons() {
                    None => {
                        Err::<(List<Adt46>, List<Adt6>), R<List<Adt6>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt46>, List<Adt6>), R<List<Adt6>>>((v3, List::cons(f700(v2)?, v1)))
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

pub fn f663(p2481: List<Adt48>, p2482: List<Adt6>) -> List<Adt6> {
    let m2480 = (p2481, p2482);
    {
        let mut m2480 = m2480;
        loop {
            let step = {
                let (v0, v1) = m2480;
                let m2479 = v0;
                match m2479.uncons() {
                    None => {
                        Err::<(List<Adt48>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt48>, List<Adt6>), List<Adt6>>((v3, List::cons(f701(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2480 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f664(p2485: List<Adt56>, p2486: List<Adt6>) -> List<Adt6> {
    let m2484 = (p2485, p2486);
    {
        let mut m2484 = m2484;
        loop {
            let step = {
                let (v0, v1) = m2484;
                let m2483 = v0;
                match m2483.uncons() {
                    None => {
                        Err::<(List<Adt56>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt56>, List<Adt6>), List<Adt6>>((v3, List::cons(f702(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2484 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f665(p2489: List<Adt58>, p2490: List<Adt6>) -> List<Adt6> {
    let m2488 = (p2489, p2490);
    {
        let mut m2488 = m2488;
        loop {
            let step = {
                let (v0, v1) = m2488;
                let m2487 = v0;
                match m2487.uncons() {
                    None => {
                        Err::<(List<Adt58>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt58>, List<Adt6>), List<Adt6>>((v3, List::cons(f703(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2488 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f666(p2494: List<Adt61>, p2495: List<Adt6>) -> R<Option<List<Adt6>>> {
    let m2493 = (p2494, p2495);
    {
        let mut m2493 = m2493;
        loop {
            let step = {
                let (v0, v1) = m2493;
                let m2491 = v0;
                match m2491.uncons() {
                    None => {
                        Err::<(List<Adt61>, List<Adt6>), R<Option<List<Adt6>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m2492 = f704(v2)?;
                        match m2492 {
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
                Ok(next) => { m2493 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f667(v0: Adt66) -> R<Option<Adt6>> {
    let m2496 = v0;
    match m2496 {
        Adt66::C0(v1) => {
            Ok(Some(f705(Str::lit("elicitation/create"), Some(f706(v1)))))
        }
        Adt66::C1(v2) => {
            let m2497 = f707(v2)?;
            match m2497 {
                None => {
                    Ok(None::<Adt6>)
                }
                Some(v3) => {
                    Ok(Some(f705(Str::lit("sampling/createMessage"), Some(v3))))
                }
            }
        }
        Adt66::C2(v4) => {
            Ok(Some(f705(Str::lit("roots/list"), {
                let m2498 = v4;
                match m2498 {
                    None => {
                        None::<Adt6>
                    }
                    Some(v5) => {
                        Some(Adt6::C5(match v5 {
                            Adt5::C0(h2499) => {
                                h2499
                            }
                        }))
                    }
                }
            })))
        }
    }
}

pub fn f668(v0: Option<List<Adt2>>) -> List<(Str, Adt6)> {
    let m2500 = v0;
    match m2500 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("icons"), Adt6::C4(f708(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f669() -> List<u64> {
    List::cons(67u64, List::cons(111u64, List::cons(110u64, List::cons(116u64, List::cons(101u64, List::cons(110u64, List::cons(116u64, List::cons(45u64, List::cons(84u64, List::cons(121u64, List::cons(112u64, List::cons(101u64, List::<u64>::nil()))))))))))))
}

pub fn f670() -> List<u64> {
    List::cons(67u64, List::cons(97u64, List::cons(99u64, List::cons(104u64, List::cons(101u64, List::cons(45u64, List::cons(67u64, List::cons(111u64, List::cons(110u64, List::cons(116u64, List::cons(114u64, List::cons(111u64, List::cons(108u64, List::<u64>::nil())))))))))))))
}

pub fn f671() -> List<u64> {
    List::cons(110u64, List::cons(111u64, List::cons(45u64, List::cons(99u64, List::cons(97u64, List::cons(99u64, List::cons(104u64, List::cons(101u64, List::<u64>::nil()))))))))
}

pub fn f672() -> List<u64> {
    List::cons(88u64, List::cons(45u64, List::cons(65u64, List::cons(99u64, List::cons(99u64, List::cons(101u64, List::cons(108u64, List::cons(45u64, List::cons(66u64, List::cons(117u64, List::cons(102u64, List::cons(102u64, List::cons(101u64, List::cons(114u64, List::cons(105u64, List::cons(110u64, List::cons(103u64, List::<u64>::nil())))))))))))))))))
}

pub fn f673() -> List<u64> {
    List::cons(110u64, List::cons(111u64, List::<u64>::nil()))
}

pub fn f674(p2505: List<List<u64>>, p2506: List<u64>) -> List<u64> {
    let m2504 = (p2505, p2506);
    {
        let mut m2504 = m2504;
        loop {
            let step = {
                let (v0, v1) = m2504;
                let m2501 = v0;
                match m2501.uncons() {
                    None => {
                        Err::<(List<List<u64>>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<u64>>, List<u64>), List<u64>>((v3, {
                            let a2502 = f709(v2);
                            let a2503 = v1;
                            append_list(a2502, a2503)
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m2504 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f675(v0: Str) -> R<Str> {
    let m2509 = {
        let a2508 = f104(f105(), f263(f710(f18({
            let a2507 = v0;
            utf8_encode(a2507)
        })?, List::<u64>::nil()), List::<u64>::nil()));
        utf8_decode(a2508)
    };
    match m2509 {
        None => {
            Ok(Str::lit("?"))
        }
        Some(v1) => {
            Ok(v1)
        }
    }
}

pub fn f676(p2516: List<u64>) -> bool {
    let m2515 = p2516;
    {
        let mut m2515 = m2515;
        loop {
            let step = {
                let v0 = m2515;
                let m2510 = v0;
                match m2510.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let a2511 = v1;
                        let a2512 = 48u64;
                        if if nat_eq(a2511, a2512) {
                            true
                        } else {
                            let a2513 = v1;
                            let a2514 = 46u64;
                            nat_eq(a2513, a2514)
                        } {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2515 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f677(v0: Adt148) -> bool {
    let m2517 = v0;
    match m2517 {
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

pub fn f678(v0: Adt16) -> Adt148 {
    let m2518 = v0;
    match m2518 {
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

pub fn f679(v0: Adt0, v1: Adt34) -> Adt137 {
    Adt137::C0(v1, Adt138::C0(Some(match v0 {
        Adt0::C0(h2519, _, _, _, _, _, _) => {
            h2519
        }
    }), None::<Adt5>), List::<Adt139>::nil())
}

pub fn f680(v0: Adt0) -> Adt35 {
    Adt35::C0(match v0.clone() {
        Adt0::C0(_, _, _, h2520, _, _, _) => {
            h2520
        }
    }, f317(), match v0.clone() {
        Adt0::C0(_, _, h2521, _, _, _, _) => {
            h2521
        }
    }, match v0 {
        Adt0::C0(_, h2522, _, _, _, _, _) => {
            h2522
        }
    })
}

pub fn f681(v0: Adt0, v1: Adt16, v2: Result<Adt34, Adt86>) -> Result<Adt137, Adt140> {
    let m2523 = v2;
    match m2523 {
        Ok(v4) => {
            if f711(f678(v1.clone()), v4.clone()) {
                f712(v0, v1, v4)
            } else {
                Err::<Adt137, Adt140>(Adt140::C4(f713()))
            }
        }
        Err(v3) => {
            Err::<Adt137, Adt140>(Adt140::C4(v3))
        }
    }
}

pub fn f682(v0: Bytes, v1: u64, v2: u64, v3: Fn2, v4: Adt16) -> R<Result<Adt34, Adt86>> {
    f714(v0.clone(), v1, v2, v3, v4.clone(), f715(v0, v4)?)
}

pub fn f683(v0: Str, v1: Adt5) -> Result<Option<Str>, Adt141> {
    let m2524 = f138(v0.clone(), v1);
    match m2524 {
        Adt108::C0 => {
            Ok::<Option<Str>, Adt141>(None::<Str>)
        }
        Adt108::C1(v2) => {
            let m2525 = f89(v2);
            match m2525 {
                None => {
                    Err::<Option<Str>, Adt141>(Adt141::C8(v0))
                }
                Some(v3) => {
                    Ok::<Option<Str>, Adt141>(Some(v3))
                }
            }
        }
    }
}

pub fn f684(v0: Str, v1: Adt5) -> Result<Str, Adt141> {
    let m2526 = f138(v0.clone(), v1);
    match m2526 {
        Adt108::C0 => {
            Err::<Str, Adt141>(Adt141::C7(v0))
        }
        Adt108::C1(v2) => {
            let m2527 = f89(v2);
            match m2527 {
                None => {
                    Err::<Str, Adt141>(Adt141::C8(v0))
                }
                Some(v3) => {
                    Ok::<Str, Adt141>(v3)
                }
            }
        }
    }
}

pub fn f685(v0: Str, v1: Adt5) -> Result<Option<Adt5>, Adt141> {
    let m2528 = f138(v0.clone(), v1);
    match m2528 {
        Adt108::C0 => {
            Ok::<Option<Adt5>, Adt141>(None::<Adt5>)
        }
        Adt108::C1(v2) => {
            let m2529 = f218(v2);
            match m2529 {
                None => {
                    Err::<Option<Adt5>, Adt141>(Adt141::C8(v0))
                }
                Some(v3) => {
                    Ok::<Option<Adt5>, Adt141>(Some(v3))
                }
            }
        }
    }
}

pub fn f686(v0: Str, v1: Adt5) -> Result<Option<List<(Str, Str)>>, Adt141> {
    let m2530 = f138(v0.clone(), v1);
    match m2530 {
        Adt108::C0 => {
            Ok::<Option<List<(Str, Str)>>, Adt141>(None::<List<(Str, Str)>>)
        }
        Adt108::C1(v2) => {
            let m2531 = f218(v2);
            match m2531 {
                None => {
                    Err::<Option<List<(Str, Str)>>, Adt141>(Adt141::C8(v0))
                }
                Some(v3) => {
                    let m2533 = f716(v0, match v3 {
                        Adt5::C0(h2532) => {
                            h2532
                        }
                    });
                    match m2533 {
                        Ok(v4) => {
                            Ok::<Option<List<(Str, Str)>>, Adt141>(Some(v4))
                        }
                        Err(v5) => {
                            Err::<Option<List<(Str, Str)>>, Adt141>(v5)
                        }
                    }
                }
            }
        }
    }
}

pub fn f687(v0: Str, v1: Adt5) -> Result<Adt5, Adt141> {
    let m2534 = f138(v0.clone(), v1);
    match m2534 {
        Adt108::C0 => {
            Err::<Adt5, Adt141>(Adt141::C7(v0))
        }
        Adt108::C1(v2) => {
            let m2535 = f218(v2);
            match m2535 {
                None => {
                    Err::<Adt5, Adt141>(Adt141::C8(v0))
                }
                Some(v3) => {
                    Ok::<Adt5, Adt141>(v3)
                }
            }
        }
    }
}

pub fn f688(v0: Adt5) -> Result<Adt27, Adt141> {
    let m2536 = f684(Str::lit("type"), v0.clone());
    match m2536 {
        Ok(v1) => {
            let a2537 = v1.clone();
            let a2538 = Str::lit("ref/prompt");
            if equal(a2537, a2538) {
                let m2539 = f684(Str::lit("name"), v0.clone());
                match m2539 {
                    Ok(v2) => {
                        let m2540 = f683(Str::lit("title"), v0);
                        match m2540 {
                            Ok(v3) => {
                                Ok::<Adt27, Adt141>(Adt27::C0(Adt28::C0(v2, v3)))
                            }
                            Err(v4) => {
                                Err::<Adt27, Adt141>(v4)
                            }
                        }
                    }
                    Err(v5) => {
                        Err::<Adt27, Adt141>(v5)
                    }
                }
            } else {
                let a2541 = v1;
                let a2542 = Str::lit("ref/resource");
                if equal(a2541, a2542) {
                    let m2543 = f684(Str::lit("uri"), v0);
                    match m2543 {
                        Ok(v6) => {
                            Ok::<Adt27, Adt141>(Adt27::C1(v6))
                        }
                        Err(v7) => {
                            Err::<Adt27, Adt141>(v7)
                        }
                    }
                } else {
                    Err::<Adt27, Adt141>(Adt141::C8(Str::lit("type")))
                }
            }
        }
        Err(v8) => {
            Err::<Adt27, Adt141>(v8)
        }
    }
}

pub fn f689(v0: Adt5) -> Result<Adt29, Adt141> {
    let m2544 = f684(Str::lit("name"), v0.clone());
    match m2544 {
        Ok(v1) => {
            let m2545 = f684(Str::lit("value"), v0);
            match m2545 {
                Ok(v2) => {
                    Ok::<Adt29, Adt141>(Adt29::C0(v1, v2))
                }
                Err(v3) => {
                    Err::<Adt29, Adt141>(v3)
                }
            }
        }
        Err(v4) => {
            Err::<Adt29, Adt141>(v4)
        }
    }
}

pub fn f690(v0: Adt5) -> Result<Option<Adt30>, Adt141> {
    let m2546 = f138(Str::lit("context"), v0);
    match m2546 {
        Adt108::C0 => {
            Ok::<Option<Adt30>, Adt141>(None::<Adt30>)
        }
        Adt108::C1(v1) => {
            let m2547 = f218(v1);
            match m2547 {
                None => {
                    Err::<Option<Adt30>, Adt141>(Adt141::C8(Str::lit("context")))
                }
                Some(v2) => {
                    let m2548 = f686(Str::lit("arguments"), v2);
                    match m2548 {
                        Ok(v3) => {
                            Ok::<Option<Adt30>, Adt141>(Some(Adt30::C0(v3)))
                        }
                        Err(v4) => {
                            Err::<Option<Adt30>, Adt141>(v4)
                        }
                    }
                }
            }
        }
    }
}

pub fn f691(v0: bool, v1: Str, v2: List<Str>) -> Option<Adt140> {
    if v0 {
        None::<Adt140>
    } else {
        Some(Adt140::C1(v1, v2))
    }
}

pub fn f692(v0: Str, p2553: List<Str>) -> bool {
    let m2552 = p2553;
    {
        let mut m2552 = m2552;
        loop {
            let step = {
                let v1 = m2552;
                let m2549 = v1;
                match m2549.uncons() {
                    None => {
                        Err::<List<Str>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a2550 = v2;
                        let a2551 = v0.clone();
                        if equal(a2550, a2551) {
                            Err::<List<Str>, bool>(true)
                        } else {
                            Ok::<List<Str>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2552 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f693(v0: Option<Adt128>, v1: List<Adt130>, v2: Adt5) -> R<Option<Adt128>> {
    let m2554 = v0;
    match m2554 {
        None => {
            f717(v1, f718(v2))
        }
        Some(v3) => {
            Ok(Some(v3))
        }
    }
}

pub fn f694(v0: Option<Str>, v1: Adt152, v2: Adt5) -> Option<Adt128> {
    let m2555 = v1.clone();
    match m2555 {
        Adt152::C0 => {
            None::<Adt128>
        }
        Adt152::C1 => {
            f719(v0, f720(v1, v2))
        }
        Adt152::C2 => {
            f719(v0, f720(v1, v2))
        }
        Adt152::C3 => {
            f719(v0, f720(v1, v2))
        }
    }
}

pub fn f695(v0: Adt108, v1: Adt108) -> Adt156 {
    let m2556 = v0;
    match m2556 {
        Adt108::C0 => {
            Adt156::C1
        }
        Adt108::C1(_) => {
            let m2557 = v1;
            match m2557 {
                Adt108::C0 => {
                    Adt156::C2
                }
                Adt108::C1(_) => {
                    Adt156::C0
                }
            }
        }
    }
}

pub fn f696(v0: Adt5, v1: Adt6, v2: Adt6) -> Result<Adt18, Adt141> {
    let m2558 = f89(v1);
    match m2558 {
        None => {
            Err::<Adt18, Adt141>(Adt141::C5)
        }
        Some(v3) => {
            let m2559 = f721(v2);
            match m2559 {
                None => {
                    Err::<Adt18, Adt141>(Adt141::C6)
                }
                Some(v4) => {
                    let m2560 = f685(f722(), v0.clone());
                    match m2560 {
                        Ok(v6) => {
                            let m2561 = f723(v0.clone());
                            match m2561 {
                                Ok(v8) => {
                                    let m2562 = f724(v0.clone());
                                    match m2562 {
                                        Ok(v10) => {
                                            Ok::<Adt18, Adt141>(Adt18::C0(v3, v4, v6, v8, v10, v0))
                                        }
                                        Err(v9) => {
                                            Err::<Adt18, Adt141>(v9)
                                        }
                                    }
                                }
                                Err(v7) => {
                                    Err::<Adt18, Adt141>(v7)
                                }
                            }
                        }
                        Err(v5) => {
                            Err::<Adt18, Adt141>(v5)
                        }
                    }
                }
            }
        }
    }
}

pub fn f697(v0: Adt8) -> Adt6 {
    Adt6::C5(f608(Str::lit("listChanged"), match v0 {
        Adt8::C0(h2563) => {
            h2563
        }
    }))
}

pub fn f698(v0: Adt37) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
        Adt37::C0(h2564, _, _, _, _, _, _, _) => {
            h2564
        }
    })), {
        let a2582 = f603(Str::lit("title"), match v0.clone() {
            Adt37::C0(_, h2565, _, _, _, _, _, _) => {
                h2565
            }
        });
        let a2583 = {
            let a2580 = f668(match v0.clone() {
                Adt37::C0(_, _, h2566, _, _, _, _, _) => {
                    h2566
                }
            });
            let a2581 = {
                let a2578 = f603(Str::lit("description"), match v0.clone() {
                    Adt37::C0(_, _, _, h2567, _, _, _, _) => {
                        h2567
                    }
                });
                let a2579 = List::cons((Str::lit("inputSchema"), Adt6::C5({
                    let m2570 = match v0.clone() {
                        Adt37::C0(_, _, _, _, h2569, _, _, _) => {
                            h2569
                        }
                    };
                    match m2570 {
                        Adt5::C0(h2568) => {
                            h2568
                        }
                    }
                })), {
                    let a2576 = f634(Str::lit("outputSchema"), match v0.clone() {
                        Adt37::C0(_, _, _, _, _, h2571, _, _) => {
                            h2571
                        }
                    });
                    let a2577 = {
                        let a2574 = f725(match v0.clone() {
                            Adt37::C0(_, _, _, _, _, _, h2572, _) => {
                                h2572
                            }
                        });
                        let a2575 = f726(match v0 {
                            Adt37::C0(_, _, _, _, _, _, _, h2573) => {
                                h2573
                            }
                        });
                        append_list(a2574, a2575)
                    };
                    append_list(a2576, a2577)
                });
                append_list(a2578, a2579)
            };
            append_list(a2580, a2581)
        };
        append_list(a2582, a2583)
    }))
}

pub fn f699(v0: Adt40) -> R<Option<Adt6>> {
    let m2584 = v0;
    match m2584 {
        Adt40::C0(v1) => {
            Ok(Some(f727(v1)))
        }
        Adt40::C1(v2) => {
            Ok(Some(f728(Str::lit("image"), match v2.clone() {
                Adt44::C0(h2585, _, _, _) => {
                    h2585
                }
            }, match v2.clone() {
                Adt44::C0(_, h2586, _, _) => {
                    h2586
                }
            }, match v2.clone() {
                Adt44::C0(_, _, h2587, _) => {
                    h2587
                }
            }, match v2 {
                Adt44::C0(_, _, _, h2588) => {
                    h2588
                }
            })))
        }
        Adt40::C2(v3) => {
            Ok(Some(f728(Str::lit("audio"), match v3.clone() {
                Adt45::C0(h2589, _, _, _) => {
                    h2589
                }
            }, match v3.clone() {
                Adt45::C0(_, h2590, _, _) => {
                    h2590
                }
            }, match v3.clone() {
                Adt45::C0(_, _, h2591, _) => {
                    h2591
                }
            }, match v3 {
                Adt45::C0(_, _, _, h2592) => {
                    h2592
                }
            })))
        }
        Adt40::C3(v4) => {
            Ok(Some(f729(v4)?))
        }
        Adt40::C4(v5) => {
            Ok(Some(f730(v5)))
        }
    }
}

pub fn f700(v0: Adt46) -> R<Adt6> {
    Ok(Adt6::C5({
        let a2616 = List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt46::C0(h2593, _, _, _, _, _, _, _, _) => {
                h2593
            }
        })), List::<(Str, Adt6)>::nil());
        let a2617 = {
            let a2614 = f603(Str::lit("title"), match v0.clone() {
                Adt46::C0(_, h2594, _, _, _, _, _, _, _) => {
                    h2594
                }
            });
            let a2615 = {
                let a2612 = f668(match v0.clone() {
                    Adt46::C0(_, _, h2595, _, _, _, _, _, _) => {
                        h2595
                    }
                });
                let a2613 = {
                    let a2610 = List::cons((Str::lit("uri"), Adt6::C3(match v0.clone() {
                        Adt46::C0(_, _, _, h2596, _, _, _, _, _) => {
                            h2596
                        }
                    })), List::<(Str, Adt6)>::nil());
                    let a2611 = {
                        let a2608 = f603(Str::lit("description"), match v0.clone() {
                            Adt46::C0(_, _, _, _, h2597, _, _, _, _) => {
                                h2597
                            }
                        });
                        let a2609 = {
                            let a2606 = f603(Str::lit("mimeType"), match v0.clone() {
                                Adt46::C0(_, _, _, _, _, h2598, _, _, _) => {
                                    h2598
                                }
                            });
                            let a2607 = {
                                let a2604 = f731(match v0.clone() {
                                    Adt46::C0(_, _, _, _, _, _, h2599, _, _) => {
                                        h2599
                                    }
                                });
                                let a2605 = {
                                    let a2602 = f732(match v0.clone() {
                                        Adt46::C0(_, _, _, _, _, _, _, h2600, _) => {
                                            h2600
                                        }
                                    })?;
                                    let a2603 = f726(match v0 {
                                        Adt46::C0(_, _, _, _, _, _, _, _, h2601) => {
                                            h2601
                                        }
                                    });
                                    append_list(a2602, a2603)
                                };
                                append_list(a2604, a2605)
                            };
                            append_list(a2606, a2607)
                        };
                        append_list(a2608, a2609)
                    };
                    append_list(a2610, a2611)
                };
                append_list(a2612, a2613)
            };
            append_list(a2614, a2615)
        };
        append_list(a2616, a2617)
    }))
}

pub fn f701(v0: Adt48) -> Adt6 {
    let m2618 = v0;
    match m2618 {
        Adt48::C0(v1) => {
            Adt6::C5(List::cons((Str::lit("uri"), Adt6::C3(match v1.clone() {
                Adt49::C0(h2619, _, _, _) => {
                    h2619
                }
            })), {
                let a2623 = f603(Str::lit("mimeType"), match v1.clone() {
                    Adt49::C0(_, h2620, _, _) => {
                        h2620
                    }
                });
                let a2624 = List::cons((Str::lit("text"), Adt6::C3(match v1.clone() {
                    Adt49::C0(_, _, h2621, _) => {
                        h2621
                    }
                })), f726(match v1 {
                    Adt49::C0(_, _, _, h2622) => {
                        h2622
                    }
                }));
                append_list(a2623, a2624)
            }))
        }
        Adt48::C1(v2) => {
            Adt6::C5(List::cons((Str::lit("uri"), Adt6::C3(match v2.clone() {
                Adt50::C0(h2625, _, _, _) => {
                    h2625
                }
            })), {
                let a2629 = f603(Str::lit("mimeType"), match v2.clone() {
                    Adt50::C0(_, h2626, _, _) => {
                        h2626
                    }
                });
                let a2630 = List::cons((Str::lit("blob"), Adt6::C3(match v2.clone() {
                    Adt50::C0(_, _, h2627, _) => {
                        h2627
                    }
                })), f726(match v2 {
                    Adt50::C0(_, _, _, h2628) => {
                        h2628
                    }
                }));
                append_list(a2629, a2630)
            }))
        }
    }
}

pub fn f702(v0: Adt56) -> Adt6 {
    Adt6::C5({
        let a2651 = List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt56::C0(h2631, _, _, _, _, _, _, _) => {
                h2631
            }
        })), List::<(Str, Adt6)>::nil());
        let a2652 = {
            let a2649 = f603(Str::lit("title"), match v0.clone() {
                Adt56::C0(_, h2632, _, _, _, _, _, _) => {
                    h2632
                }
            });
            let a2650 = {
                let a2647 = f668(match v0.clone() {
                    Adt56::C0(_, _, h2633, _, _, _, _, _) => {
                        h2633
                    }
                });
                let a2648 = {
                    let a2645 = List::cons((Str::lit("uriTemplate"), Adt6::C3(match v0.clone() {
                        Adt56::C0(_, _, _, h2634, _, _, _, _) => {
                            h2634
                        }
                    })), List::<(Str, Adt6)>::nil());
                    let a2646 = {
                        let a2643 = f603(Str::lit("description"), match v0.clone() {
                            Adt56::C0(_, _, _, _, h2635, _, _, _) => {
                                h2635
                            }
                        });
                        let a2644 = {
                            let a2641 = f603(Str::lit("mimeType"), match v0.clone() {
                                Adt56::C0(_, _, _, _, _, h2636, _, _) => {
                                    h2636
                                }
                            });
                            let a2642 = {
                                let a2639 = f731(match v0.clone() {
                                    Adt56::C0(_, _, _, _, _, _, h2637, _) => {
                                        h2637
                                    }
                                });
                                let a2640 = f726(match v0 {
                                    Adt56::C0(_, _, _, _, _, _, _, h2638) => {
                                        h2638
                                    }
                                });
                                append_list(a2639, a2640)
                            };
                            append_list(a2641, a2642)
                        };
                        append_list(a2643, a2644)
                    };
                    append_list(a2645, a2646)
                };
                append_list(a2647, a2648)
            };
            append_list(a2649, a2650)
        };
        append_list(a2651, a2652)
    })
}

pub fn f703(v0: Adt58) -> Adt6 {
    Adt6::C5({
        let a2668 = List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt58::C0(h2653, _, _, _, _, _) => {
                h2653
            }
        })), List::<(Str, Adt6)>::nil());
        let a2669 = {
            let a2666 = f603(Str::lit("title"), match v0.clone() {
                Adt58::C0(_, h2654, _, _, _, _) => {
                    h2654
                }
            });
            let a2667 = {
                let a2664 = f668(match v0.clone() {
                    Adt58::C0(_, _, h2655, _, _, _) => {
                        h2655
                    }
                });
                let a2665 = {
                    let a2662 = f603(Str::lit("description"), match v0.clone() {
                        Adt58::C0(_, _, _, h2656, _, _) => {
                            h2656
                        }
                    });
                    let a2663 = {
                        let a2660 = {
                            let m2658 = match v0.clone() {
                                Adt58::C0(_, _, _, _, h2657, _) => {
                                    h2657
                                }
                            };
                            match m2658 {
                                None => {
                                    List::<(Str, Adt6)>::nil()
                                }
                                Some(v1) => {
                                    List::cons((Str::lit("arguments"), Adt6::C4(f733(v1))), List::<(Str, Adt6)>::nil())
                                }
                            }
                        };
                        let a2661 = f726(match v0 {
                            Adt58::C0(_, _, _, _, _, h2659) => {
                                h2659
                            }
                        });
                        append_list(a2660, a2661)
                    };
                    append_list(a2662, a2663)
                };
                append_list(a2664, a2665)
            };
            append_list(a2666, a2667)
        };
        append_list(a2668, a2669)
    })
}

pub fn f704(v0: Adt61) -> R<Option<Adt6>> {
    let m2671 = f699(match v0.clone() {
        Adt61::C0(_, h2670) => {
            h2670
        }
    })?;
    match m2671 {
        None => {
            Ok(None::<Adt6>)
        }
        Some(v1) => {
            Ok(Some(Adt6::C5(List::cons((Str::lit("role"), Adt6::C3(f734(match v0 {
                Adt61::C0(h2672, _) => {
                    h2672
                }
            }))), List::cons((Str::lit("content"), v1), List::<(Str, Adt6)>::nil())))))
        }
    }
}

pub fn f705(v0: Str, v1: Option<Adt6>) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("method"), Adt6::C3(v0)), f607(Str::lit("params"), v1)))
}

pub fn f706(v0: Adt67) -> Adt6 {
    let m2673 = v0;
    match m2673 {
        Adt67::C0(v1) => {
            f735(v1)
        }
        Adt67::C1(v2) => {
            Adt6::C5(List::cons((Str::lit("mode"), Adt6::C3(Str::lit("url"))), List::cons((Str::lit("message"), Adt6::C3(match v2.clone() {
                Adt70::C0(h2674, _) => {
                    h2674
                }
            })), List::cons((Str::lit("url"), Adt6::C3(match v2 {
                Adt70::C0(_, h2675) => {
                    h2675
                }
            })), List::<(Str, Adt6)>::nil()))))
        }
    }
}

pub fn f707(v0: Adt71) -> R<Option<Adt6>> {
    let m2677 = f736(match v0.clone() {
        Adt71::C0(h2676, _, _, _, _, _, _, _, _, _) => {
            h2676
        }
    })?;
    match m2677 {
        None => {
            Ok(None::<Adt6>)
        }
        Some(v1) => {
            Ok(Some(Adt6::C5(List::cons((Str::lit("messages"), Adt6::C4(v1)), {
                let a2679 = f737(v0.clone());
                let a2680 = List::cons((Str::lit("maxTokens"), f53(match v0.clone() {
                    Adt71::C0(_, _, _, _, _, h2678, _, _, _, _) => {
                        h2678
                    }
                })?), f738(v0));
                append_list(a2679, a2680)
            }))))
        }
    }
}

pub fn f708(v0: List<Adt2>) -> List<Adt6> {
    f431(f739(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f709(v0: List<u64>) -> List<u64> {
    let a2683 = f740();
    let a2684 = {
        let a2681 = v0;
        let a2682 = f741();
        append_list(a2681, a2682)
    };
    append_list(a2683, a2684)
}

pub fn f710(v0: List<u64>, v1: List<u64>) -> List<u64> {
    f742(v0, 0u64, v1)
}

pub fn f711(v0: Adt148, v1: Adt34) -> bool {
    let m2685 = v1;
    match m2685 {
        Adt34::C0(_) => {
            f743(v0, Adt148::C0)
        }
        Adt34::C1(_) => {
            f743(v0, Adt148::C1)
        }
        Adt34::C2(_) => {
            f743(v0, Adt148::C2)
        }
        Adt34::C3(_) => {
            f743(v0, Adt148::C2)
        }
        Adt34::C4(_) => {
            f743(v0, Adt148::C3)
        }
        Adt34::C5(_) => {
            f743(v0, Adt148::C4)
        }
        Adt34::C6(_) => {
            f743(v0, Adt148::C5)
        }
        Adt34::C7(_) => {
            f743(v0, Adt148::C6)
        }
        Adt34::C8(_) => {
            f743(v0, Adt148::C7)
        }
        Adt34::C9(_) => {
            f743(v0, Adt148::C8)
        }
        Adt34::C10(_) => {
            f743(v0, Adt148::C9)
        }
        Adt34::C11 => {
            if f743(v0.clone(), Adt148::C10) {
                true
            } else {
                f743(v0, Adt148::C11)
            }
        }
        Adt34::C12(_) => {
            f744(v0)
        }
        Adt34::C13(_) => {
            f745(v0)
        }
    }
}

pub fn f712(v0: Adt0, v1: Adt16, v2: Adt34) -> Result<Adt137, Adt140> {
    let m2686 = v2.clone();
    match m2686 {
        Adt34::C0(_) => {
            Ok::<Adt137, Adt140>(f679(v0, v2))
        }
        Adt34::C1(_) => {
            Ok::<Adt137, Adt140>(f679(v0, v2))
        }
        Adt34::C2(_) => {
            Ok::<Adt137, Adt140>(f679(v0, v2))
        }
        Adt34::C3(v8) => {
            Ok::<Adt137, Adt140>(f749(v0, f747(v1), v8))
        }
        Adt34::C4(_) => {
            Ok::<Adt137, Adt140>(f679(v0, v2))
        }
        Adt34::C5(_) => {
            Ok::<Adt137, Adt140>(f679(v0, v2))
        }
        Adt34::C6(_) => {
            Ok::<Adt137, Adt140>(f679(v0, v2))
        }
        Adt34::C7(_) => {
            Ok::<Adt137, Adt140>(f679(v0, v2))
        }
        Adt34::C8(_) => {
            Ok::<Adt137, Adt140>(f679(v0, v2))
        }
        Adt34::C9(_) => {
            Ok::<Adt137, Adt140>(f679(v0, v2))
        }
        Adt34::C10(_) => {
            Ok::<Adt137, Adt140>(f679(v0, v2))
        }
        Adt34::C11 => {
            Ok::<Adt137, Adt140>(f679(v0, v2))
        }
        Adt34::C12(v3) => {
            f746(v0, f747(v1), v3)
        }
        Adt34::C13(v4) => {
            f748(v0, f747(v1), v4)
        }
    }
}

pub fn f713() -> Adt86 {
    Adt86::C9(Str::lit("the provider answered with a result that does not belong to the method"))
}

pub fn f714(v0: Bytes, v1: u64, v2: u64, v3: Fn2, v4: Adt16, v5: Result<Adt16, Adt86>) -> R<Result<Adt34, Adt86>> {
    let m2687 = v5;
    match m2687 {
        Ok(v7) => {
            f750(v0, v4, f751(v1, v2, v3, v7)?)
        }
        Err(v6) => {
            Ok(Err::<Adt34, Adt86>(v6))
        }
    }
}

pub fn f715(v0: Bytes, v1: Adt16) -> R<Result<Adt16, Adt86>> {
    let m2688 = v1.clone();
    match m2688 {
        Adt16::C0(_) => {
            Ok(Ok::<Adt16, Adt86>(v1))
        }
        Adt16::C1(_) => {
            Ok(Ok::<Adt16, Adt86>(v1))
        }
        Adt16::C2(v2) => {
            f752(v0, v2)
        }
        Adt16::C3(_) => {
            Ok(Ok::<Adt16, Adt86>(v1))
        }
        Adt16::C4(v3) => {
            f753(v0, v3)
        }
        Adt16::C5(_) => {
            Ok(Ok::<Adt16, Adt86>(v1))
        }
        Adt16::C6(_) => {
            Ok(Ok::<Adt16, Adt86>(v1))
        }
        Adt16::C7(v4) => {
            f754(v0, v4)
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

pub fn f716(v0: Str, v1: List<(Str, Adt6)>) -> Result<List<(Str, Str)>, Adt141> {
    let m2689 = f755(v0, v1, List::<(Str, Str)>::nil());
    match m2689 {
        Ok(v2) => {
            Ok::<List<(Str, Str)>, Adt141>(f756(v2, List::<(Str, Str)>::nil()))
        }
        Err(v3) => {
            Err::<List<(Str, Str)>, Adt141>(v3)
        }
    }
}

pub fn f717(p2693: List<Adt130>, v1: Adt5) -> R<Option<Adt128>> {
    let m2692 = p2693;
    {
        let mut m2692 = m2692;
        loop {
            let step = {
                let v0 = m2692;
                let m2690 = v0;
                match m2690.uncons() {
                    None => {
                        Err::<List<Adt130>, R<Option<Adt128>>>(Ok(None::<Adt128>))
                    }
                    Some((v2, v3)) => {
                        let m2691 = f757(v2, v1.clone())?;
                        match m2691 {
                            None => {
                                Ok::<List<Adt130>, R<Option<Adt128>>>(v3)
                            }
                            Some(v4) => {
                                Err::<List<Adt130>, R<Option<Adt128>>>(Ok(Some(v4)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2692 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f718(v0: Adt5) -> Adt5 {
    let m2694 = f138(Str::lit("arguments"), v0);
    match m2694 {
        Adt108::C0 => {
            f652()
        }
        Adt108::C1(v1) => {
            let m2695 = f218(v1);
            match m2695 {
                None => {
                    f652()
                }
                Some(v2) => {
                    v2
                }
            }
        }
    }
}

pub fn f719(v0: Option<Str>, v1: Option<Str>) -> Option<Adt128> {
    let m2696 = v0;
    match m2696 {
        None => {
            Some(Adt128::C2)
        }
        Some(v2) => {
            let m2697 = v1;
            match m2697 {
                None => {
                    None::<Adt128>
                }
                Some(v3) => {
                    let a2698 = v2;
                    let a2699 = v3;
                    if equal(a2698, a2699) {
                        None::<Adt128>
                    } else {
                        Some(Adt128::C5)
                    }
                }
            }
        }
    }
}

pub fn f720(v0: Adt152, v1: Adt5) -> Option<Str> {
    let m2700 = v0;
    match m2700 {
        Adt152::C0 => {
            None::<Str>
        }
        Adt152::C1 => {
            let m2701 = f138(Str::lit("name"), v1);
            match m2701 {
                Adt108::C0 => {
                    None::<Str>
                }
                Adt108::C1(v2) => {
                    f89(v2)
                }
            }
        }
        Adt152::C2 => {
            let m2702 = f138(Str::lit("uri"), v1);
            match m2702 {
                Adt108::C0 => {
                    None::<Str>
                }
                Adt108::C1(v3) => {
                    f89(v3)
                }
            }
        }
        Adt152::C3 => {
            let m2703 = f138(Str::lit("taskId"), v1);
            match m2703 {
                Adt108::C0 => {
                    None::<Str>
                }
                Adt108::C1(v4) => {
                    f89(v4)
                }
            }
        }
    }
}

pub fn f721(v0: Adt6) -> Option<Adt19> {
    let m2704 = f218(v0);
    match m2704 {
        None => {
            None::<Adt19>
        }
        Some(v1) => {
            let m2705 = f758(Str::lit("experimental"), v1.clone());
            match m2705 {
                None => {
                    None::<Adt19>
                }
                Some(v2) => {
                    let m2706 = f758(Str::lit("roots"), v1.clone());
                    match m2706 {
                        None => {
                            None::<Adt19>
                        }
                        Some(v3) => {
                            let m2707 = f758(Str::lit("sampling"), v1.clone());
                            match m2707 {
                                None => {
                                    None::<Adt19>
                                }
                                Some(v4) => {
                                    let m2708 = f758(Str::lit("elicitation"), v1.clone());
                                    match m2708 {
                                        None => {
                                            None::<Adt19>
                                        }
                                        Some(v5) => {
                                            let m2709 = f758(Str::lit("extensions"), v1);
                                            match m2709 {
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

pub fn f722() -> Str {
    Str::lit("io.modelcontextprotocol/clientInfo")
}

pub fn f723(v0: Adt5) -> Result<Option<Adt20>, Adt141> {
    let m2710 = f683(f759(), v0);
    match m2710 {
        Ok(v2) => {
            let m2711 = v2;
            match m2711 {
                None => {
                    Ok::<Option<Adt20>, Adt141>(None::<Adt20>)
                }
                Some(v3) => {
                    let m2712 = f760(v3);
                    match m2712 {
                        Adt158::C0 => {
                            Err::<Option<Adt20>, Adt141>(Adt141::C8(f759()))
                        }
                        Adt158::C1(v4) => {
                            Ok::<Option<Adt20>, Adt141>(Some(v4))
                        }
                    }
                }
            }
        }
        Err(v1) => {
            Err::<Option<Adt20>, Adt141>(v1)
        }
    }
}

pub fn f724(v0: Adt5) -> Result<Option<Adt21>, Adt141> {
    let m2713 = f138(f761(), v0);
    match m2713 {
        Adt108::C0 => {
            Ok::<Option<Adt21>, Adt141>(None::<Adt21>)
        }
        Adt108::C1(v1) => {
            let m2714 = f89(v1.clone());
            match m2714 {
                None => {
                    let m2715 = f484(v1);
                    match m2715 {
                        None => {
                            Err::<Option<Adt21>, Adt141>(Adt141::C8(f761()))
                        }
                        Some(v3) => {
                            Ok::<Option<Adt21>, Adt141>(Some(Adt21::C1(v3)))
                        }
                    }
                }
                Some(v2) => {
                    Ok::<Option<Adt21>, Adt141>(Some(Adt21::C0(v2)))
                }
            }
        }
    }
}

pub fn f725(v0: Option<Adt38>) -> List<(Str, Adt6)> {
    let m2716 = v0;
    match m2716 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("annotations"), Adt6::C5({
                let a2728 = f603(Str::lit("title"), match v1.clone() {
                    Adt38::C0(h2717, _, _, _, _) => {
                        h2717
                    }
                });
                let a2729 = {
                    let a2726 = f608(Str::lit("readOnlyHint"), match v1.clone() {
                        Adt38::C0(_, h2718, _, _, _) => {
                            h2718
                        }
                    });
                    let a2727 = {
                        let a2724 = f608(Str::lit("destructiveHint"), match v1.clone() {
                            Adt38::C0(_, _, h2719, _, _) => {
                                h2719
                            }
                        });
                        let a2725 = {
                            let a2722 = f608(Str::lit("idempotentHint"), match v1.clone() {
                                Adt38::C0(_, _, _, h2720, _) => {
                                    h2720
                                }
                            });
                            let a2723 = f608(Str::lit("openWorldHint"), match v1 {
                                Adt38::C0(_, _, _, _, h2721) => {
                                    h2721
                                }
                            });
                            append_list(a2722, a2723)
                        };
                        append_list(a2724, a2725)
                    };
                    append_list(a2726, a2727)
                };
                append_list(a2728, a2729)
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f726(v0: Option<Adt5>) -> List<(Str, Adt6)> {
    f634(Str::lit("_meta"), v0)
}

pub fn f727(v0: Adt41) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("text"))), List::cons((Str::lit("text"), Adt6::C3(match v0.clone() {
        Adt41::C0(h2730, _, _) => {
            h2730
        }
    })), {
        let a2733 = f731(match v0.clone() {
            Adt41::C0(_, h2731, _) => {
                h2731
            }
        });
        let a2734 = f726(match v0 {
            Adt41::C0(_, _, h2732) => {
                h2732
            }
        });
        append_list(a2733, a2734)
    })))
}

pub fn f728(v0: Str, v1: Str, v2: Str, v3: Option<Adt42>, v4: Option<Adt5>) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(v0)), List::cons((Str::lit("data"), Adt6::C3(v1)), List::cons((Str::lit("mimeType"), Adt6::C3(v2)), {
        let a2735 = f731(v3);
        let a2736 = f726(v4);
        append_list(a2735, a2736)
    }))))
}

pub fn f729(v0: Adt46) -> R<Adt6> {
    Ok(Adt6::C5({
        let a2760 = List::cons((Str::lit("type"), Adt6::C3(Str::lit("resource_link"))), List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt46::C0(h2737, _, _, _, _, _, _, _, _) => {
                h2737
            }
        })), List::<(Str, Adt6)>::nil()));
        let a2761 = {
            let a2758 = f603(Str::lit("title"), match v0.clone() {
                Adt46::C0(_, h2738, _, _, _, _, _, _, _) => {
                    h2738
                }
            });
            let a2759 = {
                let a2756 = f668(match v0.clone() {
                    Adt46::C0(_, _, h2739, _, _, _, _, _, _) => {
                        h2739
                    }
                });
                let a2757 = {
                    let a2754 = List::cons((Str::lit("uri"), Adt6::C3(match v0.clone() {
                        Adt46::C0(_, _, _, h2740, _, _, _, _, _) => {
                            h2740
                        }
                    })), List::<(Str, Adt6)>::nil());
                    let a2755 = {
                        let a2752 = f603(Str::lit("description"), match v0.clone() {
                            Adt46::C0(_, _, _, _, h2741, _, _, _, _) => {
                                h2741
                            }
                        });
                        let a2753 = {
                            let a2750 = f603(Str::lit("mimeType"), match v0.clone() {
                                Adt46::C0(_, _, _, _, _, h2742, _, _, _) => {
                                    h2742
                                }
                            });
                            let a2751 = {
                                let a2748 = f731(match v0.clone() {
                                    Adt46::C0(_, _, _, _, _, _, h2743, _, _) => {
                                        h2743
                                    }
                                });
                                let a2749 = {
                                    let a2746 = f732(match v0.clone() {
                                        Adt46::C0(_, _, _, _, _, _, _, h2744, _) => {
                                            h2744
                                        }
                                    })?;
                                    let a2747 = f726(match v0 {
                                        Adt46::C0(_, _, _, _, _, _, _, _, h2745) => {
                                            h2745
                                        }
                                    });
                                    append_list(a2746, a2747)
                                };
                                append_list(a2748, a2749)
                            };
                            append_list(a2750, a2751)
                        };
                        append_list(a2752, a2753)
                    };
                    append_list(a2754, a2755)
                };
                append_list(a2756, a2757)
            };
            append_list(a2758, a2759)
        };
        append_list(a2760, a2761)
    }))
}

pub fn f730(v0: Adt47) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("resource"))), List::cons((Str::lit("resource"), f701(match v0.clone() {
        Adt47::C0(h2762, _, _) => {
            h2762
        }
    })), {
        let a2765 = f731(match v0.clone() {
            Adt47::C0(_, h2763, _) => {
                h2763
            }
        });
        let a2766 = f726(match v0 {
            Adt47::C0(_, _, h2764) => {
                h2764
            }
        });
        append_list(a2765, a2766)
    })))
}

pub fn f731(v0: Option<Adt42>) -> List<(Str, Adt6)> {
    let m2767 = v0;
    match m2767 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("annotations"), Adt6::C5({
                let a2773 = f762(match v1.clone() {
                    Adt42::C0(h2768, _, _) => {
                        h2768
                    }
                });
                let a2774 = {
                    let a2771 = f763(match v1.clone() {
                        Adt42::C0(_, h2769, _) => {
                            h2769
                        }
                    });
                    let a2772 = f603(Str::lit("lastModified"), match v1 {
                        Adt42::C0(_, _, h2770) => {
                            h2770
                        }
                    });
                    append_list(a2771, a2772)
                };
                append_list(a2773, a2774)
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f732(v0: Option<u64>) -> R<List<(Str, Adt6)>> {
    let m2775 = v0;
    match m2775 {
        None => {
            Ok(List::<(Str, Adt6)>::nil())
        }
        Some(v1) => {
            Ok(List::cons((Str::lit("size"), f53(v1)?), List::<(Str, Adt6)>::nil()))
        }
    }
}

pub fn f733(v0: List<Adt59>) -> List<Adt6> {
    f431(f764(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f734(v0: Adt43) -> Str {
    let m2776 = v0;
    match m2776 {
        Adt43::C0 => {
            Str::lit("user")
        }
        Adt43::C1 => {
            Str::lit("assistant")
        }
    }
}

pub fn f735(v0: Adt68) -> Adt6 {
    Adt6::C5({
        let a2783 = {
            let m2778 = match v0.clone() {
                Adt68::C0(h2777, _, _) => {
                    h2777
                }
            };
            match m2778 {
                None => {
                    List::<(Str, Adt6)>::nil()
                }
                Some(_) => {
                    List::cons((Str::lit("mode"), Adt6::C3(Str::lit("form"))), List::<(Str, Adt6)>::nil())
                }
            }
        };
        let a2784 = List::cons((Str::lit("message"), Adt6::C3(match v0.clone() {
            Adt68::C0(_, h2779, _) => {
                h2779
            }
        })), List::cons((Str::lit("requestedSchema"), Adt6::C5({
            let m2782 = match v0 {
                Adt68::C0(_, _, h2781) => {
                    h2781
                }
            };
            match m2782 {
                Adt5::C0(h2780) => {
                    h2780
                }
            }
        })), List::<(Str, Adt6)>::nil()));
        append_list(a2783, a2784)
    })
}

pub fn f736(v0: List<Adt72>) -> R<Option<List<Adt6>>> {
    let m2785 = f765(v0, List::<Adt6>::nil())?;
    match m2785 {
        None => {
            Ok(None::<List<Adt6>>)
        }
        Some(v1) => {
            Ok(Some(f431(v1, List::<Adt6>::nil())))
        }
    }
}

pub fn f737(v0: Adt71) -> List<(Str, Adt6)> {
    let a2796 = {
        let m2787 = match v0.clone() {
            Adt71::C0(_, h2786, _, _, _, _, _, _, _, _) => {
                h2786
            }
        };
        match m2787 {
            None => {
                List::<(Str, Adt6)>::nil()
            }
            Some(v1) => {
                List::cons((Str::lit("modelPreferences"), f766(v1)), List::<(Str, Adt6)>::nil())
            }
        }
    };
    let a2797 = {
        let a2794 = f603(Str::lit("systemPrompt"), match v0.clone() {
            Adt71::C0(_, _, h2788, _, _, _, _, _, _, _) => {
                h2788
            }
        });
        let a2795 = {
            let a2792 = {
                let m2790 = match v0.clone() {
                    Adt71::C0(_, _, _, h2789, _, _, _, _, _, _) => {
                        h2789
                    }
                };
                match m2790 {
                    None => {
                        List::<(Str, Adt6)>::nil()
                    }
                    Some(v2) => {
                        List::cons((Str::lit("includeContext"), Adt6::C3(f767(v2))), List::<(Str, Adt6)>::nil())
                    }
                }
            };
            let a2793 = f768(Str::lit("temperature"), match v0 {
                Adt71::C0(_, _, _, _, h2791, _, _, _, _, _) => {
                    h2791
                }
            });
            append_list(a2792, a2793)
        };
        append_list(a2794, a2795)
    };
    append_list(a2796, a2797)
}

pub fn f738(v0: Adt71) -> List<(Str, Adt6)> {
    let a2809 = {
        let m2799 = match v0.clone() {
            Adt71::C0(_, _, _, _, _, _, h2798, _, _, _) => {
                h2798
            }
        };
        match m2799 {
            None => {
                List::<(Str, Adt6)>::nil()
            }
            Some(v1) => {
                List::cons((Str::lit("stopSequences"), f601(v1)), List::<(Str, Adt6)>::nil())
            }
        }
    };
    let a2810 = {
        let a2807 = f634(Str::lit("metadata"), match v0.clone() {
            Adt71::C0(_, _, _, _, _, _, _, h2800, _, _) => {
                h2800
            }
        });
        let a2808 = {
            let a2805 = {
                let m2802 = match v0.clone() {
                    Adt71::C0(_, _, _, _, _, _, _, _, h2801, _) => {
                        h2801
                    }
                };
                match m2802 {
                    None => {
                        List::<(Str, Adt6)>::nil()
                    }
                    Some(v2) => {
                        List::cons((Str::lit("tools"), Adt6::C4(f605(v2))), List::<(Str, Adt6)>::nil())
                    }
                }
            };
            let a2806 = {
                let m2804 = match v0 {
                    Adt71::C0(_, _, _, _, _, _, _, _, _, h2803) => {
                        h2803
                    }
                };
                match m2804 {
                    None => {
                        List::<(Str, Adt6)>::nil()
                    }
                    Some(v3) => {
                        List::cons((Str::lit("toolChoice"), Adt6::C5(f769(v3))), List::<(Str, Adt6)>::nil())
                    }
                }
            };
            append_list(a2805, a2806)
        };
        append_list(a2807, a2808)
    };
    append_list(a2809, a2810)
}

pub fn f739(p2813: List<Adt2>, p2814: List<Adt6>) -> List<Adt6> {
    let m2812 = (p2813, p2814);
    {
        let mut m2812 = m2812;
        loop {
            let step = {
                let (v0, v1) = m2812;
                let m2811 = v0;
                match m2811.uncons() {
                    None => {
                        Err::<(List<Adt2>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt2>, List<Adt6>), List<Adt6>>((v3, List::cons(f770(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2812 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f740() -> List<u64> {
    List::cons(100u64, List::cons(97u64, List::cons(116u64, List::cons(97u64, List::cons(58u64, List::cons(32u64, List::<u64>::nil()))))))
}

pub fn f741() -> List<u64> {
    List::cons(10u64, List::cons(10u64, List::<u64>::nil()))
}

pub fn f742(p2819: List<u64>, p2820: u64, p2821: List<u64>) -> List<u64> {
    let m2818 = (p2819, (p2820, p2821));
    {
        let mut m2818 = m2818;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2818;
                let m2815 = v0;
                match m2815.uncons() {
                    None => {
                        Err::<(List<u64>, (u64, List<u64>)), List<u64>>(f771(v1, v2))
                    }
                    Some((v3, v4)) => {
                        let v5: Adt157 = f772(v1, v3, v2);
                        Ok::<(List<u64>, (u64, List<u64>)), List<u64>>((v4, (match v5.clone() {
                            Adt157::C0(h2816, _) => {
                                h2816
                            }
                        }, match v5 {
                            Adt157::C0(_, h2817) => {
                                h2817
                            }
                        })))
                    }
                }
            };
            match step {
                Ok(next) => { m2818 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f743(v0: Adt148, v1: Adt148) -> bool {
    let a2822 = f582(v0);
    let a2823 = f582(v1);
    equal(a2822, a2823)
}

pub fn f744(v0: Adt148) -> bool {
    let m2824 = v0;
    match m2824 {
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

pub fn f745(v0: Adt148) -> bool {
    let m2825 = v0;
    match m2825 {
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

pub fn f746(v0: Adt0, v1: Adt18, v2: Adt83) -> Result<Adt137, Adt140> {
    f773(v0, v2.clone(), f774(match v1 {
        Adt18::C0(_, h2826, _, _, _, _) => {
            h2826
        }
    }, f616(v2)))
}

pub fn f747(v0: Adt16) -> Adt18 {
    let m2827 = v0;
    match m2827 {
        Adt16::C0(v1) => {
            match v1 {
                Adt17::C0(h2828) => {
                    h2828
                }
            }
        }
        Adt16::C1(v2) => {
            match v2 {
                Adt22::C0(h2829, _) => {
                    h2829
                }
            }
        }
        Adt16::C2(v3) => {
            match v3 {
                Adt23::C0(h2830, _, _, _, _) => {
                    h2830
                }
            }
        }
        Adt16::C3(v4) => {
            match v4 {
                Adt22::C0(h2831, _) => {
                    h2831
                }
            }
        }
        Adt16::C4(v5) => {
            match v5 {
                Adt24::C0(h2832, _, _, _) => {
                    h2832
                }
            }
        }
        Adt16::C5(v6) => {
            match v6 {
                Adt22::C0(h2833, _) => {
                    h2833
                }
            }
        }
        Adt16::C6(v7) => {
            match v7 {
                Adt22::C0(h2834, _) => {
                    h2834
                }
            }
        }
        Adt16::C7(v8) => {
            match v8 {
                Adt25::C0(h2835, _, _, _, _) => {
                    h2835
                }
            }
        }
        Adt16::C8(v9) => {
            match v9 {
                Adt26::C0(h2836, _, _, _) => {
                    h2836
                }
            }
        }
        Adt16::C9(v10) => {
            match v10 {
                Adt31::C0(h2837, _) => {
                    h2837
                }
            }
        }
        Adt16::C10(v11) => {
            match v11 {
                Adt32::C0(h2838, _, _) => {
                    h2838
                }
            }
        }
        Adt16::C11(v12) => {
            match v12 {
                Adt33::C0(h2839, _) => {
                    h2839
                }
            }
        }
    }
}

pub fn f748(v0: Adt0, v1: Adt18, v2: Adt84) -> Result<Adt137, Adt140> {
    f775(v0, v2, f776(match v1 {
        Adt18::C0(_, h2840, _, _, _, _) => {
            h2840
        }
    }))
}

pub fn f749(v0: Adt0, v1: Adt18, v2: Adt51) -> Adt137 {
    Adt137::C0(Adt34::C2(match v2.clone() {
        Adt51::C0(h2841, _) => {
            h2841
        }
    }), {
        let m2843 = f679(v0, Adt34::C11);
        match m2843 {
            Adt137::C0(_, h2842, _) => {
                h2842
            }
        }
    }, f777(v1, match v2 {
        Adt51::C0(_, h2844) => {
            h2844
        }
    }))
}

pub fn f750(v0: Bytes, v1: Adt16, v2: Result<Adt34, Adt86>) -> R<Result<Adt34, Adt86>> {
    let m2845 = v2;
    match m2845 {
        Ok(v4) => {
            f778(v0, f678(v1.clone()), f779(v1), v4)
        }
        Err(v3) => {
            Ok(Err::<Adt34, Adt86>(v3))
        }
    }
}

pub fn f751(v0: u64, v1: u64, v2: Fn2, v3: Adt16) -> R<Result<Adt34, Adt86>> {
    let m2846 = v3.clone();
    match m2846 {
        Adt16::C0(_) => {
            let c2848: Fn2 = v2;
            c2848.apply(v3)
        }
        Adt16::C1(_) => {
            let c2849: Fn2 = v2;
            c2849.apply(v3)
        }
        Adt16::C2(_) => {
            if f780(v0, v1) {
                let c2847: Fn2 = v2;
                c2847.apply(v3)
            } else {
                Ok(Err::<Adt34, Adt86>(Adt86::C7(v0)))
            }
        }
        Adt16::C3(_) => {
            let c2850: Fn2 = v2;
            c2850.apply(v3)
        }
        Adt16::C4(_) => {
            let c2851: Fn2 = v2;
            c2851.apply(v3)
        }
        Adt16::C5(_) => {
            let c2852: Fn2 = v2;
            c2852.apply(v3)
        }
        Adt16::C6(_) => {
            let c2853: Fn2 = v2;
            c2853.apply(v3)
        }
        Adt16::C7(_) => {
            let c2854: Fn2 = v2;
            c2854.apply(v3)
        }
        Adt16::C8(_) => {
            let c2855: Fn2 = v2;
            c2855.apply(v3)
        }
        Adt16::C9(_) => {
            let c2856: Fn2 = v2;
            c2856.apply(v3)
        }
        Adt16::C10(_) => {
            let c2857: Fn2 = v2;
            c2857.apply(v3)
        }
        Adt16::C11(_) => {
            let c2858: Fn2 = v2;
            c2858.apply(v3)
        }
    }
}

pub fn f752(v0: Bytes, v1: Adt23) -> R<Result<Adt16, Adt86>> {
    let m2862 = f781(v0, Str::lit("tools/call"), match v1.clone() {
        Adt23::C0(_, h2859, _, _, _) => {
            h2859
        }
    }, match v1.clone() {
        Adt23::C0(_, _, _, _, h2860) => {
            h2860
        }
    }, match v1.clone() {
        Adt23::C0(_, _, _, h2861, _) => {
            h2861
        }
    })?;
    match m2862 {
        Ok(v3) => {
            Ok(Ok::<Adt16, Adt86>(Adt16::C2(Adt23::C0(match v1.clone() {
                Adt23::C0(h2863, _, _, _, _) => {
                    h2863
                }
            }, match v1.clone() {
                Adt23::C0(_, h2864, _, _, _) => {
                    h2864
                }
            }, match v1.clone() {
                Adt23::C0(_, _, h2865, _, _) => {
                    h2865
                }
            }, match v1 {
                Adt23::C0(_, _, _, h2866, _) => {
                    h2866
                }
            }, v3))))
        }
        Err(v2) => {
            Ok(Err::<Adt16, Adt86>(v2))
        }
    }
}

pub fn f753(v0: Bytes, v1: Adt24) -> R<Result<Adt16, Adt86>> {
    let m2870 = f781(v0, Str::lit("resources/read"), match v1.clone() {
        Adt24::C0(_, h2867, _, _) => {
            h2867
        }
    }, match v1.clone() {
        Adt24::C0(_, _, _, h2868) => {
            h2868
        }
    }, match v1.clone() {
        Adt24::C0(_, _, h2869, _) => {
            h2869
        }
    })?;
    match m2870 {
        Ok(v3) => {
            Ok(Ok::<Adt16, Adt86>(Adt16::C4(Adt24::C0(match v1.clone() {
                Adt24::C0(h2871, _, _, _) => {
                    h2871
                }
            }, match v1.clone() {
                Adt24::C0(_, h2872, _, _) => {
                    h2872
                }
            }, match v1 {
                Adt24::C0(_, _, h2873, _) => {
                    h2873
                }
            }, v3))))
        }
        Err(v2) => {
            Ok(Err::<Adt16, Adt86>(v2))
        }
    }
}

pub fn f754(v0: Bytes, v1: Adt25) -> R<Result<Adt16, Adt86>> {
    let m2877 = f781(v0, Str::lit("prompts/get"), match v1.clone() {
        Adt25::C0(_, h2874, _, _, _) => {
            h2874
        }
    }, match v1.clone() {
        Adt25::C0(_, _, _, _, h2875) => {
            h2875
        }
    }, match v1.clone() {
        Adt25::C0(_, _, _, h2876, _) => {
            h2876
        }
    })?;
    match m2877 {
        Ok(v3) => {
            Ok(Ok::<Adt16, Adt86>(Adt16::C7(Adt25::C0(match v1.clone() {
                Adt25::C0(h2878, _, _, _, _) => {
                    h2878
                }
            }, match v1.clone() {
                Adt25::C0(_, h2879, _, _, _) => {
                    h2879
                }
            }, match v1.clone() {
                Adt25::C0(_, _, h2880, _, _) => {
                    h2880
                }
            }, match v1 {
                Adt25::C0(_, _, _, h2881, _) => {
                    h2881
                }
            }, v3))))
        }
        Err(v2) => {
            Ok(Err::<Adt16, Adt86>(v2))
        }
    }
}

pub fn f755(v0: Str, p2887: List<(Str, Adt6)>, p2888: List<(Str, Str)>) -> Result<List<(Str, Str)>, Adt141> {
    let m2886 = (p2887, p2888);
    {
        let mut m2886 = m2886;
        loop {
            let step = {
                let (v1, v2) = m2886;
                let m2882 = v1;
                match m2882.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<(Str, Str)>), Result<List<(Str, Str)>, Adt141>>(Ok::<List<(Str, Str)>, Adt141>(v2))
                    }
                    Some((v3, v4)) => {
                        let m2884 = f89({
                            let (_, h2883) = v3;
                            h2883
                        });
                        match m2884 {
                            None => {
                                Err::<(List<(Str, Adt6)>, List<(Str, Str)>), Result<List<(Str, Str)>, Adt141>>(Err::<List<(Str, Str)>, Adt141>(Adt141::C8(v0.clone())))
                            }
                            Some(v5) => {
                                Ok::<(List<(Str, Adt6)>, List<(Str, Str)>), Result<List<(Str, Str)>, Adt141>>((v4, List::cons(({
                                    let (h2885, _) = v3;
                                    h2885
                                }, v5), v2)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2886 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f756(p2891: List<(Str, Str)>, p2892: List<(Str, Str)>) -> List<(Str, Str)> {
    let m2890 = (p2891, p2892);
    {
        let mut m2890 = m2890;
        loop {
            let step = {
                let (v0, v1) = m2890;
                let m2889 = v0;
                match m2889.uncons() {
                    None => {
                        Err::<(List<(Str, Str)>, List<(Str, Str)>), List<(Str, Str)>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Str)>, List<(Str, Str)>), List<(Str, Str)>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2890 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f757(v0: Adt130, v1: Adt5) -> R<Option<Adt128>> {
    let m2894 = f782(match v0.clone() {
        Adt130::C0(_, h2893, _, _) => {
            h2893
        }
    }, v1);
    match m2894 {
        Adt108::C0 => {
            Ok(f783(v0))
        }
        Adt108::C1(v2) => {
            if f784(v2.clone()) {
                Ok(f783(v0))
            } else {
                f785(v0, v2)
            }
        }
    }
}

pub fn f758(v0: Str, v1: Adt5) -> Option<Option<Adt5>> {
    let m2895 = f138(v0, v1);
    match m2895 {
        Adt108::C0 => {
            Some(None::<Adt5>)
        }
        Adt108::C1(v2) => {
            let m2896 = f218(v2);
            match m2896 {
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

pub fn f759() -> Str {
    Str::lit("io.modelcontextprotocol/logLevel")
}

pub fn f760(v0: Str) -> Adt158 {
    let a2897 = v0.clone();
    let a2898 = Str::lit("debug");
    if equal(a2897, a2898) {
        Adt158::C1(Adt20::C0)
    } else {
        let a2899 = v0.clone();
        let a2900 = Str::lit("info");
        if equal(a2899, a2900) {
            Adt158::C1(Adt20::C1)
        } else {
            let a2901 = v0.clone();
            let a2902 = Str::lit("notice");
            if equal(a2901, a2902) {
                Adt158::C1(Adt20::C2)
            } else {
                let a2903 = v0.clone();
                let a2904 = Str::lit("warning");
                if equal(a2903, a2904) {
                    Adt158::C1(Adt20::C3)
                } else {
                    let a2905 = v0.clone();
                    let a2906 = Str::lit("error");
                    if equal(a2905, a2906) {
                        Adt158::C1(Adt20::C4)
                    } else {
                        let a2907 = v0.clone();
                        let a2908 = Str::lit("critical");
                        if equal(a2907, a2908) {
                            Adt158::C1(Adt20::C5)
                        } else {
                            let a2909 = v0.clone();
                            let a2910 = Str::lit("alert");
                            if equal(a2909, a2910) {
                                Adt158::C1(Adt20::C6)
                            } else {
                                let a2911 = v0;
                                let a2912 = Str::lit("emergency");
                                if equal(a2911, a2912) {
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

pub fn f761() -> Str {
    Str::lit("progressToken")
}

pub fn f762(v0: Option<List<Adt43>>) -> List<(Str, Adt6)> {
    let m2913 = v0;
    match m2913 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("audience"), Adt6::C4(f786(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f763(v0: Option<Adt7>) -> List<(Str, Adt6)> {
    let m2914 = v0;
    match m2914 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("priority"), Adt6::C2(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f764(p2917: List<Adt59>, p2918: List<Adt6>) -> List<Adt6> {
    let m2916 = (p2917, p2918);
    {
        let mut m2916 = m2916;
        loop {
            let step = {
                let (v0, v1) = m2916;
                let m2915 = v0;
                match m2915.uncons() {
                    None => {
                        Err::<(List<Adt59>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt59>, List<Adt6>), List<Adt6>>((v3, List::cons(f787(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2916 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f765(p2922: List<Adt72>, p2923: List<Adt6>) -> R<Option<List<Adt6>>> {
    let m2921 = (p2922, p2923);
    {
        let mut m2921 = m2921;
        loop {
            let step = {
                let (v0, v1) = m2921;
                let m2919 = v0;
                match m2919.uncons() {
                    None => {
                        Err::<(List<Adt72>, List<Adt6>), R<Option<List<Adt6>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m2920 = f788(v2)?;
                        match m2920 {
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
                Ok(next) => { m2921 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f766(v0: Adt77) -> Adt6 {
    Adt6::C5({
        let a2933 = {
            let m2925 = match v0.clone() {
                Adt77::C0(h2924, _, _, _) => {
                    h2924
                }
            };
            match m2925 {
                None => {
                    List::<(Str, Adt6)>::nil()
                }
                Some(v1) => {
                    List::cons((Str::lit("hints"), Adt6::C4(f431(f789(v1, List::<Adt6>::nil()), List::<Adt6>::nil()))), List::<(Str, Adt6)>::nil())
                }
            }
        };
        let a2934 = {
            let a2931 = f768(Str::lit("costPriority"), match v0.clone() {
                Adt77::C0(_, h2926, _, _) => {
                    h2926
                }
            });
            let a2932 = {
                let a2929 = f768(Str::lit("speedPriority"), match v0.clone() {
                    Adt77::C0(_, _, h2927, _) => {
                        h2927
                    }
                });
                let a2930 = f768(Str::lit("intelligencePriority"), match v0 {
                    Adt77::C0(_, _, _, h2928) => {
                        h2928
                    }
                });
                append_list(a2929, a2930)
            };
            append_list(a2931, a2932)
        };
        append_list(a2933, a2934)
    })
}

pub fn f767(v0: Adt79) -> Str {
    let m2935 = v0;
    match m2935 {
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

pub fn f768(v0: Str, v1: Option<Adt7>) -> List<(Str, Adt6)> {
    let m2936 = v1;
    match m2936 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C2(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f769(v0: Adt80) -> List<(Str, Adt6)> {
    let m2938 = match v0 {
        Adt80::C0(h2937) => {
            h2937
        }
    };
    match m2938 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("mode"), Adt6::C3(f790(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f770(v0: Adt2) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("src"), Adt6::C3(match v0.clone() {
        Adt2::C0(h2939, _, _, _) => {
            h2939
        }
    })), {
        let a2945 = f603(Str::lit("mimeType"), match v0.clone() {
            Adt2::C0(_, h2940, _, _) => {
                h2940
            }
        });
        let a2946 = {
            let a2943 = f791(match v0.clone() {
                Adt2::C0(_, _, h2941, _) => {
                    h2941
                }
            });
            let a2944 = f792(match v0 {
                Adt2::C0(_, _, _, h2942) => {
                    h2942
                }
            });
            append_list(a2943, a2944)
        };
        append_list(a2945, a2946)
    }))
}

pub fn f771(v0: u64, v1: List<u64>) -> List<u64> {
    let a2947 = v0;
    let a2948 = 0u64;
    if nat_eq(a2947, a2948) {
        v1
    } else {
        let a2949 = v0;
        let a2950 = 1u64;
        if nat_eq(a2949, a2950) {
            List::cons(194u64, v1)
        } else {
            let a2951 = v0;
            let a2952 = 2u64;
            if nat_eq(a2951, a2952) {
                List::cons(226u64, v1)
            } else {
                let a2953 = v0;
                let a2954 = 3u64;
                if nat_eq(a2953, a2954) {
                    List::cons(128u64, List::cons(226u64, v1))
                } else {
                    List::cons(129u64, List::cons(226u64, v1))
                }
            }
        }
    }
}

pub fn f772(v0: u64, v1: u64, v2: List<u64>) -> Adt157 {
    let a2955 = v0;
    let a2956 = 0u64;
    if nat_eq(a2955, a2956) {
        f793(v1, v2)
    } else {
        let a2957 = v0;
        let a2958 = 1u64;
        if nat_eq(a2957, a2958) {
            let a2959 = 128u64;
            let a2960 = v1;
            if if nat_le(a2959, a2960) {
                let a2961 = v1;
                let a2962 = 159u64;
                nat_le(a2961, a2962)
            } else {
                false
            } {
                Adt157::C0(0u64, f794(v2))
            } else {
                f793(v1, List::cons(194u64, v2))
            }
        } else {
            let a2963 = v0;
            let a2964 = 2u64;
            if nat_eq(a2963, a2964) {
                let a2965 = v1;
                let a2966 = 128u64;
                if nat_eq(a2965, a2966) {
                    Adt157::C0(3u64, v2)
                } else {
                    let a2967 = v1;
                    let a2968 = 129u64;
                    if nat_eq(a2967, a2968) {
                        Adt157::C0(4u64, v2)
                    } else {
                        f793(v1, List::cons(226u64, v2))
                    }
                }
            } else {
                let a2969 = v0;
                let a2970 = 3u64;
                if nat_eq(a2969, a2970) {
                    let a2971 = 170u64;
                    let a2972 = v1;
                    if if nat_le(a2971, a2972) {
                        let a2973 = v1;
                        let a2974 = 174u64;
                        nat_le(a2973, a2974)
                    } else {
                        false
                    } {
                        Adt157::C0(0u64, f794(v2))
                    } else {
                        f793(v1, List::cons(128u64, List::cons(226u64, v2)))
                    }
                } else {
                    let a2975 = 166u64;
                    let a2976 = v1;
                    if if nat_le(a2975, a2976) {
                        let a2977 = v1;
                        let a2978 = 169u64;
                        nat_le(a2977, a2978)
                    } else {
                        false
                    } {
                        Adt157::C0(0u64, f794(v2))
                    } else {
                        f793(v1, List::cons(129u64, List::cons(226u64, v2)))
                    }
                }
            }
        }
    }
}

pub fn f773(v0: Adt0, v1: Adt83, v2: bool) -> Result<Adt137, Adt140> {
    if v2 {
        Ok::<Adt137, Adt140>(f679(v0, Adt34::C12(v1)))
    } else {
        Err::<Adt137, Adt140>(Adt140::C4(Adt86::C8(f795(f616(v1)))))
    }
}

pub fn f774(v0: Adt19, v1: List<(Str, Adt66)>) -> bool {
    let a2979 = f796(v1.clone());
    if if if bool_not(a2979) {
        true
    } else {
        f585(match v0.clone() {
            Adt19::C0(_, _, _, h2980, _) => {
                h2980
            }
        })
    } {
        let a2981 = f797(v1.clone());
        if bool_not(a2981) {
            true
        } else {
            f585(match v0.clone() {
                Adt19::C0(_, _, h2982, _, _) => {
                    h2982
                }
            })
        }
    } else {
        false
    } {
        let a2983 = f798(v1);
        if bool_not(a2983) {
            true
        } else {
            f585(match v0 {
                Adt19::C0(_, h2984, _, _, _) => {
                    h2984
                }
            })
        }
    } else {
        false
    }
}

pub fn f775(v0: Adt0, v1: Adt84, v2: bool) -> Result<Adt137, Adt140> {
    if v2 {
        Ok::<Adt137, Adt140>(f679(v0, Adt34::C13(v1)))
    } else {
        Err::<Adt137, Adt140>(Adt140::C4(Adt86::C8(f799())))
    }
}

pub fn f776(v0: Adt19) -> bool {
    f586(match v0 {
        Adt19::C0(_, _, _, _, h2985) => {
            h2985
        }
    }, f587())
}

pub fn f777(v0: Adt18, v1: List<Adt52>) -> List<Adt139> {
    let m2987 = match v0 {
        Adt18::C0(_, _, _, _, h2986, _) => {
            h2986
        }
    };
    match m2987 {
        None => {
            List::<Adt139>::nil()
        }
        Some(v2) => {
            f800(f801(v2, v1, List::<Adt139>::nil()), List::<Adt139>::nil())
        }
    }
}

pub fn f778(v0: Bytes, v1: Adt148, v2: Str, v3: Adt34) -> R<Result<Adt34, Adt86>> {
    let m2988 = v3.clone();
    match m2988 {
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
            f802(v0, v1, v2, v4)
        }
        Adt34::C13(_) => {
            Ok(Ok::<Adt34, Adt86>(v3))
        }
    }
}

pub fn f779(v0: Adt16) -> Str {
    let m2989 = v0;
    match m2989 {
        Adt16::C0(_) => {
            Str::lit("")
        }
        Adt16::C1(_) => {
            Str::lit("")
        }
        Adt16::C2(v1) => {
            match v1 {
                Adt23::C0(_, h2990, _, _, _) => {
                    h2990
                }
            }
        }
        Adt16::C3(_) => {
            Str::lit("")
        }
        Adt16::C4(v2) => {
            match v2 {
                Adt24::C0(_, h2991, _, _) => {
                    h2991
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
                Adt25::C0(_, h2992, _, _, _) => {
                    h2992
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

pub fn f780(v0: u64, v1: u64) -> bool {
    let a2993 = v1;
    let a2994 = v0;
    nat_lt(a2993, a2994)
}

pub fn f781(v0: Bytes, v1: Str, v2: Str, v3: Option<Str>, v4: Option<Adt5>) -> R<Result<Option<Str>, Adt86>> {
    Ok(f803(f804(v0, v1, v2, v3)?, f805(v4)))
}

pub fn f782(p3000: List<Str>, p3001: Adt5) -> Adt108 {
    let m2999 = (p3000, p3001);
    {
        let mut m2999 = m2999;
        loop {
            let step = {
                let (v0, v1) = m2999;
                let m2995 = v0;
                match m2995.uncons() {
                    None => {
                        Err::<(List<Str>, Adt5), Adt108>(Adt108::C0)
                    }
                    Some((v2, v3)) => {
                        let m2996 = f138(v2, v1);
                        match m2996 {
                            Adt108::C0 => {
                                Err::<(List<Str>, Adt5), Adt108>(Adt108::C0)
                            }
                            Adt108::C1(v4) => {
                                let m2997 = v3.clone();
                                match m2997.uncons() {
                                    None => {
                                        Err::<(List<Str>, Adt5), Adt108>(Adt108::C1(v4))
                                    }
                                    Some((_, _)) => {
                                        let m2998 = f218(v4);
                                        match m2998 {
                                            None => {
                                                Err::<(List<Str>, Adt5), Adt108>(Adt108::C0)
                                            }
                                            Some(v7) => {
                                                Ok::<(List<Str>, Adt5), Adt108>((v3, v7))
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
                Ok(next) => { m2999 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f783(v0: Adt130) -> Option<Adt128> {
    let m3003 = match v0.clone() {
        Adt130::C0(_, _, _, h3002) => {
            h3002
        }
    };
    match m3003 {
        Adt131::C0 => {
            None::<Adt128>
        }
        Adt131::C1 => {
            Some(Adt128::C10(match v0 {
                Adt130::C0(h3004, _, _, _) => {
                    h3004
                }
            }))
        }
        Adt131::C2(_) => {
            None::<Adt128>
        }
    }
}

pub fn f784(v0: Adt6) -> bool {
    let m3005 = v0;
    match m3005 {
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

pub fn f785(v0: Adt130, v1: Adt6) -> R<Option<Adt128>> {
    let m3007 = match v0.clone() {
        Adt130::C0(_, _, _, h3006) => {
            h3006
        }
    };
    match m3007 {
        Adt131::C0 => {
            Ok(Some(Adt128::C8(match v0 {
                Adt130::C0(h3008, _, _, _) => {
                    h3008
                }
            })))
        }
        Adt131::C1 => {
            Ok(Some(Adt128::C10(match v0 {
                Adt130::C0(h3009, _, _, _) => {
                    h3009
                }
            })))
        }
        Adt131::C2(v2) => {
            if f806(match v0.clone() {
                Adt130::C0(_, _, h3010, _) => {
                    h3010
                }
            }, v2, v1)? {
                Ok(None::<Adt128>)
            } else {
                Ok(Some(Adt128::C9(match v0 {
                    Adt130::C0(h3011, _, _, _) => {
                        h3011
                    }
                })))
            }
        }
    }
}

pub fn f786(v0: List<Adt43>) -> List<Adt6> {
    f431(f807(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f787(v0: Adt59) -> Adt6 {
    Adt6::C5({
        let a3020 = List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt59::C0(h3012, _, _, _) => {
                h3012
            }
        })), List::<(Str, Adt6)>::nil());
        let a3021 = {
            let a3018 = f603(Str::lit("title"), match v0.clone() {
                Adt59::C0(_, h3013, _, _) => {
                    h3013
                }
            });
            let a3019 = {
                let a3016 = f603(Str::lit("description"), match v0.clone() {
                    Adt59::C0(_, _, h3014, _) => {
                        h3014
                    }
                });
                let a3017 = f608(Str::lit("required"), match v0 {
                    Adt59::C0(_, _, _, h3015) => {
                        h3015
                    }
                });
                append_list(a3016, a3017)
            };
            append_list(a3018, a3019)
        };
        append_list(a3020, a3021)
    })
}

pub fn f788(v0: Adt72) -> R<Option<Adt6>> {
    let m3023 = f808(match v0.clone() {
        Adt72::C0(_, h3022, _) => {
            h3022
        }
    })?;
    match m3023 {
        None => {
            Ok(None::<Adt6>)
        }
        Some(v1) => {
            Ok(Some(Adt6::C5(List::cons((Str::lit("role"), Adt6::C3(f734(match v0.clone() {
                Adt72::C0(h3024, _, _) => {
                    h3024
                }
            }))), List::cons((Str::lit("content"), v1), f726(match v0 {
                Adt72::C0(_, _, h3025) => {
                    h3025
                }
            }))))))
        }
    }
}

pub fn f789(p3029: List<Adt78>, p3030: List<Adt6>) -> List<Adt6> {
    let m3028 = (p3029, p3030);
    {
        let mut m3028 = m3028;
        loop {
            let step = {
                let (v0, v1) = m3028;
                let m3026 = v0;
                match m3026.uncons() {
                    None => {
                        Err::<(List<Adt78>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt78>, List<Adt6>), List<Adt6>>((v3, List::cons(Adt6::C5(f603(Str::lit("name"), match v2 {
                            Adt78::C0(h3027) => {
                                h3027
                            }
                        })), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m3028 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f790(v0: Adt81) -> Str {
    let m3031 = v0;
    match m3031 {
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

pub fn f791(v0: Option<List<Str>>) -> List<(Str, Adt6)> {
    let m3032 = v0;
    match m3032 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("sizes"), Adt6::C4(f577(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f792(v0: Option<Adt3>) -> List<(Str, Adt6)> {
    let m3033 = v0;
    match m3033 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("theme"), Adt6::C3(f809(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f793(v0: u64, v1: List<u64>) -> Adt157 {
    let a3034 = v0;
    let a3035 = 32u64;
    if nat_lt(a3034, a3035) {
        if f810(v0) {
            Adt157::C0(0u64, List::cons(v0, v1))
        } else {
            Adt157::C0(0u64, f794(v1))
        }
    } else {
        let a3036 = v0;
        let a3037 = 127u64;
        if nat_eq(a3036, a3037) {
            Adt157::C0(0u64, f794(v1))
        } else {
            let a3038 = v0;
            let a3039 = 194u64;
            if nat_eq(a3038, a3039) {
                Adt157::C0(1u64, v1)
            } else {
                let a3040 = v0;
                let a3041 = 226u64;
                if nat_eq(a3040, a3041) {
                    Adt157::C0(2u64, v1)
                } else {
                    Adt157::C0(0u64, List::cons(v0, v1))
                }
            }
        }
    }
}

pub fn f794(v0: List<u64>) -> List<u64> {
    List::cons(189u64, List::cons(191u64, List::cons(239u64, v0)))
}

pub fn f795(v0: List<(Str, Adt66)>) -> Adt19 {
    Adt19::C0(None::<Adt5>, if f798(v0.clone()) {
        Some(f652())
    } else {
        None::<Adt5>
    }, if f797(v0.clone()) {
        Some(f652())
    } else {
        None::<Adt5>
    }, if f796(v0) {
        Some(f652())
    } else {
        None::<Adt5>
    }, None::<Adt5>)
}

pub fn f796(p3046: List<(Str, Adt66)>) -> bool {
    let m3045 = p3046;
    {
        let mut m3045 = m3045;
        loop {
            let step = {
                let v0 = m3045;
                let m3042 = v0;
                match m3042.uncons() {
                    None => {
                        Err::<List<(Str, Adt66)>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        let m3044 = {
                            let (_, h3043) = v1;
                            h3043
                        };
                        match m3044 {
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
                Ok(next) => { m3045 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f797(p3051: List<(Str, Adt66)>) -> bool {
    let m3050 = p3051;
    {
        let mut m3050 = m3050;
        loop {
            let step = {
                let v0 = m3050;
                let m3047 = v0;
                match m3047.uncons() {
                    None => {
                        Err::<List<(Str, Adt66)>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        let m3049 = {
                            let (_, h3048) = v1;
                            h3048
                        };
                        match m3049 {
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
                Ok(next) => { m3050 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f798(p3056: List<(Str, Adt66)>) -> bool {
    let m3055 = p3056;
    {
        let mut m3055 = m3055;
        loop {
            let step = {
                let v0 = m3055;
                let m3052 = v0;
                match m3052.uncons() {
                    None => {
                        Err::<List<(Str, Adt66)>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        let m3054 = {
                            let (_, h3053) = v1;
                            h3053
                        };
                        match m3054 {
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
                Ok(next) => { m3055 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f799() -> Adt19 {
    Adt19::C0(None::<Adt5>, None::<Adt5>, None::<Adt5>, None::<Adt5>, Some(Adt5::C0(List::cons((f587(), Adt6::C5(List::<(Str, Adt6)>::nil())), List::<(Str, Adt6)>::nil()))))
}

pub fn f800(p3059: List<Adt139>, p3060: List<Adt139>) -> List<Adt139> {
    let m3058 = (p3059, p3060);
    {
        let mut m3058 = m3058;
        loop {
            let step = {
                let (v0, v1) = m3058;
                let m3057 = v0;
                match m3057.uncons() {
                    None => {
                        Err::<(List<Adt139>, List<Adt139>), List<Adt139>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt139>, List<Adt139>), List<Adt139>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m3058 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f801(v0: Adt21, p3063: List<Adt52>, p3064: List<Adt139>) -> List<Adt139> {
    let m3062 = (p3063, p3064);
    {
        let mut m3062 = m3062;
        loop {
            let step = {
                let (v1, v2) = m3062;
                let m3061 = v1;
                match m3061.uncons() {
                    None => {
                        Err::<(List<Adt52>, List<Adt139>), List<Adt139>>(v2)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt52>, List<Adt139>), List<Adt139>>((v4, List::cons(Adt139::C0(v0.clone(), v3), v2)))
                    }
                }
            };
            match step {
                Ok(next) => { m3062 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f802(v0: Bytes, v1: Adt148, v2: Str, v3: Adt83) -> R<Result<Adt34, Adt86>> {
    if f744(v1.clone()) {
        if f811(v3.clone()) {
            let m3065 = f812(v0, f582(v1), v2, v3)?;
            match m3065 {
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
        Ok(Err::<Adt34, Adt86>(Adt86::C12(f582(v1))))
    }
}

pub fn f803(v0: Result<Option<Str>, Adt86>, v1: bool) -> Result<Option<Str>, Adt86> {
    let m3066 = v0;
    match m3066 {
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

pub fn f804(v0: Bytes, v1: Str, v2: Str, v3: Option<Str>) -> R<Result<Option<Str>, Adt86>> {
    let m3067 = v3;
    match m3067 {
        None => {
            Ok(Ok::<Option<Str>, Adt86>(None::<Str>))
        }
        Some(v4) => {
            Ok(f813(f814(v0, v1, v2, v4)?))
        }
    }
}

pub fn f805(v0: Option<Adt5>) -> bool {
    let m3068 = v0;
    match m3068 {
        None => {
            true
        }
        Some(v1) => {
            f815(match v1 {
                Adt5::C0(h3069) => {
                    h3069
                }
            })
        }
    }
}

pub fn f806(v0: Adt13, v1: Str, v2: Adt6) -> R<bool> {
    let m3070 = v0;
    match m3070 {
        Adt13::C0 => {
            let m3071 = f89(v2);
            match m3071 {
                None => {
                    Ok(false)
                }
                Some(v3) => {
                    let a3072 = v1;
                    let a3073 = v3;
                    Ok(equal(a3072, a3073))
                }
            }
        }
        Adt13::C1 => {
            f816(v1, v2)
        }
        Adt13::C2 => {
            let m3074 = f160(v2);
            match m3074 {
                None => {
                    Ok(false)
                }
                Some(v4) => {
                    if v4 {
                        let a3075 = v1;
                        let a3076 = Str::lit("true");
                        Ok(equal(a3075, a3076))
                    } else {
                        let a3077 = v1;
                        let a3078 = Str::lit("false");
                        Ok(equal(a3077, a3078))
                    }
                }
            }
        }
    }
}

pub fn f807(p3081: List<Adt43>, p3082: List<Adt6>) -> List<Adt6> {
    let m3080 = (p3081, p3082);
    {
        let mut m3080 = m3080;
        loop {
            let step = {
                let (v0, v1) = m3080;
                let m3079 = v0;
                match m3079.uncons() {
                    None => {
                        Err::<(List<Adt43>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt43>, List<Adt6>), List<Adt6>>((v3, List::cons(Adt6::C3(f734(v2)), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m3080 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f808(v0: Adt73) -> R<Option<Adt6>> {
    let m3083 = v0;
    match m3083 {
        Adt73::C0(v1) => {
            f817(v1)
        }
        Adt73::C1(v2) => {
            let m3084 = f818(v2, List::<Adt6>::nil())?;
            match m3084 {
                None => {
                    Ok(None::<Adt6>)
                }
                Some(v3) => {
                    Ok(Some(Adt6::C4(f431(v3, List::<Adt6>::nil()))))
                }
            }
        }
    }
}

pub fn f809(v0: Adt3) -> Str {
    let m3085 = v0;
    match m3085 {
        Adt3::C0 => {
            Str::lit("light")
        }
        Adt3::C1 => {
            Str::lit("dark")
        }
    }
}

pub fn f810(v0: u64) -> bool {
    let a3086 = v0;
    let a3087 = 9u64;
    if nat_eq(a3086, a3087) {
        true
    } else {
        let a3088 = v0;
        let a3089 = 10u64;
        if nat_eq(a3088, a3089) {
            true
        } else {
            let a3090 = v0;
            let a3091 = 13u64;
            nat_eq(a3090, a3091)
        }
    }
}

pub fn f811(v0: Adt83) -> bool {
    let m3092 = v0;
    match m3092 {
        Adt83::C0(v1) => {
            let m3093 = v1;
            match m3093.uncons() {
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

pub fn f812(v0: Bytes, v1: Str, v2: Str, v3: Adt83) -> R<Result<Adt83, Adt86>> {
    let m3094 = v3.clone();
    match m3094 {
        Adt83::C0(_) => {
            Ok(Ok::<Adt83, Adt86>(v3))
        }
        Adt83::C1(v5) => {
            Ok(f819(f820(v0, v1, v2, v5)?, List::<(Str, Adt66)>::nil()))
        }
        Adt83::C2(v6, v7) => {
            Ok(f819(f820(v0, v1, v2, v7)?, v6))
        }
    }
}

pub fn f813(v0: Option<Str>) -> Result<Option<Str>, Adt86> {
    let m3095 = v0;
    match m3095 {
        None => {
            Err::<Option<Str>, Adt86>(Adt86::C10)
        }
        Some(v1) => {
            Ok::<Option<Str>, Adt86>(Some(v1))
        }
    }
}

pub fn f814(v0: Bytes, v1: Str, v2: Str, v3: Str) -> R<Option<Str>> {
    let m3099 = f821(v0, {
        let a3096 = v1;
        utf8_encode(a3096)
    }, {
        let a3097 = v2;
        utf8_encode(a3097)
    }, {
        let a3098 = v3;
        utf8_encode(a3098)
    })?;
    match m3099 {
        None => {
            Ok(None::<Str>)
        }
        Some(v4) => {
            let a3100 = v4;
            Ok(utf8_decode(a3100))
        }
    }
}

pub fn f815(p3105: List<(Str, Adt6)>) -> bool {
    let m3104 = p3105;
    {
        let mut m3104 = m3104;
        loop {
            let step = {
                let v0 = m3104;
                let m3101 = v0;
                match m3101.uncons() {
                    None => {
                        Err::<List<(Str, Adt6)>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let m3103 = f218({
                            let (_, h3102) = v1;
                            h3102
                        });
                        match m3103 {
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
                Ok(next) => { m3104 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f816(v0: Str, v1: Adt6) -> R<bool> {
    let m3106 = f363(v1);
    match m3106 {
        None => {
            Ok(false)
        }
        Some(v2) => {
            let m3107 = f388(v2)?;
            match m3107 {
                None => {
                    Ok(false)
                }
                Some(v3) => {
                    let m3109 = f822(f18({
                        let a3108 = v0;
                        utf8_encode(a3108)
                    })?)?;
                    match m3109 {
                        None => {
                            Ok(false)
                        }
                        Some(v4) => {
                            Ok(f524({
                                let a3110 = v3;
                                let a3111 = v4;
                                int_sub(a3110, a3111)?
                            }))
                        }
                    }
                }
            }
        }
    }
}

pub fn f817(v0: Adt74) -> R<Option<Adt6>> {
    let m3112 = v0;
    match m3112 {
        Adt74::C0(v1) => {
            Ok(Some(f727(v1)))
        }
        Adt74::C1(v2) => {
            Ok(Some(f728(Str::lit("image"), match v2.clone() {
                Adt44::C0(h3113, _, _, _) => {
                    h3113
                }
            }, match v2.clone() {
                Adt44::C0(_, h3114, _, _) => {
                    h3114
                }
            }, match v2.clone() {
                Adt44::C0(_, _, h3115, _) => {
                    h3115
                }
            }, match v2 {
                Adt44::C0(_, _, _, h3116) => {
                    h3116
                }
            })))
        }
        Adt74::C2(v3) => {
            Ok(Some(f728(Str::lit("audio"), match v3.clone() {
                Adt45::C0(h3117, _, _, _) => {
                    h3117
                }
            }, match v3.clone() {
                Adt45::C0(_, h3118, _, _) => {
                    h3118
                }
            }, match v3.clone() {
                Adt45::C0(_, _, h3119, _) => {
                    h3119
                }
            }, match v3 {
                Adt45::C0(_, _, _, h3120) => {
                    h3120
                }
            })))
        }
        Adt74::C3(v4) => {
            Ok(Some(f823(v4)))
        }
        Adt74::C4(v5) => {
            f824(v5)
        }
    }
}

pub fn f818(p3124: List<Adt74>, p3125: List<Adt6>) -> R<Option<List<Adt6>>> {
    let m3123 = (p3124, p3125);
    {
        let mut m3123 = m3123;
        loop {
            let step = {
                let (v0, v1) = m3123;
                let m3121 = v0;
                match m3121.uncons() {
                    None => {
                        Err::<(List<Adt74>, List<Adt6>), R<Option<List<Adt6>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m3122 = f817(v2)?;
                        match m3122 {
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
                Ok(next) => { m3123 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f819(v0: Option<Str>, v1: List<(Str, Adt66)>) -> Result<Adt83, Adt86> {
    let m3126 = v0;
    match m3126 {
        None => {
            Err::<Adt83, Adt86>(Adt86::C14)
        }
        Some(v2) => {
            let m3127 = v1.clone();
            match m3127.uncons() {
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

pub fn f820(v0: Bytes, v1: Str, v2: Str, v3: Str) -> R<Option<Str>> {
    let m3131 = f825(v0, {
        let a3128 = v1;
        utf8_encode(a3128)
    }, {
        let a3129 = v2;
        utf8_encode(a3129)
    }, {
        let a3130 = v3;
        utf8_encode(a3130)
    })?;
    match m3131 {
        None => {
            Ok(None::<Str>)
        }
        Some(v4) => {
            let a3132 = v4;
            Ok(utf8_decode(a3132))
        }
    }
}

pub fn f821(v0: Bytes, v1: Bytes, v2: Bytes, v3: Bytes) -> R<Option<Bytes>> {
    let a3134 = f826();
    let a3135 = {
        let a3133 = v0.clone();
        length_bytes(a3133)
    };
    if nat_le(a3134, a3135) {
        let a3137 = f827(v3.clone())?;
        let a3138 = {
            let a3136 = v3.clone();
            length_bytes(a3136)
        };
        if nat_eq(a3137, a3138) {
            Ok(None::<Bytes>)
        } else {
            f828(v0, v1, v2, v3.clone(), f827(v3)?)
        }
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f822(v0: List<u64>) -> R<Option<i64>> {
    let m3139 = v0.clone();
    match m3139.uncons() {
        None => {
            Ok(None::<i64>)
        }
        Some((v1, v2)) => {
            let a3140 = v1;
            let a3141 = 45u64;
            if nat_eq(a3140, a3141) {
                let m3142 = v2.clone();
                match m3142.uncons() {
                    None => {
                        Ok(None::<i64>)
                    }
                    Some((_, _)) => {
                        let m3143 = f829(v2, 0i64)?;
                        match m3143 {
                            None => {
                                Ok(None::<i64>)
                            }
                            Some(v5) => {
                                Ok(Some({
                                    let a3144 = v5;
                                    int_neg(a3144)?
                                }))
                            }
                        }
                    }
                }
            } else {
                f829(v0, 0i64)
            }
        }
    }
}

pub fn f823(v0: Adt75) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("tool_use"))), List::cons((Str::lit("id"), Adt6::C3(match v0.clone() {
        Adt75::C0(h3145, _, _, _) => {
            h3145
        }
    })), List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
        Adt75::C0(_, h3146, _, _) => {
            h3146
        }
    })), List::cons((Str::lit("input"), Adt6::C5({
        let m3149 = match v0.clone() {
            Adt75::C0(_, _, h3148, _) => {
                h3148
            }
        };
        match m3149 {
            Adt5::C0(h3147) => {
                h3147
            }
        }
    })), f726(match v0 {
        Adt75::C0(_, _, _, h3150) => {
            h3150
        }
    }))))))
}

pub fn f824(v0: Adt76) -> R<Option<Adt6>> {
    let m3152 = f606(match v0.clone() {
        Adt76::C0(_, h3151, _, _, _) => {
            h3151
        }
    })?;
    match m3152 {
        None => {
            Ok(None::<Adt6>)
        }
        Some(v1) => {
            Ok(Some(Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("tool_result"))), List::cons((Str::lit("toolUseId"), Adt6::C3(match v0.clone() {
                Adt76::C0(h3153, _, _, _, _) => {
                    h3153
                }
            })), List::cons((Str::lit("content"), Adt6::C4(v1)), {
                let a3159 = f607(Str::lit("structuredContent"), match v0.clone() {
                    Adt76::C0(_, _, h3154, _, _) => {
                        h3154
                    }
                });
                let a3160 = {
                    let a3157 = f608(Str::lit("isError"), match v0.clone() {
                        Adt76::C0(_, _, _, h3155, _) => {
                            h3155
                        }
                    });
                    let a3158 = f726(match v0 {
                        Adt76::C0(_, _, _, _, h3156) => {
                            h3156
                        }
                    });
                    append_list(a3157, a3158)
                };
                append_list(a3159, a3160)
            }))))))
        }
    }
}

pub fn f825(v0: Bytes, v1: Bytes, v2: Bytes, v3: Bytes) -> R<Option<Bytes>> {
    let a3162 = f826();
    let a3163 = {
        let a3161 = v0.clone();
        length_bytes(a3161)
    };
    if nat_le(a3162, a3163) {
        Ok(Some({
            let a3166 = f830(v3.clone())?;
            let a3167 = {
                let a3164 = Bytes::lit(&[0x2e]);
                let a3165 = f830(f831(v0, v1, v2, v3)?)?;
                append_bytes(a3164, a3165)
            };
            append_bytes(a3166, a3167)
        }))
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f826() -> u64 {
    32u64
}

pub fn f827(v0: Bytes) -> R<u64> {
    let (_, h3168) = f832(Fn4::F833(v0.clone()), {
        let a3169 = v0.clone();
        length_bytes(a3169)
    }, (0u64, {
        let a3170 = v0;
        length_bytes(a3170)
    }))?;
    Ok(h3168)
}

pub fn f828(v0: Bytes, v1: Bytes, v2: Bytes, v3: Bytes, v4: u64) -> R<Option<Bytes>> {
    let m3174 = {
        let a3171 = v3.clone();
        let a3172 = 0u64;
        let a3173 = v4;
        slice_bytes(a3171, a3172, a3173)
    };
    match m3174 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v5) => {
            let m3185 = {
                let a3182 = v3.clone();
                let a3183 = {
                    let a3175 = v4;
                    let a3176 = 1u64;
                    nat_add(a3175, a3176)?
                };
                let a3184 = {
                    let a3180 = {
                        let a3177 = v3;
                        length_bytes(a3177)
                    };
                    let a3181 = {
                        let a3178 = v4;
                        let a3179 = 1u64;
                        nat_add(a3178, a3179)?
                    };
                    nat_sub(a3180, a3181)
                };
                slice_bytes(a3182, a3183, a3184)
            };
            match m3185 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v6) => {
                    f834(v0, v1, v2, f835(v5)?, f835(v6)?)
                }
            }
        }
    }
}

pub fn f829(p3191: List<u64>, p3192: i64) -> R<Option<i64>> {
    let m3190 = (p3191, p3192);
    {
        let mut m3190 = m3190;
        loop {
            let step = {
                let (v0, v1) = m3190;
                let m3186 = v0;
                match m3186.uncons() {
                    None => {
                        Err::<(List<u64>, i64), R<Option<i64>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m3187 = f836(v2);
                        match m3187 {
                            None => {
                                Err::<(List<u64>, i64), R<Option<i64>>>(Ok(None::<i64>))
                            }
                            Some(v4) => {
                                Ok::<(List<u64>, i64), R<Option<i64>>>((v3, f216({
                                    let a3188 = v1;
                                    let a3189 = 10i64;
                                    int_mul(a3188, a3189)?
                                }, f430(v4))?))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3190 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f830(v0: Bytes) -> R<Bytes> {
    let (_, h3193) = f837(Fn5::F838(v0.clone()), {
        let a3194 = v0;
        length_bytes(a3194)
    }, (0u64, f105()))?;
    Ok(h3193)
}

pub fn f831(v0: Bytes, v1: Bytes, v2: Bytes, v3: Bytes) -> R<Bytes> {
    f839(v0, {
        let a3195 = f840(v1, v2);
        let a3196 = v3;
        append_bytes(a3195, a3196)
    })
}

pub fn f832(v0: Fn4, p3200: u64, p3201: (u64, u64)) -> R<(u64, u64)> {
    let m3199 = (p3200, p3201);
    {
        let mut m3199 = m3199;
        loop {
            let step = {
                let (v1, v2) = m3199;
                let m3197 = v1;
                if m3197 == 0 {
                    Err::<(u64, (u64, u64)), R<(u64, u64)>>(Ok(v2))
                } else {
                    let v3 = m3197 - 1;
                    Ok::<(u64, (u64, u64)), R<(u64, u64)>>((v3, {
                        let c3198: Fn4 = v0.clone();
                        c3198.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m3199 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f833(v0: Bytes, v1: (u64, u64)) -> R<(u64, u64)> {
    Ok(({
        let a3203 = {
            let (h3202, _) = v1;
            h3202
        };
        let a3204 = 1u64;
        nat_add(a3203, a3204)?
    }, {
        let a3207 = {
            let (_, h3205) = v1;
            h3205
        };
        let a3208 = {
            let a3206 = v0.clone();
            length_bytes(a3206)
        };
        if nat_eq(a3207, a3208) {
            let a3210 = f67(v0, {
                let (h3209, _) = v1;
                h3209
            })?;
            let a3211 = 46u64;
            if nat_eq(a3210, a3211) {
                let (h3212, _) = v1;
                h3212
            } else {
                let (_, h3213) = v1;
                h3213
            }
        } else {
            let (_, h3214) = v1;
            h3214
        }
    }))
}

pub fn f834(v0: Bytes, v1: Bytes, v2: Bytes, v3: Option<Bytes>, v4: Option<Bytes>) -> R<Option<Bytes>> {
    let m3215 = v3;
    match m3215 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v5) => {
            let m3216 = v4;
            match m3216 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v6) => {
                    if f841(f831(v0, v1, v2, v5.clone())?, v6)? {
                        Ok(Some(v5))
                    } else {
                        Ok(None::<Bytes>)
                    }
                }
            }
        }
    }
}

pub fn f835(v0: Bytes) -> R<Option<Bytes>> {
    let a3221 = {
        let a3218 = {
            let a3217 = v0.clone();
            length_bytes(a3217)
        };
        let a3219 = 2u64;
        let a3220 = 0u64;
        nat_rem(a3218, a3219, a3220)
    };
    let a3222 = 0u64;
    if nat_eq(a3221, a3222) {
        let v1: (u64, (Bytes, u64)) = f842(Fn6::F843(v0.clone()), {
            let a3224 = {
                let a3223 = v0;
                length_bytes(a3223)
            };
            let a3225 = 2u64;
            let a3226 = 0u64;
            nat_quot(a3224, a3225, a3226)
        }, (0u64, (f105(), 0u64)))?;
        let a3229 = {
            let (_, h3227) = {
                let (_, h3228) = v1.clone();
                h3228
            };
            h3227
        };
        let a3230 = 0u64;
        if nat_eq(a3229, a3230) {
            Ok(Some({
                let (h3231, _) = {
                    let (_, h3232) = v1;
                    h3232
                };
                h3231
            }))
        } else {
            Ok(None::<Bytes>)
        }
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f836(v0: u64) -> Option<u64> {
    let a3233 = v0;
    let a3234 = 48u64;
    if if nat_lt(a3233, a3234) {
        true
    } else {
        let a3235 = 57u64;
        let a3236 = v0;
        nat_lt(a3235, a3236)
    } {
        None::<u64>
    } else {
        Some({
            let a3237 = v0;
            let a3238 = 48u64;
            nat_sub(a3237, a3238)
        })
    }
}

pub fn f837(v0: Fn5, p3242: u64, p3243: (u64, Bytes)) -> R<(u64, Bytes)> {
    let m3241 = (p3242, p3243);
    {
        let mut m3241 = m3241;
        loop {
            let step = {
                let (v1, v2) = m3241;
                let m3239 = v1;
                if m3239 == 0 {
                    Err::<(u64, (u64, Bytes)), R<(u64, Bytes)>>(Ok(v2))
                } else {
                    let v3 = m3239 - 1;
                    Ok::<(u64, (u64, Bytes)), R<(u64, Bytes)>>((v3, {
                        let c3240: Fn5 = v0.clone();
                        c3240.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m3241 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f838(v0: Bytes, v1: (u64, Bytes)) -> R<(u64, Bytes)> {
    Ok(({
        let a3245 = {
            let (h3244, _) = v1.clone();
            h3244
        };
        let a3246 = 1u64;
        nat_add(a3245, a3246)?
    }, {
        let a3249 = {
            let (_, h3247) = v1;
            h3247
        };
        let a3250 = f844(f67(v0, {
            let (h3248, _) = v1;
            h3248
        })?)?;
        append_bytes(a3249, a3250)
    }))
}

pub fn f839(v0: Bytes, v1: Bytes) -> R<Bytes> {
    f845(f846(v0, v1)?)
}

pub fn f840(v0: Bytes, v1: Bytes) -> Bytes {
    let a3257 = f847({
        let a3251 = v0.clone();
        length_bytes(a3251)
    });
    let a3258 = {
        let a3255 = v0;
        let a3256 = {
            let a3253 = f847({
                let a3252 = v1.clone();
                length_bytes(a3252)
            });
            let a3254 = v1;
            append_bytes(a3253, a3254)
        };
        append_bytes(a3255, a3256)
    };
    append_bytes(a3257, a3258)
}

pub fn f841(v0: Bytes, v1: Bytes) -> R<bool> {
    let a3261 = {
        let a3259 = v0.clone();
        length_bytes(a3259)
    };
    let a3262 = {
        let a3260 = v1.clone();
        length_bytes(a3260)
    };
    if nat_eq(a3261, a3262) {
        let a3263 = f848(v0, v1)?;
        let a3264 = 0u64;
        Ok(nat_eq(a3263, a3264))
    } else {
        Ok(false)
    }
}

pub fn f842(v0: Fn6, p3268: u64, p3269: (u64, (Bytes, u64))) -> R<(u64, (Bytes, u64))> {
    let m3267 = (p3268, p3269);
    {
        let mut m3267 = m3267;
        loop {
            let step = {
                let (v1, v2) = m3267;
                let m3265 = v1;
                if m3265 == 0 {
                    Err::<(u64, (u64, (Bytes, u64))), R<(u64, (Bytes, u64))>>(Ok(v2))
                } else {
                    let v3 = m3265 - 1;
                    Ok::<(u64, (u64, (Bytes, u64))), R<(u64, (Bytes, u64))>>((v3, {
                        let c3266: Fn6 = v0.clone();
                        c3266.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m3267 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f843(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<(u64, (Bytes, u64))> {
    f849(v0, v1)
}

pub fn f844(v0: u64) -> R<Bytes> {
    let a3276 = f850({
        let a3270 = v0;
        let a3271 = 16u64;
        let a3272 = 0u64;
        nat_quot(a3270, a3271, a3272)
    })?;
    let a3277 = f850({
        let a3273 = v0;
        let a3274 = 16u64;
        let a3275 = 0u64;
        nat_rem(a3273, a3274, a3275)
    })?;
    Ok(append_bytes(a3276, a3277))
}

pub fn f845(v0: Adt159) -> R<Bytes> {
    let a3298 = f851(match v0.clone() {
        Adt159::C0(h3278, _, _, _, _, _, _, _) => {
            h3278
        }
    })?;
    let a3299 = {
        let a3296 = f851(match v0.clone() {
            Adt159::C0(_, h3279, _, _, _, _, _, _) => {
                h3279
            }
        })?;
        let a3297 = {
            let a3294 = f851(match v0.clone() {
                Adt159::C0(_, _, h3280, _, _, _, _, _) => {
                    h3280
                }
            })?;
            let a3295 = {
                let a3292 = f851(match v0.clone() {
                    Adt159::C0(_, _, _, h3281, _, _, _, _) => {
                        h3281
                    }
                })?;
                let a3293 = {
                    let a3290 = f851(match v0.clone() {
                        Adt159::C0(_, _, _, _, h3282, _, _, _) => {
                            h3282
                        }
                    })?;
                    let a3291 = {
                        let a3288 = f851(match v0.clone() {
                            Adt159::C0(_, _, _, _, _, h3283, _, _) => {
                                h3283
                            }
                        })?;
                        let a3289 = {
                            let a3286 = f851(match v0.clone() {
                                Adt159::C0(_, _, _, _, _, _, h3284, _) => {
                                    h3284
                                }
                            })?;
                            let a3287 = f851(match v0 {
                                Adt159::C0(_, _, _, _, _, _, _, h3285) => {
                                    h3285
                                }
                            })?;
                            append_bytes(a3286, a3287)
                        };
                        append_bytes(a3288, a3289)
                    };
                    append_bytes(a3290, a3291)
                };
                append_bytes(a3292, a3293)
            };
            append_bytes(a3294, a3295)
        };
        append_bytes(a3296, a3297)
    };
    Ok(append_bytes(a3298, a3299))
}

pub fn f846(v0: Bytes, v1: Bytes) -> R<Adt159> {
    let v2: Adt160 = f852(v0)?;
    let v3: Adt159 = f853(f854(f855(), f856(v2.clone(), 909522486u32))?, 64u64, v1)?;
    f854(f854(f855(), f856(v2, 1549556828u32))?, f857(v3))
}

pub fn f847(v0: u64) -> Bytes {
    let a3325 = f214({
        let a3303 = {
            let a3300 = v0;
            let a3301 = 16777216u64;
            let a3302 = 0u64;
            nat_quot(a3300, a3301, a3302)
        };
        let a3304 = 256u64;
        let a3305 = 0u64;
        nat_rem(a3303, a3304, a3305)
    });
    let a3326 = {
        let a3323 = f214({
            let a3309 = {
                let a3306 = v0;
                let a3307 = 65536u64;
                let a3308 = 0u64;
                nat_quot(a3306, a3307, a3308)
            };
            let a3310 = 256u64;
            let a3311 = 0u64;
            nat_rem(a3309, a3310, a3311)
        });
        let a3324 = {
            let a3321 = f214({
                let a3315 = {
                    let a3312 = v0;
                    let a3313 = 256u64;
                    let a3314 = 0u64;
                    nat_quot(a3312, a3313, a3314)
                };
                let a3316 = 256u64;
                let a3317 = 0u64;
                nat_rem(a3315, a3316, a3317)
            });
            let a3322 = f214({
                let a3318 = v0;
                let a3319 = 256u64;
                let a3320 = 0u64;
                nat_rem(a3318, a3319, a3320)
            });
            append_bytes(a3321, a3322)
        };
        append_bytes(a3323, a3324)
    };
    append_bytes(a3325, a3326)
}

pub fn f848(v0: Bytes, v1: Bytes) -> R<u64> {
    let (_, h3327) = f832(Fn4::F858(v0.clone(), v1), {
        let a3328 = v0;
        length_bytes(a3328)
    }, (0u64, 0u64))?;
    Ok(h3327)
}

pub fn f849(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<(u64, (Bytes, u64))> {
    let v2: u64 = f859(f67(v0.clone(), {
        let (h3329, _) = v1.clone();
        h3329
    })?);
    let v3: u64 = f859(f67(v0, {
        let a3331 = {
            let (h3330, _) = v1.clone();
            h3330
        };
        let a3332 = 1u64;
        nat_add(a3331, a3332)?
    })?);
    let a3333 = v2;
    let a3334 = 16u64;
    if if nat_lt(a3333, a3334) {
        let a3335 = v3;
        let a3336 = 16u64;
        nat_lt(a3335, a3336)
    } else {
        false
    } {
        Ok(({
            let a3338 = {
                let (h3337, _) = v1.clone();
                h3337
            };
            let a3339 = 2u64;
            nat_add(a3338, a3339)?
        }, ({
            let a3346 = {
                let (h3340, _) = {
                    let (_, h3341) = v1.clone();
                    h3341
                };
                h3340
            };
            let a3347 = f214({
                let a3344 = {
                    let a3342 = v2;
                    let a3343 = 16u64;
                    nat_mul(a3342, a3343)?
                };
                let a3345 = v3;
                nat_add(a3344, a3345)?
            });
            append_bytes(a3346, a3347)
        }, {
            let (_, h3348) = {
                let (_, h3349) = v1;
                h3349
            };
            h3348
        })))
    } else {
        Ok(({
            let a3351 = {
                let (h3350, _) = v1.clone();
                h3350
            };
            let a3352 = 2u64;
            nat_add(a3351, a3352)?
        }, ({
            let (h3353, _) = {
                let (_, h3354) = v1.clone();
                h3354
            };
            h3353
        }, {
            let a3357 = {
                let (_, h3355) = {
                    let (_, h3356) = v1;
                    h3356
                };
                h3355
            };
            let a3358 = 1u64;
            nat_add(a3357, a3358)?
        })))
    }
}

pub fn f850(v0: u64) -> R<Bytes> {
    Ok(f214({
        let a3359 = v0;
        let a3360 = 10u64;
        if nat_lt(a3359, a3360) {
            let a3361 = 48u64;
            let a3362 = v0;
            nat_add(a3361, a3362)?
        } else {
            let a3363 = 87u64;
            let a3364 = v0;
            nat_add(a3363, a3364)?
        }
    }))
}

pub fn f851(v0: u32) -> R<Bytes> {
    let a3369 = f860(v0, 24u32)?;
    let a3370 = {
        let a3367 = f860(v0, 16u32)?;
        let a3368 = {
            let a3365 = f860(v0, 8u32)?;
            let a3366 = f860(v0, 0u32)?;
            append_bytes(a3365, a3366)
        };
        append_bytes(a3367, a3368)
    };
    Ok(append_bytes(a3369, a3370))
}

pub fn f852(v0: Bytes) -> R<Adt160> {
    let a3372 = f861();
    let a3373 = {
        let a3371 = v0.clone();
        length_bytes(a3371)
    };
    if nat_lt(a3372, a3373) {
        Ok(f862(f863(v0)?))
    } else {
        f864(v0)
    }
}

pub fn f853(v0: Adt159, v1: u64, v2: Bytes) -> R<Adt159> {
    let v3: u64 = {
        let a3374 = v2.clone();
        length_bytes(a3374)
    };
    let v4: u64 = {
        let a3380 = {
            let a3377 = {
                let a3375 = v3;
                let a3376 = 72u64;
                nat_add(a3375, a3376)?
            };
            let a3378 = 64u64;
            let a3379 = 0u64;
            nat_quot(a3377, a3378, a3379)
        };
        let a3381 = 64u64;
        nat_mul(a3380, a3381)?
    };
    let (_, h3382) = f865(Fn7::F866(v2, v1, v3, v4), {
        let a3383 = v4;
        let a3384 = 64u64;
        let a3385 = 0u64;
        nat_quot(a3383, a3384, a3385)
    }, (0u64, v0))?;
    Ok(h3382)
}

pub fn f854(v0: Adt159, v1: Adt160) -> R<Adt159> {
    let v2: Adt161 = f867(Fn8::F868, 64u64, Adt161::C0(match v0.clone() {
        Adt159::C0(h3386, _, _, _, _, _, _, _) => {
            h3386
        }
    }, match v0.clone() {
        Adt159::C0(_, h3387, _, _, _, _, _, _) => {
            h3387
        }
    }, match v0.clone() {
        Adt159::C0(_, _, h3388, _, _, _, _, _) => {
            h3388
        }
    }, match v0.clone() {
        Adt159::C0(_, _, _, h3389, _, _, _, _) => {
            h3389
        }
    }, match v0.clone() {
        Adt159::C0(_, _, _, _, h3390, _, _, _) => {
            h3390
        }
    }, match v0.clone() {
        Adt159::C0(_, _, _, _, _, h3391, _, _) => {
            h3391
        }
    }, match v0.clone() {
        Adt159::C0(_, _, _, _, _, _, h3392, _) => {
            h3392
        }
    }, match v0.clone() {
        Adt159::C0(_, _, _, _, _, _, _, h3393) => {
            h3393
        }
    }, v1, 0u64))?;
    Ok(Adt159::C0(f869(match v0.clone() {
        Adt159::C0(h3394, _, _, _, _, _, _, _) => {
            h3394
        }
    }, match v2.clone() {
        Adt161::C0(h3395, _, _, _, _, _, _, _, _, _) => {
            h3395
        }
    }), f869(match v0.clone() {
        Adt159::C0(_, h3396, _, _, _, _, _, _) => {
            h3396
        }
    }, match v2.clone() {
        Adt161::C0(_, h3397, _, _, _, _, _, _, _, _) => {
            h3397
        }
    }), f869(match v0.clone() {
        Adt159::C0(_, _, h3398, _, _, _, _, _) => {
            h3398
        }
    }, match v2.clone() {
        Adt161::C0(_, _, h3399, _, _, _, _, _, _, _) => {
            h3399
        }
    }), f869(match v0.clone() {
        Adt159::C0(_, _, _, h3400, _, _, _, _) => {
            h3400
        }
    }, match v2.clone() {
        Adt161::C0(_, _, _, h3401, _, _, _, _, _, _) => {
            h3401
        }
    }), f869(match v0.clone() {
        Adt159::C0(_, _, _, _, h3402, _, _, _) => {
            h3402
        }
    }, match v2.clone() {
        Adt161::C0(_, _, _, _, h3403, _, _, _, _, _) => {
            h3403
        }
    }), f869(match v0.clone() {
        Adt159::C0(_, _, _, _, _, h3404, _, _) => {
            h3404
        }
    }, match v2.clone() {
        Adt161::C0(_, _, _, _, _, h3405, _, _, _, _) => {
            h3405
        }
    }), f869(match v0.clone() {
        Adt159::C0(_, _, _, _, _, _, h3406, _) => {
            h3406
        }
    }, match v2.clone() {
        Adt161::C0(_, _, _, _, _, _, h3407, _, _, _) => {
            h3407
        }
    }), f869(match v0 {
        Adt159::C0(_, _, _, _, _, _, _, h3408) => {
            h3408
        }
    }, match v2 {
        Adt161::C0(_, _, _, _, _, _, _, h3409, _, _) => {
            h3409
        }
    })))
}

pub fn f855() -> Adt159 {
    Adt159::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f856(v0: Adt160, v1: u32) -> Adt160 {
    Adt160::C0({
        let a3411 = match v0.clone() {
            Adt160::C0(h3410, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h3410
            }
        };
        let a3412 = v1;
        fixed_u32::bit_xor(a3411, a3412)
    }, {
        let a3414 = match v0.clone() {
            Adt160::C0(_, h3413, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h3413
            }
        };
        let a3415 = v1;
        fixed_u32::bit_xor(a3414, a3415)
    }, {
        let a3417 = match v0.clone() {
            Adt160::C0(_, _, h3416, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h3416
            }
        };
        let a3418 = v1;
        fixed_u32::bit_xor(a3417, a3418)
    }, {
        let a3420 = match v0.clone() {
            Adt160::C0(_, _, _, h3419, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h3419
            }
        };
        let a3421 = v1;
        fixed_u32::bit_xor(a3420, a3421)
    }, {
        let a3423 = match v0.clone() {
            Adt160::C0(_, _, _, _, h3422, _, _, _, _, _, _, _, _, _, _, _) => {
                h3422
            }
        };
        let a3424 = v1;
        fixed_u32::bit_xor(a3423, a3424)
    }, {
        let a3426 = match v0.clone() {
            Adt160::C0(_, _, _, _, _, h3425, _, _, _, _, _, _, _, _, _, _) => {
                h3425
            }
        };
        let a3427 = v1;
        fixed_u32::bit_xor(a3426, a3427)
    }, {
        let a3429 = match v0.clone() {
            Adt160::C0(_, _, _, _, _, _, h3428, _, _, _, _, _, _, _, _, _) => {
                h3428
            }
        };
        let a3430 = v1;
        fixed_u32::bit_xor(a3429, a3430)
    }, {
        let a3432 = match v0.clone() {
            Adt160::C0(_, _, _, _, _, _, _, h3431, _, _, _, _, _, _, _, _) => {
                h3431
            }
        };
        let a3433 = v1;
        fixed_u32::bit_xor(a3432, a3433)
    }, {
        let a3435 = match v0.clone() {
            Adt160::C0(_, _, _, _, _, _, _, _, h3434, _, _, _, _, _, _, _) => {
                h3434
            }
        };
        let a3436 = v1;
        fixed_u32::bit_xor(a3435, a3436)
    }, {
        let a3438 = match v0.clone() {
            Adt160::C0(_, _, _, _, _, _, _, _, _, h3437, _, _, _, _, _, _) => {
                h3437
            }
        };
        let a3439 = v1;
        fixed_u32::bit_xor(a3438, a3439)
    }, {
        let a3441 = match v0.clone() {
            Adt160::C0(_, _, _, _, _, _, _, _, _, _, h3440, _, _, _, _, _) => {
                h3440
            }
        };
        let a3442 = v1;
        fixed_u32::bit_xor(a3441, a3442)
    }, {
        let a3444 = match v0.clone() {
            Adt160::C0(_, _, _, _, _, _, _, _, _, _, _, h3443, _, _, _, _) => {
                h3443
            }
        };
        let a3445 = v1;
        fixed_u32::bit_xor(a3444, a3445)
    }, {
        let a3447 = match v0.clone() {
            Adt160::C0(_, _, _, _, _, _, _, _, _, _, _, _, h3446, _, _, _) => {
                h3446
            }
        };
        let a3448 = v1;
        fixed_u32::bit_xor(a3447, a3448)
    }, {
        let a3450 = match v0.clone() {
            Adt160::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h3449, _, _) => {
                h3449
            }
        };
        let a3451 = v1;
        fixed_u32::bit_xor(a3450, a3451)
    }, {
        let a3453 = match v0.clone() {
            Adt160::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h3452, _) => {
                h3452
            }
        };
        let a3454 = v1;
        fixed_u32::bit_xor(a3453, a3454)
    }, {
        let a3456 = match v0 {
            Adt160::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h3455) => {
                h3455
            }
        };
        let a3457 = v1;
        fixed_u32::bit_xor(a3456, a3457)
    })
}

pub fn f857(v0: Adt159) -> Adt160 {
    Adt160::C0(match v0.clone() {
        Adt159::C0(h3458, _, _, _, _, _, _, _) => {
            h3458
        }
    }, match v0.clone() {
        Adt159::C0(_, h3459, _, _, _, _, _, _) => {
            h3459
        }
    }, match v0.clone() {
        Adt159::C0(_, _, h3460, _, _, _, _, _) => {
            h3460
        }
    }, match v0.clone() {
        Adt159::C0(_, _, _, h3461, _, _, _, _) => {
            h3461
        }
    }, match v0.clone() {
        Adt159::C0(_, _, _, _, h3462, _, _, _) => {
            h3462
        }
    }, match v0.clone() {
        Adt159::C0(_, _, _, _, _, h3463, _, _) => {
            h3463
        }
    }, match v0.clone() {
        Adt159::C0(_, _, _, _, _, _, h3464, _) => {
            h3464
        }
    }, match v0 {
        Adt159::C0(_, _, _, _, _, _, _, h3465) => {
            h3465
        }
    }, 2147483648u32, f870(), f870(), f870(), f870(), f870(), f870(), 768u32)
}

pub fn f858(v0: Bytes, v1: Bytes, v2: (u64, u64)) -> R<(u64, u64)> {
    Ok(({
        let a3467 = {
            let (h3466, _) = v2;
            h3466
        };
        let a3468 = 1u64;
        nat_add(a3467, a3468)?
    }, {
        let a3520 = {
            let (_, h3469) = v2;
            h3469
        };
        let a3521 = {
            let v3: u8 = {
                let a3472 = f871(v0, {
                    let (h3470, _) = v2;
                    h3470
                });
                let a3473 = f871(v1, {
                    let (h3471, _) = v2;
                    h3471
                });
                fixed_u8::bit_xor(a3472, a3473)
            };
            let a3518 = {
                let a3494 = {
                    let a3482 = {
                        let a3476 = {
                            let a3474 = v3;
                            let a3475 = 128u8;
                            fixed_u8::bit_and(a3474, a3475)
                        };
                        let a3477 = 128u8;
                        if equal(a3476, a3477) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a3483 = {
                        let a3480 = {
                            let a3478 = v3;
                            let a3479 = 64u8;
                            fixed_u8::bit_and(a3478, a3479)
                        };
                        let a3481 = 64u8;
                        if equal(a3480, a3481) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a3482, a3483)?
                };
                let a3495 = {
                    let a3492 = {
                        let a3486 = {
                            let a3484 = v3;
                            let a3485 = 32u8;
                            fixed_u8::bit_and(a3484, a3485)
                        };
                        let a3487 = 32u8;
                        if equal(a3486, a3487) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a3493 = {
                        let a3490 = {
                            let a3488 = v3;
                            let a3489 = 16u8;
                            fixed_u8::bit_and(a3488, a3489)
                        };
                        let a3491 = 16u8;
                        if equal(a3490, a3491) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a3492, a3493)?
                };
                nat_add(a3494, a3495)?
            };
            let a3519 = {
                let a3516 = {
                    let a3504 = {
                        let a3498 = {
                            let a3496 = v3;
                            let a3497 = 8u8;
                            fixed_u8::bit_and(a3496, a3497)
                        };
                        let a3499 = 8u8;
                        if equal(a3498, a3499) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a3505 = {
                        let a3502 = {
                            let a3500 = v3;
                            let a3501 = 4u8;
                            fixed_u8::bit_and(a3500, a3501)
                        };
                        let a3503 = 4u8;
                        if equal(a3502, a3503) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a3504, a3505)?
                };
                let a3517 = {
                    let a3514 = {
                        let a3508 = {
                            let a3506 = v3;
                            let a3507 = 2u8;
                            fixed_u8::bit_and(a3506, a3507)
                        };
                        let a3509 = 2u8;
                        if equal(a3508, a3509) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a3515 = {
                        let a3512 = {
                            let a3510 = v3;
                            let a3511 = 1u8;
                            fixed_u8::bit_and(a3510, a3511)
                        };
                        let a3513 = 1u8;
                        if equal(a3512, a3513) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a3514, a3515)?
                };
                nat_add(a3516, a3517)?
            };
            nat_add(a3518, a3519)?
        };
        nat_add(a3520, a3521)?
    }))
}

pub fn f859(v0: u64) -> u64 {
    let a3522 = v0;
    let a3523 = 48u64;
    if nat_lt(a3522, a3523) {
        16u64
    } else {
        let a3524 = v0;
        let a3525 = 58u64;
        if nat_lt(a3524, a3525) {
            let a3526 = v0;
            let a3527 = 48u64;
            nat_sub(a3526, a3527)
        } else {
            let a3528 = v0;
            let a3529 = 97u64;
            if nat_lt(a3528, a3529) {
                16u64
            } else {
                let a3530 = v0;
                let a3531 = 103u64;
                if nat_lt(a3530, a3531) {
                    let a3532 = v0;
                    let a3533 = 87u64;
                    nat_sub(a3532, a3533)
                } else {
                    16u64
                }
            }
        }
    }
}

pub fn f860(v0: u32, v1: u32) -> R<Bytes> {
    Ok(f214({
        let v2: u32 = {
            let a3534 = f872(v0, v1);
            let a3535 = 255u32;
            fixed_u32::bit_and(a3534, a3535)
        };
        let a3724 = {
            let a3628 = {
                let a3580 = {
                    let a3556 = {
                        let a3544 = {
                            let a3538 = {
                                let a3536 = v2;
                                let a3537 = 2147483648u32;
                                fixed_u32::bit_and(a3536, a3537)
                            };
                            let a3539 = 2147483648u32;
                            if equal(a3538, a3539) {
                                2147483648u64
                            } else {
                                0u64
                            }
                        };
                        let a3545 = {
                            let a3542 = {
                                let a3540 = v2;
                                let a3541 = 1073741824u32;
                                fixed_u32::bit_and(a3540, a3541)
                            };
                            let a3543 = 1073741824u32;
                            if equal(a3542, a3543) {
                                1073741824u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3544, a3545)?
                    };
                    let a3557 = {
                        let a3554 = {
                            let a3548 = {
                                let a3546 = v2;
                                let a3547 = 536870912u32;
                                fixed_u32::bit_and(a3546, a3547)
                            };
                            let a3549 = 536870912u32;
                            if equal(a3548, a3549) {
                                536870912u64
                            } else {
                                0u64
                            }
                        };
                        let a3555 = {
                            let a3552 = {
                                let a3550 = v2;
                                let a3551 = 268435456u32;
                                fixed_u32::bit_and(a3550, a3551)
                            };
                            let a3553 = 268435456u32;
                            if equal(a3552, a3553) {
                                268435456u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3554, a3555)?
                    };
                    nat_add(a3556, a3557)?
                };
                let a3581 = {
                    let a3578 = {
                        let a3566 = {
                            let a3560 = {
                                let a3558 = v2;
                                let a3559 = 134217728u32;
                                fixed_u32::bit_and(a3558, a3559)
                            };
                            let a3561 = 134217728u32;
                            if equal(a3560, a3561) {
                                134217728u64
                            } else {
                                0u64
                            }
                        };
                        let a3567 = {
                            let a3564 = {
                                let a3562 = v2;
                                let a3563 = 67108864u32;
                                fixed_u32::bit_and(a3562, a3563)
                            };
                            let a3565 = 67108864u32;
                            if equal(a3564, a3565) {
                                67108864u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3566, a3567)?
                    };
                    let a3579 = {
                        let a3576 = {
                            let a3570 = {
                                let a3568 = v2;
                                let a3569 = 33554432u32;
                                fixed_u32::bit_and(a3568, a3569)
                            };
                            let a3571 = 33554432u32;
                            if equal(a3570, a3571) {
                                33554432u64
                            } else {
                                0u64
                            }
                        };
                        let a3577 = {
                            let a3574 = {
                                let a3572 = v2;
                                let a3573 = 16777216u32;
                                fixed_u32::bit_and(a3572, a3573)
                            };
                            let a3575 = 16777216u32;
                            if equal(a3574, a3575) {
                                16777216u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3576, a3577)?
                    };
                    nat_add(a3578, a3579)?
                };
                nat_add(a3580, a3581)?
            };
            let a3629 = {
                let a3626 = {
                    let a3602 = {
                        let a3590 = {
                            let a3584 = {
                                let a3582 = v2;
                                let a3583 = 8388608u32;
                                fixed_u32::bit_and(a3582, a3583)
                            };
                            let a3585 = 8388608u32;
                            if equal(a3584, a3585) {
                                8388608u64
                            } else {
                                0u64
                            }
                        };
                        let a3591 = {
                            let a3588 = {
                                let a3586 = v2;
                                let a3587 = 4194304u32;
                                fixed_u32::bit_and(a3586, a3587)
                            };
                            let a3589 = 4194304u32;
                            if equal(a3588, a3589) {
                                4194304u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3590, a3591)?
                    };
                    let a3603 = {
                        let a3600 = {
                            let a3594 = {
                                let a3592 = v2;
                                let a3593 = 2097152u32;
                                fixed_u32::bit_and(a3592, a3593)
                            };
                            let a3595 = 2097152u32;
                            if equal(a3594, a3595) {
                                2097152u64
                            } else {
                                0u64
                            }
                        };
                        let a3601 = {
                            let a3598 = {
                                let a3596 = v2;
                                let a3597 = 1048576u32;
                                fixed_u32::bit_and(a3596, a3597)
                            };
                            let a3599 = 1048576u32;
                            if equal(a3598, a3599) {
                                1048576u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3600, a3601)?
                    };
                    nat_add(a3602, a3603)?
                };
                let a3627 = {
                    let a3624 = {
                        let a3612 = {
                            let a3606 = {
                                let a3604 = v2;
                                let a3605 = 524288u32;
                                fixed_u32::bit_and(a3604, a3605)
                            };
                            let a3607 = 524288u32;
                            if equal(a3606, a3607) {
                                524288u64
                            } else {
                                0u64
                            }
                        };
                        let a3613 = {
                            let a3610 = {
                                let a3608 = v2;
                                let a3609 = 262144u32;
                                fixed_u32::bit_and(a3608, a3609)
                            };
                            let a3611 = 262144u32;
                            if equal(a3610, a3611) {
                                262144u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3612, a3613)?
                    };
                    let a3625 = {
                        let a3622 = {
                            let a3616 = {
                                let a3614 = v2;
                                let a3615 = 131072u32;
                                fixed_u32::bit_and(a3614, a3615)
                            };
                            let a3617 = 131072u32;
                            if equal(a3616, a3617) {
                                131072u64
                            } else {
                                0u64
                            }
                        };
                        let a3623 = {
                            let a3620 = {
                                let a3618 = v2;
                                let a3619 = 65536u32;
                                fixed_u32::bit_and(a3618, a3619)
                            };
                            let a3621 = 65536u32;
                            if equal(a3620, a3621) {
                                65536u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3622, a3623)?
                    };
                    nat_add(a3624, a3625)?
                };
                nat_add(a3626, a3627)?
            };
            nat_add(a3628, a3629)?
        };
        let a3725 = {
            let a3722 = {
                let a3674 = {
                    let a3650 = {
                        let a3638 = {
                            let a3632 = {
                                let a3630 = v2;
                                let a3631 = 32768u32;
                                fixed_u32::bit_and(a3630, a3631)
                            };
                            let a3633 = 32768u32;
                            if equal(a3632, a3633) {
                                32768u64
                            } else {
                                0u64
                            }
                        };
                        let a3639 = {
                            let a3636 = {
                                let a3634 = v2;
                                let a3635 = 16384u32;
                                fixed_u32::bit_and(a3634, a3635)
                            };
                            let a3637 = 16384u32;
                            if equal(a3636, a3637) {
                                16384u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3638, a3639)?
                    };
                    let a3651 = {
                        let a3648 = {
                            let a3642 = {
                                let a3640 = v2;
                                let a3641 = 8192u32;
                                fixed_u32::bit_and(a3640, a3641)
                            };
                            let a3643 = 8192u32;
                            if equal(a3642, a3643) {
                                8192u64
                            } else {
                                0u64
                            }
                        };
                        let a3649 = {
                            let a3646 = {
                                let a3644 = v2;
                                let a3645 = 4096u32;
                                fixed_u32::bit_and(a3644, a3645)
                            };
                            let a3647 = 4096u32;
                            if equal(a3646, a3647) {
                                4096u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3648, a3649)?
                    };
                    nat_add(a3650, a3651)?
                };
                let a3675 = {
                    let a3672 = {
                        let a3660 = {
                            let a3654 = {
                                let a3652 = v2;
                                let a3653 = 2048u32;
                                fixed_u32::bit_and(a3652, a3653)
                            };
                            let a3655 = 2048u32;
                            if equal(a3654, a3655) {
                                2048u64
                            } else {
                                0u64
                            }
                        };
                        let a3661 = {
                            let a3658 = {
                                let a3656 = v2;
                                let a3657 = 1024u32;
                                fixed_u32::bit_and(a3656, a3657)
                            };
                            let a3659 = 1024u32;
                            if equal(a3658, a3659) {
                                1024u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3660, a3661)?
                    };
                    let a3673 = {
                        let a3670 = {
                            let a3664 = {
                                let a3662 = v2;
                                let a3663 = 512u32;
                                fixed_u32::bit_and(a3662, a3663)
                            };
                            let a3665 = 512u32;
                            if equal(a3664, a3665) {
                                512u64
                            } else {
                                0u64
                            }
                        };
                        let a3671 = {
                            let a3668 = {
                                let a3666 = v2;
                                let a3667 = 256u32;
                                fixed_u32::bit_and(a3666, a3667)
                            };
                            let a3669 = 256u32;
                            if equal(a3668, a3669) {
                                256u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3670, a3671)?
                    };
                    nat_add(a3672, a3673)?
                };
                nat_add(a3674, a3675)?
            };
            let a3723 = {
                let a3720 = {
                    let a3696 = {
                        let a3684 = {
                            let a3678 = {
                                let a3676 = v2;
                                let a3677 = 128u32;
                                fixed_u32::bit_and(a3676, a3677)
                            };
                            let a3679 = 128u32;
                            if equal(a3678, a3679) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a3685 = {
                            let a3682 = {
                                let a3680 = v2;
                                let a3681 = 64u32;
                                fixed_u32::bit_and(a3680, a3681)
                            };
                            let a3683 = 64u32;
                            if equal(a3682, a3683) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3684, a3685)?
                    };
                    let a3697 = {
                        let a3694 = {
                            let a3688 = {
                                let a3686 = v2;
                                let a3687 = 32u32;
                                fixed_u32::bit_and(a3686, a3687)
                            };
                            let a3689 = 32u32;
                            if equal(a3688, a3689) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a3695 = {
                            let a3692 = {
                                let a3690 = v2;
                                let a3691 = 16u32;
                                fixed_u32::bit_and(a3690, a3691)
                            };
                            let a3693 = 16u32;
                            if equal(a3692, a3693) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3694, a3695)?
                    };
                    nat_add(a3696, a3697)?
                };
                let a3721 = {
                    let a3718 = {
                        let a3706 = {
                            let a3700 = {
                                let a3698 = v2;
                                let a3699 = 8u32;
                                fixed_u32::bit_and(a3698, a3699)
                            };
                            let a3701 = 8u32;
                            if equal(a3700, a3701) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a3707 = {
                            let a3704 = {
                                let a3702 = v2;
                                let a3703 = 4u32;
                                fixed_u32::bit_and(a3702, a3703)
                            };
                            let a3705 = 4u32;
                            if equal(a3704, a3705) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3706, a3707)?
                    };
                    let a3719 = {
                        let a3716 = {
                            let a3710 = {
                                let a3708 = v2;
                                let a3709 = 2u32;
                                fixed_u32::bit_and(a3708, a3709)
                            };
                            let a3711 = 2u32;
                            if equal(a3710, a3711) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a3717 = {
                            let a3714 = {
                                let a3712 = v2;
                                let a3713 = 1u32;
                                fixed_u32::bit_and(a3712, a3713)
                            };
                            let a3715 = 1u32;
                            if equal(a3714, a3715) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3716, a3717)?
                    };
                    nat_add(a3718, a3719)?
                };
                nat_add(a3720, a3721)?
            };
            nat_add(a3722, a3723)?
        };
        nat_add(a3724, a3725)?
    }))
}

pub fn f861() -> u64 {
    64u64
}

pub fn f862(v0: Adt159) -> Adt160 {
    Adt160::C0(match v0.clone() {
        Adt159::C0(h3726, _, _, _, _, _, _, _) => {
            h3726
        }
    }, match v0.clone() {
        Adt159::C0(_, h3727, _, _, _, _, _, _) => {
            h3727
        }
    }, match v0.clone() {
        Adt159::C0(_, _, h3728, _, _, _, _, _) => {
            h3728
        }
    }, match v0.clone() {
        Adt159::C0(_, _, _, h3729, _, _, _, _) => {
            h3729
        }
    }, match v0.clone() {
        Adt159::C0(_, _, _, _, h3730, _, _, _) => {
            h3730
        }
    }, match v0.clone() {
        Adt159::C0(_, _, _, _, _, h3731, _, _) => {
            h3731
        }
    }, match v0.clone() {
        Adt159::C0(_, _, _, _, _, _, h3732, _) => {
            h3732
        }
    }, match v0 {
        Adt159::C0(_, _, _, _, _, _, _, h3733) => {
            h3733
        }
    }, f870(), f870(), f870(), f870(), f870(), f870(), f870(), f870())
}

pub fn f863(v0: Bytes) -> R<Adt159> {
    let v1: u64 = {
        let a3734 = v0.clone();
        length_bytes(a3734)
    };
    let v2: u64 = {
        let a3740 = {
            let a3737 = {
                let a3735 = v1;
                let a3736 = 72u64;
                nat_add(a3735, a3736)?
            };
            let a3738 = 64u64;
            let a3739 = 0u64;
            nat_quot(a3737, a3738, a3739)
        };
        let a3741 = 64u64;
        nat_mul(a3740, a3741)?
    };
    let (_, h3742) = f865(Fn7::F873(v0, v1, v2), {
        let a3743 = v2;
        let a3744 = 64u64;
        let a3745 = 0u64;
        nat_quot(a3743, a3744, a3745)
    }, (0u64, f855()))?;
    Ok(h3742)
}

pub fn f864(v0: Bytes) -> R<Adt160> {
    Ok(Adt160::C0(f874(v0.clone(), 0u64)?, f874(v0.clone(), 4u64)?, f874(v0.clone(), 8u64)?, f874(v0.clone(), 12u64)?, f874(v0.clone(), 16u64)?, f874(v0.clone(), 20u64)?, f874(v0.clone(), 24u64)?, f874(v0.clone(), 28u64)?, f874(v0.clone(), 32u64)?, f874(v0.clone(), 36u64)?, f874(v0.clone(), 40u64)?, f874(v0.clone(), 44u64)?, f874(v0.clone(), 48u64)?, f874(v0.clone(), 52u64)?, f874(v0.clone(), 56u64)?, f874(v0, 60u64)?))
}

pub fn f865(v0: Fn7, p3749: u64, p3750: (u64, Adt159)) -> R<(u64, Adt159)> {
    let m3748 = (p3749, p3750);
    {
        let mut m3748 = m3748;
        loop {
            let step = {
                let (v1, v2) = m3748;
                let m3746 = v1;
                if m3746 == 0 {
                    Err::<(u64, (u64, Adt159)), R<(u64, Adt159)>>(Ok(v2))
                } else {
                    let v3 = m3746 - 1;
                    Ok::<(u64, (u64, Adt159)), R<(u64, Adt159)>>((v3, {
                        let c3747: Fn7 = v0.clone();
                        c3747.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m3748 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f866(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: (u64, Adt159)) -> R<(u64, Adt159)> {
    Ok(({
        let a3752 = {
            let (h3751, _) = v4.clone();
            h3751
        };
        let a3753 = 64u64;
        nat_add(a3752, a3753)?
    }, f854({
        let (_, h3754) = v4;
        h3754
    }, f875(v0, v2, v3, {
        let a3757 = {
            let a3755 = v1;
            let a3756 = v2;
            nat_add(a3755, a3756)?
        };
        let a3758 = 8u64;
        nat_mul(a3757, a3758)?
    }, {
        let (h3759, _) = v4;
        h3759
    })?)?))
}

pub fn f867(v0: Fn8, p3763: u64, p3764: Adt161) -> R<Adt161> {
    let m3762 = (p3763, p3764);
    {
        let mut m3762 = m3762;
        loop {
            let step = {
                let (v1, v2) = m3762;
                let m3760 = v1;
                if m3760 == 0 {
                    Err::<(u64, Adt161), R<Adt161>>(Ok(v2))
                } else {
                    let v3 = m3760 - 1;
                    Ok::<(u64, Adt161), R<Adt161>>((v3, {
                        let c3761: Fn8 = v0.clone();
                        c3761.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m3762 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f868(v0: Adt161) -> R<Adt161> {
    f876(v0)
}

pub fn f869(v0: u32, v1: u32) -> u32 {
    let m3767 = {
        let a3765 = f877(v0);
        let a3766 = f877(v1);
        fixed_u64::checked_add(a3765, a3766)
    };
    match m3767 {
        None => {
            0u32
        }
        Some(v2) => {
            f878(v2)
        }
    }
}

pub fn f870() -> u32 {
    0u32
}

pub fn f871(v0: Bytes, v1: u64) -> u8 {
    let m3770 = {
        let a3768 = v0;
        let a3769 = v1;
        index_bytes(a3768, a3769)
    };
    match m3770 {
        None => {
            0u8
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f872(v0: u32, v1: u32) -> u32 {
    let m3773 = {
        let a3771 = v0;
        let a3772 = v1;
        fixed_u32::shift_right(a3771, a3772)
    };
    match m3773 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f873(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt159)) -> R<(u64, Adt159)> {
    Ok(({
        let a3775 = {
            let (h3774, _) = v3.clone();
            h3774
        };
        let a3776 = 64u64;
        nat_add(a3775, a3776)?
    }, f854({
        let (_, h3777) = v3;
        h3777
    }, f879(v0, v1, v2, {
        let (h3778, _) = v3;
        h3778
    })?)?))
}

pub fn f874(v0: Bytes, v1: u64) -> R<u32> {
    Ok(f880(f881(v0.clone(), v1), f881(v0.clone(), {
        let a3779 = v1;
        let a3780 = 1u64;
        nat_add(a3779, a3780)?
    }), f881(v0.clone(), {
        let a3781 = v1;
        let a3782 = 2u64;
        nat_add(a3781, a3782)?
    }), f881(v0, {
        let a3783 = v1;
        let a3784 = 3u64;
        nat_add(a3783, a3784)?
    })))
}

pub fn f875(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Adt160> {
    Ok(Adt160::C0(f882(v0.clone(), v1, v2, v3, v4)?, f882(v0.clone(), v1, v2, v3, {
        let a3785 = v4;
        let a3786 = 4u64;
        nat_add(a3785, a3786)?
    })?, f882(v0.clone(), v1, v2, v3, {
        let a3787 = v4;
        let a3788 = 8u64;
        nat_add(a3787, a3788)?
    })?, f882(v0.clone(), v1, v2, v3, {
        let a3789 = v4;
        let a3790 = 12u64;
        nat_add(a3789, a3790)?
    })?, f882(v0.clone(), v1, v2, v3, {
        let a3791 = v4;
        let a3792 = 16u64;
        nat_add(a3791, a3792)?
    })?, f882(v0.clone(), v1, v2, v3, {
        let a3793 = v4;
        let a3794 = 20u64;
        nat_add(a3793, a3794)?
    })?, f882(v0.clone(), v1, v2, v3, {
        let a3795 = v4;
        let a3796 = 24u64;
        nat_add(a3795, a3796)?
    })?, f882(v0.clone(), v1, v2, v3, {
        let a3797 = v4;
        let a3798 = 28u64;
        nat_add(a3797, a3798)?
    })?, f882(v0.clone(), v1, v2, v3, {
        let a3799 = v4;
        let a3800 = 32u64;
        nat_add(a3799, a3800)?
    })?, f882(v0.clone(), v1, v2, v3, {
        let a3801 = v4;
        let a3802 = 36u64;
        nat_add(a3801, a3802)?
    })?, f882(v0.clone(), v1, v2, v3, {
        let a3803 = v4;
        let a3804 = 40u64;
        nat_add(a3803, a3804)?
    })?, f882(v0.clone(), v1, v2, v3, {
        let a3805 = v4;
        let a3806 = 44u64;
        nat_add(a3805, a3806)?
    })?, f882(v0.clone(), v1, v2, v3, {
        let a3807 = v4;
        let a3808 = 48u64;
        nat_add(a3807, a3808)?
    })?, f882(v0.clone(), v1, v2, v3, {
        let a3809 = v4;
        let a3810 = 52u64;
        nat_add(a3809, a3810)?
    })?, f882(v0.clone(), v1, v2, v3, {
        let a3811 = v4;
        let a3812 = 56u64;
        nat_add(a3811, a3812)?
    })?, f882(v0, v1, v2, v3, {
        let a3813 = v4;
        let a3814 = 60u64;
        nat_add(a3813, a3814)?
    })?))
}

pub fn f876(v0: Adt161) -> R<Adt161> {
    let v1: u32 = f883(match v0.clone() {
        Adt161::C0(_, _, _, _, _, _, _, _, _, h3815) => {
            h3815
        }
    });
    let v2: u32 = f869(match v0.clone() {
        Adt161::C0(_, _, _, _, _, _, _, h3816, _, _) => {
            h3816
        }
    }, f869(f884(match v0.clone() {
        Adt161::C0(_, _, _, _, h3817, _, _, _, _, _) => {
            h3817
        }
    }), f869(f885(match v0.clone() {
        Adt161::C0(_, _, _, _, h3818, _, _, _, _, _) => {
            h3818
        }
    }, match v0.clone() {
        Adt161::C0(_, _, _, _, _, h3819, _, _, _, _) => {
            h3819
        }
    }, match v0.clone() {
        Adt161::C0(_, _, _, _, _, _, h3820, _, _, _) => {
            h3820
        }
    }), f869(v1, {
        let m3823 = match v0.clone() {
            Adt161::C0(_, _, _, _, _, _, _, _, h3822, _) => {
                h3822
            }
        };
        match m3823 {
            Adt160::C0(h3821, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h3821
            }
        }
    }))));
    let v3: u32 = f869(f886(match v0.clone() {
        Adt161::C0(h3824, _, _, _, _, _, _, _, _, _) => {
            h3824
        }
    }), f887(match v0.clone() {
        Adt161::C0(h3825, _, _, _, _, _, _, _, _, _) => {
            h3825
        }
    }, match v0.clone() {
        Adt161::C0(_, h3826, _, _, _, _, _, _, _, _) => {
            h3826
        }
    }, match v0.clone() {
        Adt161::C0(_, _, h3827, _, _, _, _, _, _, _) => {
            h3827
        }
    }));
    Ok(Adt161::C0(f869(v2, v3), match v0.clone() {
        Adt161::C0(h3828, _, _, _, _, _, _, _, _, _) => {
            h3828
        }
    }, match v0.clone() {
        Adt161::C0(_, h3829, _, _, _, _, _, _, _, _) => {
            h3829
        }
    }, match v0.clone() {
        Adt161::C0(_, _, h3830, _, _, _, _, _, _, _) => {
            h3830
        }
    }, f869(match v0.clone() {
        Adt161::C0(_, _, _, h3831, _, _, _, _, _, _) => {
            h3831
        }
    }, v2), match v0.clone() {
        Adt161::C0(_, _, _, _, h3832, _, _, _, _, _) => {
            h3832
        }
    }, match v0.clone() {
        Adt161::C0(_, _, _, _, _, h3833, _, _, _, _) => {
            h3833
        }
    }, match v0.clone() {
        Adt161::C0(_, _, _, _, _, _, h3834, _, _, _) => {
            h3834
        }
    }, f888(match v0.clone() {
        Adt161::C0(_, _, _, _, _, _, _, _, h3835, _) => {
            h3835
        }
    }), {
        let a3837 = match v0 {
            Adt161::C0(_, _, _, _, _, _, _, _, _, h3836) => {
                h3836
            }
        };
        let a3838 = 1u64;
        nat_add(a3837, a3838)?
    }))
}

pub fn f877(v0: u32) -> u64 {
    let m3840 = {
        let a3839 = v0;
        fixed_u64::convert(i128::from(a3839))
    };
    match m3840 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f878(v0: u64) -> u32 {
    let m3844 = {
        let a3843 = {
            let a3841 = v0;
            let a3842 = 4294967295u64;
            fixed_u64::bit_and(a3841, a3842)
        };
        fixed_u32::convert(i128::from(a3843))
    };
    match m3844 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f879(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt160> {
    Ok(Adt160::C0(f889(v0.clone(), v1, v2, v3)?, f889(v0.clone(), v1, v2, {
        let a3845 = v3;
        let a3846 = 4u64;
        nat_add(a3845, a3846)?
    })?, f889(v0.clone(), v1, v2, {
        let a3847 = v3;
        let a3848 = 8u64;
        nat_add(a3847, a3848)?
    })?, f889(v0.clone(), v1, v2, {
        let a3849 = v3;
        let a3850 = 12u64;
        nat_add(a3849, a3850)?
    })?, f889(v0.clone(), v1, v2, {
        let a3851 = v3;
        let a3852 = 16u64;
        nat_add(a3851, a3852)?
    })?, f889(v0.clone(), v1, v2, {
        let a3853 = v3;
        let a3854 = 20u64;
        nat_add(a3853, a3854)?
    })?, f889(v0.clone(), v1, v2, {
        let a3855 = v3;
        let a3856 = 24u64;
        nat_add(a3855, a3856)?
    })?, f889(v0.clone(), v1, v2, {
        let a3857 = v3;
        let a3858 = 28u64;
        nat_add(a3857, a3858)?
    })?, f889(v0.clone(), v1, v2, {
        let a3859 = v3;
        let a3860 = 32u64;
        nat_add(a3859, a3860)?
    })?, f889(v0.clone(), v1, v2, {
        let a3861 = v3;
        let a3862 = 36u64;
        nat_add(a3861, a3862)?
    })?, f889(v0.clone(), v1, v2, {
        let a3863 = v3;
        let a3864 = 40u64;
        nat_add(a3863, a3864)?
    })?, f889(v0.clone(), v1, v2, {
        let a3865 = v3;
        let a3866 = 44u64;
        nat_add(a3865, a3866)?
    })?, f889(v0.clone(), v1, v2, {
        let a3867 = v3;
        let a3868 = 48u64;
        nat_add(a3867, a3868)?
    })?, f889(v0.clone(), v1, v2, {
        let a3869 = v3;
        let a3870 = 52u64;
        nat_add(a3869, a3870)?
    })?, f889(v0.clone(), v1, v2, {
        let a3871 = v3;
        let a3872 = 56u64;
        nat_add(a3871, a3872)?
    })?, f889(v0, v1, v2, {
        let a3873 = v3;
        let a3874 = 60u64;
        nat_add(a3873, a3874)?
    })?))
}

pub fn f880(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a3879 = f890(v0, 24u32);
    let a3880 = {
        let a3877 = f890(v1, 16u32);
        let a3878 = {
            let a3875 = f890(v2, 8u32);
            let a3876 = v3;
            fixed_u32::bit_or(a3875, a3876)
        };
        fixed_u32::bit_or(a3877, a3878)
    };
    fixed_u32::bit_or(a3879, a3880)
}

pub fn f881(v0: Bytes, v1: u64) -> u32 {
    let m3883 = {
        let a3881 = v0;
        let a3882 = v1;
        index_bytes(a3881, a3882)
    };
    match m3883 {
        None => {
            0u32
        }
        Some(v2) => {
            let m3885 = {
                let a3884 = v2;
                fixed_u32::convert(i128::from(a3884))
            };
            match m3885 {
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

pub fn f882(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u32> {
    let a3888 = {
        let a3886 = v4;
        let a3887 = 8u64;
        nat_add(a3886, a3887)?
    };
    let a3889 = v2;
    if nat_eq(a3888, a3889) {
        Ok(f891(v3, true))
    } else {
        let a3892 = {
            let a3890 = v4;
            let a3891 = 4u64;
            nat_add(a3890, a3891)?
        };
        let a3893 = v2;
        if nat_eq(a3892, a3893) {
            Ok(f891(v3, false))
        } else {
            Ok(f880(f892(v0.clone(), v1, v4), f892(v0.clone(), v1, {
                let a3894 = v4;
                let a3895 = 1u64;
                nat_add(a3894, a3895)?
            }), f892(v0.clone(), v1, {
                let a3896 = v4;
                let a3897 = 2u64;
                nat_add(a3896, a3897)?
            }), f892(v0, v1, {
                let a3898 = v4;
                let a3899 = 3u64;
                nat_add(a3898, a3899)?
            })))
        }
    }
}

pub fn f883(v0: u64) -> u32 {
    let a3900 = v0;
    let a3901 = 32u64;
    if nat_lt(a3900, a3901) {
        let a3902 = v0;
        let a3903 = 16u64;
        if nat_lt(a3902, a3903) {
            let a3904 = v0;
            let a3905 = 8u64;
            if nat_lt(a3904, a3905) {
                let a3906 = v0;
                let a3907 = 4u64;
                if nat_lt(a3906, a3907) {
                    let a3908 = v0;
                    let a3909 = 2u64;
                    if nat_lt(a3908, a3909) {
                        let a3910 = v0;
                        let a3911 = 1u64;
                        if nat_lt(a3910, a3911) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a3912 = v0;
                        let a3913 = 3u64;
                        if nat_lt(a3912, a3913) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a3914 = v0;
                    let a3915 = 6u64;
                    if nat_lt(a3914, a3915) {
                        let a3916 = v0;
                        let a3917 = 5u64;
                        if nat_lt(a3916, a3917) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a3918 = v0;
                        let a3919 = 7u64;
                        if nat_lt(a3918, a3919) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a3920 = v0;
                let a3921 = 12u64;
                if nat_lt(a3920, a3921) {
                    let a3922 = v0;
                    let a3923 = 10u64;
                    if nat_lt(a3922, a3923) {
                        let a3924 = v0;
                        let a3925 = 9u64;
                        if nat_lt(a3924, a3925) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a3926 = v0;
                        let a3927 = 11u64;
                        if nat_lt(a3926, a3927) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a3928 = v0;
                    let a3929 = 14u64;
                    if nat_lt(a3928, a3929) {
                        let a3930 = v0;
                        let a3931 = 13u64;
                        if nat_lt(a3930, a3931) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a3932 = v0;
                        let a3933 = 15u64;
                        if nat_lt(a3932, a3933) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a3934 = v0;
            let a3935 = 24u64;
            if nat_lt(a3934, a3935) {
                let a3936 = v0;
                let a3937 = 20u64;
                if nat_lt(a3936, a3937) {
                    let a3938 = v0;
                    let a3939 = 18u64;
                    if nat_lt(a3938, a3939) {
                        let a3940 = v0;
                        let a3941 = 17u64;
                        if nat_lt(a3940, a3941) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a3942 = v0;
                        let a3943 = 19u64;
                        if nat_lt(a3942, a3943) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a3944 = v0;
                    let a3945 = 22u64;
                    if nat_lt(a3944, a3945) {
                        let a3946 = v0;
                        let a3947 = 21u64;
                        if nat_lt(a3946, a3947) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a3948 = v0;
                        let a3949 = 23u64;
                        if nat_lt(a3948, a3949) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a3950 = v0;
                let a3951 = 28u64;
                if nat_lt(a3950, a3951) {
                    let a3952 = v0;
                    let a3953 = 26u64;
                    if nat_lt(a3952, a3953) {
                        let a3954 = v0;
                        let a3955 = 25u64;
                        if nat_lt(a3954, a3955) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a3956 = v0;
                        let a3957 = 27u64;
                        if nat_lt(a3956, a3957) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a3958 = v0;
                    let a3959 = 30u64;
                    if nat_lt(a3958, a3959) {
                        let a3960 = v0;
                        let a3961 = 29u64;
                        if nat_lt(a3960, a3961) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a3962 = v0;
                        let a3963 = 31u64;
                        if nat_lt(a3962, a3963) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a3964 = v0;
        let a3965 = 48u64;
        if nat_lt(a3964, a3965) {
            let a3966 = v0;
            let a3967 = 40u64;
            if nat_lt(a3966, a3967) {
                let a3968 = v0;
                let a3969 = 36u64;
                if nat_lt(a3968, a3969) {
                    let a3970 = v0;
                    let a3971 = 34u64;
                    if nat_lt(a3970, a3971) {
                        let a3972 = v0;
                        let a3973 = 33u64;
                        if nat_lt(a3972, a3973) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a3974 = v0;
                        let a3975 = 35u64;
                        if nat_lt(a3974, a3975) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a3976 = v0;
                    let a3977 = 38u64;
                    if nat_lt(a3976, a3977) {
                        let a3978 = v0;
                        let a3979 = 37u64;
                        if nat_lt(a3978, a3979) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a3980 = v0;
                        let a3981 = 39u64;
                        if nat_lt(a3980, a3981) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a3982 = v0;
                let a3983 = 44u64;
                if nat_lt(a3982, a3983) {
                    let a3984 = v0;
                    let a3985 = 42u64;
                    if nat_lt(a3984, a3985) {
                        let a3986 = v0;
                        let a3987 = 41u64;
                        if nat_lt(a3986, a3987) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a3988 = v0;
                        let a3989 = 43u64;
                        if nat_lt(a3988, a3989) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a3990 = v0;
                    let a3991 = 46u64;
                    if nat_lt(a3990, a3991) {
                        let a3992 = v0;
                        let a3993 = 45u64;
                        if nat_lt(a3992, a3993) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a3994 = v0;
                        let a3995 = 47u64;
                        if nat_lt(a3994, a3995) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a3996 = v0;
            let a3997 = 56u64;
            if nat_lt(a3996, a3997) {
                let a3998 = v0;
                let a3999 = 52u64;
                if nat_lt(a3998, a3999) {
                    let a4000 = v0;
                    let a4001 = 50u64;
                    if nat_lt(a4000, a4001) {
                        let a4002 = v0;
                        let a4003 = 49u64;
                        if nat_lt(a4002, a4003) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a4004 = v0;
                        let a4005 = 51u64;
                        if nat_lt(a4004, a4005) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a4006 = v0;
                    let a4007 = 54u64;
                    if nat_lt(a4006, a4007) {
                        let a4008 = v0;
                        let a4009 = 53u64;
                        if nat_lt(a4008, a4009) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a4010 = v0;
                        let a4011 = 55u64;
                        if nat_lt(a4010, a4011) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a4012 = v0;
                let a4013 = 60u64;
                if nat_lt(a4012, a4013) {
                    let a4014 = v0;
                    let a4015 = 58u64;
                    if nat_lt(a4014, a4015) {
                        let a4016 = v0;
                        let a4017 = 57u64;
                        if nat_lt(a4016, a4017) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a4018 = v0;
                        let a4019 = 59u64;
                        if nat_lt(a4018, a4019) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a4020 = v0;
                    let a4021 = 62u64;
                    if nat_lt(a4020, a4021) {
                        let a4022 = v0;
                        let a4023 = 61u64;
                        if nat_lt(a4022, a4023) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a4024 = v0;
                        let a4025 = 63u64;
                        if nat_lt(a4024, a4025) {
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

pub fn f884(v0: u32) -> u32 {
    let a4028 = {
        let a4026 = f893(v0, 6u32, 26u32);
        let a4027 = f893(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a4026, a4027)
    };
    let a4029 = f893(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a4028, a4029)
}

pub fn f885(v0: u32, v1: u32, v2: u32) -> u32 {
    let a4035 = {
        let a4030 = v0;
        let a4031 = v1;
        fixed_u32::bit_and(a4030, a4031)
    };
    let a4036 = {
        let a4033 = {
            let a4032 = v0;
            fixed_u32::bit_not(a4032)
        };
        let a4034 = v2;
        fixed_u32::bit_and(a4033, a4034)
    };
    fixed_u32::bit_xor(a4035, a4036)
}

pub fn f886(v0: u32) -> u32 {
    let a4039 = {
        let a4037 = f893(v0, 2u32, 30u32);
        let a4038 = f893(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a4037, a4038)
    };
    let a4040 = f893(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a4039, a4040)
}

pub fn f887(v0: u32, v1: u32, v2: u32) -> u32 {
    let a4049 = {
        let a4045 = {
            let a4041 = v0;
            let a4042 = v1;
            fixed_u32::bit_and(a4041, a4042)
        };
        let a4046 = {
            let a4043 = v0;
            let a4044 = v2;
            fixed_u32::bit_and(a4043, a4044)
        };
        fixed_u32::bit_xor(a4045, a4046)
    };
    let a4050 = {
        let a4047 = v1;
        let a4048 = v2;
        fixed_u32::bit_and(a4047, a4048)
    };
    fixed_u32::bit_xor(a4049, a4050)
}

pub fn f888(v0: Adt160) -> Adt160 {
    Adt160::C0(match v0.clone() {
        Adt160::C0(_, h4051, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h4051
        }
    }, match v0.clone() {
        Adt160::C0(_, _, h4052, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h4052
        }
    }, match v0.clone() {
        Adt160::C0(_, _, _, h4053, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h4053
        }
    }, match v0.clone() {
        Adt160::C0(_, _, _, _, h4054, _, _, _, _, _, _, _, _, _, _, _) => {
            h4054
        }
    }, match v0.clone() {
        Adt160::C0(_, _, _, _, _, h4055, _, _, _, _, _, _, _, _, _, _) => {
            h4055
        }
    }, match v0.clone() {
        Adt160::C0(_, _, _, _, _, _, h4056, _, _, _, _, _, _, _, _, _) => {
            h4056
        }
    }, match v0.clone() {
        Adt160::C0(_, _, _, _, _, _, _, h4057, _, _, _, _, _, _, _, _) => {
            h4057
        }
    }, match v0.clone() {
        Adt160::C0(_, _, _, _, _, _, _, _, h4058, _, _, _, _, _, _, _) => {
            h4058
        }
    }, match v0.clone() {
        Adt160::C0(_, _, _, _, _, _, _, _, _, h4059, _, _, _, _, _, _) => {
            h4059
        }
    }, match v0.clone() {
        Adt160::C0(_, _, _, _, _, _, _, _, _, _, h4060, _, _, _, _, _) => {
            h4060
        }
    }, match v0.clone() {
        Adt160::C0(_, _, _, _, _, _, _, _, _, _, _, h4061, _, _, _, _) => {
            h4061
        }
    }, match v0.clone() {
        Adt160::C0(_, _, _, _, _, _, _, _, _, _, _, _, h4062, _, _, _) => {
            h4062
        }
    }, match v0.clone() {
        Adt160::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h4063, _, _) => {
            h4063
        }
    }, match v0.clone() {
        Adt160::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h4064, _) => {
            h4064
        }
    }, match v0.clone() {
        Adt160::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h4065) => {
            h4065
        }
    }, f869(f894(match v0.clone() {
        Adt160::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h4066, _) => {
            h4066
        }
    }), f869(match v0.clone() {
        Adt160::C0(_, _, _, _, _, _, _, _, _, h4067, _, _, _, _, _, _) => {
            h4067
        }
    }, f869(f895(match v0.clone() {
        Adt160::C0(_, h4068, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h4068
        }
    }), match v0 {
        Adt160::C0(h4069, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h4069
        }
    }))))
}

pub fn f889(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a4072 = {
        let a4070 = v3;
        let a4071 = 8u64;
        nat_add(a4070, a4071)?
    };
    let a4073 = v2;
    if nat_eq(a4072, a4073) {
        Ok(f896({
            let a4076 = {
                let a4074 = v1;
                let a4075 = 8u64;
                nat_mul(a4074, a4075)?
            };
            let a4077 = 4294967296u64;
            let a4078 = 0u64;
            nat_quot(a4076, a4077, a4078)
        }))
    } else {
        let a4081 = {
            let a4079 = v3;
            let a4080 = 4u64;
            nat_add(a4079, a4080)?
        };
        let a4082 = v2;
        if nat_eq(a4081, a4082) {
            Ok(f896({
                let a4085 = {
                    let a4083 = v1;
                    let a4084 = 8u64;
                    nat_mul(a4083, a4084)?
                };
                let a4086 = 4294967296u64;
                let a4087 = 0u64;
                nat_rem(a4085, a4086, a4087)
            }))
        } else {
            Ok(f880(f892(v0.clone(), v1, v3), f892(v0.clone(), v1, {
                let a4088 = v3;
                let a4089 = 1u64;
                nat_add(a4088, a4089)?
            }), f892(v0.clone(), v1, {
                let a4090 = v3;
                let a4091 = 2u64;
                nat_add(a4090, a4091)?
            }), f892(v0, v1, {
                let a4092 = v3;
                let a4093 = 3u64;
                nat_add(a4092, a4093)?
            })))
        }
    }
}

pub fn f890(v0: u32, v1: u32) -> u32 {
    let m4096 = {
        let a4094 = v0;
        let a4095 = v1;
        fixed_u32::shift_left(a4094, a4095)
    };
    match m4096 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f891(v0: u64, v1: bool) -> u32 {
    if v1 {
        f896({
            let a4097 = v0;
            let a4098 = 4294967296u64;
            let a4099 = 0u64;
            nat_quot(a4097, a4098, a4099)
        })
    } else {
        f896({
            let a4100 = v0;
            let a4101 = 4294967296u64;
            let a4102 = 0u64;
            nat_rem(a4100, a4101, a4102)
        })
    }
}

pub fn f892(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a4103 = v2;
    let a4104 = v1;
    if nat_lt(a4103, a4104) {
        let m4107 = {
            let a4105 = v0;
            let a4106 = v2;
            index_bytes(a4105, a4106)
        };
        match m4107 {
            None => {
                0u32
            }
            Some(v3) => {
                let m4109 = {
                    let a4108 = v3;
                    fixed_u32::convert(i128::from(a4108))
                };
                match m4109 {
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
        let a4110 = v2;
        let a4111 = v1;
        if nat_eq(a4110, a4111) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f893(v0: u32, v1: u32, v2: u32) -> u32 {
    let a4112 = f872(v0, v1);
    let a4113 = f890(v0, v2);
    fixed_u32::bit_or(a4112, a4113)
}

pub fn f894(v0: u32) -> u32 {
    let a4116 = {
        let a4114 = f893(v0, 17u32, 15u32);
        let a4115 = f893(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a4114, a4115)
    };
    let a4117 = f872(v0, 10u32);
    fixed_u32::bit_xor(a4116, a4117)
}

pub fn f895(v0: u32) -> u32 {
    let a4120 = {
        let a4118 = f893(v0, 7u32, 25u32);
        let a4119 = f893(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a4118, a4119)
    };
    let a4121 = f872(v0, 3u32);
    fixed_u32::bit_xor(a4120, a4121)
}

pub fn f896(v0: u64) -> u32 {
    let (_, h4122) = {
        let (_, h4123) = f897(Fn9::F898, 32u64, (v0, (1u32, 0u32)));
        h4123
    };
    h4122
}

pub fn f897(v0: Fn9, p4127: u64, p4128: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m4126 = (p4127, p4128);
    {
        let mut m4126 = m4126;
        loop {
            let step = {
                let (v1, v2) = m4126;
                let m4124 = v1;
                if m4124 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m4124 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c4125: Fn9 = v0.clone();
                        c4125.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m4126 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f898(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h4129, _) = v0;
        h4129
    };
    let v2: u32 = {
        let (h4130, _) = {
            let (_, h4131) = v0;
            h4131
        };
        h4130
    };
    let v3: u32 = {
        let (_, h4132) = {
            let (_, h4133) = v0;
            h4133
        };
        h4132
    };
    ({
        let a4134 = v1;
        let a4135 = 2u64;
        let a4136 = 0u64;
        nat_quot(a4134, a4135, a4136)
    }, (f890(v2, 1u32), {
        let a4140 = {
            let a4137 = v1;
            let a4138 = 2u64;
            let a4139 = 0u64;
            nat_rem(a4137, a4138, a4139)
        };
        let a4141 = 1u64;
        if nat_eq(a4140, a4141) {
            let a4142 = v3;
            let a4143 = v2;
            fixed_u32::bit_or(a4142, a4143)
        } else {
            v3
        }
    }))
}

pub fn run(p0: Bytes) -> R<List<u64>> {
    f0(p0)
}
