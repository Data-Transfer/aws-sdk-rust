// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    ConcurrentUpdateException(crate::error::ConcurrentUpdateException),
    InternalServiceException(crate::error::InternalServiceException),
    InvalidNextTokenException(crate::error::InvalidNextTokenException),
    LimitExceededException(crate::error::LimitExceededException),
    ObjectNotFoundException(crate::error::ObjectNotFoundException),
    ValidationException(crate::error::ValidationException),
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ConcurrentUpdateException(inner) => inner.fmt(f),
            Error::InternalServiceException(inner) => inner.fmt(f),
            Error::InvalidNextTokenException(inner) => inner.fmt(f),
            Error::LimitExceededException(inner) => inner.fmt(f),
            Error::ObjectNotFoundException(inner) => inner.fmt(f),
            Error::ValidationException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::CreateScalingPlanError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::CreateScalingPlanError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::CreateScalingPlanErrorKind::ConcurrentUpdateException(inner) => {
                    Error::ConcurrentUpdateException(inner)
                }
                crate::error::CreateScalingPlanErrorKind::InternalServiceException(inner) => {
                    Error::InternalServiceException(inner)
                }
                crate::error::CreateScalingPlanErrorKind::LimitExceededException(inner) => {
                    Error::LimitExceededException(inner)
                }
                crate::error::CreateScalingPlanErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::CreateScalingPlanErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::DeleteScalingPlanError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::DeleteScalingPlanError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DeleteScalingPlanErrorKind::ConcurrentUpdateException(inner) => {
                    Error::ConcurrentUpdateException(inner)
                }
                crate::error::DeleteScalingPlanErrorKind::InternalServiceException(inner) => {
                    Error::InternalServiceException(inner)
                }
                crate::error::DeleteScalingPlanErrorKind::ObjectNotFoundException(inner) => {
                    Error::ObjectNotFoundException(inner)
                }
                crate::error::DeleteScalingPlanErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::DeleteScalingPlanErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::DescribeScalingPlanResourcesError>>
    for Error
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::DescribeScalingPlanResourcesError>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeScalingPlanResourcesErrorKind::ConcurrentUpdateException(
                    inner,
                ) => Error::ConcurrentUpdateException(inner),
                crate::error::DescribeScalingPlanResourcesErrorKind::InternalServiceException(
                    inner,
                ) => Error::InternalServiceException(inner),
                crate::error::DescribeScalingPlanResourcesErrorKind::InvalidNextTokenException(
                    inner,
                ) => Error::InvalidNextTokenException(inner),
                crate::error::DescribeScalingPlanResourcesErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::DescribeScalingPlanResourcesErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::DescribeScalingPlansError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::DescribeScalingPlansError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeScalingPlansErrorKind::ConcurrentUpdateException(inner) => {
                    Error::ConcurrentUpdateException(inner)
                }
                crate::error::DescribeScalingPlansErrorKind::InternalServiceException(inner) => {
                    Error::InternalServiceException(inner)
                }
                crate::error::DescribeScalingPlansErrorKind::InvalidNextTokenException(inner) => {
                    Error::InvalidNextTokenException(inner)
                }
                crate::error::DescribeScalingPlansErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::DescribeScalingPlansErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::GetScalingPlanResourceForecastDataError>>
    for Error
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::GetScalingPlanResourceForecastDataError>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::GetScalingPlanResourceForecastDataErrorKind::InternalServiceException(inner) => Error::InternalServiceException(inner),
                crate::error::GetScalingPlanResourceForecastDataErrorKind::ValidationException(inner) => Error::ValidationException(inner),
                crate::error::GetScalingPlanResourceForecastDataErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::UpdateScalingPlanError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::UpdateScalingPlanError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::UpdateScalingPlanErrorKind::ConcurrentUpdateException(inner) => {
                    Error::ConcurrentUpdateException(inner)
                }
                crate::error::UpdateScalingPlanErrorKind::InternalServiceException(inner) => {
                    Error::InternalServiceException(inner)
                }
                crate::error::UpdateScalingPlanErrorKind::ObjectNotFoundException(inner) => {
                    Error::ObjectNotFoundException(inner)
                }
                crate::error::UpdateScalingPlanErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::UpdateScalingPlanErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl std::error::Error for Error {}