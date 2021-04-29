# Alguns aspectos de sintaxe

* Tipos
```rust
let a: i32 = 42;
let b: f64 = 1.0;
println!("{}", a + b); // A e B não são do mesmo tipo
```

* Imutabilidade
```rust
let a = 42;
let mut b = 1;

a = 43; // A não é mutavel
b = 2;
```

* Ownership
```rust
let x = String::from("hello"); // X é dona do valor
let a = x; // Agora A é dona do valor que era de X
do_something(x); // X não possui valor mais, então não é válida
```

```rust
let x = String::from("hello");

do_something(&x);

let a = x.clone();
```
