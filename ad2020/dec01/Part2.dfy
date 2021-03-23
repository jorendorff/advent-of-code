// --- Specification

datatype Option<T> = None | Some(value: T)

predicate IsValueSuchThat<T(!new)>(opt: Option<T>, pred: T -> bool)
{
    if exists t: T :: pred(t)
        then opt.Some? && pred(opt.value)
        else opt.None?
}

predicate IsSolution(a: seq<nat>, result: Option<nat>)
{
    IsValueSuchThat(
        result,
        prd =>
            exists i, j, k ::
                0 <= i < j < k < |a|
                && a[i] + a[j] + a[k] == 2020
                && a[i] * a[j] * a[k] == prd
    )
}

// --- Infrastructure

// REVERSE lexicographical order, sorry :-\
predicate TripleLessThan(a: (nat, nat, nat), b: (nat, nat, nat))
{
    a.2 < b.2
    || (a.2 == b.2
        && (a.1 < b.1
            || (a.1 == b.1
                && a.0 < b.0)))
}

predicate NoSolution(a: seq<nat>)
{
    forall i, j, k ::
        0 <= i < j < k < |a|
        ==> a[i] + a[j] + a[k] != 2020
}

predicate NoSolutionSoFar(a: seq<nat>, current: (nat, nat, nat))
{
    forall i, j, k ::
        0 <= i < j < k < |a|
        && TripleLessThan((i, j, k), current)
        ==> a[i] + a[j] + a[k] != 2020
}

lemma NoSolutionStepJ(a: seq<nat>, j: nat, k: nat)
    requires NoSolutionSoFar(a, (j, j, k))
    ensures NoSolutionSoFar(a, (0, j + 1, k))
{
}

lemma NoSolutionStepI(a: seq<nat>, i: nat, j: nat, k: nat)
    requires 0 <= i < j < k < |a|
    requires NoSolutionSoFar(a, (i, j, k))
    requires a[i] + a[j] + a[k] != 2020
    ensures NoSolutionSoFar(a, (i + 1, j, k))
{
}

lemma NoSolutionFinalStep(a: seq<nat>)
    requires NoSolutionSoFar(a, (0, 0, |a|))
    ensures NoSolution(a)
{
    forall i: nat, j: nat, k: nat
        | 0 <= i < j < k < |a|
        ensures a[i] + a[j] + a[k] != 2020
    {
        assert k < |a|;
        assert TripleLessThan((i, j, k), (0, 0, |a|));
    }
}

method Solve(expenses: array<nat>)
    returns (result: Option<nat>)
    ensures IsSolution(expenses[..], result)
{
    var k := 0;
    while k < expenses.Length
        invariant 0 <= k <= expenses.Length
        invariant NoSolutionSoFar(expenses[..], (0, 0, k))
    {
        var vk := expenses[k];

        var j := 0;
        while j < k
            invariant 0 <= j <= k
            invariant NoSolutionSoFar(expenses[..], (0, j, k))
        {
            var vj := expenses[j];

            var i := 0;
            while i < j
                invariant 0 <= i <= j
                invariant NoSolutionSoFar(expenses[..], (i, j, k))
            {
                var vi := expenses[i];
                if vi + vj + vk == 2020 {
                    return Some(vi * vj * vk);
                }

                NoSolutionStepI(expenses[..], i, j, k);
                i := i + 1;
            }

            NoSolutionStepJ(expenses[..], j, k);
            j := j + 1;
        }

        k := k + 1;
    }

    NoSolutionFinalStep(expenses[..]);
    return None;
}


// --- Wrapper program

method ToArray<A>(s: seq<A>) returns (a: array<A>)
    ensures a[..] == s
{
    a := new A[|s|](i requires 0 <= i < |s| => s[i]);
}

function method PuzzleInput(): seq<nat> {
    [
        1833, 1949, 1745, 1863, 1422, 1980, 1695, 1233,
        1407, 1971, 1486, 1922, 1802, 1689, 214, 1864,
        1091, 1395, 1772, 1901, 1542, 1730, 1782, 1815,
        1818, 1236, 1604, 1219, 1834, 1813, 1532, 1963,
        2003, 1149, 1577, 1408, 1871, 1417, 1727, 1155,
        1953, 1287, 1259, 1548, 1126, 1927, 1225, 1172,
        11, 1301, 1869, 1808, 1238, 1989, 1027, 321,
        1564, 636, 1847, 1877, 1716, 1275, 1738, 1054,

        1966, 1019, 1256, 1940, 1821, 1914, 1556, 1389,
        1020, 1293, 1935, 1804, 1945, 508, 1856, 1674,
        1721, 1541, 1435, 1852, 1394, 2006, 1366, 1473,
        1274, 623, 1701, 1842, 1954, 1999, 1195, 1246,
        1967, 1153, 1851, 1294, 1152, 1812, 1732, 1030,
        1956, 1132, 1948, 1865, 1835, 1231, 1975, 1759,
        1843, 1379, 1657, 1267, 1062, 1850, 1817, 1543,
        1805, 1252, 1974, 1161, 876, 1647, 1796, 1634,

        1177, 1519, 1527, 1249, 1158, 2007, 1702, 1714,
        1040, 1826, 1837, 1361, 1814, 1858, 1828, 1951,
        1140, 1845, 1476, 1337, 1262, 1923, 1397, 1025,
        1412, 1930, 1164, 1300, 1369, 1777, 1591, 1919,
        1874, 1482, 2010, 1957, 1897, 1854, 1992, 1735,
        1786, 1661, 1404, 1254, 1803, 1794, 1614, 1711,
        1007, 1979, 1928, 1505, 2001, 1094, 2005, 1297,
        1933, 1976, 1104, 1279, 1012, 1943, 1679, 1958,

        1841, 1623, 1809, 1800, 919, 1620, 1936, 1209
    ]
}

// ∀x. ∃y. p(x, y)
// ∃f. ∀x. p(x, f(x))

method Main() {
    var expenses: array<nat> := ToArray(PuzzleInput());
    var result := Solve(expenses);
    match result {
        case Some(v) => { print v; }
        case None => { print "no solutions\n"; }
    }
}
