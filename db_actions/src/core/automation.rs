use crate::LaserDatabase;
use gpui::App;
use serde::{Deserialize, Serialize};
use surrealdb::types::SurrealValue;
use tokio::spawn;

#[derive(Serialize, Deserialize, Clone, Debug, Default, SurrealValue)]
pub struct AutomationCluster {
    pub amount: usize,
    pub bills: usize,
}

impl AutomationCluster {
    const ID: (&str, &str) = ("automation", "cluster");

    pub async fn new(pool: &LaserDatabase) -> AutomationCluster {
        let existing: Option<Self> = pool.select(Self::ID).await.ok().flatten();

        if let Some(data) = existing {
            return data;
        }

        let default_cluster = Self::default();
        let data: Option<Self> = pool
            .upsert(Self::ID)
            .content(default_cluster.clone())
            .await
            .ok()
            .flatten();

        data.unwrap_or_default()
    }

    pub fn save_data(&self, pool: LaserDatabase) {
        let data = self.clone();

        spawn(async move {
            let _: Option<Self> = pool.upsert(Self::ID).content(data).await.ok().flatten();
        });
    }
}
