use std::str::FromStr;

use comemo::Track;
use smallvec::{SmallVec, smallvec};

use crate::diag::{SourceResult, bail, warning};
use crate::engine::Engine;
use crate::foundations::{
    Array, Content, Context, NativeElement, Packed, Reflect, Smart, StyleChain, Styles,
    Synthesize, cast, elem, scope,
};
use crate::introspection::{Counter, Locatable, Tagged};
use crate::layout::{Alignment, Em, HAlignment, Length, VAlignment};
use crate::model::{ListItemLike, ListLike, Numbering, NumberingPattern, Refable};
use crate::text::TextElem;

/// A numbered list.
///
/// Displays a sequence of items vertically and numbers them consecutively.
///
/// # Example
/// ```example
/// Automatically numbered:
/// + Preparations
/// + Analysis
/// + Conclusions
///
/// Manually numbered:
/// 2. What is the first step?
/// 5. I am confused.
/// +  Moving on ...
///
/// Multiple lines:
/// + This enum item has multiple
///   lines because the next line
///   is indented.
///
/// Function call.
/// #enum[First][Second]
/// ```
///
/// You can easily switch all your enumerations to a different numbering style
/// with a set rule.
/// ```example
/// #set enum(numbering: "a)")
///
/// + Starting off ...
/// + Don't forget step two
/// ```
///
/// You can also use [`enum.item`] to programmatically customize the number of
/// each item in the enumeration:
///
/// ```example
/// #enum(
///   enum.item(1)[First step],
///   enum.item(5)[Fifth step],
///   enum.item(10)[Tenth step]
/// )
/// ```
///
/// # Syntax
/// This functions also has dedicated syntax:
///
/// - Starting a line with a plus sign creates an automatically numbered
///   enumeration item.
/// - Starting a line with a number followed by a dot creates an explicitly
///   numbered enumeration item.
///
/// Enumeration items can contain multiple paragraphs and other block-level
/// content. All content that is indented more than an item's marker becomes
/// part of that item.
#[elem(scope, title = "Numbered List", Locatable, Tagged, Synthesize)]
pub struct EnumElem {
    /// Defines the default [spacing]($enum.spacing) of the enumeration. If it
    /// is `{false}`, the items are spaced apart with
    /// [paragraph spacing]($par.spacing). If it is `{true}`, they use
    /// [paragraph leading]($par.leading) instead. This makes the list more
    /// compact, which can look better if the items are short.
    ///
    /// In markup mode, the value of this parameter is determined based on
    /// whether items are separated with a blank line. If items directly follow
    /// each other, this is set to `{true}`; if items are separated by a blank
    /// line, this is set to `{false}`. The markup-defined tightness cannot be
    /// overridden with set rules.
    ///
    /// ```example
    /// + If an enum has a lot of text, and
    ///   maybe other inline content, it
    ///   should not be tight anymore.
    ///
    /// + To make an enum wide, simply
    ///   insert a blank line between the
    ///   items.
    /// ```
    #[default(true)]
    pub tight: bool,

    /// How to number the enumeration. Accepts a
    /// [numbering pattern or function]($numbering).
    ///
    /// If the numbering pattern contains multiple counting symbols, they apply
    /// to nested enums. If given a function, the function receives one argument
    /// if `full` is `{false}` and multiple arguments if `full` is `{true}`.
    ///
    /// ```example
    /// #set enum(numbering: "1.a)")
    /// + Different
    /// + Numbering
    ///   + Nested
    ///   + Items
    /// + Style
    ///
    /// #set enum(numbering: n => super[#n])
    /// + Superscript
    /// + Numbering!
    /// ```
    #[default(Numbering::Pattern(NumberingPattern::from_str("1.").unwrap()))]
    pub numbering: Numbering,

    /// Which number to start the enumeration with.
    ///
    /// ```example
    /// #enum(
    ///   start: 3,
    ///   [Skipping],
    ///   [Ahead],
    /// )
    /// ```
    pub start: Smart<u64>,

    /// Whether to display the full numbering, including the numbers of
    /// all parent enumerations.
    ///
    ///
    /// ```example
    /// #set enum(numbering: "1.a)", full: true)
    /// + Cook
    ///   + Heat water
    ///   + Add ingredients
    /// + Eat
    /// ```
    #[default(false)]
    pub full: bool,

    /// Whether to reverse the numbering for this enumeration.
    ///
    /// ```example
    /// #set enum(reversed: true)
    /// + Coffee
    /// + Tea
    /// + Milk
    /// ```
    #[default(false)]
    pub reversed: bool,

    /// The indentation of each item.
    pub indent: Length,

    /// The space between the numbering and the body of each item.
    #[default(Em::new(0.5).into())]
    pub body_indent: Length,

    /// The spacing between the items of the enumeration.
    ///
    /// If set to `{auto}`, uses paragraph [`leading`]($par.leading) for tight
    /// enumerations and paragraph [`spacing`]($par.spacing) for wide
    /// (non-tight) enumerations.
    pub spacing: Smart<Length>,

    /// The alignment that enum numbers should have.
    ///
    /// By default, this is set to `{end + top}`, which aligns enum numbers
    /// towards end of the current text direction (in left-to-right script,
    /// for example, this is the same as `{right}`) and at the top of the line.
    /// The choice of `{end}` for horizontal alignment of enum numbers is
    /// usually preferred over `{start}`, as numbers then grow away from the
    /// text instead of towards it, avoiding certain visual issues. This option
    /// lets you override this behaviour, however. (Also to note is that the
    /// [unordered list]($list) uses a different method for this, by giving the
    /// `marker` content an alignment directly.).
    ///
    /// ````example
    /// #set enum(number-align: start + bottom)
    ///
    /// Here are some powers of two:
    /// 1. One
    /// 2. Two
    /// 4. Four
    /// 8. Eight
    /// 16. Sixteen
    /// 32. Thirty two
    /// ````
    #[default(HAlignment::End + VAlignment::Top)]
    pub number_align: Alignment,

    /// The numbered list's items.
    ///
    /// When using the enum syntax, adjacent items are automatically collected
    /// into enumerations, even through constructs like for loops.
    ///
    /// ```example
    /// #for phase in (
    ///    "Launch",
    ///    "Orbit",
    ///    "Descent",
    /// ) [+ #phase]
    /// ```
    #[variadic]
    #[parse(
        for item in args.items.iter() {
            if item.name.is_none() && Array::castable(&item.value.v) {
                engine.sink.warn(warning!(
                    item.value.span,
                    "implicit conversion from array to `enum.item` is deprecated";
                    hint: "use `enum.item(number)[body]` instead";
                    hint: "this conversion was never documented and is being phased out"
                ));
            }
        }
        args.all()?
    )]
    pub children: Vec<Packed<EnumItem>>,

    /// The numbers of parent items.
    #[internal]
    #[fold]
    #[ghost]
    pub parents: SmallVec<[u64; 4]>,
}

#[scope]
impl EnumElem {
    #[elem]
    type EnumItem;
}

/// An enumeration item.
#[elem(name = "item", title = "Numbered List Item", Locatable, Tagged, Refable)]
pub struct EnumItem {
    /// The item's number.
    #[positional]
    pub number: Smart<u64>,

    /// The item's body.
    #[required]
    pub body: Content,

    /// The fully resolved numbering, including all parent numbers.
    #[internal]
    #[synthesized]
    pub resolved: SmallVec<[u64; 4]>,

    /// The numbering style used at the item's location.
    #[internal]
    #[synthesized]
    pub resolved_numbering: Numbering,

    /// Whether references to this item should display full numbering.
    #[internal]
    #[synthesized]
    pub resolved_full: bool,
}

cast! {
    EnumItem,
    array: Array => {
        let mut iter = array.into_iter();
        let (number, body) = match (iter.next(), iter.next(), iter.next()) {
            (Some(a), Some(b), None) => (a.cast()?, b.cast()?),
            _ => bail!("array must contain exactly two entries"),
        };
        Self::new(body).with_number(number)
    },
    v: Content => v.unpack::<Self>().unwrap_or_else(Self::new),
}

/// Format an enum item's already resolved numbering.
pub fn display_enum_numbering(
    engine: &mut Engine,
    styles: StyleChain,
    numbering: &Numbering,
    resolved: &[u64],
    full: bool,
) -> SourceResult<Content> {
    let Some(&last) = resolved.last() else {
        return Ok(Content::empty());
    };

    // Enum references intentionally use the same untrimmed formatting as enum
    // markers so both render identically.
    let context = Context::new(None, Some(styles));

    if full {
        return Ok(numbering.apply(engine, context.track(), resolved)?.display());
    }

    Ok(match numbering {
        Numbering::Pattern(pattern) => {
            TextElem::packed(pattern.apply_kth(resolved.len() - 1, last))
        }
        other => other.apply(engine, context.track(), &[last])?.display(),
    })
}

impl Synthesize for Packed<EnumElem> {
    fn synthesize(&mut self, _: &mut Engine, styles: StyleChain) -> SourceResult<()> {
        let numbering = self.numbering.get_ref(styles).clone();
        let full = self.full.get(styles);
        let reversed = self.reversed.get(styles);
        let parents = styles.get_cloned(EnumElem::parents);

        let mut number = self
            .start
            .get(styles)
            .unwrap_or_else(|| if reversed { self.children.len() as u64 } else { 1 });

        for item in &mut self.children {
            number = item.number.get(styles).unwrap_or(number);

            let mut resolved = parents.clone();
            resolved.push(number);

            let item = item.as_mut();
            let set_parents = item.resolved.is_none();
            item.resolved = Some(resolved);
            item.resolved_numbering = Some(numbering.clone());
            item.resolved_full = Some(full);
            if set_parents {
                item.body = item.body.clone().set(EnumElem::parents, smallvec![number]);
            }

            number = if reversed {
                number.saturating_sub(1)
            } else {
                number.saturating_add(1)
            };
        }

        Ok(())
    }
}

impl ListLike for EnumElem {
    type Item = EnumItem;

    fn create(children: Vec<Packed<Self::Item>>, tight: bool) -> Self {
        Self::new(children).with_tight(tight)
    }
}

impl ListItemLike for EnumItem {
    fn styled(mut item: Packed<Self>, styles: Styles) -> Packed<Self> {
        item.body.style_in_place(styles);
        item
    }
}

impl Refable for Packed<EnumItem> {
    fn supplement(&self) -> Content {
        Content::empty()
    }

    fn counter(&self) -> Counter {
        // Enum items format references from their pre-resolved numbering in
        // `reference_number`. This counter is only used by the fallback path.
        Counter::of(EnumElem::ELEM)
    }

    fn numbering(&self) -> Option<&Numbering> {
        self.resolved_numbering.as_ref()
    }

    fn reference_number(
        &self,
        engine: &mut Engine,
        styles: StyleChain,
    ) -> SourceResult<Option<Content>> {
        let (Some(numbering), Some(resolved), Some(full)) = (
            self.resolved_numbering.as_ref(),
            self.resolved.as_ref(),
            self.resolved_full,
        ) else {
            return Ok(None);
        };

        Ok(Some(display_enum_numbering(engine, styles, numbering, resolved, full)?))
    }
}
