# Tipos Genéricos, Traits, e Tempos de vida (Lifetimes)

Cada linguagem de programação tem ferramentas para lidar de forma efetiva com a duplicação de conceitos; em Rust, uma dessas ferramentas são os tipos genéricos. Tipos genéricos são substitutos abstratos para tipos concretos ou para outras propriedades. Quando estamos escrevendo e compilando o código podemos expressar propriedades de tipos genéricos, como seu comportamento ou como eles se relacionam com outros tipos genéricos, sem precisar saber o que realmente estará no lugar deles.

Do mesmo modo que uma função aceita parâmetros cujos valores não sabemos para escrever código que será processado em múltiplos valores concretos, nós podemos escrever funções que recebem parâmetros de alguns tipos genéricos ao invés de tipos concretos como i32 ou String. Nós já usamos tipos genéricos no Capítulo 6 com Option<T>, no Capítulo 8 com Vec<T> e HashMap<K, V>, e no Capítulo 9 com Result<T, E>. Nesse capítulo, vamos explorar como definir nossos próprios tipos, funções e métodos usando tipos genéricos!

Primeiro, nós vamos revisar as mecânicas de extrair uma função que reduz duplicação de código. Então usaremos a mesma mecânica para fazer uma função genérica usando duas funções que só diferem uma da outra nos tipos dos seus parâmetros. Nós vamos usar tipos genéricos em definições de struct e enum também.

Depois disso, nós vamos discutir traits, que são um modo de definir comportamento de uma forma genérica. Traits podem ser combinados com tipos genéricos para restringir um tipo genérico aos tipos que tem um comportamento particular ao invés de qualquer tipo.

Finalmente, nós discutiremos tempos de vida, que são um tipo de generalização que nos permite dar ao compilador informações sobre como as referências são relacionadas umas com as outras. Tempos de vida são as características em Rust que nos permitem pegar valores emprestados em muitas situações e ainda ter a aprovação do compilador de que as referências serão válidas. 

## Removendo Duplicação por meio da Extração de uma Função

```rs
fn main() {
    let lista_numero = vec![34, 50, 25, 100, 65];

    let mut maior = lista_numero[0];

    for numero in lista_numero {
        if numero > maior {
            maior = numero;
        }
    }

    println!("O maior número é {}", maior);

    let lista_numero = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let mut maior = lista_numero[0];

    for numero in lista_numero {
        if numero > maior {
            maior = numero;
        }
    }

    println!("O maior número é {}", maior);
}
```

invez de usar :

```rs
fn maior_i32(lista: &[i32]) -> i32 {
    let mut maior = lista[0];

    for &item in lista.iter() {
        if item > maior {
            maior = item;
        }
    }

    maior
}

fn maior_char(lista: &[char]) -> char {
    let mut maior = lista[0];

    for &item in lista.iter() {
        if item > maior {
            maior = item;
        }
    }

    maior
}
```
Usar:

```rs
fn maior<T>(lista: &[T]) -> T {
    let mut maior = lista[0];

    for &item in lista.iter() {
        if item > maior {
            maior = item;
        }
    }

    maior
}
```

## Usando Tipos de Dados Genéros em Definições de Structs

Nós podemos definir structs para usar um parâmetro de tipo genérico em um ou mais campos de um struct com a sintaxe <> também. A listagem 10-6 mostra a definição e faz uso do struct Ponto que contém as coordenadas x e y com valores de qualquer tipo:

Nome do arquivo: src/main.rs

```rs
struct Ponto<T> {
    x: T,
    y: T,
}

fn main() {
    let inteiro = Ponto { x: 5, y: 10 };
    let float = Ponto { x: 1.0, y: 4.0 };
}
```

Listagem 10-6: Uma struct Ponto contém os valores x e y do tipo T

A sintaxe é similar a que se usa em definições de funções usando tipos genéricos. Primeiro, nós temos que declarar o nome do tipo de parâmetro dentro de colchetes angulares logo após o nome da struct. Então nós podemos usar tipos genéricos na definição da struct onde nós especificaríamos tipos concretos de dados.

Note que porque só usamos um tipo genérico na definição de Ponto, o que estamos dizendo é que o struct Ponto é genérico sobre algum tipo T, e os campos x e y são ambos do mesmo tipo, qualquer que seja. Se nós tentarmos criar uma instância de um Ponto que possui valores de tipos diferentes, como na Listagem 10-7, nosso código não compilará:

Nome do arquivo: src/main.rs

```rs
struct Ponto<T> {
    x: T,
    y: T,
}

fn main() {
    let nao_funciona = Ponto { x: 5, y: 4.0 };
}

Listagem 10-7: Os campos x e y precisam ser do mesmo tipo porque ambos tem o tipo genérico de dado T

Se nós tentarmos compilar isso, receberemos o seguinte erro:

error[E0308]: mismatched types
 -->
  |
7 |     let nao_funciona = Point { x: 5, y: 4.0 };
  |                                         ^^^ expected integral variable, found
  floating-point variable
  |
  = note: expected type `{integer}`
  = note:    found type `{float}`
```


## Usando Tipos de Dados Genéricos em Definições de Enum

Similar a structs, enums podem ser definidos para conter tipos genéricos de dados nas suas variantes. Nós usamos o enum Option<T> concedido pela biblioteca padrão no capítulo 6, e agora a definição deve fazer mais sentido. Vamos dar uma outra olhada:

```rs
enum Option<T> {
    Some(T),
    None,
}
```

Em outras palavras, Option<T> é um enum genérico do tipo T. Ele têm duas variantes: Some, que contém o valor do tipo T, e uma variante None que não contém nenhum valor. A biblioteca padrão tem que ter apenas essa deifinição para suportar a criação de valores desse enum que pode conter qualquer tipo concreto. A ideia de um "um valor opcional" é um conceito mais abstrato que o de um tipo específico, e Rust nos deixa expressar esse conceito abstrato sem muitas duplicações.

Enum podem usar tipos múltiplos genéricos também. A definição do enum Resultado que usamos no Capítulo 9 é um exemplo:

```rs
enum Resultado<T, E> {
    Ok(T),
    Err(E),
}
```

Usando Tipos Genéricos de Dados em Definições de Métodos

Como fizemos no Capítulo 5, nós podemos implementar métodos em estruturas e enums que têm tipos genéricos em suas definições. A Listagem 10-9 mostra o struct Ponto<T> que definimos na Listagem 10-6. Nós, então, definimos um método chamado x no Ponto<T> que retorna a referência para o dado no campo x:

Nome do arquivo: src/main.rs

```rs
struct Ponto<T> {
    x: T,
    y: T,
}

impl<T> Ponto<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p = Ponto { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}
```

Listagem 10-9: Implementando um método chamado x na struct Ponto<T> que retornará uma referência para o campo x, que é do tipo T.

Note que temos que declarar T logo após impl para usar T no tipo Ponto<T>. Declarar T como um tipo genérico depois e impl é como o Rust sabe se o tipo dentro das chaves angulares em Ponto é um tipo genérico ou um tipo concreto. Por exemplo, nós poderíamos escolher implementar métodos nas instâncias de Ponto<f32> ao invés nas de Ponto com qualquer tipo genérico. A listagem 10-10 mostra que não declaramos nada depois de impl nesse caso, já que estamos usanod um tipo concreto, f32:

```rs
impl Ponto<f32> {
    fn distancia_da_origem(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
```

Listagem 10-10: Construindo um bloco de impl que só se aplica a uma struct com o tipo específico usado pelo parâmetro de tipo genérico T

Esse código significa que o tipo Ponto<f32> terá um método chamado distancia_da_origem, e outras instâncias do Ponto<T> onde T não é do tipo f32 não terá esse método definido. Esse método quão longe nosso ponto está das coordenadas (0.0, 0.0) e usa operações matemáticas que só estão disponíveis para tipos de ponto-flutuantes.

Parâmetros de tipos genéricos em uma definição de struct não são sempre os parâmetros de tipos genéricos que você quer usar na assinatura de método daquela struct. A Listagem 10-11 define um método mistura na estrutura Ponto<T, U> da Listagem 10-8. O método recebe outro Ponto como parâmetro, que pode ter tipos diferentes de self Ponto dos quais usamos no mistura. O método cria uma nova instância de Ponto que possui o valor x de self Ponto (que é um tipo de T) e o valor de y passado de Ponto (que é do tipo W):

```rs
struct Ponto<T, U> {
    x: T,
    y: U,
}

impl<T, U> Ponto<T, U> {
    fn mistura<V, W>(self, other: Ponto<V, W>) -> Ponto<T, W> {
        Ponto {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Ponto { x: 5, y: 10.4 };
    let p2 = Ponto { x: "Ola", y: 'c'};

    let p3 = p1.mistura(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
```


# Traits: Definindo Comportamento Compartilhado

Traits nos permitem usar outro tipo de abstração: eles nos permitem abstrair sobre o comportamento que tipos têm em comum. Um trait diz ao compilador de Rust sobre uma funcionalidade que um tipo particular possui e pode compartilhar com outros tipos. Em situações onde nós usamos parâmetros de tipos genéricos, nós podemos usar limites de trait para especificar, em tempo de compilação, que o tipo genérico pode ser qualquer tipo que implementa um trait e por conseguinte tem o comportamento que queremos usar nessa situação.

    Nota: Traits são similares a um recurso frequentemente chamado de 'interface' em outras linguagens, com algumas diferenças.

## Definindo um Trait

O comportamento de um tipo consiste nos métodos que podemos chamar para aquele tipo. Tipos diferentes dividem o mesmo comportamento se podemos chamar os mesmos métodos em todos esses tipos. Definições de traits são um modo de agrupar métodos de assinaturas juntos a fim de definir um conjunto de comportamentos para atingir algum propósito.

Por exemplo, digamos que temos múltiplos structs que contém vários tipos e quantidades de texto: um struct ArtigoDeNoticiasque contém uma notícia preenchida em um lugar do mundo, e um Tweet que pode ter no máximo 140 caracteres em seu conteúdo além dos metadados como se ele foi um retweet ou uma resposta a outro tweet.

Nós queremos fazer uma biblioteca agregadora de mídia que pode mostrar resumos de dados que podem estar guardados em uma instância de ArtigoDeNoticia ou Tweet. O comportamento que precisamos cada struct possua é que seja capaz de ser resumido, e que nós possamos pedir pelo resumo chamando um método resumo em uma instância. A Listagem 10-12 mostra a definição de um trait Resumir que expressa esse conceito:

Nome do arquivo: lib.rs

```rs
pub trait Resumir {
    fn resumo(&self) -> String;
}
```

Implementando um Trait em um Tipo

Agora que deifnimos o trait Resumir, podemos implementa-lo nos tipos do nosso agregador de mídias que queremos que tenham esse comportamento. A Listagem 10-13 mostra uma implementação do trait Resumir no struct ArtigoNotícia que possui o título, o autor e a localização para criar e retornar o valor de resumo. Para o struct Tweet, nós escolhemos definir resumo como o nome de usuário seguido por todo o texto do tweet, assumindo que o conteúdo do tweet já está limitado a 140 caracteres.

Nome do arquivo: lib.rs

```rs
pub struct ArtigoDeNoticia {
    pub titulo: String,
    pub local: String,
    pub autor: String,
    pub conteudo: String,
}

impl Resumir for ArtigoDeNoticia {
    fn resumo(&self) -> String {
        format!("{}, by {} ({})", self.titulo, self.autor, self.local)
    }
}

pub struct Tweet {
    pub nomeusuario: String,
    pub conteudo: String,
    pub resposta: bool,
    pub retweet: bool,
}

impl Resumir for Tweet {
    fn resumo(&self) -> String {
        format!("{}: {}", self.nomeusuario, self.conteudo)
    }
}
```

Listagem 10-13: Implementando o trait Resumir nos tipos ArtigoDeNoticia e Tweet

Implementar um trait em um tipo é similar a implementar métodos que não estão relacionados com um trait. A diferença está depois de impl, nós colocamos o nome do trait que queremos implementar, então dizemos for e o nome do tipo que queremos implementar. Dentro do bloco impl, nós colocamos as assinaturas dos métodos que a definição do trait definiu, mas ao invés de colocar um ponto e vírgula depois de cada assinatura, nós colocamos chaves e preenchemos o corpo do método com o comportamento específico que queremos que os métodos dos traits tenham para um tipo particular.

Implementações Padrão

As vezes é útil ter um comportamento padrão pra alguns ou todos os métodos em um trait, ao invés de fazer toda implementação em todo tipo e definir um comportamento personalizado. Quando implementamos o trait em um tipo particular, nós podemos escolher manter ou sobrescrever o comportamento padrão de cada método.

A Listagem 10-15 mostra como poderíamos ter escolhido especificar uma string padrão para o método resumo do trait Resumir ao invés de escolher de apenas definir a assinatura do método como fizemos na Listagem 10-12:

Nome do arquivo: lib.rs

```rs
pub trait Resumir {
    fn resumo(&self) -> String {
        String::from("(Leia mais...)")
    }
}
```

Listagem 10-15: Definição de um trait Resumir com a implementação padrão do método resumo

Se nós quiséssemos usar a implementação padrão para resumir as instâncias de ArtigoDeNoticia ao invés de definir uma implementação personalizada como fizemos na Listagem 10-13, nós especificaríamos um bloco impl vazio:

```rs
impl Resumir for ArtigoDeNoticia {}
```

Mesmo que não estejamos mais escolhendo definir o método resumo diretamente em ArtigoDeNoticia, já que o método resumo tem uma implementação padrão e nós especificamos que ArtigoDeNoticia implementa o trait Resumir, nós ainda podemos chamar o método resumo em uma instância de ArtigoDeNoticia:

```rs
let artigo = ArtigoDeNoticia {
    titulo: String::from("Os Penguins ganham a copa do campeonato Stanley"),
    lugar: String::from("Pittsburgh, PA, USA"),
    autor: String::from("Iceburgh"),
    conteudo: String::from("Os Penguins de Pittsburgh são novamente o melhor
    time de hockey da NHL."),
};

println!("Novo artigo disponível! {}", artigo.summary());
```

Esse código imprime Novo artigo disponível! (Leia mais...)

Mudando o trait Resumir para ter uma implementação padrão para resumo não requer que nós mudemos nada na implementação de Resumir em Tweet na Listagem 10-13 ou em PrevisaoTempo na Listagem 10-14: a sintaxe para sobrepor uma implementação padrão é exatamente a mesma de uma sintaxe para implementar um método de trait que não tem uma implementação padrão.

Implementações padrões são autorizadas a chamar outros métodos no mesmo trait, mesmo se os outros métodos não tiverem uma implementação padrão. Desse modo, um trait pode prover muitas funcionalidades úteis e apenas requerir implementações para especificar uma pequena parte dele. Nós poderíamos escolher que o trait Resumir também tivesse o método resumo_autor qual a implementação é necessária, então um método resumo que tem a implementação padrão que chama pelo método resumo_autor:

```rs
pub trait Resumir {
    fn resumo_autor(&self) -> String;

    fn resumo(&self) -> String {
        format!("(Leia mais de {}...)", self.resumo_autor())
    }
}

// Para usar essa versão de Resumir, nós só precisamos definir resumo_autor quando nós implementamos o trait em um tipo:

impl Resumir for Tweet {
    fn autor_resumo(&self) -> String {
        format!("@{}", self.nomeusuario)
    }
}
```

Uma vez que definimos resumo_autor, nós podemos chamar resumo em instâncias do struct Tweet, e a implementação padrão de resumo chamará a definição de resumo_autor que fornecemos.

```rs
let tweet = Tweet {
    nomeusuario: String::from("horse_ebooks"),
    conteudo: String::from("claro, como vocês provavelmente já sabem, 
    pessoas"),
    resposta: false,
    retweet: false,
};

println!("1 novo tweet: {}", tweet.resumo());
```

Isso irá imprimir 1 novo tweet: (Leia mais de @horse_ebooks...).

Note que não é possível chamar a implementação padrão de uma implementação primordial.

# Limites de traits

Agora que definimos traits e os implementamos em tipos, podemos usar traits com parâmetros de tipos genéricos. Podemos restringir tipos genéricos para que ao invés de serem qualquer tipo, o compilador tenha certeza que o tipo estará limitado a aqueles tipos que implementam um trait em particular e por consequência tenham o comportamento que precisamos que os tipos tenham. Isso é chamado de especificar os limites dos traits em um tipo genérico.

```rs
pub fn notificar<T: Resumir>(item: T) {
    println!("Notícias de última hora! {}", item.resumo());
}
```
Limites de traits vão juntos com a declaração de um parâmetro de tipo genérico, depois de uma vírgula e entre colchetes angulares. Por causa do limite de trait em T, nós podemos chamar notificar e passar qualquer instância de ArtigoDeNoticia ou Tweet. O código externo da Listagem 10-14 que está usando nosso crate aggregator pode chamar nossa função notificar e passar uma instância de PrevisaoTempo, já que Resumir é implementado para PrevisaoTempo também. O código que chama notificar com qualquer outro tipo, como uma String ou um i32, não compilará, já que esses tipos não implementam Resumir.

Para funções que têm múltiplos parâmetros de tipos genéricos, cada tipo genérico tem seu próprio limite de trait. Especificar muitas informações de limites de trait dentro de chaves angulares entre o nome de uma função e sua lista de parâmetros pode tornar o código difícil de ler, então há uma sintaxe alternativa para especificar limites de traits que nos permite movê-los para uma cláusula depois da assinatura da função. Então ao invés de:

```rs
fn alguma_funcao<T: Mostrar + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {
```

Nós podemos escrever isso com uma cláusula de where:

```rs
fn alguma_funcao<T, U>(t: T, u: U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
```

Consertando a Função maior com Limites de Traits

Então qualquer hora que você queira usar um comportamento definido por um trait em um tipo genérico, você precisa especificar aquele trait nos limites dos parâmetros dos tipos genéricos. Agora podemos consertar a definição da função maior que usa um parâmetro de tipo genérico da Listagem 10-5! Quando deixamos esse código de lado, nós recebemos esse erro:

error[E0369]: binary operation `>` cannot be applied to type `T`
  |
5 |         if item > maior {
  |            ^^^^
  |
note: an implementation of `std::cmp::PartialOrd` might be missing for `T`

No corpo de maior nós queríamos ser capazes de comparar dois valores de tipo T usando o operador maior-que. Esse operador é definido com o método padrão na biblioteca padrão de trait std::cmp::PartialOrd. Então para que possamos usar o operador maior-que, precisamos especificar PartialOrd nos limites do trait para T para que a função maior funcione em partes de qualquer tipo que possa ser comparada. Não precisamos trazer PartialOrd para o escopo porque está no prelúdio.

```rs
fn maior<T: PartialOrd>(list: &[T]) -> T {
```

Se tentarmos compilar isso, receberemos diferentes erros:

error[E0508]: cannot move out of type `[T]`, a non-copy array
 --> src/main.rs:4:23
  |
4 |     let mut maior = list[0];
  |         -----------   ^^^^^^^ cannot move out of here
  |         |
  |         hint: to prevent move, use `ref maior` or `ref mut maior`

error[E0507]: cannot move out of borrowed content
 --> src/main.rs:6:9
  |
6 |     for &item in list.iter() {
  |         ^----
  |         ||
  |         |hint: to prevent move, use `ref item` or `ref mut item`
  |         cannot move out of borrowed content

A chave para esse erro é cannot move out of type [T], a non-copy array. Com nossas versões não genéricas da função maior, nós estávamos apenas tentando encontrar o maior i32 ou char. Como discutimos no Capítulo 4, tipos como o i32 e char que têm um tamanho conhecido podem ser armazenados na pilha, então eles implementam o trait Copia. Quando mudamos a função maior para ser genérica, agora é possível que o parâmetro list poderia ter tipos nele que não implementam o trait Copia, o que significa que não seríamos capazes de mover o valor para fora de list[0] para a variável maior.

Se quisermos ser capazes de chamar esse código com tipos que são Copia, nós podemos adicionar Copia para os limites de trait de T! A Listagem 10-16 mostra o código completo de uma função maior genérica que compilará desde que os tipos dos valores nessa parte que passamos para maior implementem ambos os traits PartialOrd e Copia, como i32 e char:

Nome do arquivo: src/main.rs

```rs
fn maior<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut maior = list[0];

    for &item in list.iter() {
        if item > maior {
            maior = item;
        }
    }

    maior
}

fn main() {
    let lista_numero = vec![34, 50, 25, 100, 65];

    let result = maior(&lista_numero);
    println!("O maior número é {}", result);

    let lista_char = vec!['y', 'm', 'a', 'q'];

    let result = maior(&lista_char);
    println!("O maior char é {}", result);
}
```

Listagem 10-16: Uma definição funcional da função maior que funciona em qualquer tipo genérico que implementa os traits PartialOrd e Copia

Se não quisermos restringir nossa função maior para apenas tipos que implementam o trait Copia, podemos especificar que T tem o limite de trait Clone ao invés de Copia e clonar cada valor na parte quando quisermos que a função maior tenha domínio. Usando a função clone significa que potencialmente estamos fazendo mais alocações no heap, porém, e alocações no heap podem ser vagarosas se estivermos trabalhando com grande quantidade de dados. Outro jeito que podemos implementar maior é para a função retornar uma referência ao valor de T em uma parte. Se retornarmos o tipo de retorno para ser &T ao invés de T e mudar o corpo da função para retornar uma referência, não precisaríamos usar os limites de traits Clone ou Copia e nós não estaríamos fazendo nenhuma alocação de heap. Tente implementar essas soluções alternativas você mesmo! 

## Usando Limites de Trait para Implementar Métodos Condicionalmente

Usando um limite de trait com um bloco impl que usa parâmetros de tipos genéricos podemos implementar métodos condicionalmente apenas para tipos que implementam os traits específicos. Por exemplo, o tipo Par<T> na listagem 10-17 sempre implementa o método novo, mas Par<T> implementa apenas o cmp_display se seu tipo interno T implementa o trait PartialOrd que permite a comparação e do trait Display que permite a impressão:

```rs
use std::fmt::Display;

struct Par<T> {
    x: T,
    y: T,
}

impl<T> Par<T> {
    fn novo(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
}

impl<T: Display + PartialOrd> Par<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("O maior membro é x = {}", self.x);
        } else {
            println!("O maior membro é y = {}", self.y);
        }
    }
}
```

Porque a biblioteca padrão tem essa implementação cobertor, podemos chamar o método to_string definido pelo tipo ToString em qualquer tipo que implemente o trait Display. Por exemplo, nós podemos transformar inteiros em seus correspondentes valores de String do seguinte modo, já que inteiros implementam Display:

```rs
let s = 3.to_string();
```

# Lifetimes Validating References with Lifetimes

Quandos falamos sobre referêcias no Capítulo 4, nós deixamos de fora um detalhe importante: toda referência em Rust tem um lifetime, que é o escopo no qual aquela referência é válida. A maior parte das vezes tempos de vida são implícitos e inferidos, assim como a maior parte do tempo tipos são inferidos. Similarmente quando temos que anotar tipos porque múltiplos tipos são possíveis, há casos em que os tempos de vida das referências poderiam estar relacionados de alguns modos diferentes, então Rust precisa que nós anotemos as relações usando parâmetros genéricos de tempo de vida para que ele tenha certeza que as referênciais reais usadas em tempo de execução serão definitivamente válidas.

Sim, é um pouco incomum, e será diferente de ferramentas que você usou em outras linguagens de programação. Tempos de vida são, de alguns jeitos, a característica mais distinta de Rust.

Tempos de vida são um tópico grande que não poderão ser cobertos inteiramente nesse capítulo, então nós vamos cobrir algumas formas comuns que você pode encontrar a sintaxe de tempo de vida nesse capítulo para que você se familiarize com os conceitos. O Capítulo 19 conterá informações mais avançadas sobre tudo que tempos de vida podem fazer.

```rs
{
    let r;

    {
        let x = 5;
        r = &x;
    }

    println!("r: {}", r);
}

Listagem 10-18: Uma tentativa de usar uma refência cujo valor saiu de escopo

    Variáveis Não Inicializadas Não Podem Ser Usadas

    Os próximos exemplos declaram vaŕiáveis sem darem a elas um valor inicial, então o nome da variável existe no escopo exterior. Isso pode parecer um conflito com Rust não ter null. No entanto, se tentarmos usar uma variável antes de atribuir um valor a ela, nós teremos um erro em tempo de compilação. Tente!

Quando compilarmos esse código, nós teremos um erro:

error: `x` does not live long enough
   |
6  |         r = &x;
   |              - borrow occurs here
7  |     }
   |     ^ `x` dropped here while still borrowed
...
10 | }
   | - borrowed value needs to live until here
```

## O Verificador de Empréstimos

A parte do compilador chamada de verificador de empréstimos compara escopos para determinar que todos os empréstimos são válidos. A Listagem 10-19 mostra o mesmo exemplo da Listagem 10-18 com anotações mostrando os tempos de vida das variáveis.

```rs
{
    let r;         // -------+-- 'a
                   //        |
    {              //        |
        let x = 5; // -+-----+-- 'b
        r = &x;    //  |     |
    }              // -+     |
                   //        |
    println!("r: {}", r); // |
                   //        |
                   // -------+
}
```

Tempos de Vida Génericos em Funções

Vamos escrever uma função que retornará a mais longa de dois cortes de string. Nós queremos ser capazes de chamar essa função passando para ela dois cortes de strings, e queremos que retorne uma string. O código na Listagem 10-21 deve imprimir A string mais longa é abcd uma vez que tivermos implementado a função maior:

Nome do Arquivo: src/main.rs

```rs
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let resultado = maior(string1.as_str(), string2);
    println!("A string mais longa é {}", resultado);
}
```


Note que queremos que a função pegue cortes de string (que são referências, como falamos no Capítulo 4) já que não queremos que a função maior tome posse de seus argumentos. Nós queremos que uma função seja capaz de aceitar cortes de uma String (que é o tipo de variável string1) assim como literais de string (que é o que a variável strin2 contém).

Recorra à seção do Capítulo 4 "Cortes de Strings como Parâmetros" para mais discussões sobre porque esses são os argumentos que queremos.

Se tentarmos implementar a função maior como mostrado na Listagem 10-22 ela não vai compilar:

Nome do arquivo: src/main.rs
```rs
fn maior(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

error[E0106]: missing lifetime specifier
   |
1  | fn maior(x: &str, y: &str) -> &str {
   |                                 ^ expected lifetime parameter
   |
   = help: this function's return type contains a borrowed value, but the
   signature does not say whether it is borrowed from `x` or `y`

```

O texto de ajuda está nos dizendo que o tipo de retorno precisa de um parâmetro de tempo de vida genérico nele porque o Rust não pode dizer se a referência que está sendo retornada se refere a x ou y. Atualmente, nós também não sabemos, já que o bloco if no corpo dessa função retorna uma referência para x e o bloco else retorna uma referência para y!

## Sintaxe de Anotação de Tempo de Vida

Anotações de tempo de vida não mudam quanto tempo qualquer uma das referências envolvidas viverão. Do mesmo modo que funções podem aceitar qualquer tipo de assinatura que especifica um parâmetro de tipo genérico, funções podem aceitar referências com qualquer tempo de vida quando a assinatura especificar um parâmetro genérico de tempo de vida. O que anotações de tempo de vida fazem é relacionar os tempos de vida de múltiplas referências uns com os outros.

```rs
&i32        // uma referência
&'a i32     // uma referência com um tempo de vida explícito
&'a mut i32 // uma referência mutável com um tempo de vida explícito
```

Uma anotação de tempo de vida por si só não tem muito significado: anotações de tempos de vida dizem ao Rust como os parâmetros genéricos de tempos de vida de múltiplas referências se relacionam uns com os outros. Se tivermos uma função com o parâmetro primeiro que é uma referência para um i32 que tem um tempo de vida de 'a, e a função tem outro parâmetro chamado segundo que é outra referência para um i32 que também possui um tempo de vida 'a, essas duas anotações de tempo de vida com o mesmo nome indicam que as referências primeiro e segundo precisam ambas viver tanto quanto o mesmo tempo de vida genérico.

Anotações de Tempo de Vida em Assinaturas de Funções

Vamos olhar para anotações de tempo de vida no contexto da função maior que estamos trabalhando. Assim como parâmetros de tipos genéricos, parâmetros de tempos de vida genéricos precisam ser declarados dentro de colchetes angulares entre o nome da função e a lista de parâmetros. A limitanção que queremos dar ao Rust é que para as referências nos parâmetros e o valor de retorno devem ter o mesmo tempo de vida, o qual nomearemos 'a e adicionaremos para cada uma das referências como mostrado na Listagem 10-23:

Nome do Arquivo: src/main.rs

```rs
fn maior<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("a string longa é longa");

    {
        let string2 = String::from("xyz");
        let resultado = maior(string1.as_str(), string2.as_str());
        println!("A string mais longa é {}", resultado);
    }
}

```


```rs
fn main() {
    let string1 = String::from("a string longa é longa");
    let resultado;
    {
        let string2 = String::from("xyz");
        resultado = longest(string1.as_str(), string2.as_str());  // Basicamente o bo aqui 'e que a string2 vai sair do escopo quando for usar a variavel resultado, mas como especificamos que o tempo de vida da string2 e o tempo de vida do resultado sao iguais, o compilador nao vai deixar compilar
    }
    println!("A string mais longa é {}", resultado);
}

Listagem 10-25: A tentativa de usar resultado depois que string2 saiu de escopo não compilará

Se tentarmos compilar isso, receberemos esse erro:

error: `string2` does not live long enough
   |
6  |         resultadod = longest(string1.as_str(), string2.as_str());
   |                                            ------- borrow occurs here
7  |     }
   |     ^ `string2` dropped here while still borrowed
8  |     println!("The longest string is {}", result);
9  | }
   | - borrowed value needs to live until here
```

Anotações de Tempo de Vida em Definições de Struct

Até agora, nós só definimos structs para conter tipos com posse. É possível para structs manter referências, mas precisamos adicionar anotações de tempo de vida em todas as referências na definição do struct. A Listagem 10-26 tem a struct chamada ExcertoImportante que contém um corte de string:

Nome do arquivo: src/main.rs

```rs
struct ExcertoImportante<'a> {
    parte: &'a str,
}

fn main() {
    let romance = String::from("Chame-me Ishmael. Há alguns anos...");
    let primeira_sentenca = romance.split('.')
        .next()
        .expect("Não pôde achar um '.'");
    let i = ExcertoImportante { parte: primeira_sentenca };
}
```

Parâmetros de Tipos Genéricos, Limites de Trais e Tempos de Vida Juntos

Vamos rapidamente olhar para a sintaxe de especificar parâmetros de tipos genéricos, limites de traits e tempos de vida todos em uma função!

```rs
use std::fmt::Display;

fn maior_com_um_anuncio<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display
{
    println!("Anúncio! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

Essa é a função maior da Listagem 10-23 que retorna a maior de dois cortes de string, mas com um argumento extra chamado ann. O tipo de ann é o tipo genérico T, que pode ser preenchido por qualquer tipo que implemente o trait Display como está especificado na cláusula where. Esse argumento extra será impresso antes da função comparar os comprimentos dos cortes de string, que é porque o trait de Display possui um limite. Porque tempos de vida são um tipo genérico, a declaração de ambos os parâmetros de tempo de vida 'a e o tipo genérico T vão na mesma lista com chaves angulares depois do nome da função.