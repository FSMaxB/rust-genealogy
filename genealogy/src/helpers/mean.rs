#[derive(Default)]
pub struct Mean {
	sum: Sum,
	count: usize,
}

impl Mean {
	pub fn add(mut self, number: f64) -> Mean {
		self.sum = self.sum.add(number);
		self.count += 1;
		self
	}
}

impl From<Mean> for Option<f64> {
	fn from(mean: Mean) -> Self {
		if mean.count == 0 {
			None
		} else {
			Some(f64::from(mean.sum) / (mean.count as f64))
		}
	}
}

#[derive(Default)]
pub struct Sum {
	partials: Vec<f64>,
}

impl Sum {
	/// This implementation is stolen from https://github.com/rust-lang/rust/blob/d577c535b4fac58c63c850b97d45b66993c6df3d/library/test/src/stats.rs#L140-L173
	/// since `Stats` is not stable yet.
	fn add(mut self, mut x: f64) -> Self {
		let mut j = 0;
		// This inner loop applies `hi`/`lo` summation to each
		// partial so that the list of partial sums remains exact.
		for i in 0..self.partials.len() {
			let mut y: f64 = self.partials[i];
			if x.abs() < y.abs() {
				std::mem::swap(&mut x, &mut y);
			}
			// Rounded `x+y` is stored in `hi` with round-off stored in
			// `lo`. Together `hi+lo` are exactly equal to `x+y`.
			let hi = x + y;
			let lo = y - (hi - x);
			if lo != 0.0 {
				self.partials[j] = lo;
				j += 1;
			}
			x = hi;
		}
		if j >= self.partials.len() {
			self.partials.push(x);
		} else {
			self.partials[j] = x;
			self.partials.truncate(j + 1);
		}
		self
	}
}

impl From<Sum> for f64 {
	/// This implementation is stolen from https://github.com/rust-lang/rust/blob/d577c535b4fac58c63c850b97d45b66993c6df3d/library/test/src/stats.rs#L140-L173
	/// since `Stats` is not stable yet.
	fn from(sum: Sum) -> Self {
		let zero: f64 = 0.0;
		sum.partials.iter().fold(zero, |p, q| p + *q)
	}
}
