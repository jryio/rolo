//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.10

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "contact_interaction")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    #[serde(skip_deserializing)]
    pub contact_id: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    #[serde(skip_deserializing)]
    pub interaction_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::contact::Entity",
        from = "Column::ContactId",
        to = "super::contact::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Contact,
    #[sea_orm(
        belongs_to = "super::interaction::Entity",
        from = "Column::InteractionId",
        to = "super::interaction::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Interaction,
}

impl Related<super::contact::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Contact.def()
    }
}

impl Related<super::interaction::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Interaction.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
