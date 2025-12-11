import Std.Internal.Parsec
import Std.Data.HashMap

open Std.Internal.Parsec
open Std.Internal.Parsec.String
open Std(HashMap)

-- # Input parser

abbrev BitSet := Nat

structure Machine where
  lights : BitSet
  wiring : Array (Array Nat)
  wiringAsBits : Array BitSet
  joltage : Array Nat
deriving Repr

abbrev Input := Array Machine

-- Parser combinator for lists with a separator.
def sepBy1 {t : Type} (sep : Parser Unit) (elem : Parser t) : Parser (Array t) := do
  let h <- elem
  manyCore (attempt $ sep *> elem) #[h]

def parser : Parser Input := many do
  skipChar '['
  let lights <- many ((skipChar '.' *> return false) <|> (skipChar '#' *> return true))
  skipChar ']'
  skipChar ' '
  let wiring <- many1 $
    skipChar '(' *> sepBy1 (skipChar ',') digits <* skipChar ')' <* skipChar ' '
  let joltage <- skipChar '{' *> sepBy1 (skipChar ',') digits <* skipChar '}'
  skipChar '\n'
  let lights := lights.foldr (fun bit acc => (acc <<< 1) ||| (if bit then 1 else 0)) 0
  let wiringAsBits := wiring.map (fun nums =>
    nums.foldl (fun acc num => acc ||| (1 <<< num)) 0)
  return {
    lights := lights,
    wiring := wiring,
    wiringAsBits := wiringAsBits,
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

def Machine.successors (m : Machine) (s : State) : Array State :=
  Array.finRange m.wiringAsBits.size
    |>.drop s.i
    |>.map (fun (button : Fin m.wiringAsBits.size) =>
      {lights := s.lights ^^^ m.wiringAsBits[button], i := button + 1})

def Machine.solve (m : Machine) : Nat :=
  let result := search
    m.successors
    (fun state => state.lights == m.lights)
    { lights := 0, i := 0 }
  match result with
  | none => panic! "path not found for machine {repr m}"
  | some path => path.length

def solve1 (input : Input) : Nat :=
  input.map Machine.solve |>.sum

-- # Part 2

def Machine.joltageSuccessors (m : Machine) (levels : Array Nat)
  : Array (Array Nat)
:= Array.finRange m.wiring.size
  |>.filterMap (fun (button : Fin m.wiring.size) =>
    let w := m.wiring[button]
    if w.all (fun i => levels[i]! < m.joltage[i]!)
    then
      let levels' := levels |> w.foldl (fun levels i =>
        let x := levels[i]!
        levels.set! i x.succ)
      some levels'
    else none)

def Machine.solveJoltage (m : Machine) : Nat :=
  let result := search
    m.joltageSuccessors
    (fun levels => levels == m.joltage)
    (Array.replicate m.joltage.size 0)
  match result with
  | none => panic! s!"path not found for machine {repr m}"
  | some path =>
    dbg_trace "{path.length}"
    path.length

def solve2 (input : Input) : Nat :=
  input.map Machine.solveJoltage |>.sum
