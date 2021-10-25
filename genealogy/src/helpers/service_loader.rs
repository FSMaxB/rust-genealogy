use crate::helpers::exception::Exception;
use crate::helpers::exception::Exception::ServiceConfigurationError;
use crate::helpers::stream::Stream;
use lazy_static::lazy_static;
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::marker::PhantomData;
use std::sync::Mutex;

pub struct ServiceLoader<Service> {
	_service: PhantomData<Service>,
}

lazy_static! {
	static ref PROVIDERS: Mutex<Registry> = Default::default();
}

impl<Service> ServiceLoader<Service> {
	pub fn stream(&self) -> Result<Stream<Provider<Service>>, Exception>
	where
		Vec<Provider<Service>>: Clone,
		Service: 'static,
	{
		Ok(Stream::of(
			PROVIDERS
				.lock()
				.map_err(|_| ServiceConfigurationError("Poisoned mutex."))?
				.providers()?
				.clone(),
		))
	}

	pub fn register(service: Service)
	where
		Service: Send + Sync + 'static,
	{
		PROVIDERS.lock().unwrap().register(service);
	}

	pub fn register_many(services: impl IntoIterator<Item = Service>)
	where
		Service: Send + Sync + 'static,
	{
		for service in services {
			Self::register(service);
		}
	}

	pub fn load(_service: PhantomData<Service>) -> Self {
		Self { _service }
	}
}

pub trait Class {
	fn class() -> PhantomData<Self> {
		PhantomData::default()
	}
}

impl<T> Class for T {}

#[derive(Default)]
struct Registry {
	providers_by_type: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
}

impl Registry {
	fn providers<Service>(&mut self) -> Result<&mut Vec<Provider<Service>>, Exception>
	where
		Service: 'static,
	{
		let type_id = TypeId::of::<Service>();
		self.providers_by_type
			.get_mut(&type_id)
			.ok_or(ServiceConfigurationError("Failed to load service."))
			.map(|providers| providers.downcast_mut().unwrap())
	}

	fn register<Service>(&mut self, service: Service)
	where
		Service: Send + Sync + 'static,
	{
		let type_id = TypeId::of::<Service>();
		let providers = self
			.providers_by_type
			.entry(type_id)
			.or_insert_with(|| Box::new(Vec::<Provider<Service>>::new()))
			.downcast_mut::<Vec<Provider<Service>>>()
			.unwrap();
		providers.push(Provider { service });
	}
}

#[derive(Clone)]
pub struct Provider<Service> {
	service: Service,
}

impl<Service> Provider<Service> {
	pub fn get(self) -> Service {
		self.service
	}
}
