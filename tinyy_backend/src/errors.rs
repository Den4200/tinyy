pub enum TinyUrlError {
    CodeNotFound,
    GenericServerError,
    InvalidHttpUrl,
    UniqueCodeViolation
}
