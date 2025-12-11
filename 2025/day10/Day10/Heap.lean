structure Heap (α : Type) where
  compare : α -> α -> Bool
  elems : Array α
  -- the two children of element k are 2k+1 and 2k+2.
  -- compareTrans : ∀ (a b c : α), compare a b ∧ compare b c -> compare a c
  -- compareAntiSym : ∀ (a b : α), compare a b -> ¬compare b a
  -- heapProperty : ∀ k : Fin elems.size, k > 0 -> compare elems[k] elems [(k - 1) / 2]

def Heap.empty (compare : α -> α -> Bool) : Heap α :=
  {compare := compare, elems := #[]}

def Heap.emptyMinHeap {α : Type} [LT α] [DecidableLT α] : Heap α :=
  Heap.empty (fun a b => decide (a < b))

def Heap.size {α : Type} (h : Heap α) : Nat := h.elems.size

def Heap.isEmpty {α : Type} (h : Heap α) : Bool := h.elems.isEmpty

def siftUp (compare : α -> α -> Bool) (elems : Array α) (i : Nat) : Array α :=
  if h : elems.size <= i
  then panic! "out of bounds"
  else if hnz : i = 0
  then elems
  else
    let j := (i - 1) / 2
    if compare elems[j] elems[i]
    then elems
    else siftUp compare (elems.swap i j) j
termination_by i
decreasing_by
  apply Nat.lt_of_le_of_lt
  apply Nat.div_le_self
  apply Nat.sub_lt
  omega
  omega

def Heap.push {α : Type} (heap : Heap α) (x : α) : Heap α :=
  {heap with elems := heap.elems.push x |> (siftUp heap.compare · (heap.size - 1))}

-- termination would be by elems.size - i, requiring manual proof involving Array.size_swap
partial def siftDown (compare : α -> α -> Bool) (elems : Array α) (i : Nat)
: (Array α × Nat) :=
  if _ : 2 * i + 1 < elems.size
  then
    if _ : 2 * i + 2 < elems.size
    then if compare elems[2 * i + 2] elems[i]
      then elems.swap (2 * i + 2) i |> (siftDown compare · (2 * i + 2))
      else if compare elems[2 * i + 1] elems[i]
      then elems.swap (2 * i + 1) i |> (siftDown compare · (2 * i + 1))
      else (elems, i)
    else if compare elems[2 * i + 1] elems[i]
      then elems.swap (2 * i + 1) i |> (siftDown compare · (2 * i + 1))
      else (elems, i)
  else
    (elems, i)

def Heap.pop {α : Type} (heap : Heap α) : (Option α × Heap α) :=
  if heap.isEmpty
  then (none, heap)
  else
    let ⟨arr, i⟩ := siftDown heap.compare heap.elems 0
    if _ : arr.size <= i
    then
      dbg_trace "internal error: out of bounds"
      (none, heap)
    else
      let v := arr[i]
      let arr := arr.swapIfInBounds i (arr.size - 1)
      let arr := siftUp heap.compare arr.pop (heap.size - 1)
      (some v, {heap with elems := arr})
