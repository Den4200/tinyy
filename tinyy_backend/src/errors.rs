pub enum TinyUrlError {
    CodeNotFound,
    GenericServerError,
    InvalidHttpUrl,
    UniqueCodeViolation,
}

pub enum UserError {
    DuplicateEmail,
    GenericServerError,
    InvalidUser,
    UserNotFound,
}
