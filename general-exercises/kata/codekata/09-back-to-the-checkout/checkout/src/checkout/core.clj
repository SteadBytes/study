(ns checkout.core
  (:require [clojure.spec.alpha :as s])
  (:gen-class))

(s/def ::name string?)
(s/def ::price double?)
(s/def ::n integer?)
(s/def ::quantity pos?)
(s/def ::for-price ::price)
(s/def ::total ::price)
(s/def ::products (s/coll-of ::name :kind vector?))

(s/def :rule/type keyword?)
(defmulti rule-type :rule/type)

; n items for ::for-price, with remaining at ::price each
(defmethod rule-type :for [_] (s/keys :req [::n ::for-price ::price]))

; Continuous pricing i.e. rate of n per price (extra rule type not specific in
; Kata)
(defmethod rule-type :per [_] (s/keys :req [::quantity ::price]))

(s/def ::pricing-rule (s/multi-spec rule-type :rule/type))
(s/def ::rules (s/map-of ::name ::pricing-rule))
(s/def ::price-fn
  (s/fspec :args (s/cat :n-products integer?)
           :ret double?))
(s/def ::price-fns (s/map-of ::name ::price-fn))
(s/def ::checkout (s/keys :req [::products ::rules ::total ::price-fns]))

(s/fdef each
  :args (s/cat :p ::price)
  :ret ::pricing-rule)
(s/fdef n-for-p
  :args (s/cat :n ::n
               :p ::price)
  :ret ::pricing-rule)
(s/fdef per
  :args (s/cat :quant ::quantity
               :n int?
               :p ::price)
  :ret ::pricing-rule)
(s/fdef price :args ::pricing-rule :ret ::price-fn)
(s/fdef checkout
  :args (s/cat ::products ::rules)
  :ret ::checkout)
(s/fdef total :args ::checkout :ret ::checkout)
(s/fdef scan
  :args (s/cat ::checkout ::name)
  :ret ::checkout)

(defn each [p] {:rule/type :for, ::n 1, ::for-price p, ::price p})
(defn n-for-p [n p1 p2] {:rule/type :for, ::n n, ::for-price p1, ::price p2})
(defn per [quant p] {:rule/type :per, ::quantity quant, ::price p})

(defmulti price :rule/type)

(defmethod price :per
  [{rule-quant ::quantity, p ::price}]
  (fn [quant]
    {:pre [(or (zero? quant) (pos? quant))]}
    (/ (* quant p) rule-quant)))

(defmethod price :for
  [{n ::n, p1 ::for-price, p2 ::price}]
  (fn [n-products]
    {:pre [(or (zero? n-products) (pos-int? n-products))]}
    (let [r (rem n-products n)] (+ (/ (* (- n-products r) p1) n) (* r p2)))))

(defn calc-total
  [product-freqs price-fns]
  (if (empty? product-freqs)
    0.0
    (reduce + (map (fn [[k v]] ((get price-fns k) v)) product-freqs))))

(defn total
  [co]
  (assoc co
    ::total (calc-total (frequencies (::products co)) (::price-fns co))))

; TODO:  Handle continuous goods (:per rules)
; currently checkout only takes individual items i.e. ["A" "B" "C"]
; needs to handle ["A" "B" "C" n-grams-of-D e.t.c]
(defn scan [co item] (total (update co ::products #(conj % item))))

(defn checkout
  [products rules]
  (let [co {::products products,
            ::rules rules,
            ::price-fns (into {} (for [[k v] rules] [k (price v)]))}]
    (total co)))

(comment (let [rules {"A" (n-for-p 3 130.00 50.00),
                      "B" (n-for-p 2 45.00 30.00),
                      "C" (each 20.0),
                      "D" (each 15.0)}
               co (checkout ["A" "A" "A" "B"] rules)]
           (println (::total co))
           (println (::total (scan co "C")))))