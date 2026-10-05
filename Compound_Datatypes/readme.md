# 📦 Compound Data Types: Code Explained

A line-by-line explanation of `src/main.rs`.

---

## 1. Number array

```rust
let number: [i32; 2] = [0, 1];
```

- `[i32; 2]` means an array of **2** values of type `i32`. The format is `[type; length]`.
- The length is fixed. An array can't grow or shrink.
- Every element must be the same type.

## 2. String array

```rust
let strings: [&str; 3] = ["Muhammad", "Taha", "Rajput"];
```

- Same idea as above, but each element is a `&str` (a string literal).
- This is an array of 3 strings.

## 3. Tuple

```rust
let tuple: (&str, i32, bool) = ("Taha", 20, true);
```

- A tuple groups values of **different types** in a fixed order.
- The type lists each element's type: text, number, boolean.
- Access items with a dot and the index: `tuple.0` is `"Taha"`, `tuple.1` is `20`, `tuple.2` is `true`.

## 4. Mixed tuple

```rust
let mix = ("Taha", 20, true, [45, 22, 45, 11, 45]);
```

- A tuple can hold anything, including an array.
- There is no type written here because Rust works it out itself. The type is `(&str, i32, bool, [i32; 5])`.
- The array inside is reached with `mix.3`, and its first item with `mix.3[0]`.

## 5. Slice

```rust
let slice1: &[i32] = &[1, 5, 4, 7, 96, 1, 2, 3];
let slice2: &[i32] = &slice1[0..5];
```

- `&[i32]` is a slice: a **borrowed view** of a list of numbers. It doesn't own the data.
- The `&` means "borrow".
- `[0..5]` is a range. It starts at index `0` and stops **before** index `5`, so you get the first 5 items: `[1, 5, 4, 7, 96]`.

| Range | Result |
|---|---|
| `[0..5]` | `[1, 5, 4, 7, 96]` |
| `[..3]` | `[1, 5, 4]` |
| `[5..]` | `[1, 2, 3]` |
| `[0..=2]` | `[1, 5, 4]` (the `=` includes the end) |

## 6. String and `mut`

```rust
let mut mike: String = String::from("Hello");
mike.push_str("! Idoits");
```

- `String` is a text value you own, and it can grow.
- `String::from("Hello")` turns a string literal into a `String`.
- `push_str` adds text to the end, so `"Hello"` becomes `"Hello! Idoits"`.

**Why `mut`?** Variables in Rust can't be changed by default. Because `push_str` changes `mike`, it has to be declared with `mut`. Without it, the code won't compile.

```rust
let name = String::from("Hi");
name.push_str("!");        // ❌ error: name is not mutable

let mut name = String::from("Hi");
name.push_str("!");        // ✅ works
```

Only `mike` uses `mut` here because it's the only variable that changes.

## 7. String slice

```rust
let mike_slice: &str = &mike[0..6];
```

- Takes the first 6 bytes of `mike`, which is `"Hello!"`.
- Ranges on strings count **bytes**, not characters. That's fine for English letters, but cutting through a character like `é` or an emoji will crash the program.
- While `mike_slice` exists, `mike` can't be changed, because the slice is borrowing it.

---

## `{:?}`: how it works

```rust
println!("Array {:?}", number);
```

- `{}` and `{:?}` are **placeholders**. `println!` replaces each one with the next value you pass in.
- `{:?}` means **Debug format**. It prints a value in a developer-friendly way.

| Placeholder | Use |
|---|---|
| `{}` | Normal output. Works for plain numbers and strings. |
| `{:?}` | Debug output. Needed for arrays, tuples and slices. |
| `{:#?}` | Debug output, pretty-printed over multiple lines. |

Arrays, tuples and slices can't be printed with `{}` because Rust doesn't have a default way to display them:

```rust
println!("{}", number);    // ❌ error
println!("{:?}", number);  // ✅ [0, 1]
```

On strings, `{:?}` also adds quotes, which is why the output shows `"Hello"` and not `Hello`.

---

## Output

```text
Array [0, 1]
Strings ["Muhammad", "Taha", "Rajput"]
Tuple: ("Taha", 20, true)
MIX ("Taha", 20, true, [45, 22, 45, 11, 45])
Slice1: [1, 5, 4, 7, 96, 1, 2, 3]
Slice2 [1, 5, 4, 7, 96]
Mike: "Hello"
Mike Update: "Hello! Idoits"
Slice2 "Hello!"
```