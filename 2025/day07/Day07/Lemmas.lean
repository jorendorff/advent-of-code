import Day07.Basic

abbrev Grid.nr {α : Type} (grid : Grid α) : Nat := grid.rows.size

abbrev startCell? (grid : Grid Cell) (r : Fin grid.nr) (c : Fin grid.nc) : Prop :=
  grid.rows[r][c] = Cell.start

structure Predecessor {n : Nat} (succ : Fin n) where
  value : Fin n
  proof : value.toNat.succ = succ.toNat

theorem Predecessor.lt {n : Nat} {succ : Fin n} (pred : Predecessor succ)
  : pred.value < succ
:= by
  rw [Fin.lt_def, ←Fin.toNat_eq_val, ←Fin.toNat_eq_val, ←pred.proof]
  apply Nat.lt_succ_self

mutual
  def belowReachableNonSplitter? (grid : Grid Cell) (r : Fin grid.nr) (c : Fin grid.nc) : Prop :=
    ∃ r' : Predecessor r,
      have := r'.lt
      reachable? grid r'.value c
      ∧ grid.rows[r'.value][c] ≠ Cell.splitter
  termination_by (r, 0)

  def byReachableSplitter? (grid : Grid Cell) (r : Fin grid.nr) (c : Fin grid.nc) : Prop :=
    ∃ (r' : Predecessor r) (c' : Fin grid.nc),
      (c'.toNat + 1 = c.toNat ∨ c'.toNat = c.toNat + 1)
      ∧ have := r'.lt
        reachable? grid r'.value c'
      ∧ grid.rows[r][c'] = Cell.splitter
  termination_by (r, 0)

  def reachable? (grid : Grid Cell) (r : Fin grid.nr) (c : Fin grid.nc) : Prop :=
    startCell? grid r c
    ∨ belowReachableNonSplitter? grid r c
    ∨ byReachableSplitter? grid r c
  termination_by (r, 1)
end

theorem solve1_spec (grid : Grid Cell) (hNonEmpty : grid.nr > 0)
: have lastR : Fin grid.nr := (cast · (Fin.last (grid.nr - 1))) (by
    apply congrArg
    sorry)
  solve1 grid = (
    Array.finRange grid.nc
    |>.filter (decide (reachable? grid lastR))
    |>.size)
:= sorry
