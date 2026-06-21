
# Mirl Windowing Core (0.0.0-alpha)

#### Ciwi 🥝 - Traits and helpers for creating interactive windows

<details>
<summary>Flags<summary>

### Default:

**Core**

- `std` (Default)
- `c_compatible`

**Codec**

- `serde`
- `bitcode`
- `wincode` (bitcode recommended)

**Enum**

- `strum`
- `enum_ext`

### Custom: TODO

</details>

### Purpose

A lib from where any backend trait-impls may pull functions and traits from, for a unified api.

### Disclaimer

This lib lower level and as such not front-end dev friendly, for existing implementations refer to the "[See also](#see-also)" section.

### Origin

Each backend pulls from this crate and the [super crate](https://crates.io/crates/mirl_windowing) ties every backend together

### TODO

1. Move implementations into main lib
2. Implement the rest of the traits for all backends (minifb+glfw)
3. Implement the console backend
4. Add tiling render mode? Or some other backend specific rendering method.

### See Also

- [`mirl_windowing_minifb`](https://crates.io/crates/mirl_windowing_minifb) - An implementation of this libs api for [`MiniFb`](https://crates.io/crates/minifb)
- [`mirl_windowing_glfw`](https://crates.io/crates/mirl_windowing_glfw) - An implementation of this libs api for [`glfw-rs`](https://crates.io/crates/glfw)
- [`mirl_windowing`](https://crates.io/crates/mirl_windowing) - A wrapper for the above mentioned libs
