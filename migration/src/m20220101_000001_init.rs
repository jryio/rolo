use sea_orm_migration::prelude::*;

/*
* By default, SeaORM will not automatically create a transaction for every migration.
*
* Transactions need to be created manually
*
*/
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            // Contacts Table
            .create_table(
                Table::create()
                    .table(Contacts::Table)
                    .if_not_exists()
                    // ID
                    .col(
                        ColumnDef::new(Contacts::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Contacts::Name)
                            .string()
                            .not_null()
                            .default(Expr::value("")),
                    )
                    .col(
                        ColumnDef::new(Contacts::FormatedName)
                            .string()
                            .not_null()
                            .default(Expr::value("")),
                    )
                    // SeaORM treats time crate values as strings in the database
                    .col(ColumnDef::new(Contacts::Birthday).text())
                    // In sqlite BlobSize::Long is a NOOP
                    .col(ColumnDef::new(Contacts::Photo).blob(BlobSize::Long))
                    .to_owned(),
            )
            .await?;

        manager
            // Interactions Table
            .create_table(
                Table::create()
                    .table(Interactions::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Interactions::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Interactions::DateTime).text().not_null())
                    .col(ColumnDef::new(Interactions::Description).text())
                    .col(ColumnDef::new(Interactions::Type).text().not_null().check(
                        Expr::col(Interactions::Type).in_tuples([
                            "phone",
                            "video",
                            "in-person",
                            "other",
                        ]),
                    ))
                    .to_owned(),
            )
            .await?;

        manager
            // Intervals Table
            .create_table(
                Table::create()
                    .table(Intervals::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Intervals::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Intervals::Interval).text().not_null())
                    .col(ColumnDef::new(Intervals::ContactId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("interval_to_contact")
                            .from(Intervals::Table, Intervals::ContactId)
                            .to(Contacts::Table, Contacts::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            // ContactInteractions Table
            .create_table(
                Table::create()
                    .table(ContactInteractions::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ContactInteractions::ContactId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ContactInteractions::InteractionId)
                            .integer()
                            .not_null(),
                    )
                    .primary_key(
                        Index::create()
                            .col(ContactInteractions::ContactId)
                            .col(ContactInteractions::InteractionId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("contact_interactions_to_contact")
                            .from(ContactInteractions::Table, ContactInteractions::ContactId)
                            .to(Contacts::Table, Contacts::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("contact_interactions_to_interactions")
                            .from(
                                ContactInteractions::Table,
                                ContactInteractions::InteractionId,
                            )
                            .to(Interactions::Table, Interactions::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        // todo!();

        manager
            .drop_table(Table::drop().table(Contacts::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Interactions::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Intervals::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(ContactInteractions::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Contacts {
    Table,
    Id,
    Name,
    FormatedName,
    Birthday,
    Photo,
}

#[derive(DeriveIden)]
pub enum Interactions {
    Table,
    Id,
    DateTime,
    Type,
    Description,
}

#[derive(DeriveIden)]
pub enum Intervals {
    Table,
    Id,
    ContactId,
    Interval,
}

#[derive(DeriveIden)]
pub enum ContactInteractions {
    Table,
    ContactId,
    InteractionId,
}
