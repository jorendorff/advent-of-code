import Std.Internal.Parsec

open Std.Internal.Parsec
open Std.Internal.Parsec.String

-- # Input parser

abbrev Input := Array (Array Bool)

def cell : Parser Bool :=
  (do skipChar '@'; return true) <|> (do skipChar '.'; return false)

def parser : Parser Input := many (many cell <* skipChar '\n')

-- # Part 1

def dirs : Array (Int × Int) := #[
  (-1, -1), (-1, 0), (-1, 1),
  (0, -1),           (0, 1),
  (1, -1),  (1, 0),  (1, 1),
]

def countAdjRolls (input : Array (Array Bool)) (nr : Nat) (nc : Nat) (r : Nat) (c : Nat) : Nat :=
  dirs
    |> Array.map (fun ⟨dr, dc⟩ =>
      if 0 <= r + dr && r + dr < nr && 0 <= c + dc && c + dc < nc
      then if input[(r + dr).natAbs]![(c + dc).natAbs]! == some true
        then 1
        else 0
      else 0
    )
    |> Array.sum

def solve1 (input : Input) : Nat :=
  let nr := input.size
  let nc := (input.getD 0 #[]).size
  let rows := Array.range nr
  let cols := Array.range nc
  rows
    |>.map (fun r =>
      cols
        |>.map (fun c =>
          if (input.getD r #[]).getD c false == true
          then
            let adjRolls := countAdjRolls input nr nc r c
            if adjRolls < 4 then 1 else 0
          else
            0)
        |>.sum)
    |>.sum

-- # Part 2

def countRolls (input : Array (Array Bool)) : Nat :=
  input.foldl
    (fun acc row =>
      acc + row.foldl (fun acc cell => acc + if cell then 1 else 0) 0)
    0

def iterate {α : Type} (step : α -> Nat -> (α × Nat)) (state : α) (metric : Nat) : (α × Nat) :=
  let ⟨state', metric'⟩ := step state metric
  if metric' < metric
  then iterate step state' metric'
  else (state', metric')

-- Of course the right way would be to store an adjacency count in each cell
-- On removing one, decrement its neighbors and enqueue all those that dip below 4.
def clearSome (input : Array (Array Bool)) (count : Nat) : (Array (Array Bool) × Nat) :=
  let nr := input.size
  let nc := (input.getD 0 #[]).size
  let rows := Array.range nr
  let cols := Array.range nc
  let clearOne := fun (grid : Array (Array Bool)) (⟨r, c⟩ : (Nat × Nat)) =>
    grid.set! r $ grid[r]!.set! c false
  let points := rows
    |> Array.flatMap (fun r =>
      cols
        |> Array.flatMap (fun c =>
          if (input.getD r #[]).getD c false == false
          then #[]
          else if countAdjRolls input nr nc r c >= 4
              then #[]
              else #[(r, c)]))
  let input' := points.foldl clearOne input
  let count' := count - points.size
  (input', count')

def solve2 (input : Array (Array Bool)) : Nat :=
  let count := countRolls input
  let ⟨_grid, count'⟩ := iterate clearSome input count
  count'
