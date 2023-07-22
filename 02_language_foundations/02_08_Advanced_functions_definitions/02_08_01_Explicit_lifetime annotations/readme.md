Rust verifica o lifetime. De forma com que todas a tentativas de acessar dados sao validos. 
Todos o valores associdados a um lifetime, devem viver ate o ultimo acesso a qualquer valor associado aquele lifetime.

Geralmente todos os lifetimes sao identificados sem ajuda. O compilador consegue inferir. 

```rust
fn add_with_lifetimes<'a, 'b>(i: &'a i32, j: &'b i32 ) -> i32{
    *i + *j

}

```
