`String` Suporta operacoes como concatenacao, incluir texto em um string existente, trim, etc

`str` é um tipo relativamente pobre e de alta performance. 
Uma vez criados nao podem ser expandidos ou aumentados.

`str` sao geralmente visto na forma de `&str` (string slice) que é um tipo pesqueno que tem uma referencia para um dado de str e tamanho.
Tentativa de atribuir uma variavel para um tipo str vai falhar.
Rust cria tamanho fixos para variaveis locais (i32, u32, etc)
Valores `str` podem ser de tamanho arbitrarios, sendo assim só é possivel utilizar em variavies locais como referencia

Usar `&str` no lugar de strings evita alocacao dinamica de memoria.

`String` é um tipo owned e `&str` é borrowed. 
