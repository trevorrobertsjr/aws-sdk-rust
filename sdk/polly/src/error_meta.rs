// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    EngineNotSupportedException(crate::error::EngineNotSupportedException),
    InvalidLexiconException(crate::error::InvalidLexiconException),
    InvalidNextTokenException(crate::error::InvalidNextTokenException),
    InvalidS3BucketException(crate::error::InvalidS3BucketException),
    InvalidS3KeyException(crate::error::InvalidS3KeyException),
    InvalidSampleRateException(crate::error::InvalidSampleRateException),
    InvalidSnsTopicArnException(crate::error::InvalidSnsTopicArnException),
    InvalidSsmlException(crate::error::InvalidSsmlException),
    InvalidTaskIdException(crate::error::InvalidTaskIdException),
    LanguageNotSupportedException(crate::error::LanguageNotSupportedException),
    LexiconNotFoundException(crate::error::LexiconNotFoundException),
    LexiconSizeExceededException(crate::error::LexiconSizeExceededException),
    MarksNotSupportedForFormatException(crate::error::MarksNotSupportedForFormatException),
    MaxLexemeLengthExceededException(crate::error::MaxLexemeLengthExceededException),
    MaxLexiconsNumberExceededException(crate::error::MaxLexiconsNumberExceededException),
    ServiceFailureException(crate::error::ServiceFailureException),
    SsmlMarksNotSupportedForTextTypeException(
        crate::error::SsmlMarksNotSupportedForTextTypeException,
    ),
    SynthesisTaskNotFoundException(crate::error::SynthesisTaskNotFoundException),
    TextLengthExceededException(crate::error::TextLengthExceededException),
    UnsupportedPlsAlphabetException(crate::error::UnsupportedPlsAlphabetException),
    UnsupportedPlsLanguageException(crate::error::UnsupportedPlsLanguageException),
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::EngineNotSupportedException(inner) => inner.fmt(f),
            Error::InvalidLexiconException(inner) => inner.fmt(f),
            Error::InvalidNextTokenException(inner) => inner.fmt(f),
            Error::InvalidS3BucketException(inner) => inner.fmt(f),
            Error::InvalidS3KeyException(inner) => inner.fmt(f),
            Error::InvalidSampleRateException(inner) => inner.fmt(f),
            Error::InvalidSnsTopicArnException(inner) => inner.fmt(f),
            Error::InvalidSsmlException(inner) => inner.fmt(f),
            Error::InvalidTaskIdException(inner) => inner.fmt(f),
            Error::LanguageNotSupportedException(inner) => inner.fmt(f),
            Error::LexiconNotFoundException(inner) => inner.fmt(f),
            Error::LexiconSizeExceededException(inner) => inner.fmt(f),
            Error::MarksNotSupportedForFormatException(inner) => inner.fmt(f),
            Error::MaxLexemeLengthExceededException(inner) => inner.fmt(f),
            Error::MaxLexiconsNumberExceededException(inner) => inner.fmt(f),
            Error::ServiceFailureException(inner) => inner.fmt(f),
            Error::SsmlMarksNotSupportedForTextTypeException(inner) => inner.fmt(f),
            Error::SynthesisTaskNotFoundException(inner) => inner.fmt(f),
            Error::TextLengthExceededException(inner) => inner.fmt(f),
            Error::UnsupportedPlsAlphabetException(inner) => inner.fmt(f),
            Error::UnsupportedPlsLanguageException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::DeleteLexiconError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::DeleteLexiconError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DeleteLexiconErrorKind::LexiconNotFoundException(inner) => {
                    Error::LexiconNotFoundException(inner)
                }
                crate::error::DeleteLexiconErrorKind::ServiceFailureException(inner) => {
                    Error::ServiceFailureException(inner)
                }
                crate::error::DeleteLexiconErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::DescribeVoicesError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::DescribeVoicesError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeVoicesErrorKind::InvalidNextTokenException(inner) => {
                    Error::InvalidNextTokenException(inner)
                }
                crate::error::DescribeVoicesErrorKind::ServiceFailureException(inner) => {
                    Error::ServiceFailureException(inner)
                }
                crate::error::DescribeVoicesErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::GetLexiconError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::GetLexiconError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::GetLexiconErrorKind::LexiconNotFoundException(inner) => {
                    Error::LexiconNotFoundException(inner)
                }
                crate::error::GetLexiconErrorKind::ServiceFailureException(inner) => {
                    Error::ServiceFailureException(inner)
                }
                crate::error::GetLexiconErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::GetSpeechSynthesisTaskError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::GetSpeechSynthesisTaskError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::GetSpeechSynthesisTaskErrorKind::InvalidTaskIdException(inner) => {
                    Error::InvalidTaskIdException(inner)
                }
                crate::error::GetSpeechSynthesisTaskErrorKind::ServiceFailureException(inner) => {
                    Error::ServiceFailureException(inner)
                }
                crate::error::GetSpeechSynthesisTaskErrorKind::SynthesisTaskNotFoundException(
                    inner,
                ) => Error::SynthesisTaskNotFoundException(inner),
                crate::error::GetSpeechSynthesisTaskErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::ListLexiconsError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::ListLexiconsError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListLexiconsErrorKind::InvalidNextTokenException(inner) => {
                    Error::InvalidNextTokenException(inner)
                }
                crate::error::ListLexiconsErrorKind::ServiceFailureException(inner) => {
                    Error::ServiceFailureException(inner)
                }
                crate::error::ListLexiconsErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::ListSpeechSynthesisTasksError>> for Error {
    fn from(
        err: smithy_http::result::SdkError<crate::error::ListSpeechSynthesisTasksError>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListSpeechSynthesisTasksErrorKind::InvalidNextTokenException(
                    inner,
                ) => Error::InvalidNextTokenException(inner),
                crate::error::ListSpeechSynthesisTasksErrorKind::ServiceFailureException(inner) => {
                    Error::ServiceFailureException(inner)
                }
                crate::error::ListSpeechSynthesisTasksErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::PutLexiconError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::PutLexiconError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::PutLexiconErrorKind::InvalidLexiconException(inner) => {
                    Error::InvalidLexiconException(inner)
                }
                crate::error::PutLexiconErrorKind::LexiconSizeExceededException(inner) => {
                    Error::LexiconSizeExceededException(inner)
                }
                crate::error::PutLexiconErrorKind::MaxLexemeLengthExceededException(inner) => {
                    Error::MaxLexemeLengthExceededException(inner)
                }
                crate::error::PutLexiconErrorKind::MaxLexiconsNumberExceededException(inner) => {
                    Error::MaxLexiconsNumberExceededException(inner)
                }
                crate::error::PutLexiconErrorKind::ServiceFailureException(inner) => {
                    Error::ServiceFailureException(inner)
                }
                crate::error::PutLexiconErrorKind::UnsupportedPlsAlphabetException(inner) => {
                    Error::UnsupportedPlsAlphabetException(inner)
                }
                crate::error::PutLexiconErrorKind::UnsupportedPlsLanguageException(inner) => {
                    Error::UnsupportedPlsLanguageException(inner)
                }
                crate::error::PutLexiconErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::StartSpeechSynthesisTaskError>> for Error {
    fn from(
        err: smithy_http::result::SdkError<crate::error::StartSpeechSynthesisTaskError>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::StartSpeechSynthesisTaskErrorKind::EngineNotSupportedException(inner) => Error::EngineNotSupportedException(inner),
                crate::error::StartSpeechSynthesisTaskErrorKind::InvalidS3BucketException(inner) => Error::InvalidS3BucketException(inner),
                crate::error::StartSpeechSynthesisTaskErrorKind::InvalidS3KeyException(inner) => Error::InvalidS3KeyException(inner),
                crate::error::StartSpeechSynthesisTaskErrorKind::InvalidSampleRateException(inner) => Error::InvalidSampleRateException(inner),
                crate::error::StartSpeechSynthesisTaskErrorKind::InvalidSnsTopicArnException(inner) => Error::InvalidSnsTopicArnException(inner),
                crate::error::StartSpeechSynthesisTaskErrorKind::InvalidSsmlException(inner) => Error::InvalidSsmlException(inner),
                crate::error::StartSpeechSynthesisTaskErrorKind::LanguageNotSupportedException(inner) => Error::LanguageNotSupportedException(inner),
                crate::error::StartSpeechSynthesisTaskErrorKind::LexiconNotFoundException(inner) => Error::LexiconNotFoundException(inner),
                crate::error::StartSpeechSynthesisTaskErrorKind::MarksNotSupportedForFormatException(inner) => Error::MarksNotSupportedForFormatException(inner),
                crate::error::StartSpeechSynthesisTaskErrorKind::ServiceFailureException(inner) => Error::ServiceFailureException(inner),
                crate::error::StartSpeechSynthesisTaskErrorKind::SsmlMarksNotSupportedForTextTypeException(inner) => Error::SsmlMarksNotSupportedForTextTypeException(inner),
                crate::error::StartSpeechSynthesisTaskErrorKind::TextLengthExceededException(inner) => Error::TextLengthExceededException(inner),
                crate::error::StartSpeechSynthesisTaskErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::SynthesizeSpeechError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::SynthesizeSpeechError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::SynthesizeSpeechErrorKind::EngineNotSupportedException(inner) => Error::EngineNotSupportedException(inner),
                crate::error::SynthesizeSpeechErrorKind::InvalidSampleRateException(inner) => Error::InvalidSampleRateException(inner),
                crate::error::SynthesizeSpeechErrorKind::InvalidSsmlException(inner) => Error::InvalidSsmlException(inner),
                crate::error::SynthesizeSpeechErrorKind::LanguageNotSupportedException(inner) => Error::LanguageNotSupportedException(inner),
                crate::error::SynthesizeSpeechErrorKind::LexiconNotFoundException(inner) => Error::LexiconNotFoundException(inner),
                crate::error::SynthesizeSpeechErrorKind::MarksNotSupportedForFormatException(inner) => Error::MarksNotSupportedForFormatException(inner),
                crate::error::SynthesizeSpeechErrorKind::ServiceFailureException(inner) => Error::ServiceFailureException(inner),
                crate::error::SynthesizeSpeechErrorKind::SsmlMarksNotSupportedForTextTypeException(inner) => Error::SsmlMarksNotSupportedForTextTypeException(inner),
                crate::error::SynthesizeSpeechErrorKind::TextLengthExceededException(inner) => Error::TextLengthExceededException(inner),
                crate::error::SynthesizeSpeechErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl std::error::Error for Error {}
