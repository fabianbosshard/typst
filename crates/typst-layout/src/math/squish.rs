use bumpalo::boxed::Box as BumpBox;
use typst_library::diag::SourceResult;
use typst_library::foundations::StyleChain;
use typst_library::layout::{Abs, Frame, Point, Size};
use typst_library::math::SquishMode;
use typst_library::math::ir::{MathProperties, SquishItem};

use super::MathContext;
use super::fragment::FrameFragment;

/// Lays out a [`SquishItem`].
#[typst_macros::time(name = "math squish layout", span = props.span)]
pub(super) fn layout_squish(
    item: &BumpBox<SquishItem>,
    ctx: &mut MathContext,
    styles: StyleChain,
    props: &MathProperties,
) -> SourceResult<()> {
    // Layout child normally.
    let fragment = ctx.layout_into_fragment(&item.base, styles)?;

    // Preserve these; squish should not change them.
    let italics = fragment.italics_correction();
    let accent_attach = fragment.accent_attach();
    let text_like = fragment.is_text_like();

    let child_frame = fragment.into_frame();
    let width = child_frame.width();
    let ascent = child_frame.ascent();
    let height = child_frame.height();
    let descent = height - ascent;

    // New reported metrics + paint shift.
    let (new_ascent, new_height, paint_shift_y) = match item.mode {
        SquishMode::Both => (Abs::zero(), Abs::zero(), -ascent),
        SquishMode::Bottom => (ascent, ascent, Abs::zero()),
        SquishMode::Top => (Abs::zero(), descent, -ascent),
    };

    // Wrapper frame lies about height/ascent, but still paints the child.
    let mut frame = Frame::soft(Size::new(width, new_height));
    frame.set_baseline(new_ascent);
    frame.push_frame(Point::with_y(paint_shift_y), child_frame);

    ctx.push(
        FrameFragment::new(props, styles, frame)
            .with_italics_correction(italics)
            .with_accent_attach(accent_attach)
            .with_text_like(text_like),
    );

    Ok(())
}
