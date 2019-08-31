(ns tpp-14-ex7.core
  (:require [instaparse.core :as insta]))

(def time-spec
  (insta/parser
   "time = hour period | hour ':' minute period | hour ':' minute
    period = \"am\" | \"pm\"
    hour = hour-tens-place digit | digit
    minute = minute-tens-place digit | digit
    hour-tens-place = '0' | '1' | '2'
    minute-tens-place = '0' | '1' | '2' | '3' | '4' | '5'
    digit = '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9'"))

(time-spec "4pm")
; => [:time [:hour [:digit "4"]] [:period "pm"]]

(time-spec "7:38pm")
; => [:time [:hour [:digit "7"]] ":" [:minute [:minute-tens-place "3"] [:digit "8"]] [:period "pm"]]

(time-spec "23:42")
; => [:time [:hour [:hour-tens-place "2"] [:digit "3"]] ":" [:minute [:minute-tens-place "4"] [:digit "2"]]]

(time-spec "3:16")
; => [:time [:hour [:digit "3"]] ":" [:minute [:minute-tens-place "1"] [:digit "6"]]]

(time-spec "3:16am")
; => [:time [:hour [:digit "3"]] ":" [:minute [:minute-tens-place "1"] [:digit "6"]] [:period "am"]]

