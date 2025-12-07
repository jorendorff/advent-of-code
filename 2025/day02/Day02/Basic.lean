import Std.Internal.Parsec

open Std.Internal.Parsec.String

-- # Input parser

structure Range where
  start : Nat
  stop : Nat

abbrev Input := Array Range

-- Parser combinator for optional syntax.
def opt {α : Type} (elem : Parser α) : Parser (Option α) :=
  (do let x <- elem; return some x) <|> return none

-- Parser combinator for lists with a separator.
def sepBy {α : Type} (elem : Parser α) (sep : Parser Unit) : Parser (Array α) := do
  let some first <- opt elem | return #[]
  (do sep; elem).manyCore #[first]

def range : Parser Range := do
  let first <- digits
  skipChar '-'
  let last <- digits
  return ⟨first, last + 1⟩

def parser : Parser Input := do
  let nums <- sepBy range (skipChar ',')
  skipChar '\n'
  return nums

-- # Part 1

def Range.toArray (range : Range) : Array Nat :=
  Array.range' range.start (range.stop - range.start) (step := 1)

def periodic? (s: String.Slice) (period: Nat) : Bool :=
  let n := s.utf8ByteSize
  period > 0 && n % period == 0 && (
    let start1 := s.pos! $ String.Pos.Raw.mk $ period
    let stop0 := s.pos! $ String.Pos.Raw.mk $ (n - period)
    s.replaceEnd stop0 == s.replaceStart start1)

def invalid1? (id: Nat) : Bool :=
  let s := id.repr.toSlice
  let n := s.utf8ByteSize
  periodic? s (n / 2)

def solve1 (input : Input) : Nat :=
  input.flatMap Range.toArray |>.filter invalid1? |>.sum

-- # Part 2

def invalid2? (id: Nat) : Bool :=
  let s := id.repr.toSlice
  let n := s.utf8ByteSize
  Nat.any (n / 2 + 1) (fun p _ => periodic? s p)

def solve2 (input : Input) : Nat :=
  input.flatMap Range.toArray |>.filter invalid2? |>.sum
