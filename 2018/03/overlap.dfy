// *** Specification (part 1 only) ********************************************

datatype Claim = Claim(id: nat, left: nat, top: nat, width: nat, height: nat)

predicate contains?(c: Claim, p: (nat, nat)) {
    c.left <= p.0 < c.left + c.width &&
    c.top <= p.1 < c.top + c.height
}

function contents(c: Claim): set<(nat, nat)> {
    set x, y |
        c.left <= x < c.left + c.width &&
        c.top <= y < c.top + c.height
        :: (x, y)
}

predicate contested?(claims: seq<Claim>, point: (nat, nat)) {
    exists i, j :: 0 <= i < j < |claims| &&
        contains?(claims[i], point) && contains?(claims[j], point)
}

function contested_area(claims: seq<Claim>): nat {
    var contested_squares :=
        // set p :: contested?(claims, p);
        set c, x, y |
            c in claims &&
            c.left <= x < c.left + c.width &&
            c.top <= y < c.top + c.height &&
            contested?(claims, (x, y))
            :: (x, y);
    |contested_squares|
}

function filter<T>(test: T -> bool, src: seq<T>): seq<T> {
    if src == []
    then []
    else var head? := test(src[0]);
         var tail := filter(test, src[1..]);
         if head?
         then [src[0]] + tail
         else tail
}

function uncontested_claims(claims: seq<Claim>): seq<Claim> {
    // filter((c: Claim) =>
    //            forall p: (nat, nat) :: contains?(c, p) ==> !contested?(claims, p),
    //        claims)
    filter(
        (c: Claim) =>
            forall x :: c.left <= x < c.left + c.width ==>
                forall y :: c.top <= y < c.top + c.height ==>
                    !contested?(claims, (x, y)),
        claims)
}
