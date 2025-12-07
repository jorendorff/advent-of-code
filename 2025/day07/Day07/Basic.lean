import Std.Internal.Parsec

open Std.Internal.Parsec
open Std.Internal.Parsec.String

-- # Input parser

structure Grid α where
  nc : Nat
  rows : Array (Vector α nc)

inductive Cell
| empty
| start
| splitter
deriving BEq

def repeatN (p : Parser α) (n : Nat) : Parser (Vector α n) :=
  match n with
    | 0 => return #v[]
    | Nat.succ m => do
      let xs <- repeatN p m
      let x <- p
      return xs.push x

def line (p: Parser α) (n : Nat) : Parser (Vector α n) := do
  let elems <- repeatN p n
  skipChar '\n'
  return elems

def grid (p : Parser α) : Parser (Grid α) := do
  let first <- many p
  skipChar '\n'
  let nc := first.size
  let rows <- manyCore (line p nc) #[first.toVector]
  return Grid.mk nc rows

def Input := Grid Cell

def cell : Parser Cell :=
  (skipChar '.' *> return Cell.empty)
  <|> (skipChar 'S' *> return Cell.start)
  <|> (skipChar '^' *> return Cell.splitter)

def parser : Parser Input := grid cell <* eof

-- # Part 1

def hasBeam {nc: Nat} (row : Vector Cell nc) (beams : Vector Bool nc) (i : Fin nc) : Bool :=
  row[i] == Cell.start
  || (beams[i] && row[i] == Cell.empty)
  || (
    i.val > 0
    && beams.getD (i - 1) False
    && row.getD (i - 1) Cell.empty == Cell.splitter)
  || (
    beams.getD (↑i + 1) False
    && row.getD (↑i + 1) Cell.empty == Cell.splitter)

def solve1 (input : Input) : Nat :=
  let step := fun ⟨count, beams⟩ row => (
    -- count splitters hit by beams
    count + (beams.zip row
      |>.map (fun | ⟨Bool.true, Cell.splitter⟩ => 1 | _ => 0)
      |>.sum),
    -- beams afterwards
    Vector.finRange input.nc
      |>.map (hasBeam row beams))

  let ⟨count, _beams⟩ := Array.foldl
    step
    (0, Vector.replicate input.nc false)
    input.rows
  count

-- # Part 2

def countTimelines {nc: Nat} (row : Vector Cell nc) (timelines : Vector Nat nc)
  (i : Fin nc) : Nat
:=
  (if row[i] == Cell.start then 1
    else if row[i] == Cell.empty then timelines[i]
    else 0)
  + (
    if i.val > 0 && row.getD (i - 1) Cell.empty == Cell.splitter
    then timelines.getD (i - 1) 0
    else 0)
  + (
    if row.getD (↑i + 1) Cell.empty == Cell.splitter
    then timelines.getD (↑i + 1) 0
    else 0)

def solve2 (input : Input) : Nat :=
  -- because the tachyon manifold is quantum, the step becomes a leap
  let leap := fun timelines row =>
    Vector.finRange input.nc |>.map (countTimelines row timelines)
  Array.foldl leap (Vector.replicate input.nc 0) input.rows |>.sum
