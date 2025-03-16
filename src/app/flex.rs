use std::time::Duration;

pub trait FlexItem {
    // original duration, for example
    fn max_size(&self) -> Duration;
    // time elapsed, for example
    fn min_size(&self) -> Duration;
}

pub trait Flex {
    fn get_items(&self) -> &Vec<impl FlexItem>;

    fn min_size(&self) -> Duration {
        self.get_items().iter()
            .fold(Duration::ZERO, |acc, x| acc + x.min_size())
    }

    fn max_size(&self) -> Duration {
        self.get_items().iter()
            .fold(Duration::ZERO, |acc, x| acc + x.max_size())
    }

    fn max_sizes(&self) -> Vec<Duration> {
        self.get_items().iter()
            .map(|item| item.max_size())
            .collect()
    }

    fn flex(&self, size: Duration) -> Result<Vec<Duration>, Duration> {
        let wiggle_room = size.saturating_sub(self.min_size());
        let shrinkable = self.max_size().saturating_sub(self.min_size());
        dbg!(shrinkable);
        dbg!(self.min_size());
        dbg!(self.max_size());
        dbg!(size);
        dbg!(wiggle_room);
        if size < self.min_size() {
            // TODO better way to fail?
            return Err(self.min_size());
        }
        if size > self.max_size() {
            return Ok(self.max_sizes());
        }
        let ratio = wiggle_room.div_duration_f64(shrinkable);
        dbg!(ratio);
        Ok(self.get_items().iter()
            .map(|item| {
                let item_wiggle = item.max_size().saturating_sub(item.min_size());
                dbg!(item_wiggle);
                item.min_size() + item_wiggle.mul_f64(ratio)
            })
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Amount {
        max: Duration,
        min: Duration,
    }

    impl From<(f64, f64)> for Amount {
        fn from(item: (f64, f64)) -> Self {
            Amount {
                min: Duration::try_from_secs_f64(item.0).unwrap(),
                max: Duration::try_from_secs_f64(item.1).unwrap(),
            }
        }
    }

    impl FlexItem for Amount {
        fn max_size(&self) -> Duration {
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
            let items = item.iter()
                .map(|&tuple| Amount::from(tuple))
                .collect();
            Self {
                items
            }
        }
    }

    impl Flex for List {
        fn get_items(&self) -> &Vec<Amount> {
            &self.items
        }
    }

    fn to_durations(l: Vec<f64>) -> Vec<Duration> {
        l.iter().map(|&f| Duration::try_from_secs_f64(f).expect("failed to convert to Duration")).collect()
    }

    #[test]
    fn plenty_of_space() {
        let list: List = vec![(0.0, 10.4), (4.3, 5.3), (2.0, 8.4)].into();
        let result = list.flex(Duration::try_from_secs_f64(9999.0).unwrap());

        let target = to_durations(vec![10.4, 5.3, 8.4]);
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

    #[test]
    fn min_over_max() {
        let list: List = vec![(11.0, 10.0), (0.0, 4.0)].into();
        let result = list.flex(Duration::try_from_secs_f64(12.0).unwrap());

        let target = to_durations(vec![11.0, 0.0]);
        assert_eq!(Ok(target), result);
    }
}
