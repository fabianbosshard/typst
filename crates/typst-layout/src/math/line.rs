use typst_library::diag::SourceResult;
use typst_library::foundations::{Resolve, StyleChain};
use typst_library::layout::{Abs, Frame, FrameItem, Point, Size};
use typst_library::math::ir::{LineItem, MathProperties, Position};
use typst_library::math::MathSize;
use typst_library::text::TextElem;
use typst_library::visualize::{FixedStroke, Geometry};

use super::MathContext;
use super::fragment::FrameFragment;

/// Lays out a [`LineItem`].
#[typst_macros::time(name = "math line layout", span = props.span)]
pub fn layout_line(
    item: &LineItem,
    ctx: &mut MathContext,
    styles: StyleChain,
    props: &MathProperties,
) -> SourceResult<()> {
    let (
        extra_height,
        content,
        mut line_pos,
        content_pos,
        baseline,
        thickness,
        line_adjust,
        font_size,
    );
    match item.position {
        Position::Below => {
            content = ctx.layout_into_fragment(&item.base, styles)?;

            let (font, size) = content.font(ctx, item.base.styles().unwrap_or(styles));
            font_size = size;
            let sep = font.math().underbar_extra_descender.at(size);
            thickness = font.math().underbar_rule_thickness.at(size);
            let gap = font.math().underbar_vertical_gap.at(size);
            extra_height = sep + thickness + gap;

            let y_base = if item.smash { content.ascent() } else { content.height() };
            line_pos = Point::with_y(y_base + gap + thickness / 2.0);
            content_pos = Point::zero();
            baseline = content.ascent();
            line_adjust = if item.smash {
                    Abs::zero()
                } else {
                    -content.italics_correction()
                };
        }
        Position::Above => {
            content = ctx.layout_into_fragment(&item.base, styles)?;

            let (font, size) = content.font(ctx, item.base.styles().unwrap_or(styles));
            font_size = size;
            let sep = font.math().overbar_extra_ascender.at(size);
            thickness = font.math().overbar_rule_thickness.at(size);
            let gap = font.math().overbar_vertical_gap.at(size);
            extra_height = sep + thickness + gap;

            line_pos = Point::with_y(sep + thickness / 2.0);
            content_pos = Point::with_y(extra_height);
            baseline = content.ascent() + extra_height;
            line_adjust = Abs::zero();
        }
    }

    let width = content.width();
    let height = match item.position {
        Position::Below if item.smash => baseline + extra_height,
        _ => content.height() + extra_height,
    };
    let size = Size::new(width, height);
    let line_width = width + line_adjust;
    let (drawn_width, line_x) = if item.smash && matches!(item.position, Position::Below) {
        let mu = font_size / 18.0;
        // let mu = styles.resolve(TextElem::size) / 18.0;

        let size_boost = match props.size { // to make sure in scriptsizes the line is not too wide
            MathSize::Display | MathSize::Text => 1.0,
            MathSize::Script => 1.0/0.7,        // tune
            MathSize::ScriptScript => 1.0/0.5,  // tune
        };

        let trim = 0.8 * mu * size_boost;
        ((line_width - 2.0 * trim).max(Abs::zero()), trim)
    } else {
        (line_width, Abs::zero())
    };

    let content_text_like = content.is_text_like();
    let content_italics_correction = content.italics_correction();
    let mut frame = Frame::soft(size);
    frame.set_baseline(baseline);
    frame.push_frame(content_pos, content.into_frame());

    let text_fill = styles.get_ref(TextElem::fill).as_decoration();
    let line = match styles.get_ref(TextElem::stroke) {
        Some(stroke) => Geometry::Rect(Size::new(drawn_width, thickness))
            .filled_and_stroked(
                text_fill.clone(),
                stroke.clone().resolve(styles).unwrap_or_default(),
            ),
        None => Geometry::Line(Point::with_x(drawn_width))
            .stroked(FixedStroke::from_pair(text_fill, thickness)),
    };
    line_pos = Point::new(line_pos.x + line_x, line_pos.y);
    frame.push(line_pos, FrameItem::Shape(line, props.span));

    ctx.push(
        FrameFragment::new(props, styles, frame)
            .with_italics_correction(content_italics_correction)
            .with_text_like(content_text_like),
    );
    Ok(())
}
