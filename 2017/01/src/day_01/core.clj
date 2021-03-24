(ns day-01.core
  (:gen-class))


(defn g
  [s n]
  (let [rotated-s (str (subs s n) (subs s 0 n))
        pairs (map vector s rotated-s)]
    (reduce + 0
            (for [[x y] pairs
                  :when (and (= x y)
                             (Character/isDigit x))]
              (read-string (str x))))))

(defn f
  [s]
  (g s 1))

(assert (= (f "1122") 3))
(assert (= (f "1111") 4))
(assert (= (f "1234") 0))
(assert (= (f "91212129") 9))

(defn h
  [s]
  (g s (/ (count s) 2)))

(assert (= (h "1212") 6))
(assert (= (h "1221") 0))
(assert (= (h "123425") 4))
(assert (= (h "123123") 12))
(assert (= (h "12131415") 4))


(defn -main
  "I don't do a whole lot ... yet."
  [& args]
  (let [line (read-line)]
    (println (h line))))
