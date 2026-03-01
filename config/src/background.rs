use crate::{Config, Dimension, HsbTransform, PixelUnit, RgbaColor, default_one_point_oh};
use luahelper::impl_lua_conversion_dynamic;
use termwiz::color::SrgbaTuple;
use wezterm_dynamic::{FromDynamic, FromDynamicOptions, ToDynamic, Value};

#[derive(Debug, Clone, FromDynamic, ToDynamic)]
pub struct ImageFileSource {
    pub path: String,

    /// Adjust the animation rate for animated images
    #[dynamic(default = "default_one_point_oh")]
    pub speed: f32,
}

#[derive(Debug, Clone, ToDynamic)]
pub struct ImageFileSourceWrap {
    #[dynamic(flatten)]
    inner: ImageFileSource,
}

impl std::ops::Deref for ImageFileSourceWrap {
    type Target = ImageFileSource;
    fn deref(&self) -> &ImageFileSource {
        &self.inner
    }
}

impl FromDynamic for ImageFileSourceWrap {
    fn from_dynamic(
        value: &Value,
        options: FromDynamicOptions,
    ) -> Result<Self, wezterm_dynamic::Error> {
        if let Value::String(path) = value {
            Ok(Self {
                inner: ImageFileSource {
                    path: path.clone(),
                    speed: 1.0,
                },
            })
        } else {
            let inner = ImageFileSource::from_dynamic(value, options)?;
            Ok(Self { inner })
        }
    }
}

#[derive(Debug, Clone, FromDynamic, ToDynamic)]
pub enum BackgroundSource {
    Gradient(Gradient),
    File(ImageFileSourceWrap),
    Color(RgbaColor),
}

#[derive(Debug, Clone, FromDynamic, ToDynamic)]
pub struct BackgroundLayer {
    pub source: BackgroundSource,

    /// Where the top left corner of the background begins
    #[dynamic(default)]
    pub origin: BackgroundOrigin,

    #[dynamic(default)]
    pub attachment: BackgroundAttachment,

    #[dynamic(default)]
    pub repeat_x: BackgroundRepeat,
    #[dynamic(try_from = "crate::units::OptPixelUnit", default)]
    pub repeat_x_size: Option<Dimension>,

    #[dynamic(default)]
    pub repeat_y: BackgroundRepeat,
    #[dynamic(try_from = "crate::units::OptPixelUnit", default)]
    pub repeat_y_size: Option<Dimension>,

    #[dynamic(default)]
    pub vertical_align: BackgroundVerticalAlignment,
    #[dynamic(try_from = "crate::units::OptPixelUnit", default)]
    pub vertical_offset: Option<Dimension>,

    #[dynamic(default)]
    pub horizontal_align: BackgroundHorizontalAlignment,
    #[dynamic(try_from = "crate::units::OptPixelUnit", default)]
    pub horizontal_offset: Option<Dimension>,

    /// Additional alpha modifier
    #[dynamic(default = "default_one_point_oh")]
    pub opacity: f32,

    /// Additional hsb transform
    #[dynamic(default)]
    pub hsb: HsbTransform,

    #[dynamic(default)]
    pub width: BackgroundSize,

    #[dynamic(default)]
    pub height: BackgroundSize,
}

impl BackgroundLayer {
    #[must_use]
    pub fn with_legacy(cfg: &Config) -> Option<Self> {
        let source = if let Some(gradient) = &cfg.window_background_gradient {
            BackgroundSource::Gradient(gradient.clone())
        } else if let Some(path) = &cfg.window_background_image {
            BackgroundSource::File(ImageFileSourceWrap {
                inner: ImageFileSource {
                    path: path.to_string_lossy().to_string(),
                    speed: 1.0,
                },
            })
        } else {
            return None;
        };
        Some(Self {
            source,
            opacity: cfg.window_background_opacity,
            hsb: cfg.window_background_image_hsb.unwrap_or_default(),
            origin: Default::default(),
            attachment: Default::default(),
            repeat_x: Default::default(),
            repeat_y: Default::default(),
            repeat_x_size: None,
            repeat_y_size: None,
            vertical_align: Default::default(),
            horizontal_align: Default::default(),
            vertical_offset: None,
            horizontal_offset: None,
            width: BackgroundSize::Dimension(Dimension::Percent(1.)),
            height: BackgroundSize::Dimension(Dimension::Percent(1.)),
        })
    }
}

/// <https://developer.mozilla.org/en-US/docs/Web/CSS/background-size>
#[derive(Debug, Copy, Clone, Default)]
pub enum BackgroundSize {
    /// Scales image as large as possible without cropping or stretching.
    /// If the container is larger than the image, tiles the image unless
    /// the correspond `repeat` is `NoRepeat`.
    Contain,
    /// Scale the image (preserving aspect ratio) to the smallest possible
    /// size to the fill the container leaving no empty space.
    /// If the aspect ratio differs from the background, the image is
    /// cropped.
    #[default]
    Cover,
    /// Stretches the image to the specified length in pixels
    Dimension(Dimension),
}

impl FromDynamic for BackgroundSize {
    fn from_dynamic(
        value: &Value,
        options: FromDynamicOptions,
    ) -> Result<Self, wezterm_dynamic::Error> {
        if let Value::String(label) = value {
            match label.as_str() {
                "Contain" => return Ok(Self::Contain),
                "Cover" => return Ok(Self::Cover),
                _ => {}
            }
        }
        match PixelUnit::from_dynamic(value, options) {
            Ok(pix) => Ok(Self::Dimension(pix.into())),
            Err(_) => Err(wezterm_dynamic::Error::Message(format!(
                "expected either 'Contain', 'Cover', \
                        a number, or a string of \
                        the form '123px' where 'px' is a unit and \
                        can be one of 'px', '%', 'pt' or 'cell', \
                        but got {}",
                value.variant_name()
            ))),
        }
    }
}

impl ToDynamic for BackgroundSize {
    fn to_dynamic(&self) -> Value {
        let s = match self {
            Self::Cover => "Cover".to_string(),
            Self::Contain => "Contain".to_string(),
            Self::Dimension(d) => return d.to_dynamic(),
        };
        Value::String(s)
    }
}

#[derive(Debug, Copy, Clone, FromDynamic, ToDynamic, Default)]
pub enum BackgroundHorizontalAlignment {
    #[default]
    Left,
    Center,
    Right,
}

#[derive(Debug, Copy, Clone, FromDynamic, ToDynamic, Default)]
pub enum BackgroundVerticalAlignment {
    #[default]
    Top,
    Middle,
    Bottom,
}

#[derive(Debug, Copy, Clone, FromDynamic, ToDynamic, PartialEq, Eq, Default)]
pub enum BackgroundRepeat {
    /// Repeat as much as possible to cover the area.
    /// The last image will be clipped if it doesn't fit.
    #[default]
    Repeat,
    /// Like Repeat, except that the image is alternately
    /// mirrored. Helpful when the image doesn't seamlessly
    /// tile.
    Mirror,
    /*
    /// Repeat as much as possible without clipping.
    /// The first and last images are aligned with the edges,
    /// with any gaps being distributed evenly between
    /// the images.
    /// The `position` property is ignored unless only
    /// a single image an be displayed without clipping.
    /// Clipping will only occur when there isn't enough
    /// room to display a single image.
    Space,
    /// As the available space increases, the images will
    /// stretch until there is room (space >= 50% of image
    /// size) for another one to be added. When adding a
    /// new image, the current images compress to allow
    /// room.
    Round,
    */
    /// The image is not repeated.
    /// The position of the image is defined by the
    /// `position` property
    NoRepeat,
}

#[derive(Debug, Copy, Clone, FromDynamic, ToDynamic, Default)]
pub enum BackgroundAttachment {
    #[default]
    Fixed,
    Scroll,
    Parallax(f32),
}

impl BackgroundAttachment {
    #[must_use]
    pub const fn scroll_factor(&self) -> Option<f32> {
        match self {
            Self::Fixed => None,
            Self::Scroll => Some(1.0),
            Self::Parallax(f) => Some(*f),
        }
    }
}

#[derive(Debug, Copy, Clone, FromDynamic, ToDynamic, Default)]
pub enum BackgroundOrigin {
    #[default]
    BorderBox,
    PaddingBox,
}

#[derive(Debug, Copy, Clone, FromDynamic, ToDynamic, PartialEq, Eq, Default)]
pub enum SystemBackdrop {
    #[default]
    Auto,
    Disable,
    Acrylic,
    Mica,
    Tabbed,
}

#[must_use]
pub fn default_win32_acrylic_accent_color() -> RgbaColor {
    SrgbaTuple(0.156863, 0.156863, 0.156863, 0.003922).into()
}

#[derive(Debug, Copy, Clone, FromDynamic, ToDynamic, PartialEq, Eq, Default)]
pub enum Interpolation {
    #[default]
    Linear,
    Basis,
    CatmullRom,
}

#[derive(Debug, Copy, Clone, FromDynamic, ToDynamic, PartialEq, Eq, Default)]
pub enum BlendMode {
    #[default]
    Rgb,
    LinearRgb,
    Hsv,
    Oklab,
}

#[derive(Debug, Copy, Clone, FromDynamic, ToDynamic, PartialEq, Default)]
pub enum GradientOrientation {
    #[default]
    Horizontal,
    Vertical,
    Linear {
        angle: Option<f64>,
    },
    Radial {
        radius: Option<f64>,
        cx: Option<f64>,
        cy: Option<f64>,
    },
}

#[derive(Debug, Copy, Clone, FromDynamic, ToDynamic, PartialEq, Eq)]
pub enum GradientPreset {
    Blues,
    BrBg,
    BuGn,
    BuPu,
    Cividis,
    Cool,
    CubeHelixDefault,
    GnBu,
    Greens,
    Greys,
    Inferno,
    Magma,
    OrRd,
    Oranges,
    PiYg,
    Plasma,
    PrGn,
    PuBu,
    PuBuGn,
    PuOr,
    PuRd,
    Purples,
    Rainbow,
    RdBu,
    RdGy,
    RdPu,
    RdYlBu,
    RdYlGn,
    Reds,
    Sinebow,
    Spectral,
    Turbo,
    Viridis,
    Warm,
    YlGn,
    YlGnBu,
    YlOrBr,
    YlOrRd,
}

impl GradientPreset {
    fn build(self) -> Box<dyn colorgrad::Gradient> {
        match self {
            Self::Blues => Box::new(colorgrad::preset::blues()),
            Self::BrBg => Box::new(colorgrad::preset::br_bg()),
            Self::BuGn => Box::new(colorgrad::preset::bu_gn()),
            Self::BuPu => Box::new(colorgrad::preset::bu_pu()),
            Self::Cividis => Box::new(colorgrad::preset::cividis()),
            Self::Cool => Box::new(colorgrad::preset::cool()),
            Self::CubeHelixDefault => Box::new(colorgrad::preset::cubehelix_default()),
            Self::GnBu => Box::new(colorgrad::preset::gn_bu()),
            Self::Greens => Box::new(colorgrad::preset::greens()),
            Self::Greys => Box::new(colorgrad::preset::greys()),
            Self::Inferno => Box::new(colorgrad::preset::inferno()),
            Self::Magma => Box::new(colorgrad::preset::magma()),
            Self::OrRd => Box::new(colorgrad::preset::or_rd()),
            Self::Oranges => Box::new(colorgrad::preset::oranges()),
            Self::PiYg => Box::new(colorgrad::preset::pi_yg()),
            Self::Plasma => Box::new(colorgrad::preset::plasma()),
            Self::PrGn => Box::new(colorgrad::preset::pr_gn()),
            Self::PuBu => Box::new(colorgrad::preset::pu_bu()),
            Self::PuBuGn => Box::new(colorgrad::preset::pu_bu_gn()),
            Self::PuOr => Box::new(colorgrad::preset::pu_or()),
            Self::PuRd => Box::new(colorgrad::preset::pu_rd()),
            Self::Purples => Box::new(colorgrad::preset::purples()),
            Self::Rainbow => Box::new(colorgrad::preset::rainbow()),
            Self::RdBu => Box::new(colorgrad::preset::rd_bu()),
            Self::RdGy => Box::new(colorgrad::preset::rd_gy()),
            Self::RdPu => Box::new(colorgrad::preset::rd_pu()),
            Self::RdYlBu => Box::new(colorgrad::preset::rd_yl_bu()),
            Self::RdYlGn => Box::new(colorgrad::preset::rd_yl_gn()),
            Self::Reds => Box::new(colorgrad::preset::reds()),
            Self::Sinebow => Box::new(colorgrad::preset::sinebow()),
            Self::Spectral => Box::new(colorgrad::preset::spectral()),
            Self::Turbo => Box::new(colorgrad::preset::turbo()),
            Self::Viridis => Box::new(colorgrad::preset::viridis()),
            Self::Warm => Box::new(colorgrad::preset::warm()),
            Self::YlGn => Box::new(colorgrad::preset::yl_gn()),
            Self::YlGnBu => Box::new(colorgrad::preset::yl_gn_bu()),
            Self::YlOrBr => Box::new(colorgrad::preset::yl_or_br()),
            Self::YlOrRd => Box::new(colorgrad::preset::yl_or_rd()),
        }
    }
}

#[derive(Debug, Clone, FromDynamic, ToDynamic, PartialEq)]
pub struct Gradient {
    #[dynamic(default)]
    pub orientation: GradientOrientation,

    #[dynamic(default)]
    pub colors: Vec<String>,

    #[dynamic(default)]
    pub preset: Option<GradientPreset>,

    #[dynamic(default)]
    pub interpolation: Interpolation,

    #[dynamic(default)]
    pub blend: BlendMode,

    #[dynamic(default)]
    pub segment_size: Option<usize>,

    #[dynamic(default)]
    pub segment_smoothness: Option<f64>,

    #[dynamic(default)]
    pub noise: Option<usize>,
}
impl_lua_conversion_dynamic!(Gradient);

impl Gradient {
    pub fn build(&self) -> anyhow::Result<Box<dyn colorgrad::Gradient>> {
        use colorgrad::BlendMode as CGMode;
        let g: Box<dyn colorgrad::Gradient> = if let Some(p) = &self.preset {
            p.build()
        } else {
            let colors: Vec<&str> = self
                .colors
                .iter()
                .map(std::string::String::as_str)
                .collect();
            let mut g = colorgrad::GradientBuilder::new();
            g.html_colors(&colors);
            g.mode(match self.blend {
                BlendMode::Rgb => CGMode::Rgb,
                BlendMode::LinearRgb => CGMode::LinearRgb,
                BlendMode::Hsv => CGMode::Oklab,
                BlendMode::Oklab => CGMode::Oklab,
            });
            match self.interpolation {
                Interpolation::Linear => Box::new(g.build::<colorgrad::LinearGradient>()?),
                Interpolation::Basis => Box::new(g.build::<colorgrad::BasisGradient>()?),
                Interpolation::CatmullRom => Box::new(g.build::<colorgrad::CatmullRomGradient>()?),
            }
        };
        match (self.segment_size, self.segment_smoothness) {
            (Some(size), Some(smoothness)) => Ok(Box::new(
                g.sharp(size.try_into().unwrap_or(u16::MAX), smoothness as f32),
            )),
            (None, None) => Ok(g),
            _ => anyhow::bail!(
                "Gradient must either specify both segment_size and segment_smoothness, or neither"
            ),
        }
    }
}
