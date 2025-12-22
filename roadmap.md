# Corrozy Roadmap

This roadmap reflects the **current state**, **short-term goals**, and **long-term vision** of Corrozy.

> Status legend:
>
> * ✅ Implemented (with parsing + codegen + tests)
> * 🎲 Implemented but unstable / missing tests
> * ⚠️ Work in progress
> * 🚧 Not implemented

---

## ✅ Implemented

### Variables & Constants

* `let` declarations
* `const` declarations
* Optional type annotations
* PHPDoc generation

### Output Statements

* `print()`
* `println()`

### String Literals

* Raw strings (`'text'`)
* Interpolated strings (`"Hello {name}"`)

### Expressions

* Binary expressions (`+ - * /`)
* Parenthesized expressions
* Variable references

### Postfix Expressions

* Array indexing (`users[0]`)
* Property access (`user.name`)
* Method calls (`user.getName()`)

### Control Flow

* `if / else`

### Namespaces

* Automatic namespace generation based on folder structure
* Manual namespace configuration

---

## 🎲 Implemented (Needs More Tests)

* Function calls
* Return statements
* Basic expression chaining

---

## ⚠️ Work in Progress

### Functions

* Function declarations
* Return types

### Closures / Lambdas

* Block closures
* Expression closures

---

## 🚧 Planned

### Loops

* `while`
* `for`

### Collections

* Array literals
* Records / key-value objects

### Language Features

* Optional chaining (`?.`)
* Null coalescing (`??`)
* Pattern matching

### OOP

* Classes
* Interfaces
* Traits
* Inheritance

### Modules

* Imports
* Explicit namespace declarations

---

## 🔮 Future / Experimental

### Types

* Union types (`int | float`)
* Nullable types (`string?`)
* Generics (`Array<T>`)

### Tooling

* Better error messages (line / column)
* Source maps
* Formatter
* LSP improvements

### Framework Templates

* Laravel
* WordPress
* API skeleton

---

## 🧭 Philosophy

Corrozy prioritizes:

* Readable generated PHP
* Explicit typing
* Predictable behavior
* Clean internal architecture

This roadmap will evolve as the language matures.
