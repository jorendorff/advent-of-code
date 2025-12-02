import Std.Internal.Parsec

open Std.Internal.Parsec.String

-- Input parser

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

-- Part 1

def numbers : Int := 100

def step1 (state : Nat × Int) (move : Int) : Nat × Int :=
  let ⟨count, pos⟩ := state
  let pos' := (pos + move) % numbers
  ⟨count + (if pos' == 0 then 1 else 0), pos'⟩

def solve1 (input : Array Int) : Nat :=
  let ⟨count, _⟩ := input.toList.foldl step1 ⟨0, 50⟩
  count

-- Part 2

-- Return a list of n copies of the given value v.
def rep {t : Type} (v : t) (n : Nat) : List t :=
  match n with
  | 0 => []
  | Nat.succ n' => List.cons v $ rep v n'

def tickify (move : Int) : List Int :=
  match move with
  | Int.ofNat n => rep 1 n
  | Int.negSucc n => rep (-1) n.succ

def solve2 (input : Array Int) : Nat :=
  let ticks := input.toList.flatMap tickify
  let ⟨count, _⟩ := ticks.foldl step1 ⟨0, 50⟩
  count
