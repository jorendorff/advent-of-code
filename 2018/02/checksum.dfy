// *** Specification **********************************************************

predicate contains_exactly_two_of_anything<T(==)>(s: seq<T>) {
    exists t :: t in s && multiset(s)[t] == 2
}

predicate contains_exactly_three_of_anything<T(==)>(s: seq<T>) {
    exists t :: t in s && multiset(s)[t] == 3
}

function checksum(box_ids: seq<string>): nat {
    var twos   := set i | 0 <= i < |box_ids| && contains_exactly_two_of_anything(box_ids[i]);
    var threes := set i | 0 <= i < |box_ids| && contains_exactly_three_of_anything(box_ids[i]);
    |twos| * |threes|
}


// *** Implementation *********************************************************

method Checksum(box_ids: array<string>) returns (c: nat)
ensures c == checksum(box_ids[..])
{
    var num_twos := 0;
    var num_threes := 0;
    var i := 0;
    while i < box_ids.Length
        invariant 0 <= i <= box_ids.Length;
        invariant num_twos * num_threes == checksum(box_ids[..i]);
    {

        i := i + 1;
    }
    c := num_twos * num_threes;
}