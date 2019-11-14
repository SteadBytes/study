(ns checkout.core
  (:gen-class))

(defn multi-price [{n :n price :price}]
  (fn [n-items unit-price]
    (let [n-unit (rem n-items n)
          n-multi (/ (- n-items n-unit) n)]
      (+ (* n-multi price) (* n-unit unit-price)))))

(defn calc-total
  [items pricing-rules]
  (reduce + (map
             (fn [[item n]]
               (let [special-price (get-in pricing-rules [item :special-price])
                     unit-price (get-in pricing-rules [item :unit-price])]
                 (if special-price
                   ((multi-price special-price) n unit-price)
                   (* n unit-price))))
             (frequencies items))))

(defn add-total
  [{items :items, pricing-rules :pricing-rules, :as co}]
  (assoc co :total (calc-total items pricing-rules)))

(defn scan
  [co item]
  (add-total (update co :items  #(conj % item))))

(defn checkout [items pricing-rules]
  (add-total {:items items :pricing-rules pricing-rules}))

(comment
  (def pricing-rules {"A" {:unit-price 50 :special-price {:n 3 :price 130}}
                      "B" {:unit-price 30 :special-price {:n 2 :price 45}}
                      "C" {:unit-price 20}
                      "D" {:unit-price 15}})
  (:total (checkout ["B" "A" "B"] pricing-rules)))