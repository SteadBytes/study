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

(comment
  (s/explain ::pricing-rule {:rule/type :per ::n 1 ::price 10})
  (s/explain ::pricing-rule {:rule/type :for ::n 1 ::price-1 10 ::price-2 5})
  (s/explain ::product (each "apples" 0.50)))
