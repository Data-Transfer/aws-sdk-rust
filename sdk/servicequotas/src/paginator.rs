// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Paginator for [`ListAWSDefaultServiceQuotas`](crate::operation::ListAWSDefaultServiceQuotas)
pub struct ListAwsDefaultServiceQuotasPaginator<
    C = aws_smithy_client::erase::DynConnector,
    M = crate::middleware::DefaultMiddleware,
    R = aws_smithy_client::retry::Standard,
> {
    handle: std::sync::Arc<crate::client::Handle<C, M, R>>,
    builder: crate::input::list_aws_default_service_quotas_input::Builder,
}

impl<C, M, R> ListAwsDefaultServiceQuotasPaginator<C, M, R>
where
    C: aws_smithy_client::bounds::SmithyConnector,
    M: aws_smithy_client::bounds::SmithyMiddleware<C>,
    R: aws_smithy_client::retry::NewRequestPolicy,
{
    /// Create a new paginator-wrapper
    pub(crate) fn new(
        handle: std::sync::Arc<crate::client::Handle<C, M, R>>,
        builder: crate::input::list_aws_default_service_quotas_input::Builder,
    ) -> Self {
        Self { handle, builder }
    }

    /// Set the page size
    ///
    /// _Note: this method will override any previously set value for `max_results`_
    pub fn page_size(mut self, limit: i32) -> Self {
        self.builder.max_results = Some(limit);
        self
    }

    /// Create a flattened paginator
    ///
    /// This paginator automatically flattens results using `quotas`. Queries to the underlying service
    /// are dispatched lazily.
    pub fn items(self) -> crate::paginator::ListAwsDefaultServiceQuotasPaginatorItems<C, M, R> {
        crate::paginator::ListAwsDefaultServiceQuotasPaginatorItems(self)
    }

    /// Create the pagination stream
    ///
    /// _Note:_ No requests will be dispatched until the stream is used (eg. with [`.next().await`](tokio_stream::StreamExt::next)).
    pub fn send(
        self,
    ) -> impl tokio_stream::Stream<
        Item = std::result::Result<
            crate::output::ListAwsDefaultServiceQuotasOutput,
            aws_smithy_http::result::SdkError<crate::error::ListAWSDefaultServiceQuotasError>,
        >,
    > + Unpin
    where
        R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
            crate::input::ListAwsDefaultServiceQuotasInputOperationOutputAlias,
            crate::output::ListAwsDefaultServiceQuotasOutput,
            crate::error::ListAWSDefaultServiceQuotasError,
            crate::input::ListAwsDefaultServiceQuotasInputOperationRetryAlias,
        >,
    {
        // Move individual fields out of self for the borrow checker
        let builder = self.builder;
        let handle = self.handle;
        aws_smithy_async::future::fn_stream::FnStream::new(move |tx| {
            Box::pin(async move {
                // Build the input for the first time. If required fields are missing, this is where we'll produce an early error.
                let mut input = match builder.build().map_err(|err| {
                    aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                }) {
                    Ok(input) => input,
                    Err(e) => {
                        let _ = tx.send(Err(e)).await;
                        return;
                    }
                };
                loop {
                    let op = match input.make_operation(&handle.conf).await.map_err(|err| {
                        aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                    }) {
                        Ok(op) => op,
                        Err(e) => {
                            let _ = tx.send(Err(e)).await;
                            return;
                        }
                    };
                    let resp = handle.client.call(op).await;
                    // If the input member is None or it was an error
                    let done = match resp {
                        Ok(ref resp) => {
                            input.next_token = crate::lens::reflens_structure_crate_output_list_aws_default_service_quotas_output_next_token(resp).cloned();
                            input.next_token.is_none()
                        }
                        Err(_) => true,
                    };
                    if tx.send(resp).await.is_err() {
                        // receiving end was dropped
                        return;
                    }
                    if done {
                        return;
                    }
                }
            })
        })
    }
}

/// Paginator for [`ListRequestedServiceQuotaChangeHistory`](crate::operation::ListRequestedServiceQuotaChangeHistory)
pub struct ListRequestedServiceQuotaChangeHistoryPaginator<
    C = aws_smithy_client::erase::DynConnector,
    M = crate::middleware::DefaultMiddleware,
    R = aws_smithy_client::retry::Standard,
> {
    handle: std::sync::Arc<crate::client::Handle<C, M, R>>,
    builder: crate::input::list_requested_service_quota_change_history_input::Builder,
}

impl<C, M, R> ListRequestedServiceQuotaChangeHistoryPaginator<C, M, R>
where
    C: aws_smithy_client::bounds::SmithyConnector,
    M: aws_smithy_client::bounds::SmithyMiddleware<C>,
    R: aws_smithy_client::retry::NewRequestPolicy,
{
    /// Create a new paginator-wrapper
    pub(crate) fn new(
        handle: std::sync::Arc<crate::client::Handle<C, M, R>>,
        builder: crate::input::list_requested_service_quota_change_history_input::Builder,
    ) -> Self {
        Self { handle, builder }
    }

    /// Set the page size
    ///
    /// _Note: this method will override any previously set value for `max_results`_
    pub fn page_size(mut self, limit: i32) -> Self {
        self.builder.max_results = Some(limit);
        self
    }

    /// Create a flattened paginator
    ///
    /// This paginator automatically flattens results using `requested_quotas`. Queries to the underlying service
    /// are dispatched lazily.
    pub fn items(
        self,
    ) -> crate::paginator::ListRequestedServiceQuotaChangeHistoryPaginatorItems<C, M, R> {
        crate::paginator::ListRequestedServiceQuotaChangeHistoryPaginatorItems(self)
    }

    /// Create the pagination stream
    ///
    /// _Note:_ No requests will be dispatched until the stream is used (eg. with [`.next().await`](tokio_stream::StreamExt::next)).
    pub fn send(
        self,
    ) -> impl tokio_stream::Stream<
        Item = std::result::Result<
            crate::output::ListRequestedServiceQuotaChangeHistoryOutput,
            aws_smithy_http::result::SdkError<
                crate::error::ListRequestedServiceQuotaChangeHistoryError,
            >,
        >,
    > + Unpin
    where
        R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
            crate::input::ListRequestedServiceQuotaChangeHistoryInputOperationOutputAlias,
            crate::output::ListRequestedServiceQuotaChangeHistoryOutput,
            crate::error::ListRequestedServiceQuotaChangeHistoryError,
            crate::input::ListRequestedServiceQuotaChangeHistoryInputOperationRetryAlias,
        >,
    {
        // Move individual fields out of self for the borrow checker
        let builder = self.builder;
        let handle = self.handle;
        aws_smithy_async::future::fn_stream::FnStream::new(move |tx| {
            Box::pin(async move {
                // Build the input for the first time. If required fields are missing, this is where we'll produce an early error.
                let mut input = match builder.build().map_err(|err| {
                    aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                }) {
                    Ok(input) => input,
                    Err(e) => {
                        let _ = tx.send(Err(e)).await;
                        return;
                    }
                };
                loop {
                    let op = match input.make_operation(&handle.conf).await.map_err(|err| {
                        aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                    }) {
                        Ok(op) => op,
                        Err(e) => {
                            let _ = tx.send(Err(e)).await;
                            return;
                        }
                    };
                    let resp = handle.client.call(op).await;
                    // If the input member is None or it was an error
                    let done = match resp {
                        Ok(ref resp) => {
                            input.next_token = crate::lens::reflens_structure_crate_output_list_requested_service_quota_change_history_output_next_token(resp).cloned();
                            input.next_token.is_none()
                        }
                        Err(_) => true,
                    };
                    if tx.send(resp).await.is_err() {
                        // receiving end was dropped
                        return;
                    }
                    if done {
                        return;
                    }
                }
            })
        })
    }
}

/// Paginator for [`ListRequestedServiceQuotaChangeHistoryByQuota`](crate::operation::ListRequestedServiceQuotaChangeHistoryByQuota)
pub struct ListRequestedServiceQuotaChangeHistoryByQuotaPaginator<
    C = aws_smithy_client::erase::DynConnector,
    M = crate::middleware::DefaultMiddleware,
    R = aws_smithy_client::retry::Standard,
> {
    handle: std::sync::Arc<crate::client::Handle<C, M, R>>,
    builder: crate::input::list_requested_service_quota_change_history_by_quota_input::Builder,
}

impl<C, M, R> ListRequestedServiceQuotaChangeHistoryByQuotaPaginator<C, M, R>
where
    C: aws_smithy_client::bounds::SmithyConnector,
    M: aws_smithy_client::bounds::SmithyMiddleware<C>,
    R: aws_smithy_client::retry::NewRequestPolicy,
{
    /// Create a new paginator-wrapper
    pub(crate) fn new(
        handle: std::sync::Arc<crate::client::Handle<C, M, R>>,
        builder: crate::input::list_requested_service_quota_change_history_by_quota_input::Builder,
    ) -> Self {
        Self { handle, builder }
    }

    /// Set the page size
    ///
    /// _Note: this method will override any previously set value for `max_results`_
    pub fn page_size(mut self, limit: i32) -> Self {
        self.builder.max_results = Some(limit);
        self
    }

    /// Create a flattened paginator
    ///
    /// This paginator automatically flattens results using `requested_quotas`. Queries to the underlying service
    /// are dispatched lazily.
    pub fn items(
        self,
    ) -> crate::paginator::ListRequestedServiceQuotaChangeHistoryByQuotaPaginatorItems<C, M, R>
    {
        crate::paginator::ListRequestedServiceQuotaChangeHistoryByQuotaPaginatorItems(self)
    }

    /// Create the pagination stream
    ///
    /// _Note:_ No requests will be dispatched until the stream is used (eg. with [`.next().await`](tokio_stream::StreamExt::next)).
    pub fn send(
        self,
    ) -> impl tokio_stream::Stream<
        Item = std::result::Result<
            crate::output::ListRequestedServiceQuotaChangeHistoryByQuotaOutput,
            aws_smithy_http::result::SdkError<
                crate::error::ListRequestedServiceQuotaChangeHistoryByQuotaError,
            >,
        >,
    > + Unpin
    where
        R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
            crate::input::ListRequestedServiceQuotaChangeHistoryByQuotaInputOperationOutputAlias,
            crate::output::ListRequestedServiceQuotaChangeHistoryByQuotaOutput,
            crate::error::ListRequestedServiceQuotaChangeHistoryByQuotaError,
            crate::input::ListRequestedServiceQuotaChangeHistoryByQuotaInputOperationRetryAlias,
        >,
    {
        // Move individual fields out of self for the borrow checker
        let builder = self.builder;
        let handle = self.handle;
        aws_smithy_async::future::fn_stream::FnStream::new(move |tx| {
            Box::pin(async move {
                // Build the input for the first time. If required fields are missing, this is where we'll produce an early error.
                let mut input = match builder.build().map_err(|err| {
                    aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                }) {
                    Ok(input) => input,
                    Err(e) => {
                        let _ = tx.send(Err(e)).await;
                        return;
                    }
                };
                loop {
                    let op = match input.make_operation(&handle.conf).await.map_err(|err| {
                        aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                    }) {
                        Ok(op) => op,
                        Err(e) => {
                            let _ = tx.send(Err(e)).await;
                            return;
                        }
                    };
                    let resp = handle.client.call(op).await;
                    // If the input member is None or it was an error
                    let done = match resp {
                        Ok(ref resp) => {
                            input.next_token = crate::lens::reflens_structure_crate_output_list_requested_service_quota_change_history_by_quota_output_next_token(resp).cloned();
                            input.next_token.is_none()
                        }
                        Err(_) => true,
                    };
                    if tx.send(resp).await.is_err() {
                        // receiving end was dropped
                        return;
                    }
                    if done {
                        return;
                    }
                }
            })
        })
    }
}

/// Paginator for [`ListServiceQuotaIncreaseRequestsInTemplate`](crate::operation::ListServiceQuotaIncreaseRequestsInTemplate)
pub struct ListServiceQuotaIncreaseRequestsInTemplatePaginator<
    C = aws_smithy_client::erase::DynConnector,
    M = crate::middleware::DefaultMiddleware,
    R = aws_smithy_client::retry::Standard,
> {
    handle: std::sync::Arc<crate::client::Handle<C, M, R>>,
    builder: crate::input::list_service_quota_increase_requests_in_template_input::Builder,
}

impl<C, M, R> ListServiceQuotaIncreaseRequestsInTemplatePaginator<C, M, R>
where
    C: aws_smithy_client::bounds::SmithyConnector,
    M: aws_smithy_client::bounds::SmithyMiddleware<C>,
    R: aws_smithy_client::retry::NewRequestPolicy,
{
    /// Create a new paginator-wrapper
    pub(crate) fn new(
        handle: std::sync::Arc<crate::client::Handle<C, M, R>>,
        builder: crate::input::list_service_quota_increase_requests_in_template_input::Builder,
    ) -> Self {
        Self { handle, builder }
    }

    /// Set the page size
    ///
    /// _Note: this method will override any previously set value for `max_results`_
    pub fn page_size(mut self, limit: i32) -> Self {
        self.builder.max_results = Some(limit);
        self
    }

    /// Create a flattened paginator
    ///
    /// This paginator automatically flattens results using `service_quota_increase_request_in_template_list`. Queries to the underlying service
    /// are dispatched lazily.
    pub fn items(
        self,
    ) -> crate::paginator::ListServiceQuotaIncreaseRequestsInTemplatePaginatorItems<C, M, R> {
        crate::paginator::ListServiceQuotaIncreaseRequestsInTemplatePaginatorItems(self)
    }

    /// Create the pagination stream
    ///
    /// _Note:_ No requests will be dispatched until the stream is used (eg. with [`.next().await`](tokio_stream::StreamExt::next)).
    pub fn send(
        self,
    ) -> impl tokio_stream::Stream<
        Item = std::result::Result<
            crate::output::ListServiceQuotaIncreaseRequestsInTemplateOutput,
            aws_smithy_http::result::SdkError<
                crate::error::ListServiceQuotaIncreaseRequestsInTemplateError,
            >,
        >,
    > + Unpin
    where
        R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
            crate::input::ListServiceQuotaIncreaseRequestsInTemplateInputOperationOutputAlias,
            crate::output::ListServiceQuotaIncreaseRequestsInTemplateOutput,
            crate::error::ListServiceQuotaIncreaseRequestsInTemplateError,
            crate::input::ListServiceQuotaIncreaseRequestsInTemplateInputOperationRetryAlias,
        >,
    {
        // Move individual fields out of self for the borrow checker
        let builder = self.builder;
        let handle = self.handle;
        aws_smithy_async::future::fn_stream::FnStream::new(move |tx| {
            Box::pin(async move {
                // Build the input for the first time. If required fields are missing, this is where we'll produce an early error.
                let mut input = match builder.build().map_err(|err| {
                    aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                }) {
                    Ok(input) => input,
                    Err(e) => {
                        let _ = tx.send(Err(e)).await;
                        return;
                    }
                };
                loop {
                    let op = match input.make_operation(&handle.conf).await.map_err(|err| {
                        aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                    }) {
                        Ok(op) => op,
                        Err(e) => {
                            let _ = tx.send(Err(e)).await;
                            return;
                        }
                    };
                    let resp = handle.client.call(op).await;
                    // If the input member is None or it was an error
                    let done = match resp {
                        Ok(ref resp) => {
                            input.next_token = crate::lens::reflens_structure_crate_output_list_service_quota_increase_requests_in_template_output_next_token(resp).cloned();
                            input.next_token.is_none()
                        }
                        Err(_) => true,
                    };
                    if tx.send(resp).await.is_err() {
                        // receiving end was dropped
                        return;
                    }
                    if done {
                        return;
                    }
                }
            })
        })
    }
}

/// Paginator for [`ListServiceQuotas`](crate::operation::ListServiceQuotas)
pub struct ListServiceQuotasPaginator<
    C = aws_smithy_client::erase::DynConnector,
    M = crate::middleware::DefaultMiddleware,
    R = aws_smithy_client::retry::Standard,
> {
    handle: std::sync::Arc<crate::client::Handle<C, M, R>>,
    builder: crate::input::list_service_quotas_input::Builder,
}

impl<C, M, R> ListServiceQuotasPaginator<C, M, R>
where
    C: aws_smithy_client::bounds::SmithyConnector,
    M: aws_smithy_client::bounds::SmithyMiddleware<C>,
    R: aws_smithy_client::retry::NewRequestPolicy,
{
    /// Create a new paginator-wrapper
    pub(crate) fn new(
        handle: std::sync::Arc<crate::client::Handle<C, M, R>>,
        builder: crate::input::list_service_quotas_input::Builder,
    ) -> Self {
        Self { handle, builder }
    }

    /// Set the page size
    ///
    /// _Note: this method will override any previously set value for `max_results`_
    pub fn page_size(mut self, limit: i32) -> Self {
        self.builder.max_results = Some(limit);
        self
    }

    /// Create a flattened paginator
    ///
    /// This paginator automatically flattens results using `quotas`. Queries to the underlying service
    /// are dispatched lazily.
    pub fn items(self) -> crate::paginator::ListServiceQuotasPaginatorItems<C, M, R> {
        crate::paginator::ListServiceQuotasPaginatorItems(self)
    }

    /// Create the pagination stream
    ///
    /// _Note:_ No requests will be dispatched until the stream is used (eg. with [`.next().await`](tokio_stream::StreamExt::next)).
    pub fn send(
        self,
    ) -> impl tokio_stream::Stream<
        Item = std::result::Result<
            crate::output::ListServiceQuotasOutput,
            aws_smithy_http::result::SdkError<crate::error::ListServiceQuotasError>,
        >,
    > + Unpin
    where
        R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
            crate::input::ListServiceQuotasInputOperationOutputAlias,
            crate::output::ListServiceQuotasOutput,
            crate::error::ListServiceQuotasError,
            crate::input::ListServiceQuotasInputOperationRetryAlias,
        >,
    {
        // Move individual fields out of self for the borrow checker
        let builder = self.builder;
        let handle = self.handle;
        aws_smithy_async::future::fn_stream::FnStream::new(move |tx| {
            Box::pin(async move {
                // Build the input for the first time. If required fields are missing, this is where we'll produce an early error.
                let mut input = match builder.build().map_err(|err| {
                    aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                }) {
                    Ok(input) => input,
                    Err(e) => {
                        let _ = tx.send(Err(e)).await;
                        return;
                    }
                };
                loop {
                    let op = match input.make_operation(&handle.conf).await.map_err(|err| {
                        aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                    }) {
                        Ok(op) => op,
                        Err(e) => {
                            let _ = tx.send(Err(e)).await;
                            return;
                        }
                    };
                    let resp = handle.client.call(op).await;
                    // If the input member is None or it was an error
                    let done = match resp {
                        Ok(ref resp) => {
                            input.next_token = crate::lens::reflens_structure_crate_output_list_service_quotas_output_next_token(resp).cloned();
                            input.next_token.is_none()
                        }
                        Err(_) => true,
                    };
                    if tx.send(resp).await.is_err() {
                        // receiving end was dropped
                        return;
                    }
                    if done {
                        return;
                    }
                }
            })
        })
    }
}

/// Paginator for [`ListServices`](crate::operation::ListServices)
pub struct ListServicesPaginator<
    C = aws_smithy_client::erase::DynConnector,
    M = crate::middleware::DefaultMiddleware,
    R = aws_smithy_client::retry::Standard,
> {
    handle: std::sync::Arc<crate::client::Handle<C, M, R>>,
    builder: crate::input::list_services_input::Builder,
}

impl<C, M, R> ListServicesPaginator<C, M, R>
where
    C: aws_smithy_client::bounds::SmithyConnector,
    M: aws_smithy_client::bounds::SmithyMiddleware<C>,
    R: aws_smithy_client::retry::NewRequestPolicy,
{
    /// Create a new paginator-wrapper
    pub(crate) fn new(
        handle: std::sync::Arc<crate::client::Handle<C, M, R>>,
        builder: crate::input::list_services_input::Builder,
    ) -> Self {
        Self { handle, builder }
    }

    /// Set the page size
    ///
    /// _Note: this method will override any previously set value for `max_results`_
    pub fn page_size(mut self, limit: i32) -> Self {
        self.builder.max_results = Some(limit);
        self
    }

    /// Create a flattened paginator
    ///
    /// This paginator automatically flattens results using `services`. Queries to the underlying service
    /// are dispatched lazily.
    pub fn items(self) -> crate::paginator::ListServicesPaginatorItems<C, M, R> {
        crate::paginator::ListServicesPaginatorItems(self)
    }

    /// Create the pagination stream
    ///
    /// _Note:_ No requests will be dispatched until the stream is used (eg. with [`.next().await`](tokio_stream::StreamExt::next)).
    pub fn send(
        self,
    ) -> impl tokio_stream::Stream<
        Item = std::result::Result<
            crate::output::ListServicesOutput,
            aws_smithy_http::result::SdkError<crate::error::ListServicesError>,
        >,
    > + Unpin
    where
        R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
            crate::input::ListServicesInputOperationOutputAlias,
            crate::output::ListServicesOutput,
            crate::error::ListServicesError,
            crate::input::ListServicesInputOperationRetryAlias,
        >,
    {
        // Move individual fields out of self for the borrow checker
        let builder = self.builder;
        let handle = self.handle;
        aws_smithy_async::future::fn_stream::FnStream::new(move |tx| {
            Box::pin(async move {
                // Build the input for the first time. If required fields are missing, this is where we'll produce an early error.
                let mut input = match builder.build().map_err(|err| {
                    aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                }) {
                    Ok(input) => input,
                    Err(e) => {
                        let _ = tx.send(Err(e)).await;
                        return;
                    }
                };
                loop {
                    let op = match input.make_operation(&handle.conf).await.map_err(|err| {
                        aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                    }) {
                        Ok(op) => op,
                        Err(e) => {
                            let _ = tx.send(Err(e)).await;
                            return;
                        }
                    };
                    let resp = handle.client.call(op).await;
                    // If the input member is None or it was an error
                    let done = match resp {
                        Ok(ref resp) => {
                            input.next_token = crate::lens::reflens_structure_crate_output_list_services_output_next_token(resp).cloned();
                            input.next_token.is_none()
                        }
                        Err(_) => true,
                    };
                    if tx.send(resp).await.is_err() {
                        // receiving end was dropped
                        return;
                    }
                    if done {
                        return;
                    }
                }
            })
        })
    }
}

/// Flattened paginator for `ListAwsDefaultServiceQuotasPaginator`
///
/// This is created with [`.items()`](ListAwsDefaultServiceQuotasPaginator::items)
pub struct ListAwsDefaultServiceQuotasPaginatorItems<
    C = aws_smithy_client::erase::DynConnector,
    M = crate::middleware::DefaultMiddleware,
    R = aws_smithy_client::retry::Standard,
>(ListAwsDefaultServiceQuotasPaginator<C, M, R>);

impl<C, M, R> ListAwsDefaultServiceQuotasPaginatorItems<C, M, R>
where
    C: aws_smithy_client::bounds::SmithyConnector,
    M: aws_smithy_client::bounds::SmithyMiddleware<C>,
    R: aws_smithy_client::retry::NewRequestPolicy,
{
    /// Create the pagination stream
    ///
    /// _Note: No requests will be dispatched until the stream is used (eg. with [`.next().await`](tokio_stream::StreamExt::next))._
    ///
    /// To read the entirety of the paginator, use [`.collect::<Result<Vec<_>, _>()`](tokio_stream::StreamExt::collect).
    pub fn send(
        self,
    ) -> impl tokio_stream::Stream<
        Item = std::result::Result<
            crate::model::ServiceQuota,
            aws_smithy_http::result::SdkError<crate::error::ListAWSDefaultServiceQuotasError>,
        >,
    > + Unpin
    where
        R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
            crate::input::ListAwsDefaultServiceQuotasInputOperationOutputAlias,
            crate::output::ListAwsDefaultServiceQuotasOutput,
            crate::error::ListAWSDefaultServiceQuotasError,
            crate::input::ListAwsDefaultServiceQuotasInputOperationRetryAlias,
        >,
    {
        aws_smithy_async::future::fn_stream::TryFlatMap::new(self.0.send()).flat_map(|page| {
            crate::lens::lens_structure_crate_output_list_aws_default_service_quotas_output_quotas(
                page,
            )
            .unwrap_or_default()
            .into_iter()
        })
    }
}

/// Flattened paginator for `ListRequestedServiceQuotaChangeHistoryPaginator`
///
/// This is created with [`.items()`](ListRequestedServiceQuotaChangeHistoryPaginator::items)
pub struct ListRequestedServiceQuotaChangeHistoryPaginatorItems<
    C = aws_smithy_client::erase::DynConnector,
    M = crate::middleware::DefaultMiddleware,
    R = aws_smithy_client::retry::Standard,
>(ListRequestedServiceQuotaChangeHistoryPaginator<C, M, R>);

impl<C, M, R> ListRequestedServiceQuotaChangeHistoryPaginatorItems<C, M, R>
where
    C: aws_smithy_client::bounds::SmithyConnector,
    M: aws_smithy_client::bounds::SmithyMiddleware<C>,
    R: aws_smithy_client::retry::NewRequestPolicy,
{
    /// Create the pagination stream
    ///
    /// _Note: No requests will be dispatched until the stream is used (eg. with [`.next().await`](tokio_stream::StreamExt::next))._
    ///
    /// To read the entirety of the paginator, use [`.collect::<Result<Vec<_>, _>()`](tokio_stream::StreamExt::collect).
    pub fn send(
        self,
    ) -> impl tokio_stream::Stream<
        Item = std::result::Result<
            crate::model::RequestedServiceQuotaChange,
            aws_smithy_http::result::SdkError<
                crate::error::ListRequestedServiceQuotaChangeHistoryError,
            >,
        >,
    > + Unpin
    where
        R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
            crate::input::ListRequestedServiceQuotaChangeHistoryInputOperationOutputAlias,
            crate::output::ListRequestedServiceQuotaChangeHistoryOutput,
            crate::error::ListRequestedServiceQuotaChangeHistoryError,
            crate::input::ListRequestedServiceQuotaChangeHistoryInputOperationRetryAlias,
        >,
    {
        aws_smithy_async::future::fn_stream::TryFlatMap::new(self.0.send()).flat_map(|page| crate::lens::lens_structure_crate_output_list_requested_service_quota_change_history_output_requested_quotas(page).unwrap_or_default().into_iter())
    }
}

/// Flattened paginator for `ListRequestedServiceQuotaChangeHistoryByQuotaPaginator`
///
/// This is created with [`.items()`](ListRequestedServiceQuotaChangeHistoryByQuotaPaginator::items)
pub struct ListRequestedServiceQuotaChangeHistoryByQuotaPaginatorItems<
    C = aws_smithy_client::erase::DynConnector,
    M = crate::middleware::DefaultMiddleware,
    R = aws_smithy_client::retry::Standard,
>(ListRequestedServiceQuotaChangeHistoryByQuotaPaginator<C, M, R>);

impl<C, M, R> ListRequestedServiceQuotaChangeHistoryByQuotaPaginatorItems<C, M, R>
where
    C: aws_smithy_client::bounds::SmithyConnector,
    M: aws_smithy_client::bounds::SmithyMiddleware<C>,
    R: aws_smithy_client::retry::NewRequestPolicy,
{
    /// Create the pagination stream
    ///
    /// _Note: No requests will be dispatched until the stream is used (eg. with [`.next().await`](tokio_stream::StreamExt::next))._
    ///
    /// To read the entirety of the paginator, use [`.collect::<Result<Vec<_>, _>()`](tokio_stream::StreamExt::collect).
    pub fn send(
        self,
    ) -> impl tokio_stream::Stream<
        Item = std::result::Result<
            crate::model::RequestedServiceQuotaChange,
            aws_smithy_http::result::SdkError<
                crate::error::ListRequestedServiceQuotaChangeHistoryByQuotaError,
            >,
        >,
    > + Unpin
    where
        R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
            crate::input::ListRequestedServiceQuotaChangeHistoryByQuotaInputOperationOutputAlias,
            crate::output::ListRequestedServiceQuotaChangeHistoryByQuotaOutput,
            crate::error::ListRequestedServiceQuotaChangeHistoryByQuotaError,
            crate::input::ListRequestedServiceQuotaChangeHistoryByQuotaInputOperationRetryAlias,
        >,
    {
        aws_smithy_async::future::fn_stream::TryFlatMap::new(self.0.send()).flat_map(|page| crate::lens::lens_structure_crate_output_list_requested_service_quota_change_history_by_quota_output_requested_quotas(page).unwrap_or_default().into_iter())
    }
}

/// Flattened paginator for `ListServiceQuotaIncreaseRequestsInTemplatePaginator`
///
/// This is created with [`.items()`](ListServiceQuotaIncreaseRequestsInTemplatePaginator::items)
pub struct ListServiceQuotaIncreaseRequestsInTemplatePaginatorItems<
    C = aws_smithy_client::erase::DynConnector,
    M = crate::middleware::DefaultMiddleware,
    R = aws_smithy_client::retry::Standard,
>(ListServiceQuotaIncreaseRequestsInTemplatePaginator<C, M, R>);

impl<C, M, R> ListServiceQuotaIncreaseRequestsInTemplatePaginatorItems<C, M, R>
where
    C: aws_smithy_client::bounds::SmithyConnector,
    M: aws_smithy_client::bounds::SmithyMiddleware<C>,
    R: aws_smithy_client::retry::NewRequestPolicy,
{
    /// Create the pagination stream
    ///
    /// _Note: No requests will be dispatched until the stream is used (eg. with [`.next().await`](tokio_stream::StreamExt::next))._
    ///
    /// To read the entirety of the paginator, use [`.collect::<Result<Vec<_>, _>()`](tokio_stream::StreamExt::collect).
    pub fn send(
        self,
    ) -> impl tokio_stream::Stream<
        Item = std::result::Result<
            crate::model::ServiceQuotaIncreaseRequestInTemplate,
            aws_smithy_http::result::SdkError<
                crate::error::ListServiceQuotaIncreaseRequestsInTemplateError,
            >,
        >,
    > + Unpin
    where
        R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
            crate::input::ListServiceQuotaIncreaseRequestsInTemplateInputOperationOutputAlias,
            crate::output::ListServiceQuotaIncreaseRequestsInTemplateOutput,
            crate::error::ListServiceQuotaIncreaseRequestsInTemplateError,
            crate::input::ListServiceQuotaIncreaseRequestsInTemplateInputOperationRetryAlias,
        >,
    {
        aws_smithy_async::future::fn_stream::TryFlatMap::new(self.0.send()).flat_map(|page| crate::lens::lens_structure_crate_output_list_service_quota_increase_requests_in_template_output_service_quota_increase_request_in_template_list(page).unwrap_or_default().into_iter())
    }
}

/// Flattened paginator for `ListServiceQuotasPaginator`
///
/// This is created with [`.items()`](ListServiceQuotasPaginator::items)
pub struct ListServiceQuotasPaginatorItems<
    C = aws_smithy_client::erase::DynConnector,
    M = crate::middleware::DefaultMiddleware,
    R = aws_smithy_client::retry::Standard,
>(ListServiceQuotasPaginator<C, M, R>);

impl<C, M, R> ListServiceQuotasPaginatorItems<C, M, R>
where
    C: aws_smithy_client::bounds::SmithyConnector,
    M: aws_smithy_client::bounds::SmithyMiddleware<C>,
    R: aws_smithy_client::retry::NewRequestPolicy,
{
    /// Create the pagination stream
    ///
    /// _Note: No requests will be dispatched until the stream is used (eg. with [`.next().await`](tokio_stream::StreamExt::next))._
    ///
    /// To read the entirety of the paginator, use [`.collect::<Result<Vec<_>, _>()`](tokio_stream::StreamExt::collect).
    pub fn send(
        self,
    ) -> impl tokio_stream::Stream<
        Item = std::result::Result<
            crate::model::ServiceQuota,
            aws_smithy_http::result::SdkError<crate::error::ListServiceQuotasError>,
        >,
    > + Unpin
    where
        R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
            crate::input::ListServiceQuotasInputOperationOutputAlias,
            crate::output::ListServiceQuotasOutput,
            crate::error::ListServiceQuotasError,
            crate::input::ListServiceQuotasInputOperationRetryAlias,
        >,
    {
        aws_smithy_async::future::fn_stream::TryFlatMap::new(self.0.send()).flat_map(|page| {
            crate::lens::lens_structure_crate_output_list_service_quotas_output_quotas(page)
                .unwrap_or_default()
                .into_iter()
        })
    }
}

/// Flattened paginator for `ListServicesPaginator`
///
/// This is created with [`.items()`](ListServicesPaginator::items)
pub struct ListServicesPaginatorItems<
    C = aws_smithy_client::erase::DynConnector,
    M = crate::middleware::DefaultMiddleware,
    R = aws_smithy_client::retry::Standard,
>(ListServicesPaginator<C, M, R>);

impl<C, M, R> ListServicesPaginatorItems<C, M, R>
where
    C: aws_smithy_client::bounds::SmithyConnector,
    M: aws_smithy_client::bounds::SmithyMiddleware<C>,
    R: aws_smithy_client::retry::NewRequestPolicy,
{
    /// Create the pagination stream
    ///
    /// _Note: No requests will be dispatched until the stream is used (eg. with [`.next().await`](tokio_stream::StreamExt::next))._
    ///
    /// To read the entirety of the paginator, use [`.collect::<Result<Vec<_>, _>()`](tokio_stream::StreamExt::collect).
    pub fn send(
        self,
    ) -> impl tokio_stream::Stream<
        Item = std::result::Result<
            crate::model::ServiceInfo,
            aws_smithy_http::result::SdkError<crate::error::ListServicesError>,
        >,
    > + Unpin
    where
        R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
            crate::input::ListServicesInputOperationOutputAlias,
            crate::output::ListServicesOutput,
            crate::error::ListServicesError,
            crate::input::ListServicesInputOperationRetryAlias,
        >,
    {
        aws_smithy_async::future::fn_stream::TryFlatMap::new(self.0.send()).flat_map(|page| {
            crate::lens::lens_structure_crate_output_list_services_output_services(page)
                .unwrap_or_default()
                .into_iter()
        })
    }
}