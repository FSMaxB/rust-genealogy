use crate::exception::Exception;
use crate::list::List;
use crate::map::Map;
use crate::set::Set;
use crate::string::JString;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

/// The characteristics where left out because they aren't used in the original Java code anyways.
pub struct Collector<Input, Accumulated, Reduced> {
	pub supplier: Box<dyn FnOnce() -> Result<Accumulated, Exception>>,
	pub accumulator: Box<dyn Fn(&mut Accumulated, Input) -> Result<(), Exception>>,
	pub combiner: Box<dyn Fn(Accumulated, Accumulated) -> Result<Accumulated, Exception>>,
	pub finisher: Box<dyn FnOnce(Accumulated) -> Result<Reduced, Exception>>,
}

impl<Input, Accumulated, Reduced> Collector<Input, Accumulated, Reduced> {
	pub fn of(
		supplier: impl FnOnce() -> Result<Accumulated, Exception> + 'static,
		accumulator: impl Fn(&mut Accumulated, Input) -> Result<(), Exception> + 'static,
		combiner: impl Fn(Accumulated, Accumulated) -> Result<Accumulated, Exception> + 'static,
		finisher: impl Fn(Accumulated) -> Result<Reduced, Exception> + 'static,
	) -> Self {
		Self {
			supplier: Box::new(supplier),
			accumulator: Box::new(accumulator),
			combiner: Box::new(combiner),
			finisher: Box::new(finisher),
		}
	}
}

pub enum Collectors {}

impl Collectors {
	pub fn mapping<Input, DownstreamInput, Accumulated, Reduced>(
		mapper: impl Fn(Input) -> DownstreamInput + 'static,
		downstream: Collector<DownstreamInput, Accumulated, Reduced>,
	) -> Collector<Input, Accumulated, Reduced>
	where
		DownstreamInput: 'static,
		Accumulated: 'static,
	{
		let Collector {
			supplier,
			accumulator,
			combiner,
			finisher,
		} = downstream;
		Collector {
			supplier,
			accumulator: Box::new(move |accumulated, item| {
				let mapped = mapper(item);
				accumulator(accumulated, mapped)
			}),
			combiner,
			finisher,
		}
	}

	pub fn teeing<Input, Accumulated1, Reduced1, Accumulated2, Reduced2, Reduced>(
		downstream1: Collector<Input, Accumulated1, Reduced1>,
		downstream2: Collector<Input, Accumulated2, Reduced2>,
		merger: impl FnOnce(Reduced1, Reduced2) -> Result<Reduced, Exception> + 'static,
	) -> Collector<Input, (Accumulated1, Accumulated2), Reduced>
	where
		Input: Clone + 'static,
		Accumulated1: 'static,
		Reduced1: 'static,
		Accumulated2: 'static,
		Reduced2: 'static,
	{
		Collector {
			supplier: Box::new(move || {
				let accumulated1 = (downstream1.supplier)()?;
				let accumulated2 = (downstream2.supplier)()?;
				Ok((accumulated1, accumulated2))
			}),
			accumulator: Box::new(move |(accumulated1, accumulated2), input| {
				(downstream1.accumulator)(accumulated1, input.clone())?;
				(downstream2.accumulator)(accumulated2, input)
			}),
			combiner: Box::new(
				move |(first_accumulated1, first_accumulated2), (second_accumulated1, second_accumulated2)| {
					let accumulated1 = (downstream1.combiner)(first_accumulated1, second_accumulated1)?;
					let accumulated2 = (downstream2.combiner)(first_accumulated2, second_accumulated2)?;
					Ok((accumulated1, accumulated2))
				},
			),
			finisher: Box::new(move |(accumulated1, accumulated2)| {
				let reduced1 = (downstream1.finisher)(accumulated1)?;
				let reduced2 = (downstream2.finisher)(accumulated2)?;
				merger(reduced1, reduced2)
			}),
		}
	}

	pub fn to_unmodifiable_set<Input>() -> Collector<Input, HashSet<Input>, Set<Input>>
	where
		Input: Hash + Eq + 'static,
	{
		Collector {
			supplier: Box::new(|| Ok(HashSet::default())),
			accumulator: Box::new(|set, element| {
				set.insert(element);
				Ok(())
			}),
			combiner: Box::new(|mut set1, set2| {
				set1.extend(set2);
				Ok(set1)
			}),
			finisher: Box::new(|set| Ok(set.into())),
		}
	}

	pub fn to_map<Input, Key, Value>(
		key_mapper: impl Fn(&Input) -> Key + 'static,
		value_mapper: impl Fn(&Input) -> Value + 'static,
	) -> Collector<Input, HashMap<Key, Value>, Map<Key, Value>>
	where
		Key: Hash + Eq + 'static,
		Value: 'static,
	{
		Collector {
			supplier: Box::new(|| Ok(HashMap::default())),
			accumulator: Box::new(move |map, input| {
				map.insert(key_mapper(&input), value_mapper(&input));
				Ok(())
			}),
			combiner: Box::new(|mut map1, map2| {
				map1.extend(map2);
				Ok(map1)
			}),
			finisher: Box::new(|map| Ok(map.into())),
		}
	}

	#[allow(clippy::type_complexity)]
	pub fn grouping_by<Input, Key>(
		classifier: impl Fn(&Input) -> Key + 'static,
	) -> Collector<Input, HashMap<Key, Vec<Input>>, Map<Key, List<Input>>>
	where
		Key: Hash + Eq,
	{
		Collector {
			supplier: Box::new(|| Ok(HashMap::new())),
			accumulator: Box::new(move |map, input| {
				let vector = map.entry(classifier(&input)).or_insert_with(Vec::new);
				vector.push(input);
				Ok(())
			}),
			combiner: Box::new(|mut map1, map2| {
				map1.extend(map2);
				Ok(map1)
			}),
			finisher: Box::new(|hash_map| {
				Ok(hash_map
					.into_iter()
					.map(|(key, vector)| (key, List::from(vector)))
					.collect::<HashMap<_, _>>()
					.into())
			}),
		}
	}

	pub fn joining(delimiter: &'static str) -> Collector<JString, Vec<String>, JString> {
		Collector {
			supplier: Box::new(|| Ok(Vec::new())),
			accumulator: Box::new(|strings, string| {
				strings.push(string.to_string());
				Ok(())
			}),
			combiner: Box::new(|mut strings1, strings2| {
				strings1.extend(strings2);
				Ok(strings1)
			}),
			finisher: Box::new(|strings| Ok(strings.join(delimiter).into())),
		}
	}

	/// ```java
	///
	/// /**
	///  * Returns a {@code Collector} that produces the arithmetic mean of a double-valued
	///  * function applied to the input elements.  If no elements are present,
	///  * the result is 0.
	///  *
	///  * <p>The average returned can vary depending upon the order in which
	///  * values are recorded, due to accumulated rounding error in
	///  * addition of values of differing magnitudes. Values sorted by increasing
	///  * absolute magnitude tend to yield more accurate results.  If any recorded
	///  * value is a {@code NaN} or the sum is at any point a {@code NaN} then the
	///  * average will be {@code NaN}.
	///  *
	///  * @implNote The {@code double} format can represent all
	///  * consecutive integers in the range -2<sup>53</sup> to
	///  * 2<sup>53</sup>. If the pipeline has more than 2<sup>53</sup>
	///  * values, the divisor in the average computation will saturate at
	///  * 2<sup>53</sup>, leading to additional numerical errors.
	///  *
	///  * @param <T> the type of the input elements
	///  * @param mapper a function extracting the property to be averaged
	///  * @return a {@code Collector} that produces the arithmetic mean of a
	///  * derived property
	///  */
	/// public static <T> Collector<T, ?, Double>
	/// averagingDouble(ToDoubleFunction<? super T> mapper) {
	/// 	/*
	/// 	 * In the arrays allocated for the collect operation, index 0
	/// 	 * holds the high-order bits of the running sum, index 1 holds
	/// 	 * the low-order bits of the sum computed via compensated
	/// 	 * summation, and index 2 holds the number of values seen.
	/// 	 */
	/// 	return new CollectorImpl<>(
	/// 		() -> new double[4],
	/// 		(a, t) -> { double val = mapper.applyAsDouble(t); sumWithCompensation(a, val); a[2]++; a[3]+= val;},
	/// 		(a, b) -> { sumWithCompensation(a, b[0]); sumWithCompensation(a, b[1]); a[2] += b[2]; a[3] += b[3]; return a; },
	/// 		a -> (a[2] == 0) ? 0.0d : (computeFinalSum(a) / a[2]),
	/// 		CH_NOID);
	/// }
	/// ```

	pub fn averaging_double<Input>(
		mapper: impl Fn(Input) -> Result<f64, Exception> + 'static,
	) -> Collector<Input, AccumulatedDoubleAverage, f64> {
		Collector {
			supplier: Box::new(|| Ok(AccumulatedDoubleAverage::default())),
			accumulator: Box::new(move |accumulated, input| {
				let double = mapper(input)?;
				accumulated.sum_with_compensation(double);
				accumulated.value_count += 1.0;
				accumulated.simple_sum += double;
				Ok(())
			}),
			combiner: Box::new(|mut a, b| {
				a.sum_with_compensation(b.high_order_bits);
				a.sum_with_compensation(b.low_order_bits);
				a.value_count += b.value_count;
				a.simple_sum += b.simple_sum;
				Ok(a)
			}),
			finisher: Box::new(|accumulated| {
				let average = if accumulated.value_count == 0.0 {
					0.0
				} else {
					accumulated.compute_final_sum() / accumulated.value_count
				};

				Ok(average)
			}),
		}
	}
}

#[derive(Default)]
pub struct AccumulatedDoubleAverage {
	high_order_bits: f64,
	low_order_bits: f64,
	value_count: f64,
	simple_sum: f64,
}

impl AccumulatedDoubleAverage {
	/// ```java
	/// /**
	/// * Incorporate a new double value using Kahan summation /
	/// * compensation summation.
	/// *
	/// * High-order bits of the sum are in intermediateSum[0], low-order
	/// * bits of the sum are in intermediateSum[1], any additional
	/// * elements are application-specific.
	/// *
	/// * @param intermediateSum the high-order and low-order words of the intermediate sum
	/// * @param value the name value to be included in the running sum
	/// */
	/// static double[] sumWithCompensation(double[] intermediateSum, double value) {
	/// 	double tmp = value - intermediateSum[1];
	/// 	double sum = intermediateSum[0];
	/// 	double velvel = sum + tmp; // Little wolf of rounding error
	/// 	intermediateSum[1] = (velvel - sum) - tmp;
	/// 	intermediateSum[0] = velvel;
	/// 	return intermediateSum;
	/// }
	/// ```
	pub(crate) fn sum_with_compensation(&mut self, value: f64) {
		let tmp = value - self.low_order_bits;
		let sum = self.high_order_bits;
		let velvel = sum + tmp; // Little wolf of rounding error
		self.low_order_bits = (velvel - sum) - tmp;
		self.high_order_bits = velvel;
	}

	/// ```java
	/// /**
	///  * If the compensated sum is spuriously NaN from accumulating one
	///  * or more same-signed infinite values, return the
	///  * correctly-signed infinity stored in the simple sum.
	///  */
	/// static double computeFinalSum(double[] summands) {
	/// 	// Better error bounds to add both terms as the final sum
	/// 	double tmp = summands[0] + summands[1];
	/// 	double simpleSum = summands[summands.length - 1];
	/// 	if (Double.isNaN(tmp) && Double.isInfinite(simpleSum))
	/// 		return simpleSum;
	/// 	else
	/// 		return tmp;
	/// }
	/// ```
	pub(crate) fn compute_final_sum(&self) -> f64 {
		// Better error bounds to add both terms as the final sum
		let tmp = self.high_order_bits + self.low_order_bits;
		if tmp.is_nan() && self.simple_sum.is_finite() {
			self.simple_sum
		} else {
			tmp
		}
	}
}
