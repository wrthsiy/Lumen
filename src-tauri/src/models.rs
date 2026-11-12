use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Stats {
    pub launches: u64,
    pub playtime_sec: u64,
    pub last_played: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Entry {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub target: String,
    #[serde(default)]
    pub args: String,
    #[serde(default)]
    pub work_dir: String,
    #[serde(default)]
    pub track_process: Option<String>,
    #[serde(default)]
    pub cover: Option<String>,
    #[serde(default)]
    pub background: Option<String>,
    #[serde(default)]
    pub video: Option<String>,
    #[serde(default)]
    pub accent: Option<String>,
    #[serde(default)]
    pub category: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub stats: Stats,
    #[serde(default)]
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BgSettings {
    #[serde(default = "default_bg_type")]
    pub r#type: String,
    #[serde(default)]
    pub value: Option<String>,
    #[serde(default = "default_blur")]
    pub blur: f64,
    #[serde(default = "default_dim")]
    pub dim: f64,
    #[serde(default = "default_brightness")]
    pub brightness: f64,
}

fn default_bg_type() -> String { "dynamic".into() }
fn default_blur() -> f64 { 20.0 }
fn default_dim() -> f64 { 0.4 }
fn default_brightness() -> f64 { 1.0 }

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TextSettings {
    #[serde(default = "default_font")]
    pub font: String,
    #[serde(default = "default_text_color")]
    pub color: String,
    #[serde(default)]
    pub shadow: bool,
    #[serde(default = "default_text_size")]
    pub size: f64,
}

fn default_font() -> String { "Segoe UI".into() }
fn default_text_color() -> String { "#ffffff".into() }
fn default_text_size() -> f64 { 16.0 }

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TileSettings {
    #[serde(default = "default_radius")]
    pub radius: f64,
    #[serde(default = "default_scale")]
    pub scale_selected: f64,
    #[serde(default)]
    pub glow: bool,
    #[serde(default = "default_tile_size")]
    pub size: f64,
}

fn default_radius() -> f64 { 14.0 }
fn default_scale() -> f64 { 1.15 }
fn default_tile_size() -> f64 { 140.0 }

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SoundSettings {
    #[serde(default = "default_pack")]
    pub pack: String,
    #[serde(default = "default_volume")]
    pub volume: f64,
}

fn default_pack() -> String { "default".into() }
fn default_volume() -> f64 { 0.5 }

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Theme {
    #[serde(default = "default_theme_name")]
    pub name: String,
    #[serde(default)]
    pub background: BgSettings,
    #[serde(default)]
    pub text: TextSettings,
    #[serde(default)]
    pub tile: TileSettings,
    #[serde(default)]
    pub sounds: SoundSettings,
    #[serde(default = "default_layout")]
    pub layout: String,
    #[serde(default)]
    pub custom_css: Option<String>,
    #[serde(default)]
    pub animation: AnimationSettings,
}

fn default_theme_name() -> String { "Default".into() }
fn default_layout() -> String { "rows".into() }

impl Default for Theme {
    fn default() -> Self {
        Self {
            name: default_theme_name(),
            background: BgSettings::default(),
            text: TextSettings::default(),
            tile: TileSettings::default(),
            sounds: SoundSettings::default(),
            layout: default_layout(),
            custom_css: None,
            animation: AnimationSettings::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Settings {
    #[serde(default)]
    pub fullscreen: bool,
    #[serde(default)]
    pub always_on_top: bool,
    #[serde(default = "default_hotkey")]
    pub hotkey: String,
    #[serde(default)]
    pub autostart: bool,
    #[serde(default = "default_true")]
    pub tray: bool,
    #[serde(default)]
    pub performance_mode: bool,
    #[serde(default = "default_layout")]
    pub layout: String,
    #[serde(default)]
    pub active_theme: Option<String>,
    #[serde(default)]
    pub animation: AnimationSettings,
}

fn default_hotkey() -> String { "Ctrl+Shift+L".into() }
fn default_true() -> bool { true }

impl Default for Settings {
    fn default() -> Self {
        Self {
            fullscreen: true,
            always_on_top: false,
            hotkey: default_hotkey(),
            autostart: false,
            tray: true,
            performance_mode: false,
            layout: default_layout(),
            active_theme: None,
            animation: AnimationSettings::default(),
        }
    }
}

impl Default for BgSettings {
    fn default() -> Self {
        Self {
            r#type: default_bg_type(),
            value: None,
            blur: default_blur(),
            dim: default_dim(),
            brightness: default_brightness(),
        }
    }
}

impl Default for TextSettings {
    fn default() -> Self {
        Self {
            font: default_font(),
            color: default_text_color(),
            shadow: false,
            size: default_text_size(),
        }
    }
}

impl Default for TileSettings {
    fn default() -> Self {
        Self {
            radius: default_radius(),
            scale_selected: default_scale(),
            glow: false,
            size: default_tile_size(),
        }
    }
}

impl Default for SoundSettings {
    fn default() -> Self {
        Self {
            pack: default_pack(),
            volume: default_volume(),
        }
    }
}

fn default_anim_mode() -> String { "swipe".into() }
fn default_anim_speed() -> f64 { 1.0 }
fn default_anim_intensity() -> f64 { 0.7 }
fn default_anim_easing() -> String { "spring".into() }

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AnimationSettings {
    #[serde(default = "default_anim_mode")]
    pub mode: String,
    #[serde(default = "default_anim_speed")]
    pub speed: f64,
    #[serde(default = "default_anim_intensity")]
    pub intensity: f64,
    #[serde(default = "default_anim_easing")]
    pub easing: String,
    #[serde(default = "default_true")]
    pub parallax: bool,
    #[serde(default = "default_true")]
    pub tilt: bool,
    #[serde(default = "default_true")]
    pub glow: bool,
    #[serde(default)]
    pub reflection: bool,
    #[serde(default)]
    pub reduced_motion: bool,
}

impl Default for AnimationSettings {
    fn default() -> Self {
        Self {
            mode: default_anim_mode(),
            speed: default_anim_speed(),
            intensity: default_anim_intensity(),
            easing: default_anim_easing(),
            parallax: true,
            tilt: true,
            glow: true,
            reflection: false,
            reduced_motion: false,
        }
    }
}