use std::cmp::{max, min, Ordering};
use std::ops::RangeInclusive;
use crate::{Condition, PROPERTIES_COUNT, Property};

#[derive(Debug, Clone)]
pub struct XMASRange([RangeInclusive<usize>; PROPERTIES_COUNT]);

impl XMASRange {
    pub fn from_range(range: RangeInclusive<usize>) -> Self {
        Self([range.clone(), range.clone(), range.clone(), range.clone()])
    }

    pub fn get(&self, property: Property) -> &RangeInclusive<usize> {
        self.0.get(property as usize).unwrap()
    }

    pub fn with_condition_applied(&self, condition: &Condition) -> Self {
        let mut new_range = self.clone();
        let &Condition(property, operator, value) = condition;
        let ref mut selected_range = new_range.0[property as usize];
        match operator {
            Ordering::Less => {
                *selected_range = *selected_range.start()..=min(*selected_range.end(), value - 1);
            }
            Ordering::Greater => {
                *selected_range = max(*selected_range.start(), value + 1)..=*selected_range.end();
            }
            Ordering::Equal => panic!("Equal not supported")
        }
        new_range
    }

    pub fn combinations_count(&self) -> usize {
        self.0.iter()
            .map(|r| if r.end() >= r.start() {
                r.end() + 1 - r.start()
            } else {
                0
            })
            .reduce(|acc, e| acc * e)
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;
    use crate::{Condition, Property};
    use crate::xmas_range::XMASRange;

    #[test]
    fn applies_greater_condition_correctly() {
        let previous = XMASRange::from_range(1..=4000);
        let condition = Condition(Property::X, Ordering::Greater, 41);

        let new = previous.with_condition_applied(&condition);

        assert_eq!(*new.get(Property::X), 42..=4000);
        assert_eq!(*new.get(Property::M), 1..=4000);
        assert_eq!(*new.get(Property::A), 1..=4000);
        assert_eq!(*new.get(Property::S), 1..=4000);
    }

    #[test]
    fn applies_lesser_condition_correctly() {
        let previous = XMASRange::from_range(1..=4000);
        let condition = Condition(Property::X, Ordering::Less, 420);

        let new = previous.with_condition_applied(&condition);

        assert_eq!(*new.get(Property::X), 1..=419);
        assert_eq!(*new.get(Property::M), 1..=4000);
        assert_eq!(*new.get(Property::A), 1..=4000);
        assert_eq!(*new.get(Property::S), 1..=4000);
    }

    #[test]
    fn calculates_combinations_correctly() {
        let range = XMASRange::from_range(1..=2); // 2 ^ 4 combinations

        assert_eq!(range.combinations_count(), 16);
    }
}
