[![CI](https://github.com/lpenz/sqrid/actions/workflows/ci.yml/badge.svg)](https://github.com/lpenz/sqrid/actions/workflows/ci.yml)
[![coveralls](https://coveralls.io/repos/github/lpenz/sqrid/badge.svg?branch=main)](https://coveralls.io/github/lpenz/sqrid?branch=main)
[![crates.io](https://img.shields.io/crates/v/sqrid)](https://crates.io/crates/sqrid)
[![doc.rs](https://docs.rs/sqrid/badge.svg)](https://docs.rs/sqrid)

# sqrid

*sqrid* provides square grid coordinates and related operations,
in a crate with zero dependencies.

It's easier to explain the features of this crate in terms of the
types it provides:
- [`Qa`]: position, as absolute coordinates in a grid of fixed
  size. The dimensions of the grid are const generics type
  parameters; invalid coordinates can't be created.
- [`Qr`]: "movement", relative coordinates. These are the
  cardinal (and intercardinal) directions.
  Addition is implemented in the form of `Qa + Qr = Option<Qa>`,
  which can be `None` if the result is outside the grid.
- [`Grid`]: a `Qa`-indexed array.
- [`Gridbool`]: a bitmap-backed `Qa`-indexed grid of booleans.
- [`Sqrid`]: "factory" type that acts as an entry point to the
  fundamental types below and to algorithms.

Besides these fundamental types, we also have some algorithms
attached to [`Sqrid`]:
- [`Sqrid::bf_iter`]: breadth-first iteration
- [`Sqrid::bfs_path`]: breadth-first search that accepts an
  arbitrary `found` function.
- [`Sqrid::astar_path`]: A* search that takes a destination `Qa`

All basic types have the standard `iter`, `iter_mut`, `extend`,
`as_ref`, and conversion operations that should be expected.

## Fundamental types

### `Qa`: absolute coordinates, position

The [`Qa`] type represents an absolute position in a square
grid. The type itself receives the height and width of the grid as
const generic parameter.

We should usually create a type alias for the grid size we are using:

```rust
use sqrid;

type Qa = sqrid::Qa<6, 7>;
```

We can only generate [`Qa`] instances that are valid - i.e. inside
the grid. Some of the ways to create instances:
- Using one of the const associated items: [`Qa::FIRST`] and
  [`Qa::LAST`]; [`Qa::TOP_LEFT`], etc.; [`Qa::CENTER`].
- Using `try_from` with a `(i16, i16)` tuple or a tuple reference.
- Calling [`Qa::new`], which checks the bounds in const contexts:
  ```rust
  type Qa = sqrid::Qa<6, 7>;
  const MY_FIRST : Qa = Qa::new::<3, 4>();
  ```
  The following, for instance, doesn't compile:
  ```rust
  type Qa = sqrid::Qa<6, 7>;
  const MY_FIRST : Qa = Qa::new::<12, 4>();
  ```

### `Qr`: relative coordinates, direction, movement

The [`Qr`] type represents a relative movement of one square. It can
only be one of the 8 cardinal and intercardinal directions:
[`Qr::N`], [`Qr::NE`], [`Qr::E`], [`Qr::SE`], [`Qr::S`], [`Qr::SW`],
[`Qr::W`], [`Qr::NW`].

It's a building block for paths, iterating on a [`Qa`] neighbors,
etc. It effectively represents the edges in a graph where the
[`Qa`] type represents nodes.

All functions that iterate on `Qr` values accept a boolean const
argument that specifies whether the intercardinal directions
(`NE`, `SE`, `SW`, `NW`) should be considered.

### `Grid`: a `Qa`-indexed array

A [`grid`] is a generic array that can be indexed by a [`Qa`]

We can create the type from a suitable [`Sqrid`] type by using the
[`grid_create`] macro. We can then interact with specific lines
with [`Grid::line`] and [`Grid::line_mut`], or with the whole
underlying array with `as_ref` (see [`std::convert::AsRef`]) and
`as_mut` (see [`std::convert::AsMut`]).

Usage example:

```rust
type Sqrid = sqrid::sqrid_create!(3, 3, false);
type Qa = sqrid::qa_create!(Sqrid);
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
for (qa, &i) in gridnums.iter_qa() {
    println!("[{}] = {}", qa, i);
}

// And we can always use `as_ref` or `as_mut` to interact with the
// inner array directly. To reverse it, for example, with the
// [`std::slice::reverse`] function:
gridnums.as_mut().reverse();
```

### `Gridbool`: a bitmap-backed `Qa`-indexed grid of booleans

[`Gridbool`] is a compact abstraction of a grid of booleans.

The type itself can be created with [`gridbool_create`] macro.
It's optimized for getting and setting values at specific
coordinates, but we can also get all `true`/`false` coordinates
with suboptimal performance - in this case, the time is
proportional to the size of the grid and not to the quantity of
`true`/`false` values.

Usage example:

```rust
type Sqrid = sqrid::sqrid_create!(3, 3, false);
type Qa = sqrid::qa_create!(Sqrid);
type Gridbool = sqrid::gridbool_create!(Sqrid);

// We can create a gridbool from a Qa iterator via `collect`:
let mut gb = Qa::iter().filter(|qa| qa.is_corner()).collect::<Gridbool>();

// We can also set values from an iterator:
gb.set_iter_t(Qa::iter().filter(|qa| qa.is_side()));

// Iterate on the true/false values:
for b in gb.iter() {
    println!("{}", b);
}

// Iterate on the true coordinates:
for qa in gb.iter_t() {
    assert!(qa.is_side());
}

// Iterate on (coordinate, bool):
for (qa, b) in gb.iter_qa() {
    println!("[{}] = {}", qa, b);
}
```

## `Sqrid`: entry point for algorithms

The [`Qa`] type and some methods on the [`Qr`] type require const
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
type Qa = sqrid::qa_create!(Sqrid);
type Grid = sqrid::grid_create!(Sqrid, i32);
type Gridbool = sqrid::gridbool_create!(Sqrid);
```

## Algorithms

### Breadth-first traversal

The [`Sqrid::bf_iter`] function instantiates an iterator struct
([`BfIterator`]) that can be used to iterate coordinates in
breadth-first order, from a given origin, using a provided
function to evaluate a given [`Qa`] position + [`Qr`] direction
into the next `Qa` position.

Example usage:

```rust
type Sqrid = sqrid::sqrid_create!(3, 3, false);
type Qa = sqrid::qa_create!(Sqrid);

for (qa, qr) in Sqrid::bf_iter(&Qa::CENTER, sqrid::qaqr_eval)
                .flatten() {
    println!("breadth-first qa {} from {}", qa, qr);
}
```

### Breadth-first search

[`Sqrid::bfs_path`] takes an origin, a movement function and a
goal function, and figures out the shortest path to a goal by
using a breadth-first iteration.

The function returns the [`Qa`] that fulfills the goal and a
path in the form of a `Vec<Qr>`.

Example usage:

```rust
type Sqrid = sqrid::sqrid_create!(3, 3, false);
type Qa = sqrid::qa_create!(Sqrid);

// Generate the grid of "came from" directions from bottom-right to
// top-left:
if let Ok((goal, path)) = Sqrid::bfs_path(
                              &Qa::TOP_LEFT, sqrid::qaqr_eval,
                              |qa| qa == Qa::BOTTOM_RIGHT) {
    println!("goal: {}, path: {:?}", goal, path);
}
```

### A* search

[`Sqrid::astar_path`] takes a movement function, an origin and a
destination, and figures out the shortest path by using A*.

The function returns path in the form of a `Vec<Qr>`.

Example usage:

```rust
type Sqrid = sqrid::sqrid_create!(3, 3, false);
type Qa = sqrid::qa_create!(Sqrid);

// Generate the grid of "came from" directions from bottom-right to
// top-left:
if let Ok(path) = Sqrid::astar_path(sqrid::qaqr_eval, &Qa::TOP_LEFT,
                                    &Qa::BOTTOM_RIGHT) {
    println!("path: {:?}", path);
}
```

### Uniform-cost search

[`Sqrid::ucs_path`] takes a movement-cost function, an origin and a
destination, and figures out the path with the lowest cost by using
uniform-cost search, which is essentially a variation of
[Dijkstra](https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm).

The function returns path in the form of a `Vec<Qr>`.

Example usage:

```rust
type Sqrid = sqrid::sqrid_create!(3, 3, false);
type Qa = sqrid::qa_create!(Sqrid);

fn traverse(position: Qa, direction: sqrid::Qr) -> Option<(Qa, usize)> {
    let next_position = (position + direction)?;
    let cost = 1;
    Some((next_position, cost))
}

// Generate the grid of "came from" directions from bottom-right to
// top-left:
if let Ok(path) = Sqrid::ucs_path(traverse, &Qa::TOP_LEFT,
                                  &Qa::BOTTOM_RIGHT) {
    println!("path: {:?}", path);
}
```

[`std::convert::AsRef`]: https://doc.rust-lang.org/std/convert/trait.AsRef.html
[`std::convert::AsMut`]: https://doc.rust-lang.org/std/convert/trait.AsMut.html
[`Qa`]: https://docs.rs/sqrid/latest/sqrid/qa/struct.Qa.html
[`Qa::FIRST`]: https://docs.rs/sqrid/latest/sqrid/qa/struct.Qa.html#associatedconstant.FIRST
[`Qa::LAST`]: https://docs.rs/sqrid/latest/sqrid/qa/struct.Qa.html#associatedconstant.LAST
[`Qa::TOP_LEFT`]: https://docs.rs/sqrid/latest/sqrid/qa/struct.Qa.html#associatedconstant.TOP_LEFT
[`Qa::CENTER`]: https://docs.rs/sqrid/latest/sqrid/qa/struct.Qa.html#associatedconstant.CENTER
[`Qa::new`]: https://docs.rs/sqrid/latest/sqrid/qa/struct.Qa.html#method.new
[`Qa::iter`]: https://docs.rs/sqrid/latest/sqrid/qa/struct.Qa.html#method.iter
[`Qr`]: https://docs.rs/sqrid/latest/sqrid/qr/enum.Qr.html
[`Qr::iter`]: https://docs.rs/sqrid/latest/sqrid/qr/enum.Qr.html#method.iter
[`Qr::N`]: https://docs.rs/sqrid/latest/sqrid/qr/enum.Qr.html#variant.N
[`Qr::NE`]: https://docs.rs/sqrid/latest/sqrid/qr/enum.Qr.html#variant.NE
[`Qr::E`]: https://docs.rs/sqrid/latest/sqrid/qr/enum.Qr.html#variant.E
[`Qr::SE`]: https://docs.rs/sqrid/latest/sqrid/qr/enum.Qr.html#variant.SE
[`Qr::S`]: https://docs.rs/sqrid/latest/sqrid/qr/enum.Qr.html#variant.S
[`Qr::SW`]: https://docs.rs/sqrid/latest/sqrid/qr/enum.Qr.html#variant.SW
[`Qr::W`]: https://docs.rs/sqrid/latest/sqrid/qr/enum.Qr.html#variant.W
[`Qr::NW`]: https://docs.rs/sqrid/latest/sqrid/qr/enum.Qr.html#variant.NW
[`Grid`]: https://docs.rs/sqrid/latest/sqrid/grid/struct.Grid.html
[`grid_create`]: https://docs.rs/sqrid/latest/sqrid/macro.grid_create.html
[`Grid::line`]: https://docs.rs/sqrid/latest/sqrid/grid/struct.Grid.html#method.line
[`Grid::line_mut`]: https://docs.rs/sqrid/latest/sqrid/grid/struct.Grid.html#method.line_mut
[`Gridbool`]: https://docs.rs/sqrid/latest/sqrid/gridbool/struct.Gridbool.html
[`gridbool_create`]: https://docs.rs/sqrid/latest/sqrid/macro.gridbool_create.html
[`Sqrid`]: https://docs.rs/sqrid/latest/sqrid/struct.Sqrid.html
[`Sqrid::bf_iter`]: https://docs.rs/sqrid/latest/sqrid/base/struct.Sqrid.html#method.bf_iter
[`BfIterator`]: https://docs.rs/sqrid/latest/sqrid/struct.BfIterator.html
[`Sqrid::bfs_path`]: https://docs.rs/sqrid/latest/sqrid/base/struct.Sqrid.html#method.bfs_path
[`Sqrid::astar_path`]: https://docs.rs/sqrid/latest/sqrid/base/struct.Sqrid.html#method.astar_path
[`Sqrid::ucs_path`]: https://docs.rs/sqrid/latest/sqrid/base/struct.Sqrid.html#method.ucs_path

