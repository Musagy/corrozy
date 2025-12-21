# CORROZY - Feature Status

## ✅ COMPLETE (Parsing + Codegen + Tests)
- [x] Variable declarations (`let x = 5;`)
  - [Example](./php-example/src/variable/var.crz)
  - [Example (Output)](./php-example/dist/variable/var.php)
  - [Unit tests:](./tests/unit/variable.rs#L5)
  
    <details>  

    - **[test_variable_declaration_with_type](./tests/unit/variable.rs#L10)**
    - **[test_variable_declaration_double_quotes](./tests/unit/variable.rs#L25)**
    - **[test_variable_declaration_single_quotes](./tests/unit/variable.rs#L40)**
    - **[test_variable_declaration_special_names](./tests/unit/variable.rs#L55)**
    - **[test_variable_declaration_empty_string](./tests/unit/variable.rs#L70)**
    
    </details>


- [x] Constant declarations (`const PI = 3.14;`)
  - [Example](./php-example/src/variable/var.crz)
  - [Example (Output)](./php-example/dist/variable/var.php)
  - [Unit tests:](./tests/unit/variable.rs#L85)

    <details>  

    - **[test_constant_declaration_with_type](./tests/unit/variable.rs#L90)**
    - **[test_constant_declaration_without_type](./tests/unit/variable.rs#L105)**
    - **[test_constant_declaration_large_numbers](./tests/unit/variable.rs#L120)**
    
    </details>

- [x] Prints (`println("hello")`)
  - [Example](./php-example/src/variable/var.crz#L16)
  - [Example (Output)](./php-example/dist/variable/var.php#L18)
  - [Unit tests:](./tests/unit/prints.rs)

    <details>  

    - **[test_println](./tests/unit/prints.rs#L10)**
    - **[test_print](./tests/unit/prints.rs#22)**
    
    </details>

- [x] namespace automatic (`namespace MyApp/Utils M`)
  - [Unit Tests:](./corrozy-core/src/transpiler.rs#L166)

    <details>  

    - **[test_auto_namespace_root_file](./corrozy-core/src/transpiler.rs#L172)**
    - **[test_auto_namespace_nested_dirs](./corrozy-core/src/transpiler.rs#L183)**
    
    </details>

- [x] String literals (raw + interpolated)
  - [Unit Tests:](./corrozy-core/src/language/features/expression/generator.rs#L113)

    <details>  

    - **[test_raw_string_literal](./corrozy-core/src/language/features/expression/generator.rs#L119)**
    - **[test_interpolated_string_literal](./corrozy-core/src/language/features/expression/generator.rs#L130)**
    
    </details>

## 🎲 YOLO MODE (Parsing y codegen funcionales PERO dudoso, sin tests completos/0 tests)
- [x] Basic expressions (binary operations)
- [x] Postfix expressions (`users[1].name`)
- [x] If/else statements
- [x] Function call (`add(1, 2)`)
- [x] Return statements

## ⚠️ WIP (Parsing + Codegen en trabajo, sin tests completos/0 tests)
- [ ] Function declarations
- [ ] Closures/lambdas

## 🚧 TODO (No implementado)
- [ ] While loops
- [ ] For loops
- [ ] Array literals
- [ ] Semantic validation
- [ ] Advanced string interpolation
- [ ] Optional chaining (`?.`)
- [ ] Null coalescing (`??`)
- [ ] Pattern matching
- [ ] Classes/Objects
- [ ] Imports/modules
- [ ] Error messages con line/column
- [ ] Source maps