## Arrays
É uma colecao leve que elementos sao do mesmo tipo.
Elementos podems ser trocados, mas o tamanho nao.
- A notacao pode ser confusa `[T;n]`
- `[u8;3]` por exemplo é diferente `[u8;4]`. O tamanho importa para o sistema

## Slices

Como arrays, mas dinamically sized. 
O que significa que o tamanho nao é conhecido em momento de compilacao, mas ainda assim como os arrays nao é  possivel expandir.
Essa diferenca(e semelhança) se da na assinatura `[T]`
Slices podem agir como views para arrays. (Forma rapida de utilizar como readonly).
Assim como os `str`,  como o rust precisa localmente de saber o tamanho do tipos, o slices sao vistos geramente como referencias como &[T].

## Vectors

`Vec<T>` é um lista crescrente de T. Existe uma pequena penalidade no runtime por usar-las.

