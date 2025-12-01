import Std.Internal.Parsec

open Std.Internal.Parsec.String

def parseLeft : Parser Int := do
  skipChar 'L'
  let n <- digits
  skipChar '\n'
  return -(Int.ofNat n)

def parseRight : Parser Int := do
  skipChar 'R'
  let n <- digits
  skipChar '\n'
  return Int.ofNat n

def parser : Parser (Array Int) :=
  (parseLeft <|> parseRight).many

def step (state : Nat × Int) (move : Int) : Nat × Int :=
  let ⟨count, pos⟩ := state
  let pos' := (pos + move) % 100
  ⟨count + (if pos' == 0 then 1 else 0), pos'⟩

def solve (input : Array Int) : Nat :=
  let ⟨count, _⟩ := input.toList.foldl step ⟨0, 50⟩
  count
