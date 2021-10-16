// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use sqrid;

use anyhow::anyhow;
use anyhow::Result;

type Sqrid = sqrid::sqrid_create!(30, 15, false);
type Qa = sqrid::qa_create!(Sqrid);
// type GridStr = sqrid::grid_create!(Sqrid, &'static str);
type Gridbool = sqrid::gridbool_create!(Sqrid);

fn walls_from_str(wallstr: &Vec<&str>) -> (Gridbool, Qa, Qa) {
    let mut walls = Gridbool::default();
    let mut start = Qa::FIRST;
    let mut end = Qa::LAST;
    for y in 0..Qa::HEIGHT {
        for x in 0..Qa::WIDTH {
            let c = wallstr[y as usize].as_bytes()[x as usize] as char;
            let qa = Qa::tryfrom_tuple((x, y)).unwrap();
            walls.set(qa, c == '#');
            if c == 'T' {
                start = qa;
            } else if c == 'C' {
                end = qa;
            }
        }
    }
    (walls, start, end)
}

fn path(wall: &Gridbool) -> Box<impl Fn(Qa, sqrid::Qr) -> Option<Qa> + '_> {
    Box::new(move |qa: Qa, qr| {
        {
            let newqa: Option<Qa> = qa + qr;
            newqa
        }
        .filter(|qa| !wall.get(qa))
    })
}

fn goal(end: &Qa) -> Box<impl Fn(Qa) -> bool + '_> {
    Box::new(move |qa| qa == *end)
}

fn test_variant(distance: usize, wall: Gridbool, start: &Qa, end: &Qa) -> Result<()> {
    eprintln!("start {}, end {}", start, end);
    let mut qa = *start;
    let mut i = distance;
    while qa != *end {
        eprintln!("from {} to {}: {}", qa, end, i);
        eprintln!("{}", wall);
        // BFS:
        let (g1, dirgrid) = Sqrid::bfs_qrgrid(path(&wall), &qa, goal(&end))?;
        assert_eq!(g1, *end);
        let path1 = dirgrid.camefrom_into_path(&qa, &end)?;
        let (g2, path2) = Sqrid::bfs_path(path(&wall), &qa, goal(&end))?;
        assert_eq!(g1, g2);
        assert_eq!(path1, path2);
        eprintln!("{:?}", path1);
        assert_eq!(path1.len(), i);
        // A*:
        let dirgrid = Sqrid::astar_qrgrid(path(&wall), &qa, end)?;
        let path1 = dirgrid.camefrom_into_path(&qa, &end)?;
        let path2 = Sqrid::astar_path(path(&wall), &qa, end)?;
        assert_eq!(path1, path2);
        assert_eq!(path1.len(), i);
        // Try next coordinate:
        let last = path1.last().ok_or(anyhow!("unexpected empty path"))?;
        qa = (qa + last).ok_or(anyhow!("sum failed"))?;
        i -= 1;
    }
    Ok(())
}

fn do_test(distance: usize, wallstr: &Vec<&str>) -> Result<()> {
    let (mut wall, mut start, mut end) = walls_from_str(wallstr);
    test_variant(distance, wall, &start, &end)?;
    wall.flip_h();
    start = start.flip_h();
    end = end.flip_h();
    test_variant(distance, wall, &start, &end)?;
    wall.flip_v();
    start = start.flip_v();
    end = end.flip_v();
    test_variant(distance, wall, &start, &end)?;
    wall.flip_h();
    start = start.flip_h();
    end = end.flip_h();
    test_variant(distance, wall, &start, &end)?;
    Ok(())
}

#[test]
fn test_bfs1() -> Result<()> {
    do_test(
        6,
        &vec![
            "##############################",
            "##############################",
            "#####################C.....T##",
            "##############################",
            "##############################",
            "##############################",
            "##############################",
            "##############################",
            "##############################",
            "##############################",
            "##############################",
            "##############################",
            "##############################",
            "##############################",
            "##############################",
        ],
    )
}

#[test]
fn test_bfs4() -> Result<()> {
    do_test(
        68,
        &vec![
            //00000000011111111112222222222
            //12345678901234567890123456789
            "##############################", //  0
            "##.###########.###############", //  1
            "#...#.######.#......#......T##", //  2
            "###.#.######.#.######.########", //  3
            "##...........#.######.########", //  4
            "###.#.######......###.########", //  5
            "#...#.###....#.##.###.########", //  6
            "#.############.##.....########", //  7
            "#......##......###############", //  8
            "###.###############..........#", //  9
            "###.#####......####.########.#", // 10
            "###.......####............####", // 11
            "##############.######.###.##.#", // 12
            "####C..........###....###....#", // 13
            "##############################", // 14
        ],
    )
}

#[test]
fn test_bfs5() -> Result<()> {
    do_test(
        42,
        &vec![
            //00000000011111111112222222222
            //12345678901234567890123456789
            "##############################", //  0
            "####.....#..........T........#", //  1
            "##########.#################.#", //  2
            "####.......................#.#", //  3
            "####.##########.##########.#.#", //  4
            "####.......................#.#", //  5
            "##########.#########.#######.#", //  6
            "##########...........#######.#", //  7
            "##########.#########.#######.#", //  8
            "####.......................#.#", //  9
            "####.##########.##########.#.#", // 10
            "####.......................#.#", // 11
            "##########.#########.#######.#", // 12
            "####.....#C###########.....#.#", // 13
            "##############################", // 14
        ],
    )
}

#[test]
fn test_bfs6() -> Result<()> {
    do_test(
        33,
        &vec![
            "##############################",
            "##.##....#.##.....##.###.....#",
            "##....##......####.......###.#",
            "#..##.##.#.##.####.##.##.###.#",
            "##.##.....#........##C.......#",
            "#.....###...###......###.#####",
            "###.#.###.#.....##..........##",
            "#.........###.###.##........##",
            "###.###.#............###.##..#",
            "###.....###.###.#........##.##",
            "#.########...##.##.##.#.#...##",
            "#.............###..#.#......##",
            "#.####.######.###.#######...##",
            "###........T#.......##......##",
            "##############################",
        ],
    )
}

#[test]
fn test_bfs7() -> Result<()> {
    do_test(
        70,
        &vec![
            "##############################",
            "#............................#",
            "#.#######################.#..#",
            "#.....T.................#.#..#",
            "#.....#.................#.#..#",
            "#.#######################.#..#",
            "#.....##......##......#....###",
            "#...####..##..##..##..#..#...#",
            "#.........##......##.....#...#",
            "###########################.##",
            "#......#......#..............#",
            "#...C..#.....................#",
            "#...#..####################..#",
            "#............................#",
            "##############################",
        ],
    )
}

#[test]
fn test_bfs8() -> Result<()> {
    do_test(
        71,
        &vec![
            "##############################",
            "#............................#",
            "#..####################......#",
            "#.....................#..C...#",
            "#............###..##..#..#...#",
            "##.###########################",
            "#...#.....##......##.........#",
            "#...#..#..##..##..##..####...#",
            "###....#......##......##.....#",
            "#..#.#######################.#",
            "#..#.#................#......#",
            "#..#.#................T......#",
            "#..#.#######################.#",
            "#............................#",
            "##############################",
        ],
    )
}
