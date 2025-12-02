import Std.Internal.Parsec

open Std.Internal.Parsec.String

-- # Input parser

-- Parser combinator for optional syntax.
def opt {t : Type} (elem : Parser t) : Parser (Option t) :=
  (do let x <- elem; return some x) <|> return none

-- Parser combinator for lists with a separator.
def sepBy {t : Type} (elem : Parser t) (sep : Parser Unit) : Parser (Array t) := do
  let some first <- opt elem | return #[]
  (do sep; elem).manyCore #[first]

def range : Parser (Nat × Nat) := do
  let first <- digits
  skipChar '-'
  let last <- digits
  return ⟨first, last⟩

def parser : Parser (Array (Nat × Nat)) := do
  let nums <- sepBy range (skipChar ',')
  skipChar '\n'
  return nums

-- # Part 1

def toRange (bounds : Nat × Nat) : List Nat :=
  let ⟨first, last⟩ := bounds
  List.range' first (last - first + 1) (step := 1)

def makesPattern? (s: String.Slice) (period: Nat) : Bool :=
  let n := s.utf8ByteSize
  period > 0 && n % period == 0 && (
    let start1 := s.pos! $ String.Pos.Raw.mk $ period
    let stop0 := s.pos! $ String.Pos.Raw.mk $ (n - period)
    s.replaceEnd stop0 == s.replaceStart start1)

def invalid1? (id: Nat) : Bool :=
  let s := id.repr.toSlice
  let n := s.utf8ByteSize
  makesPattern? s (n / 2)

def solve1 (input : Array (Nat × Nat)) : Nat :=
  ((input.toList.flatMap toRange).filter invalid1?).sum

-- # Part 2

def invalid2? (id: Nat) : Bool :=
  let s := id.repr.toSlice
  let n := s.utf8ByteSize
  Nat.any (n / 2 + 1) (fun p _ => makesPattern? s p)

def solve2 (input : Array (Nat × Nat)) : Nat :=
  ((input.toList.flatMap toRange).filter invalid2?).sum
