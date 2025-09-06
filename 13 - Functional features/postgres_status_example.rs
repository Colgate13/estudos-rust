use std::time::Instant;

#[derive(Debug, Clone)]
enum ApplicationStatus {
    Active,
    Inactive,
}

// Simulando as estruturas que você está usando
struct PoolHandler {
    // pool: Pool
}

impl PoolHandler {
    fn get(&self) -> Result<Connection, String> {
        // Simula sucesso ou erro
        Ok(Connection {})
    }
}

struct Connection {}

fn sql_query(query: &str) -> Query {
    Query {}
}

struct Query {}

impl Query {
    fn execute(&self, connection: &mut Connection) -> Result<(), String> {
        // Simula sucesso ou erro
        Ok(())
    }
}

// Simulando web::block
async fn web_block<F, T>(f: F) -> Result<T, String>
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static,
{
    // Simula execução em thread separada
    Ok(f())
}

async fn check_postgres_status(pool_handler: &PoolHandler) -> Result<ApplicationStatus, String> {
    let start = Instant::now();
    
    // Solução 1: Retornar o status da closure
    let postgres_status = web_block(move || {
        // Check connection to database
        let current_connection = match pool_handler.get() {
            Err(_error) => return ApplicationStatus::Inactive,
            Ok(connection) => connection,
        };

        let mut current_connection = current_connection;

        let select_result = match sql_query("SELECT 1;").execute(&mut current_connection) {
            Err(_error) => false,
            _ => true,
        };
        
        if !select_result {
            return ApplicationStatus::Inactive;
        }

        ApplicationStatus::Active
    }).await?;

    println!("Postgres status: {:?}", postgres_status);
    println!("Check took: {:?}", start.elapsed());
    
    Ok(postgres_status)
}

#[tokio::main]
async fn main() -> Result<(), String> {
    let pool_handler = PoolHandler {};
    
    let status = check_postgres_status(&pool_handler).await?;
    println!("Final status: {:?}", status);
    
    Ok(())
}
