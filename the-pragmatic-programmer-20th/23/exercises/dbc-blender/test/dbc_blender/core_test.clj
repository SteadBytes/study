(ns dbc-blender.core-test
  (:require [clojure.test :refer :all]
            [dbc-blender.core :as b]
            [clojure.spec.alpha :as s]))

(deftest blender

  (testing "blender can only be on when full"
    (is (not
         (s/valid? ::b/blender {::b/speed 5 ::b/full false})))
    (is (s/valid? ::b/blender {::b/speed 0 ::b/full false}))
    (is (s/valid? ::b/blender {::b/speed 5 ::b/full true})))

  (testing "set-speed"
    (is (= 1 (::b/speed (b/set-speed {::b/speed 0 ::b/full true} 1))))
    (is (= 11 (::b/speed (b/set-speed {::b/speed 10 ::b/full true} 11))))
    (is (= 0 (::b/speed (b/set-speed {::b/speed 1 ::b/full true} 0))))
    (is (thrown?
         java.lang.AssertionError
         (b/set-speed {::b/speed 0 ::b/full false} 1))
        "cannot start an empty blender")
    (is (thrown?
         java.lang.AssertionError
         (b/set-speed {::b/speed 0 ::b/full true} 2))
        "cannot change speed by more than 1 at a time")
    (testing "speeds out of valid range 0 <= ::speed <= 11"
      (is (thrown?
           java.lang.AssertionError
           (b/set-speed {::b/speed 0 ::b/full true} -1)))
      (is (thrown?
           java.lang.AssertionError
           (b/set-speed {::b/speed 11 ::b/full true} 12)))))

  (testing "fill"
    (is (::b/full (b/fill {::b/speed 0 ::b/full false})))
    (is (thrown?
         java.lang.AssertionError
         (b/fill {::b/speed 0 ::b/full true}))
        "cannot fill an already full blender")
    (is (thrown?
         java.lang.AssertionError
         (b/fill {::b/speed 1 ::b/full true}))
        "cannot fill a spinning blender"))

  (testing "empty"
    (is (not (::b/full (b/empty {::b/speed 0 ::b/full true}))))
    (is (thrown?
         java.lang.AssertionError
         (b/empty {::b/speed 0 ::b/full false}))
        "cannot empty an already empty blender")
    (is (thrown?
         java.lang.AssertionError
         (b/empty {::b/speed 1 ::b/full true}))
        "cannot empty a spinning blender")))