use wither::bson::oid::ObjectId;
use wither::bson::{doc};
use wither::Model;
use wither::mongodb::Database;
use crate::errors::AppErrors;

use crate::models::SearchById;
use serde::{Serialize, Deserialize};
use futures::stream::StreamExt;
use serde_json::json;

#[derive(Debug, Model, Serialize, Deserialize, Clone)]
#[model(collection_name = "positions")]
pub struct Position {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub belongs_to: String,
    // 属于的小组
    pub longitude: f64,
    pub latitude: f64,
}

// 为其实现 SearchById 接口
impl SearchById for Position {}

impl Position {
    pub async fn by_group(db: &Database, group_id: &str) -> Result<Vec<Position>, AppErrors> {
        let filter = doc! {"belongs_to": group_id};

        let positions_: Vec<_> = Position::find(db, Some(filter), None)
            .await
            .expect("Failed to find positions")
            .collect()
            .await;
        let mut positions = vec![];
        for position in positions_ {
            if let Ok(position) = position {
                positions.push(position);
            }
        }
        Ok(positions)
    }
    pub fn to_response(self) -> serde_json::Value {
        json!({
            "id": self.id.unwrap().to_hex(),
            "name": self.name,
            "belongs_to": self.belongs_to,
            "longitude": self.longitude,
            "latitude": self.latitude,
        })
    }
}