// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

#![warn(rust_2018_idioms)]

use criterion::{criterion_group, criterion_main, Criterion};

fn qaqr_mutual() {
    type Qa = sqrid::Qa<256, 257>;
    for qa in Qa::iter() {
        for qr in sqrid::Qr::iter::<true>() {
            if let Some(qa2) = qa + qr {
                let found = sqrid::Qr::iter::<true>()
                    .filter(|qr| qa2 + *qr == Some(qa))
                    .next()
                    .is_some();
                assert!(found);
            }
        }
        for qr in sqrid::Qr::iter::<false>() {
            if let Some(qa2) = qa + qr {
                let found = sqrid::Qr::iter::<false>()
                    .filter(|qr| qa2 + *qr == Some(qa))
                    .next()
                    .is_some();
                assert!(found);
            }
        }
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("qaqr_mutual", |b| b.iter(|| qaqr_mutual()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
