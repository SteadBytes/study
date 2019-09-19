(ns dbc-blender.core
  (:gen-class)
  (:require [clojure.spec.alpha :as s]
            [clojure.math.numeric-tower :as math]))

; turn it up to 11!
(def max-speed 11)

; speed is an integer and in correct range
(s/def ::speed (s/and int? #(and (>= % 0) (<= % max-speed))))
(s/def ::full boolean?)
(s/def ::blender (s/and (s/keys :req [::speed ::full])
                        #(if (> (::speed %) 0) ; should only be on when full
                           (::full %)
                           true)))

(defn set-speed [blender x]
  {:pre [(s/valid? ::blender blender)
         (s/valid? ::speed x)
         (::full blender)
         (= (math/abs (- (::speed blender) x)) 1)] ; increase in single increments
   :post [(= (::speed %) x) ; speed was set
          (s/valid? ::blender %)]}
  (assoc blender ::speed x))

(defn fill [blender]
  {:pre [(s/valid? ::blender blender)
         (= (::speed blender) 0) ; can't fill a spinning blender
         (not (::full blender))] ; can't fill an already full blender
   :post [(s/valid? ::blender %)
          (::full %)]} ; blender was filled
  (assoc blender ::full true))

(defn empty [blender]
  {:pre [(s/valid? ::blender blender)
         (= (::speed blender) 0) ; can't empty a spinning blender
         (::full blender)] ; can't empty an empty blender
   :post [(s/valid? ::blender %)
          (not (::full %))]} ; blender was emptied
  (assoc blender ::full false))