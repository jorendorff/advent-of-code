import Day06

import Init.Data.Repr
import Std.Internal.Parsec
import Std.Time.DateTime.Timestamp

open Std.Internal.Parsec
open Std.Internal.Parsec.String

def run {α β : Type} [ToString β]
  (filename : String) (part : Nat) (parser : Parser α) (solve : α -> β)
  : IO Unit
:= do
  let t0 <- Std.Time.Timestamp.now
  let inputStr <- IO.FS.readFile filename
  let result := parser inputStr.mkIterator
  let dt <- t0.since
  IO.println s!"parse finished ({dt})"
  match result with
  | ParseResult.error pos err  => do
    IO.println s!"parse error at {pos.pos.byteIdx}: {err}"
  | ParseResult.success _ input => do
    let t0 <- Std.Time.Timestamp.now
    let answer := solve input
    IO.print s!"part {part}: {answer}"
    let dt <- t0.since
    IO.println s!" ({dt})"

def main (args : List String) : IO Unit := do
  let [filename] := args | IO.println "usage: day06 inputs/example.txt"
  run filename 1 parser1 solve1
  run filename 2 parser2 solve2
