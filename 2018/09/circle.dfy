// the circular linked list of marbles needed for part1.dfy

datatype Node = Node(prev: nat, next: nat)

class Circle {
    ghost var marbles: seq<nat>;
    var edges: array<Node>;
    var current_marble: nat;

    predicate valid() reads this, this.edges {
        // the circle can't be empty; nothing would work
        |marbles| > 0 &&
        // `marbles` is rotated so that the current marble is the first
        current_marble == marbles[0] &&
        // the list of marbles doesn't have any duplicates
        (forall m :: m in marbles ==> multiset(marbles)[m] == 1) &&
        // and, for every marble in the list,
        (forall i :: 0 <= i < |marbles| ==> (
            var m := marbles[i];
            // it has a corresponding entry in the table of edges
            0 <= m < edges.Length &&
            // and that entry is correct
            edges[m] == Node(marbles[(i - 1) % |marbles|],
                             marbles[(i + 1) % |marbles|])))
    }

    constructor()
        ensures this.valid()
    {
        this.marbles := [0];
        this.edges := new Node[1];
        this.current_marble := 0;
        new;
        edges[0] := Node(0, 0);
    }

    lemma has_edges(index: nat)
        requires valid() && 0 <= index < |marbles|
        ensures 0 <= marbles[index] < edges.Length
        //ensures edges[marbles[index]] == Node(marbles[(index - 1) % |marbles|],
        //                                      marbles[(index + 1) % |marbles|])
    {
        assert (var m := marbles[index];
                0 <= m < edges.Length &&
                edges[m] == Node(marbles[(index - 1) % |marbles|],
                                 marbles[(index + 1) % |marbles|]));
    }

    method move_forward()
        modifies this
        requires valid()
        ensures valid()
    {
        has_edges(0);
        ghost var old_marbles := marbles;

        marbles := marbles[1..] + marbles[..1];
        this.current_marble := this.edges[this.current_marble].next;

        assert |marbles| == |old_marbles|;
        assert |marbles| > 0;
        assert current_marble == marbles[0];
        assert is_permutation(old_marbles, x => (x - 1) % |old_marbles|, marbles);
        sequence_permutation_preserves_multisets(old_marbles, x => (x - 1) % |old_marbles|, marbles);
        assert multiset(marbles) == multiset(old_marbles);
        forall m | m in marbles ensures multiset(marbles)[m] == 1 {
            assert multiset(marbles)[m] == 1;
        }
        forall i | 0 <= i < |marbles| ensures (
            var m := marbles[i];
            0 <= m < edges.Length &&
            edges[m] == Node(marbles[(i - 1) % |marbles|],
            marbles[(i + 1) % |marbles|]))
        {
        }
    }
    
}

predicate is_permutation<T>(before: seq<T>, f: nat -> nat, after: seq<T>) {
    |before| == |after| &&
        forall i :: 0 <= i < |before| ==> 0 <= f(i) < |after| && before[i] == after[f(i)]
}


lemma sequence_permutation_preserves_multisets<T>(before: seq<T>, f: nat -> nat, after: seq<T>)
    requires is_permutation(before, f, after)
    ensures multiset(before) == multiset(after)

    
