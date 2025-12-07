import Std.Internal.Parsec
import Std.Data.HashSet

open Std.Internal.Parsec
open Std.Internal.Parsec.String

-- # Input parser

structure Range where
  start : Nat
  stop : Nat
deriving Ord

structure Input where
  fresh : Array Range
  avail : Array Nat

def range : Parser Range := do
  let start <- digits
  skipChar '-'
  let last <- digits
  return ⟨start, last + 1⟩

def lines {α : Type} (p : Parser α) : Parser (Array α) :=
  many do
    let item <- p
    skipChar '\n'
    return item

def parser : Parser Input := do
  let f <- lines range
  skipChar '\n'
  let a <- lines digits
  return ⟨f, a⟩

-- # Part 1

def fresh? (fresh : Array Range) (id : Nat) : Bool :=
  fresh.any fun range => range.start <= id && id < range.stop

def solve1 (input : Input) : Nat :=
  input.avail
    |> Array.filter (fresh? input.fresh)
    |> Array.size

-- # Part 2

def solve2 (input : Input) : Nat :=
  let ⟨_mark, total⟩ := Array.foldl
    (fun ⟨mark, total⟩ range =>
      let stop := range.stop
      let start := min stop (max range.start mark)
      (max mark stop, total + (stop - start)))
    (0, 0)
    input.fresh.qsortOrd
  total

/-
theorem solve2_correct (freshRanges : Array Range) :
  let S := {id | ∃ r ∈ freshRanges, r.start ≤ id ∧ id < r.stop}
  ∃ hS : S.Finite, solve2 fresh = hS.toFinset.card
:= by
  sorry
-/
