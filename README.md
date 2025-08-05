# CORROZY ROADMAP

## ✅ IMPLEMENTED

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

### 📄 Output Statements
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

### 🛠️ Functions & Function Calls
**Corrozy:**
```rust
fn greeter(name: string): void {
    println("Hello {}", name);
}

fn add(x: int, y: int): int {
    return x + y;
}

greeter("Diego");
let result = add(5, 3);
```
**PHP output:**
```php
function greeter(string $name): void {
    echo "Hello " . $name . "\n";
}

function add(int $x, int $y): int {
    return $x + $y;
}

greeter("Diego");
$result = add(5, 3);
```

---

## 🚧 IN PROGRESS

### 🔀 Control Flow (Next)
**Corrozy:**
```rust
if (user.isActive) {
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

## 📋 PLANNED

### 🔄 Loops
**Corrozy:**
```rust
while (count < 10) {
    count = count + 1;
}

for (let i = 0; i < 10; i++) {
    println("Number: {}", i);
}
```

### 📚 Arrays
**Corrozy:**
```rust
let numbers: Array<int> = [1, 2, 3, 4];
let names: Array<string> = ["John", "Jane"];
```
**PHP output:**
```php
/** @var int[] */
$numbers = [1, 2, 3, 4];

/** @var string[] */
$names = ["John", "Jane"];
```

### 📞 Closures
**Corrozy:**
```rust
let add = (x: int, y: int): int => {
    return x + y;
};
```
**PHP output:**
```php
$add = function(int $x, int $y): int {
    return $x + $y;
};
```

### 🏗️ Classes
**Corrozy:**
```rust
class User extends Model impl[HasFactory, SoftDeletes] {
    let name: string;
    let email: string;
    
    fn getName(): string {
        return this.name;
    }
}
```

**PHP output:**
```php
class User extends Model {
    use HasFactory;
    use SoftDeletes;

    $name = string;
    $email =  string;
    

    function getName() {
        return this->name;
    }
}
```

### 📦 Records (Key-Value Objects)

**Corrozy:**
```rust
let config: Record<string, string> = {
    host: "localhost",
    port: "3306"
};
```

**PHP output:**

```php
/** @var array<string, string> */
$config = [
    'host' => 'localhost',
    'port' => '3306'
];
```

### 🎯 Interfaces
**Corrozy:**
```rust
interface User: Model(), HasFactory {
    name: string;
    email: string;
}
```

### 🎁 Imports / Namespaces
**Corrozy:**
```rust
// File: App/src/router/user.crz
import http.Request;
```
**PHP output:**
```php
<?php
namespace App\Router;

use App\Http\Request;
```

### 🔧 Custom Types
**Corrozy:**
```rust
type Number = int | float;
type UserStatus = "active" | "inactive" | "pending";
```

---

## 🔮 FUTURE FEATURES

### 🧬 Advanced Types
- Generic types: `Array<T>`
- Nullable types: `address: string?`
- default value: `name: string = "Osito Perú"`

### 🎭 Traits & Advanced OOP
- Trait implementations
- Abstract classes
- Method overriding

### 🚀 Framework Integration
- Laravel template: `corrozy init laravel`
- WordPress plugin template: `corrozy init wordpress-plugin`
- API template: `corrozy init api`