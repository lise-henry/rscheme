;; a short form for defining functions
(defmacro defn (name args body)
  `(def ,name   (lambda ,name ,args
                        ,body)))



(defn car (xs)
  (_car xs))

(defn cdr (xs)
  (_cdr xs))

(defn cadr (xs)
  (car (cdr xs)))

(defn cddr (xs)
  (cdr (cdr xs)))

(defn cons (x xs)
  (_cons x xs))

(defn apply (f args)
  (eval (cons f args)))

(defn = (x y)
  (_= x y))

(defn nil? (xs)
  (_nil? xs))

(defn list? (xs)
  (_list? xs))

(defn lambda? (xs)
  (_lambda? xs))

(defn integer? (xs)
  (_integer? xs))

(defn float? (xs)
  (_float? xs))

(defn ident? (xs)
  (_ident? xs))

(defn string? (xs)
  (_string? xs))

(defn quote? (xs)
  (_quote? xs))

(defn quasiquote? (xs)
  (_quasiquote? xs))

(defn unquote? (xs)
  (_unquote? xs))

(defn + (x & args)
  (if (nil? args)
      x
      (_+ x (apply + args))))

(defn - (x y)
  (_- x y))

(defn * (x y)
  (_* x y))

(defn / (x y)
  (_/ x y))

(defn < (x y)
  (_< x y))

(defn > (x y)
  (_> x y))

(defn count (xs)
  (if (nil? xs)
      0
      (+ 1 (count (cdr xs)))))

(defmacro cond (preds)
  `(if ,(car (car preds))
       ,(cadr (car preds))
       ,(if (nil? (cdr preds))
            ()
            `(cond ,(cdr preds)))))

(defn do (& args)
  (cond (((nil? args) ())
         ((= 1 (count args)) (car args))
         ('else (apply do (cdr args))))))

(defmacro define (name expr & exprs)
  `(def ,(if (list? name)
             (car name)
             name)
        ,(if (list? name)
             `(lambda ,(car name) ,(cdr name)
                      ,(if (nil? exprs)
                           expr
                           (cons do
                                 (cons expr exprs))))
             expr)))

(defmacro or (p1 p2)
  `(if ,p1
       't
       ,p2))

(defmacro and (p1 p2)
  `(if ,p1
       ,p2
       ()))

(defn <= (x y)
  (or (< x y)
      (= x y)))

(defn >= (x y)
  (or (> x y)
      (= x y)))

(defn map (f xs)
  (if (nil? xs)
      ()
      (cons (f (car xs))
            (map f (cdr xs)))))

(defn inc (x)
  (+ x 1))

(defmacro let (args body)
  (cons `(lambda ,(map car args)
           ,body)
        (map cadr args)))

(defn str (s & args)
  (cond (((nil? args) s)
         ((= 1 (count args)) (_str s (car args)))
         ('else 
         (_str s (apply str args))))))

(defn println (s & args)
  (cond (((nil? args) (_print (str s "\n")))
         ((= 1 (count args)) (_print (str s " " (car args) "\n")))
         ('else (apply println (cons s
                                     (cons (str (car args) " " (cadr args))
                                           (cddr args))))))))
  


  
