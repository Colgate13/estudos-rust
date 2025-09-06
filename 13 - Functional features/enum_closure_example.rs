#[derive(Debug, Clone)]
enum Status {
    Pending,
    InProgress,
    Completed,
    Failed,
}

fn main() {
    // Exemplo 1: Usando referência mutável
    let mut status = Status::Pending;
    println!("Status inicial: {:?}", status);
    
    {
        let mut closure = || {
            status = Status::InProgress;
            println!("Status modificado dentro da closure: {:?}", status);
        };
        
        closure();
    }
    
    println!("Status após closure: {:?}", status);
    
    // Exemplo 2: Closure que pode ser chamada múltiplas vezes
    let mut contador = 0;
    let mut process_status = || {
        match status {
            Status::Pending => {
                status = Status::InProgress;
                contador += 1;
            },
            Status::InProgress => {
                status = Status::Completed;
                contador += 1;
            },
            Status::Completed => {
                println!("Já está completo!");
            },
            Status::Failed => {
                status = Status::Pending;
                contador += 1;
            }
        }
        println!("Status: {:?}, Contador: {}", status, contador);
    };
    
    process_status(); // InProgress
    process_status(); // Completed
    process_status(); // Já está completo!
    
    println!("Status final: {:?}", status);
    println!("Contador final: {}", contador);
}
