import Std.Internal.Parsec
import Init.Data.Array.QSort.Basic

open Std.Internal.Parsec
open Std.Internal.Parsec.String

-- # Input parser

structure Point where
  x : Int
  y : Int
  z : Int
deriving BEq, Repr, Inhabited

abbrev Input := Array Point

def triple : Parser Point := do
  let x <- digits
  skipChar ','
  let y <- digits
  skipChar ','
  let z <- digits
  skipChar '\n'
  return ⟨x, y, z⟩

def parser : Parser Input := many triple

-- # Part 1

def sqr (x : Int) : Int := x * x

def distance2 (a b : Point) : Int :=
  sqr (b.x - a.x) + sqr (b.y - a.y) + sqr (b.z - a.z)

structure UnionFind (n : Nat) where
  map : Vector (Fin n) n
  count : Fin (n + 1)

def UnionFind.new (n : Nat) : UnionFind n :=
  UnionFind.mk
    (Vector.finRange n)
    (Fin.mk n (Nat.lt_succ_self n))

partial def UnionFind.find {n : Nat} (u : UnionFind n) (i : Fin n)
  : (UnionFind n × Fin n)
:=
  if u.map[i] == i
  then (u, i)
  else
    let ⟨u, x⟩ := find u u.map[i]
    (UnionFind.mk (u.map.set i x) u.count, x)

partial def UnionFind.union {n : Nat} (u : UnionFind n) (a b : Fin n)
  : UnionFind n
:=
  let ⟨u, a⟩ := u.find a
  let ⟨u, b⟩ := u.find b
  if a != b
  then UnionFind.mk (u.map.set a b) (u.count - 1)
  else u

def histogram {n : Nat} (u : UnionFind n) : Vector Nat n :=
  Array.finRange n
    |>.foldl
      (fun ⟨u, counts⟩ i =>
        let ⟨u, grpid⟩ := u.find i
        let c := counts[grpid]
        (u, counts.set grpid (c + 1)))
      (u, Vector.replicate n 0)
    |>.snd

def enumerate (arr : Array α) : Array (Fin arr.size × α) :=
  Array.finRange arr.size |>.zip arr

def k : Nat := 1000

def all_pairs (input : Input) : Array (Int × Fin input.size × Fin input.size) :=
  enumerate input
    |>.flatMap (fun ⟨i, pi⟩ =>
      enumerate input |>.filterMap (fun ⟨j, pj⟩ =>
        if i >= j
        then none
        else some (distance2 pi pj, i, j)))
    |>.qsort (fun a b => a.1 < b.1)

def solve1 (input : Input) : Nat :=
  let top_k := all_pairs input |>.take k
  UnionFind.new input.size
    |> top_k.foldl (fun u ⟨_dist, i, j⟩ => u.union i j)
    |> histogram
    |>.toArray
    |>.qsortOrd
    |> (fun arr => arr[arr.size - 3:].toArray)
    |>.filter (fun c => c != 0)
    |>.foldl (fun a b => a * b) 1

-- # Part 2

def counting_foldl_until {α β} (done? : α -> Bool)
  (step : α -> β -> α) (state : α) (list : List β)
  (count : Nat := 0) : Nat
:=
  if done? state
  then count
  else match list with
    | [] => count
    | List.cons h t =>
      counting_foldl_until done? step (step state h) t (count + 1)

def unified? (u : UnionFind n) : Bool :=
  u.count == 1

def solve2 (input : Input) : Int :=
  let pairs := all_pairs input
  let count := counting_foldl_until
    unified?
    (fun u ⟨_dist, i, j⟩ => u.union i j)
    (UnionFind.new input.size)
    pairs.toList
  match pairs[count - 1]? with
  | none => panic! "oh no what are you doing"
  | some ⟨_d, i, j⟩ => input[i].x * input[j].x
