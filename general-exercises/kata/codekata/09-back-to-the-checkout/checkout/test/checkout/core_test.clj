(ns checkout.core-test
  (:require [clojure.test :refer :all]
            [checkout.core :refer :all]
            [clojure.string :as str]))

(def rules {"A" {:unit-price 50 :special-price {:n 3 :price 130}}
            "B" {:unit-price 30 :special-price {:n 2 :price 45}}
            "C" {:unit-price 20}
            "D" {:unit-price 15}})

(defn price
  [goods]
  (:total (checkout (filter not-empty (str/split goods #"")) rules)))

(deftest totals
  (is (= 0 (price "")))
  (is (= 50 (price "A")))
  (is (= 80 (price "AB")))
  (is (= 115 (price "CDBA")))

  (is (= 100 (price "AA")))
  (is (= 130 (price "AAA")))
  (is (= 180 (price "AAAA")))
  (is (= 230 (price "AAAAA")))
  (is (= 260 (price "AAAAAA")))

  (is (= 160 (price "AAAB")))
  (is (= 175 (price "AAABB")))
  (is (= 190 (price "AAABBD")))
  (is (= 190 (price "DABABA"))))

(deftest incremental
  (let [co (checkout [] rules)]
    (is (= 0 (:total co)))
    (loop [co co
           expected-totals [50 80 130 160 175]
           items-to-scan ["A" "B" "A" "A" "B"]]
      (if-not (empty? items-to-scan)
        (let [co' (scan co (first items-to-scan))]
          (is (= (first expected-totals) (:total co')))
          (recur co' (rest expected-totals) (rest items-to-scan)))))))
