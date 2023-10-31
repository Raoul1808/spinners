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
pub struct AlbumArtReference {
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
pub struct Difficulty {
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
    pub album_art_reference: AlbumArtReference,
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
    pub title_offset_y: f32,
    #[serde(default)]
    pub apple_music_link: String,
    pub spotify_link: String,
    pub difficulties: Vec<Difficulty>,
    pub platform_filter: i32,
    pub track_type: i32,
    pub editor_function: String,
    pub allow_custom_leaderboard_creation: bool,
}

#[derive(Debug)]
pub struct SrtbFile {
    pub raw_content: RawSrtbFile,
    pub track_info: TrackInfo,
}
