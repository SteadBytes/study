-- Enabled to allow deriving Ord in question 4
{-# LANGUAGE StandaloneDeriving #-}

module WhatCanWeDo where

data Rocks =
   Rocks String deriving (Eq, Show) 

data Yeah =
    Yeah Bool deriving (Eq, Show)

data Papu =
    Papu Rocks Yeah
    deriving (Eq, Show)

-- 1
-- Doesn't typecheck. Papu requires arguments of types Rocks and Yeah but is
-- being applied to values of types String and Bool. 
-- To fix, the values need to first used to construct instances of Rocks and Yeah.

phew = Papu (Rocks "chases")
            (Yeah True)

-- 2
-- Typechecks.
truth = Papu (Rocks "chomskydoz")
             (Yeah True)

-- 3
-- Typechecks.
equalityForAll :: Papu -> Papu -> Bool
equalityForAll p p' = p == p'

-- 4
-- Doesn't typecheck. Papu doesn't have an Ord instance
-- To fix, Papu must have an Ord instance. My chosen definition of Ord for
-- Papu also requires comparing it's constituents therefore both Rocks and Yeah
-- required Ord instances. These would better be added as part of the initial
-- datatype declaration to avoid the need for the StandaloneDeriving extension.

deriving instance Ord Rocks
deriving instance Ord Yeah
deriving instance Ord Papu

-- OR 'manually' (doesn't require StandaloneDeriving):
-- instance Ord Rocks where
--     compare (Rocks a) (Rocks a')  = compare a a'
-- instance Ord Yeah where
--     compare (Yeah a) (Yeah a')  = compare a a'
-- instance Ord Papu where
--     compare (Papu a b) (Papu a' b') = compare (a, b) (a', b')

comparePapus :: Papu -> Papu -> Bool
comparePapus p p' = p > p'