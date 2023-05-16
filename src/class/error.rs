use crate::class::*;

use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq, Eq, Hash)]
pub enum ClassError {
    #[error("illegal status")]
    IllegalStatus,

    #[error("invalid capabilities")]
    InvalidCapabilities,

    #[error("invalid message ID")]
    InvalidMsgId,

    #[error("invalid terminal character")]
    InvalidTermChar,

    #[error("tag check failure")]
    TagCheckFailure,

    #[error("truncated bulk-out")]
    TruncatedBulkOut,

    #[error("truncated control response")]
    TruncatedControlResponse,

    #[error("truncated header")]
    TruncatedHeader,

    #[error("unexpected status \"{0:?}\"")]
    UnexpectedStatus(Status),

    #[error("unsupported feature")]
    UnsupportedFeature,
}
