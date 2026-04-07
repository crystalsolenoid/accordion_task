use std::time::Duration;

// Clippy reason: TODO I tried changing it to Item to try it out but something wouldn't
// compile.
#[allow(clippy::module_name_repetitions)]
pub trait FlexItem {
	// original duration, for example
	fn max_size(&self) -> Option<Duration>;
	// time elapsed, for example
	fn min_size(&self) -> Duration;
}

pub trait Flex<I>
where
	I: FlexItem,
{
	fn get_items(&self) -> &Vec<I>;

	fn min_size(&self) -> Duration {
		self.get_items()
			.iter()
			.fold(Duration::ZERO, |acc, x| acc + x.min_size())
	}

	fn max_size(&self) -> Duration {
		// TODO am I handling max size: None correctly?
		self.get_items().iter().fold(Duration::ZERO, |acc, x| {
			acc + x.max_size().unwrap_or(Duration::ZERO)
		})
	}

	fn max_sizes(&self, remainder: Duration) -> Vec<Duration> {
		// TODO rename fn
		// TODO account for multiple growing items
		self.get_items()
			.iter()
			.map(|i| i.max_size().unwrap_or(remainder))
			.collect()
	}

	fn flex(&self, size: Duration) -> Result<Vec<Duration>, Duration> {
		let wiggle_room = size.saturating_sub(self.min_size());
		let shrinkable = self.max_size().saturating_sub(self.min_size());
		if size < self.min_size() {
			// TODO better way to fail?
			return Err(self.min_size());
		}
		if size > self.max_size() {
			// TODO grow growing here
			return Ok(self.max_sizes(size - self.max_size()));
		}
		let ratio = wiggle_room.div_duration_f64(shrinkable);
		Ok(self
			.get_items()
			.iter()
			.map(|item| {
				let item_wiggle = item
					.max_size()
					.unwrap_or(Duration::ZERO)
					.saturating_sub(item.min_size());
				item.min_size() + item_wiggle.mul_f64(ratio)
			})
			.collect())
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	struct Amount {
		min: Duration,
		max: Option<Duration>,
	}

	impl From<(f64, f64)> for Amount {
		fn from(item: (f64, f64)) -> Self {
			Amount {
				min: Duration::try_from_secs_f64(item.0).unwrap(),
				max: Some(Duration::try_from_secs_f64(item.1).unwrap()),
			}
		}
	}

	impl From<(f64, Option<f64>)> for Amount {
		fn from(item: (f64, Option<f64>)) -> Self {
			Amount {
				min: Duration::try_from_secs_f64(item.0).unwrap(),
				max: item.1.map(|x| Duration::try_from_secs_f64(x).unwrap()),
			}
		}
	}

	impl FlexItem for Amount {
		fn max_size(&self) -> Option<Duration> {
			self.max
		}
		fn min_size(&self) -> Duration {
			self.min
		}
	}

	struct List {
		items: Vec<Amount>,
	}

	impl From<Vec<(f64, f64)>> for List {
		fn from(item: Vec<(f64, f64)>) -> Self {
			let items = item.iter().map(|&tuple| Amount::from(tuple)).collect();
			Self { items }
		}
	}

	impl Flex<Amount> for List {
		fn get_items(&self) -> &Vec<Amount> {
			&self.items
		}
	}

	fn to_durations(l: Vec<f64>) -> Vec<Duration> {
		l.iter()
			.map(|&f| Duration::try_from_secs_f64(f).expect("failed to convert to Duration"))
			.collect()
	}

	#[test]
	fn plenty_of_space() {
		let list: List = vec![(0.0, 10.4), (4.3, 5.3), (2.0, 8.4)].into();
		let result = list.flex(Duration::try_from_secs_f64(9999.0).unwrap());

		let target = to_durations(vec![10.4, 5.3, 8.4]);
		assert_eq!(Ok(target), result);
	}

	#[test]
	fn plenty_of_space_growing() {
		// TODO write test
		let mut list: List = vec![(0.0, 10.0), (4.3, 6.0)].into();
		list.items.push((0.3, None).into());
		list.items.push((0.0, 4.0).into());

		let result = list.flex(Duration::try_from_secs_f64(100.0).unwrap());

		let target = to_durations(vec![10.0, 6.0, 80.0, 4.0]);
		assert_eq!(Ok(target), result);
	}

	#[test]
	fn no_minimum() {
		let list: List = vec![(0.0, 10.0), (0.0, 4.0), (0.0, 8.0)].into();
		let result = list.flex(Duration::try_from_secs_f64(11.0).unwrap());

		let target = to_durations(vec![5.0, 2.0, 4.0]);
		assert_eq!(Ok(target), result);
	}

	#[test]
	fn has_minimum() {
		let list: List = vec![(10.0, 10.0), (0.0, 4.0), (0.0, 8.0)].into();
		let result = list.flex(Duration::try_from_secs_f64(16.0).unwrap());

		let target = to_durations(vec![10.0, 2.0, 4.0]);
		assert_eq!(Ok(target), result);
	}

	#[test]
	fn not_enough_space() {
		let list: List = vec![(10.0, 10.0), (0.0, 4.0), (0.0, 8.0)].into();
		let result = list.flex(Duration::try_from_secs_f64(10.0).unwrap());

		let target = to_durations(vec![10.0, 0.0, 0.0]);
		assert_eq!(Ok(target), result);
	}
}
