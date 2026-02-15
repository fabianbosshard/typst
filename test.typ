#let matr(x) = math.bold(math.underline-smash(x))

$ matr(A) = matr(Q) matr(R) $
$ underline(bold(A)) = underline(bold(Q)) underline(bold(R)) $
$ bold(A) = bold(Q) bold(R) $

$ e^(matr(A)) != e^(matr(Q)) e^(matr(R)) $
$ e^(underline(bold(A))) != e^(underline(bold(Q))) e^(underline(bold(R))) $
$ e^(bold(A)) != e^(bold(Q)) e^(bold(R)) $

$ matr(A) matr(W) matr(Q) matr(R) matr(I) $
$ e^(matr(A)) e^(matr(Q) matr(W) matr(R) matr(I)) $
// $e^(e^matr(A))$
$ e^(e^matr(A)) e^(e^(matr(Q) matr(W) matr(R) matr(I))) $
// $e^(e^(e^matr(A)))$
$ e^(e^(e^matr(A))) e^(e^(e^(matr(Q) matr(W) matr(R) matr(I)))) $

$ matr(A) = matr(W) matr(Q) $
$ matr(A) = matr(Q) matr(R) $
$ matr(A) = matr(I) matr(I) $
// TeX: 1mu = 1/18 em (in math). So 0.8mu = 0.8/18 em.






// compare vertical spacing correctness
$
matr(A)_(matr(A)_matr(A))^matr(A)^matr(A) // subscripts work correctly
bold(A)_(bold(A)_bold(A))^bold(A)^bold(A) // subscripts work correctly
$



// compare horizontal spacing correctness
$
matr(A)_(matr(A)_matr(A))^matr(A)^matr(A) // subscripts work correctly
$
$
bold(A)_(bold(A)_bold(A))^bold(A)^bold(A) // subscripts work correctly
$



// compare dfferences between the approaches
$ 
matr(Q)_(matr(Q)_matr(Q))^matr(Q)^matr(Q) // subscripts do not work correctly
matr(A)_(matr(A)_matr(A))^matr(A)^matr(A) // subscripts work correctly
matr(q)_(matr(q)_matr(q))^matr(q)^matr(q) // subscripts do not work correctly
bold(q)_(bold(q)_bold(q))^bold(q)^bold(q) 
bold(A)_(bold(A)_bold(A))^bold(A)^bold(A) 
bold(Q)_(bold(Q)_bold(Q))^bold(Q)^bold(Q) // subscripts work correctly
bold(underline(Q))_(bold(underline(Q))_bold(underline(Q)))^bold(underline(Q))^bold(underline(Q)) // subscripts work correctly (in terms of underline philosphy)
$


// compare vertical spacing correctness
$
matr(Q)_(matr(Q)_matr(Q))^matr(Q)^matr(Q) // subscripts do not work correctly
bold(Q)_(bold(Q)_bold(Q))^bold(Q)^bold(Q) // subscripts work correctly
$



// compare horizontal spacing correctness
$
matr(Q)_(matr(Q)_matr(Q))^matr(Q)^matr(Q) // subscripts do not work correctly
$
$
bold(Q)_(bold(Q)_bold(Q))^bold(Q)^bold(Q) // subscripts work correctly
$

