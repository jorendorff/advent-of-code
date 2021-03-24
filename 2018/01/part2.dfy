// *** Specification **********************************************************

// You notice that the device repeats the same frequency change list over and
// over. To calibrate the device, you need to find the first frequency it
// reaches twice.
//
// For example, using the same list of changes above, the device would loop as
// follows:
//
//     Current frequency  0, change of +1; resulting frequency  1.
//     Current frequency  1, change of -2; resulting frequency -1.
//     Current frequency -1, change of +3; resulting frequency  2.
//     Current frequency  2, change of +1; resulting frequency  3.
//     (At this point, the device continues from the start of the list.)
//     Current frequency  3, change of +1; resulting frequency  4.
//     Current frequency  4, change of -2; resulting frequency  2,
//         which has already been seen.
//
// In this example, the first frequency reached twice is 2. Note that your
// device might need to repeat its list of frequency changes many times before
// a duplicate frequency is found, and that duplicates might be found while in
// the middle of processing the list.
//
// Here are other examples:
//
//     +1, -1 first reaches 0 twice.
//     +3, +3, +4, -2, -4 first reaches 10 twice.
//     -6, +3, +8, +5, -6 first reaches 5 twice.
//     +7, +7, -2, -7, -4 first reaches 14 twice.
//
// What is the first frequency your device reaches twice?

function nth_change(change_pattern: seq<int>, n: nat): int
    requires change_pattern != []
{
    // "the device repeats the same frequency change list over and over"
    change_pattern[n % |change_pattern|]
}

// Return the i'th element in the sequence of frequencies.
function frequency(change_pattern: seq<int>, i: nat): int
    requires change_pattern != []
{
    if i == 0
    then 0
    else frequency(change_pattern, i - 1) + nth_change(change_pattern, i)
}

// True if element i in the sequence of frequencies visited is the same as some
// previously-visited frequency.
predicate revisit?(change_pattern: seq<int>, i: nat)
    requires change_pattern != []
{
    exists j :: 0 <= j < i && frequency(change_pattern, j) == frequency(change_pattern, i)
}


datatype Option<T> = Some(T) | None

// The first natural number in the inclusive range [a, b] satisfying the given
// predicate, given that b does satisfy it.
function first(a: nat, b: nat, test: nat->bool): nat
    requires a <= b
    requires test(b)
    decreases b - a
{
    if test(a)
    then a
    else assert a < b;
         first(a + 1, b, test)
}

// The least natural number satisfying the given predicate, if any.
function least(test: nat -> bool): Option<nat> {
    if forall x :: !test(x)
    then None
    else var bound :| test(bound);
         Some(first(0, bound, test))
}

function first_revisited_frequency(change_pattern: seq<int>): Option<int>
    requires change_pattern != []
{
    match least((i: nat) => revisit?(change_pattern, i)) {
        case None => None
        case Some(i) => Some(frequency(change_pattern, i))
    }
}



// *** Implementation *********************************************************

function sum(ns: seq<int>): int {
    if ns == [] then 0 else sum(ns[..|ns| - 1]) + ns[|ns| - 1]
}

method PartialSums(vals: array<int>) returns (sums: array<int>)
    ensures sums.Length == vals.Length + 1
    ensures forall i :: 0 <= i < vals.Length ==> sums[i] == sum(vals[..i])
{
    sums := new int[vals.Length + 1];

    var total := 0;
    var i := 0;
    while i < vals.Length
        invariant 0 <= i <= vals.Length
        invariant total == sum(vals[..i])
        invariant forall x :: 0 <= x < i ==> sums[x] == sum(vals[..x])
    {
        sums[i] := total;
        total := total + vals[i];
        assert vals[..i + 1] == vals[..i] + [vals[i]];
        assert sum(vals[..i + 1]) == sum(vals[..i]) + vals[i];
        i := i + 1;
    }
    sums[i] := total;
}

method NumOfCycles(f1: int, f2: int, d: int) returns (k: Option<int>)
    ensures match k {
        case Some(k) => f1 == f2 + k * d && (d == 0 ==> k == 0)
        case None => !exists k :: f1 == f2 + k * d
    }
{
    if d == 0 {
        if f1 == f2 {
            k := Some(0);
        } else {
            k := None;
        }
    } else {
        if (f2 - f1) % d == 0 {
            k := Some((f2 - f1) / d);
        } else {
            k := None;
        }
    }
}

method Better(a: Option<(nat, int)>, b_pair: (nat, int)) returns (result: Option<(nat, int)>)
{
    match a {
        case None => Some(b_pair),
        case Some(a_pair) =>
            if a_pair.0 < b_pair.0
            then Some(a_pair)
            else Some(b_pair)
    }
}

method FirstRevisitedFrequency(change_pattern: array<int>) returns (freq: Option<int>)
    requires change_pattern.Length > 0
    ensures freq == first_revisited_frequency(change_pattern[..])
{
    var n := change_pattern.Length;
    var ff := PartialSums(change_pattern);
    var d := ff[n];
    assert d == sum(change_pattern[..n]);

    var i := 0;

    var best_pair: Option<(nat, int)> := None;

    while i < n - 1
        invariant 0 <= i < n
    {
        var j := i + 1;
        while j < n
            invariant 0 <= i < j <= n
        {
            var df := ff[j] - ff[i];
            var kopt := NumOfCycles(ff[i], ff[j], d);
            match kopt {
                case None => {}
                case Some(k) => {
                    var pair :=
                        if k >= 0 then (
                            assert ff[i] == ff[j] + k * d;
                            assert i < j + k * n;
                            (j + k * n, ff[i])
                        ) else (
                            assert ff[i] - k * d == ff[j];
                            assert i - k * n > j;
                            (i - k * n, ff[j])
                        );
                    best_pair := Better(best_pair, pair);
                }
            }
            j := j + 1;
        }
        i := i + 1;
    }
    freq := match best_pair {
                case None => None
                case Some(pair) => pair.1
            };
}
