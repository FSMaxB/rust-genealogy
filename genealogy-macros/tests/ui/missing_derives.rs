use genealogy_macros::record;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;

#[record(equals = false, hash = false)]
struct Record {}

pub fn main() {
	let record = Record::new();
	assert_eq!(record, record);
	let mut hasher = DefaultHasher::new();
	Hash::hash(&record, &mut hasher);
}
