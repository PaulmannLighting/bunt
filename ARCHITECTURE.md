# Architecture

`bunt` is a small Rust library for deterministic color-space conversions. The
public API is exposed from `src/lib.rs` and consists of four value types:

- `Rgb`
- `LinearRgb`
- `Xyz`
- `Xy`

Each type owns its channel values and exposes simple constructors and accessors.
Conversions are implemented with standard `From` implementations so callers can
compose conversion steps explicitly or use the direct paths provided by the
crate.

## Module layout

```mermaid
flowchart TD
    lib["src/lib.rs"]
    rgb["src/rgb.rs"]
    linear["src/linear_rgb.rs"]
    xyz["src/xyz.rs"]
    coefficients["src/xyz/coefficients.rs"]
    xy["src/xy.rs"]

    lib --> rgb
    lib --> linear
    lib --> xyz
    lib --> xy
    xyz --> coefficients
```

## Conversion flow

```mermaid
flowchart LR
    rgb["Rgb\n8-bit gamma-encoded channels"]
    linear["LinearRgb\nnormalized linear-light channels"]
    xyz["Xyz\nCIE 1931 tristimulus values"]
    xy["Xy\nu16 fixed-point chromaticity"]

    rgb --> linear
    linear --> xyz
    xyz --> xy
    rgb --> xyz
    linear --> xy
    rgb --> xy
```

## Responsibilities

`Rgb` stores 8-bit gamma-encoded channel values and defines common color
constants.

`LinearRgb` applies the sRGB transfer function to convert gamma-encoded channels
into normalized linear-light intensities.

`Xyz` applies the crate's RGB-to-XYZ coefficient matrix. The matrix rows are
represented by the private `Coefficients` helper.

`Xy` converts XYZ tristimulus values into chromaticity coordinates. The result is
stored as two fixed-point `u16` values scaled over the full `0..=u16::MAX`
range.

## Feature flags

The optional `serde` feature derives `Serialize` and `Deserialize` for the color
value types. The feature does not change conversion behavior.
