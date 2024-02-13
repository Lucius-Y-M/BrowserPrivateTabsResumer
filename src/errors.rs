#[derive(Debug, Clone, Copy)]
pub enum Errors {


    MutexLockFailedError,

    EventReadFailedError,

    // TOML file IO
    FSReadError,
    NoTOMLFilesFoundError,
    ParseTOMLFilError,
    TOMLFileIsEmpty,

    // URL
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