#[derive(Debug, Clone, Copy)]
pub enum Errors {
    RequestGetError,
    URLParseError,
    ParseTextError,

    PairAlreadyExistsError,

    LookupDeletionFailedError,
    NothingFoundError,

    SelectorGenerateError,
    ParseTitleError,

    LookupFailedError,
}
