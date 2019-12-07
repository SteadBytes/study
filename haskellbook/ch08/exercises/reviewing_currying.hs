module ReviewingCurrying where

cattyConny :: String -> String -> String
cattyConny x y = x ++ " mrow " ++ y

flippy = flip cattyConny

appedCatty = cattyConny "woops"
frappe = flippy "haha"

main = do
    -- 1
    -- "woops mrow woohoo!"
    print (appedCatty "woohoo!")

    -- 2
    -- "1 mrow haha"
    print (frappe "1")

    -- 3
    -- "woops mrow blue mrow haha"

    -- 4
    -- "woops mrow blue mrow haha"
    print (appedCatty (frappe "blue"))

    -- 5
    -- "pink mrow haha green mrow woops mrow blue"
    print (cattyConny (frappe "pink")
                      (cattyConny "green" (appedCatty "blue")))
    
    -- 6
    -- "are mrow Pugs mrow awesome"
    print (cattyConny (flippy "Pugs" "are") "awesome")