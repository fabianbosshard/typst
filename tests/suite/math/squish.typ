// Test squish (TeX-like smash) in math.

--- math-squish-code-call eval ---
#test(type(math.squish($A$)), content)
#test(type(math.squish($A$, "bottom")), content)
#test(type(math.squish($A$, "top")), content)
#test(type(math.squish($A$, "both")), content)
#test(type(math.squish($A$, mode: "top")), content)
#test(type(math.squish($A$, mode: "bottom")), content)

--- math-squish-code-call-mode-aliases eval ---
#test(type(math.squish($A$, "t")), content)
#test(type(math.squish($A$, "b")), content)
#test(type(math.squish($A$, top)), content)
#test(type(math.squish($A$, bottom)), content)

--- math-squish-math-call eval ---
#test(type($squish(A)$), content)
#test(type($squish(A, "bottom")$), content)
#test(type($squish(A, "top")$), content)
#test(type($squish(A, "t")$), content)
#test(type($squish(A, "b")$), content)
#test(type($squish(A, #top)$), content)
#test(type($squish(A, #bottom)$), content)

--- math-squish-set-rule eval ---
#let _ = {
  set math.squish(mode: "bottom")
  assert(type($squish(A)$) == content)
  assert(type($squish(A, "top")$) == content)
}

--- math-squish-measure-width paged empty ---
// Squishing should only affect vertical metrics, not width.
#context {
  let normal = measure($g$).width
  assert.eq(measure($squish(g)$).width, normal)
  assert.eq(measure($squish(g, "top")$).width, normal)
  assert.eq(measure($squish(g, "bottom")$).width, normal)
}

--- math-squish-measure-height paged empty ---
// All squish modes should reduce reported height for a glyph with descent.
#context {
  let normal = measure(text(top-edge: "bounds", bottom-edge: "bounds", $g$)).height
  let both = measure(text(top-edge: "bounds", bottom-edge: "bounds", $squish(g)$)).height
  let top = measure(text(top-edge: "bounds", bottom-edge: "bounds", $squish(g, "top")$)).height
  let bottom = measure(text(top-edge: "bounds", bottom-edge: "bounds", $squish(g, "bottom")$)).height

  assert(both < normal)
  assert(top < normal)
  assert(bottom < normal)
  assert(both < top)
  assert(both < bottom)
}

--- math-squish-measure-set-rule paged empty ---
// Set rule should affect default mode and explicit mode should override it.
#set math.squish(mode: "bottom")
#context {
  let implicit = measure(text(top-edge: "bounds", bottom-edge: "bounds", $squish(g)$)).height
  let explicit = measure(text(top-edge: "bounds", bottom-edge: "bounds", $squish(g, "bottom")$)).height
  let override = measure(text(top-edge: "bounds", bottom-edge: "bounds", $squish(g, "top")$)).height
  assert.eq(implicit, explicit)
  assert(override != explicit)
}

--- math-squish-subscript paged ---
// Squishing changes metrics used by scripts.
$g_2 != squish(g)_2 != squish(g, "top")_2 != squish(g, "bottom")_2$

--- math-squish-superscript paged ---
// Squishing changes metrics used by superscripts.
$g^2 != squish(g)^2 != squish(g, "top")^2 != squish(g, "bottom")^2$

--- math-squish-under-overline paged ---
// Interactions with line elements.
$underline(g) != underline(squish(g)) != underline(squish(g, "top")) \
 overline(g) != overline(squish(g)) != overline(squish(g, "bottom"))$
