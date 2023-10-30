use serde_json::Error;

mod chart;

pub fn load_srtb_from_str(str: &str) -> Result<chart::RawSrtbFile, Error> {
    serde_json::from_str(str)
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
