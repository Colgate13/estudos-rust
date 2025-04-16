📚 Resumo - Ownership em Rust

Ownership (posse) é o conceito central do Rust e define como a linguagem gerencia a memória de forma segura e eficiente sem usar garbage collector. O sistema é baseado em regras simples verificadas em tempo de compilação, garantindo segurança sem custo em tempo de execução.
🧠 Regras de Ownership

    Cada valor tem um único owner (dono).

    Apenas um owner por vez.

    Quando o owner sai de escopo, o valor é destruído automaticamente (via drop()).

📦 Pilha vs Heap

    Pilha (stack): rápida, dados de tamanho fixo e conhecido.

    Heap: usada para dados dinâmicos (ex: String), exige alocação/desalocação manual.

    Rust lida com heap automaticamente via ownership.

🆚 Tipos Simples vs Compostos

    Tipos simples (ex: i32, bool, char) implementam o trait Copy, permitindo cópias automáticas.

    Tipos como String não implementam Copy, e sim move: a variável original é invalidada ao ser atribuída a outra.

🔁 Clone x Move

    let s2 = s1; → s1 é movido, não pode mais ser usado.

    let s2 = s1.clone(); → faz cópia profunda, s1 continua válido.

📞 Ownership e Funções

    Passar valor para função move ou copia, dependendo do tipo.

    Para evitar perda de ownership, é possível retornar o valor ou usar referências (visto nos próximos capítulos).

🧪 Exemplo Prático com Função

```rs
fn calcula_tamanho(s: String) -> (String, usize) {
    let tamanho = s.len();
    (s, tamanho)
}
```

Aqui, s é movido para a função e depois retornado em uma tupla para ser reutilizado.


```rs
fn main() {
    let s = String::from("texto");  // s entra em escopo.

    toma_posse(s);                  // move o valor de s para dentro da função...
                                    // ... e ele não é mais válido aqui.

    let x = 5;                      // x entra em escopo.

    faz_uma_copia(x);               // x seria movido para dentro da função,
                                    // mas i32 é Copy, então está tudo bem em
                                    // usar x daqui para a frente.

} // Aqui, x sai de escopo, e depois s. Mas como o valor de s foi movido, nada
  // de especial acontece.

fn toma_posse(uma_string: String) { // uma_string entra em escopo.
    println!("{}", uma_string);
} // Aqui, uma_string sai de escopo, e o método `drop` é chamado. A memória que
  // guarda seus dados é liberada.

fn faz_uma_copia(um_inteiro: i32) { // um_inteiro entra em escopo.
    println!("{}", um_inteiro);
} // Aqui, um_inteiro sai de escopo. Nada de especial acontece.
```


Referências e Borrowing

Aqui está uma forma de como você poderia definir e usar uma função calcula_tamanho que recebe uma referência para um objeto como parâmetro, em vez de pegar este valor para si:

(Referencia é basicamente o ponteiro, mas com segurança de memória)
Passar o valor via referencia deixa a o valor do endereco de memoria onde o ponteiro aponta imutável, ou seja, não pode ser alterado.

```rs
fn main() {
    let s1 = String::from("texto");

    let tamanho = calcula_tamanho(&s1);

    println!("O tamanho de '{}' é {}.", s1, tamanho);
}

fn calcula_tamanho(s: &String) -> usize {
    s.len()
}
```

Referencia com mutabilidade:

```rs
fn main() {
    let mut s = String::from("texto");

    modifica(&mut s); // passa a referencia mutável de s para a função
}

fn modifica(uma_string: &mut String) { // uma_string entra em escopo.
    uma_string.push_str(" longo");
}
```

Nao podemos ter dois ponteiros/variaveis apontando para mesma referencia mutavel:

CODIGO ERRADO:

```rs
let mut s = String::from("texto");

let r1 = &mut s;
let r2 = &mut s;


error[E0499]: cannot borrow `s` as mutable more than once at a time
 --> main.rs:5:19
  |
4 |     let r1 = &mut s;
  |                   - first mutable borrow occurs here
5 |     let r2 = &mut s;
  |                   ^ second mutable borrow occurs here
6 | }
  | - first borrow ends here
```

Esta restrição permite a mutação, mas de uma forma bem controlada. Isto é algo com que novos Rustáceos passam trabalho, porque a maioria das linguagens de programação permitem modificar um valor quando você quiser. O benefício de ter esta restrição é que o Rust previne data races em tempo de compilação.

Um data race é parecido com uma condição de corrida, e acontece quando esses três fatores ocorrem:

  1  Dois ou mais ponteiros acessam o mesmo dado ao mesmo tempo.
  2  Ao menos um dos ponteiros é usado para escrever sobre o dado.
  3  Não há nenhum mecanismo sendo usado para sincronizar o acesso ao dado.

Data races causam comportamento indefinido e pode ser difíceis de diagnosticar e corrigir quando você está tentando rastreá-los em tempo de execução. Rust previne este problema de acontecer porque não vai nem deixar compilar um código com data races!

Como sempre, podemos usar chaves ({}) para criar um novo escopo, permitindo múltiplas referências mutáveis, mas não simultâneas:

```rs
let mut s = String::from("texto");

{
    let r1 = &mut s;

} // aqui r1 sai de escopo, então já podemos criar uma nova referência sem
  // problema nenhum.

let r2 = &mut s;
```

Existe uma regra parecida para combinar referências mutáveis e imutáveis. Este código resulta em erro:

```rs
let mut s = String::from("texto");

let r1 = &s; // sem problema
let r2 = &s; // sem problema
let r3 = &mut s; // PROBLEMA GRANDE
```
Não podemos ter uma referência mutável enquanto temos uma imutável. Usuários de uma referência imutável não esperam que os valores mudem de repente! Porém, múltiplas referências imutáveis são permitidas, pois ninguém que esteja apenas lendo os dados será capaz de afetar a leitura que está sendo feita em outra parte do código.

Referencias soltas (ponteiros com a referencia para um valor que ja foi deallocado)

```rs
fn main() {
    let referencia_para_o_nada = soltar();
}

fn soltar() -> &String { // soltar retorna uma referência a uma String

    let s = String::from("texto"); // s é uma nova String

    &s // retornamos uma referência a uma String, s
} // Aqui, s sai de escopo e é destruída. Sua memória é devolvida.
  // Perigo!

// Aqui está o erro do compilar

error[E0106]: missing lifetime specifier
 --> main.rs:5:16
  |
5 | fn soltar() -> &String {
  |                ^ expected lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but there is
  no value for it to be borrowed from
  = help: consider giving it a 'static lifetime
```

A Solucao é uma funcao nao retornar a referencia mas sim o valor 
```rs
fn nao_soltar() -> String {
    let s = String::from("texto");

    s
}
```

As Regras de Referências

Vamos recapitular o que discutimos sobre referências:

    Em um dado momento, você pode ter um ou outro, mas não os dois:

    Uma referência mutável.
    Qualquer número de referências imutáveis.

    Referências devem ser válidas sempre.



Slices
Outro tipo de dados em que não há ownership é a slice (do inglês, fatia). Slices lhe permitem referenciar uma sequência contígua de elementos em uma coleção em vez de referenciar a coleção inteira.

Esta função, primeira_palavra, tem uma &String como parâmetro. Nós não queremos tomar posse dela, então tudo bem. Mas o que nós deveríamos retornar? Não temos uma forma de falar sobre parte de uma string. No entanto, poderíamos retornar o índice do final de uma palavra. Vamos tentar fazer isso, conforme mostrado na Listagem 4-5:

Arquivo: src/main.rs

```rs
fn primeira_palavra(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
```

Agora temos uma forma de descobrir o índice do fim da primeira palavra na string, mas tem um problema. Estamos retornando um usize por si só, mas ele só possui um significado no contexto da &String. Em outras palavras, como é um valor separado da String, não há garantia de que ele ainda será válido no futuro. Considere o programa na Listagem 4-6, que usa a função da Listagem 4-5:

Arquivo: src/main.rs

```rs
fn main() {
    let mut s = String::from("texto longo");

    let palavra = primeira_palavra(&s); // palavra vai ter o valor 5.

    s.clear(); // Isso esvazia a String, deixando ela igual a "".

    // palavra ainda tem o valor 5 aqui, mas já não há mais uma string para a
    // qual o valor 5 faça algum sentido. palavra agora é totalmente inválida!
}
```

Slices de String

Uma slice de string é uma referência para uma parte de uma String, e tem a seguinte forma:

```rs
let s = String::from("texto longo");

let texto = &s[0..5];
let longo = &s[6..11];
```

Isto é similar a pegar uma referência à String inteira, mas com um [0..5] a mais. Em vez de uma referência à String inteira, trata-se de uma referência a uma porção da String. A sintaxe início..fim representa um range (uma faixa) que começa em início e continua até, mas não incluindo, fim.

Podemos criar slices usando um range entre colchetes especificando [índice_inicial..índice_final], em que índice_inicial é a primeira posição inclusa na slice, e índice_final é um a mais que a última posição inclusa na slice. Internamente, a estrutura de dados de uma slice armazena a posição inicial e o tamanho da slice, que corresponde a índice_final menos índice_inicial. Então, no caso do let longo = &s[6..11];, longo seria uma slice que contém um ponteiro para o sétimo byte de s (índice 6) e um tamanho igual a 5.


```rs
fn primeira_palavra(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}


fn main() {
    let mut s = String::from("texto longo");

    let palavra = first_word(&s);

    s.clear(); // Erro!
}

Aqui está o erro:

error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
 --> src/main.rs:6:5
  |
4 |     let word = primeira_palavra(&s);
  |                                  - immutable borrow occurs here
5 |
6 |     s.clear(); // Erro!
  |     ^ mutable borrow occurs here
7 | }
  | - immutable borrow ends here
```

Voltando às regras de borrowing, lembre-se que, se temos uma referência imutável para algum valor, não podemos também obter uma referência mutável do mesmo. Como clear precisa truncar a String, esse método tenta obter uma referência mutável, e acaba falhando. O Rust não só tornou nossa API mais fácil de usar, como também eliminou uma classe inteira de erros em tempo de compilação!

Outras Slices

Slices de string, como você pode imaginar, são específicas de strings. Mas há também um tipo de slice mais genérico. Considere esta array:


let a = [1, 2, 3, 4, 5];

Assim como às vezes queremos nos referir a uma parte de uma string, podemos também querer nos referir a uma parte de uma array, e faríamos isso da seguinte forma:


let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];

Essa slice tem o tipo &[i32]. Ela funciona da mesma forma que as slices de string, armazenando uma referência para o primeiro elemento e um tamanho. Você vai usar esse tipo de slice para todos os tipos de coleções. Vamos discutir essas coleções em mais detalhe quando falarmos sobre vetores no Capítulo 8.


RESUMO: 

Resumo

Os conceitos de ownership, borrowing, e slices são o que garante a segurança de memória dos programas em Rust em tempo de compilação. A linguagem Rust lhe dá controle sobre o uso da memória, assim como outras linguagens de programação de sistemas, mas como o dono dos dados limpa automaticamente a memória quando ele sai de escopo, você não tem que escrever e debugar código extra para ter esse controle.

O ownership afeta o funcionamento de várias outras partes do Rust, por isso vamos falar um pouco mais sobre esses conceitos neste livro daqui para a frente. Vamos seguir para o próximo capítulo e ver como agrupar dados em uma struct.