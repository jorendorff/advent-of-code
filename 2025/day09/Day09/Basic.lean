import Init.Data.Array.QSort.Basic
import Std.Internal.Parsec
import Std.Data.TreeMap

open Std.Internal.Parsec
open Std.Internal.Parsec.String
open Std(TreeMap)

-- # Input parser

abbrev Input := Array (Int × Int)

def parser : Parser Input := many do
  let left <- digits <* skipChar ','
  let right <- digits <* skipChar '\n'
  return (left, right)

-- # Part 1

def combinations {α} (input : Array α) : Array (α × α) :=
  let is := Array.finRange input.size
  is |>.flatMap fun i =>
    is[i.succ:].toArray |>.map fun j =>
      (input[i], input[j])

def abs (x : Int) : Int :=
  if x < 0 then -x else x

def solve1 (input : Input) : Int :=
  combinations input
    |>.map (fun ⟨a, b⟩ =>
      (abs (a.1 - b.1) + 1) * (abs (a.2 - b.2) + 1))
    |>.foldl max 0

-- # Part 2

def uniq {α : Type} [BEq α] [Inhabited α] (arr : Array α) : Array α :=
  Array.finRange arr.size
    |>.foldl
      (fun ⟨arr, prev, w⟩ r =>
        let v := arr[r]!
        if prev == some v
        then (arr, prev, w)
        else ⟨arr.set! w v, some v, w + 1⟩)
      (arr, none, 0)
    |> (·.1)

structure Corner where
  isTop : Bool
  winding : Int -- 1 if top left or bottom right corner, -1 top right or bottom left

-- A row of corners that all share the same y coordinate.
-- Maps x coordinates to facts about that corner.
def Row := TreeMap Int Corner

abbrev AllCorners := TreeMap Int Row

def Row.insert (row : Row) (x : Int) (isTop : Bool) (wind : Int) : Row :=
  row.alter x fun
    | none => some (Corner.mk isTop wind)
    | _ => panic! "oh no repeated tile"

-- Wrapping +1 function for array indices.
def Fin.wrappingSucc {n : Nat} (i : Fin n) : Fin n :=
  -- Addition is wrapping in `Fin n`, so all we have to do is write `i + 1`.
  -- But first Lean makes us prove `n` is nonzero, because
  -- if `n = 0`, then `Fin n` has no values, so we can't write `1`.
  have nz : NeZero n := by constructor; apply Nat.ne_of_gt; exact Fin.pos i
  i + 1

def tabulateCorners (input : Input) : AllCorners :=
  Array.finRange input.size
    |>.foldl (fun (rows : TreeMap Int Row) (i : Fin input.size) =>
      let a := input[i]
      let b := input[i.wrappingSucc]
      if a.1 == b.1
      then -- vertical stroke
        let x := a.1
        if a.2 < b.2
        then -- downward stroke
          rows
            |>.alter a.2 (fun (row : Option Row) =>
              row.getD TreeMap.empty
                |>.insert x True 1
                |> some)
            |>.alter b.2 (fun row =>
              row.getD TreeMap.empty
                |>.insert x False (-1)
                |> some)
        else -- upward stroke
          rows
            |>.alter b.2 (fun row =>
              row.getD TreeMap.empty
                |>.insert x True (-1)
                |> some)
            |>.alter a.2 (fun row =>
              row.getD TreeMap.empty
                |>.insert x False 1
                |> some)
      else
        if a.2 == b.2
        then rows -- horizontal stroke, do nothing
        else panic! "diagonal line detected")
      default

structure SkyPoint where
  y : Int
  x : Int
  winding : Int

structure State where
  -- Info about the green-and-red-tiled area above the current row
  -- (i.e. having lower y coordinates).
  --
  -- a new row of corners affects this as follows:
  -- . top corners add new points
  -- . bottom corners remove points
  -- . both kinds can affect the winding of subsequent points
  skyline : Array SkyPoint
  -- maximum area of any rectangle that can be drawn in the figure so far
  best : Int
deriving Nonempty

structure InnerState where
  -- y coordinate of the current row
  y : Int
  -- current winding of a scan line immediately below the current row
  winding : Int
  -- remaining corners (red tiles) in the current row
  -- items are dropped from the front of this as we scan left to right
  row : List (Int × Corner)
  -- remaining points in the skyline of prevState
  -- items are dropped from the front of this as we scan left to right
  skyline : List SkyPoint
  -- upper left corners of potential rectangles
  stack : Array (Int × Int)
  -- output to pass on to the next row
  nextSkyline : Array SkyPoint
  -- maximum rectangle area discovered so far
  best : Int


def InnerState.handleCorner (s : InnerState)
  (x : Int) (corner : Corner) : InnerState
:=
  let w' := s.winding + corner.winding
  -- if this is a new top corner of an ???
  {s with winding := w'}

-- If the new point has a winding of 0, that means a vertical stroke cuts across
-- the current row here, so pop all points.
--
-- Whenever we pop a point, we check for the largest rectangle that could be
-- made with it as the top left corner.
--
-- If the stack is empty, or this point is higher than the previous skyPoint,
-- we just push it.
--
-- if this point is lower than the previous skyPoint, we need to pop
-- one or more points, checking for rectangles as we go; and push a stack point
-- with the x coordinate of the last point popped and the y of `point`.
def InnerState.handleSkyPoint (s : InnerState)
  (point : SkyPoint) : InnerState
:=
  sorry

-- We are scanning left to right, reacting to vertical-stroke endpoints
-- on this line and keeping an eye on the skyline. So this function mainly
-- merges the two event sources for this left-to-right pass.
partial def InnerState.runLoop (s : InnerState) : State
:= match (s.skyline, s.row) with
  | ⟨[], []⟩ => {skyline := s.nextSkyline, best := s.best}
  | ⟨[], List.cons ⟨x, corner⟩ row'⟩ =>
    {s.handleCorner x corner with row := row'}.runLoop
  | ⟨List.cons point skyline', []⟩ =>
    {s.handleSkyPoint point with skyline := skyline'}.runLoop
  | ⟨List.cons point skyline', List.cons ⟨x, corner⟩ row'⟩ =>
    if point.x < x
    then {s.handleSkyPoint point with skyline := skyline'}.runLoop
    else {s.handleCorner x corner with row := row'}.runLoop

def solve2 (input : Input) : Int :=
  let rows := tabulateCorners input
  -- outer loop - iterate over rows, y increasing
  {skyline := #[], best := 0 : State}
    |> rows.foldl (fun (state : State) (y : Int) (row : Row) =>
      -- inner loop - iterate over corners, x increasing
      let innerState : InnerState := {
        y := y,
        winding := 0,
        row := row.toList,
        skyline := state.skyline.toList,
        stack := #[],
        nextSkyline := #[],
        best := state.best,
      }
      innerState.runLoop)
    |>.best
