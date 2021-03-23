/*
predicate ProductsIsValid(expenses: seq<nat>, products: seq<Option<nat>>, i1: nat, i2: nat)
    requires 0 <= i1 <= i2 < |expenses| || (i1 == 0 && i2 == |expenses|)
{
    forall v :: 0 <= v < |products| ==>
        IsValueSuchThat(
            products[v],
            prd => exists i, j ::
                0 <= i < j <= i2
                && (j < i2 || i < i1)
                && expenses[i] + expenses[j] == v
                && expenses[i] * expenses[j] == prd
        )
}

lemma EmptyProductsIsValid(expenses: seq<nat>, products: seq<Option<nat>>)
    requires forall prd :: prd in products ==> prd.None?
    ensures ProductsIsValid(expenses, products, 0, 0)
{
    // XXX TODO
}

lemma ProductsIsValidStep(expenses: seq<nat>, products: seq<Option<nat>>, i2: nat)
    requires 0 <= i2 < |expenses|
    requires ProductsIsValid(expenses, products, i2, i2)
    ensures ProductsIsValid(expenses, products, 0, i2 + 1)
{
    // XXX TODO
}

method Solve(expenses: array<nat>)
    returns (result: Option<nat>)
    ensures IsSolution(expenses[..], result)
{
    ghost var expenses_seq := expenses[..];
    
    var seen := new bool[2021](i => false);
    var products := new Option<nat>[2021](i => Option.None);
    EmptyProductsIsValid(expenses[..], products[..]);

    var c := 0;
    while c < expenses.Length
        invariant 0 <= c <= expenses.Length
        // seen[v] tells if we've seen v previously
        invariant forall v :: 0 <= v <= 2020 ==> (seen[v] <==> v in expenses[..c])
        // products[x+y] is Some(x*y) for each pair (x, y) we've seen previously
        invariant ProductsIsValid(expenses[..], products[..], 0, c)
    {
        var v2 := expenses[c];
        if 0 <= v2 <= 2020 {
            var i1 := 0;
            while i1 < c
                invariant 0 <= i1 <= c
                invariant ProductsIsValid(expenses[..], products[..], i1, c)
            {
                var v1 := expenses[i1];
                i1 := i1 + 1;
            }

            seen[v2] := true;
        }

        ProductsIsValidStep(expenses[..], products[..], c);
        c := c + 1;
    }

    return None;
}

*/


method ToArray<A>(s: seq<A>) returns (a: array<A>)
    ensures a[..] == s
{
    a := new A[|s|](i requires 0 <= i < |s| => s[i]);
}

method Main() {
    var expenses_data: seq<nat> := [
        1833,
        1949,
        1745,
        1863,
        1422,
        1980,
        1695,
        1233,
        1407,
        1971,
        1486,
        1922,
        1802,
        1689,
        214,
        1864,
        1091,
        1395,
        1772,
        1901,
        1542,
        1730,
        1782,
        1815,
        1818,
        1236,
        1604,
        1219,
        1834,
        1813,
        1532,
        1963,
        2003,
        1149,
        1577,
        1408,
        1871,
        1417,
        1727,
        1155,
        1953,
        1287,
        1259,
        1548,
        1126,
        1927,
        1225,
        1172,
        11,
        1301,
        1869,
        1808,
        1238,
        1989,
        1027,
        321,
        1564,
        636,
        1847,
        1877,
        1716,
        1275,
        1738,
        1054,
        1966,
        1019,
        1256,
        1940,
        1821,
        1914,
        1556,
        1389,
        1020,
        1293,
        1935,
        1804,
        1945,
        508,
        1856,
        1674,
        1721,
        1541,
        1435,
        1852,
        1394,
        2006,
        1366,
        1473,
        1274,
        623,
        1701,
        1842,
        1954,
        1999,
        1195,
        1246,
        1967,
        1153,
        1851,
        1294,
        1152,
        1812,
        1732,
        1030,
        1956,
        1132,
        1948,
        1865,
        1835,
        1231,
        1975,
        1759,
        1843,
        1379,
        1657,
        1267,
        1062,
        1850,
        1817,
        1543,
        1805,
        1252,
        1974,
        1161,
        876,
        1647,
        1796,
        1634,
        1177,
        1519,
        1527,
        1249,
        1158,
        2007,
        1702,
        1714,
        1040,
        1826,
        1837,
        1361,
        1814,
        1858,
        1828,
        1951,
        1140,
        1845,
        1476,
        1337,
        1262,
        1923,
        1397,
        1025,
        1412,
        1930,
        1164,
        1300,
        1369,
        1777,
        1591,
        1919,
        1874,
        1482,
        2010,
        1957,
        1897,
        1854,
        1992,
        1735,
        1786,
        1661,
        1404,
        1254,
        1803,
        1794,
        1614,
        1711,
        1007,
        1979,
        1928,
        1505,
        2001,
        1094,
        2005,
        1297,
        1933,
        1976,
        1104,
        1279,
        1012,
        1943,
        1679,
        1958,
        1841,
        1623,
        1809,
        1800,
        919,
        1620,
        1936,
        1209
    ];
    var expenses: array<nat> := ToArray(expenses_data);
    var result := Solve(expenses);
    match result {
        case Some(v) => { print v; }
        case None => { print "no solutions\n"; }
    }
}
