use std::{collections::HashMap, time::Instant};
use actix_web::{web::{self, Data}, Result};
use diesel::{sql_query, RunQueryDsl};

use crate::infra::{database::PoolHandler, Application, ApplicationStatus, EApplications, StatusResponder};

pub async fn execute(pool_handler: Data<PoolHandler>) -> Result<web::Json<StatusResponder>> {
  let mut applications: HashMap<EApplications, Application> = HashMap::new();

  let start = Instant::now();
  let postgres_status = web::block(move || -> ApplicationStatus {
    let current_connection = match pool_handler.pool.get() {
      Err(_) => Err(false),
      Ok(connection) => Ok(connection)
    };

    if current_connection.is_err() {
      return ApplicationStatus::Inactive;
    }

    let mut current_connection = current_connection.unwrap();

    let select_result = match sql_query("SELECT 1;").execute(&mut current_connection) {
      Err(_) => false,
      Ok(result) => {
        println!("result: {}", result);
        true
      }
    };
    
    if !select_result {
      return ApplicationStatus::Inactive;
    }

    ApplicationStatus::Active
  }).await?;
  let duration = start.elapsed();
  applications.insert(EApplications::Postgres, Application {
    status: postgres_status,
    response_time: duration.as_millis()
  });

  // Verificar se todas as aplicações estão ativas
  let mut status = ApplicationStatus::Active;
  for (_, application) in &applications {
    if application.status == ApplicationStatus::Inactive {
      status = ApplicationStatus::Inactive;
      break;
    }
  }

  Ok(web::Json(StatusResponder {
      status,
      applications
  }))
}
