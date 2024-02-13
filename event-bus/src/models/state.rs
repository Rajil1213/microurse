use crate::repository::db::Db;

use super::ServiceClient;

#[derive(Debug, Clone, Default)]
pub struct AppState {
    client: ServiceClient,
    db: Db,
}

impl AppState {
    pub fn client(&self) -> &ServiceClient {
        &self.client
    }

    pub fn db(&self) -> &Db {
        &self.db
    }
}
