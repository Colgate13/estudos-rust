### Iteradores

O padrão iterador permite que você execute alguma tarefa em uma sequência de itens por vez. Um iterador é responsável pela lógica de iterar sobre cada item e determinar quando a sequência termina. Ao usar iteradores, você não precisa reimplementar essa lógica.

**Em Rust, iteradores são preguiçosos , o que significa que não têm efeito até que você chame métodos que consumam o iterador para usá-lo.** Por exemplo, o código na Listagem 13-10 cria um iterador sobre os itens no vetor v1chamando o itermétodo definido em Vec<T>. Este código por si só não faz nada de útil.

```rs
let v1 = vec![1, 2, 3];

let v1_iter = v1.iter();

// ---

let v1 = vec![1, 2, 3];

let v1_iter = v1.iter();

for val in v1_iter {
    println!("Got: {val}");
}
```

O iterador é armazenado na v1_itervariável. Depois de criar um iterador, podemos usá-lo de diversas maneiras. Na Listagem 3-5 do Capítulo 3, iteramos sobre um array usando um forloop para executar código em cada um de seus itens. Nos bastidores, isso implicitamente criava e consumia um iterador, mas até agora não explicamos exatamente como isso funciona.

### O IteratorTraço e o nextMétodo

Todos os iteradores implementam uma característica chamada Iterator, definida na biblioteca padrão. A definição da característica é semelhante a esta:

```rs
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // methods with default implementations elided
}
```

Observe que esta definição usa uma nova sintaxe: type Iteme Self::Item, que definem um tipo associado a esta característica. Falaremos sobre tipos associados em detalhes no Capítulo 20. Por enquanto, tudo o que você precisa saber é que este código diz que a implementação da Iteratorcaracterística requer que você também defina um Itemtipo, e esse Itemtipo é usado no tipo de retorno do nextmétodo. Em outras palavras, o Itemtipo será o tipo retornado pelo iterador.

A Iterator característica requer apenas que os implementadores definam um método: o next método, que retorna um item do iterador por vez, encapsulado em Some e, quando a iteração termina, retorna None.

Podemos chamar o next método diretamente nos iteradores; a Listagem 13-12 demonstra quais valores são retornados de chamadas repetidas next no iterador criado a partir do vetor.

```rs
#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}
```

Observe que precisávamos tornar v1_iter mutável: chamar o next método em um iterador altera o estado interno que o iterador usa para rastrear sua posição na sequência. Em outras palavras, este código consome , ou seja, esgota, o iterador. Cada chamada a next consome um item do iterador. Não precisávamos tornar v1_iter mutável quando usamos um for loop, pois o loop assumiu a propriedade v1_iter e o tornou mutável em segundo plano.


### Métodos que consomem o iterador
O Trait de iterador possuir diversors metodos diferentes com implementacoes padrao forncedicas pela biblioteca standard. 
Alguns desses metodos chamam o next em sua definicao, e é por isso que voce precisa implementar o next ao implementar o Trait de iterador. Esses metodos consomem o iterador, ou seja, eles esgotam o iterador e não podem ser chamados novamente depois que forem usados.

Os metodos que chama o next sao chamados de **Adaptadores de consumo** porque eles consome o iterador. Um exemplo é o metodos sum, que assume a propriedade do iterador e itera pelos itens chamando repedidament o next, consumindo o assim o iterador e vai adicionando cada item a um total acumulado e retorna o total quando a iteracao é concluida.

```rs
    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];

        let v1_iter = v1.iter();

        let total: i32 = v1_iter.sum();

        assert_eq!(total, 6);
    }
```

Não temos permissão para usar v1_iterdepois da chamada sumporque sumassume a propriedade do iterador em que o chamamos.

### Metodos que produzem outros iteradores

**Adaptadores de iteradores** sao metodos definidos no trait Iterator que nao consome o iterador, em vez disso eles produzem iteradores diferentes alterando algum aspecto do iterador origial. Exemplo **map** que recebe uma clausula para chamar cada item a medida que os itens sao iterados, o metodo map retorna um novo iterador que produza os itens modificacos, a clausua aqui cria um novo iterador no qual cada item do vetor sera incrementado em 1.

```rs
let v1: Vec<i32> = vec![1, 2, 3];

v1.iter().map(|x| x + 1);
```

O codigo acima da avisos. O código na Listagem 13-14 não faz nada; o fechamento que especificamos nunca é chamado. O aviso nos lembra o porquê: adaptadores de iterador são preguiçosos e precisamos consumir o iterador aqui.

Para corrigir esse aviso e consumir o iterador, usaremos o **collect** método que usamos no Capítulo 12 env::argsna Listagem 12-1. Este método consome o iterador e coleta os valores resultantes em um tipo de dado de coleção.

```rs
let v1: Vec<i32> = vec![1, 2, 3];

let v2: Vec<_> = v1.iter().map(|x| x + 1).collect(); // Collect vai consumir o iterador

assert_eq!(v2, vec![2, 3, 4]);
```

**Você pode encadear várias chamadas a adaptadores iteradores para executar ações complexas de forma legível. Mas, como todos os iteradores são preguiçosos, você precisa chamar um dos métodos do adaptador consumidor para obter os resultados das chamadas aos adaptadores iteradores.**

### Closures que capturam o ambiente

Muitos adaptadores de iteradores aceiram closures como argumentos e geralmente os fechamentos que especificaremos como argumento para adaptadores de iteradores serao clousures que capturam o ambiente.

Vamos fazer um exemplo usando o filter, que recebe uma closure , obtem um item do iterador e retorna um **bool** se a closure retornar true o valor sera incluido na iteracao produzida por filter, se a closure retornar false o valor sera descartado.

Usamos filter com uma closure que captura a **shoe_size** de seu ambiente para iterar sobre uma colecao de  Shoe.

```rs
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}
```

#### into_iter cria um iterador que assume a propriedade do vetor

A shoes_in_size função toma como parâmetros um vetor de sapatos e um tamanho de sapato. Ela retorna um vetor contendo apenas sapatos do tamanho especificado.

No corpo de shoes_in_size, chamamos into_iter para criar um iterador que assume a propriedade do vetor. Em seguida, chamamos filterpara adaptar esse iterador em um novo iterador que contém apenas elementos para os quais o fechamento retorna true.

O fechamento captura o shoe_sizeparâmetro do ambiente e compara o valor com o tamanho de cada sapato, mantendo apenas os sapatos do tamanho especificado. Por fim, a chamada collectreúne os valores retornados pelo iterador adaptado em um vetor retornado pela função.

O teste mostra que quando chamamos shoes_in_size, obtemos apenas sapatos que têm o mesmo tamanho que o valor que especificamos.


into_inter, iter_mut e iter

🍽️ 1. .iter() → “Me empresta rapidinho?”

for fruta in caixa.iter() {
    println!("Eu vi a fruta: {}", fruta);
}

    Você está só olhando a fruta.

    A fruta continua na caixa depois do loop.

    Você não pode comer (mover).

    fruta é só uma referência (&str), tipo: “tô só olhando, juro”.

🍽️ 2. .iter_mut() → “Me empresta, mas deixa eu mexer”

for fruta in caixa.iter_mut() {
    fruta.make_mutant(); // imagina que você mexe nelas
}

    Você ainda não tirou da caixa, mas pode mexer nelas.

    Continua sendo um empréstimo, mas com permissão de alteração.

🍽️ 3. .into_iter() → “Tô pegando pra mim!”

for fruta in caixa.into_iter() {
    println!("Comi a fruta: {}", fruta);
}

    Aqui você pega as frutas da caixa e come.

    Ou seja: você toma posse das frutas, uma por uma.

    A caixa fica vazia (ou nem pode mais ser usada), porque você pegou tudo dela.


🔥 Tradução emocional:

    .iter() → “Prometo que só vou olhar.”

    .iter_mut() → “Vou dar uma ajeitada, mas devolvo.”

    .into_iter() → “Tô levando. Já era.”