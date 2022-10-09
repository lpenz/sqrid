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

fn grid_index() {
    type Qa = sqrid::Qa<256, 257>;
    type GridArray = sqrid::GridArray<usize, 256, 257, { 256 * 257 }>;
    let mut g = GridArray::default();
    for qa in Qa::iter() {
        g[qa] = qa.to_usize();
    }
    for qa in Qa::iter() {
        assert_eq!(g[qa], qa.to_usize());
    }
}

type Astar = sqrid::sqrid_create!(30, 15, false);
type Qa = sqrid::qa_create!(Astar);
type Gridbool = sqrid::gridbool_create!(Astar);

fn astar_data() -> Vec<(Qa, Qa, Gridbool)> {
    vec![
        (
            Qa::new::<11, 13>(),
            Qa::new::<21, 4>(),
            // Test 6:
            "##############################\
            ##.##....#.##.....##.###.....#\
            ##....##......####.......###.#\
            #..##.##.#.##.####.##.##.###.#\
            ##.##.....#........##C.......#\
            #.....###...###......###.#####\
            ###.#.###.#.....##..........##\
            #.........###.###.##........##\
            ###.###.#............###.##..#\
            ###.....###.###.#........##.##\
            #.########...##.##.##.#.#...##\
            #.............###..#.#......##\
            #.####.######.###.#######...##\
            ###........T#.......##......##\
            ##############################"
                .chars()
                .map(|c| c == '#')
                .collect::<Gridbool>(),
        ),
        (
            Qa::new::<22, 11>(),
            Qa::new::<25, 3>(),
            // Test 8:
            "##############################\
           #............................#\
           #..####################......#\
           #.....................#..C...#\
           #............###..##..#..#...#\
           ##.###########################\
           #...#.....##......##.........#\
           #...#..#..##..##..##..####...#\
           ###....#......##......##.....#\
           #..#.#######################.#\
           #..#.#................#......#\
           #..#.#................T......#\
           #..#.#######################.#\
           #............................#\
           ##############################"
                .chars()
                .map(|c| c == '#')
                .collect::<Gridbool>(),
        ),
    ]
}

fn astar_search(pars: &[(Qa, Qa, Gridbool)]) {
    for par in pars {
        let _ = Astar::astar_path(
            |qa, qr| sqrid::qaqr_eval(qa, qr).filter(|qa| !par.2.get(qa)),
            &par.0,
            &par.1,
        );
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("qaqr_mutual", |b| b.iter(|| qaqr_mutual()));
    c.bench_function("grid_index", |b| b.iter(|| grid_index()));
    let data = astar_data();
    c.bench_function("astar_search", |b| b.iter(|| astar_search(&data)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
