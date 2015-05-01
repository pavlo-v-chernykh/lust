(def a 1)
(def b 2)
(def k :some-keyword)
(def l '(:1 :2 :3 :4))
(def v [1 2 3 4])

(+ a b (+ 3 b))
(- a b (- 3 b))
(* b 1 (* 10 a))
(/ a 10 3 b)

(> 10 a)
(< 10 a)
(= 10 a)

(if (< a b)
    (+ a b)
    (- b a))

(def m
    (macro [a]
        '(+ 1 ~a)))
(def f (fn [b] (m b)))

(= (m 3) 4)
(= (f 3) 4)

(let [c (+ a b) d 80]
    (+ a b c d))

(def when                           ;; comments
    (macro [test body]              ;; comments
        '(if ~test ~body nil)))     ;; comments

(when (> a 10)
    (+ a b))

(def l '(1 2 3 4))

'(+ ~@l 5)

(in-ns "other")

(def a 1)

(def m (macro [b] `(+ a ~b)))

(in-ns "user")

(other/m 3)
