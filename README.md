[![CI](https://github.com/lpenz/sqrid/actions/workflows/ci.yml/badge.svg)](https://github.com/lpenz/sqrid/actions/workflows/ci.yml)
[![coveralls](https://coveralls.io/repos/github/lpenz/sqrid/badge.svg?branch=main)](https://coveralls.io/github/lpenz/sqrid?branch=main)
[![crates.io](https://img.shields.io/crates/v/sqrid)](https://crates.io/crates/sqrid)
[![doc.rs](https://docs.rs/sqrid/badge.svg)](https://docs.rs/sqrid)

# sqrid

*sqrid* provides square grid coordinates and related operations,
in a single-file create, with no dependencies.

## `Qa`: absolute coordinates, position

The [`Qa`] type represents an absolute position in a square
grid. The type itself receives the height and width of the grid as
const generic parameter.

We should usually create a type alias for the grid size we are using:

```rust
use sqrid;

type Qa = sqrid::Qa<6, 7>;
```

We can get [`Qa`] instances by:
- Using one of the const associated items:
  ```rust
  type Qa = sqrid::Qa<6, 7>;
  const MY_FIRST : Qa = Qa::FIRST;
  const MY_LAST : Qa = Qa::LAST;
  ```
- Using `try_from` with a `(i16, i16)` tuple or a tuple reference:
  ```rust
  use std::convert::TryFrom;
  use std::error::Error;

  type Qa = sqrid::Qa<6, 7>;

  fn main() -> Result<(), Box<dyn Error>> {
      let qa1 = Qa::try_from((2_i16, 3_i16))?;

      println!("qa1: {}", qa1);
      Ok(())
  }
  ```
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
- Calling [`Qa::iter`] to iterate all coordinates in the grid:
  ```rust
  type Qa = sqrid::Qa<6, 7>;
  for qa in Qa::iter() {
      println!("{}", qa);
  }
  ```

## `Qr`: relative coordinates, direction, movement

This type represents a relative movement of one square. It can
only be one of the 8 cardinal and intercardinal directions (N, NE,
E, SE, S, SW, W, NW).

It's a building block for paths, iterating on a [`Qa`] neighbors,
etc. It effectively represents the edges in a graph where the
[`Qa`] type represents nodes.

We can get [`Qr`] instances by:
- Using one of the const associated items that represent all
  cardinal directions (recommended):
  ```rust
  use sqrid::Qr;
  const RIGHT : Qr = Qr::E;
  const DOWN : Qr = Qr::S;
  ```
- Using `try_from` with a `(i16, i16)` tuple or a tuple reference:
  ```rust
  use std::convert::TryFrom;
  use std::error::Error;

  use sqrid::Qr;

  fn main() -> Result<(), Box<dyn Error>> {
      // Re-create West:
      let qr1 = Qr::try_from((0_i16, -1_i16))?;
      // Re-create Northeast:
      let qr2 = Qr::try_from((-1_i16, 1_i16))?;
      Ok(())
  }
  ```
- Calling [`Qr::new`], which checks the bounds in const contexts:
  ```rust
  use sqrid::Qr;
  const DOWNRIGHT : Qr = Qr::new::<1, 1>();
  ```
  The following, for instance, doesn't compile due to the
  arguments not being units:
  ```compile_fail
  use sqrid::Qr;
  const DOWNRIGHT2 : Qr = Qr::new::<2, 2>();
  ```
- Calling [`Qr::iter`] to iterate all directions:
  ```rust
  for qr in sqrid::Qr::iter::<true>() {
      println!("{}", qr);
  }
  ```
  The const argument to Qr::iter signals it to iterate over the
  intercardinal directions too. Passing `false` gets us only the 4
  cardinal directions.


[`Qa`]: https://docs.rs/sqrid/0/sqrid/_sqrid/struct.Qa.html
[`Qa::new`]: https://docs.rs/sqrid/0/sqrid/_sqrid/struct.Qa.html#method.new
[`Qa::iter`]: https://docs.rs/sqrid/0/sqrid/_sqrid/struct.Qa.html#method.iter
[`Qr`]: https://docs.rs/sqrid/0/sqrid/_sqrid/struct.Qr.html
[`Qr::new`]: https://docs.rs/sqrid/0/sqrid/_sqrid/struct.Qr.html#method.new
[`Qr::iter`]: https://docs.rs/sqrid/0/sqrid/_sqrid/struct.Qr.html#method.iter

