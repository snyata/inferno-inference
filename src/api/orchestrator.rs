/// Data Orchestrator
///  TBC
/// 

use tokio::task;
use futures::future::join_all;
use ockam::Context;
use crate::logging::*;
use crate::api::client::ApiClient;
use syn_crabs::

pub struct Orchestrator {
    api_client: ApiClient,
}

impl Orchestrator {
    pub async fn new(api_client: ApiClient) -> Self {
        log.info("Orchestrator initialized: {}", api_client);
        Orchestrator { api_client }
    }

    pub async fn execute_workflow(&self, ctx: &Context, tasks: Vec<Task>) -> Result<()> {
        setup_logging()?;
        let mut handles = vec![];

        log::info("Beginning to handle tasks: {:?}", tasks);
        for task in tasks {
            let api_client =  self.api_client.clone();
            let ctx = ctx.clone();

        handles.push(task::spawn(async move {
            match api_client.send_request(&ctx, &task.url, task.payload, &task.forwarder).await {
                Ok(response) => {
                    log.info(&format!("Task Completed With Response {:?}", response));
                },
                Err(e) => {
                    log.error(&format!("Task Failed With Error {:?}", e));
                }
            }
            }));

        }

        let results: Vec<Result<(), CustomError>> = join_all(handles).await;

        for result in results {
            match result {
                Ok(Ok(_)) => log.info!("Task: {} completed successfully", result),
                Ok(Err(e)) => log.error!(&format!("Task: {} Error", error)),
                Err(e) => log.error!(&format!("Task: {} panicked", error)),
            
            }
        }

        Ok(())
    }
}

