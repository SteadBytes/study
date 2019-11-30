(ns checkout.core
  (:require [clojure.spec.alpha :as s])
  (:gen-class))

(s/def ::name string?)
(s/def ::n integer?)
(s/def ::quantity pos?)
(s/def ::price double?)
(s/def ::for-price ::price)
(s/def ::total ::price)
(s/def ::products (s/map-of ::name ::quantity))
(s/def ::input-item
  (s/or ::name (s/cat ::name (s/or ::quantity ::n))))

(s/def :rule/type keyword?)
(defmulti rule :rule/type)

; n items for ::for-price, with remaining at ::price each
(defmethod rule :for [_] (s/keys :req [::n ::for-price ::price]))

; Continuous pricing i.e. rate of n per price (extra rule type not specific in
; Kata)
(defmethod rule :per [_] (s/keys :req [::quantity ::price]))

(s/def ::pricing-rule (s/multi-spec rule :rule/type))
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
  :args (s/cat ::items-to-scan (s/coll-of ::input-item)
               ::rules ::rules)
  :ret ::checkout)
(s/fdef total :args ::checkout :ret ::checkout)
(s/fdef scan
  :args (s/cat ::checkout ::input-item)
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

(defn total
  [{products ::products, price-fns ::price-fns, :as co}]
  (assoc co
    ::total (if (empty? products)
              0.0
              (reduce + (map (fn [[k v]] ((get price-fns k) v)) products)))))

(defn rule-type
  [{rules ::rules} product-name]
  (:rule/type (get rules product-name)))

(defmulti valid-scan? (fn [co [product-name _]] (rule-type co product-name)))
; cannot have negative, or partial :for products i.e -1 TVs or 0.5 TVs
(defmethod valid-scan? :for [_ [_ n]] (pos-int? n))
; cannot have negative :per products i.e. -100g of oats
(defmethod valid-scan? :per [_ [_ n]] (pos? n))

(defmulti scan (fn [_ item] (class item)))
(defmethod scan String
  [co product-name]
  ; scanning a :per rule product *requires* specifying a quantity
  {:pre [(= (rule-type co product-name) :for)]}
  (scan co [product-name 1]))
(defmethod scan clojure.lang.PersistentVector
  [co [product-name n]]
  {:pre [(valid-scan? co [product-name n])]}
  (total (update-in co [::products product-name] (fnil + 0) n)))

(defn checkout
  [items-to-scan rules]
  (let [co {::rules rules,
            ::price-fns (into {} (for [[k v] rules] [k (price v)])),
            ::total 0.0}]
    (reduce scan co items-to-scan)))

(comment (let [rules {"toothpaste" (n-for-p 3 2.00 1.00),
                      "bread-loaf" (n-for-p 2 1.50 0.90),
                      "yoghurt" (each 1.00),
                      "oats" (per 100 2.00)}
               co (checkout ["toothpaste" "toothpaste" "toothpaste"
                             ["bread-loaf" 2] ["yoghurt" 2] ["oats" 450]]
                            rules)]
           (println (::total co))
           (println (::total (scan co "bread-loaf")))
           (println (::total (scan co ["oats" 50])))))
