# Corrozy (v0.0.1)

**Corrozy** is an **experimental programming language** that transpiles to **PHP**, written in **Rust**.

The project focuses on exploring **language design**, **compiler/transpiler architecture**, and generating **clean, typed, readable PHP code**.

> ⚠️ **Project status**: Early development / Experimental
>
> * APIs are unstable
> * Breaking changes are expected
> * Not production-ready

---

## ✨ What Corrozy Does Today

Corrozy already supports a small but functional subset of the language, including parsing, AST generation, PHP code generation, and unit tests.

### 📤 Variables & Constants

**Corrozy:**

```rust
const PI: float = 3.14;
let nombre: string = "Diego";
```

**PHP output:**

```php
/** @var float */
const PI = 3.14;

/** @var string $nombre */
$nombre = "Diego";
```

---

### 🖨️ Output Statements

**Corrozy:**

```rust
println("Hello world");
print("Hello");
```

**PHP output:**

```php
echo "Hello world" . "\n";
echo "Hello";
```

---

### 🧵 String Literals

Supports **raw** and **interpolated** strings.

**Corrozy:**

```rust
let a = 'raw string';
let b = "Hello {name}";
```

**PHP output:**

```php
$a = 'raw string';
$b = "Hello {$name}";
```

---

### 🔀 Control Flow (if / else)

**Corrozy:**

```rust
if user.isActive {
    println("User is active");
} else {
    println("User is inactive");
}
```

**PHP output:**

```php
if ($user->isActive) {
    echo "User is active" . "\n";
} else {
    echo "User is inactive" . "\n";
}
```

---

## 🧪 Testing

Corrozy includes **unit tests** for implemented features, focused on:

* Correct parsing
* AST correctness
* PHP output generation

Tests live in:

* `tests/` (workspace-level integration & unit tests)
* `corrozy-core/src/**` (feature-local tests)

---

## 🧠 Goals

Corrozy is built to:

* Explore language and syntax design
* Learn compiler and transpiler architecture in Rust
* Generate readable and typed PHP
* Experiment with "screaming architecture" and feature-based organization

---

## 📍 Versioning

Current version: **v0.0.1**

* No stability guarantees
* Frequent refactors
* Structure and APIs may change at any time

---

## 🗺️ Roadmap

See [roadmap.md](./roadmap.md) for planned and future features.

---

## 📄 License

MIT License
