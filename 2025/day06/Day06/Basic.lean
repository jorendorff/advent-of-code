import Std.Internal.Parsec
import Std.Data.HashSet
import Init.Data.String.Slice
import Init.Data.Iterators.Consumers.Collect

open Std.Internal.Parsec
open Std.Internal.Parsec.String

inductive Op : Type
| add : Op
| mul : Op
deriving Repr

def Op.apply (op: Op) (x y : Nat) : Nat :=
  match op with
  | Op.add => x + y
  | Op.mul => x * y

structure Input where
  ops : Array Op
  rows : Array (Array Nat)
deriving Repr

-- # Input parser for part 1

def sp : Parser Unit := many (skipChar ' ') *> return ()

def space : Parser Unit := many1 (skipChar ' ') *> return ()

def nl : Parser Unit := skipChar '\n'

-- Parser combinator for lists with a separator.
def sepBy1 {t : Type} (sep : Parser Unit) (elem : Parser t) : Parser (Array t) := do
  let h <- elem
  manyCore (attempt $ sep *> elem) #[h]

def op : Parser Op :=
  (do skipChar '+'; return Op.add)
  <|> (do skipChar '*'; return Op.mul)

def parser1 : Parser Input := do
  let rows <- many $ attempt (sp *> sepBy1 space digits <* sp <* nl)
  let ops <- sp *> sepBy1 space op <* sp <* nl
  eof
  return Input.mk ops rows

-- # Part 1

def solve1 (input : Input) : Nat :=
  let totals := Array.foldl
    (fun acc row =>
      Array.zipWith
        (fun (op : Op) (pair : Nat × Nat) => op.apply pair.fst pair.snd)
        input.ops
        (acc.zip row))
    input.rows[0]!
    input.rows[1:]
  totals.sum

-- # Input parser for part 2

structure Grid α where
  nc : Nat
  rows : Array (Vector α nc)

def repeatN (p : Parser α) (n : Nat) : Parser (Vector α n) :=
  match n with
    | 0 => return #v[]
    | Nat.succ m => do
      let xs <- repeatN p m
      let x <- p
      return xs.push x

def line (p: Parser α) (n : Nat) : Parser (Vector α n) := do
  repeatN p n <* nl

def elidedSpace : Parser Char := do
  let some _nl <- peekWhen? (· == '\n') | fail "unexpected character"
  return ' '

def grid (p : Parser Char) : Parser (Grid Char) := do
  let first <- many p
  skipChar '\n'
  let nc := first.size
  let rows <- manyCore (line (p <|> elidedSpace) nc) #[first.toVector]
  return Grid.mk nc rows

def parser2 : Parser (Grid Char) := do
  let g <- grid (digit <|> pchar ' ' <|> pchar '+' <|> pchar '*')
  eof
  return g

-- # Part 2

def Grid.transpose (grid : Grid α) : Grid α :=
  Grid.mk
    grid.rows.size
    (Array.finRange grid.nc |>.map (fun i => by
      rw [← Array.size_map]
      exact Array.map (·[i]) grid.rows |>.toVector))

def Grid.toString (grid : Grid Char) : String :=
  grid.rows.flatMap (·.toArray.push '\n') |>.toList |>.asString

def block : Parser (Op × Array Nat) := do
  let first <- sp *> digits
  let op <- sp *> op <* sp      <* nl
  let nums <- manyCore (attempt (sp *> digits <* sp <* nl)) #[first]
  return (op, nums)

def solve2 (input : Grid Char) : Nat :=
  let st := input.transpose.toString
  match sepBy1 (sp *> nl) block st.mkIterator with
  | ParseResult.error _pos _err => 0
  | ParseResult.success _pos blocks =>
    Array.map (fun ⟨op, nums⟩ => Array.foldl op.apply nums[0]! nums[1:]) blocks |> Array.sum
