// Test text decorations.

--- underline-overline-strike ---
#let red = rgb("fc0030")

// Basic strikethrough.
#strike[Statements dreamt up by the utterly deranged.]

// Move underline down.
#underline(offset: 5pt)[Further below.]

// Different color.
#underline(stroke: red, evade: false)[Critical information is conveyed here.]

// Inherits font color.
#text(fill: red, underline[Change with the wind.])

// Both over- and underline.
#overline(underline[Running amongst the wolves.])

--- strike-with ---
#let redact = strike.with(stroke: 10pt, extent: 0.05em)
#let highlight-custom = strike.with(stroke: 10pt + rgb("abcdef88"), extent: 0.05em)

// Abuse thickness and transparency for redacting and highlighting stuff.
Sometimes, we work #redact[in secret].
There might be #highlight-custom[redacted] things.

--- underline-stroke-folding ---
// Test stroke folding.
#set underline(stroke: 2pt, offset: 2pt)
#underline(text(red, [DANGER!]))

--- underline-background ---
// Test underline background
#set underline(background: true, stroke: (thickness: 0.5em, paint: red, cap: "round"))
#underline[This is in the background]

--- overline-background ---
// Test overline background
#set overline(background: true, stroke: (thickness: 0.5em, paint: red, cap: "round"))
#overline[This is in the background]

--- strike-background ---
// Test strike background
#set strike(background: true, stroke: 5pt + red)
#strike[This is in the background]

--- highlight ---
// Test highlight.
This is the built-in #highlight[highlight with default color].
We can also specify a customized value
#highlight(fill: green.lighten(80%))[to highlight].

--- highlight-bounds ---
// Test default highlight bounds.
#highlight[ace],
#highlight[base],
#highlight[super],
#highlight[phone #sym.integral]

--- highlight-edges ---
// Test a tighter highlight.
#set highlight(top-edge: "x-height", bottom-edge: "baseline")
#highlight[ace],
#highlight[base],
#highlight[super],
#highlight[phone #sym.integral]

--- highlight-edges-bounds ---
// Test a bounds highlight.
#set highlight(top-edge: "bounds", bottom-edge: "bounds")
#highlight[abc]
#highlight[abc #sym.integral]

--- highlight-radius ---
// Test highlight radius
#highlight(radius: 3pt)[abc],
#highlight(radius: 1em)[#lorem(5)]

--- highlight-stroke ---
// Test highlight stroke
#highlight(stroke: 2pt + blue)[abc]
#highlight(stroke: (top: blue, left: red, bottom: green, right: orange))[abc]
#highlight(stroke: 1pt, radius: 3pt)[#lorem(5)]

--- highlight-math-inline ---
// Test highlight for inline math with mixed content and wrapping.
#set page(width: 140pt, height: auto, margin: 8pt)
#block(width: 92pt)[#highlight[Words before $x + y^2$ and after, with wrapping.]]
#highlight[$ integral_0^1 x dif x $]

--- highlight-math-display ---
// Test highlight for display math with non-default edges, extent, and radius.
#set page(width: 140pt, height: auto, margin: 8pt)
#set highlight(top-edge: "bounds", bottom-edge: "bounds", extent: 1.2pt, radius: 2pt)
#highlight[$ x^2 + y^2 = z^2 $]
#highlight[$ "sdlfjdsk" + x + y $]
#highlight[
  Before.
  $ "sdlfjdsk" + sum_(i=1)^n i = n(n+1)/2 $
  After.
]

--- html-deco html ---
#strike[Struck]
#highlight[Highlighted]
#underline[Underlined]
#overline[Overlined]

#(strike, highlight, underline, overline).fold([Mixed], (it, f) => f(it))
