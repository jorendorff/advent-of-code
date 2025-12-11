import Std.Internal.Parsec
import Std.Data.HashMap
import Std.Data.HashSet

open Std.Internal.Parsec
open Std.Internal.Parsec.String
open Std(HashMap HashSet)

-- # Input parser

abbrev Device := String
abbrev Input := Array (Device × (Array Device))

def device : Parser Device := many1Chars asciiLetter

def parser : Parser Input := many (do
    let src <- device
    skipChar ':'
    let dsts <- many (skipChar ' ' *> device)
    skipChar '\n'
    return (src, dsts))


-- # Producing the tier array

-- Maps each device to its tier.
abbrev TierMap := HashMap Device Nat

-- Maps tier numbers to the array of all devices in that tier.
-- A TierMap is basically a function Device -> Nat;
-- the TierArray is the inverse, Nat -> Array Device.
abbrev TierArray := Array (Array Device)

-- List of all vertices mentioned in the input.
def allDevices (input : Input) : List Device :=
  input
    |>.flatMap (fun ⟨src, dsts⟩ => dsts.push src)
    |>.toList
    |> HashSet.ofList
    |>.toList

-- Repeatedly apply a function to a value until the given metric
-- stops changing.
partial def toFixedPointBy {α β: Type} [BEq β] (f : α -> α) (metric : α -> β) (start : α) : α :=
  let next := f start
  if metric start == metric next then next else toFixedPointBy f metric next

-- Slow algorithm to stratify a graph.
partial def stratify (input : Input) : TierMap :=
  let devices := allDevices input
  let edges := input.flatMap (fun ⟨src, dsts⟩ => dsts.map (src, ·))
  let fwd :=
    devices.map (·, #[])
      |> HashMap.ofList
      |> edges.foldl (fun fwd ⟨src, dst⟩ =>
        fwd.alter src (fun dsts => some $ dsts.getD #[] |>.push dst))

  toFixedPointBy
    (fun tierMap =>
      HashMap.ofList (devices.map (fun src =>
        let newTier := fwd.getD src #[]
          |>.map (fun dst => tierMap.getD dst 0 + 1)
          |>.foldl max 0
        (src, newTier))))
    -- Could use the identity function as our metric, except
    -- somehow HashMap String Nat doesn't have BEq
    (fun tierMap => tierMap.valuesArray.sum)
    (HashMap.ofList (devices.map (· , 0)))

-- For the graph described by input, return the reverse graph.
-- The resulting hashMap maps destination nodes to the array
-- of all their predecessors.
def revMap (input : Input) : HashMap Device (Array Device) :=
  let devices := allDevices input
  let edges := input.flatMap (fun ⟨src, dsts⟩ => dsts.map (src, ·))
  HashMap.emptyWithCapacity devices.length
  |> edges.foldl (fun rev ⟨src, dst⟩ =>
    rev.alter dst (fun srcs => srcs.getD #[] |>.push src))

-- Invert a TierMap, producing the TierArray.
def TierMap.toTierArray (tierMap : TierMap) : TierArray :=
  let maxTier := tierMap.valuesIter.fold max 0
  Array.replicate (maxTier + 1) #[]
    |> tierMap.fold (fun tiers dev tier =>
      tiers.set! tier $ tiers[tier]!.push dev)

-- # Count paths given the tiers

def countPaths
  (tierMap : TierMap) (tiers : TierArray)
  (rev : HashMap Device (Array Device))
  (pathSrc : Device) (pathDst : Device)
: Nat :=
  let start := tierMap.getD pathDst 0
  let stop := tierMap.getD pathSrc 0
  -- The point of all this tier stuff is to get the relevant edges
  -- in just the right order.
  let edges := tiers[start:stop]
    |>.toArray
    |>.flatMap id
    |>.flatMap (fun dst => rev.getD dst #[] |>.map (·, dst))
  let numPaths := ({(pathDst, 1)} : HashMap Device Nat)
    |> edges.foldl (fun numPaths ⟨src, dst⟩ =>
      numPaths.alter src (fun n => n.getD 0 + numPaths.getD dst 0))
  let n := numPaths.getD pathSrc 0
  dbg_trace "countPaths {pathSrc} {pathDst} = {n}"
  n

-- # Part 1

def solve1 (input : Input) : Nat :=
  let tierMap := stratify input
  let tiers := tierMap.toTierArray
  let rev := revMap input
  countPaths tierMap tiers rev "you" "out"

-- # Part 2

def solve2 (input : Input) : Nat :=
  let tierMap := stratify input
  let tiers := tierMap.toTierArray
  let rev := revMap input
  let countPaths := countPaths tierMap tiers rev
  countPaths "svr" "fft" * countPaths "fft" "dac" * countPaths "dac" "out"
    + countPaths "svr" "dac" * countPaths "dac" "fft" * countPaths "fft" "out"
