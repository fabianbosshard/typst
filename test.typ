
// this seems to work now
#let matr(x) = context {
  let em_abs = measure(math.inline[box(width: 1em)]).width.to-absolute()
  let mu = em_abs / 18
  let matrspace = 0.1 * mu

  math.class("normal",
    math.squish(h(matrspace)
    + math.underline(
        math.squish(h(-matrspace) + math.bold(x) + h(-matrspace), mode: "bottom")
      )
    + h(matrspace), mode: "bottom")
  )
}

$ matr(A) = matr(Q) matr(R) bold(x) $
$ underline(bold(A)) = underline(bold(Q)) underline(bold(R)) bold(x) $
$ bold(A) = bold(Q) bold(R) bold(x) $

$ e^(matr(A)) != e^(matr(Q)) e^(matr(R)) $
$ e^(underline(bold(A))) != e^(underline(bold(Q))) e^(underline(bold(R))) $
$ e^(bold(A)) != e^(bold(Q)) e^(bold(R)) $

$ matr(A)_1 matr(W) matr(Q)_1 matr(R) matr(I) $ 
$ A_1 matr(W) Q_1 matr(R) matr(I) $ 
$ e^(matr(A)) e^(matr(Q) matr(W) matr(R) matr(I)) $ 
$ e^(e^matr(A)) e^(e^(matr(Q) matr(W) matr(R) matr(I))) $ 
$ e^e^bold(A) e^(e^(bold(Q) bold(W) bold(R) bold(I))) $ 


$ matr(A) = matr(W) matr(Q) $
$ matr(A) = matr(Q) matr(R) $
$ matr(A) = matr(I) matr(I) $


$ tilde(matr(chi))_1^((1)) tilde(chi)_1^((1)) tilde(b)_1^((1)) $
$ matr(chi)_1^((1)) chi_1^((1)) b_1^((1)) $


$ matr(A)_matr(A)_matr(A) bold(underline(A))_bold(underline(A))_bold(underline(A)) bold(A)_bold(A)_bold(A) $


