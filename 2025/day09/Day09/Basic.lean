import Init.Data.Array.QSort.Basic
import Std.Internal.Parsec
import Std.Data.HashSet

open Std.Internal.Parsec
open Std.Internal.Parsec.String
open Std(HashSet)

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

-- ## General algorithms

-- Wrapping +1 function for array indices.
def Fin.wrappingSucc {n : Nat} (i : Fin n) : Fin n :=
  -- Addition is wrapping in `Fin n`, so all we have to do is write `i + 1`.
  -- But first Lean makes us prove `n` is nonzero, because
  -- if `n = 0`, then `Fin n` has no values, so we can't write `1`.
  have nz : NeZero n := by constructor; apply Nat.ne_of_gt; exact Fin.pos i
  i + 1

-- Set one cell of an array of arrays.
def poke {α : Type} {nr nc : Nat}
  (arr : Array (Array α)) (r : Fin nr) (c : Fin nc) (value : α)
: Array (Array α) :=
  let ⟨row, arr⟩ := arr.swapAt! r #[]
  arr.set! r $ row.set! c value

-- Like the C++ std::partial_sum algorithm, replaces each element of `arr`
-- with the sum of all values up to and including that element.
def partialSums {α : Type} [Zero α] [Add α] (arr : Array α) : Array α :=
  let indices := Array.finRange arr.size
  (0, arr.toVector)
    |> indices.foldl (fun ⟨sum, arr⟩ i =>
      let sum := sum + arr[i]
      (sum, arr.set i sum))
    |>.snd
    |>.toArray

-- In an array of arrays, do partial sums vertically. That is, replace each
-- element `arr[r][c]` with the sum of that value and all elements above it,
-- `arr[0..r][c]`.
def columnPartialSums {α : Type} [Zero α] [Add α] (arr : Array (Array α))
: Array (Array α) :=
  if h : 0 < arr.size
  then
    arr.foldl
      (fun ⟨latest, acc⟩ row =>
        let row := row.zipWith (· + ·) latest
        (row, acc.push row))
      (Array.replicate arr[0].size 0, #[])
    |>.snd
  else
    arr

-- Binary search.
--
-- Return the minimum value `i` in `start..stop` for which `test i` is `True`;
-- or `stop` if `test i` is not true for any of those numbers (or `stop < start`).
--
-- `test` must be monotone: that is, `test i < test j -> i < j`.
def bisectRange {limit : Nat} (test : Fin limit -> Bool)
(start : Nat) (stop : Nat) (hStop : stop <= limit): Fin (limit + 1) :=
  if h : start < stop
  then
    let mid := (stop + start) / 2
    have : mid < stop := by
      rw [Nat.div_lt_iff_lt_mul, Nat.mul_two, Nat.add_lt_add_iff_left]
      exact h
      exact Nat.zero_lt_two
    have : NeZero limit := by
      constructor; apply Nat.ne_zero_of_lt; exact Nat.lt_of_lt_of_le h hStop
    if test $ Fin.ofNat limit mid
    then bisectRange test start mid (by omega)
    else bisectRange test (mid + 1) stop (by omega)
  else Fin.ofNat (limit + 1) start

-- Return the minimum value `i` in `0..n` for which `test i` is `True`;
-- or `n` if `test i` is not true for any of those numbers (or n = 0).
--
-- `test` must be monotone: that is, `test i < test j -> i < j`.
def bisect (n : Nat) (test : Fin n -> Bool) : Fin (n + 1) :=
  bisectRange test 0 n (Nat.le_refl n)

example : bisect 4 (5 <= #[1, 4, 5, 6][·]) = 2 := by simp [bisect, bisectRange]

-- def bisect_range_correct (limit : Nat) (test : Fin limit -> Bool)
-- (start : Nat) (stop : Nat) (hstop : stop <= limit) (i : Fin (limit + 1))
-- (hStart : start <= i.toNat) (hStop : i.toNat <= stop)
-- (hMonotone : ∀ j : Fin limit, test j == (i.toNat <= j.toNat))
-- : bisectRange test start stop hstop = i
-- := by sorry
--
-- def bisect_correct (n : Nat) (test : Fin n -> Bool) (i : Fin (n + 1))
-- : (∀ j : Fin n, test j == (i.toNat <= j.toNat)) -> bisect n test == i
-- := by sorry


-- ## Particulars

structure BitMap where
  points : HashSet (Int × Int)
  xc : Array Int
  yc : Array Int
  map : Array (Array Bool)

def BitMap.fromPoints (input : Input) : BitMap :=
  -- Our basic approach is to compress the grid to only coordinates that
  -- actually have points. So the first step is to make:
  -- Sorted lists of all x coordinates and all y coordinates
  let xc := input.map Prod.fst |> Array.qsortOrd |> Array.eraseReps
  let yc := input.map Prod.snd |> Array.qsortOrd |> Array.eraseReps
  -- Functions mapping coordinates back to indices in those arrays.
  let find_xc := fun x => bisect xc.size (x <= xc[·])
  let find_yr := fun y => bisect yc.size (y <= yc[·])
  -- Now plot the red tiles in a grid.
  -- Again, notionally the grid contains only rows and columns which have red tiles.
  -- We leave a margin of 1 to the right and bottom to appease the type system.
  let redTiles := Array.replicate (yc.size + 1) (Array.replicate (xc.size + 1) false)
  let range := Array.finRange input.size
  let redTiles := redTiles
    |> range.foldl (fun redTiles i =>
      let a := input[i]
      let c := find_xc a.1
      let r := find_yr a.2
      poke redTiles r c true)
  let rows := Array.replicate (yc.size + 1) (Array.replicate (xc.size + 1) 0)
  let rows := rows
    |> range.foldl (fun (rows : Array (Array Int)) (i : Fin input.size) =>
      let a := input[i]
      let b := input[i.wrappingSucc]
      if a.1 == b.1
      then -- vertical stroke
        let c := find_xc a.1
        let ra := find_yr a.2
        let rb := find_yr b.2
        rows
          |> (poke · ra c 1)
          |> (poke · rb c (-1))
      else
        if a.2 == b.2
        then rows -- horizontal stroke, do nothing
        else panic! "diagonal line detected")
  let rows := columnPartialSums rows
    |>.map (fun row => partialSums row |>.map (· != 0))
  {points := HashSet.ofArray input, xc := xc, yc := yc, map := rows}

def rectArea (x₀ : Int) (y₀ : Int) (x₁ : Int) (y₁ : Int) : Int :=
  -- dbg_trace "      found rectangle ({x₀}, {y₀}) - ({x₁}, {y₁})"
  (x₁ - x₀ + 1) * (y₁ - y₀ + 1)

def maybePopStack
  (points : HashSet (Int × Int))
  (stack : Array (Int × Int))
  (x₁ : Int) (y₁ : Int) (y : Option Int)
: Array (Int × Int) :=
  if h : stack.size = 0
  then stack
  else
    let ⟨x₀, y₀⟩ := stack.back
    if y₀ < y.getD (y₀ + 1)
    then
      -- dbg_trace "    popping stack"
      maybePopStack points stack.pop x₁ y₁ y
    else stack
termination_by stack.size

def maybePushStack
  (points : HashSet (Int × Int))
  (stack : Array (Int × Int)) (x : Int) (sky : Option Int)
: Array (Int × Int) :=
  match sky with
  | none => stack
  | some y =>
    let shouldPush :=
      (x, y) ∈ points
      && if h : stack.size = 0
        then true
        else stack.back.snd > y
    if shouldPush
    then
      -- dbg_trace "    pushing point {(x, y)} "
      stack.push (x, y)
    else stack

def showRow (row : Array Bool) : String :=
  row.foldl (fun s b => s.push (if b then '#' else '.')) ""

def maybeImproveBest
  (points : HashSet (Int × Int))
  (stack : Array (Int × Int))
  (x : Int)
  (y : Int)
  (best : Int)
: Int :=
  if (x, y) ∈ points
  then
    stack
      |>.filter (· ∈ points)
      |>.map (fun ⟨x₀, y₀⟩ => rectArea x₀ y₀ x y)
      |>.foldl max best
  else best

def handleRow
  (points : HashSet (Int × Int))
  (xc : Array Int)
  (skyline : Array (Option Int))
  (y : Int)
  (row : Array Bool)
: (Array (Option Int) × Int) :=
  -- dbg_trace "  handleRow y={y}, row={showRow row}"
  let ⟨_stack, skyline', rowBest⟩ := xc.zip (skyline.zip row)
    |>.foldl
      (fun ⟨stack, skyline', rowBest⟩ ⟨x, sky, cell⟩ =>
        let rowBest := maybeImproveBest points stack x y rowBest
        let stack := maybePopStack points stack x y sky
        let stack := maybePushStack points stack x sky
        let sky' := if cell then sky.or (some y) else none
        (stack, skyline'.push sky', rowBest))
      (#[], #[], 0)
  -- dbg_trace "    done, skyline={repr skyline'}, rowBest={rowBest}"
  (skyline', rowBest)

def BitMap.maxRectArea (b : BitMap) : Int :=
  let ⟨_skyline, best⟩ := b.yc.zip b.map |>.foldl
    (fun ⟨skyline, best⟩ ⟨y, row⟩ =>
      let ⟨skyline, rowBest⟩ := handleRow b.points b.xc skyline y row
      (skyline, max rowBest best))
    (Array.replicate b.xc.size none, 0)
  -- dbg_trace "answer: {best}"
  best

def flipPoints (arr : Input) : Input :=
  let ymax := arr.map Prod.snd |>.foldl max 0
  arr.map (fun ⟨x, y⟩ => (x, ymax + 1 - y))

def solve2 (input : Input) : Int :=
  let rows := BitMap.fromPoints input
  let rows' := BitMap.fromPoints (flipPoints input)
  max rows.maxRectArea rows'.maxRectArea
