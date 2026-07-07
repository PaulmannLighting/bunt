# bunt

`bunt` converts colors between gamma-encoded RGB, linear RGB, CIE 1931 XYZ, and
CIE 1931 xy chromaticity coordinates.

The crate is intentionally small and focuses on the conversion path commonly
needed by lighting and device-control software:

```text
Rgb -> LinearRgb -> Xyz -> Xy
```

## Installation

```toml
[dependencies]
bunt = "0.1"
```

Enable the optional `serde` feature when color values need to be serialized or
deserialized:

```toml
[dependencies]
bunt = { version = "0.1", features = ["serde"] }
```

## Usage

Convert an 8-bit RGB color directly into fixed-point xy coordinates:

```rust
use bunt::{Rgb, Xy};

let xy = Xy::from(Rgb::new(255, 0, 0));

assert_eq!(xy, Xy::new(0xB35A, 0x4C9F));
```

Use the intermediate color spaces when an application needs to inspect or reuse
the conversion steps:

```rust
use bunt::{LinearRgb, Rgb, Xy, Xyz};

let rgb = Rgb::new(128, 64, 32);
let linear = LinearRgb::from(rgb);
let xyz = Xyz::from(linear);
let xy = Xy::from(xyz);

assert!(linear.red() > linear.green());
assert!(xy.x() > 0);
```

## Types

- `Rgb` stores gamma-encoded 8-bit red, green, and blue channels.
- `LinearRgb` stores normalized linear-light red, green, and blue intensities.
- `Xyz` stores CIE 1931 XYZ tristimulus values.
- `Xy` stores CIE 1931 xy chromaticity as two `u16` fixed-point channels scaled
  over `0..=u16::MAX`.

## Fixed-point xy coordinates

`Xy` uses the full `u16` range because many compact protocols exchange
chromaticity as integer channel values. To recover normalized coordinates,
divide each channel by `u16::MAX`:

```rust
use bunt::Xy;

let xy = Xy::new(0x8000, 0x4000);
let normalized_x = f32::from(xy.x()) / f32::from(u16::MAX);
let normalized_y = f32::from(xy.y()) / f32::from(u16::MAX);

assert!(normalized_x > normalized_y);
```
