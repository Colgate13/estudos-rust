use std::sync::{Arc, Mutex};
use std::time::Instant;

#[derive(Debug, Clone)]
enum ApplicationStatus {
    Active,
    Inactive,
}

// Simulando as estruturas
struct PoolHandler {}
struct Connection {}
struct Query {}

impl PoolHandler {
    fn get(&self) -> Result<Connection, String> {
        Ok(Connection {})
    }
}

impl Query {
    fn execute(&self, connection: &mut Connection) -> Result<(), String> {
        Ok(())
    }
}

fn sql_query(query: &str) -> Query {
    Query {}
}

async fn web_block<F, T>(f: F) -> Result<T, String>
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static,
{
    Ok(f())
}

async fn check_postgres_with_shared_state(pool_handler: &PoolHandler) -> Result<ApplicationStatus, String> {
    // Solução 2: Usando Arc<Mutex<T>>
    let postgres_status = Arc::new(Mutex::new(ApplicationStatus::Inactive));
    let status_clone = Arc::clone(&postgres_status);
    let start = Instant::now();
    
    web_block(move || {
        // Check connection to database
        let current_connection = match pool_handler.get() {
            Err(_error) => {
                *status_clone.lock().unwrap() = ApplicationStatus::Inactive;
                return;
            },
            Ok(connection) => connection,
        };

        let mut current_connection = current_connection;

        let select_result = match sql_query("SELECT 1;").execute(&mut current_connection) {
            Err(_error) => false,
            _ => true,
        };
        
        if !select_result {
            *status_clone.lock().unwrap() = ApplicationStatus::Inactive;
            return;
        }

        *status_clone.lock().unwrap() = ApplicationStatus::Active;
    }).await?;

    let final_status = postgres_status.lock().unwrap().clone();
    println!("Postgres status: {:?}", final_status);
    println!("Check took: {:?}", start.elapsed());
    
    Ok(final_status)
}

#[tokio::main]
async fn main() -> Result<(), String> {
    let pool_handler = PoolHandler {};
    
    let status = check_postgres_with_shared_state(&pool_handler).await?;
    println!("Final status: {:?}", status);
    
    Ok(())
}
