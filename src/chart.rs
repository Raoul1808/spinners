use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnityObjectValue {
    pub key: String,
    pub json_key: String,
    pub full_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnityObjectValuesContainer {
    pub values: Vec<UnityObjectValue>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LargeStringValue {
    pub key: String,
    pub val: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LargeStringValuesContainer {
    pub values: Vec<LargeStringValue>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawSrtbFile {
    pub unity_object_values_container: UnityObjectValuesContainer,
    pub large_string_values_container: LargeStringValuesContainer,
    pub clip_info_count: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetBundleReference {
    pub bundle: String,
    pub asset_name: String,
    #[serde(rename = "m_guid")]
    pub m_guid: String,
}

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackgroundId {
    pub background_id: String,
}

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackgroundColoring {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActiveAssetBundleReference {
    pub bundle: String,
    pub asset_name: String,
    #[serde(rename = "m_guid")]
    pub m_guid: String,
    #[serde(rename = "_active")]
    pub active: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackInfo {
    pub track_order: i32,
    pub track_id: i32,
    pub album_art_reference: AssetBundleReference,
    pub background_id: BackgroundId,
    pub background_coloring: BackgroundColoring,
    #[serde(default)]
    pub fallback_background_id: BackgroundId,
    #[serde(default)]
    pub fallback_background_coloring: BackgroundColoring,
    pub artist_name: String,
    pub feat_artists: String,
    pub title: String,
    pub subtitle: String,
    pub track_label: String,
    pub charter: String,
    pub description: String,
    pub title_offset_y: f64,
    #[serde(default)]
    pub apple_music_link: String,
    pub spotify_link: String,
    pub difficulties: Vec<ActiveAssetBundleReference>,
    pub platform_filter: i32,
    pub track_type: i32,
    pub editor_function: String,
    pub allow_custom_leaderboard_creation: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreviewLoopBars {
    pub min: u32,
    pub max: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TranslationKey {
    pub key: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileReference {
    #[serde(rename = "m_FileID")]
    pub file_id: i32,
    #[serde(rename = "m_PathID")]
    pub path_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClipData {
    pub clip_index: i32,
    pub start_bar: i32,
    pub end_bar: i32,
    pub transition_in: i32,
    pub transition_in_value: f64,
    pub transition_in_offset: f64,
    pub transition_out: i32,
    pub transition_out_value: f64,
    pub transition_out_offset: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Note {
    pub time: f64,
    pub r#type: u8,
    pub color_index: u8,
    pub column: i32,
    #[serde(rename = "m_size")]
    pub m_size: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackData {
    pub revision_version: i32,
    pub compatibility_version: i32,
    pub difficulty_rating: u8,
    pub preview_loop_bars: PreviewLoopBars,
    pub go_beat_offset_from_first_note: f64,
    pub difficulty_type: i32,
    pub is_tutorial: bool,
    pub tutorial_title_translation: TranslationKey,
    pub clip_info_asset_references: Vec<AssetBundleReference>,
    pub background_id: BackgroundId,
    pub background: AssetBundleReference,
    pub ground_settings_to_use: AssetBundleReference,
    pub ground_settings_over_time: AssetBundleReference,
    // TODO: add beter compatibility here
    // pub spline_path_data: FileReference,
    // #[serde(rename = "_feverTime")]
    // pub fever_time: FileReference,
    // TODO: tutorial objects
    // TODO: clip data
    pub notes: Vec<Note>,
    // TODO: rewind sections
    pub last_edited_on_date: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeSignatureMarker {
    pub starting_beat: i32,
    pub ticks_per_bar: i32,
    pub tick_divisor: i32,
    pub beat_length_type: i32,
    pub beat_length_dotted: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BpmMarker {
    pub beat_length: f64,
    pub clip_time: f64,
    pub r#type: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CuePoint {
    pub name: String,
    pub time: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClipInfo {
    pub time_signature_markers: Vec<TimeSignatureMarker>,
    pub bpm_markers: Vec<BpmMarker>,
    pub cue_points: Vec<CuePoint>,
    pub clip_asset_reference: AssetBundleReference,
}

#[derive(Debug)]
pub struct SrtbFile {
    pub raw_content: RawSrtbFile,
    pub track_info: TrackInfo,
    pub easy_diff: TrackData,
    pub normal_diff: TrackData,
    pub hard_diff: TrackData,
    pub expert_diff: TrackData,
    pub xd_diff: TrackData,
    pub clip_info: ClipInfo,
}
