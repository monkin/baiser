# Baiser

[![Tests Status](https://github.com/monkin/baiser/actions/workflows/tests.yml/badge.svg)](https://github.com/monkin/baiser/actions/workflows/tests.yml)
[![Build Status](https://github.com/monkin/baiser/actions/workflows/build.yml/badge.svg)](https://github.com/monkin/baiser/actions/workflows/build.yml)
[![Clippy Status](https://github.com/monkin/baiser/actions/workflows/clippy.yml/badge.svg)](https://github.com/monkin/baiser/actions/workflows/clippy.yml)
[![Format Status](https://github.com/monkin/baiser/actions/workflows/format.yml/badge.svg)](https://github.com/monkin/baiser/actions/workflows/format.yml)

Baiser is a Rust library designed to work with curves, providing tools for creating and manipulating them with ease. It
offers a variety of curve types, including Bezier curves of different orders, and utilities for composing curves. One of
the library's key features is the ability to adjust curves to have linear speed, making it particularly useful for
animations, and any application where consistent movement along a curve is required.

## Features

- **Bezier Curves**: Supports Bézier curves of up to third order (`Bezier0` (dot), `Bezier1` (
  line), `Bezier2`, `Bezier3`), allowing for a wide range of shapes and motions.
- **Composed Curve**: Enables the combination of multiple curves into a single, continuous path.
- **Curve Manipulation**: Provides functionality to calculate points and tangents along curves, making it easier to work
  with geometric shapes.
- **Linear Speed Adjustment**: Includes a `LinearSpeed` struct that adjusts a given curve to ensure a linear
  relationship between time and distance traveled along the curve.
- **Any Point Type**: Curves work well with any point type implementing the `CurvePoint` trait. For data types that
  implement `Distance` trait, building a linear speed curve is as simple as calling `Curve::linear_speed` method.

## Usage

Here's a quick example of how to create a linear speed adjusted Bezier curve:

```rust
use baiser::Curve;

let linear_speed_curve = Curve::cubic_bezier(v1, v2, v3, v4)
    .linear_speed(/* table size */ 64, /* steps count */ 100);

// Now you can use `linear_speed_curve` for operations requiring linear speed
let middle_value = linear_speed_curve.point_at(0.5);

// An example of building a composed curve
let composed_curve = Curve::composed(p0)
    .line_to(p1, p2)
    .quadratic_to(p3, p4)
    .cubic_to(p5, p6, p7);
```

## Contributing

Contributions are welcome! Please feel free to submit pull requests or open issues to discuss potential improvements or
features.

## License

This project is licensed under the MIT License - see the [LICENSE](./LICENSE.md) file for details.
