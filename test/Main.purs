module Test.Main where

import Prelude

import Control.Monad.Rec.Class (Step(..), tailRecM)
import Control.Monad.ST as ST
import Control.Monad.ST.Ref as STRef
import Effect (Effect)
import Effect.Console (log)
import Test.Assert (assertEqual)

sumOfSquares :: Int
sumOfSquares = ST.run do
  total <- STRef.new 0
  let loop 0 = STRef.read total
      loop n = do
        _ <- STRef.modify (_ + (n * n)) total
        loop (n - 1)
  loop 100

testRef :: Effect Unit
testRef = do
  let res = ST.run do
        ref <- STRef.new 0
        v1 <- STRef.read ref
        v2 <- STRef.write 10 ref
        v3 <- STRef.read ref
        v4 <- STRef.modify (\x -> x + 5) ref
        v5 <- STRef.modify' (\x -> { state: x * 2, value: x }) ref
        v6 <- STRef.read ref
        pure { v1, v2, v3, v4, v5, v6 }
  assertEqual { expected: 0, actual: res.v1 }
  assertEqual { expected: 10, actual: res.v2 }
  assertEqual { expected: 10, actual: res.v3 }
  assertEqual { expected: 15, actual: res.v4 }
  assertEqual { expected: 15, actual: res.v5 }
  assertEqual { expected: 30, actual: res.v6 }

testWhile :: Effect Unit
testWhile = do
  let res = ST.run do
        ref <- STRef.new 10
        ST.while ((_ > 0) <$> STRef.read ref) do
          void $ STRef.modify (\x -> x - 1) ref
        STRef.read ref
  assertEqual { expected: 0, actual: res }

testFor :: Effect Unit
testFor = do
  let res = ST.run do
        ref <- STRef.new 0
        ST.for 1 11 \i -> do
          void $ STRef.modify (_ + i) ref
        STRef.read ref
  -- 1 + 2 + ... + 10 = 55
  assertEqual { expected: 55, actual: res }

testForeach :: Effect Unit
testForeach = do
  let res = ST.run do
        ref <- STRef.new 0
        ST.foreach [1, 2, 3, 4, 5] \i -> do
          void $ STRef.modify (_ + i) ref
        STRef.read ref
  assertEqual { expected: 15, actual: res }

testMonadRec :: Effect Unit
testMonadRec = do
  let res = ST.run do
        ref <- STRef.new 0
        tailRecM (\n -> do
          void $ STRef.modify (_ + n) ref
          if n == 0
            then pure (Done unit)
            else pure (Loop (n - 1))
        ) 10
        STRef.read ref
  assertEqual { expected: 55, actual: res }

main :: Effect Unit
main = do
  log "Testing STRef operations..."
  testRef
  log "Testing while loop..."
  testWhile
  log "Testing for loop..."
  testFor
  log "Testing foreach loop..."
  testForeach
  log "Testing MonadRec instance..."
  testMonadRec

  log "Testing sumOfSquares..."
  assertEqual { expected: 338350, actual: sumOfSquares }

  log "All tests passed!"
