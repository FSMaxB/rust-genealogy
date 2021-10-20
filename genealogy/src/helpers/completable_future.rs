use crate::helpers::exception::Exception;
use futures_lite::future::block_on;
use pin_project::pin_project;
use std::convert::identity;
use std::future::{ready, Future};
use std::ops::FnOnce;
use std::pin::Pin;

#[pin_project]
pub struct CompletableFuture<Output> {
	#[pin]
	future: Pin<Box<dyn Future<Output = Result<Output, Exception>> + 'static>>,
}

impl<Output> CompletableFuture<Output>
where
	Output: 'static,
{
	pub fn supply_async(supplier: impl FnOnce() -> Result<Output, Exception> + 'static) -> Self {
		async move { supplier() }.into()
	}

	pub fn completed_future(output: Output) -> Self {
		ready(Ok(output)).into()
	}

	pub fn exceptionally_compose_async(
		self,
		function: impl FnOnce(Exception) -> Result<Self, Exception> + 'static,
	) -> Self {
		async move {
			match self.future.await {
				ok @ Ok(_) => ok,
				Err(exception) => match function(exception) {
					Ok(completable_future) => completable_future.future.await,
					Err(exception) => Err(exception),
				},
			}
		}
		.into()
	}

	pub fn exceptionally_compose(
		self,
		function: impl FnOnce(Exception) -> Result<Output, Exception> + 'static,
	) -> Self {
		async move {
			match self.future.await {
				ok @ Ok(_) => ok,
				Err(exception) => function(exception),
			}
		}
		.into()
	}

	pub fn then_apply<NewOutput: 'static>(
		self,
		function: impl FnOnce(Output) -> Result<NewOutput, Exception> + 'static,
	) -> CompletableFuture<NewOutput> {
		async move { self.future.await.map(function).and_then(identity) }.into()
	}

	pub fn join(self) -> Result<Output, Exception> {
		block_on(self.future)
	}
}

impl<FutureType, Output> From<FutureType> for CompletableFuture<Output>
where
	FutureType: Future<Output = Result<Output, Exception>> + 'static,
{
	fn from(future: FutureType) -> Self {
		Self {
			future: Box::pin(future),
		}
	}
}
