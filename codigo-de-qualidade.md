# Como Rust força o dev a escrever código de qualidade

Em Rust não é possível encontrarmos comportamento desconhecido, o Compiler sabe tudo que pode acontecer!

Mas e se encontrarmos um Null?

```rust
let x = Null; // Não encontrado
let y = null; // Não encontrado
let z = None; // Some<>
```

Se não existe Null, como gerenciamos erros?

```rust
let mut x: Option<String> = None;

if 1 == 0 {
    x = Some(String::from("teste"));
}

match x {
    Some(valor) => println!("{}", valor),
    None => panic!("x é None")
    }
    
if let Some(valor) = x {
    println!("{}", valor);
}
```

```rust
fn operacao() -> Result<i32, String> {
    // operação feita
    
    //let resultado: &str = "resultado esperado";
     let resultado: &str = "resultado não esperado";
    
    match resultado {
        "resultado esperado" => Ok(200),
        _ => Err(String::from(resultado))
    }
}

println!("{:?}", operacao());

//println!("{}", operacao().unwrap());

match operacao() {
    Ok(x) => println!("{}", x),
    Err(x) => println!("{}", x)
}
```

Comportamento indefinido?

```rust
let a = "string";
match a {
    "string" => todo!()
    // Garanta que todos os casos possíveis sejam considerados
}
```

