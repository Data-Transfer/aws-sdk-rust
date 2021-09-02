// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn parse_create_endpoint_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::CreateEndpointOutput, crate::error::CreateEndpointError> {
    let generic = crate::json_deser::parse_http_generic_error(response)
        .map_err(crate::error::CreateEndpointError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::error::CreateEndpointError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => crate::error::CreateEndpointError {
            meta: generic,
            kind: crate::error::CreateEndpointErrorKind::AccessDeniedException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::access_denied_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_access_denied_exceptionjson_err(
                        response.body().as_ref(),
                        output,
                    )
                    .map_err(crate::error::CreateEndpointError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "ConflictException" => crate::error::CreateEndpointError {
            meta: generic,
            kind: crate::error::CreateEndpointErrorKind::ConflictException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::conflict_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_conflict_exceptionjson_err(
                        response.body().as_ref(),
                        output,
                    )
                    .map_err(crate::error::CreateEndpointError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "InternalServerException" => crate::error::CreateEndpointError {
            meta: generic,
            kind: crate::error::CreateEndpointErrorKind::InternalServerException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_server_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_internal_server_exceptionjson_err(
                        response.body().as_ref(),
                        output,
                    )
                    .map_err(crate::error::CreateEndpointError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "ResourceNotFoundException" => {
            crate::error::CreateEndpointError {
                meta: generic,
                kind: crate::error::CreateEndpointErrorKind::ResourceNotFoundException({
                    #[allow(unused_mut)]
                    let mut tmp = {
                        #[allow(unused_mut)]
                        let mut output =
                            crate::error::resource_not_found_exception::Builder::default();
                        let _ = response;
                        output = crate::json_deser::deser_structure_resource_not_found_exceptionjson_err(response.body().as_ref(), output).map_err(crate::error::CreateEndpointError::unhandled)?;
                        output.build()
                    };
                    if (&tmp.message).is_none() {
                        tmp.message = _error_message;
                    }
                    tmp
                }),
            }
        }
        "ValidationException" => crate::error::CreateEndpointError {
            meta: generic,
            kind: crate::error::CreateEndpointErrorKind::ValidationException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::validation_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_validation_exceptionjson_err(
                        response.body().as_ref(),
                        output,
                    )
                    .map_err(crate::error::CreateEndpointError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        _ => crate::error::CreateEndpointError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_create_endpoint_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::CreateEndpointOutput, crate::error::CreateEndpointError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::create_endpoint_output::Builder::default();
        let _ = response;
        output =
            crate::json_deser::deser_operation_create_endpoint(response.body().as_ref(), output)
                .map_err(crate::error::CreateEndpointError::unhandled)?;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_delete_endpoint_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::DeleteEndpointOutput, crate::error::DeleteEndpointError> {
    let generic = crate::json_deser::parse_http_generic_error(response)
        .map_err(crate::error::DeleteEndpointError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::error::DeleteEndpointError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => crate::error::DeleteEndpointError {
            meta: generic,
            kind: crate::error::DeleteEndpointErrorKind::AccessDeniedException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::access_denied_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_access_denied_exceptionjson_err(
                        response.body().as_ref(),
                        output,
                    )
                    .map_err(crate::error::DeleteEndpointError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "InternalServerException" => crate::error::DeleteEndpointError {
            meta: generic,
            kind: crate::error::DeleteEndpointErrorKind::InternalServerException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_server_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_internal_server_exceptionjson_err(
                        response.body().as_ref(),
                        output,
                    )
                    .map_err(crate::error::DeleteEndpointError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "ResourceNotFoundException" => {
            crate::error::DeleteEndpointError {
                meta: generic,
                kind: crate::error::DeleteEndpointErrorKind::ResourceNotFoundException({
                    #[allow(unused_mut)]
                    let mut tmp = {
                        #[allow(unused_mut)]
                        let mut output =
                            crate::error::resource_not_found_exception::Builder::default();
                        let _ = response;
                        output = crate::json_deser::deser_structure_resource_not_found_exceptionjson_err(response.body().as_ref(), output).map_err(crate::error::DeleteEndpointError::unhandled)?;
                        output.build()
                    };
                    if (&tmp.message).is_none() {
                        tmp.message = _error_message;
                    }
                    tmp
                }),
            }
        }
        "ValidationException" => crate::error::DeleteEndpointError {
            meta: generic,
            kind: crate::error::DeleteEndpointErrorKind::ValidationException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::validation_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_validation_exceptionjson_err(
                        response.body().as_ref(),
                        output,
                    )
                    .map_err(crate::error::DeleteEndpointError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        _ => crate::error::DeleteEndpointError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_delete_endpoint_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::DeleteEndpointOutput, crate::error::DeleteEndpointError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::delete_endpoint_output::Builder::default();
        let _ = response;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_list_endpoints_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::ListEndpointsOutput, crate::error::ListEndpointsError> {
    let generic = crate::json_deser::parse_http_generic_error(response)
        .map_err(crate::error::ListEndpointsError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::error::ListEndpointsError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => crate::error::ListEndpointsError {
            meta: generic,
            kind: crate::error::ListEndpointsErrorKind::AccessDeniedException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::access_denied_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_access_denied_exceptionjson_err(
                        response.body().as_ref(),
                        output,
                    )
                    .map_err(crate::error::ListEndpointsError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "InternalServerException" => crate::error::ListEndpointsError {
            meta: generic,
            kind: crate::error::ListEndpointsErrorKind::InternalServerException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_server_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_internal_server_exceptionjson_err(
                        response.body().as_ref(),
                        output,
                    )
                    .map_err(crate::error::ListEndpointsError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "ResourceNotFoundException" => {
            crate::error::ListEndpointsError {
                meta: generic,
                kind: crate::error::ListEndpointsErrorKind::ResourceNotFoundException({
                    #[allow(unused_mut)]
                    let mut tmp = {
                        #[allow(unused_mut)]
                        let mut output =
                            crate::error::resource_not_found_exception::Builder::default();
                        let _ = response;
                        output = crate::json_deser::deser_structure_resource_not_found_exceptionjson_err(response.body().as_ref(), output).map_err(crate::error::ListEndpointsError::unhandled)?;
                        output.build()
                    };
                    if (&tmp.message).is_none() {
                        tmp.message = _error_message;
                    }
                    tmp
                }),
            }
        }
        "ValidationException" => crate::error::ListEndpointsError {
            meta: generic,
            kind: crate::error::ListEndpointsErrorKind::ValidationException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::validation_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_validation_exceptionjson_err(
                        response.body().as_ref(),
                        output,
                    )
                    .map_err(crate::error::ListEndpointsError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        _ => crate::error::ListEndpointsError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_list_endpoints_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::ListEndpointsOutput, crate::error::ListEndpointsError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::list_endpoints_output::Builder::default();
        let _ = response;
        output =
            crate::json_deser::deser_operation_list_endpoints(response.body().as_ref(), output)
                .map_err(crate::error::ListEndpointsError::unhandled)?;
        output.build()
    })
}