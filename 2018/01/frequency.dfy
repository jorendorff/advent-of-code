// *** Specification **********************************************************

function sum(ns: seq<int>): int {
    if ns == [] then 0 else sum(ns[..|ns| - 1]) + ns[|ns| - 1]
}


// *** Implementation *********************************************************

method Sum(ns: array<int>) returns (result: int)
ensures result == sum(ns[..]);
{
    var i := 0;
    result := 0;
    while i < ns.Length
        invariant 0 <= i <= ns.Length && result == sum(ns[..i])
    {
        assert sum(ns[..i + 1]) == sum(ns[..i + 1][..i]) + ns[..i + 1][i];
        assert ns[..i + 1][..i] == ns[..i];
        assert ns[..i + 1][i] == ns[i];
        result := result + ns[i];
        i := i + 1;
    }
    assert ns[..i] == ns[..];
}
