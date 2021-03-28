extern crate serde_json;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    #[serde(rename = "logging_page_id")]
    pub logging_page_id: String,
    #[serde(rename = "show_suggested_profiles")]
    pub show_suggested_profiles: bool,
    #[serde(rename = "show_follow_dialog")]
    pub show_follow_dialog: bool,
    pub graphql: Graphql,
    #[serde(rename = "toast_content_on_load")]
    pub toast_content_on_load: ::serde_json::Value,
    #[serde(rename = "show_view_shop")]
    pub show_view_shop: bool,
    #[serde(rename = "profile_pic_edit_sync_props")]
    pub profile_pic_edit_sync_props: ::serde_json::Value,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Graphql {
    pub user: User,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub biography: String,
    #[serde(rename = "blocked_by_viewer")]
    pub blocked_by_viewer: bool,
    #[serde(rename = "restricted_by_viewer")]
    pub restricted_by_viewer: bool,
    #[serde(rename = "country_block")]
    pub country_block: bool,
    #[serde(rename = "external_url")]
    pub external_url: ::serde_json::Value,
    #[serde(rename = "external_url_linkshimmed")]
    pub external_url_linkshimmed: ::serde_json::Value,
    #[serde(rename = "edge_followed_by")]
    pub edge_followed_by: EdgeFollowedBy,
    pub fbid: String,
    #[serde(rename = "followed_by_viewer")]
    pub followed_by_viewer: bool,
    #[serde(rename = "edge_follow")]
    pub edge_follow: EdgeFollow,
    #[serde(rename = "follows_viewer")]
    pub follows_viewer: bool,
    #[serde(rename = "full_name")]
    pub full_name: String,
    #[serde(rename = "has_ar_effects")]
    pub has_ar_effects: bool,
    #[serde(rename = "has_clips")]
    pub has_clips: bool,
    #[serde(rename = "has_guides")]
    pub has_guides: bool,
    #[serde(rename = "has_channel")]
    pub has_channel: bool,
    #[serde(rename = "has_blocked_viewer")]
    pub has_blocked_viewer: bool,
    #[serde(rename = "highlight_reel_count")]
    pub highlight_reel_count: i64,
    #[serde(rename = "has_requested_viewer")]
    pub has_requested_viewer: bool,
    pub id: String,
    #[serde(rename = "is_business_account")]
    pub is_business_account: bool,
    #[serde(rename = "is_joined_recently")]
    pub is_joined_recently: bool,
    #[serde(rename = "business_category_name")]
    pub business_category_name: ::serde_json::Value,
    #[serde(rename = "overall_category_name")]
    pub overall_category_name: ::serde_json::Value,
    #[serde(rename = "category_enum")]
    pub category_enum: ::serde_json::Value,
    #[serde(rename = "category_name")]
    pub category_name: String,
    #[serde(rename = "is_private")]
    pub is_private: bool,
    #[serde(rename = "is_verified")]
    pub is_verified: bool,
    #[serde(rename = "edge_mutual_followed_by")]
    pub edge_mutual_followed_by: EdgeMutualFollowedBy,
    #[serde(rename = "profile_pic_url")]
    pub profile_pic_url: String,
    #[serde(rename = "profile_pic_url_hd")]
    pub profile_pic_url_hd: String,
    #[serde(rename = "requested_by_viewer")]
    pub requested_by_viewer: bool,
    #[serde(rename = "should_show_category")]
    pub should_show_category: bool,
    pub username: String,
    #[serde(rename = "connected_fb_page")]
    pub connected_fb_page: ::serde_json::Value,
    #[serde(rename = "edge_felix_video_timeline")]
    pub edge_felix_video_timeline: EdgeFelixVideoTimeline,
    #[serde(rename = "edge_owner_to_timeline_media")]
    pub edge_owner_to_timeline_media: EdgeOwnerToTimelineMedia,
    #[serde(rename = "edge_saved_media")]
    pub edge_saved_media: EdgeSavedMedia,
    #[serde(rename = "edge_media_collections")]
    pub edge_media_collections: EdgeMediaCollections,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EdgeFollowedBy {
    pub count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EdgeFollow {
    pub count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EdgeMutualFollowedBy {
    pub count: i64,
    pub edges: Vec<::serde_json::Value>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EdgeFelixVideoTimeline {
    pub count: i64,
    #[serde(rename = "page_info")]
    pub page_info: PageInfo,
    pub edges: Vec<::serde_json::Value>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageInfo {
    #[serde(rename = "has_next_page")]
    pub has_next_page: bool,
    #[serde(rename = "end_cursor")]
    pub end_cursor: ::serde_json::Value,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EdgeOwnerToTimelineMedia {
    pub count: i64,
    #[serde(rename = "page_info")]
    pub page_info: PageInfo2,
    pub edges: Vec<Edge>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageInfo2 {
    #[serde(rename = "has_next_page")]
    pub has_next_page: bool,
    #[serde(rename = "end_cursor")]
    pub end_cursor: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Edge {
    pub node: Node,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    #[serde(rename = "__typename")]
    pub typename: String,
    pub id: String,
    pub shortcode: String,
    pub dimensions: Dimensions,
    #[serde(rename = "display_url")]
    pub display_url: String,
    #[serde(rename = "edge_media_to_tagged_user")]
    pub edge_media_to_tagged_user: EdgeMediaToTaggedUser,
    #[serde(rename = "fact_check_overall_rating")]
    pub fact_check_overall_rating: ::serde_json::Value,
    #[serde(rename = "fact_check_information")]
    pub fact_check_information: ::serde_json::Value,
    #[serde(rename = "gating_info")]
    pub gating_info: ::serde_json::Value,
    #[serde(rename = "sharing_friction_info")]
    pub sharing_friction_info: SharingFrictionInfo,
    #[serde(rename = "media_overlay_info")]
    pub media_overlay_info: ::serde_json::Value,
    #[serde(rename = "media_preview")]
    pub media_preview: Option<String>,
    pub owner: Owner,
    #[serde(rename = "is_video")]
    pub is_video: bool,
    #[serde(rename = "accessibility_caption")]
    pub accessibility_caption: Option<String>,
    #[serde(rename = "dash_info")]
    pub dash_info: Option<DashInfo>,
    #[serde(rename = "has_audio")]
    pub has_audio: Option<bool>,
    #[serde(rename = "tracking_token")]
    pub tracking_token: Option<String>,
    #[serde(rename = "video_url")]
    pub video_url: Option<String>,
    #[serde(rename = "video_view_count")]
    pub video_view_count: Option<i64>,
    #[serde(rename = "edge_media_to_caption")]
    pub edge_media_to_caption: EdgeMediaToCaption,
    #[serde(rename = "edge_media_to_comment")]
    pub edge_media_to_comment: EdgeMediaToComment,
    #[serde(rename = "comments_disabled")]
    pub comments_disabled: bool,
    #[serde(rename = "taken_at_timestamp")]
    pub taken_at_timestamp: i64,
    #[serde(rename = "edge_liked_by")]
    pub edge_liked_by: EdgeLikedBy,
    #[serde(rename = "edge_media_preview_like")]
    pub edge_media_preview_like: EdgeMediaPreviewLike,
    pub location: ::serde_json::Value,
    #[serde(rename = "thumbnail_src")]
    pub thumbnail_src: String,
    #[serde(rename = "thumbnail_resources")]
    pub thumbnail_resources: Vec<ThumbnailResource>,
    #[serde(rename = "felix_profile_grid_crop")]
    pub felix_profile_grid_crop: ::serde_json::Value,
    #[serde(rename = "product_type")]
    pub product_type: Option<String>,
    #[serde(rename = "clips_music_attribution_info")]
    pub clips_music_attribution_info: Option<ClipsMusicAttributionInfo>,
    #[serde(rename = "edge_sidecar_to_children")]
    pub edge_sidecar_to_children: Option<EdgeSidecarToChildren>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dimensions {
    pub height: i64,
    pub width: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EdgeMediaToTaggedUser {
    pub edges: Vec<Edge2>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Edge2 {
    pub node: Node2,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Node2 {
    pub user: User2,
    pub x: f64,
    pub y: f64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User2 {
    #[serde(rename = "full_name")]
    pub full_name: String,
    pub id: String,
    #[serde(rename = "is_verified")]
    pub is_verified: bool,
    #[serde(rename = "profile_pic_url")]
    pub profile_pic_url: String,
    pub username: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SharingFrictionInfo {
    #[serde(rename = "should_have_sharing_friction")]
    pub should_have_sharing_friction: bool,
    #[serde(rename = "bloks_app_url")]
    pub bloks_app_url: ::serde_json::Value,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Owner {
    pub id: String,
    pub username: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DashInfo {
    #[serde(rename = "is_dash_eligible")]
    pub is_dash_eligible: bool,
    #[serde(rename = "video_dash_manifest")]
    pub video_dash_manifest: String,
    #[serde(rename = "number_of_qualities")]
    pub number_of_qualities: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EdgeMediaToCaption {
    pub edges: Vec<Edge3>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Edge3 {
    pub node: Node3,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Node3 {
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EdgeMediaToComment {
    pub count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EdgeLikedBy {
    pub count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EdgeMediaPreviewLike {
    pub count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThumbnailResource {
    pub src: String,
    #[serde(rename = "config_width")]
    pub config_width: i64,
    #[serde(rename = "config_height")]
    pub config_height: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClipsMusicAttributionInfo {
    #[serde(rename = "artist_name")]
    pub artist_name: String,
    #[serde(rename = "song_name")]
    pub song_name: String,
    #[serde(rename = "uses_original_audio")]
    pub uses_original_audio: bool,
    #[serde(rename = "should_mute_audio")]
    pub should_mute_audio: bool,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EdgeSidecarToChildren {
    pub edges: Vec<Edge4>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Edge4 {
    pub node: Node4,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Node4 {
    #[serde(rename = "__typename")]
    pub typename: String,
    pub id: String,
    pub shortcode: String,
    pub dimensions: Dimensions2,
    #[serde(rename = "display_url")]
    pub display_url: String,
    #[serde(rename = "edge_media_to_tagged_user")]
    pub edge_media_to_tagged_user: EdgeMediaToTaggedUser2,
    #[serde(rename = "fact_check_overall_rating")]
    pub fact_check_overall_rating: ::serde_json::Value,
    #[serde(rename = "fact_check_information")]
    pub fact_check_information: ::serde_json::Value,
    #[serde(rename = "gating_info")]
    pub gating_info: ::serde_json::Value,
    #[serde(rename = "sharing_friction_info")]
    pub sharing_friction_info: SharingFrictionInfo2,
    #[serde(rename = "media_overlay_info")]
    pub media_overlay_info: ::serde_json::Value,
    #[serde(rename = "media_preview")]
    pub media_preview: String,
    pub owner: Owner2,
    #[serde(rename = "is_video")]
    pub is_video: bool,
    #[serde(rename = "accessibility_caption")]
    pub accessibility_caption: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dimensions2 {
    pub height: i64,
    pub width: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EdgeMediaToTaggedUser2 {
    pub edges: Vec<Edge5>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Edge5 {
    pub node: Node5,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Node5 {
    pub user: User3,
    pub x: f64,
    pub y: f64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User3 {
    #[serde(rename = "full_name")]
    pub full_name: String,
    pub id: String,
    #[serde(rename = "is_verified")]
    pub is_verified: bool,
    #[serde(rename = "profile_pic_url")]
    pub profile_pic_url: String,
    pub username: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SharingFrictionInfo2 {
    #[serde(rename = "should_have_sharing_friction")]
    pub should_have_sharing_friction: bool,
    #[serde(rename = "bloks_app_url")]
    pub bloks_app_url: ::serde_json::Value,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Owner2 {
    pub id: String,
    pub username: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EdgeSavedMedia {
    pub count: i64,
    #[serde(rename = "page_info")]
    pub page_info: PageInfo3,
    pub edges: Vec<::serde_json::Value>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageInfo3 {
    #[serde(rename = "has_next_page")]
    pub has_next_page: bool,
    #[serde(rename = "end_cursor")]
    pub end_cursor: ::serde_json::Value,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EdgeMediaCollections {
    pub count: i64,
    #[serde(rename = "page_info")]
    pub page_info: PageInfo4,
    pub edges: Vec<::serde_json::Value>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageInfo4 {
    #[serde(rename = "has_next_page")]
    pub has_next_page: bool,
    #[serde(rename = "end_cursor")]
    pub end_cursor: ::serde_json::Value,
}



