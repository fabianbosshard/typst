// #let matr(x) = math.bold(math.underline(math.squish(x, bottom)))
// #let matr(x) = math.bold(math.underline(x))



/*
#let matrspace = 0.4pt
#let matr(x) = $ #h(matrspace) underline(squish(#h(-matrspace) #x #h(-matrspace), "bottom")) #h(matrspace) $ // desired behavior, except for the absence of squish and the swallowing of space
*/




/*
#let matr(x) = context {
  // force “inline/text math size” for the measurement
  let em_abs = measure(math.inline[box(width: 1em)]).width.to-absolute()
  let mu = em_abs / 18
  let trim = mu * 0.1

  $ #h(trim)
    underline(
      squish(#h(-trim) bold(#x) #h(-trim), "bottom")
    )
    #h(trim) $
}
*/


/*
#let matrspace = 0.01em

#let matr(x) = math.class("normal",
    h(matrspace) +
    math.underline(
      math.squish(
        h(-matrspace) + math.bold(x) + h(-matrspace),
        mode: "bottom",
      )
    ) +
    h(matrspace)
  )
*/



// this seems to work now
#let matr(x) = context {
  let em_abs = measure(math.inline[box(width: 1em)]).width.to-absolute()
  let mu = em_abs / 18
  let matrspace = 0.1 * mu

  math.class("normal",
    h(matrspace)
    + math.underline(
        math.squish(h(-matrspace) + math.bold(x) + h(-matrspace), mode: "bottom")
      )
    + h(matrspace)
  )
}

$ matr(A) = matr(Q) matr(R) bold(x) $
$ underline(bold(A)) = underline(bold(Q)) underline(bold(R)) bold(x) $
$ bold(A) = bold(Q) bold(R) bold(x) $

$ e^(matr(A)) != e^(matr(Q)) e^(matr(R)) $
$ e^(underline(bold(A))) != e^(underline(bold(Q))) e^(underline(bold(R))) $
$ e^(bold(A)) != e^(bold(Q)) e^(bold(R)) $

$ matr(A) matr(W) matr(Q) matr(R) matr(I) $ 
$ e^(matr(A)) e^(matr(Q) matr(W) matr(R) matr(I)) $ 
$ e^(e^matr(A)) e^(e^(matr(Q) matr(W) matr(R) matr(I))) $ 
$ e^e^bold(A) e^(e^(bold(Q) bold(W) bold(R) bold(I))) $ 


$ matr(A) = matr(W) matr(Q) $
$ matr(A) = matr(Q) matr(R) $
$ matr(A) = matr(I) matr(I) $
