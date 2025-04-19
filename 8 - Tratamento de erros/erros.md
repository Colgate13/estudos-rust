# Tratamento de Erros


Tratamento de Erros

O comprometimento de Rust à segurança se extende ao tratamento de erros. Erros são um fato da vida em software, portanto Rust possui um número de features para lidar com situações em que algo dá errado. Em vários casos, Rust requer que você reconheça a possibilidade de um erro acontecer e aja preventivamente antes que seu código compile. Esse requisito torna seu programa mais robusto ao assegurar que voce irá descobrir erros e lidar com eles apropriadamente antes de mandar seu código para produção!

Rust agrupa erros em duas categorias principais: recuperáveis e irrecuperáveis. Erros recuperáveis são situações em que é razoável reportar o problema ao usuário e tentar a operação novamente, como um erro de arquivo não encontrado. Erros irrecuperáveis são sempre sintomas de bugs, como tentar acessar uma localização além do fim de um array.

A maioria das linguagens não distingue esses dois tipos de erros e lida com ambos da mesma maneira usando mecanismos como exceções. Rust não tem exceções. Em vez disso, ele tem o valor Result<T, E> para erros recuperáveis e a macro panic! que para a execução ao encontrar um erro irrecuperável. Esse capítulo cobre primeiro como chamar panic! e depois fala sobre retornar valores Result<T, E>. Adicionalmente, vamos explorar o que se levar em consideração para decidir entre tentar se recuperar de um erro ou parar execução.

## Erros irrecuperáveis com panic!

```rs
fn main() {
    let v = vec![1, 2, 3];

    v[99];
}
```

$ cargo run
   Compiling panic v0.1.0 (file:///projects/panic)
    Finished dev [unoptimized + debuginfo] target(s) in 0.27 secs
     Running `target/debug/panic`
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is
100', /stable-dist-rustc/build/src/libcollections/vec.rs:1362
note: Run with `RUST_BACKTRACE=1` for a backtrace.
error: Process didn't exit successfully: `target/debug/panic` (exit code: 101)


## Erros recuperáveis com Result

A maior parte dos erros não são sérios o suficiente para precisar que o programa pare totalmente. Às vezes, quando uma função falha, é por uma razão que nós podemos facilmente interpretar e responder. Por exemplo, se tentamos abrir um arquivo e essa operação falhar porque o arquivo não existe, nós podemos querer criar o arquivo em vez de terminar o processo.

Lembre-se do Capítulo 2, na seção “Tratando Potenciais Falhas com o Tipo Result
” que o enum `Result` é definido como tendo duas variantes,

Ok e Err, como mostrado a seguir:

```rs
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```


```rs
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("Houve um problema ao abrir o arquivo: {:?}", error)
        },
    };
}
```

tratamento erro

```rs
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => {
                    panic!(
                        "Tentou criar um arquivo e houve um problema: {:?}",
                        e
                    )
                },
            }
        },
        Err(error) => {
            panic!(
                "Houve um problema ao abrir o arquivo: {:?}",
                error
            )
        },
    };
}
```

Atalhos para Pânico em Erro: unwrap e expect

Usar match funciona bem o suficiente, mas pode ser um pouco verboso e nem sempre comunica tão bem a intenção. O tipo Result<T, E> tem vários métodos auxiliares definidos para fazer diversas tarefas. Um desses métodos, chamado unwrap, é um método de atalho que é implementado justamente como o match que escrevemos na Listagem 9-4. Se o valor de Result for da variante Ok, unwrap vai retornar o valor dentro de Ok. Se o Result for da variante Err, unwrap vai chamar a macro panic!. Aqui um exemplo de unwrap em ação:

```rs
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").unwrap();
}

// ou a diferenca é que expect pode ser passado uma mensagem

use std::fs::File;

fn main() {
    let f = File::open("hello.txt").expect("Falhou ao abrir hello.txt");
}
```

# Propagando Erros

```rs
use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
```

## Atalhos para propagar erros:

O ? colocado após um valor de Result é definido para funcionar quase da mesma maneira que as expressões match que definimos para tratar o valor de Result na Listagem 9-6. Se o valor de Result é um Ok, o valor dentro dele vai ser retornado dessa expressão e o programa vai continuar. Se o valor é um Err, o valor dentro dele vai ser retornado da função inteira como se tivéssemos usado a palavra-chave return de modo que o valor de erro é propagado ao código que chamou a função.

A única diferença entre a expressão match da Listagem 9-6 e o que o operador de interrogação faz é que quando usamos o operador de interrogação, os valores de erro passam pela função from definida no trait From na biblioteca padrão. Vários tipos de erro implementam a função from para converter um erro de um tipo em outro. Quando usado pelo operador de interrogação, a chamada à função from converte o tipo de erro que o operador recebe no tipo de erro definido no tipo de retorno da função em que estamos usando ?. Isso é útil quando partes de uma função podem falhar por várias razões diferentes, mas a função retorna um tipo de erro que representa todas as maneiras que a função pode falhar. Enquanto cada tipo de erro implementar a função from para definir como se converter ao tipo de erro retornado, o operador de interrogação lida com a conversão automaticamente.

```rs
use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// ou


use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

```

## ? Somente Pode Ser Usado em Funções Que Retornam Result


## Quando entrar ou nao em panic

## Casos em que Você Tem Mais Informação Que o Compilador

Seria também apropriado chamar unwrap quando você tem outra lógica que assegura que o Result vai ter um valor Ok, mas essa lógica não é algo que o compilador entenda. Você ainda vai ter um valor de Result que precisa lidar: seja qual for a operação que você está chamando, ela ainda tem uma possibilidade de falhar em geral, mesmo que seja logicamente impossível que isso ocorra nessa situação particular. Se você consegue assegurar ao inspecionar manualmente o código que você nunca tera uma variante Err, é perfeitamente aceitável chamar unwrap. Aqui temos um exemplo:


use std::net::IpAddr;

let home = "127.0.0.1".parse::<IpAddr>().unwrap();

ós estamos criando uma instância IpAddr ao analisar uma string hardcoded. Nós podemos ver que 127.0.0.1 é um endereço de IP válido, então é aceitável usar unwrap aqui. No entanto, ter uma string válida hardcoded não muda o tipo retornado pelo método parse: ainda teremos um valor de Result, e o compilador ainda vai nos fazer tratar o Result como se a variante Err fosse uma possibilidade, porque o compilador não é inteligente o bastante para ver que essa string é sempre um endereço IP válido. Se a string de endereço IP viesse de um usuário ao invés de ser hardcoded no programa, e portanto, de fato tivesse uma possibilidade de falha, nós definitivamente iríamos querer tratar o Result de uma forma mais robusta.