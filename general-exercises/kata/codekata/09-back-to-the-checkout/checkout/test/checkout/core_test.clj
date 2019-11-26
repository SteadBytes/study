(ns checkout.core-test
  (:require [clojure.test :refer :all]
            [checkout.core :as checkout]
            [clojure.string :as str]))

(defn total-price
  [goods rules]
  (::checkout/total (checkout/checkout (filter not-empty (str/split goods #""))
                                       rules)))

(deftest provided-test-cases
  (let [rules {"A" (checkout/n-for-p 3 130.00 50.00),
               "B" (checkout/n-for-p 2 45.00 30.00),
               "C" (checkout/each 20.00),
               "D" (checkout/each 15.00)}
        total-price #(total-price % rules)]
    (testing "totals"
             (is (= 0.0 (total-price "")))
             (is (= 50.0 (total-price "A")))
             (is (= 80.0 (total-price "AB")))
             (is (= 115.0 (total-price "CDBA")))
             (is (= 100.0 (total-price "AA")))
             (is (= 130.0 (total-price "AAA")))
             (is (= 180.0 (total-price "AAAA")))
             (is (= 230.0 (total-price "AAAAA")))
             (is (= 260.0 (total-price "AAAAAA")))
             (is (= 160.0 (total-price "AAAB")))
             (is (= 175.0 (total-price "AAABB")))
             (is (= 190.0 (total-price "AAABBD")))
             (is (= 190.0 (total-price "DABABA"))))
    (testing
      "incremental"
      (let [co (checkout/checkout [] rules)]
        (is (= 0.0 (::checkout/total co)))
        (loop [co co
               expected-totals [50.0 80.0 130.0 160.0 175.0]
               items-to-scan ["A" "B" "A" "A" "B"]]
          (if-not (empty? items-to-scan)
            (let [co' (checkout/scan co (first items-to-scan))]
              (is (= (first expected-totals) (::checkout/total co')))
              (recur co' (rest expected-totals) (rest items-to-scan)))))))))

(deftest rule-types
  (let [price (fn [rule n] ((checkout/price rule) n))]
    (testing "per"
             (let [rule (checkout/per 100 0.50)]
               (is (= 0.00 (price rule 0)))
               (is (= 0.50 (price rule 100)))
               (is (= 0.25 (price rule 50)))
               (is (= 0.75 (price rule 150)))
               (is (= 1.00 (price rule 200)))))
    (testing "each"
             (let [rule (checkout/each 0.50)]
               (is (= 0.00 (price rule 0)))
               (is (= 0.50 (price rule 1)))
               (is (= 1.0 (price rule 2)))
               (is (= 50.0 (price rule 100)))
               (is (thrown? java.lang.AssertionError (price rule -1)))
               (is (thrown? java.lang.AssertionError (price rule 0.5)))))))