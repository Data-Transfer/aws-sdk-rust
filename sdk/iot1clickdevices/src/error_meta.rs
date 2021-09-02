// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    ForbiddenException(crate::error::ForbiddenException),
    InternalFailureException(crate::error::InternalFailureException),
    InvalidRequestException(crate::error::InvalidRequestException),
    PreconditionFailedException(crate::error::PreconditionFailedException),
    RangeNotSatisfiableException(crate::error::RangeNotSatisfiableException),
    ResourceConflictException(crate::error::ResourceConflictException),
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ForbiddenException(inner) => inner.fmt(f),
            Error::InternalFailureException(inner) => inner.fmt(f),
            Error::InvalidRequestException(inner) => inner.fmt(f),
            Error::PreconditionFailedException(inner) => inner.fmt(f),
            Error::RangeNotSatisfiableException(inner) => inner.fmt(f),
            Error::ResourceConflictException(inner) => inner.fmt(f),
            Error::ResourceNotFoundException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::ClaimDevicesByClaimCodeError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::ClaimDevicesByClaimCodeError, R>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ClaimDevicesByClaimCodeErrorKind::ForbiddenException(inner) => {
                    Error::ForbiddenException(inner)
                }
                crate::error::ClaimDevicesByClaimCodeErrorKind::InternalFailureException(inner) => {
                    Error::InternalFailureException(inner)
                }
                crate::error::ClaimDevicesByClaimCodeErrorKind::InvalidRequestException(inner) => {
                    Error::InvalidRequestException(inner)
                }
                crate::error::ClaimDevicesByClaimCodeErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::DescribeDeviceError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::DescribeDeviceError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeDeviceErrorKind::InternalFailureException(inner) => {
                    Error::InternalFailureException(inner)
                }
                crate::error::DescribeDeviceErrorKind::InvalidRequestException(inner) => {
                    Error::InvalidRequestException(inner)
                }
                crate::error::DescribeDeviceErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::DescribeDeviceErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::FinalizeDeviceClaimError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::FinalizeDeviceClaimError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::FinalizeDeviceClaimErrorKind::InternalFailureException(inner) => {
                    Error::InternalFailureException(inner)
                }
                crate::error::FinalizeDeviceClaimErrorKind::InvalidRequestException(inner) => {
                    Error::InvalidRequestException(inner)
                }
                crate::error::FinalizeDeviceClaimErrorKind::PreconditionFailedException(inner) => {
                    Error::PreconditionFailedException(inner)
                }
                crate::error::FinalizeDeviceClaimErrorKind::ResourceConflictException(inner) => {
                    Error::ResourceConflictException(inner)
                }
                crate::error::FinalizeDeviceClaimErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::FinalizeDeviceClaimErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::GetDeviceMethodsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::GetDeviceMethodsError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::GetDeviceMethodsErrorKind::InternalFailureException(inner) => {
                    Error::InternalFailureException(inner)
                }
                crate::error::GetDeviceMethodsErrorKind::InvalidRequestException(inner) => {
                    Error::InvalidRequestException(inner)
                }
                crate::error::GetDeviceMethodsErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::GetDeviceMethodsErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::InitiateDeviceClaimError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::InitiateDeviceClaimError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::InitiateDeviceClaimErrorKind::InternalFailureException(inner) => {
                    Error::InternalFailureException(inner)
                }
                crate::error::InitiateDeviceClaimErrorKind::InvalidRequestException(inner) => {
                    Error::InvalidRequestException(inner)
                }
                crate::error::InitiateDeviceClaimErrorKind::ResourceConflictException(inner) => {
                    Error::ResourceConflictException(inner)
                }
                crate::error::InitiateDeviceClaimErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::InitiateDeviceClaimErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::InvokeDeviceMethodError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::InvokeDeviceMethodError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::InvokeDeviceMethodErrorKind::InternalFailureException(inner) => {
                    Error::InternalFailureException(inner)
                }
                crate::error::InvokeDeviceMethodErrorKind::InvalidRequestException(inner) => {
                    Error::InvalidRequestException(inner)
                }
                crate::error::InvokeDeviceMethodErrorKind::PreconditionFailedException(inner) => {
                    Error::PreconditionFailedException(inner)
                }
                crate::error::InvokeDeviceMethodErrorKind::RangeNotSatisfiableException(inner) => {
                    Error::RangeNotSatisfiableException(inner)
                }
                crate::error::InvokeDeviceMethodErrorKind::ResourceConflictException(inner) => {
                    Error::ResourceConflictException(inner)
                }
                crate::error::InvokeDeviceMethodErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::InvokeDeviceMethodErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::ListDeviceEventsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::ListDeviceEventsError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListDeviceEventsErrorKind::InternalFailureException(inner) => {
                    Error::InternalFailureException(inner)
                }
                crate::error::ListDeviceEventsErrorKind::InvalidRequestException(inner) => {
                    Error::InvalidRequestException(inner)
                }
                crate::error::ListDeviceEventsErrorKind::RangeNotSatisfiableException(inner) => {
                    Error::RangeNotSatisfiableException(inner)
                }
                crate::error::ListDeviceEventsErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::ListDeviceEventsErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::ListDevicesError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::ListDevicesError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListDevicesErrorKind::InternalFailureException(inner) => {
                    Error::InternalFailureException(inner)
                }
                crate::error::ListDevicesErrorKind::InvalidRequestException(inner) => {
                    Error::InvalidRequestException(inner)
                }
                crate::error::ListDevicesErrorKind::RangeNotSatisfiableException(inner) => {
                    Error::RangeNotSatisfiableException(inner)
                }
                crate::error::ListDevicesErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::ListTagsForResourceError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::ListTagsForResourceError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListTagsForResourceErrorKind::InternalFailureException(inner) => {
                    Error::InternalFailureException(inner)
                }
                crate::error::ListTagsForResourceErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::ListTagsForResourceErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::TagResourceError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::TagResourceError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::TagResourceErrorKind::InternalFailureException(inner) => {
                    Error::InternalFailureException(inner)
                }
                crate::error::TagResourceErrorKind::InvalidRequestException(inner) => {
                    Error::InvalidRequestException(inner)
                }
                crate::error::TagResourceErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::TagResourceErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::UnclaimDeviceError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::UnclaimDeviceError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::UnclaimDeviceErrorKind::InternalFailureException(inner) => {
                    Error::InternalFailureException(inner)
                }
                crate::error::UnclaimDeviceErrorKind::InvalidRequestException(inner) => {
                    Error::InvalidRequestException(inner)
                }
                crate::error::UnclaimDeviceErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::UnclaimDeviceErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::UntagResourceError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::UntagResourceError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::UntagResourceErrorKind::InternalFailureException(inner) => {
                    Error::InternalFailureException(inner)
                }
                crate::error::UntagResourceErrorKind::InvalidRequestException(inner) => {
                    Error::InvalidRequestException(inner)
                }
                crate::error::UntagResourceErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::UntagResourceErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::UpdateDeviceStateError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::UpdateDeviceStateError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::UpdateDeviceStateErrorKind::InternalFailureException(inner) => {
                    Error::InternalFailureException(inner)
                }
                crate::error::UpdateDeviceStateErrorKind::InvalidRequestException(inner) => {
                    Error::InvalidRequestException(inner)
                }
                crate::error::UpdateDeviceStateErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::UpdateDeviceStateErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl std::error::Error for Error {}