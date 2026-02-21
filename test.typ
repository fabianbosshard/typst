
// this seems to work now
#let matr(x) = context {
  let em_abs = measure(math.inline[box(width: 1em)]).width.to-absolute()
  let mu = em_abs / 18
  let matrspace = 0.1 * mu

  math.class("normal",
    math.squish(
    h(matrspace)
    + math.underline(
        math.squish(h(-matrspace) + math.bold(x) + h(-matrspace), mode: "bottom")
      )
    + h(matrspace)
    , mode: "bottom")
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




it seems to work...

but it doesnt...


#let matr1(x) = context {
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

#let matr2(x) = context {
  let em_abs = measure(math.inline[box(width: 1em)]).width.to-absolute()
  let mu = em_abs / 18
  let matrspace = 0.1 * mu

  math.class("normal", math.squish(
    h(matrspace)
    + math.underline(
        math.squish(h(-matrspace) + math.bold(x) + h(-matrspace), mode: "bottom")
      )
    + h(matrspace), mode: "bottom")
  )
}



// compare dfferences between the approaches
$ 
matr1(A)_(matr1(A)_matr1(A))^matr1(A)^matr1(A) // subscripts do not work correctly
matr2(A)_(matr2(A)_matr2(A))^matr2(A)^matr2(A) // subscripts work correctly
bold(A)_(bold(A)_bold(A))^bold(A)^bold(A) // subscripts work correctly
bold(underline(A))_(bold(underline(A))_bold(underline(A)))^bold(underline(A))^bold(underline(A)) // subscripts work correctly (in terms of underline philosphy)
$


// compare vertical spacing correctness
$
matr2(A)_(matr2(A)_matr2(A))^matr2(A)^matr2(A) // subscripts work correctly
bold(A)_(bold(A)_bold(A))^bold(A)^bold(A) // subscripts work correctly
$



// compare horizontal spacing correctness
$
matr2(A)_(matr2(A)_matr2(A))^matr2(A)^matr2(A) // subscripts work correctly
$
$
bold(A)_(bold(A)_bold(A))^bold(A)^bold(A) // subscripts work correctly
$






// compare dfferences between the approaches
$ 
matr1(Q)_(matr1(Q)_matr1(Q))^matr1(Q)^matr1(Q) // subscripts do not work correctly
matr2(Q)_(matr2(Q)_matr2(Q))^matr2(Q)^matr2(Q) // subscripts do not work correctly
matr2(A)_(matr2(A)_matr2(A))^matr2(A)^matr2(A) // subscripts work correctly
matr2(q)_(matr2(q)_matr2(q))^matr2(q)^matr2(q) // subscripts do not work correctly
bold(q)_(bold(q)_bold(q))^bold(q)^bold(q) 
bold(A)_(bold(A)_bold(A))^bold(A)^bold(A) 
bold(Q)_(bold(Q)_bold(Q))^bold(Q)^bold(Q) // subscripts work correctly
bold(underline(Q))_(bold(underline(Q))_bold(underline(Q)))^bold(underline(Q))^bold(underline(Q)) // subscripts work correctly (in terms of underline philosphy)
$


// compare vertical spacing correctness
$
matr2(Q)_(matr2(Q)_matr2(Q))^matr2(Q)^matr2(Q) // subscripts do not work correctly
bold(Q)_(bold(Q)_bold(Q))^bold(Q)^bold(Q) // subscripts work correctly
$



// compare horizontal spacing correctness
$
matr2(Q)_(matr2(Q)_matr2(Q))^matr2(Q)^matr2(Q) // subscripts do not work correctly
$
$
bold(Q)_(bold(Q)_bold(Q))^bold(Q)^bold(Q) // subscripts work correctly
$





+ hello
+ hello <tag>
+ now it 


@tag works now, great


fdfkj


fdsljfdflsdfkjj

$
  matr(A) = matr(Q)_1 matr(R)_1 bold(Q)_1
$

