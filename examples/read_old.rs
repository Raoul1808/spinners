fn main() {
    let srtb = include_str!("../thirdsun.srtb");
    let srtb = srtb_rs::load_srtb_from_str(srtb).unwrap();
    println!("{:#?}", srtb);
    println!("{}", srtb.clip_info.cue_points.len());
}
