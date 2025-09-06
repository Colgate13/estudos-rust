#[derive(Debug, Clone)]
enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

fn main() {
    let priority = Priority::Low;
    println!("Prioridade inicial: {:?}", priority);
    
    // Closure que retorna o enum modificado
    let increase_priority = |p: Priority| -> Priority {
        match p {
            Priority::Low => Priority::Medium,
            Priority::Medium => Priority::High,
            Priority::High => Priority::Critical,
            Priority::Critical => {
                println!("Já está na prioridade máxima!");
                Priority::Critical
            }
        }
    };
    
    let new_priority = increase_priority(priority.clone());
    println!("Nova prioridade: {:?}", new_priority);
    
    // Encadeando chamadas
    let final_priority = increase_priority(increase_priority(new_priority));
    println!("Prioridade final: {:?}", final_priority);
    
    // Usando com iteradores
    let priorities = vec![Priority::Low, Priority::Medium, Priority::High];
    let upgraded_priorities: Vec<Priority> = priorities
        .into_iter()
        .map(increase_priority)
        .collect();
    
    println!("Prioridades atualizadas: {:?}", upgraded_priorities);
}
