#[test]
fn test_vcard_properties() {
    let contacts = include_str!("all-contacts.vcf");
    let mut cards = vcard4::iter(contacts, false /* strict */);
    for card in cards {
        match card {
            Ok(c) => {
                println!("Card = {c:#?}");
            }
            Err(e) => {
                println!("This card has an error: {e}")
            }
        }
    }
}
