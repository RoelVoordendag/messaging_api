use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Deserialize, Serialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Message,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Message => Entity::has_many(super::messages::Entity).into(),
        }
    }
}

impl Related<super::messages::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Message.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
