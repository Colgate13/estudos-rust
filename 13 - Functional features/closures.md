### Closures anonymous functions

Os closures do Rust são funções anônimas que você pode salvar em uma variável ou passar como argumentos para outras funções. Você pode criar o closure em um local e, em seguida, chamá-lo em outro local para avaliá-lo em um contexto diferente. Ao contrário das funções, os closures podem capturar valores do escopo em que são definidos. Demonstraremos como esses recursos de closure permitem a reutilização de código e a personalização de comportamento.

Varias formas de criar uma funcao em rust

```rs
fn  add_one_v1   (x: u32) -> u32 { x + 1 } // Função normal
let add_one_v2 = |x: u32| -> u32 { x + 1 }; // Closure com tipo explícito
let add_one_v3 = |x|             { x + 1 }; // Closure com tipo implícito
let add_one_v4 = |x|               x + 1  ; // Closure de uma linha
```

Closures podem capturar o embiente em que sao criadas, permitindo que acessem variáveis fora de seu escopo local. Isso é útil para criar funções que mantêm estado ou para criar funções de callback.

Exemplo de closure que captura o ambiente:

```rs
#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
}
```


### Capturando referências ou movendo propriedade

A Closures podem capturar valores de seu ambiente de três maneiras, que se correlacionam diretamente com as três maneiras pelas quais uma função pode receber um parâmetro: tomando emprestado imutavelmente, tomando emprestado mutávelmente e assumindo a propriedade. O fechamento decidirá qual delas usar com base no que o corpo da função faz com os valores capturados.

definimos um fechamento que captura uma referência imutável ao vetor nomeado listporque ele só precisa de uma referência imutável para imprimir o valor:

```rs
fn main() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let only_borrows = || println!("From closure: {list:?}");

    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");
}
```

Este exemplo também ilustra que uma variável pode ser vinculada a uma definição de fechamento, e posteriormente podemos chamar o fechamento usando o nome da variável e parênteses como se o nome da variável fosse um nome de função.


alteramos o corpo do fechamento para que ele adicione um elemento ao listvetor. O fechamento agora captura uma referência mutável:

```rs
fn main() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let mut borrows_mutably = || list.push(7);

    borrows_mutably();
    println!("After calling closure: {list:?}");
}
```

ele captura uma referência mutável para list. Não usamos o fechamento novamente após sua chamada, então o empréstimo mutável termina.

Essa técnica é mais útil ao passar um closure para uma nova thread para mover os dados para que sejam propriedade da nova thread. Discutiremos threads e por que você deve usá-las em detalhes no Capítulo 16, quando falarmos sobre simultaneidade, mas, por enquanto, vamos explorar brevemente a geração de uma nova thread usando um closure que precisa da movepalavra-chave. A Listagem 13-6 mostra a Listagem 13-4 modificada para imprimir o vetor em uma nova thread em vez de na thread principal:

Nome do arquivo: src/main.rs

```rs
use std::thread;

fn main() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();
}
```

### A maneira como um fechamento captura e manipula valores do ambiente afeta quais características o fechamento implementa, e características são como funções e estruturas podem especificar quais tipos de fechamentos podem usar. Os fechamentos implementarão automaticamente uma, duas ou todas as três Fncaracterísticas, de forma aditiva, dependendo de como o corpo do fechamento manipula os valores:

  **FnOnce** aplica-se a fechamentos que podem ser chamados uma vez. Todos os fechamentos implementam pelo menos esta característica, pois todos os fechamentos podem ser chamados. Um fechamento que move valores capturados para fora de seu corpo implementará apenas FnOncee nenhuma das outras Fncaracterísticas, pois só pode ser chamado uma vez.
  **FnMut** aplica-se a fechamentos que não movem valores capturados para fora do seu corpo, mas que podem causar mutação nos valores capturados. Esses fechamentos podem ser chamados mais de uma vez.
  **Fn** aplica-se a fechamentos que não movem valores capturados para fora de seu corpo e que não os alteram, bem como a fechamentos que não capturam nada de seu ambiente. Esses fechamentos podem ser chamados mais de uma vez sem alterar seu ambiente, o que é importante em casos como chamar um fechamento várias vezes simultaneamente.

Vejamos a definição do unwrap_or_elsemétodo Option<T>que usamos

```rs
impl<T> Option<T> {
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T
    {
        match self {
            Some(x) => x,
            None => f(),
        }
    }
}
```
### Exemplo de fnOnce, FnMut e Fn

1. FnOnce

Aplica-se a fechamentos que podem ser chamados uma vez e que movem valores capturados para fora de seu corpo. Após a chamada, o fechamento não pode ser usado novamente.
rust

```rs
fn main() {
    let nome = String::from("Alice");
    let closure = || {
        println!("Nome: {}", nome);
        // O valor `nome` é movido para o fechamento e destruído aqui
    };
    
    closure(); // Pode ser chamado uma vez
    // closure(); // Erro! O fechamento já consumiu `nome` e não pode ser chamado novamente
}
```

Explicação: O fechamento captura nome por movimento (move), consumindo o valor. Como nome é destruído após a chamada, o fechamento só pode ser usado uma vez, implementando apenas FnOnce.
2. FnMut

Aplica-se a fechamentos que não movem valores capturados, mas podem modificar os valores capturados. Esses fechamentos podem ser chamados várias vezes.
rust

```rs
fn main() {
    let mut contador = 0;
    let mut closure = || {
        contador += 1; // Modifica o valor capturado
        println!("Contador: {}", contador);
    };
    
    closure(); // Contador: 1
    closure(); // Contador: 2
    closure(); // Contador: 3
}
```

Explicação: O fechamento captura contador por referência mutável (&mut), permitindo modificá-lo a cada chamada. Como não consome os valores capturados, pode ser chamado várias vezes, implementando FnMut.
3. Fn

Aplica-se a fechamentos que não movem nem modificam valores capturados, ou que não capturam nada do ambiente. Podem ser chamados várias vezes, inclusive simultaneamente (seguro para concorrência).
rust

```rs
fn main() {
    let x = 42;
    let closure = || {
        println!("Valor de x: {}", x); // Apenas lê o valor, sem modificá-lo
    };
    
    closure(); // Valor de x: 42
    closure(); // Valor de x: 42
    // Pode ser chamado várias vezes sem problemas
}
```

Explicação: O fechamento captura x por referência imutável (&), apenas lendo seu valor sem modificá-lo. Como não altera o ambiente, implementa Fn e pode ser chamado várias vezes, até em contextos concorrentes.
Resumo dos Exemplos

    FnOnce: Consome valores capturados (ex.: move de String), só pode ser chamado uma vez.
    FnMut: Modifica valores capturados (ex.: incrementa contador), pode ser chamado várias vezes.
    Fn: Apenas lê valores capturados (ou não captura nada), pode ser chamado várias vezes e é seguro para concorrência.

Se precisar de mais exemplos ou explicações, é só pedir!
975ms
O unico motivo para o exemplo que usa o fnonce acima fazer dar erro é porque é uma string e por ser string a referencia dela vai para dentro da function? se fosse um tipo primitivo iria ser um fnmut?

Boa pergunta! Vamos esclarecer por que o exemplo de FnOnce dá erro e se o comportamento seria diferente com um tipo primitivo.
Por que o exemplo FnOnce dá erro?

No exemplo fornecido:

```rs
fn main() {
    let nome = String::from("Alice");
    let closure = || {
        println!("Nome: {}", nome);
        // O valor `nome` é movido para o fechamento
    };
    
    closure(); // Pode ser chamado uma vez
    // closure(); // Erro! O fechamento consumiu `nome`
}
```
O erro ocorre porque o fechamento move o valor nome (do tipo String) para dentro de si. Em Rust, quando um fechamento captura uma variável por movimento (move), ele assume a posse (ownership) dessa variável. Após a primeira chamada, nome é consumido e destruído, então o fechamento não pode ser chamado novamente, pois a variável capturada não existe mais. Isso faz com que o fechamento implemente apenas o traço FnOnce.

O fato de ser uma String é relevante porque String é um tipo que não implementa o traço Copy, ou seja, ele é movido por padrão em vez de ser copiado. Tipos que não implementam Copy (como String, Vec, ou outros tipos que gerenciam memória no heap) são consumidos quando movidos, o que força o fechamento a ser FnOnce nesse caso.
E se fosse um tipo primitivo?

Se usarmos um tipo primitivo, como um i32, que implementa o traço Copy, o comportamento muda. Tipos Copy (como i32, f64, bool, etc.) não são movidos; em vez disso, eles são copiados implicitamente quando usados em um fechamento. Isso permite que o fechamento seja chamado várias vezes, implementando Fn (ou potencialmente FnMut, se houver mutação).

Vamos ver um exemplo com um tipo primitivo:

```rs
fn main() {
    let x = 42; // `i32` implementa `Copy`
    let closure = || {
        println!("Valor: {}", x); // `x` é copiado, não movido
    };
    
    closure(); // Valor: 42
    closure(); // Valor: 42
    // Funciona várias vezes, pois `x` não é consumido
}
```

Por quê? Como i32 implementa Copy, o fechamento não toma posse de x; ele apenas copia seu valor. Assim, x permanece válido no escopo original, e o fechamento pode ser chamado várias vezes, implementando o traço Fn (não FnOnce).
E se o tipo primitivo for mutável?

Se o fechamento modificar um tipo primitivo capturado, ele será classificado como FnMut em vez de Fn, porque está alterando o estado do ambiente capturado. Veja o exemplo:

```rs
fn main() {
    let mut x = 42; // `i32`, mas mutável
    let mut closure = || {
        x += 1; // Modifica `x`
        println!("Valor: {}", x);
    };
    
    closure(); // Valor: 43
    closure(); // Valor: 44
    // Funciona várias vezes, pois `x` não é consumido
}
```

Aqui, o fechamento captura x por referência mutável (&mut), permitindo modificá-lo. Como x não é movido (apenas modificado), o fechamento implementa FnMut, e pode ser chamado várias vezes.
Resposta direta à sua pergunta

    O erro no exemplo FnOnce ocorre porque String não é Copy: O fechamento move a String (toma posse), e após a primeira chamada, ela é destruída. Isso faz o fechamento ser FnOnce.
    Se fosse um tipo primitivo (como i32): O fechamento não moveria o valor, mas sim o copiaria (devido ao traço Copy). Nesse caso:
        Se o fechamento apenas lê o valor (sem modificá-lo), ele implementa Fn.
        Se o fechamento modifica o valor, ele implementa FnMut.
        Ele não seria FnOnce, porque tipos Copy não são consumidos.

Portanto, o comportamento do fechamento depende de:

    Se o tipo é Copy ou não: Tipos não-Copy (como String) são movidos, resultando em FnOnce. Tipos Copy (como i32) são copiados, permitindo Fn ou FnMut.
    Se há mutação: Mutação implica FnMut (se não mover) ou FnOnce (se mover). Sem mutação, pode ser Fn (se não mover).