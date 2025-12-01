import Day01
import Std.Internal.Parsec
open Std.Internal.Parsec

def main (args : List String) : IO Unit := do
  let [filename] := args | IO.println "usage: day01 input.txt"
  let inputStr <- IO.FS.readFile filename
  let result := parser inputStr.mkIterator
  let action := match result with
  | ParseResult.error pos err  =>
      IO.println s!"parse error at {pos}: {err}"
  | ParseResult.success _ input =>
      let answer := solve input
      IO.println s!"{answer}"
  action
