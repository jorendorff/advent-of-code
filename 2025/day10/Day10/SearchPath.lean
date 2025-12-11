import Std.Data.HashMap

open Std(HashMap)

-- # Breadth-first search

inductive Path (α β : Type)
| nil : α -> Path α β
| append : Path α β -> β -> α -> Path α β
deriving Inhabited

def Path.length {α β : Type} (path : Path α β) : Nat :=
  match path with
  | Path.nil _ => 0
  | Path.append p _ _ => p.length.succ

def Path.end {α β : Type} (path : Path α β) : α :=
  match path with
  | Path.nil a => a
  | Path.append _ _ a => a

partial def searchLoop {α β : Type} [Hashable α] [BEq α] [Repr α] [Repr β]
  (outEdges : α -> Array (β × α))
  (isExit : α -> Bool)
  (queue : Array (Path α β))
  (i : Nat)
  (seen : HashMap α (Path α β))
  : Option (Path α β)
:=
  if h : i >= queue.size
  then none
  else
    let pathSoFar := queue[i]
    let node := pathSoFar.end
    -- Use the Error monad because it provides a way to `break` out of a loop.
    let result := outEdges node
      |>.foldlM (fun ⟨seen, queue⟩ ⟨edge, node'⟩ => do
        if seen.contains node'
        then pure (seen, queue) -- continue
        else
          let newPath := pathSoFar.append edge node'
          if isExit node'
          then Except.error newPath -- break
          else pure (seen.insert node' newPath, queue.push newPath))
        (seen, queue)
    match result with
    | Except.error path => some path
    | Except.ok ⟨seen, queue⟩ =>
      searchLoop outEdges isExit queue i.succ seen

def search {α β : Type} [Hashable α] [BEq α] [Repr α] [Repr β]
  (outEdges : α -> Array (β × α))
  (isExit : α -> Bool)
  (start : α)
  : Option (Path α β)
:=
  let startPath := Path.nil start
  if isExit start
  then some startPath
  else
    searchLoop outEdges isExit
      #[startPath]
      0
      (HashMap.ofList [(start, startPath)])
