#[derive(Debug, Clone, Copy)]
pub enum Errors {

    // TOML file IO
    ParseTOMLFilError,
    TOMLFileIsEmpty,

    RequestGetError,
    URLParseError,
    ParseTextError,

    PairAlreadyExistsError,

    LookupDeletionFailedError,
    NothingFoundError,

    SelectorGenerateError,
    ParseTitleError,

    LookupFailedError,


    WriteToStdoutError,

    CursorPosOverflowError,
}
