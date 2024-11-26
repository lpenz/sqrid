// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use sqrid::boundedint::*;
use sqrid::BoundedInt;
use sqrid::Error;

use std::panic::catch_unwind;

macro_rules! or_panic {
    ($e:expr) => {{
        let Ok(value) = $e else { panic!() };
        value
    }};
}

fn _test_boundedint_trait_basic<T: BoundedInt, const UNSIGNED: bool>() {
    let i5 = or_panic!(T::try_from(5));
    assert!(i5 == i5);
    assert!(i5 >= i5);
    assert!(i5 <= i5);
    assert_eq!(i5, i5);
    assert_ne!(i5 < i5, true);
    assert_ne!(i5 > i5, true);
    assert_eq!(i5.checked_add(i5), Some(or_panic!(T::try_from(10))));
    assert_eq!(i5.checked_sub(i5), Some(or_panic!(T::try_from(0))));
    assert_eq!(i5.inc(), Some(or_panic!(T::try_from(6))));
    assert_eq!(i5.dec(), Some(or_panic!(T::try_from(4))));
    if UNSIGNED {
        assert_eq!(T::default().dec(), None);
    }
}

#[test]
fn test_basic_uints() {
    _test_boundedint_trait_basic::<u8, true>();
    _test_boundedint_trait_basic::<u16, true>();
    _test_boundedint_trait_basic::<u32, true>();
    _test_boundedint_trait_basic::<u64, true>();
    _test_boundedint_trait_basic::<u128, true>();
}

#[test]
fn test_basic_iints() {
    _test_boundedint_trait_basic::<i8, false>();
    _test_boundedint_trait_basic::<i16, false>();
    _test_boundedint_trait_basic::<i32, false>();
    _test_boundedint_trait_basic::<i64, false>();
    _test_boundedint_trait_basic::<i128, false>();
}

#[test]
fn test_basic_bounded_uint() {
    _test_boundedint_trait_basic::<BoundedU8<0, 20>, true>();
    _test_boundedint_trait_basic::<BoundedU16<0, 20>, true>();
    _test_boundedint_trait_basic::<BoundedU32<0, 20>, true>();
    _test_boundedint_trait_basic::<BoundedU64<0, 20>, true>();
    _test_boundedint_trait_basic::<BoundedU128<0, 20>, true>();
}

#[test]
fn test_basic_bounded_iint() {
    _test_boundedint_trait_basic::<BoundedI8<0, 20>, false>();
    _test_boundedint_trait_basic::<BoundedI16<0, 20>, false>();
    _test_boundedint_trait_basic::<BoundedI32<0, 20>, false>();
    _test_boundedint_trait_basic::<BoundedI64<0, 20>, false>();
    _test_boundedint_trait_basic::<BoundedI128<0, 20>, false>();
}

#[test]
fn test_bounded_type() {
    assert_eq!(
        BoundedI8::<-1, 5>::new(2).unwrap(),
        BoundedI8::<-1, 5>::new_static::<2>()
    );
    assert_eq!(BoundedI8::<-1, 5>::new_unwrap(2).into_inner(), 2);
    assert_eq!(BoundedI8::<-1, 5>::new(-2), Err(Error::OutOfBounds));
    assert!(BoundedI8::<-1, 5>::new(5).is_ok());
    assert!(catch_unwind(|| BoundedI8::<-1, 5>::new_unwrap(6)).is_err());
}
