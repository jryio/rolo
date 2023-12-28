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
            // Contact Table
            .create_table(
                Table::create()
                    .table(Contact::Table)
                    .if_not_exists()
                    // ID
                    .col(
                        ColumnDef::new(Contact::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Contact::Name)
                            .string()
                            .not_null()
                            .default(Expr::value("")),
                    )
                    .col(
                        ColumnDef::new(Contact::FormattedName)
                            .string()
                            .not_null()
                            .default(Expr::value("")),
                    )
                    // SeaORM treats time crate values as strings in the database
                    .col(ColumnDef::new(Contact::Birthday).text())
                    // In sqlite BlobSize::Long is a NOOP
                    .col(ColumnDef::new(Contact::Photo).blob(BlobSize::Long))
                    .to_owned(),
            )
            .await?;

        manager
            // Interaction Table
            .create_table(
                Table::create()
                    .table(Interaction::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Interaction::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Interaction::DateTime).text().not_null())
                    .col(ColumnDef::new(Interaction::Description).text())
                    .col(ColumnDef::new(Interaction::Type).text().not_null().check(
                        Expr::col(Interaction::Type).in_tuples([
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
            // Interval Table
            .create_table(
                Table::create()
                    .table(Interval::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Interval::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Interval::Interval).text().not_null())
                    .col(ColumnDef::new(Interval::ContactId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("interval_to_contact")
                            .from(Interval::Table, Interval::ContactId)
                            .to(Contact::Table, Contact::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            // ContactInteraction Table
            .create_table(
                Table::create()
                    .table(ContactInteraction::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ContactInteraction::ContactId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ContactInteraction::InteractionId)
                            .integer()
                            .not_null(),
                    )
                    .primary_key(
                        Index::create()
                            .col(ContactInteraction::ContactId)
                            .col(ContactInteraction::InteractionId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("contact_interactions_to_contact")
                            .from(ContactInteraction::Table, ContactInteraction::ContactId)
                            .to(Contact::Table, Contact::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("contact_interactions_to_interactions")
                            .from(ContactInteraction::Table, ContactInteraction::InteractionId)
                            .to(Interaction::Table, Interaction::Id)
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
            .drop_table(Table::drop().table(Contact::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Interaction::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Interval::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(ContactInteraction::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Contact {
    Table,
    Id,
    Name,
    FormattedName,
    Birthday,
    Photo,
}

#[derive(DeriveIden)]
pub enum Interaction {
    Table,
    Id,
    DateTime,
    Type,
    Description,
}

#[derive(DeriveIden)]
pub enum Interval {
    Table,
    Id,
    ContactId,
    Interval,
}

/// This is a junction table between Contact <> Interaction
#[derive(DeriveIden)]
pub enum ContactInteraction {
    Table,
    ContactId,
    InteractionId,
}
