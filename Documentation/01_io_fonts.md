# Fonts

* [**Fonts**](./../src/io/output/fonts)

This module manages the main font usage for the kernel output.

## Types of Fonts

### Bitmap

* [**Bitmap**](./../src/io/output/fonts/bitmap.rs)

Bitmap fonts (or raster fonts) are typefaces made up of a grid of pixels. Unlike vector fonts (such as TrueType or OpenType), which use mathematical formulas to scale letters infinitely without losing quality, bitmap fonts are designed for a specific pixel size.

#### Key Characteristics
* **Pixel Structure**: Each letter is drawn pixel by pixel on a fixed matrix, similar to pixel art.
* **High Performance**: They require minimal computational power to render, which is why they were historically used in older computers, terminal displays, and low-resolution digital screens.
* **Lack of Scalability**: Resizing a bitmap font causes pixelation, leading to blurred or jagged edges.

---

### Vector (Not Implemented Yet)
* Placeholder for future vector font support.

---

## Why We Use It
* **Dynamic Font Selection**: Allows dynamic font selection within the kernel, tailored to the target architecture.
* **Simplified Implementation**: Keeps early boot and kernel-level text rendering simple and dependency-free.

---

## Where It Is Used
The font structures are utilized by core output mechanisms, including the kernel's `println!` macro and basic terminal text rendering.
