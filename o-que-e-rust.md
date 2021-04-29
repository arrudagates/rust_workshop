
# O que é Rust?

* Open-source
* Linguagem de sistemas (?)
* Performance
* Strongly Typed
```rust
let variavel: i32 = 2
```

* Type Inference
```rust
let variavel = 2 // integer
```

* Segurança de memória / Ownership
```rust
let variavel = String::from("teste");
let variavel_b = variavel;
println!("{}", variavel); // borrow of moved value: `variavel`
```

* Macros
```rust
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}
```


# Histórico

* 2006 - Funcionário da Mozilla Graydon Hoare começa um projeto pessoal
* 2009 - Mozilla começa a investir no projeto
* 2010 - Mozilla anuncia o investimento no projeto
* 2015 - Rust 1.0 é publicado em 15 de Maio, sendo oficialmente chamada de Rust 2015
* 2018 - Em 6 de Dezembro a versão 1.31.0 é liberada, sendo oficialmente chamada de Rust 2018


# Pra que é usada?
* Game engines
* Sistemas operacionais
* File systems
* Componentes de Browser (Firefox Quantum)
* Simulação
* Servers
* Ferramentas em geral antes escritas em C/CPP
* E mais recentemente, Web (Backend e Frontend!)
