
Usando Structs para Estruturar Dados Relacionados

Uma struct, ou estrutura, é um tipo de dados personalizado que nos permite nomear e criar um conjunto de vários valores relacionados que compõem um grupo de dados. Se você estiver familiarizado com uma linguagem orientada a objeto, um struct é como os atributos de dados de um objeto. Neste capítulo, vamos comparar e diferenciar tuplas com structs, demonstrar como usar structs e discutir como definir os métodos e funções associadas às structs para especificar o comportamento associado com os dados de uma struct. Os conceitos de struct e enum (que será discutido no Capítulo 6) são os blocos necessários para a criação de novos tipos para o seu programa, para tirar o máximo proveito da verificação de tipo no tempo de compilação do Rust.


```rs
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

let mut user1 = User {
    email: String::from("alguem@exemplo.com"),
    username: String::from("algumnome123"),
    active: true,
    sign_in_count: 1,
};

user1.email = String::from("outroemail@exemplo.com");
```

(Struct Update Syntax)

```rs

let user2 = User {
    email: String::from("outro@exemplo.com"),
    username: String::from("outronome567"),
    active: user1.active,
    sign_in_count: user1.sign_in_count,
};

let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    ..user1
};
```

Structs-Tuplas sem Campos Nomeados para Criar Tipos Diferentes

```rs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);
``` 

Reprogramação com Structs: Adicionando Mais Significado

A macro 'println!' pode fazer muitos tipos de formatação, e por padrão, {} diz a println!, para utilizar a formatação conhecida como Display: saída destinada para consumo do utilizador final. Os tipos primitivos que vimos até agora implementam Display por padrão, porque só há uma maneira que você deseja mostrar um 1 ou qualquer outro tipo primitivo para um usuário. Mas com Structs, a forma como println! deve formatar a saída é menos clara, pois existem mais possibilidades de exibição: você quer vírgulas ou não? Deseja imprimir as chavetas {}? Todos os campos devem ser mostrados? Devido a esta ambiguidade, Rust não tenta adivinhar o que queremos e as structs não têm uma implementação de Display.



```rs
#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

fn main() {
    let rect1 = Rectangle { length: 50, width: 30 };

    println!("rect1 is {:?}", rect1); // rect1 is Rectangle { length: 50, width: 30 }
    println!("rect1 is {:#?", rect1); 
    // rect1 is Rectangle {
    //   length: 50,
    //   width: 30
    // }
}
```

# Sintaxe do Método -> Definindo Métodos

```rs

#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn square(size: u32) -> Rectangle { // Funcao associada. Parece metodo estatico
        Rectangle { length: size, width: size }
    }

    fn area(&self) -> u32 {
        self.length * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}

fn main() {
    let rect1 = Rectangle { length: 50, width: 30 };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    let rect2 = Rectangle::square(20);
    println!("rect2 is {:?}", rect2);
    println!("rect1 can hold rect2: {}", rect1.can_hold(&rect2));
    // println!("rect1 is {:?}", rect1); // rect1 is Rectangle { length: 50, width: 30 }
}
```

Onde está o Operador ->?


Em linguagens como C++, dois operadores diferentes são usados para chamar métodos: você usa . se você está chamando um método do objeto diretamente e -> se você está chamando o método em um apontador para o objeto e necessita de desreferenciar o apontadr primeiro. Em outras palavras, se objeto é um apontador, objeto->algo() é semelhante a (*objeto).algo().

Rust não tem um equivalente para o operador ->; em vez disso, Rust tem um recurso chamado referenciamento e desreferenciamento automático. Chamada de métodos é um dos poucos lugares em Rust que têm este comportamento.

Eis como funciona: quando você chamar um método com objecto.algo(), Rust adiciona automaticamente &, &mut ou * para que objecto corresponda à assinatura do método. Em outras palavras, as seguintes são as mesmas:


p1.distance(&p2);
(&p1).distance(&p2);

Sumário

As Structs permitem-nos criar tipos personalizados que são significativos para o nosso domínio. Usando structs, podemos manter pedaços de dados associados ligados uns aos outros e nomear cada pedaço para fazer nosso código claro. Métodos ajudam-nos a especificar o comportamento que as instâncias das nossas structs têm, funções associadas dão-nos a funcionalidade de namespace que é particular à nossa struct sem ter uma instância disponível.

Mas structs não são a única maneira que nós podemos criar tipos personalizados: vamos ao recurso do Rust, enum, para adicionar uma outra ferramenta à nossa caixa de ferramentas.