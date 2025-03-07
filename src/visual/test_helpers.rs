use fake::{Fake, Faker};
use windows::Win32::Foundation::POINT;
use crate::locator::locator::Locator;


pub struct KeyQueueLengths(usize);

impl KeyQueueLengths {
    pub const SINGLE_CHAR: usize = 14;
    pub const DOUBLE_CHAR: usize = 196;
    pub const TRIPLE_CHAR: usize = 2744;
}

pub fn get_test_locators(count: usize) -> Vec<Locator> {
    let mut test_locators: Vec<Locator> = Vec::new();
    let point = || Locator {
        physical_point: POINT {
            x: Faker.fake(),
            y: Faker.fake(),
        },
        resolution_point: POINT {
            x: Faker.fake(),
            y: Faker.fake(),
        },
    };
    for _ in 0..count {
        test_locators.push(point());
    }

    test_locators
}
