use fake::{Fake, Faker};
use windows::Win32::Foundation::POINT;
use crate::locator::locator::Locator;

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
