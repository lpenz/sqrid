[![CI](https://github.com/lpenz/sqrid/actions/workflows/ci.yml/badge.svg)](https://github.com/lpenz/sqrid/actions/workflows/ci.yml)
[![coveralls](https://coveralls.io/repos/github/lpenz/sqrid/badge.svg?branch=main)](https://coveralls.io/github/lpenz/sqrid?branch=main)
[![crates.io](https://img.shields.io/crates/v/sqrid)](https://crates.io/crates/sqrid)
[![doc.rs](https://docs.rs/sqrid/badge.svg)](https://docs.rs/sqrid)

# sqrid

*sqrid* provides square grid coordinates and related operations,
in a single-file create, with no dependencies.

## Usage

The [`Qa`] type represents an absolute position in a square
grid. The type itself receives the height and width of the grid as
const generic parameter.

We should usually create a type alias for the grid size we are using:

```rust
use sqrid;

type Qa = sqrid::Qa<6, 7>;
```

### Creating `Qa` instances

We can get [`Qa`] instances by:
- Using one of the const associated items:
  ```rust
  type Qa = sqrid::Qa<6, 7>;
  const MyFirst : Qa = Qa::FIRST;
  const MyLast : Qa = Qa::LAST;
  ```
- Using `try_from` with a `(i16, i16)` tuple or a tuple reference:
  ```rust
  use std::convert::TryFrom;
  use std::error::Error;

  type Qa = sqrid::Qa<6, 7>;
  const MyFirst : Qa = Qa::FIRST;

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
- Calling [`Qa::iter`] to iterate all coorinates in the grid:
  ```rust
  type Qa = sqrid::Qa<6, 7>;
  for qa in Qa::iter() {
      println!("{}", qa);
  }
  ```


[`Qa`]: https://docs.rs/sqrid/0/sqrid/struct.Qa.html
[`Qa::new`]: https://docs.rs/sqrid/0/sqrid/struct.Qa.html#method.new

