// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    AccessDeniedException(crate::error::AccessDeniedException),
    InternalServiceException(crate::error::InternalServiceException),
    InvalidRequestException(crate::error::InvalidRequestException),
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    ThrottlingException(crate::error::ThrottlingException),
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::AccessDeniedException(inner) => inner.fmt(f),
            Error::InternalServiceException(inner) => inner.fmt(f),
            Error::InvalidRequestException(inner) => inner.fmt(f),
            Error::ResourceNotFoundException(inner) => inner.fmt(f),
            Error::ThrottlingException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl<R>
    From<smithy_http::result::SdkError<crate::error::ListRealtimeContactAnalysisSegmentsError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: smithy_http::result::SdkError<
            crate::error::ListRealtimeContactAnalysisSegmentsError,
            R,
        >,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::ListRealtimeContactAnalysisSegmentsErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::ListRealtimeContactAnalysisSegmentsErrorKind::InternalServiceException(inner) => Error::InternalServiceException(inner),
                crate::error::ListRealtimeContactAnalysisSegmentsErrorKind::InvalidRequestException(inner) => Error::InvalidRequestException(inner),
                crate::error::ListRealtimeContactAnalysisSegmentsErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
                crate::error::ListRealtimeContactAnalysisSegmentsErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
                crate::error::ListRealtimeContactAnalysisSegmentsErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl std::error::Error for Error {}