import Std.Internal.Parsec

open Std.Internal.Parsec
open Std.Internal.Parsec.String

-- # Input parser

def Input := Array (Array Nat)

def battery : Parser Nat := do
  let d <- digit
  return d.toNat - '0'.toNat

def bank : Parser (Array Nat) := do
  let chars <- many battery
  skipChar '\n'
  return chars

def parser : Parser Input := many bank

-- # Part 1

def bankJoltage (bank : Array Nat) : Nat :=
  let acc := fun (state : Nat × Nat) (bat : Nat) => (
    let ⟨pairMax, batMax⟩ := state
    (max pairMax (10 * batMax + bat), max batMax bat))
  let ⟨pairMax, _⟩ := Array.foldl acc ⟨0, 0⟩ bank
  pairMax

def solve1 (input : Input) : Nat :=
  (input.map bankJoltage).sum

-- # Part 2

-- Append `bat` to `seq`, then remove any element of `seq`
-- to maximize the result.
def boost (seq : Array Nat) (bat : Nat) (i : Nat) (hi : i < seq.size): Array Nat :=
  if hi' : i + 1 < seq.size
  then if seq[i] < seq[i + 1]
       then seq[0:i] ++ seq[i + 1:] ++ #[bat]
       else boost seq bat (i + 1) hi'
  else if seq[i] < bat
       then seq[0:i] ++ #[bat]
       else seq

def bankJoltage' (bank : Array Nat) : Nat :=
  let acc := fun (seqMax : Array Nat) (bat : Nat) => (
    if h : seqMax.size < 12
    then seqMax ++ #[bat]
    else boost seqMax bat 0 (by omega)
  )
  let seqMax := Array.foldl acc #[] bank
  Array.foldl (fun acc digit => 10 * acc + digit) 0 seqMax

def solve2 (input : Input) : Nat :=
  (input.map bankJoltage').sum
