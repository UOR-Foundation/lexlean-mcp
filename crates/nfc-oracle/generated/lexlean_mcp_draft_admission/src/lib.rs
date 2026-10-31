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
    F34(Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: (List<u64>, u64)) -> R<(List<u64>, u64)> {
        match self {
            Fn0::F34(k0) => f34(k0.clone(), p0),
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
    Ok(f11(Adt1::C10, f14(f15(v0, List::<List<u64>>::nil())?)?))
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

pub fn f14(v0: List<List<u64>>) -> R<bool> {
    Ok(f23(f24(v0)?))
}

pub fn f15(p18: List<(Bytes, Bytes)>, p19: List<List<u64>>) -> R<List<List<u64>>> {
    let m17 = (p18, p19);
    {
        let mut m17 = m17;
        loop {
            let step = {
                let (v0, v1) = m17;
                let m15 = v0;
                match m15.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<List<u64>>), R<List<List<u64>>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<List<u64>>), R<List<List<u64>>>>((v3, List::cons(f20({
                            let (h16, _) = v2;
                            h16
                        })?, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m17 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f16(v0: Bytes) -> Adt0 {
    let m21 = {
        let a20 = v0;
        utf8_decode(a20)
    };
    match m21 {
        None => {
            Adt0::C1(Adt1::C0)
        }
        Some(_) => {
            Adt0::C0
        }
    }
}

pub fn f17(v0: (Bytes, Bytes), v1: Bytes) -> R<Adt0> {
    if f25(v0, f26(f20(v1)?)?)? {
        Ok(Adt0::C0)
    } else {
        Ok(Adt0::C1(Adt1::C1))
    }
}

pub fn f18(v0: List<List<u64>>, v1: List<u64>) -> Adt0 {
    f5(f27(v1.clone()), f5(f28(f29(v1.clone())), f5(f30(v0, v1.clone()), f31(v1))))
}

pub fn f19(p24: List<Bytes>, p25: List<List<u64>>) -> R<List<List<u64>>> {
    let m23 = (p24, p25);
    {
        let mut m23 = m23;
        loop {
            let step = {
                let (v0, v1) = m23;
                let m22 = v0;
                match m22.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<List<u64>>), R<List<List<u64>>>>(Ok(f32(v1, List::<List<u64>>::nil())))
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<List<u64>>), R<List<List<u64>>>>((v3, List::cons(f20(v2)?, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m23 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f20(v0: Bytes) -> R<List<u64>> {
    let (h26, _) = f33(Fn0::F34(v0.clone()), {
        let a27 = v0.clone();
        length_bytes(a27)
    }, (List::<u64>::nil(), {
        let a28 = v0;
        length_bytes(a28)
    }))?;
    Ok(h26)
}

pub fn f21(p33: List<(Bytes, Bytes)>, p34: u64) -> R<u64> {
    let m32 = (p33, p34);
    {
        let mut m32 = m32;
        loop {
            let step = {
                let (v0, v1) = m32;
                let m29 = v0;
                match m29.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, u64), R<u64>>(Ok(v1))
                    }
                    Some((_, v3)) => {
                        Ok::<(List<(Bytes, Bytes)>, u64), R<u64>>((v3, {
                            let a30 = v1;
                            let a31 = 1u64;
                            nat_add(a30, a31)?
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m32 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f22(p41: List<(Bytes, Bytes)>, p42: u64) -> R<u64> {
    let m40 = (p41, p42);
    {
        let mut m40 = m40;
        loop {
            let step = {
                let (v0, v1) = m40;
                let m35 = v0;
                match m35.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, u64), R<u64>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Bytes, Bytes)>, u64), R<u64>>((v3, {
                            let a38 = v1;
                            let a39 = {
                                let a37 = {
                                    let (_, h36) = v2;
                                    h36
                                };
                                length_bytes(a37)
                            };
                            nat_add(a38, a39)?
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m40 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f23(p46: List<List<u64>>) -> bool {
    let m45 = p46;
    {
        let mut m45 = m45;
        loop {
            let step = {
                let v0 = m45;
                let m43 = v0;
                match m43.uncons() {
                    None => {
                        Err::<List<List<u64>>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        let m44 = v2.clone();
                        match m44.uncons() {
                            None => {
                                Err::<List<List<u64>>, bool>(false)
                            }
                            Some((v3, _)) => {
                                if f35(v1, v3) {
                                    Err::<List<List<u64>>, bool>(true)
                                } else {
                                    Ok::<List<List<u64>>, bool>(v2)
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m45 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f24(v0: List<List<u64>>) -> R<List<List<u64>>> {
    let v1: u64 = f36(v0.clone(), 0u64)?;
    Ok(f37({
        let a47 = v1;
        let a48 = 1u64;
        nat_add(a47, a48)?
    }, {
        let a49 = v1;
        let a50 = 1u64;
        nat_add(a49, a50)?
    }, f38(v0, List::<List<List<u64>>>::nil())))
}

pub fn f25(v0: (Bytes, Bytes), v1: List<u64>) -> R<bool> {
    if f39(f40(), v1.clone()) {
        Ok(true)
    } else {
        Ok(f35(v1.clone(), f41(v0, v1)?))
    }
}

pub fn f26(v0: List<u64>) -> R<List<u64>> {
    Ok(f42(f43(v0, 0u64, 0u64, List::<u64>::nil())?, List::<u64>::nil()))
}

pub fn f27(v0: List<u64>) -> Adt0 {
    let m51 = v0;
    match m51.uncons() {
        None => {
            Adt0::C0
        }
        Some((v1, _)) => {
            let a52 = v1;
            let a53 = 47u64;
            if nat_eq(a52, a53) {
                Adt0::C1(Adt1::C2)
            } else {
                Adt0::C0
            }
        }
    }
}

pub fn f28(p57: List<List<u64>>) -> Adt0 {
    let m56 = p57;
    {
        let mut m56 = m56;
        loop {
            let step = {
                let v0 = m56;
                let m54 = v0;
                match m54.uncons() {
                    None => {
                        Err::<List<List<u64>>, Adt0>(Adt0::C0)
                    }
                    Some((v1, v2)) => {
                        let m55 = f44(v1);
                        match m55 {
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
                Ok(next) => { m56 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f29(v0: List<u64>) -> List<List<u64>> {
    f32(f45(v0, List::<u64>::nil(), List::<List<u64>>::nil()), List::<List<u64>>::nil())
}

pub fn f30(p60: List<List<u64>>, v1: List<u64>) -> Adt0 {
    let m59 = p60;
    {
        let mut m59 = m59;
        loop {
            let step = {
                let v0 = m59;
                let m58 = v0;
                match m58.uncons() {
                    None => {
                        Err::<List<List<u64>>, Adt0>(Adt0::C1(Adt1::C6))
                    }
                    Some((v2, v3)) => {
                        if f46(v2, v1.clone()) {
                            Err::<List<List<u64>>, Adt0>(Adt0::C0)
                        } else {
                            Ok::<List<List<u64>>, Adt0>(v3)
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

pub fn f31(v0: List<u64>) -> Adt0 {
    let m61 = f47(v0, List::cons(46u64, List::cons(108u64, List::cons(101u64, List::cons(120u64, List::cons(46u64, List::cons(116u64, List::cons(101u64, List::cons(120u64, List::<u64>::nil())))))))));
    if m61 {
        Adt0::C0
    } else {
        Adt0::C1(Adt1::C7)
    }
}

pub fn f32(p64: List<List<u64>>, p65: List<List<u64>>) -> List<List<u64>> {
    let m63 = (p64, p65);
    {
        let mut m63 = m63;
        loop {
            let step = {
                let (v0, v1) = m63;
                let m62 = v0;
                match m62.uncons() {
                    None => {
                        Err::<(List<List<u64>>, List<List<u64>>), List<List<u64>>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<u64>>, List<List<u64>>), List<List<u64>>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m63 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f33(v0: Fn0, p69: u64, p70: (List<u64>, u64)) -> R<(List<u64>, u64)> {
    let m68 = (p69, p70);
    {
        let mut m68 = m68;
        loop {
            let step = {
                let (v1, v2) = m68;
                let m66 = v1;
                if m66 == 0 {
                    Err::<(u64, (List<u64>, u64)), R<(List<u64>, u64)>>(Ok(v2))
                } else {
                    let v3 = m66 - 1;
                    Ok::<(u64, (List<u64>, u64)), R<(List<u64>, u64)>>((v3, {
                        let c67: Fn0 = v0.clone();
                        c67.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m68 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f34(v0: Bytes, v1: (List<u64>, u64)) -> R<(List<u64>, u64)> {
    Ok((List::cons(f48(v0, {
        let a72 = {
            let (_, h71) = v1.clone();
            h71
        };
        let a73 = 1u64;
        nat_sub(a72, a73)
    })?, {
        let (h74, _) = v1;
        h74
    }), {
        let a76 = {
            let (_, h75) = v1;
            h75
        };
        let a77 = 1u64;
        nat_sub(a76, a77)
    }))
}

pub fn f35(p84: List<u64>, p85: List<u64>) -> bool {
    let m83 = (p84, p85);
    {
        let mut m83 = m83;
        loop {
            let step = {
                let (v0, v1) = m83;
                let m78 = v0;
                match m78.uncons() {
                    None => {
                        let m79 = v1;
                        match m79.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m80 = v1;
                        match m80.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                            Some((v6, v7)) => {
                                let a81 = v4;
                                let a82 = v6;
                                if nat_eq(a81, a82) {
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
                Ok(next) => { m83 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f36(p90: List<List<u64>>, p91: u64) -> R<u64> {
    let m89 = (p90, p91);
    {
        let mut m89 = m89;
        loop {
            let step = {
                let (v0, v1) = m89;
                let m86 = v0;
                match m86.uncons() {
                    None => {
                        Err::<(List<List<u64>>, u64), R<u64>>(Ok(v1))
                    }
                    Some((_, v3)) => {
                        Ok::<(List<List<u64>>, u64), R<u64>>((v3, {
                            let a87 = v1;
                            let a88 = 1u64;
                            nat_add(a87, a88)?
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m89 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f37(p97: u64, v1: u64, p98: List<List<List<u64>>>) -> List<List<u64>> {
    let m96 = (p97, p98);
    {
        let mut m96 = m96;
        loop {
            let step = {
                let (v0, v2) = m96;
                let m92 = v0;
                if m92 == 0 {
                    let m93 = v2;
                    match m93.uncons() {
                        None => {
                            Err::<(u64, List<List<List<u64>>>), List<List<u64>>>(List::<List<u64>>::nil())
                        }
                        Some((v3, _)) => {
                            Err::<(u64, List<List<List<u64>>>), List<List<u64>>>(v3)
                        }
                    }
                } else {
                    let v5 = m92 - 1;
                    let m94 = v2.clone();
                    match m94.uncons() {
                        None => {
                            Err::<(u64, List<List<List<u64>>>), List<List<u64>>>(List::<List<u64>>::nil())
                        }
                        Some((v6, v7)) => {
                            let m95 = v7;
                            match m95.uncons() {
                                None => {
                                    Err::<(u64, List<List<List<u64>>>), List<List<u64>>>(v6)
                                }
                                Some((_, _)) => {
                                    Ok::<(u64, List<List<List<u64>>>), List<List<u64>>>((v5, f49(v1, v2, List::<List<List<u64>>>::nil(), List::<List<List<u64>>>::nil())))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m96 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f38(p101: List<List<u64>>, p102: List<List<List<u64>>>) -> List<List<List<u64>>> {
    let m100 = (p101, p102);
    {
        let mut m100 = m100;
        loop {
            let step = {
                let (v0, v1) = m100;
                let m99 = v0;
                match m99.uncons() {
                    None => {
                        Err::<(List<List<u64>>, List<List<List<u64>>>), List<List<List<u64>>>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<u64>>, List<List<List<u64>>>), List<List<List<u64>>>>((v3, List::cons(List::cons(v2, List::<List<u64>>::nil()), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m100 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f39(v0: u64, p107: List<u64>) -> bool {
    let m106 = p107;
    {
        let mut m106 = m106;
        loop {
            let step = {
                let v1 = m106;
                let m103 = v1;
                match m103.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        let a104 = v2;
                        let a105 = v0;
                        if nat_lt(a104, a105) {
                            Ok::<List<u64>, bool>(v3)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m106 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f40() -> u64 {
    768u64
}

pub fn f41(v0: (Bytes, Bytes), v1: List<u64>) -> R<List<u64>> {
    f50(v0.clone(), f51(v0, v1)?, false, 0u64, List::<u64>::nil(), List::<u64>::nil())
}

pub fn f42(p110: List<u64>, p111: List<u64>) -> List<u64> {
    let m109 = (p110, p111);
    {
        let mut m109 = m109;
        loop {
            let step = {
                let (v0, v1) = m109;
                let m108 = v0;
                match m108.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m109 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f43(p138: List<u64>, p139: u64, p140: u64, p141: List<u64>) -> R<List<u64>> {
    let m137 = (p138, (p139, (p140, p141)));
    {
        let mut m137 = m137;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m137;
                let m112 = v0;
                match m112.uncons() {
                    None => {
                        Err::<(List<u64>, (u64, (u64, List<u64>))), R<List<u64>>>(Ok(v3))
                    }
                    Some((v4, v5)) => {
                        let a113 = v2;
                        let a114 = 0u64;
                        if nat_eq(a113, a114) {
                            let a115 = v4;
                            let a116 = 128u64;
                            if nat_lt(a115, a116) {
                                Ok::<(List<u64>, (u64, (u64, List<u64>))), R<List<u64>>>((v5, (0u64, (0u64, List::cons(v4, v3)))))
                            } else {
                                let a117 = v4;
                                let a118 = 224u64;
                                if nat_lt(a117, a118) {
                                    Ok::<(List<u64>, (u64, (u64, List<u64>))), R<List<u64>>>((v5, ({
                                        let a119 = v4;
                                        let a120 = 192u64;
                                        nat_sub(a119, a120)
                                    }, (1u64, v3))))
                                } else {
                                    let a121 = v4;
                                    let a122 = 240u64;
                                    if nat_lt(a121, a122) {
                                        Ok::<(List<u64>, (u64, (u64, List<u64>))), R<List<u64>>>((v5, ({
                                            let a123 = v4;
                                            let a124 = 224u64;
                                            nat_sub(a123, a124)
                                        }, (2u64, v3))))
                                    } else {
                                        Ok::<(List<u64>, (u64, (u64, List<u64>))), R<List<u64>>>((v5, ({
                                            let a125 = v4;
                                            let a126 = 240u64;
                                            nat_sub(a125, a126)
                                        }, (3u64, v3))))
                                    }
                                }
                            }
                        } else {
                            let v6: u64 = {
                                let a131 = {
                                    let a127 = v1;
                                    let a128 = 64u64;
                                    nat_mul(a127, a128)?
                                };
                                let a132 = {
                                    let a129 = v4;
                                    let a130 = 128u64;
                                    nat_sub(a129, a130)
                                };
                                nat_add(a131, a132)?
                            };
                            let a133 = v2;
                            let a134 = 1u64;
                            if nat_eq(a133, a134) {
                                Ok::<(List<u64>, (u64, (u64, List<u64>))), R<List<u64>>>((v5, (0u64, (0u64, List::cons(v6, v3)))))
                            } else {
                                Ok::<(List<u64>, (u64, (u64, List<u64>))), R<List<u64>>>((v5, (v6, ({
                                    let a135 = v2;
                                    let a136 = 1u64;
                                    nat_sub(a135, a136)
                                }, v3))))
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

pub fn f44(v0: List<u64>) -> Adt0 {
    f52(f35(v0.clone(), List::<u64>::nil()), f35(v0.clone(), List::cons(46u64, List::<u64>::nil())), f35(v0, List::cons(46u64, List::cons(46u64, List::<u64>::nil()))))
}

pub fn f45(p146: List<u64>, p147: List<u64>, p148: List<List<u64>>) -> List<List<u64>> {
    let m145 = (p146, (p147, p148));
    {
        let mut m145 = m145;
        loop {
            let step = {
                let (v0, (v1, v2)) = m145;
                let m142 = v0;
                match m142.uncons() {
                    None => {
                        Err::<(List<u64>, (List<u64>, List<List<u64>>)), List<List<u64>>>(List::cons(f42(v1, List::<u64>::nil()), v2))
                    }
                    Some((v3, v4)) => {
                        let a143 = v3;
                        let a144 = 47u64;
                        if nat_eq(a143, a144) {
                            Ok::<(List<u64>, (List<u64>, List<List<u64>>)), List<List<u64>>>((v4, (List::<u64>::nil(), List::cons(f42(v1, List::<u64>::nil()), v2))))
                        } else {
                            Ok::<(List<u64>, (List<u64>, List<List<u64>>)), List<List<u64>>>((v4, (List::cons(v3, v1), v2)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m145 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f46(p157: List<u64>, p158: List<u64>) -> bool {
    let m156 = (p157, p158);
    {
        let mut m156 = m156;
        loop {
            let step = {
                let (v0, v1) = m156;
                let m149 = v0;
                match m149.uncons() {
                    None => {
                        let m150 = v1;
                        match m150.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                            Some((v2, _)) => {
                                let a151 = v2;
                                let a152 = 47u64;
                                Err::<(List<u64>, List<u64>), bool>(nat_eq(a151, a152))
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m153 = v1;
                        match m153.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                            Some((v6, v7)) => {
                                let a154 = v4;
                                let a155 = v6;
                                if nat_eq(a154, a155) {
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
                Ok(next) => { m156 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f47(p161: List<u64>, v1: List<u64>) -> bool {
    let m160 = p161;
    {
        let mut m160 = m160;
        loop {
            let step = {
                let v0 = m160;
                let m159 = v0;
                match m159.uncons() {
                    None => {
                        Err::<List<u64>, bool>(f35(List::<u64>::nil(), v1.clone()))
                    }
                    Some((v2, v3)) => {
                        if f35(List::cons(v2, v3.clone()), v1.clone()) {
                            Err::<List<u64>, bool>(true)
                        } else {
                            Ok::<List<u64>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m160 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f48(v0: Bytes, v1: u64) -> R<u64> {
    let m164 = {
        let a162 = v0;
        let a163 = v1;
        index_bytes(a162, a163)
    };
    match m164 {
        None => {
            Ok(0u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a209 = {
                let a185 = {
                    let a173 = {
                        let a167 = {
                            let a165 = v3;
                            let a166 = 128u8;
                            fixed_u8::bit_and(a165, a166)
                        };
                        let a168 = 128u8;
                        if equal(a167, a168) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a174 = {
                        let a171 = {
                            let a169 = v3;
                            let a170 = 64u8;
                            fixed_u8::bit_and(a169, a170)
                        };
                        let a172 = 64u8;
                        if equal(a171, a172) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a173, a174)?
                };
                let a186 = {
                    let a183 = {
                        let a177 = {
                            let a175 = v3;
                            let a176 = 32u8;
                            fixed_u8::bit_and(a175, a176)
                        };
                        let a178 = 32u8;
                        if equal(a177, a178) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a184 = {
                        let a181 = {
                            let a179 = v3;
                            let a180 = 16u8;
                            fixed_u8::bit_and(a179, a180)
                        };
                        let a182 = 16u8;
                        if equal(a181, a182) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a183, a184)?
                };
                nat_add(a185, a186)?
            };
            let a210 = {
                let a207 = {
                    let a195 = {
                        let a189 = {
                            let a187 = v3;
                            let a188 = 8u8;
                            fixed_u8::bit_and(a187, a188)
                        };
                        let a190 = 8u8;
                        if equal(a189, a190) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a196 = {
                        let a193 = {
                            let a191 = v3;
                            let a192 = 4u8;
                            fixed_u8::bit_and(a191, a192)
                        };
                        let a194 = 4u8;
                        if equal(a193, a194) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a195, a196)?
                };
                let a208 = {
                    let a205 = {
                        let a199 = {
                            let a197 = v3;
                            let a198 = 2u8;
                            fixed_u8::bit_and(a197, a198)
                        };
                        let a200 = 2u8;
                        if equal(a199, a200) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a206 = {
                        let a203 = {
                            let a201 = v3;
                            let a202 = 1u8;
                            fixed_u8::bit_and(a201, a202)
                        };
                        let a204 = 1u8;
                        if equal(a203, a204) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a205, a206)?
                };
                nat_add(a207, a208)?
            };
            nat_add(a209, a210)
        }
    }
}

pub fn f49(v0: u64, p215: List<List<List<u64>>>, p216: List<List<List<u64>>>, p217: List<List<List<u64>>>) -> List<List<List<u64>>> {
    let m214 = (p215, (p216, p217));
    {
        let mut m214 = m214;
        loop {
            let step = {
                let (v1, (v2, v3)) = m214;
                let m211 = v1;
                match m211.uncons() {
                    None => {
                        let m212 = v2;
                        match m212.uncons() {
                            None => {
                                Err::<(List<List<List<u64>>>, (List<List<List<u64>>>, List<List<List<u64>>>)), List<List<List<u64>>>>(v3)
                            }
                            Some((v4, _)) => {
                                Err::<(List<List<List<u64>>>, (List<List<List<u64>>>, List<List<List<u64>>>)), List<List<List<u64>>>>(List::cons(v4, v3))
                            }
                        }
                    }
                    Some((v6, v7)) => {
                        let m213 = v2;
                        match m213.uncons() {
                            None => {
                                Ok::<(List<List<List<u64>>>, (List<List<List<u64>>>, List<List<List<u64>>>)), List<List<List<u64>>>>((v7, (List::cons(v6, List::<List<List<u64>>>::nil()), v3)))
                            }
                            Some((v8, _)) => {
                                Ok::<(List<List<List<u64>>>, (List<List<List<u64>>>, List<List<List<u64>>>)), List<List<List<u64>>>>((v7, (List::<List<List<u64>>>::nil(), List::cons(f53(v0, v8, v6, List::<List<u64>>::nil()), v3))))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m214 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f50(v0: (Bytes, Bytes), p226: List<u64>, p227: bool, p228: u64, p229: List<u64>, p230: List<u64>) -> R<List<u64>> {
    let m225 = (p226, (p227, (p228, (p229, p230))));
    {
        let mut m225 = m225;
        loop {
            let step = {
                let (v1, (v2, (v3, (v4, v5)))) = m225;
                let m218 = v1;
                match m218.uncons() {
                    None => {
                        if v2 {
                            Err::<(List<u64>, (bool, (u64, (List<u64>, List<u64>)))), R<List<u64>>>(Ok(f42(f54(v3, v4, v5), List::<u64>::nil())))
                        } else {
                            Err::<(List<u64>, (bool, (u64, (List<u64>, List<u64>)))), R<List<u64>>>(Ok(f42(v5, List::<u64>::nil())))
                        }
                    }
                    Some((v6, v7)) => {
                        let v8: u64 = f55(v0.clone(), v6)?;
                        if v2 {
                            let v9: u64 = f56(f57(v0.clone(), v4.clone(), v8)?, f58(v0.clone(), v3, v6)?);
                            let a219 = v9;
                            let a220 = 0u64;
                            if nat_eq(a219, a220) {
                                let a221 = v8;
                                let a222 = 0u64;
                                if nat_eq(a221, a222) {
                                    Ok::<(List<u64>, (bool, (u64, (List<u64>, List<u64>)))), R<List<u64>>>((v7, (true, (v6, (List::<u64>::nil(), f54(v3, v4, v5))))))
                                } else {
                                    Ok::<(List<u64>, (bool, (u64, (List<u64>, List<u64>)))), R<List<u64>>>((v7, (true, (v3, (List::cons(v6, v4), v5)))))
                                }
                            } else {
                                Ok::<(List<u64>, (bool, (u64, (List<u64>, List<u64>)))), R<List<u64>>>((v7, (true, (v9, (v4, v5)))))
                            }
                        } else {
                            let a223 = v8;
                            let a224 = 0u64;
                            if nat_eq(a223, a224) {
                                Ok::<(List<u64>, (bool, (u64, (List<u64>, List<u64>)))), R<List<u64>>>((v7, (true, (v6, (List::<u64>::nil(), v5)))))
                            } else {
                                Ok::<(List<u64>, (bool, (u64, (List<u64>, List<u64>)))), R<List<u64>>>((v7, (false, (0u64, (List::<u64>::nil(), List::cons(v6, v5))))))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m225 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f51(v0: (Bytes, Bytes), v1: List<u64>) -> R<List<u64>> {
    f59(v0.clone(), f60(v0, v1)?)
}

pub fn f52(v0: bool, v1: bool, v2: bool) -> Adt0 {
    let m231 = v0;
    if m231 {
        Adt0::C1(Adt1::C3)
    } else {
        let m232 = v1;
        if m232 {
            Adt0::C1(Adt1::C4)
        } else {
            let m233 = v2;
            if m233 {
                Adt0::C1(Adt1::C5)
            } else {
                Adt0::C0
            }
        }
    }
}

pub fn f53(p238: u64, p239: List<List<u64>>, p240: List<List<u64>>, p241: List<List<u64>>) -> List<List<u64>> {
    let m237 = (p238, (p239, (p240, p241)));
    {
        let mut m237 = m237;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m237;
                let m234 = v0;
                if m234 == 0 {
                    Err::<(u64, (List<List<u64>>, (List<List<u64>>, List<List<u64>>))), List<List<u64>>>(f32(v3, v1))
                } else {
                    let v4 = m234 - 1;
                    let m235 = v1.clone();
                    match m235.uncons() {
                        None => {
                            Err::<(u64, (List<List<u64>>, (List<List<u64>>, List<List<u64>>))), List<List<u64>>>(f32(v3, v2))
                        }
                        Some((v5, v6)) => {
                            let m236 = v2.clone();
                            match m236.uncons() {
                                None => {
                                    Err::<(u64, (List<List<u64>>, (List<List<u64>>, List<List<u64>>))), List<List<u64>>>(f32(v3, v1))
                                }
                                Some((v7, v8)) => {
                                    if f61(v7.clone(), v5.clone()) {
                                        Ok::<(u64, (List<List<u64>>, (List<List<u64>>, List<List<u64>>))), List<List<u64>>>((v4, (v1, (v8, List::cons(v7, v3)))))
                                    } else {
                                        Ok::<(u64, (List<List<u64>>, (List<List<u64>>, List<List<u64>>))), List<List<u64>>>((v4, (v6, (v2, List::cons(v5, v3)))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m237 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f54(v0: u64, v1: List<u64>, v2: List<u64>) -> List<u64> {
    f42(f42(v1, List::<u64>::nil()), List::cons(v0, v2))
}

pub fn f55(v0: (Bytes, Bytes), v1: u64) -> R<u64> {
    let a242 = v1;
    let a243 = f40();
    if nat_lt(a242, a243) {
        Ok(0u64)
    } else {
        let v2: u64 = f62(v0.clone(), f63(v0.clone(), 0u64)?, 7u64, f64(v0.clone(), 0u64)?, 3u64, v1)?;
        let a244 = v1;
        let a245 = f65(v0.clone(), v2, 3u64, 0u64)?;
        if nat_lt(a244, a245) {
            Ok(0u64)
        } else {
            let a248 = f65(v0.clone(), {
                let a246 = v2;
                let a247 = 3u64;
                nat_add(a246, a247)?
            }, 3u64, 0u64)?;
            let a249 = v1;
            if nat_lt(a248, a249) {
                Ok(0u64)
            } else {
                f66(v0, {
                    let a250 = v2;
                    let a251 = 6u64;
                    nat_add(a250, a251)?
                })
            }
        }
    }
}

pub fn f56(v0: bool, v1: u64) -> u64 {
    if v0 {
        0u64
    } else {
        v1
    }
}

pub fn f57(v0: (Bytes, Bytes), v1: List<u64>, v2: u64) -> R<bool> {
    let m252 = v1;
    match m252.uncons() {
        None => {
            Ok(false)
        }
        Some((v3, _)) => {
            let a253 = f55(v0, v3)?;
            let a254 = v2;
            Ok(!nat_lt(a253, a254))
        }
    }
}

pub fn f58(v0: (Bytes, Bytes), v1: u64, v2: u64) -> R<u64> {
    Ok(f67(f68(v1, v2)?, f69(v0.clone(), v1, v2)?, f70(v0.clone(), f69(v0, v1, v2)?)?))
}

pub fn f59(v0: (Bytes, Bytes), v1: List<u64>) -> R<List<u64>> {
    Ok(f42(f71(v0, v1, List::<(u64, u64)>::nil(), List::<u64>::nil())?, List::<u64>::nil()))
}

pub fn f60(v0: (Bytes, Bytes), v1: List<u64>) -> R<List<u64>> {
    f72(v0, f73(), v1)
}

pub fn f61(p263: List<u64>, p264: List<u64>) -> bool {
    let m262 = (p263, p264);
    {
        let mut m262 = m262;
        loop {
            let step = {
                let (v0, v1) = m262;
                let m255 = v0;
                match m255.uncons() {
                    None => {
                        let m256 = v1;
                        match m256.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                            Some((_, _)) => {
                                Err::<(List<u64>, List<u64>), bool>(true)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m257 = v1;
                        match m257.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                            Some((v6, v7)) => {
                                let a258 = v4;
                                let a259 = v6;
                                if nat_lt(a258, a259) {
                                    Err::<(List<u64>, List<u64>), bool>(true)
                                } else {
                                    let a260 = v6;
                                    let a261 = v4;
                                    if nat_lt(a260, a261) {
                                        Err::<(List<u64>, List<u64>), bool>(false)
                                    } else {
                                        Ok::<(List<u64>, List<u64>), bool>((v5, v7))
                                    }
                                }
                            }
                        }
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

pub fn f62(v0: (Bytes, Bytes), v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<u64> {
    let a267 = v1;
    let a268 = {
        let a265 = f74(16u64, v0, v1, v2, v4, v5, 0u64, v3)?;
        let a266 = v2;
        nat_mul(a265, a266)?
    };
    nat_add(a267, a268)
}

pub fn f63(v0: (Bytes, Bytes), v1: u64) -> R<u64> {
    f65(v0, {
        let a269 = v1;
        let a270 = 6u64;
        nat_mul(a269, a270)?
    }, 3u64, 0u64)
}

pub fn f64(v0: (Bytes, Bytes), v1: u64) -> R<u64> {
    f65(v0, {
        let a273 = {
            let a271 = v1;
            let a272 = 6u64;
            nat_mul(a271, a272)?
        };
        let a274 = 3u64;
        nat_add(a273, a274)?
    }, 3u64, 0u64)
}

pub fn f65(v0: (Bytes, Bytes), p283: u64, p284: u64, p285: u64) -> R<u64> {
    let m282 = (p283, (p284, p285));
    {
        let mut m282 = m282;
        loop {
            let step = {
                let (v1, (v2, v3)) = m282;
                let m275 = v2;
                if m275 == 0 {
                    Err::<(u64, (u64, u64)), R<u64>>(Ok(v3))
                } else {
                    let v4 = m275 - 1;
                    Ok::<(u64, (u64, u64)), R<u64>>(({
                        let a276 = v1;
                        let a277 = 1u64;
                        nat_add(a276, a277)?
                    }, (v4, {
                        let a280 = {
                            let a278 = v3;
                            let a279 = 256u64;
                            nat_mul(a278, a279)?
                        };
                        let a281 = f66(v0.clone(), v1)?;
                        nat_add(a280, a281)?
                    })))
                }
            };
            match step {
                Ok(next) => { m282 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f66(v0: (Bytes, Bytes), v1: u64) -> R<u64> {
    let a286 = v1;
    let a287 = f75();
    if nat_lt(a286, a287) {
        f48({
            let (h288, _) = v0;
            h288
        }, v1)
    } else {
        f48({
            let (_, h289) = v0;
            h289
        }, {
            let a290 = v1;
            let a291 = f75();
            nat_sub(a290, a291)
        })
    }
}

pub fn f67(v0: u64, v1: u64, v2: bool) -> u64 {
    let a292 = v0;
    let a293 = 0u64;
    if nat_eq(a292, a293) {
        let a294 = v1;
        let a295 = 0u64;
        if nat_eq(a294, a295) {
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

pub fn f68(v0: u64, v1: u64) -> R<u64> {
    if f76(v0) {
        let a301 = {
            let a298 = {
                let a296 = v0;
                let a297 = 44032u64;
                nat_sub(a296, a297)
            };
            let a299 = 28u64;
            let a300 = 0u64;
            nat_rem(a298, a299, a300)
        };
        let a302 = 0u64;
        if nat_eq(a301, a302) {
            let a303 = v1;
            let a304 = 4520u64;
            if nat_lt(a303, a304) {
                Ok(0u64)
            } else {
                let a305 = v1;
                let a306 = 4547u64;
                if nat_lt(a305, a306) {
                    let a309 = v0;
                    let a310 = {
                        let a307 = v1;
                        let a308 = 4519u64;
                        nat_sub(a307, a308)
                    };
                    nat_add(a309, a310)
                } else {
                    Ok(0u64)
                }
            }
        } else {
            Ok(0u64)
        }
    } else {
        let a311 = v0;
        let a312 = 4352u64;
        if nat_lt(a311, a312) {
            Ok(0u64)
        } else {
            let a313 = v0;
            let a314 = 4371u64;
            if nat_lt(a313, a314) {
                let a315 = v1;
                let a316 = 4449u64;
                if nat_lt(a315, a316) {
                    Ok(0u64)
                } else {
                    let a317 = v1;
                    let a318 = 4470u64;
                    if nat_lt(a317, a318) {
                        let a329 = 44032u64;
                        let a330 = {
                            let a327 = {
                                let a325 = {
                                    let a321 = {
                                        let a319 = v0;
                                        let a320 = 4352u64;
                                        nat_sub(a319, a320)
                                    };
                                    let a322 = 21u64;
                                    nat_mul(a321, a322)?
                                };
                                let a326 = {
                                    let a323 = v1;
                                    let a324 = 4449u64;
                                    nat_sub(a323, a324)
                                };
                                nat_add(a325, a326)?
                            };
                            let a328 = 28u64;
                            nat_mul(a327, a328)?
                        };
                        nat_add(a329, a330)
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

pub fn f69(v0: (Bytes, Bytes), v1: u64, v2: u64) -> R<u64> {
    let a331 = v2;
    let a332 = f40();
    if nat_lt(a331, a332) {
        Ok(0u64)
    } else {
        let v3: u64 = {
            let a335 = {
                let a333 = v1;
                let a334 = 16777216u64;
                nat_mul(a333, a334)?
            };
            let a336 = v2;
            nat_add(a335, a336)?
        };
        let v4: u64 = f62(v0.clone(), f63(v0.clone(), 4u64)?, 9u64, f64(v0.clone(), 4u64)?, 6u64, v3)?;
        let a337 = f65(v0.clone(), v4, 6u64, 0u64)?;
        let a338 = v3;
        if nat_eq(a337, a338) {
            f65(v0, {
                let a339 = v4;
                let a340 = 6u64;
                nat_add(a339, a340)?
            }, 3u64, 0u64)
        } else {
            Ok(0u64)
        }
    }
}

pub fn f70(v0: (Bytes, Bytes), v1: u64) -> R<bool> {
    let a341 = v1;
    let a342 = f40();
    if nat_lt(a341, a342) {
        Ok(false)
    } else {
        let v2: u64 = f62(v0.clone(), f63(v0.clone(), 1u64)?, 6u64, f64(v0.clone(), 1u64)?, 3u64, v1)?;
        let a343 = v1;
        let a344 = f65(v0.clone(), v2, 3u64, 0u64)?;
        if nat_lt(a343, a344) {
            Ok(false)
        } else {
            let a347 = f65(v0, {
                let a345 = v2;
                let a346 = 3u64;
                nat_add(a345, a346)?
            }, 3u64, 0u64)?;
            let a348 = v1;
            Ok(!nat_lt(a347, a348))
        }
    }
}

pub fn f71(v0: (Bytes, Bytes), p353: List<u64>, p354: List<(u64, u64)>, p355: List<u64>) -> R<List<u64>> {
    let m352 = (p353, (p354, p355));
    {
        let mut m352 = m352;
        loop {
            let step = {
                let (v1, (v2, v3)) = m352;
                let m349 = v1;
                match m349.uncons() {
                    None => {
                        Err::<(List<u64>, (List<(u64, u64)>, List<u64>)), R<List<u64>>>(Ok(f77(f78(v2)?, v3)))
                    }
                    Some((v4, v5)) => {
                        let v6: u64 = f55(v0.clone(), v4)?;
                        let a350 = v6;
                        let a351 = 0u64;
                        if nat_eq(a350, a351) {
                            Ok::<(List<u64>, (List<(u64, u64)>, List<u64>)), R<List<u64>>>((v5, (List::<(u64, u64)>::nil(), List::cons(v4, f77(f78(v2)?, v3)))))
                        } else {
                            Ok::<(List<u64>, (List<(u64, u64)>, List<u64>)), R<List<u64>>>((v5, (List::cons((v6, v4), v2), v3)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m352 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f72(v0: (Bytes, Bytes), p358: u64, p359: List<u64>) -> R<List<u64>> {
    let m357 = (p358, p359);
    {
        let mut m357 = m357;
        loop {
            let step = {
                let (v1, v2) = m357;
                let m356 = v1;
                if m356 == 0 {
                    Err::<(u64, List<u64>), R<List<u64>>>(Ok(v2))
                } else {
                    let v3 = m356 - 1;
                    Ok::<(u64, List<u64>), R<List<u64>>>((v3, f79(v0.clone(), v2)?))
                }
            };
            match step {
                Ok(next) => { m357 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f73() -> u64 {
    3u64
}

pub fn f74(p377: u64, v1: (Bytes, Bytes), v2: u64, v3: u64, v4: u64, v5: u64, p378: u64, p379: u64) -> R<u64> {
    let m376 = (p377, (p378, p379));
    {
        let mut m376 = m376;
        loop {
            let step = {
                let (v0, (v6, v7)) = m376;
                let m360 = v0;
                if m360 == 0 {
                    Err::<(u64, (u64, u64)), R<u64>>(Ok(v6))
                } else {
                    let v8 = m360 - 1;
                    let a363 = {
                        let a361 = v7;
                        let a362 = v6;
                        nat_sub(a361, a362)
                    };
                    let a364 = 2u64;
                    if nat_lt(a363, a364) {
                        Err::<(u64, (u64, u64)), R<u64>>(Ok(v6))
                    } else {
                        let v9: u64 = {
                            let a367 = {
                                let a365 = v6;
                                let a366 = v7;
                                nat_add(a365, a366)?
                            };
                            let a368 = 2u64;
                            let a369 = 0u64;
                            nat_quot(a367, a368, a369)
                        };
                        let a374 = v5;
                        let a375 = f65(v1.clone(), {
                            let a372 = v2;
                            let a373 = {
                                let a370 = v9;
                                let a371 = v3;
                                nat_mul(a370, a371)?
                            };
                            nat_add(a372, a373)?
                        }, v4, 0u64)?;
                        if nat_lt(a374, a375) {
                            Ok::<(u64, (u64, u64)), R<u64>>((v8, (v6, v9)))
                        } else {
                            Ok::<(u64, (u64, u64)), R<u64>>((v8, (v9, v7)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m376 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f75() -> u64 {
    16384u64
}

pub fn f76(v0: u64) -> bool {
    let a380 = v0;
    let a381 = 44032u64;
    if nat_lt(a380, a381) {
        false
    } else {
        let a382 = v0;
        let a383 = 55204u64;
        nat_lt(a382, a383)
    }
}

pub fn f77(p387: List<(u64, u64)>, p388: List<u64>) -> List<u64> {
    let m386 = (p387, p388);
    {
        let mut m386 = m386;
        loop {
            let step = {
                let (v0, v1) = m386;
                let m384 = v0;
                match m384.uncons() {
                    None => {
                        Err::<(List<(u64, u64)>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(u64, u64)>, List<u64>), List<u64>>((v3, List::cons({
                            let (_, h385) = v2;
                            h385
                        }, v1)))
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

pub fn f78(v0: List<(u64, u64)>) -> R<List<(u64, u64)>> {
    let v1: u64 = f80(v0.clone(), 0u64)?;
    Ok(f81({
        let a389 = v1;
        let a390 = 1u64;
        nat_add(a389, a390)?
    }, {
        let a391 = v1;
        let a392 = 1u64;
        nat_add(a391, a392)?
    }, f82(v0, List::<List<(u64, u64)>>::nil())))
}

pub fn f79(v0: (Bytes, Bytes), v1: List<u64>) -> R<List<u64>> {
    Ok(f42(f83(v0, v1, List::<u64>::nil())?, List::<u64>::nil()))
}

pub fn f80(p397: List<(u64, u64)>, p398: u64) -> R<u64> {
    let m396 = (p397, p398);
    {
        let mut m396 = m396;
        loop {
            let step = {
                let (v0, v1) = m396;
                let m393 = v0;
                match m393.uncons() {
                    None => {
                        Err::<(List<(u64, u64)>, u64), R<u64>>(Ok(v1))
                    }
                    Some((_, v3)) => {
                        Ok::<(List<(u64, u64)>, u64), R<u64>>((v3, {
                            let a394 = v1;
                            let a395 = 1u64;
                            nat_add(a394, a395)?
                        }))
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

pub fn f81(p404: u64, v1: u64, p405: List<List<(u64, u64)>>) -> List<(u64, u64)> {
    let m403 = (p404, p405);
    {
        let mut m403 = m403;
        loop {
            let step = {
                let (v0, v2) = m403;
                let m399 = v0;
                if m399 == 0 {
                    let m400 = v2;
                    match m400.uncons() {
                        None => {
                            Err::<(u64, List<List<(u64, u64)>>), List<(u64, u64)>>(List::<(u64, u64)>::nil())
                        }
                        Some((v3, _)) => {
                            Err::<(u64, List<List<(u64, u64)>>), List<(u64, u64)>>(v3)
                        }
                    }
                } else {
                    let v5 = m399 - 1;
                    let m401 = v2.clone();
                    match m401.uncons() {
                        None => {
                            Err::<(u64, List<List<(u64, u64)>>), List<(u64, u64)>>(List::<(u64, u64)>::nil())
                        }
                        Some((v6, v7)) => {
                            let m402 = v7;
                            match m402.uncons() {
                                None => {
                                    Err::<(u64, List<List<(u64, u64)>>), List<(u64, u64)>>(v6)
                                }
                                Some((_, _)) => {
                                    Ok::<(u64, List<List<(u64, u64)>>), List<(u64, u64)>>((v5, f84(v1, v2, List::<List<(u64, u64)>>::nil(), List::<List<(u64, u64)>>::nil())))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m403 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f82(p408: List<(u64, u64)>, p409: List<List<(u64, u64)>>) -> List<List<(u64, u64)>> {
    let m407 = (p408, p409);
    {
        let mut m407 = m407;
        loop {
            let step = {
                let (v0, v1) = m407;
                let m406 = v0;
                match m406.uncons() {
                    None => {
                        Err::<(List<(u64, u64)>, List<List<(u64, u64)>>), List<List<(u64, u64)>>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(u64, u64)>, List<List<(u64, u64)>>), List<List<(u64, u64)>>>((v3, List::cons(List::cons(v2, List::<(u64, u64)>::nil()), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m407 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f83(v0: (Bytes, Bytes), p412: List<u64>, p413: List<u64>) -> R<List<u64>> {
    let m411 = (p412, p413);
    {
        let mut m411 = m411;
        loop {
            let step = {
                let (v1, v2) = m411;
                let m410 = v1;
                match m410.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), R<List<u64>>>(Ok(v2))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<u64>, List<u64>), R<List<u64>>>((v4, f42(f85(v0.clone(), v3)?, v2)))
                    }
                }
            };
            match step {
                Ok(next) => { m411 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f84(v0: u64, p418: List<List<(u64, u64)>>, p419: List<List<(u64, u64)>>, p420: List<List<(u64, u64)>>) -> List<List<(u64, u64)>> {
    let m417 = (p418, (p419, p420));
    {
        let mut m417 = m417;
        loop {
            let step = {
                let (v1, (v2, v3)) = m417;
                let m414 = v1;
                match m414.uncons() {
                    None => {
                        let m415 = v2;
                        match m415.uncons() {
                            None => {
                                Err::<(List<List<(u64, u64)>>, (List<List<(u64, u64)>>, List<List<(u64, u64)>>)), List<List<(u64, u64)>>>(f86(v3, List::<List<(u64, u64)>>::nil()))
                            }
                            Some((v4, _)) => {
                                Err::<(List<List<(u64, u64)>>, (List<List<(u64, u64)>>, List<List<(u64, u64)>>)), List<List<(u64, u64)>>>(f86(List::cons(v4, v3), List::<List<(u64, u64)>>::nil()))
                            }
                        }
                    }
                    Some((v6, v7)) => {
                        let m416 = v2;
                        match m416.uncons() {
                            None => {
                                Ok::<(List<List<(u64, u64)>>, (List<List<(u64, u64)>>, List<List<(u64, u64)>>)), List<List<(u64, u64)>>>((v7, (List::cons(v6, List::<List<(u64, u64)>>::nil()), v3)))
                            }
                            Some((v8, _)) => {
                                Ok::<(List<List<(u64, u64)>>, (List<List<(u64, u64)>>, List<List<(u64, u64)>>)), List<List<(u64, u64)>>>((v7, (List::<List<(u64, u64)>>::nil(), List::cons(f87(v0, v8, v6, List::<(u64, u64)>::nil()), v3))))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m417 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f85(v0: (Bytes, Bytes), v1: u64) -> R<List<u64>> {
    if f76(v1) {
        f88(v1)
    } else {
        let v2: u64 = f89(v0.clone(), v1)?;
        let a421 = v2;
        let a422 = 0u64;
        if nat_eq(a421, a422) {
            let m423 = f90(v0, v1)?;
            match m423.uncons() {
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

pub fn f86(p426: List<List<(u64, u64)>>, p427: List<List<(u64, u64)>>) -> List<List<(u64, u64)>> {
    let m425 = (p426, p427);
    {
        let mut m425 = m425;
        loop {
            let step = {
                let (v0, v1) = m425;
                let m424 = v0;
                match m424.uncons() {
                    None => {
                        Err::<(List<List<(u64, u64)>>, List<List<(u64, u64)>>), List<List<(u64, u64)>>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<(u64, u64)>>, List<List<(u64, u64)>>), List<List<(u64, u64)>>>((v3, List::cons(v2, v1)))
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

pub fn f87(p436: u64, p437: List<(u64, u64)>, p438: List<(u64, u64)>, p439: List<(u64, u64)>) -> List<(u64, u64)> {
    let m435 = (p436, (p437, (p438, p439)));
    {
        let mut m435 = m435;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m435;
                let m428 = v0;
                if m428 == 0 {
                    Err::<(u64, (List<(u64, u64)>, (List<(u64, u64)>, List<(u64, u64)>))), List<(u64, u64)>>(f91(v3, v1))
                } else {
                    let v4 = m428 - 1;
                    let m429 = v1.clone();
                    match m429.uncons() {
                        None => {
                            Err::<(u64, (List<(u64, u64)>, (List<(u64, u64)>, List<(u64, u64)>))), List<(u64, u64)>>(f91(v3, v2))
                        }
                        Some((v5, v6)) => {
                            let m430 = v2.clone();
                            match m430.uncons() {
                                None => {
                                    Err::<(u64, (List<(u64, u64)>, (List<(u64, u64)>, List<(u64, u64)>))), List<(u64, u64)>>(f91(v3, v1))
                                }
                                Some((v7, v8)) => {
                                    let a433 = {
                                        let (h431, _) = v7;
                                        h431
                                    };
                                    let a434 = {
                                        let (h432, _) = v5;
                                        h432
                                    };
                                    if nat_lt(a433, a434) {
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
                Ok(next) => { m435 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f88(v0: u64) -> R<List<u64>> {
    let v1: u64 = {
        let a440 = v0;
        let a441 = 44032u64;
        nat_sub(a440, a441)
    };
    let v2: u64 = {
        let a445 = 4352u64;
        let a446 = {
            let a442 = v1;
            let a443 = 588u64;
            let a444 = 0u64;
            nat_quot(a442, a443, a444)
        };
        nat_add(a445, a446)?
    };
    let v3: u64 = {
        let a453 = 4449u64;
        let a454 = {
            let a450 = {
                let a447 = v1;
                let a448 = 588u64;
                let a449 = 0u64;
                nat_rem(a447, a448, a449)
            };
            let a451 = 28u64;
            let a452 = 0u64;
            nat_quot(a450, a451, a452)
        };
        nat_add(a453, a454)?
    };
    let v4: u64 = {
        let a458 = 4519u64;
        let a459 = {
            let a455 = v1;
            let a456 = 28u64;
            let a457 = 0u64;
            nat_rem(a455, a456, a457)
        };
        nat_add(a458, a459)?
    };
    let a460 = v4;
    let a461 = 4519u64;
    if nat_eq(a460, a461) {
        Ok(List::cons(v2, List::cons(v3, List::<u64>::nil())))
    } else {
        Ok(List::cons(v2, List::cons(v3, List::cons(v4, List::<u64>::nil()))))
    }
}

pub fn f89(v0: (Bytes, Bytes), v1: u64) -> R<u64> {
    let a462 = v1;
    let a463 = f40();
    if nat_lt(a462, a463) {
        Ok(0u64)
    } else {
        let v2: u64 = f62(v0.clone(), f63(v0.clone(), 2u64)?, 6u64, f64(v0.clone(), 2u64)?, 3u64, v1)?;
        let a464 = f65(v0.clone(), v2, 3u64, 0u64)?;
        let a465 = v1;
        if nat_eq(a464, a465) {
            f65(v0, {
                let a466 = v2;
                let a467 = 3u64;
                nat_add(a466, a467)?
            }, 3u64, 0u64)
        } else {
            Ok(0u64)
        }
    }
}

pub fn f90(v0: (Bytes, Bytes), v1: u64) -> R<List<u64>> {
    let a468 = v1;
    let a469 = f92();
    if nat_lt(a468, a469) {
        Ok(List::<u64>::nil())
    } else {
        let v2: u64 = f62(v0.clone(), f63(v0.clone(), 3u64)?, 9u64, f64(v0.clone(), 3u64)?, 3u64, v1)?;
        let a470 = f65(v0.clone(), v2, 3u64, 0u64)?;
        let a471 = v1;
        if nat_eq(a470, a471) {
            Ok(List::cons(f65(v0.clone(), {
                let a472 = v2;
                let a473 = 3u64;
                nat_add(a472, a473)?
            }, 3u64, 0u64)?, List::cons(f65(v0, {
                let a474 = v2;
                let a475 = 6u64;
                nat_add(a474, a475)?
            }, 3u64, 0u64)?, List::<u64>::nil())))
        } else {
            Ok(List::<u64>::nil())
        }
    }
}

pub fn f91(p478: List<(u64, u64)>, p479: List<(u64, u64)>) -> List<(u64, u64)> {
    let m477 = (p478, p479);
    {
        let mut m477 = m477;
        loop {
            let step = {
                let (v0, v1) = m477;
                let m476 = v0;
                match m476.uncons() {
                    None => {
                        Err::<(List<(u64, u64)>, List<(u64, u64)>), List<(u64, u64)>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(u64, u64)>, List<(u64, u64)>), List<(u64, u64)>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m477 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f92() -> u64 {
    192u64
}

pub fn run(p0: Bytes, p1: Bytes, p2: u64, p3: u64, p4: List<Bytes>, p5: List<(Bytes, Bytes)>) -> R<List<u64>> {
    f0(p0, p1, p2, p3, p4, p5)
}
