use crate::exception::Exception;
use std::ops::Index;
use std::panic::{catch_unwind, RefUnwindSafe};

/// Simulate the behavior of an indexing operation in Java by throwing and IndexOutOfBounds exception if
/// indexing fails.
pub fn index<Collection>(collection: &Collection, index: usize) -> Result<&Collection::Output, Exception>
where
	Collection: Index<usize> + RefUnwindSafe,
	Collection::Output: Sized,
{
	catch_unwind(|| collection.index(index)).map_err(|_| Exception::IndexOutOfBoundsException(index))
}
