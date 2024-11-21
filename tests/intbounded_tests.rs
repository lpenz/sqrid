// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use sqrid::intbounded::*;
use sqrid::Int;

macro_rules! or_panic {
    ($e:expr) => {{
        let Ok(value) = $e else { panic!() };
        value
    }};
}

fn _test_basic<T: Int>() {
    let i5 = or_panic!(T::try_from(5));
    assert!(i5 == i5);
    assert!(i5 >= i5);
    assert!(i5 <= i5);
    assert_eq!(i5, i5);
    assert_ne!(i5 < i5, true);
    assert_ne!(i5 > i5, true);
    assert_eq!(i5.checked_add(i5), Some(or_panic!(T::try_from(10))));
}

#[test]
fn test_uints() {
    _test_basic::<u8>();
    _test_basic::<u16>();
    _test_basic::<u32>();
    _test_basic::<u64>();
    _test_basic::<u128>();
}

#[test]
fn test_iints() {
    _test_basic::<i8>();
    _test_basic::<i16>();
    _test_basic::<i32>();
    _test_basic::<i64>();
    _test_basic::<i128>();
}

#[test]
fn test_uintbounded() {
    _test_basic::<U8Bounded<0, 20>>();
    _test_basic::<U16Bounded<0, 20>>();
    _test_basic::<U32Bounded<0, 20>>();
    _test_basic::<U64Bounded<0, 20>>();
    _test_basic::<U128Bounded<0, 20>>();
}

#[test]
fn test_iintbounded() {
    _test_basic::<I8Bounded<0, 20>>();
    _test_basic::<I16Bounded<0, 20>>();
    _test_basic::<I32Bounded<0, 20>>();
    _test_basic::<I64Bounded<0, 20>>();
    _test_basic::<I128Bounded<0, 20>>();
}
