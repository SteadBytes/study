(ns checkout.core
  (:require [clojure.spec.alpha :as s])
  (:gen-class))

(s/def :rule/type keyword?)
(defmulti rule-type :rule/type)
(defmethod rule-type :per
  "continuous pricing; rate of n per price"
  [_]
  (s/keys :req [::n ::price]))
(defmethod rule-type :for
  "n items for price-1, with remaining at price-2 each"
  [_]
  (s/keys :req [::n ::price-1 ::price-2]))
(s/def ::pricing-rule (s/multi-spec rule-type :rule/type))
(s/def ::name string?)
(s/def ::quantity int?)
(s/def ::product (s/keys :req [::name ::quantity ::pricing-rule]))

(comment
  (s/explain ::pricing-rule {:rule/type :per ::n 1 ::price 10})
  (s/explain ::pricing-rule {:rule/type :for ::n 1 ::price-1 10 ::price-2 5}))
