import Day02

import Std.Internal.Parsec
open Std.Internal.Parsec

def main (args : List String) : IO Unit := do
  let [filename] := args | IO.println "usage: day02 inputs/example.txt"
  let inputStr <- IO.FS.readFile filename
  let result := parser inputStr.mkIterator
  match result with
  | ParseResult.error pos err  => do
    IO.println s!"parse error at {pos}: {err}"
  | ParseResult.success _ input => do
    let answer1 := solve1 input
    IO.println s!"part 1: {answer1}"
    let answer2 := solve2 input
    IO.println s!"part 2: {answer2}"
