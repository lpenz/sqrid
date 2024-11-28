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

fn _test_boundedint_trait_basic<T: BoundedInt, const UNSIGNED: bool>()
where
    usize: TryFrom<T>,
{
    let i5 = or_panic!(T::try_from(5));
    assert!(i5 == i5);
    assert!(i5 >= i5);
    assert!(i5 <= i5);
    assert_eq!(i5, i5);
    assert_eq!(or_panic!(usize::try_from(i5)), 5);
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

type BI8 = BoundedI8<-1, 5>;

#[test]
fn test_bounded_type() {
    let two = BI8::new_static::<2>();
    // Test into_inner:
    assert_eq!(two.into_inner(), 2_i8);
    // Test constructor:
    assert_eq!(BI8::new(2), Ok(two));
    assert_eq!(BI8::new_unwrap(2), two);
    // Test constructor errors:
    assert_eq!(BI8::new(-2), Err(Error::OutOfBounds));
    assert_eq!(BI8::try_from(6), Err(Error::OutOfBounds));
    assert!(catch_unwind(|| BI8::new_unwrap(6)).is_err());
    // Test conversion:
    assert_eq!(BI8::try_from(2_i32), Ok(two));
    assert_eq!(BI8::try_from(2_u64), Ok(two));
    assert_eq!(i8::from(two), 2_i8);
    // Test conversion failure:
    assert_eq!(BI8::try_from(6_i32), Err(Error::OutOfBounds));
    // Test try_into usize:
    assert_eq!(usize::try_from(two), Ok(2_usize));
}
