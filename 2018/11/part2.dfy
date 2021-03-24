function MostPoweredSquare(grid: seq<seq<int>>): (nat, nat, nat)
    requires |grid| > 0 && forall i :: 0 <= i < |grid| ==> |grid| == |grid[i]|
{
  max(scored_squares(grid)).2
}