
# ROADMAP

### 📤 Variables

**Corrozy:**
```ts
const PI: float = 3.14

let nombre: string = "Diego"
```

**PHP output:**
```php
define("PI", 3.14);

/** @var string $nombre */
$nombre = "Juan";
```
---

### 📄 Print

**Corrozy:**
```go
println("hello");
print("hello");
```

**PHP output:**
```php
echo "Hello" . "\n";
echo "Hello";
```

---

### 🛠️ Functions

**Corrozy:**
```rust
fn greeter(name: string) {
  println("Hello {}", name);
}

fn filterUserById(users: User[]) -> int {
  // ...
}
```

**PHP output:**
```php
function greeter(string $name) {
    echo "Hello, " . $name . "\n";
}

/**
 * @param User[] $users
 */
function filterUserById(array $users): int {
  // ...
}
```

## Work In Progress

### 🎁 Imports / Namespaces (planned)

**Corrozy:**

```go
// File: App/src/router/user.crz

import http.Request;
```

**PHP output:**

```php
namespace App\Router\User // autocreate

use App\Http\Request;
```

### 📞 Callback

**Corrozy:**
```rust
let f = (x: int, y: int): int => {
  return x + y
}
```

**PHP output:**
```php
$f = function(int $x, int $y): int {
    return $x + $y;
};
```