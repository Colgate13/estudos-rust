Usando Módulos para Reutilizar e Organizar Código


    A palavra-chave mod declara um novo módulo. O código dentro do módulo aparece    imediatamente após esta declaração dentro de chaves ou em    outro arquivo.
    Por padrão, as funções, tipos, constantes e módulos são privados. A palavra-chave pub    torna um item público e, portanto, visível fora do seu namespace.
    A palavra-chave use traz módulos, ou as definições dentro dos módulos, ao    escopo, assim é mais fácil se referir a eles.

mod e o Sistema de Arquivos

Vamos iniciar o nosso exemplo de módulo fazendo um novo projeto com o Cargo, mas em vez de criar um crate binário, faremos um crate de biblioteca: um projeto que as outras pessoas podem puxar para os seus projetos como uma dependência. Por exemplo, o crate rand discutido no Capítulo 2, é um crate de biblioteca que usamos como uma dependência no projeto do jogo de adivinhação.

Criaremos um esqueleto de uma biblioteca que fornece algumas funcionalidades gerais de rede; nos concentraremos na organização dos módulos e funções, mas não nos preocuparemos com o código que está dentro das funções. Chamaremos nossa biblioteca de communicator. Por padrão, o Cargo criará uma biblioteca, a menos que outro tipo de projeto seja especificado: se omitimos a opção --bin, que temos usado em todos os capítulos anteriores a este, nosso projeto será um biblioteca:

$ cargo new communicator
$ cd communicator

## Definindo um modulo

Regras dos Módulos e Seus Arquivos

Vamos resumir as regras dos módulos em relação aos arquivos:

    Se um módulo chamado foo não possui submódulos, você deve colocar as declarações    para foo em um arquivo chamado foo.rs.
    Se um módulo chamado foo possui submódulos, você deve colocar as declarações    para foo em um arquivo chamado foo/mod.rs.


```rs
mod network {
    fn connect() {
    }
}
```

```rs
communicator
 ├── network
 └── client

mod network {
    fn connect() {
    }
}

mod client {
    fn connect() {
    }
}

Ou

communicator
 └── network
     └── client

mod network {
    fn connect() {
    }

    mod client {
        fn connect() {
        }
    }
}
```

Separando por arquivos:

```rs
Arquivo: src/lib.rs

mod client; // Rust vai buscar por client.rs OU client/mod.rs

mod network {
    fn connect() {
    }

    mod server {
        fn connect() {
        }
    }
}

Arquivo: src/client.rs
mod client {
    // conteúdo de client.rs
}
```

Poderiamos fazer assim, caso o client tenha submodulos:

```rs
Arquivo: src/lib.rs
mod client; // Rust vai buscar por client.rs OU client/mod.rs
mod network {
    fn connect() {
    }

    mod server {
        fn connect() {
        }
    }
}

Arquivo: src/client/mod.rs
mod client {
    // conteúdo de client.rs
}
mod subclient;

Arquivo: src/client/subclient.rs
mod subclient {
    // conteúdo de subclient.rs
}
```
Essas regras se aplicam de forma recursiva, então, se um módulo chamado foo tiver um submódulo chamado bar e bar não possui submódulos, você deve ter os seguintes arquivos no seu diretório src:

├── foo
│   ├── bar.rs (contém as declarações em `foo::bar`)
│   └── mod.rs (contém as declarações em `foo`, incluindo `mod bar`)

Os módulos devem ser declarados no arquivo do módulo pai usando a palavra-chave mod.

Em seguida, vamos falar sobre a palavra-chave pub e nos livrar dessas warnings!

## Controlando a Visibilidade com pub

Controlando a Visibilidade com pub

Resolvemos as mensagens de erro mostradas na Listagem 7-5 movendo o código de network e network::server para os arquivos src/network/mod.rs e src/network/server.rs, respectivamente. Nesse ponto, cargo build era capaz de construir nosso projeto, mas ainda recebemos mensagens de warning sobre as funções client::connect, network::connect, e network::server::connect não estarem em uso:

extern crate communicator;

fn main() {
    communicator::client::connect();
}

Usamos o comando extern crate para trazer o crate de biblioteca communicator para o escopo. Nosso pacote agora contém duas crates. Cargo trata src/main.rs como um arquivo raiz de um crate binário, que é separado do crate de biblioteca existente cujo arquivo raiz é src/lib.rs. Esse padrão é bastante comum para projetos executáveis: a maioria das funcionalidades está em um crate de biblioteca e o crate binário usa esse crate de biblioteca. Como resultado, outros programas também podem usar o crate de biblioteca, e é uma boa separação de responsabilidades.

Fazendo uma Função Pública

Para dizer ao Rust que torne pública uma função, adicionamos a palavra-chave pub ao início da declaração. Nos focaremos em corrigir o warning que indica client::connect não foi utilizado por enquanto, assim como o erro module `client` is private (módulo `client` é privado) do nosso crate binário. Modifique src/lib.rs para tornar o módulo client público, assim:


```rs
Arquivo: src/lib.rs

pub mod client;

mod network;


mod outermost {
    pub fn middle_function() {}

    fn middle_secret_function() {}

    mod inside {
        pub fn inner_function() {}

        fn secret_function() {}
    }
}

fn try_me() {
    outermost::middle_function(); // ok
    outermost::middle_secret_function(); // erro: função privada
    outermost::inside::inner_function(); // ok
    outermost::inside::secret_function(); // erro: função privada
}

```

# Referindo-se a Nomes em Módulos Diferentes

```rs
Vimos como chamar funções definidas dentro de um módulo usando o nome do módulo como parte da chamada, como na chamada para a função nested_modules mostrada aqui na Listagem 7-7:

Arquivo: src/main.rs

pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}

fn main() {
    a::series::of::nested_modules();
}
```

## Trazendo Nomes no Escopo com a Palavra-Chave use

```rs
pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}

use a::series::of;

fn main() {
    of::nested_modules();
}

// Podemos fazer assim tambem:

pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}

use a::series::of::nested_modules;

fn main() {
    nested_modules();
}
```

### Podemos listar quais itens queremos trazer para o escopo:

```rs
pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
            pub fn another_function() {}
        }
    }
}

use a::series::of::{nested_modules, another_function};

fn main() {
    nested_modules();
    another_function();
}

// Ou com enum
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

use TrafficLight::{Red, Yellow};

fn main() {
    let red = Red;
    let yellow = Yellow;
    let green = TrafficLight::Green;
}
```


## Operador Glob. Trazendo Todos os Itens de um Módulo

```rs
pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
            pub fn another_function() {}
        }
    }
}

use a::series::of::*;

fn main() {
    nested_modules();
    another_function();
}
```


## SUPER

```rs

pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
            pub fn another_function() {}
        }
    }
}
use a::series::of;

fn main() {
    a::series::of::nested_modules();
    a::series::of::another_function();
}

// Com super

pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
            pub fn another_function() {}
        }
    }
}

// Usar super ou :: é a mesma coisa
fn main() {
    ::series::of::nested_modules(); // a::series::of::nested_modules()
    super::series::of::another_function(); // a::series::of::another_function()
}
```

## Usando use com super 
```rs 
mod tests {
    use super::client; // importa o módulo client do escopo pai

    #[test]
    fn it_works() {
        client::connect();
    }
}
```

Resumo

Agora você conhece algumas técnicas novas para organizar o seu código! Use estas técnicas para agrupar as funcionalidades relacionadas, evitar que os arquivos tornem-se muito longos, e apresentar uma API pública arrumada para os usuários da sua biblioteca.

Em seguida, analisaremos algumas estruturas de dados de coleções na biblioteca padrão que você pode usar em seu código limpo e elegante!