use super::publish_job;
use crate::state::WorkerClient;
use futari_errors::errors::Errors;
use futari_worker::jobs::index::user::{IndexUserJob, UserIndexAction};
use futari_worker::nats::streams::INDEX_USER_SUBJECT;
use tracing::info;
use uuid::Uuid;

/// Push a user indexing job to the worker queue.
pub async fn index_user(worker: &WorkerClient, user_id: Uuid) -> Result<(), Errors> {
    let job = IndexUserJob {
        user_id,
        action: UserIndexAction::Index,
    };

    publish_job(worker, INDEX_USER_SUBJECT, &job).await?;

    info!(%user_id, action = "index", "User index job queued");
    Ok(())
}

/// Push a user deletion job to the worker queue.
pub async fn delete_user_from_index(worker: &WorkerClient, user_id: Uuid) -> Result<(), Errors> {
    let job = IndexUserJob {
        user_id,
        action: UserIndexAction::Delete,
    };

    publish_job(worker, INDEX_USER_SUBJECT, &job).await?;

    info!(%user_id, action = "delete", "User deletion job queued");
    Ok(())
}
