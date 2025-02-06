
# OUTDATED I NEED TO UPDATE IT DONT READ IT

# **osu! Pattern Detector**

`pdetector` is a Rust-based library designed to analyze patterns in osu! beatmaps. It currently supports **osu!mania** mode exclusively, focusing on detecting specific **secondary patterns** such as **Jack**, **Jumpstream**, **Singlestream**, and **Handstream**. While osu!mania is the only mode supported right now, the project intends to evolve into a universal tool for analyzing all osu! game modes.

---

## **Features**

- **Current Game Mode Support**: osu!mania only.
- **Secondary Pattern Analysis**:
    - Jack
    - Jumpstream
    - Singlestream
    - Handstream
- **Extensible Design**:
    - Although limited to osu!mania for now, the library is structured for future extension to other osu! game modes (osu!standard, taiko, catch) and more advanced patterns.

---

## **Installation**

### In Rust:

Add the library and its dependencies to your `Cargo.toml` file:

```toml
[dependencies]
rosu-map = "0.2.0"
serde = "1.0"
serde_json = "1.0"
pyo3 = "0.20.3"
reqwest = "0.12.12"
```

---

## **Usage Examples**

### **Python Usage**

You can use the library in Python via the bundled PyO3 bindings. The example below demonstrates how to analyze an osu!mania beatmap for secondary patterns.

**Code**:

```python
import pdetector

# Example URL for downloading an osu!mania map
url = "https://example.com/path/to/osu/map.osu"

# Analyze the beatmap and retrieve detected patterns
result = pdetector.get_map(url)

# Display the detected patterns in JSON format
print(result)
```

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

### **Rust Usage**

You can use the library directly in Rust to analyze beatmaps.

**Code**:

```rust
use pdetector::{get_map, download_file};

fn main() {
    let url = "https://example.com/path/to/osu/map.osu";

    // Download and analyze an osu!mania beatmap
    match download_file(url) {
        Ok(data) => {
            let results = get_map(&data).unwrap();
            println!("JSON Results: {}", results);
        }
        Err(e) => println!("Error: {}", e),
    }
}
```

After running the above Rust program, the detected patterns will be printed as JSON.

---

## **How It Works**

The library analyzes an osu!mania beatmap in the following steps:

1. **Get the Beatmap**:
    - The `.osu` file is downloaded from the provided URL using the `reqwest` crate.

2. **Transform Objects**:
    - The hit objects of the `.osu` file are mapped to internal representations specific to osu!mania.

3. **Group Notes into Measures**:
    - Notes per measure are grouped for precise analysis based on timings and patterns.

4. **Secondary Pattern Detection**:
    - Rules are applied to detect secondary patterns:
        - **Jack**: Repeated notes in the same column.
        - **Jumpstream**: Consecutive "jumps."
        - **Singlestream**: Uninterrupted single-note sequences.
        - **Handstream**: Alternating dense streams.

5. **JSON Results**:
    - The detected values of each pattern type are compiled into a JSON object.

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
    - Expand functionality to cover more complex and tertiary-level patterns.

- **Custom Pattern Configurations**:
    - Allow users to define and detect customized patterns.

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
