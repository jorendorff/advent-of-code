import Std.Internal.Parsec

open Std.Internal.Parsec
open Std.Internal.Parsec.String

-- # Input parser

structure Region where
  nc : Nat
  nr : Nat
  quantity : Array Nat
deriving Repr

structure Input where
  shapes : Array (Array (Array Bool))
  regions : Array Region
deriving Repr

def parser : Parser Input := do
  let shapes <- many1 do
    attempt
      (do let _ <- digits; skipChar ':'; skipChar '\n')
      <|> fail "nope"
    many1 (do
      many1 (
        (skipChar '.' *> pure false)
        <|> (skipChar '#' *> pure true)
      ) <* skipChar '\n')
      <* skipChar '\n'
  let regions <- many do
    let nc <- digits
    skipChar 'x'
    let nr <- digits
    skipChar ':'
    let quantity <- many (skipChar ' ' *> digits)
    skipChar '\n'
    return {nc := nc, nr := nr, quantity := quantity}
  eof
  return {shapes := shapes, regions := regions}


-- # Part 1

def asserting {α : Type} [Inhabited α]
  (p : Prop) [Decidable p] (f : p -> α) : α
:=
  if h : p then f h else panic! "assertion failed"

def Input.canFitAll (input : Input) (region : Region) : Bool :=
  let shape_volume := input.shapes
    |>.map (·.flatMap id |>.filter id |>.size)
  let total_volume :=
    Array.zipWith (· * ·) region.quantity shape_volume |>.sum
  if region.nc * region.nr < total_volume
  then false
  else
    if (region.nc / 3) * (region.nr / 3) >= region.quantity.sum
    then true
    else panic! "can't tell if the package fits"

def solve1 (input : Input) : Nat :=
  input.regions.filter input.canFitAll |>.size

-- # Part 2

def solve2 (input : Input) : Nat := 0
