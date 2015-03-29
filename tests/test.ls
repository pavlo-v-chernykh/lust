(def a 1)
(def b 2)
(+ a b (- 3 b))
(/ a 10 3 b)
(* b 1 (- 10 a))
(def m
    (macro (a)
        '(+ 1 ~a)))
(def f (fn (b) (m b)))
