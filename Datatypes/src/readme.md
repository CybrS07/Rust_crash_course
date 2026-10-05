# Data Types

Rust has the following primitive (scalar) data types:

1. Integer (`int`)
2. Floating-point (`float`)
3. Boolean (`bool`)
4. Character (`char`)

## 1. Integer

Integers come in two flavors: signed and unsigned.

### 1.1 Signed Integers

Signed integers can hold both negative and positive values.

| Type   | Size     |
|--------|----------|
| `i8`   | 8 bits   |
| `i16`  | 16 bits  |
| `i32`  | 32 bits  |
| `i64`  | 64 bits  |
| `i128` | 128 bits |
| `isize`| pointer-sized (32 or 64 bits depending on the architecture) |

### 1.2 Unsigned Integers

Unsigned integers can hold only zero and positive values.

| Type   | Size     |
|--------|----------|
| `u8`   | 8 bits   |
| `u16`  | 16 bits  |
| `u32`  | 32 bits  |
| `u64`  | 64 bits  |
| `u128` | 128 bits |
| `usize`| pointer-sized (32 or 64 bits depending on the architecture) |

### 1.3 Explanation

The number in the type name tells the compiler how many **bits** the variable uses. This determines the range of values it can hold.

- A signed type with `n` bits ranges from `-2^(n-1)` to `2^(n-1) - 1`.
- An unsigned type with `n` bits ranges from `0` to `2^n - 1`.

For example:

| Type  | Range                                                  |
|-------|--------------------------------------------------------|
| `i8`  | -128 to 127                                            |
| `i32` | -2,147,483,648 to 2,147,483,647 (-2³¹ to 2³¹ - 1)      |
| `i64` | -2⁶³ to 2⁶³ - 1                                        |
| `u8`  | 0 to 255                                               |
| `u32` | 0 to 4,294,967,295 (0 to 2³² - 1)                      |

If a value is outside the range of its type, the compiler gives an error:

```rust
let x: i32 = 2_147_483_648; // error: literal out of range for `i32`
```

> **Note:** Rust does not allow commas in numeric literals. Use underscores (`_`) as visual separators, e.g. `2_147_483_647`.

## 2. Floating-Point

| Type  | Size    |
|-------|---------|
| `f32` | 32 bits |
| `f64` | 64 bits (default) |

```rust
let pi: f64 = 3.14159;
```

## 3. Boolean

The `bool` type has two possible values: `true` and `false`. It takes 1 byte.

```rust
let is_active: bool = true;
```

## 4. Character

The `char` type represents a single Unicode scalar value and is 4 bytes in size. Character literals use single quotes.

```rust
let letter: char = 'A';
let emoji: char = '😀';
```