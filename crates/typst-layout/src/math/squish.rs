use typst_library::diag::SourceResult;
use typst_library::foundations::{Packed, StyleChain};
use typst_library::layout::{Abs, Frame, Point, Size};
use typst_library::math::{SquishElem, SquishMode};

use super::{FrameFragment, MathContext};

/// Lays out a [`SquishElem`].
#[typst_macros::time(name = "math.squish", span = elem.span())]
pub fn layout_squish(
    elem: &Packed<SquishElem>,
    ctx: &mut MathContext,
    styles: StyleChain,
) -> SourceResult<()> {
    // Layout child normally.
    let fragment = ctx.layout_into_fragment(&elem.body, styles)?;

    // Preserve class-dependent behavior and text metrics metadata.
    let class = fragment.class();
    let limits = fragment.limits();
    let spaced = fragment.is_spaced();
    let italics = fragment.italics_correction();
    let accent_attach = fragment.accent_attach();
    let text_like = fragment.is_text_like();
    let ignorant = fragment.is_ignorant();

    let child_frame = fragment.into_frame();
    let width = child_frame.width();
    let ascent = child_frame.ascent();
    let height = child_frame.height();
    let descent = height - ascent;

    // New reported metrics + paint shift.
    let mode = elem.mode.get(styles);
    let (new_ascent, new_height, paint_shift_y) = match mode {
        SquishMode::Both => (Abs::zero(), Abs::zero(), -ascent),
        SquishMode::Bottom => (ascent, ascent, Abs::zero()),
        SquishMode::Top => (Abs::zero(), descent, -ascent),
    };

    // Wrapper frame lies about height/ascent, but still paints the child.
    let mut frame = Frame::soft(Size::new(width, new_height));
    frame.set_baseline(new_ascent);
    frame.push_frame(Point::with_y(paint_shift_y), child_frame);

    ctx.push(
        FrameFragment::new(styles, frame)
            .with_class(class)
            .with_limits(limits)
            .with_spaced(spaced)
            .with_italics_correction(italics)
            .with_accent_attach(accent_attach)
            .with_text_like(text_like)
            .with_ignorant(ignorant),
    );

    Ok(())
}
