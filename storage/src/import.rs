use entity::contact;
use sea_orm::{ActiveValue, DatabaseConnection, EntityTrait};
use vcard4::{
    self,
    property::{
        DateAndOrTimeProperty, DateTimeOrTextProperty, TextListProperty, TextOrUriProperty,
        TextProperty,
    },
};

/// Imports vcard file contents into Contact Entities
///
/// Takes the vcard file contents as a str
pub struct Import {}

impl Import {
    // TODO: Start a datbase transaction in SQLITE and perform all of these `INSERT` statements
    // within the transaction
    pub async fn file(db: &DatabaseConnection, source: &str) {
        let strict = false;
        let cards = vcard4::iter(source, strict);
        for card in cards {
            if let Ok(card) = card {
                let name = card.name.as_ref().map_or(
                    ActiveValue::NotSet,
                    |TextListProperty { ref value, .. }| {
                        ActiveValue::set(
                            value
                                .iter()
                                .filter_map(|x| if x.len() > 0 { Some(x.clone()) } else { None })
                                .collect::<Vec<String>>()
                                .join(",")
                                .clone(),
                        )
                    },
                );
                let formatted_name = card
                    .formatted_name
                    .first()
                    .map_or(ActiveValue::NotSet, |TextProperty { value, .. }| {
                        ActiveValue::set(value.clone())
                    });
                let birthday = card
                    .bday
                    .as_ref()
                    .map_or(ActiveValue::NotSet, |ref dt| match dt {
                        DateTimeOrTextProperty::DateTime(DateAndOrTimeProperty {
                            ref value,
                            ..
                        }) => value.first().map_or(ActiveValue::NotSet, |date| {
                            ActiveValue::set(Some(date.to_string()))
                        }),
                        DateTimeOrTextProperty::Text(TextProperty { value, .. }) => {
                            ActiveValue::set(Some(value.clone()))
                        }
                    });
                let photo = card.photo.first().map_or(ActiveValue::NotSet, |p| match p {
                    TextOrUriProperty::Text(TextProperty { value, .. }) => {
                        ActiveValue::set(Some(value.clone().into_bytes()))
                    }
                    TextOrUriProperty::Uri(_) => ActiveValue::NotSet,
                });
                let contact = contact::ActiveModel {
                    id: ActiveValue::NotSet,
                    name,
                    formatted_name,
                    birthday,
                    photo,
                };
                // TODO: Should handle if this fails
                let _ = contact::Entity::insert(contact).exec(db).await;
            }
        }
    }
}
