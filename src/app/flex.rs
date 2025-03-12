pub trait FlexItem<T> {
    // original duration, for example
    fn max_size(&self) -> T;
    // time elapsed, for example
    fn min_size(&self) -> T;
}

pub trait Flex<T> where T:
    std::ops::Add<Output = T>
    + std::ops::Sub<Output = T>
    + std::ops::Div<Output = T>
    + std::ops::Mul<Output = T>
    + std::cmp::PartialOrd
    + std::marker::Copy
    + std::fmt::Debug
    {
    const ZERO: T;

    fn get_items(&self) -> &Vec<impl FlexItem<T>>;

    fn min_size(&self) -> T {
        self.get_items().iter()
            .fold(Self::ZERO, |acc, x| acc + x.min_size())
    }

    fn max_size(&self) -> T {
        self.get_items().iter()
            .fold(Self::ZERO, |acc, x| acc + x.max_size())
    }

    fn max_sizes(&self) -> Vec<T> {
        self.get_items().iter()
            .map(|item| item.max_size())
            .collect()
    }

    fn flex(&self, size: T) -> Result<Vec<T>, T> {
        if size < self.min_size() {
            // TODO better way to fail?
            todo!();
            return Err(self.min_size());
        }
        if size > self.max_size() {
            return Ok(self.max_sizes());
        }
        let wiggle_room = size - self.min_size();
        dbg!(wiggle_room);
        let shrinkable = self.max_size() - self.min_size();
        dbg!(shrinkable);
        let ratio = wiggle_room / shrinkable;
        dbg!(ratio);
        Ok(self.get_items().iter()
            .map(|item| {
                let item_wiggle = self.max_size() - self.min_size();
                self.min_size() + item_wiggle * ratio
            })
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Amount {
        max: f32,
        min: f32,
    }

    impl From<(f32, f32)> for Amount {
        fn from(item: (f32, f32)) -> Self {
            Amount {
                min: item.0,
                max: item.1,
            }
        }
    }

    impl FlexItem<f32> for Amount {
        fn max_size(&self) -> f32 {
            self.max
        }
        fn min_size(&self) -> f32 {
            self.min
        }
    }

    struct List {
        items: Vec<Amount>,
    }

    impl From<Vec<(f32, f32)>> for List {
        fn from(item: Vec<(f32, f32)>) -> Self {
            let items = item.iter()
                .map(|&tuple| Amount::from(tuple))
                .collect();
            Self {
                items
            }
        }
    }

    impl Flex<f32> for List {
        const ZERO: f32 = 0.0;

        fn get_items(&self) -> &Vec<Amount> {
            &self.items
        }
    }

    #[test]
    fn plenty_of_space() {
        let list: List = vec![(0.0, 10.4), (4.3, 5.3), (2.0, 8.4)].into();
        let result = list.flex(9999.0);

        assert_eq!(Ok(vec![10.4, 5.3, 8.4]), result);
    }
}
