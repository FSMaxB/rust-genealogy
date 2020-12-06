use parking_lot::Mutex;
use std::collections::VecDeque;
use std::sync::Arc;

pub fn split_pair<ElementLeft, ElementRight, Iter>(
	iterator: Iter,
) -> (
	SplitPairLeft<ElementLeft, ElementRight, Iter>,
	SplitPairRight<ElementLeft, ElementRight, Iter>,
) {
	let buffer = Arc::new(Mutex::new(SplitBuffer::from(iterator)));
	(SplitPairLeft { buffer: buffer.clone() }, SplitPairRight { buffer })
}

pub struct SplitPairLeft<ElementLeft, ElementRight, Iter> {
	buffer: Arc<Mutex<SplitBuffer<ElementLeft, ElementRight, Iter>>>,
}

pub struct SplitPairRight<ElementLeft, ElementRight, Iter> {
	buffer: Arc<Mutex<SplitBuffer<ElementLeft, ElementRight, Iter>>>,
}

struct SplitBuffer<ElementLeft, ElementRight, Iter> {
	iterator: Option<Iter>,
	left_buffer: VecDeque<ElementLeft>,
	right_buffer: VecDeque<ElementRight>,
}

impl<ElementLeft, ElementRight, Iter> From<Iter> for SplitBuffer<ElementLeft, ElementRight, Iter> {
	fn from(iterator: Iter) -> Self {
		Self {
			iterator: Some(iterator),
			left_buffer: Default::default(),
			right_buffer: Default::default(),
		}
	}
}

impl<ElementLeft, ElementRight, Iter> Iterator for SplitPairLeft<ElementLeft, ElementRight, Iter>
where
	Iter: Iterator<Item = (ElementLeft, ElementRight)>,
{
	type Item = ElementLeft;

	fn next(&mut self) -> Option<Self::Item> {
		self.buffer.lock().next_left()
	}
}

impl<ElementLeft, ElementRight, Iter> Iterator for SplitPairRight<ElementLeft, ElementRight, Iter>
where
	Iter: Iterator<Item = (ElementLeft, ElementRight)>,
{
	type Item = ElementRight;

	fn next(&mut self) -> Option<Self::Item> {
		self.buffer.lock().next_right()
	}
}

impl<ElementLeft, ElementRight, Iter> SplitBuffer<ElementLeft, ElementRight, Iter>
where
	Iter: Iterator<Item = (ElementLeft, ElementRight)>,
{
	fn next_left(&mut self) -> Option<ElementLeft> {
		if let Some(element) = self.left_buffer.pop_front() {
			return Some(element);
		}

		self.queue_next();
		self.left_buffer.pop_front()
	}

	fn next_right(&mut self) -> Option<ElementRight> {
		if let Some(element) = self.right_buffer.pop_front() {
			return Some(element);
		}

		self.queue_next();
		self.right_buffer.pop_front()
	}

	fn queue_next(&mut self) {
		let iterator = if let Some(iterator) = self.iterator.as_mut() {
			iterator
		} else {
			return;
		};

		let (left, right) = if let Some(next) = iterator.next() {
			next
		} else {
			self.iterator = None;
			return;
		};

		self.left_buffer.push_back(left);
		self.right_buffer.push_back(right);
	}
}
