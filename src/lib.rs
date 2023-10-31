use chart::RawSrtbFile;
use serde_json::Error;

mod chart;

pub fn load_srtb_from_str(str: &str) -> Result<chart::SrtbFile, Error> {
    let raw_file: RawSrtbFile = serde_json::from_str(str)?;
    let track_info = serde_json::from_str(raw_file.large_string_values_container.values[0].val.as_str())?;
    Ok(chart::SrtbFile {
        raw_content: raw_file,
        track_info
    })
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
