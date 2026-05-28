use core::fmt;

pub type MtResult<T> = Result<T, MtError>;

pub enum MtError {
    // General
    DtiNotFound(&'static str),

    // Dti
    FailedToCreateInstance(&'static str),

    // fs
    FailedToOpenFile,
}

impl fmt::Display for MtError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MtError::DtiNotFound(dti) => write!(f, "Could not find Dti: {dti}"),
            MtError::FailedToCreateInstance(type_name) => write!(f, "Could not create instance of type {type_name}"),
            MtError::FailedToOpenFile => write!(f, "Could not open file"),
        }
    }
}
