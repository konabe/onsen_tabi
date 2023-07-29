use super::domain_error::DomainError;

pub struct Hotel {
    name: String,
    has_washitsu: bool,
}

impl Hotel {
    pub fn new(name: &str, has_washitsu: bool) -> Result<Self, DomainError> {
        if name.is_empty() {
            return Err(DomainError);
        }
        return Ok(Self {
            name: name.to_string(),
            has_washitsu,
        });
    }
}

#[test]
fn new_test() {
    let hotel: Result<Hotel, DomainError> = Hotel::new("積善館", true);
    let inside: Hotel = hotel.expect("");
    assert!(inside.name == "積善館");
    assert!(inside.has_washitsu == true);
}

#[test]
#[should_panic]
fn new_test_none() {
    let hotel: Result<Hotel, DomainError> = Hotel::new("", true);
    hotel.unwrap();
}
