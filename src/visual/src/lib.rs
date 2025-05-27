pub  mod locators_canvas;
pub  mod visual_root;
pub  mod vignette_canvas;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = windows_operations::add(2, 2);
        assert_eq!(result, 4);
    }
}
