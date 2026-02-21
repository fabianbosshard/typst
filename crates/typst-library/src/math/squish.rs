use crate::diag::bail;
use crate::foundations::{Content, cast, elem};
use crate::layout::VAlignment;
use crate::math::Mathy;

/// Squish content vertically in math.
///
/// This behaves similarly to TeX's `\\smash`, `\\smash[t]`, and
/// `\\smash[b]`.
#[elem(Mathy)]
pub struct SquishElem {
    /// The content to squish.
    #[required]
    pub body: Content,

    /// Which side to squish.
    ///
    /// - `"both"`: squish height and depth.
    /// - `"t"`/`"top"`: squish height (keep depth).
    /// - `"b"`/`"bottom"`: squish depth (keep height).
    ///
    /// Can be given as either a second positional argument or as
    /// `mode: ...`.
    #[parse(args.named_or_find("mode")?)]
    #[default(SquishMode::Both)]
    pub mode: SquishMode,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum SquishMode {
    Both,
    Top,
    Bottom,
}

cast! {
    SquishMode,
    self => crate::foundations::IntoValue::into_value(match self {
        Self::Both => "both",
        Self::Top => "top",
        Self::Bottom => "bottom",
    }),
    "both" => Self::Both,
    "t" => Self::Top,
    "top" => Self::Top,
    "b" => Self::Bottom,
    "bottom" => Self::Bottom,
    v: VAlignment => match v {
        VAlignment::Top => Self::Top,
        VAlignment::Bottom => Self::Bottom,
        VAlignment::Horizon => bail!("expected `top` or `bottom`"),
    },
}
