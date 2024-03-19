use uuid::Uuid;

pub struct UuidService {
}

impl UuidService {
    #[allow(non_snake_case)]
    pub fn is_uuid_valid (uuid: &String) -> bool {
        return match Uuid::try_parse(&uuid) {
            Err(_) => false,
            _ => true,
        };
    }
}
