/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0.
 */

use crate::SendOperationError;
use smithy_http::body::SdkBody;
use smithy_http::operation;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tower::{BoxError, Layer, Service};
use tracing::trace;

/// Connects Operation driven middleware to an HTTP implementation.
///
/// It will also wrap the error type in OperationError to enable operation middleware
/// reporting specific errors
#[derive(Clone)]
pub struct DispatchService<S> {
    inner: S,
}

type BoxedResultFuture<T, E> = Pin<Box<dyn Future<Output = Result<T, E>> + Send>>;

impl<S> Service<operation::Request> for DispatchService<S>
where
    S: Service<http::Request<SdkBody>> + Clone + Send + 'static,
    S::Error: Into<BoxError>,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = SendOperationError;
    type Future = BoxedResultFuture<Self::Response, Self::Error>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner
            .poll_ready(cx)
            .map_err(|e| SendOperationError::RequestDispatchError(e.into()))
    }

    fn call(&mut self, req: operation::Request) -> Self::Future {
        let (req, _property_bag) = req.into_parts();
        let mut inner = self.inner.clone();
        let future = async move {
            trace!(request = ?req);
            inner
                .call(req)
                .await
                .map_err(|e| SendOperationError::RequestDispatchError(e.into()))
        };
        Box::pin(future)
    }
}

#[derive(Clone, Default)]
#[non_exhaustive]
pub struct DispatchLayer;

impl DispatchLayer {
    pub fn new() -> Self {
        DispatchLayer
    }
}

impl<S> Layer<S> for DispatchLayer
where
    S: Service<http::Request<SdkBody>>,
{
    type Service = DispatchService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        DispatchService { inner }
    }
}
