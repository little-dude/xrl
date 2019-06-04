use crate::ViewId;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfigChanged {
    pub view_id: ViewId,
    pub changes: ConfigChanges
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfigChanges {
    pub font_face: Option<String>,
    pub font_size: Option<f32>,
    pub line_ending: Option<String>,
    pub plugin_search_path: Option<Vec<String>>,
    pub tab_size: Option<u64>,
    pub translate_tabs_to_spaces: Option<bool>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ModifyUserConfig {
    pub domain: ConfigDomain,
    pub changes: ConfigChanges,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfigDomain {
    General,
    Syntax(String),
    UserOverride(String),
}
