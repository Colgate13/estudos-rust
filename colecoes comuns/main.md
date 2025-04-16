# Vetores

O primeiro tipo que iremos ver é Vec<T>, também conhecido como vetor. Vetores permitem guardar mais de um valor na mesma estrutura de dados que coloca todos os valores um ao lado do outro na memória. Vetores só podem guardar valores do mesmo tipo. Eles são úteis em situações onde há uma lista de itens, como as linha de texto em um arquivo ou preços de itens em um carrinho de compras.

```rs
let v: Vec<i32> = Vec::new();

// ou

let v = vec![1, 2, 3];
```

### Modificando um Vetor

Para criar um vetor e adicionar elementos a ele, podemos usar o método push:

```rs
let mut v = Vec::new();

v.push(5);
v.push(6);
v.push(7);
v.push(8);
```

### Descartar um Vetor Descarta seus Elementos

Como qualquer outro struct, um vetor será liberado quando ele sair do escopo:

```rs
{
    let v = vec![1, 2, 3, 4];

    // use as informações em v

} // <- v sai do escopo e é liberado aqui
``` 


### Usando um Enum para Armazenar Vários Tipos em um Vetor

```rs
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
];
```

### Referências Inválidas

Uma vez que o programa tenha uma referência válida, o verificador de empréstimo (borrow checker) faz valer as regras de propriedade e empréstimo abrangidas no Capítulo 4 para garantir que essa referência e quaisquer outras referências aos conteúdos do vetor permaneçam válidas. Lembre-se da regra que diz que não podemos ter referências mutáveis e imutáveis no mesmo escopo. Essa regra se aplica neste exemplo, onde mantemos uma referência imutável ao primeiro elemento em um vetor e tentamos adicionar um elemento ao final:

```
let mut v = vec![1, 2, 3, 4, 5];

let first = &v[0];

v.push(6);

println!("O primeiro elemento é: {}", first);
```
Compilar isso nos dará esse erro:
```

error[E0502]: cannot borrow `v` as mutable because it is also borrowed as
immutable
  |
4 | let first = &v[0];
  |              - immutable borrow occurs here
5 |
6 | v.push(6);
  | ^ mutable borrow occurs here
7 | }
  | - immutable borrow ends here
```

Isso nao daria erro: porque o rust percebeu que a variavel first não é mais usada depois de v.push(6), então o rust libera a referência imutável e permite que a referência mutável seja criada. O código abaixo compila sem erro:

```rs
let mut v = vec![1, 2, 3, 4, 5];

let first = &v[0];

v.push(6);
```

# Strings

Nós já conversamos sobre as strings no capítulo 4, mas vamos dar uma olhada mais em profundidade agora. As strings são uma área que os novos Rustáceos geralmente tem maior dificuldade. Isto é devido a uma combinação de três coisas: a propensão de Rust de certificar-se de expor possíveis erros, as strings são estruturas de dados mais complicadas que muitos programadores lhes dão crédito, e UTF-8. Essas coisas combina de tal forma que parecem difícil quando se vem de outras linguagens.

### Criando uma Nova String

Muitas das mesmas operações disponíveis com Vec também estão disponíveis em String, começando com a função new para criar uma string, assim:

```rs
let mut s = String::new();
```

Isso cria uma nova string vazia chamada s na qual podemos carregar dados.

Muitas vezes, teremos alguns dados iniciais que gostaríamos de já colocar na string. Para isso, usamos o método to_string, que está disponível em qualquer tipo que implementa a trait Display, como as strings literais:

```rs
let data = "initial contents";

let s = data.to_string();

// o método também funciona em literais diretamente
let s = "initial contents".to_string();
```

### Anexando a uma String com Push
```rs
let mut s = String::from("foo");
s.push_str("bar");

// ------------------------------------

let mut s1 = String::from("foo");
let s2 = String::from("bar");
s1.push_str(&s2);
```

### Concatenação com o Operador + ou a macro format!

Muitas vezes, queremos combinar duas strings existentes. Uma maneira é usar o operador + dessa forma:

```rs
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2; // Note que s1 foi movido aqui e não pode ser mais usado
```

Indexação em Strings

Em muitas outras linguagens, acessar caracteres individuais em uma string por referenciando por índice é uma operação válida e comum. Em Rust, no entanto, se nós tentamos acessar partes de uma String usando sintaxe de indexação, vamos ter um erro. Ou seja, este código:

```rs
let s1 = String::from("hello");
let h = s1[0];
```

resultará neste erro:

error: the trait bound `std::string::String: std::ops::Index<_>` is not
satisfied [--explain E0277]
  |>
  |>     let h = s1[0];
  |>             ^^^^^
note: the type `std::string::String` cannot be indexed by `_`

O erro e a nota contam a história: as strings em Rust não suportam a indexação. Assim a próxima pergunta é, por que não? Para responder a isso, temos que conversar um pouco sobre como o Rust armazena strings na memória.


# Hash Maps

A última das nossas coleções comuns é o hash map. O tipo HashMap <K, V> armazena um mapeamento de chaves do tipo K para valores do tipo V. Ele faz isso através de um hashing function, que determina como ele coloca essas chaves e valores em memória. Muitas linguagens de programação diferentes suportam este tipo de estrutura de dados, mas muitas vezes com um nome diferente: hash, map, object, hash table ou associative array, apenas para citar alguns.

Os Hash maps são úteis para quando você deseja poder procurar dados sem uso de índice, como você pode com vetores, mas usando uma chave que pode ser de qualquer tipo. Por exemplo, em um jogo, você poderia acompanhar a pontuação de cada equipe em um hash map onde cada chave é o nome de uma equipe e os valores são cada pontuação da equipe. Dado um nome da equipe, você pode recuperar sua pontuação.

Examinaremos a API básica dos hash map neste capítulo, mas há muitos mais coisas escondidas nas funções definidas no HashMap pela biblioteca padrão. Como sempre, verifique a documentação da biblioteca padrão para mais informação. 

### Criando um novo Hash Map

Podemos criar um HashMap vazio com new, e adicionar elementos com insert. Aqui, estamos acompanhando as pontuações de duas equipes cujos nomes são Blue e Yellow. A equipe blue começará com 10 pontos e a equipe yellow começa com 50:

```rs
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);
``` 


### Hash Maps e Ownership

Para os tipos que implementam a Copy trait, como i32, os valores são copiados no hash map. Para valores owned como String, os valores serão movidos e o hash map será o owner desses valores:

```rs
use std::collections::HashMap;

let field_name = String::from("Favorite color");
let field_value = String::from("Blue");

let mut map = HashMap::new();
map.insert(field_name, field_value);
// field_name e field_value são inválidos neste ponto
```

### Acessando Valores em um Hash Map

Podemos obter um valor do hash map fornecendo a chave para o método get:

```rs
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

let team_name = String::from("Blue");
let score = scores.get(&team_name);
```

Podemos iterar sobre cada par chave/valor em um hash map de uma maneira similar à que fazemos com vetores, usando um loop for:

```rs
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

for (key, value) in &scores {
    println!("{}: {}", key, value);
}
```

Insira Apenas se a Chave Não Possui Valor

É comum querer verificar se uma determinada chave tem um valor e, se não tiver, inserir um valor para ela. Os Hash maps possuem uma API especial para isso, chamada entry, que leva a chave que queremos verificar como um argumento. O valor de retorno da função entry é um enum, Entry, que representa um valor que pode ou não existir. Digamos que queremos verificar se a chave para o time Yellow tem um valor associado a ela. Se não tiver, queremos inserir o valor 50, e o mesmo para a equipe Blue. Com a API de entrada, o código irá parecer com:

```rs
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);

scores.entry(String::from("Yellow")).or_insert(50);
scores.entry(String::from("Blue")).or_insert(50);

println!("{:?}", scores);
``` 

O método or_insert em Entry retorna o valor para o Entry correspondente se a chave existir, e se não, insere seu argumento como o novo valor para esta chave e retorna a Entry modificada. Isso é muito mais limpo do que escrever a lógica por nós mesmos e, além disso, trabalha-se de forma mais limpa com o borrow checker.

Este código imprimirá {"Yellow": 50, "Blue": 10}. A primeira chamada para entry irá inserir a chave para a equipe Yellow com o valor 50, uma vez que o time Yellow já não possua um valor. A segunda chamada para entry não vai mudar o hash map pois o time Blue já possui o valor 10.
Atualize um Valor com Base no Valor Antigo

Outro caso de uso comum para hash maps é procurar o valor de uma chave e, em seguida, atualiza-la , com base no valor antigo. Por exemplo, se quisermos contar quantas vezes cada palavra apareceu em algum texto, podemos usar um hash map com as palavras como chaves e incrementar o valor para acompanhar quantas vezes vimos essa palavra. Se esta é a primeira vez que vimos uma palavra, primeiro inseriremos o valor 0.

```rs
use std::collections::HashMap;

let text = "hello world wonderful world";

let mut map = HashMap::new();

for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
}

println!("{:?}", map);
```
Isso imprimirá {"world": 2, "hello": 1, "wonderful": 1}. O método or_insert na verdade retorna uma referência mutável (& mutV) para o valor desta chave. Aqui nós armazenamos essa referência mutável na variável count, então, para poder atribuir esse valor, devemos primeiro desreferenciar count usando o asterisco (*). A referência mutável fica fora do escopo no final do loop for, então todas essas mudanças são seguras e permitidas pelas regras de borrow.