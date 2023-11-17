use chart::{RawSrtbFile, TrackData};
use serde_json::Error;

mod chart;

/// Loads an SRTB file from the given string slice.
/// 
/// # Errors
/// 
/// This functions errors on invalid json or missing SRTB fields.
/// 
/// # Examples
/// 
/// ```
/// let file_contents = include_str!("../thirdsun.srtb");
/// let srtb_file = spinners::load_srtb_from_str(file_contents).unwrap_or_else(|err| {
///     eprintln!("Parsing error on file thirdsun.srtb: {err}");
///     std::process::exit(1);
/// });
/// 
/// // You can then use normally the SRTB data
/// println!("{}", srtb_file.track_info.title);
/// ```
pub fn load_srtb_from_str(str: &str) -> Result<chart::SrtbFile, Error> {
    let raw_file: RawSrtbFile = serde_json::from_str(str)?;
    let track_info = serde_json::from_str(raw_file.large_string_values_container.values[0].val.as_str())?;
    let mut diff = Vec::<TrackData>::new();
    for i in 1..=5 {
        let str = raw_file.large_string_values_container.values[i].val.as_str();
        let d = serde_json::from_str(str)?;
        diff.push(d);
    }
    let clip_info = serde_json::from_str(raw_file.large_string_values_container.values[6].val.as_str())?;
    Ok(chart::SrtbFile {
        raw_content: raw_file,
        track_info,
        xd_diff: diff.pop().unwrap(),
        expert_diff: diff.pop().unwrap(),
        hard_diff: diff.pop().unwrap(),
        normal_diff: diff.pop().unwrap(),
        easy_diff: diff.pop().unwrap(),
        clip_info,
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
