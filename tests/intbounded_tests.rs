// Copyright (C) 2024 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use sqrid::intbounded::UIntBounded;

type UIB = UIntBounded<0, 20, u8>;

#[test]
fn test_basic() {
    let i5 = UIB::new(5).unwrap();
    assert!(i5 == i5);
    assert!(i5 >= i5);
    assert!(i5 <= i5);
    assert_eq!(i5, i5);
    assert_ne!(i5 < i5, true);
    assert_ne!(i5 > i5, true);
    assert_eq!(i5 + i5, UIB::new(10).unwrap());
}
