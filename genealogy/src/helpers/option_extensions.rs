use crate::helpers::exception::Exception;

pub trait OptionExtensions<Value> {
	fn or_else_throw<Thrower>(self, thrower: Thrower) -> Result<Value, Exception>
	where
		Thrower: FnOnce() -> Exception;
}

impl<Value> OptionExtensions<Value> for Option<Value> {
	fn or_else_throw<Thrower>(self, thrower: Thrower) -> Result<Value, Exception>
	where
		Thrower: FnOnce() -> Exception,
	{
		self.ok_or_else(thrower)
	}
}
