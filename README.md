# **Rosu Pattern Dtector**

`rosu-pattern-detector` is a Rust-based library designed to analyze patterns in osu! beatmaps.
It currently supports **osu!mania** mode exclusively,
focusing on detecting specific **secondary patterns** such as **Jack**, **Jumpstream**, **Singlestream**, and *
*Handstream**,
and tertiary patterns see mania::struct for more info.
While osu!mania is the only mode supported right now, the project intends to evolve into a universal tool for analyzing
all osu! game modes.

---

## **Features**

- **Current Game Mode Support**: osu!mania only.
- **Secondary Pattern Analysis**:
    - Jack
    - Jumpstream
    - Singlestream
    - Handstream


- **Tertiary Pattern Analysis**:

### Jack Patterns

- Chordjack
- DenseChordjack
- ChordStream
- Speedjack
- All

### Jumpstream Patterns

- LightJs
- AnchorJs
- JS
- JT
- All

### Handstream Patterns

- LightHs
- AnchorHs
- DenseHs
- HS
- All

### Singlestream Patterns

- Singlestream
- All

**Extensible Design**:
    - Although limited to osu!mania for now, the library is structured for future extension to other osu! game modes (osu!standard, taiko, catch) and more advanced patterns.

---

## **Installation**

### In Rust:

Add the library and its dependencies to your `Cargo.toml` file:

```toml
[dependencies]
rosu-map = "0.2.0"
serde = { version = "1.0.217", features = ["derive"] }
serde_json = "1.0.137"
eyre = "0.6.12"
```

---

**Example JSON Output**:

```json
{
  "jack": 14.5,
  "jumpstream": 8.3,
  "singlestream": 12.0,
  "handstream": 5.4
}
```

---

## **Current Limitations**

- **osu!mania-only**:
    - The library currently does not support other osu! modes (osu!standard, taiko, or catch).

- **Secondary Patterns Only**:
    - Currently, it focuses on detecting only **secondary patterns**. More advanced **tertiary patterns** (e.g., polyrhythms, advanced time signature detection) will be added in future releases.

---

## **Roadmap**

- **Support for Other Game Modes**:
    - Add compatibility for osu!standard, taiko, and catch.

- **Advanced Patterns**:
    - Expand functionality to cover more complex patterns.

- **Performance Optimizations**:
    - Improve computation speed for large and complex maps.

---

## **Contributing**

Contributions are more than welcome! Whether it's adding support for a new game mode, detecting more advanced patterns, or optimizing the existing logic, feel free to open a pull request or submit an issue on GitHub.

Before contributing, ensure that your code adheres to the Rust standard practices, and provide relevant test cases to validate your changes.

---

## **License**

This project is licensed under the **MIT License**. You are free to use, modify, and distribute this library in compliance with the terms of the license.

---

If you have any feedback or questions, feel free to contribute or contact the project maintainers. Enjoy building your osu! map analysis tools with `pdetector`! 😊
