[![CI](https://github.com/lpenz/sqrid/actions/workflows/ci.yml/badge.svg)](https://github.com/lpenz/sqrid/actions/workflows/ci.yml)
[![coveralls](https://coveralls.io/repos/github/lpenz/sqrid/badge.svg?branch=main)](https://coveralls.io/github/lpenz/sqrid?branch=main)
[![crates.io](https://img.shields.io/crates/v/sqrid)](https://crates.io/crates/sqrid)
[![doc.rs](https://docs.rs/sqrid/badge.svg)](https://docs.rs/sqrid)

# sqrid

*sqrid* provides square grid coordinates and related operations,
in a crate with zero dependencies.

It's easier to explain the features of this crate in terms of the
types it provides:
- [`Pos`]: position, as absolute coordinates in a grid of fixed
  size. The dimensions of the grid are const generics type
  parameters; invalid coordinates can't be created.
- [`Dir`]: "movement", relative coordinates. These are the
  cardinal (and intercardinal) directions.
  Addition is implemented in the form of `Pos + Dir = Option<Pos>`,
  which can be `None` if the result is outside the grid.
- [`Grid`]: a `Pos`-indexed array.
- [`Gridbool`]: a bitmap-backed `Pos`-indexed grid of booleans.
- [`Sqrid`]: "factory" type that acts as an entry point to the
  fundamental types below and to algorithms.

We also have traits that generalize `Grid` and `Gridbool`:
- [`MapPos`]: trait that maps `Pos` to parameterized items;
  it's implemented by `Grid`, and some `HashMap`/`BTreeMap` based types.
- [`SetPos`]: trait that maps each `Pos` to a bool; it's implemented
  by `Gridbool`, `HashSet<Pos>` and `BTreeSet<Pos>`.

We then use these generalization to implement some grid algorithms:
- [`bf`]: breadth-first iteration and search.
- [`astar`]: A* search that takes a destination `Pos`.
- [`ucs`]: uniform-cost search.

All basic types have the standard `iter`, `iter_mut`, `extend`,
`as_ref`, and conversion operations that should be expected.

## Fundamental types

### `Pos`: absolute coordinates, position

The [`Pos`] type represents an absolute position in a square
grid. The type itself receives the height and width of the grid as
const generic parameter.

We should usually create a type alias for the grid size we are using:

```rust
use sqrid;

type Pos = sqrid::Pos<6, 7>;

let pos = Pos::new(3, 3)?;
```

We can only generate [`Pos`] instances that are inside the passed
dimensions.

### `Dir`: relative coordinates, direction, movement

The [`Dir`] type represents a relative movement of one square. It
can only be one of the 8 cardinal and intercardinal directions:
[`Dir::N`], [`Dir::NE`], [`Dir::E`], [`Dir::SE`], [`Dir::S`],
[`Dir::SW`], [`Dir::W`], [`Dir::NW`].

It's a building block for paths, iterating on a [`Pos`] neighbors,
etc. It effectively represents the edges in a graph where the
[`Pos`] type represents nodes.

All functions that iterate on `Dir` values accept a boolean const
argument that specifies whether the intercardinal directions
(`NE`, `SE`, `SW`, `NW`) should be considered.

### `Grid`: a `Pos`-indexed array

A [`Grid`] is a generic array that can be indexed by a [`Pos`].

We can create the type from a suitable [`Sqrid`] type by using the
[`grid_create`] macro. We can then interact with specific lines
with [`Grid::line`] and [`Grid::line_mut`], or with the whole
underlying array with `as_ref` (see [`std::convert::AsRef`]) and
`as_mut` (see [`std::convert::AsMut`]).

Usage example:

```rust
type Sqrid = sqrid::sqrid_create!(3, 3, false);
type Pos = sqrid::pos_create!(Sqrid);
type Grid = sqrid::grid_create!(Sqrid, i32);

// The grid create macro above is currently equivalent to:
type Grid2 = sqrid::Grid<i32, { Sqrid::WIDTH }, { Sqrid::HEIGHT },
                              { (Sqrid::WIDTH * Sqrid::HEIGHT) as usize }>;

// We can create grids from iterators via `collect`:
let mut gridnums = (0..9).collect::<Grid>();

// Iterate on their members:
for i in &gridnums {
    println!("i {}", i);
}

// Change the members in a loop:
for i in &mut gridnums {
    *i *= 10;
}

// Iterate on (coordinate, member) tuples:
for (pos, &i) in gridnums.iter_pos() {
    println!("[{}] = {}", pos, i);
}

// And we can always use `as_ref` or `as_mut` to interact with the
// inner array directly. To reverse it, for example, with the
// [`std::slice::reverse`] function:
gridnums.as_mut().reverse();
```

### `Gridbool`: a bitmap-backed `Pos`-indexed grid of booleans

The [`Gridbool`] is a compact abstraction of a grid of booleans.

The type itself can be created with [`gridbool_create`] macro.
It's optimized for getting and setting values at specific
coordinates, but we can also get all `true`/`false` coordinates
with suboptimal performance - in this case, the time is
proportional to the size of the grid and not to the quantity of
`true`/`false` values.

Usage example:

```rust
type Sqrid = sqrid::sqrid_create!(3, 3, false);
type Pos = sqrid::pos_create!(Sqrid);
type Gridbool = sqrid::gridbool_create!(Sqrid);

// We can create a gridbool from a Pos iterator via `collect`:
let mut gb = Pos::iter().filter(|pos| pos.is_corner()).collect::<Gridbool>();

// We can also set values from an iterator:
gb.set_iter_t(Pos::iter().filter(|pos| pos.is_side()));

// Iterate on the true/false values:
for b in gb.iter() {
    println!("{}", b);
}

// Iterate on the true coordinates:
for pos in gb.iter_t() {
    assert!(pos.is_side());
}

// Iterate on (coordinate, bool):
for (pos, b) in gb.iter_pos() {
    println!("[{}] = {}", pos, b);
}
```

## `Sqrid`: entry point for algorithms

The [`Pos`] type and some methods on the [`Dir`] type require const
generic arguments that usually don't change inside an application.
Both [`Grid`] and [`Gridbool`] also require further arguments that
can actually be derived from the width and height of the grid, but
that have to be explicitly specified due to some Rust limitations.

To make the creation of these types easier, we provide the
[`Sqrid`] type, which acumulates all const generic parameters and
can be used to create the other types via macros.

Example usage:

```rust
type Sqrid = sqrid::sqrid_create!(4, 4, false);
type Pos = sqrid::pos_create!(Sqrid);
type Grid = sqrid::grid_create!(Sqrid, i32);
type Gridbool = sqrid::gridbool_create!(Sqrid);
```

## Algorithms

### Breadth-first traversal

The [`Sqrid::bf_iter`] function instantiates an iterator struct
([`bf::BfIterator`]) that can be used to iterate coordinates in
breadth-first order, from a given origin, using a provided
function to evaluate a given [`Pos`] position + [`Dir`] direction
into the next `Pos` position.

Example usage:

```rust
type Sqrid = sqrid::sqrid_create!(3, 3, false);
type Pos = sqrid::pos_create!(Sqrid);

for (pos, dir) in Sqrid::bf_iter(sqrid::mov_eval, &Pos::CENTER)
                .flatten() {
    println!("breadth-first pos {} from {}", pos, dir);
}
```

### Breadth-first search

[`Sqrid::bfs_path`] takes an origin, a movement function and a
goal function, and figures out the shortest path to a goal by
using a breadth-first iteration.

The function returns the [`Pos`] that fulfills the goal and a
path in the form of a `Vec<Dir>`.

Example usage:

```rust
type Sqrid = sqrid::sqrid_create!(3, 3, false);
type Pos = sqrid::pos_create!(Sqrid);

// Generate the grid of "came from" directions from bottom-right to
// top-left:
if let Ok((goal, path)) = Sqrid::bfs_path(
                              sqrid::mov_eval, &Pos::TOP_LEFT,
                              |pos| pos == Pos::BOTTOM_RIGHT) {
    println!("goal: {}, path: {:?}", goal, path);
}
```

### A* search

[`Sqrid::astar_path`] takes a movement function, an origin and a
destination, and figures out the shortest path by using A*.

The function returns path in the form of a `Vec<Dir>`.

Example usage:

```rust
type Sqrid = sqrid::sqrid_create!(3, 3, false);
type Pos = sqrid::pos_create!(Sqrid);

// Generate the grid of "came from" directions from bottom-right to
// top-left:
if let Ok(path) = Sqrid::astar_path(sqrid::mov_eval, &Pos::TOP_LEFT,
                                    &Pos::BOTTOM_RIGHT) {
    println!("path: {:?}", path);
}
```

### Uniform-cost search

[`Sqrid::ucs_path`] takes a movement-cost function, an origin and a
destination, and figures out the path with the lowest cost by using
uniform-cost search, which is essentially a variation of
[Dijkstra](https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm).

The function returns path in the form of a `Vec<Dir>`.

Example usage:

```rust
type Sqrid = sqrid::sqrid_create!(3, 3, false);
type Pos = sqrid::pos_create!(Sqrid);

fn traverse(position: Pos, direction: sqrid::Dir) -> Option<(Pos, usize)> {
    let next_position = (position + direction).ok()?;
    let cost = 1;
    Some((next_position, cost))
}

// Generate the grid of "came from" directions from bottom-right to
// top-left:
if let Ok(path) = Sqrid::ucs_path(traverse, &Pos::TOP_LEFT,
                                  &Pos::BOTTOM_RIGHT) {
    println!("path: {:?}", path);
}
```

[`std::convert::AsRef`]: https://doc.rust-lang.org/std/convert/trait.AsRef.html
[`std::convert::AsMut`]: https://doc.rust-lang.org/std/convert/trait.AsMut.html
[`Pos`]: https://docs.rs/sqrid/latest/sqrid/Pos/struct.Pos.html
[`Pos::FIRST`]: https://docs.rs/sqrid/latest/sqrid/Pos/struct.Pos.html#associatedconstant.FIRST
[`Pos::LAST`]: https://docs.rs/sqrid/latest/sqrid/Pos/struct.Pos.html#associatedconstant.LAST
[`Pos::TOP_LEFT`]: https://docs.rs/sqrid/latest/sqrid/Pos/struct.Pos.html#associatedconstant.TOP_LEFT
[`Pos::CENTER`]: https://docs.rs/sqrid/latest/sqrid/Pos/struct.Pos.html#associatedconstant.CENTER
[`Pos::new`]: https://docs.rs/sqrid/latest/sqrid/Pos/struct.Pos.html#method.new
[`Pos::iter`]: https://docs.rs/sqrid/latest/sqrid/Pos/struct.Pos.html#method.iter
[`Dir`]: https://docs.rs/sqrid/latest/sqrid/Dir/enum.Dir.html
[`Dir::iter`]: https://docs.rs/sqrid/latest/sqrid/Dir/enum.Dir.html#method.iter
[`Dir::N`]: https://docs.rs/sqrid/latest/sqrid/Dir/enum.Dir.html#variant.N
[`Dir::NE`]: https://docs.rs/sqrid/latest/sqrid/Dir/enum.Dir.html#variant.NE
[`Dir::E`]: https://docs.rs/sqrid/latest/sqrid/Dir/enum.Dir.html#variant.E
[`Dir::SE`]: https://docs.rs/sqrid/latest/sqrid/Dir/enum.Dir.html#variant.SE
[`Dir::S`]: https://docs.rs/sqrid/latest/sqrid/Dir/enum.Dir.html#variant.S
[`Dir::SW`]: https://docs.rs/sqrid/latest/sqrid/Dir/enum.Dir.html#variant.SW
[`Dir::W`]: https://docs.rs/sqrid/latest/sqrid/Dir/enum.Dir.html#variant.W
[`Dir::NW`]: https://docs.rs/sqrid/latest/sqrid/Dir/enum.Dir.html#variant.NW
[`Grid`]: https://docs.rs/sqrid/latest/sqrid/grid/struct.Grid.html
[`grid_create`]: https://docs.rs/sqrid/latest/sqrid/macro.grid_create.html
[`Grid::line`]: https://docs.rs/sqrid/latest/sqrid/grid/struct.Grid.html#method.line
[`Grid::line_mut`]: https://docs.rs/sqrid/latest/sqrid/grid/struct.Grid.html#method.line_mut
[`Gridbool`]: https://docs.rs/sqrid/latest/sqrid/gridbool/struct.Gridbool.html
[`gridbool_create`]: https://docs.rs/sqrid/latest/sqrid/macro.gridbool_create.html
[`Sqrid`]: https://docs.rs/sqrid/latest/sqrid/struct.Sqrid.html
[`Sqrid::bf_iter`]: https://docs.rs/sqrid/latest/sqrid/base/struct.Sqrid.html#method.bf_iter
[`bf::BfIterator`]: https://docs.rs/sqrid/latest/sqrid/struct.BfIterator.html
[`bf`]: https://docs.rs/sqrid/latest/sqrid/bf
[`astar`]: https://docs.rs/sqrid/latest/sqrid/astar
[`ucs`]: https://docs.rs/sqrid/latest/sqrid/ucs
[`Sqrid::bfs_path`]: https://docs.rs/sqrid/latest/sqrid/base/struct.Sqrid.html#method.bfs_path
[`Sqrid::astar_path`]: https://docs.rs/sqrid/latest/sqrid/base/struct.Sqrid.html#method.astar_path
[`Sqrid::ucs_path`]: https://docs.rs/sqrid/latest/sqrid/base/struct.Sqrid.html#method.ucs_path
[`MapPos`]: https://docs.rs/sqrid/latest/sqrid/mappos/trait.MapPos.html
[`SetPos`]: https://docs.rs/sqrid/latest/sqrid/setpos/trait.SetPos.html
