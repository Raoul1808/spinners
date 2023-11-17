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

#[cfg(test)]
mod tests {
    use crate::load_srtb_from_str;

    #[test]
    fn load_srtb() {
        let file_contents = include_str!("../thirdsun.srtb");
        let srtb = load_srtb_from_str(file_contents);
        // Testing proper loading
        assert!(srtb.is_ok());
        let srtb = srtb.unwrap();
        // Testing a few correct values
        assert_eq!(srtb.track_info.charter, "Raoul1808");  // I charted this
        assert_eq!(srtb.xd_diff.notes.len(), 881);  // There are 881 notes in this chart's XD difficulty
        assert_eq!(srtb.clip_info.cue_points.len(), 15);  // There are 15 cue points in this chart
        
        // I could test everything, but there are too many fields to check, so we'll count this as being enough.
        // If it doesn't sound professional, it's because I'm very much new to unit-testing and publishing libraries/crates.
    }
}
