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
use core::str;

use std::mem;

use std::rc::Rc;

#[derive(Clone)]
pub struct Str(Rc<str>);

impl Str {
    pub fn lit(text: &str) -> Str { Str(Rc::from(text)) }
    pub fn text(&self) -> &str { &self.0 }
    pub fn length(&self) -> u64 { chars(&self.0) }
    pub fn utf8_encode(&self) -> Bytes { tick(self.0.len() as u64); Bytes::lit(self.0.as_bytes()) }
    pub fn split_exact(&self, delimiter: &Str, maximum: u32) -> Option<List<Str>> { if delimiter.0.is_empty() { return None; }; chars(&self.0); let mut fields: Vec<Str> = Vec::new(); for piece in self.0.split(&*delimiter.0) { fields.push(Str::lit(piece)); }; if fields.len() as u128 > u128::from(maximum) { return None; }; Some(List::onto(fields, List::nil())) }
    pub fn join(texts: List<Str>, delimiter: &Str) -> Str { let mut out = String::new(); let mut first = true; let mut cursor = &texts; while let Repr::Cell(cell) = &cursor.0 { tick(1); if !first { chars(&delimiter.0); out.push_str(&delimiter.0); }; first = false; chars(&cell.head.0); out.push_str(&cell.head.0); cursor = &cell.tail; }; match &cursor.0 { Repr::Run { buffer, len } => { for text in buffer[..*len].iter().rev() { tick(1); if !first { chars(&delimiter.0); out.push_str(&delimiter.0); }; first = false; chars(&text.0); out.push_str(&text.0); } }, _ => { }, }; Str::lit(&out) }
}

#[derive(Clone)]
pub struct Bytes(Rc<[u8]>);

impl Bytes {
    pub fn lit(octets: &[u8]) -> Bytes { Bytes(Rc::from(octets)) }
    pub fn octets(&self) -> &[u8] { &self.0 }
    pub fn append(a: Bytes, b: Bytes) -> Bytes { tick((a.0.len() + b.0.len()) as u64); let mut out = Vec::with_capacity(a.0.len() + b.0.len()); out.extend_from_slice(&a.0); out.extend_from_slice(&b.0); Bytes(Rc::from(out)) }
    pub fn length(&self) -> u64 { self.0.len() as u64 }
    pub fn index(&self, i: u64) -> Option<u8> { match usize::try_from(i) { Ok(at) => self.0.get(at).copied(), Err(_) => None, } }
    pub fn slice(&self, start: u64, count: u64) -> Option<Bytes> { let end = u128::from(start) + u128::from(count); if end > self.0.len() as u128 { return None; }; tick(count); Some(Bytes::lit(&self.0[start as usize..end as usize])) }
    pub fn utf8_decode(&self) -> Option<Str> { tick(self.0.len() as u64); match str::from_utf8(&self.0) { Ok(text) => Some(Str::lit(text)), Err(_) => None, } }
    pub fn compare(&self, other: &Bytes) -> Ordering { tick(1 + self.0.len().min(other.0.len()) as u64); self.0.cmp(&other.0) }
}

pub struct Node<T> { head: T, tail: List<T> }

pub enum Repr<T> { Nil, Run { buffer: Rc<Vec<T>>, len: usize }, Cell(Rc<Node<T>>) }

pub struct List<T>(Repr<T>);

impl<T> Clone for List<T> {
    fn clone(&self) -> Self { List(match &self.0 { Repr::Nil => Repr::Nil, Repr::Run { buffer, len } => Repr::Run { buffer: buffer.clone(), len: *len }, Repr::Cell(cell) => Repr::Cell(cell.clone()), }) }
}

impl<T> List<T> {
    fn take(&mut self) -> Repr<T> { mem::replace(&mut self.0, Repr::Nil) }
}

impl<T> Drop for Node<T> {
    fn drop(&mut self) { let mut next = match self.tail.take() { Repr::Cell(cell) => Some(cell), _ => None, }; while let Some(cell) = next { match Rc::try_unwrap(cell) { Ok(mut node) => next = match node.tail.take() { Repr::Cell(cell) => Some(cell), _ => None, }, Err(_) => break, } } }
}

impl<T: Clone> List<T> {
    pub fn nil() -> Self { List(Repr::Nil) }
    pub fn cons(head: T, mut tail: List<T>) -> Self { List(match tail.take() { Repr::Nil => { let mut buffer = Vec::with_capacity(16); buffer.push(head); Repr::Run { buffer: Rc::new(buffer), len: 1 } }, Repr::Run { mut buffer, len } => { match Rc::get_mut(&mut buffer) { Some(cells) => { cells.truncate(len); cells.push(head); Repr::Run { buffer, len: len + 1 } }, None => Repr::Cell(Rc::new(Node { head, tail: List(Repr::Run { buffer, len }) })), } }, rest => Repr::Cell(Rc::new(Node { head, tail: List(rest) })), }) }
    pub fn uncons(&self) -> Option<(T, List<T>)> { match &self.0 { Repr::Nil => None, Repr::Run { buffer, len } => { let head = buffer[len - 1].clone(); Some((head, List(if *len == 1 { Repr::Nil } else { Repr::Run { buffer: buffer.clone(), len: len - 1 } }))) }, Repr::Cell(cell) => Some((cell.head.clone(), cell.tail.clone())), } }
    pub fn length(&self) -> u64 { let mut count = 0; let mut cursor = self; while let Repr::Cell(cell) = &cursor.0 { tick(1); count += 1; cursor = &cell.tail; }; match &cursor.0 { Repr::Run { buffer, len } => { for _ in buffer[..*len].iter().rev() { tick(1); count += 1; } }, _ => { }, }; count }
    pub fn append(a: List<T>, b: List<T>) -> List<T> { List::onto(a.items(), b) }
    pub fn index(&self, i: u64) -> Option<T> { let mut position = 0; let mut cursor = self; while let Repr::Cell(cell) = &cursor.0 { tick(1); if position == i { return Some(cell.head.clone()); }; position += 1; cursor = &cell.tail; }; match &cursor.0 { Repr::Run { buffer, len } => { for item in buffer[..*len].iter().rev() { tick(1); if position == i { return Some(item.clone()); }; position += 1; } }, _ => { }, }; None }
    pub fn slice(&self, start: u64, count: u64) -> Option<List<T>> { let end = u128::from(start) + u128::from(count); let mut part = Vec::new(); let mut position: u128 = 0; let mut cursor = self; while let Repr::Cell(cell) = &cursor.0 { tick(1); if position >= end { return Some(List::onto(part, List::nil())); }; if position >= u128::from(start) { part.push(cell.head.clone()); }; position += 1; cursor = &cell.tail; }; match &cursor.0 { Repr::Run { buffer, len } => { for item in buffer[..*len].iter().rev() { tick(1); if position >= end { return Some(List::onto(part, List::nil())); }; if position >= u128::from(start) { part.push(item.clone()); }; position += 1; } }, _ => { }, }; if position < end { None } else { Some(List::onto(part, List::nil())) } }
    fn items(&self) -> Vec<T> { let mut out = Vec::new(); let mut cursor = self; while let Repr::Cell(cell) = &cursor.0 { tick(1); out.push(cell.head.clone()); cursor = &cell.tail; }; match &cursor.0 { Repr::Run { buffer, len } => { for item in buffer[..*len].iter().rev() { tick(1); out.push(item.clone()); } }, _ => { }, }; out }
    fn onto(mut items: Vec<T>, tail: List<T>) -> List<T> { let mut out = tail; while let Some(item) = items.pop() { tick(1); out = List::cons(item, out); }; out }
}

pub fn chars(text: &str) -> u64 { let mut count = 0; for _ in text.chars() { tick(1); count += 1; }; count }

impl Same for Str {
    fn same(&self, other: &Self) -> bool { let mut left = self.0.chars(); let mut right = other.0.chars(); loop { tick(1); match (left.next(), right.next()) { (None, None) => return true, (a, b) if a != b => return false, _ => { }, } } }
}

impl Same for Bytes {
    fn same(&self, other: &Self) -> bool { tick(1 + self.0.len().min(other.0.len()) as u64); self.0 == other.0 }
}

impl Key for Str {
    fn key(&self, other: &Self) -> Ordering { let mut left = self.0.chars(); let mut right = other.0.chars(); loop { tick(1); match (left.next(), right.next()) { (None, None) => return Ordering::Equal, (None, Some(_)) => return Ordering::Less, (Some(_), None) => return Ordering::Greater, (Some(a), Some(b)) => match a.cmp(&b) { Ordering::Equal => { }, decided => return decided, }, } } }
}

pub fn canonical_decimal(text: &str) -> Option<Option<i128>> { let digits = match text.strip_prefix('-') { Some(rest) => rest, None => text, }; let mut canonical = !digits.is_empty() && (digits == "0" || !digits.starts_with('0')) && text != "-0"; for c in digits.chars() { tick(1); canonical = canonical && c.is_ascii_digit(); }; if !canonical { return None; }; Some(text.parse::<i128>().ok()) }

pub fn parse_int(text: Str) -> R<Option<i64>> { match canonical_decimal(&text.0) { None => Ok(None), Some(Some(n)) => match i64::try_from(n) { Ok(value) => Ok(Some(value)), Err(_) => Err(Overflow), }, Some(None) => Err(Overflow), } }

pub fn format_u8(a: u8) -> Str { let text = a.to_string(); tick(text.len() as u64); Str::lit(&text) }

pub fn parse_u8(text: Str) -> Option<u8> { match canonical_decimal(&text.0) { Some(Some(n)) => u8::try_from(n).ok(), _ => None, } }

pub fn format_u16(a: u16) -> Str { let text = a.to_string(); tick(text.len() as u64); Str::lit(&text) }

pub fn parse_u16(text: Str) -> Option<u16> { match canonical_decimal(&text.0) { Some(Some(n)) => u16::try_from(n).ok(), _ => None, } }

pub fn format_u32(a: u32) -> Str { let text = a.to_string(); tick(text.len() as u64); Str::lit(&text) }

pub fn parse_u32(text: Str) -> Option<u32> { match canonical_decimal(&text.0) { Some(Some(n)) => u32::try_from(n).ok(), _ => None, } }

pub fn format_u64(a: u64) -> Str { let text = a.to_string(); tick(text.len() as u64); Str::lit(&text) }

pub fn parse_u64(text: Str) -> Option<u64> { match canonical_decimal(&text.0) { Some(Some(n)) => u64::try_from(n).ok(), _ => None, } }

pub fn format_i8(a: i8) -> Str { let text = a.to_string(); tick(text.len() as u64); Str::lit(&text) }

pub fn parse_i8(text: Str) -> Option<i8> { match canonical_decimal(&text.0) { Some(Some(n)) => i8::try_from(n).ok(), _ => None, } }

pub fn format_i16(a: i16) -> Str { let text = a.to_string(); tick(text.len() as u64); Str::lit(&text) }

pub fn parse_i16(text: Str) -> Option<i16> { match canonical_decimal(&text.0) { Some(Some(n)) => i16::try_from(n).ok(), _ => None, } }

pub fn format_i32(a: i32) -> Str { let text = a.to_string(); tick(text.len() as u64); Str::lit(&text) }

pub fn parse_i32(text: Str) -> Option<i32> { match canonical_decimal(&text.0) { Some(Some(n)) => i32::try_from(n).ok(), _ => None, } }

pub fn format_i64(a: i64) -> Str { let text = a.to_string(); tick(text.len() as u64); Str::lit(&text) }

pub fn parse_i64(text: Str) -> Option<i64> { match canonical_decimal(&text.0) { Some(Some(n)) => i64::try_from(n).ok(), _ => None, } }

pub fn append_list<T: Clone>(p0: List<T>, p1: List<T>) -> List<T> {
    List::append(p0, p1)
}

pub fn length_list<T: Clone>(p0: List<T>) -> u64 {
    List::length(&p0)
}

pub fn index_list<T: Clone>(p0: List<T>, p1: u64) -> Option<T> {
    List::index(&p0, p1)
}

pub fn slice_list<T: Clone>(p0: List<T>, p1: u64, p2: u64) -> Option<List<T>> {
    List::slice(&p0, p1, p2)
}

pub fn append_bytes(p0: Bytes, p1: Bytes) -> Bytes {
    Bytes::append(p0, p1)
}

pub fn length_bytes(p0: Bytes) -> u64 {
    Bytes::length(&p0)
}

pub fn length_string(p0: Str) -> u64 {
    Str::length(&p0)
}

pub fn index_bytes(p0: Bytes, p1: u64) -> Option<u8> {
    Bytes::index(&p0, p1)
}

pub fn slice_bytes(p0: Bytes, p1: u64, p2: u64) -> Option<Bytes> {
    Bytes::slice(&p0, p1, p2)
}

pub fn utf8_encode(p0: Str) -> Bytes {
    Str::utf8_encode(&p0)
}

pub fn utf8_decode(p0: Bytes) -> Option<Str> {
    Bytes::utf8_decode(&p0)
}

pub fn compare_bytes(p0: Bytes, p1: Bytes) -> Ordering {
    Bytes::compare(&p0, &p1)
}

pub fn split_exact(p0: Str, p1: Str, p2: u32) -> Option<List<Str>> {
    Str::split_exact(&p0, &p1, p2)
}

pub fn join(p0: List<Str>, p1: Str) -> Str {
    Str::join(p0, &p1)
}

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
    Ok(f24(f50(v0, v1, List::<(u64, u64)>::nil(), List::<u64>::nil())?, List::<u64>::nil()))
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

pub fn f50(v0: (Bytes, Bytes), p273: List<u64>, p274: List<(u64, u64)>, p275: List<u64>) -> R<List<u64>> {
    let m272 = (p273, (p274, p275));
    {
        let mut m272 = m272;
        loop {
            let step = {
                let (v1, (v2, v3)) = m272;
                let m269 = v1;
                match m269.uncons() {
                    None => {
                        Err::<(List<u64>, (List<(u64, u64)>, List<u64>)), R<List<u64>>>(Ok(f56(f57(v2)?, v3)))
                    }
                    Some((v4, v5)) => {
                        let v6: u64 = f35(v0.clone(), v4)?;
                        let a270 = v6;
                        let a271 = 0u64;
                        if nat_eq(a270, a271) {
                            Ok::<(List<u64>, (List<(u64, u64)>, List<u64>)), R<List<u64>>>((v5, (List::<(u64, u64)>::nil(), List::cons(v4, f56(f57(v2)?, v3)))))
                        } else {
                            Ok::<(List<u64>, (List<(u64, u64)>, List<u64>)), R<List<u64>>>((v5, (List::cons((v6, v4), v2), v3)))
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

pub fn f51(v0: (Bytes, Bytes), p278: u64, p279: List<u64>) -> R<List<u64>> {
    let m277 = (p278, p279);
    {
        let mut m277 = m277;
        loop {
            let step = {
                let (v1, v2) = m277;
                let m276 = v1;
                if m276 == 0 {
                    Err::<(u64, List<u64>), R<List<u64>>>(Ok(v2))
                } else {
                    let v3 = m276 - 1;
                    Ok::<(u64, List<u64>), R<List<u64>>>((v3, f58(v0.clone(), v2)?))
                }
            };
            match step {
                Ok(next) => { m277 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f52() -> u64 {
    3u64
}

pub fn f53(p297: u64, v1: (Bytes, Bytes), v2: u64, v3: u64, v4: u64, v5: u64, p298: u64, p299: u64) -> R<u64> {
    let m296 = (p297, (p298, p299));
    {
        let mut m296 = m296;
        loop {
            let step = {
                let (v0, (v6, v7)) = m296;
                let m280 = v0;
                if m280 == 0 {
                    Err::<(u64, (u64, u64)), R<u64>>(Ok(v6))
                } else {
                    let v8 = m280 - 1;
                    let a283 = {
                        let a281 = v7;
                        let a282 = v6;
                        nat_sub(a281, a282)
                    };
                    let a284 = 2u64;
                    if nat_lt(a283, a284) {
                        Err::<(u64, (u64, u64)), R<u64>>(Ok(v6))
                    } else {
                        let v9: u64 = {
                            let a287 = {
                                let a285 = v6;
                                let a286 = v7;
                                nat_add(a285, a286)?
                            };
                            let a288 = 2u64;
                            let a289 = 0u64;
                            nat_quot(a287, a288, a289)
                        };
                        let a294 = v5;
                        let a295 = f44(v1.clone(), {
                            let a292 = v2;
                            let a293 = {
                                let a290 = v9;
                                let a291 = v3;
                                nat_mul(a290, a291)?
                            };
                            nat_add(a292, a293)?
                        }, v4, 0u64)?;
                        if nat_lt(a294, a295) {
                            Ok::<(u64, (u64, u64)), R<u64>>((v8, (v6, v9)))
                        } else {
                            Ok::<(u64, (u64, u64)), R<u64>>((v8, (v9, v7)))
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

pub fn f54() -> u64 {
    16384u64
}

pub fn f55(v0: u64) -> bool {
    let a300 = v0;
    let a301 = 44032u64;
    if nat_lt(a300, a301) {
        false
    } else {
        let a302 = v0;
        let a303 = 55204u64;
        nat_lt(a302, a303)
    }
}

pub fn f56(p307: List<(u64, u64)>, p308: List<u64>) -> List<u64> {
    let m306 = (p307, p308);
    {
        let mut m306 = m306;
        loop {
            let step = {
                let (v0, v1) = m306;
                let m304 = v0;
                match m304.uncons() {
                    None => {
                        Err::<(List<(u64, u64)>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(u64, u64)>, List<u64>), List<u64>>((v3, List::cons({
                            let (_, h305) = v2;
                            h305
                        }, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m306 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f57(v0: List<(u64, u64)>) -> R<List<(u64, u64)>> {
    let v1: u64 = f59(v0.clone(), 0u64)?;
    Ok(f60({
        let a309 = v1;
        let a310 = 1u64;
        nat_add(a309, a310)?
    }, {
        let a311 = v1;
        let a312 = 1u64;
        nat_add(a311, a312)?
    }, f61(v0, List::<List<(u64, u64)>>::nil())))
}

pub fn f58(v0: (Bytes, Bytes), v1: List<u64>) -> R<List<u64>> {
    Ok(f24(f62(v0, v1, List::<u64>::nil())?, List::<u64>::nil()))
}

pub fn f59(p317: List<(u64, u64)>, p318: u64) -> R<u64> {
    let m316 = (p317, p318);
    {
        let mut m316 = m316;
        loop {
            let step = {
                let (v0, v1) = m316;
                let m313 = v0;
                match m313.uncons() {
                    None => {
                        Err::<(List<(u64, u64)>, u64), R<u64>>(Ok(v1))
                    }
                    Some((_, v3)) => {
                        Ok::<(List<(u64, u64)>, u64), R<u64>>((v3, {
                            let a314 = v1;
                            let a315 = 1u64;
                            nat_add(a314, a315)?
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m316 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f60(p324: u64, v1: u64, p325: List<List<(u64, u64)>>) -> List<(u64, u64)> {
    let m323 = (p324, p325);
    {
        let mut m323 = m323;
        loop {
            let step = {
                let (v0, v2) = m323;
                let m319 = v0;
                if m319 == 0 {
                    let m320 = v2;
                    match m320.uncons() {
                        None => {
                            Err::<(u64, List<List<(u64, u64)>>), List<(u64, u64)>>(List::<(u64, u64)>::nil())
                        }
                        Some((v3, _)) => {
                            Err::<(u64, List<List<(u64, u64)>>), List<(u64, u64)>>(v3)
                        }
                    }
                } else {
                    let v5 = m319 - 1;
                    let m321 = v2.clone();
                    match m321.uncons() {
                        None => {
                            Err::<(u64, List<List<(u64, u64)>>), List<(u64, u64)>>(List::<(u64, u64)>::nil())
                        }
                        Some((v6, v7)) => {
                            let m322 = v7;
                            match m322.uncons() {
                                None => {
                                    Err::<(u64, List<List<(u64, u64)>>), List<(u64, u64)>>(v6)
                                }
                                Some((_, _)) => {
                                    Ok::<(u64, List<List<(u64, u64)>>), List<(u64, u64)>>((v5, f63(v1, v2, List::<List<(u64, u64)>>::nil(), List::<List<(u64, u64)>>::nil())))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m323 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f61(p328: List<(u64, u64)>, p329: List<List<(u64, u64)>>) -> List<List<(u64, u64)>> {
    let m327 = (p328, p329);
    {
        let mut m327 = m327;
        loop {
            let step = {
                let (v0, v1) = m327;
                let m326 = v0;
                match m326.uncons() {
                    None => {
                        Err::<(List<(u64, u64)>, List<List<(u64, u64)>>), List<List<(u64, u64)>>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(u64, u64)>, List<List<(u64, u64)>>), List<List<(u64, u64)>>>((v3, List::cons(List::cons(v2, List::<(u64, u64)>::nil()), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m327 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f62(v0: (Bytes, Bytes), p332: List<u64>, p333: List<u64>) -> R<List<u64>> {
    let m331 = (p332, p333);
    {
        let mut m331 = m331;
        loop {
            let step = {
                let (v1, v2) = m331;
                let m330 = v1;
                match m330.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), R<List<u64>>>(Ok(v2))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<u64>, List<u64>), R<List<u64>>>((v4, f24(f64(v0.clone(), v3)?, v2)))
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

pub fn f63(v0: u64, p338: List<List<(u64, u64)>>, p339: List<List<(u64, u64)>>, p340: List<List<(u64, u64)>>) -> List<List<(u64, u64)>> {
    let m337 = (p338, (p339, p340));
    {
        let mut m337 = m337;
        loop {
            let step = {
                let (v1, (v2, v3)) = m337;
                let m334 = v1;
                match m334.uncons() {
                    None => {
                        let m335 = v2;
                        match m335.uncons() {
                            None => {
                                Err::<(List<List<(u64, u64)>>, (List<List<(u64, u64)>>, List<List<(u64, u64)>>)), List<List<(u64, u64)>>>(f65(v3, List::<List<(u64, u64)>>::nil()))
                            }
                            Some((v4, _)) => {
                                Err::<(List<List<(u64, u64)>>, (List<List<(u64, u64)>>, List<List<(u64, u64)>>)), List<List<(u64, u64)>>>(f65(List::cons(v4, v3), List::<List<(u64, u64)>>::nil()))
                            }
                        }
                    }
                    Some((v6, v7)) => {
                        let m336 = v2;
                        match m336.uncons() {
                            None => {
                                Ok::<(List<List<(u64, u64)>>, (List<List<(u64, u64)>>, List<List<(u64, u64)>>)), List<List<(u64, u64)>>>((v7, (List::cons(v6, List::<List<(u64, u64)>>::nil()), v3)))
                            }
                            Some((v8, _)) => {
                                Ok::<(List<List<(u64, u64)>>, (List<List<(u64, u64)>>, List<List<(u64, u64)>>)), List<List<(u64, u64)>>>((v7, (List::<List<(u64, u64)>>::nil(), List::cons(f66(v0, v8, v6, List::<(u64, u64)>::nil()), v3))))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m337 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f64(v0: (Bytes, Bytes), v1: u64) -> R<List<u64>> {
    if f55(v1) {
        f67(v1)
    } else {
        let v2: u64 = f68(v0.clone(), v1)?;
        let a341 = v2;
        let a342 = 0u64;
        if nat_eq(a341, a342) {
            let m343 = f69(v0, v1)?;
            match m343.uncons() {
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

pub fn f65(p346: List<List<(u64, u64)>>, p347: List<List<(u64, u64)>>) -> List<List<(u64, u64)>> {
    let m345 = (p346, p347);
    {
        let mut m345 = m345;
        loop {
            let step = {
                let (v0, v1) = m345;
                let m344 = v0;
                match m344.uncons() {
                    None => {
                        Err::<(List<List<(u64, u64)>>, List<List<(u64, u64)>>), List<List<(u64, u64)>>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<(u64, u64)>>, List<List<(u64, u64)>>), List<List<(u64, u64)>>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m345 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f66(p356: u64, p357: List<(u64, u64)>, p358: List<(u64, u64)>, p359: List<(u64, u64)>) -> List<(u64, u64)> {
    let m355 = (p356, (p357, (p358, p359)));
    {
        let mut m355 = m355;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m355;
                let m348 = v0;
                if m348 == 0 {
                    Err::<(u64, (List<(u64, u64)>, (List<(u64, u64)>, List<(u64, u64)>))), List<(u64, u64)>>(f70(v3, v1))
                } else {
                    let v4 = m348 - 1;
                    let m349 = v1.clone();
                    match m349.uncons() {
                        None => {
                            Err::<(u64, (List<(u64, u64)>, (List<(u64, u64)>, List<(u64, u64)>))), List<(u64, u64)>>(f70(v3, v2))
                        }
                        Some((v5, v6)) => {
                            let m350 = v2.clone();
                            match m350.uncons() {
                                None => {
                                    Err::<(u64, (List<(u64, u64)>, (List<(u64, u64)>, List<(u64, u64)>))), List<(u64, u64)>>(f70(v3, v1))
                                }
                                Some((v7, v8)) => {
                                    let a353 = {
                                        let (h351, _) = v7;
                                        h351
                                    };
                                    let a354 = {
                                        let (h352, _) = v5;
                                        h352
                                    };
                                    if nat_lt(a353, a354) {
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
                Ok(next) => { m355 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f67(v0: u64) -> R<List<u64>> {
    let v1: u64 = {
        let a360 = v0;
        let a361 = 44032u64;
        nat_sub(a360, a361)
    };
    let v2: u64 = {
        let a365 = 4352u64;
        let a366 = {
            let a362 = v1;
            let a363 = 588u64;
            let a364 = 0u64;
            nat_quot(a362, a363, a364)
        };
        nat_add(a365, a366)?
    };
    let v3: u64 = {
        let a373 = 4449u64;
        let a374 = {
            let a370 = {
                let a367 = v1;
                let a368 = 588u64;
                let a369 = 0u64;
                nat_rem(a367, a368, a369)
            };
            let a371 = 28u64;
            let a372 = 0u64;
            nat_quot(a370, a371, a372)
        };
        nat_add(a373, a374)?
    };
    let v4: u64 = {
        let a378 = 4519u64;
        let a379 = {
            let a375 = v1;
            let a376 = 28u64;
            let a377 = 0u64;
            nat_rem(a375, a376, a377)
        };
        nat_add(a378, a379)?
    };
    let a380 = v4;
    let a381 = 4519u64;
    if nat_eq(a380, a381) {
        Ok(List::cons(v2, List::cons(v3, List::<u64>::nil())))
    } else {
        Ok(List::cons(v2, List::cons(v3, List::cons(v4, List::<u64>::nil()))))
    }
}

pub fn f68(v0: (Bytes, Bytes), v1: u64) -> R<u64> {
    let a382 = v1;
    let a383 = f21();
    if nat_lt(a382, a383) {
        Ok(0u64)
    } else {
        let v2: u64 = f41(v0.clone(), f42(v0.clone(), 2u64)?, 6u64, f43(v0.clone(), 2u64)?, 3u64, v1)?;
        let a384 = f44(v0.clone(), v2, 3u64, 0u64)?;
        let a385 = v1;
        if nat_eq(a384, a385) {
            f44(v0, {
                let a386 = v2;
                let a387 = 3u64;
                nat_add(a386, a387)?
            }, 3u64, 0u64)
        } else {
            Ok(0u64)
        }
    }
}

pub fn f69(v0: (Bytes, Bytes), v1: u64) -> R<List<u64>> {
    let a388 = v1;
    let a389 = f71();
    if nat_lt(a388, a389) {
        Ok(List::<u64>::nil())
    } else {
        let v2: u64 = f41(v0.clone(), f42(v0.clone(), 3u64)?, 9u64, f43(v0.clone(), 3u64)?, 3u64, v1)?;
        let a390 = f44(v0.clone(), v2, 3u64, 0u64)?;
        let a391 = v1;
        if nat_eq(a390, a391) {
            Ok(List::cons(f44(v0.clone(), {
                let a392 = v2;
                let a393 = 3u64;
                nat_add(a392, a393)?
            }, 3u64, 0u64)?, List::cons(f44(v0, {
                let a394 = v2;
                let a395 = 6u64;
                nat_add(a394, a395)?
            }, 3u64, 0u64)?, List::<u64>::nil())))
        } else {
            Ok(List::<u64>::nil())
        }
    }
}

pub fn f70(p398: List<(u64, u64)>, p399: List<(u64, u64)>) -> List<(u64, u64)> {
    let m397 = (p398, p399);
    {
        let mut m397 = m397;
        loop {
            let step = {
                let (v0, v1) = m397;
                let m396 = v0;
                match m396.uncons() {
                    None => {
                        Err::<(List<(u64, u64)>, List<(u64, u64)>), List<(u64, u64)>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(u64, u64)>, List<(u64, u64)>), List<(u64, u64)>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m397 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f71() -> u64 {
    192u64
}

pub fn run(p0: Bytes, p1: Bytes, p2: List<Bytes>, p3: Bytes) -> R<List<u64>> {
    f0(p0, p1, p2, p3)
}
