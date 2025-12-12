import Std.Internal.Parsec
import Std.Data.HashMap

open Std.Internal.Parsec
open Std.Internal.Parsec.String
open Std(HashMap)

-- # Input parser

abbrev Device := String
abbrev Input := HashMap Device (Array Device)

def device : Parser Device := many1Chars asciiLetter

def parser : Parser Input := do
  let pairs <- many (do
    let src <- device
    skipChar ':'
    let dsts <- many (skipChar ' ' *> device)
    skipChar '\n'
    return (src, dsts))
  return HashMap.ofList pairs.toList

-- # Part 1

partial def countPathsWithCache (dag : Device -> Array Device) (dst src : Device) (cache : HashMap Device Nat)
: (Nat × HashMap Device Nat) :=
  match cache.get? src with
  | some val => (val, cache)
  | none =>
    let ⟨answer, cache⟩ := dag src
      |>.foldl
        (fun ⟨acc, cache⟩ next =>
          let ⟨subcount, cache⟩ := countPathsWithCache dag dst next cache
          (acc + subcount, cache))
        (0, cache)
    (answer, cache.insert src answer)

def countPaths (input : Input) (src dst : Device) : Nat :=
  let cache := HashMap.emptyWithCapacity 1024 |>.insert dst 1
  countPathsWithCache (input.getD · #[]) dst src cache |>.fst

def solve1 (input : Input) : Nat :=
  countPaths input "you" "out"

-- # Part 2

def solve2 (input : Input) : Nat :=
  let countPaths := countPaths input
  countPaths "svr" "fft" * countPaths "fft" "dac" * countPaths "dac" "out"
    + countPaths "svr" "dac" * countPaths "dac" "fft" * countPaths "fft" "out"
