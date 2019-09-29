#[derive(Debug)]
pub struct PapiError(i32);

impl From<i32> for PapiError {
    fn from(errno: i32) -> Self {
        PapiError(errno)
    }
}