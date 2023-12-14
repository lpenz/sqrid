// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

#![warn(rust_2018_idioms)]

use criterion::{criterion_group, criterion_main, Criterion};

fn mov_mutual() {
    type Pos = sqrid::Pos<256, 257>;
    for pos in Pos::iter() {
        for dir in sqrid::Dir::iter::<true>() {
            if let Ok(pos2) = pos + dir {
                let found = sqrid::Dir::iter::<true>()
                    .filter(|dir| pos2 + *dir == Ok(pos))
                    .next()
                    .is_some();
                assert!(found);
            }
        }
        for dir in sqrid::Dir::iter::<false>() {
            if let Ok(pos2) = pos + dir {
                let found = sqrid::Dir::iter::<false>()
                    .filter(|dir| pos2 + *dir == Ok(pos))
                    .next()
                    .is_some();
                assert!(found);
            }
        }
    }
}

fn grid_index() {
    type Pos = sqrid::Pos<256, 257>;
    type Grid = sqrid::Grid<usize, 256, 257, { 256 * 257 }>;
    let mut g = Grid::default();
    for pos in Pos::iter() {
        g[pos] = pos.to_usize();
    }
    for pos in Pos::iter() {
        assert_eq!(g[pos], pos.to_usize());
    }
}

type Astar = sqrid::sqrid_create!(30, 15, false);
type Pos = sqrid::pos_create!(Astar);
type Gridbool = sqrid::gridbool_create!(Astar);

fn astar_data() -> Vec<(Pos, Pos, Gridbool)> {
    vec![
        (
            Pos::new_static::<11, 13>(),
            Pos::new_static::<21, 4>(),
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
            Pos::new_static::<22, 11>(),
            Pos::new_static::<25, 3>(),
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

fn astar_search(pars: &[(Pos, Pos, Gridbool)]) {
    for par in pars {
        let _ = Astar::astar_path(
            |pos, dir| sqrid::mov_eval(pos, dir).filter(|pos| !par.2.get(pos)),
            &par.0,
            &par.1,
        );
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("mov_mutual", |b| b.iter(|| mov_mutual()));
    c.bench_function("grid_index", |b| b.iter(|| grid_index()));
    let data = astar_data();
    c.bench_function("astar_search", |b| b.iter(|| astar_search(&data)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
