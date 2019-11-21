(ns checkout.core
  (:require [clojure.spec.alpha :as s])
  (:gen-class))

(s/def :rule/type keyword?)
(defmulti rule-type :rule/type)

; n items for price-1, with remaining at price-2 each
(defmethod rule-type :for
  [_]
  (s/keys :req [::n ::price-1 ::price-2]))

; continuous pricing; rate of n per price (extra rule type not specific in Kata)
(defmethod rule-type :per
  [_]
  (s/keys :req [::n ::price]))

(s/def ::pricing-rule (s/multi-spec rule-type :rule/type))
(s/def ::name string?)
(s/def ::quantity int?)
(s/def ::price double?)
(s/def ::price-1 ::price)
(s/def ::price-2 ::price)
(s/def ::product (s/keys :req [::name ::quantity ::pricing-rule]))

(s/fdef each :args (s/cat :name ::name :p ::price) :ret ::product)
(s/fdef n-for-p :args (s/cat :name ::name :n ::quantity :p ::price) :ret ::product)
(s/fdef per :args (s/cat :name ::name :quant ::quantity :n int? :p ::price) :ret ::product)

(defn each [name p]
  {::name name ::quantity 1 ::pricing-rule {:rule/type :for ::n 1  ::price-1 p ::price-2 p}})
(defn n-for-p [name n p]
  {::name name ::quantity 1 ::pricing-rule {:rule/type :for ::n n  ::price-1 p ::price-2 p}})
(defn per [name quant n p]
  {::name name ::quantity quant ::pricing-rule {:rule/type :per ::n n ::price p}})

(s/fdef checkout :args (s/cat :products (s/coll-of ::product :kind vector?)))

(defmulti price #(get-in % [::pricing-rule :rule/type]))
(defmethod price :per
  [{quantity ::quantity, {n ::n, p ::price} ::pricing-rule}]
  (/ (* quantity p) n))
(defmethod price :for
  [{quantity ::quantity, {n ::n, p1 ::price-1, p2 ::price-2} ::pricing-rule}]
  (let [r (rem quantity n)]
    (+ (/ (* (- quantity r) p1) n) (* r p2))))

(comment
  (s/fdef combine
    :args (s/and (s/cat :product1 ::product :product2 ::product)
                 #(not= (get-in %1 [::pricing-rule :rule/type])
                        (get-in %2 [::pricing-rule :rule/type])))))
(defn combine
  [products]
  {:pre [(apply = (map #(get-in % [::pricing-rule :rule/type]) products))
         (apply = (map :name products))]}
  (assoc (first products) ::quantity (reduce + (map ::quantity products))))

(defn checkout-by
  [f]
  (fn
    [products]
    (let [grouped (group-by ::name products)]
      (f (zipmap (keys grouped) (map combine (vals grouped)))))))

(defn transform-vals
  [m f]
  (zipmap (keys m) (map f (vals m))))

(defn subtotals
  [products]
  ((checkout-by #(transform-vals % price)) products))

(defn checkout
  "Calculate total price of products"
  [products]
  (reduce + (vals (subtotals products))))


(comment
  (s/explain ::pricing-rule {:rule/type :per ::n 1 ::price 10.00})
  (s/explain ::pricing-rule {:rule/type :for ::n 1 ::price-1 10.00 ::price-2 5.00})
  (s/explain ::product (each "apples" 0.50))
  (let [apples (each "apples" 0.50)
        pears (n-for-p "pears" 3 1.00)
        oats (per "oats" 1000 100 0.50)]
    (subtotals [apples pears apples oats pears oats])))
