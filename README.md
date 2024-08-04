# rust_lisp
rewrite a [lisp interpreter](https://github.com/gpgkd906/PhpLisp2) in rust， for fun.

## test 
```lisp
lisp:> (setf lst '(a b c))
(a b c)
lisp:> lst
(a b c)
lisp:> (car lst)
a
lisp:> (cdr lst)
(b c)
lisp:> (cons 'x lst)
(x a b c)
lisp:> (+ 1 2 3)
6
lisp:> (- 10 3 2)
5
lisp:> (* 2 3 4)
24
lisp:> (/ 20 2)
10
lisp:> (cond ((> 5 4) 'yes) ((> 3 5) 'no))
yes
```

## todo...

- [] add macro support (defmacro)
- [] add function support (defun)
- [] add lambda support (lambda)
- [] add test...