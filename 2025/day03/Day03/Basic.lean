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

def enumerate {a : Type} (xs : List a) : List (Nat × a) :=
  (List.range xs.length).zip xs

def handleDigit (len : Nat) (k : Nat) (acc : Array Nat) (item : Nat × Nat) : Array Nat :=
  let ⟨i, digit⟩ := item
  if _ : i + k < acc.size + len ∧ acc.size > 0 ∧ acc.back! < digit
  then handleDigit len k acc.pop item
  else if acc.size < k
    then acc.push digit
    else acc
termination_by acc.size

def bankJoltage' (bank : Array Nat) : Nat :=
  bank.toList
    |> enumerate
    |> List.foldl (handleDigit bank.size 12) #[]
    |> Array.foldl (fun acc digit => 10 * acc + digit) 0

def solve2 (input : Input) : Nat :=
  (input.map bankJoltage').sum
