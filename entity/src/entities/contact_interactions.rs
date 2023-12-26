//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.10

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "contact_interactions")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub contact_id: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub interaction_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::contacts::Entity",
        from = "Column::ContactId",
        to = "super::contacts::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Contacts,
    #[sea_orm(
        belongs_to = "super::interactions::Entity",
        from = "Column::InteractionId",
        to = "super::interactions::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Interactions,
}

impl Related<super::contacts::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Contacts.def()
    }
}

impl Related<super::interactions::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Interactions.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}