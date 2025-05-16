# 🧠 Learning GPU Programming in Rust

This repository documents my learnings of GPU programming using Rust, the CubeCL crate, and the book "Programming Massively Parallel Processors" by David B. Kirk and Wen-mei W. Hwu.

## ✅ Implemented Kernels

| Kernel     | Description                        |
| ---------- | ---------------------------------- |
| Flip Image | Horizontal and vertical flipping   |
| Blur Image | N×N box blur (averaging neighbors) |

## 🛠 How to Run

Each kernel can be run individually as a cargo example:

cargo run --example flip_image
cargo run --example blur_image

Data files live in examples/data/
