import Day04

import Std.Internal.Parsec
import Std.Time.DateTime.Timestamp

open Std.Internal.Parsec

def main (args : List String) : IO Unit := do
  let [filename] := args | IO.println "usage: day03 inputs/example.txt"
  let t0 <- Std.Time.Timestamp.now
  let inputStr <- IO.FS.readFile filename
  let result := parser inputStr.mkIterator
  let dt <- t0.since
  IO.println s!"parse finished ({dt})"
  match result with
  | ParseResult.error pos err  => do
    IO.println s!"parse error at {pos}: {err}"
  | ParseResult.success _ input => do
    let t0 <- Std.Time.Timestamp.now
    let answer1 := solve1 input
    IO.print s!"part 1: {answer1}"
    let dt <- t0.since
    IO.println s!" ({dt})"
    let t0 <- Std.Time.Timestamp.now
    let answer2 := solve2 input
    IO.print s!"part 2: {answer2}"
    let dt <- t0.since
    IO.println s!" ({dt})"
