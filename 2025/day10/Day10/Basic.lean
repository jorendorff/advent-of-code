import Std.Internal.Parsec
import Std.Data.HashMap

open Std.Internal.Parsec
open Std.Internal.Parsec.String
open Std(HashMap)

-- # Input parser

abbrev BitSet := Nat

structure Machine where
  numLevels : Nat  -- number of joltage levels
  lights : Vector Bool numLevels
  wiring : Array (Array (Fin numLevels))
  joltage : Vector Nat numLevels
deriving Repr

abbrev Input := Array Machine

-- Parser combinator for lists with a separator.
def commaSep1 {t : Type} (elem : Parser t) : Parser (Array t) := do
  let h <- elem
  manyCore (skipChar ',' *> elem) #[h]

def parseFin (n : Nat) : Parser (Fin n) := do
  let x <- digits
  if h : x < n
  then
    have : NeZero n := ⟨Nat.ne_zero_of_lt h⟩
    return Fin.ofNat n x
  else fail s!"number out of range: {x}"

def parser : Parser Input := many do
  skipChar '['
  let lights <- many ((skipChar '.' *> return false) <|> (skipChar '#' *> return true))
  skipChar ']'
  skipChar ' '
  let n := lights.size
  let wiring <- many1 $
    skipChar '(' *> commaSep1 (parseFin n) <* skipChar ')' <* skipChar ' '
  let joltage <- skipChar '{' *> commaSep1 digits <* skipChar '}'
  let joltage <-
    if h : joltage.size = n
    then pure $ cast (by rw [h]) joltage.toVector
    else fail s!"number of joltage levels should be = number of display lights"
  skipChar '\n'
  return {
    numLevels := n,
    lights := lights.toVector,
    wiring := wiring,
    joltage := joltage
  }

-- # Breadth-first search

structure Path (α : Type) where
  last : α
  length : Nat

partial def searchLoop {α : Type} [Hashable α] [BEq α]
  (outEdges : α -> Array (α))
  (isExit : α -> Bool)
  (queue : Array (Path α))
  (i : Nat)
  (seen : HashMap α (Path α))
  : Option (Path α)
:=
  if h : i >= queue.size
  then none
  else
    let pathSoFar := queue[i]
    let node := pathSoFar.last
    -- Use the Error monad because it provides a way to `break` out of a loop.
    let result := outEdges node
      |>.foldlM (fun ⟨seen, queue⟩ node' => do
        if seen.contains node'
        then pure (seen, queue) -- continue
        else
          let newPath := {last := node', length := pathSoFar.length.succ}
          if isExit node'
          then Except.error newPath -- break
          else pure (seen.insert node' newPath, queue.push newPath))
        (seen, queue)
    match result with
    | Except.error path => some path
    | Except.ok ⟨seen, queue⟩ =>
      searchLoop outEdges isExit queue i.succ seen

def search {α : Type} [Hashable α] [BEq α]
  (outEdges : α -> Array α)
  (isExit : α -> Bool)
  (start : α)
  : Option (Path α)
:=
  let startPath := {last := start, length := 0}
  if isExit start
  then some startPath
  else
    searchLoop outEdges isExit
      #[startPath]
      0
      (HashMap.ofList [(start, startPath)])

-- # Part 1

structure State where
  lights : BitSet
  -- next element of `wiring` to consider
  i : Nat
deriving BEq, Hashable, Repr

def Machine.solve (m : Machine) : Nat :=
  let lights := m.lights.foldr
    (fun bit acc => (acc <<< 1) ||| (if bit then 1 else 0))
    0
  let wiringAsBits := m.wiring.map (fun nums =>
    nums.foldl (fun acc num => acc ||| (1 <<< num.toNat)) 0)

  let successors := fun (s : State) =>
    Array.finRange wiringAsBits.size
      |>.drop s.i
      |>.map (fun (button : Fin wiringAsBits.size) =>
        {lights := s.lights ^^^ wiringAsBits[button], i := button + 1 : State})

  let result := search
    successors
    (fun state => state.lights == lights)
    { lights := 0, i := 0 }
  match result with
  | none => panic! "path not found for machine {repr m}"
  | some path => path.length

def solve1 (input : Input) : Nat :=
  input.map Machine.solve |>.sum

-- # Part 2


structure State2 (m : Machine) where
  levels : Vector Nat m.numLevels
  -- next button to consider
  i : Fin m.wiring.size
deriving BEq, Repr

-- can't derive Hashable because Vectors aren't hashable for some reason
instance {m : Machine} : Hashable (State2 m) where
  hash s := mixHash (hash s.levels.toArray) (hash s.i)

def Machine.joltageSuccessors (m : Machine) (s : State2 m) : Array (State2 m) :=
  --dbg_trace "looking for paths forward from {repr s}"
  let options := Array.emptyWithCapacity 2
  let options :=
    let w := m.wiring[s.i]
    let levels' := s.levels |> w.foldl (fun levels i =>
      let x := levels[i]!
      levels.set! i x.succ)
    if w.all (fun i => levels'[i]! <= m.joltage[i]!)
    then options.push {s with levels := levels'}
    else options
  let options :=
    if h1 : s.i.succ < m.wiring.size
    then
      have : NeZero m.wiring.size := by constructor; exact Nat.ne_zero_of_lt h1
      options.push {s with i := s.i + 1}
    else options
  --dbg_trace "  found: {repr options}"
  options

def Machine.solveJoltage (m : Machine) : Nat :=
  if h : m.wiring.size = 0
  then
    if m.joltage.all (· = 0) then 0 else panic! "no buttons to mash"
  else
    have : NeZero m.wiring.size := NeZero.mk h
    let result := search
      m.joltageSuccessors
      (fun s => s.levels == m.joltage)
      {levels := Vector.replicate m.numLevels 0, i := 0}
    match result with
    | none => panic! s!"path not found for machine {repr m}"
    | some path =>
      dbg_trace "{path.length}"
      path.length

def solve2 (input : Input) : Nat :=
  input.map Machine.solveJoltage |>.sum
