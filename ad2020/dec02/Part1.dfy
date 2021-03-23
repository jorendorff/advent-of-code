
// --- Specification

datatype Entry = Entry(min: nat, max: nat, letter: char, password: string)

function CountIf<T>(pred: T -> bool, elems: seq<T>): nat {
    |(set i | 0 <= i < |elems| && pred(elems[i]))|
}

function CountSpec<T>(value: T, elems: seq<T>): nat {
    CountIf(x => x == value, elems)
}

predicate EntryIsValidSpec(entry: Entry) {
    entry.min <= CountSpec(entry.letter, entry.password) <= entry.max
}

function CountValidEntriesSpec(entries: seq<Entry>): nat {
    |(set i | 0 <= i < |entries| && EntryIsValidSpec(entries[i]))|
}


// --- Parsing
    
predicate method IsLower(c: char) {
    'a' <= c <= 'z'
}

predicate method IsDigit(c: char) {
    '0' <= c <= '9'
}

method DigitValue(c: char)
    returns (value: nat)
    requires IsDigit(c)
{
    return c as nat - '0' as nat;
}

class Parser {
    var s: string;
    var point: nat;
    var ok: bool;

    constructor(input: string)
        ensures Valid()
    {
        s := input;
        point := 0;
        ok := true;
    }

    predicate Valid()
        reads this
    {
        0 <= point <= |s|
    }

    method ReadNat()
        returns (n: nat)
        requires Valid()
        modifies this
        ensures Valid()
    {
        var i := point;
        var value := 0;
        var leadingZero := i < |s| && s[i] == '0';
        while i < |s| && IsDigit(s[i])
            invariant 0 <= i <= |s|
        {
            var digit := DigitValue(s[i]);
            value := value * 10 + digit;
            i := i + 1;
        }

        if i == point || (leadingZero && i != point + 1) {
            ok := false;
        }
        point := i;
        return value;
    }

    method Expect(c: char)
        requires Valid()
        modifies this
        ensures Valid()
    {
        var _ := ReadOne(x => x == c);
    }

    method ReadOne(p: char -> bool)
        returns (result: char)
        requires Valid()
        modifies this
        ensures Valid()
    {
        if point < |s| && p(s[point]) {
            result := s[point];
            point := point + 1;
        } else {
            ok := false;
            result := '\0';
        }
    }

    method ReadOneOrMore(p: char -> bool)
        returns (chars: string)
        requires Valid()
        modifies this
        ensures Valid() && old(point) <= point && chars == s[old(point)..point]
    {
        var i := point;
        while i < |s| && p(s[i])
            invariant point <= i <= |s|
        {
            i := i + 1;
        }
        if i == point {
            ok := false;
        }
        chars := s[point..i];
        point := i;
    }

    method ReadAll()
        returns (result: string)
        requires Valid()
        modifies this
        ensures Valid()
    {
        result := s[point..];
        point := |s|;
    }

    method End()
        requires Valid()
        modifies this
        ensures Valid()
    {
        if point != |s| {
            ok := false;
        }
    }
}

method Parse(line: string)
    returns (entry: Entry)
    //ensures line == Str(entry.min) + "-" + Str(entry.max) + " " + [letter] + ": " + password
{
    var parser := new Parser(line);
    var min := parser.ReadNat();
    parser.Expect('-');
    var max := parser.ReadNat();
    parser.Expect(' ');
    var c := parser.ReadOne(IsLower);
    parser.Expect(':');
    parser.Expect(' ');
    var password := parser.ReadOneOrMore(IsLower);
    parser.End();

    if !parser.ok {
        print "WARNING - unable to parse input line: " + line;
    }
    
    return Entry(min, max, c, password);
}


// --- Lemmas

lemma CountIfStep<T>(p: T -> bool, s: seq<T>, i: nat)
    requires 0 <= i < |s|
    ensures CountIf(p, s[..i + 1]) == CountIf(p, s[..i]) + if p(s[i]) then 1 else 0
{
    // XXX TODO
    var old_indices := set j | 0 <= j < i && p(s[j]);
    calc {
            CountIf(p, s[..i]);
        ==
            |(set j | 0 <= j < |s[..i]| && p(s[..i][j]))|;
        ==
            |(set j | 0 <= j < i && p(s[..i][j]))|;
        ==
            { assert forall j :: 0 <= j < i ==> s[..i][j] == s[j];
              assert forall j :: 0 <= j < i ==> p(s[..i][j]) == p(s[j]);
              assert (set j | 0 <= j < i && p(s[..i][j])) == (set j | 0 <= j < i && p(s[j])); }
            |(set j | 0 <= j < i && p(s[j]))|;
        ==
            |old_indices|;
    }
    var indices := set j | 0 <= j <= i && p(s[j]);
    calc {
            CountIf(p, s[..i + 1]);
        ==
            |(set j | 0 <= j < |s[..i + 1]| && p(s[..i + 1][j]))|;
        ==
            calc {
                    set j | 0 <= j < |s[..i + 1]| && p(s[..i + 1][j]);
                ==
                    set j | 0 <= j < i + 1 && p(s[..i + 1][j]);
                ==
                    set j | 0 <= j <= i && p(s[..i + 1][j]);
                ==
                    set j | 0 <= j <= i && p(s[j]);
                ==
                    indices;
            }
            |indices|;
    }
    if p(s[i]) {
        calc {
                CountIf(p, s[..i + 1]);
            ==
                |indices|;
            ==
                { assert indices == old_indices + {i}; }
                |old_indices + {i}|;
            ==
                |old_indices| + |{i}|;
            ==
                |old_indices| + 1;
            ==
                CountIf(p, s[..i]) + 1;
        }
    } else {
        assert indices == old_indices;
    }
}

lemma CountStep<T>(value: T, s: seq<T>, i: nat)
    requires 0 <= i < |s|
    ensures CountSpec(value, s[..i + 1]) == CountSpec(value, s[..i]) + if s[i] == value then 1 else 0
{
    CountIfStep(x => x == value, s, i);
}


// --- Implementation

method Count(c: char, s: string)
    returns (count: nat)
    ensures count == CountSpec(c, s)
{
    var i := 0;
    count := 0;
    while i < |s|
        invariant 0 <= i <= |s|
        invariant count == CountSpec(c, s[..i])
    {
        if s[i] == c {
            count := count + 1;
        }
        CountStep(c, s, i);
        i := i + 1;
    }
    assert s[..i] == s;
}

method EntryIsValid(entry: Entry)
    returns (result: bool)
    ensures result == EntryIsValidSpec(entry)
{
    var n := Count(entry.letter, entry.password);
    return entry.min <= n <= entry.max;
}

method CountValidEntries(entries: array<Entry>)
    returns (count: nat)
    ensures count == CountValidEntriesSpec(entries[..])
{
    ghost var entries_seq := entries[..];

    var i := 0;
    count := 0;
    while i < entries.Length
        invariant 0 <= i <= entries.Length
        invariant count == CountValidEntriesSpec(entries[..i])
    {
        var isValid := EntryIsValid(entries[i]);
        if isValid {
            count := count + 1;
        }
        CountIfStep(x => EntryIsValidSpec(x), entries_seq, i);
        i := i + 1;
    }
    assert entries[..i] == entries[..];
}


function method PuzzleInput(): seq<string> {
    [
        "2-4 p: vpkpp",
        "6-16 b: bbbbbbbbbbbbbbbpb",
        "6-7 z: zzfzzdz",
        "4-6 q: tfzqvqcpcmqqjqzd",
        "7-8 k: rkkkknkw",
        "5-14 t: ttttnttttttdttttttt",
        "2-10 b: bfbbbbcbnpbbbbt",
        "3-4 h: hrht",
        "2-6 c: ccccccc",
        "5-7 g: pmtgqgg",
        "16-18 h: vhhhhhhhhhhhhphhrnh",
        "8-10 k: kklxkkkqkkkkk",
        "2-5 b: bcbdbbr",
        "6-8 l: lllnllxb",
        "2-13 t: kvdsdnbclhxntktxdwq",
        "3-4 z: zjwz",
        "12-16 s: ssssssssssswssss",
        "1-5 h: mckhhhs",
        "11-18 s: ksssssssgssssssssk",
        "6-7 f: hcmxbfx",
        "4-11 r: grrrcnkjfdr",
        "6-10 t: tdttttrtbjl",
        "8-10 h: vphrhhmhhz",
        "7-8 t: tghttttt",
        "2-11 h: hhhhhhhhhhhhhh",
        "5-6 g: gggggq",
        "4-6 p: dgpmnqv",
        "5-9 v: nmfnvhtrlzhbvd",
        "1-8 z: zwzqzzzz",
        "10-12 l: hllllllhlklldzlmk",
        "10-11 n: brnjfbnnnmnnnnw",
        "1-3 d: jmscsdw",
        "13-20 d: xdqcsdqdpkppddbdtdgg",
        "1-3 w: wfppvkswrwmnq",
        "2-10 p: xbppppmppppppc",
        "1-4 r: rrrr",
        "4-5 q: qqqgq",
        "7-8 f: fchfwjcfpvffdfh",
        "2-5 d: dtddddddgdddx",
        "11-15 t: zcwngpdjtzcpfvt",
        "11-12 c: ccccccccccqz",
        "8-10 v: vvkbszvvwr",
        "3-5 t: psttst",
        "1-7 f: fffsfffffffffrftfff",
        "3-4 z: zqzz",
        "12-13 l: mlklllhkdmllndplj",
        "11-17 x: jttxgmtmpxxxxzfpf",
        "2-7 d: ddqggvrdmrgc",
        "5-15 l: ckjvllsnmczlnsh",
        "1-3 l: nlllllllllll",
        "4-7 r: ctlrfrb",
        "4-12 k: sknbxdzmnckkk",
        "2-16 m: nmmgwpkncdpfglcpzj",
        "4-5 n: nmnnkjn",
        "3-6 h: hhhkhk",
        "5-15 n: ncmnngnngbnnndjx",
        "1-6 r: rrrklrh",
        "7-16 j: jzjjjjdkjjjjjjjjjj",
        "1-2 l: lcxdxl",
        "1-11 t: gttttttttttt",
        "6-10 w: cvtsdlwggwbgn",
        "3-4 q: qpzhj",
        "5-6 b: sbjcbx",
        "4-5 h: dxjhhhpj",
        "4-5 z: zbckhf",
        "8-13 r: xhscjbqthpfkffjh",
        "5-7 j: jjjxjcdtj",
        "13-15 b: bbbbbblbbbwbtbbbbb",
        "16-19 x: xxxgxxxxcwxxxxxksxx",
        "2-5 s: sssss",
        "11-12 s: fwgcsmxfszgs",
        "1-3 m: mmbm",
        "5-10 p: ppgfpzczphpp",
        "11-13 r: rrrrrrrtgrrrc",
        "4-6 g: nggdwbhktgjhsnrwmg",
        "6-9 b: pnrtwgbwwdhmrbp",
        "5-6 w: wwwqqww",
        "10-14 s: ssssssssstsssss",
        "1-9 k: hkkkkkkkkk",
        "2-3 n: nnktnnnc",
        "4-7 m: prmrmmmsj",
        "13-15 t: tfqttttttkttqttttttt",
        "6-8 p: ppppptpplprh",
        "12-14 l: lllllllllllllll",
        "11-12 g: ggggqppgkpzc",
        "2-3 n: hpnnvn",
        "1-5 f: gffffdfffmzf",
        "9-18 w: lwjnfkwrjqtqnvjkhcw",
        "6-7 x: mmxxxxn",
        "12-14 j: hhpwdljfljpvxl",
        "3-5 m: zxlrmrqknmv",
        "6-9 l: slbllllllxlld",
        "11-16 g: gggggggggglggggcg",
        "1-6 h: hwhwlg",
        "18-19 k: kkxvgrkkzkmkkkkkktk",
        "3-10 c: cchccccccfccc",
        "9-10 m: nssbtshpmmn",
        "6-7 m: mtvbmpfpmm",
        "4-13 t: wkmdgpwpcznlqsqtcjf",
        "1-3 m: mmvm",
        "2-11 p: ljvgtkpdlmpznlphxfgj",
        "5-9 p: kppppcppnqpppqpzpppw",
        "8-9 w: wwwwwwwwfw",
        "4-7 m: bvmmlsqsh",
        "2-5 v: vvrvv",
        "4-10 j: jjfwjvxjjjwjj",
        "5-6 f: ffffjf",
        "1-3 w: pwwzd",
        "4-6 v: vvvrvzvvv",
        "11-15 s: ssssspsssscssssd",
        "3-7 c: wrxltgcvpmc",
        "2-3 p: pplp",
        "3-5 f: nfnxfg",
        "8-9 r: rrrsrrrhrr",
        "3-7 j: dpmcvjj",
        "10-16 b: bbbbbbbbbbbbbbrnbb",
        "13-15 q: qqqqqxqqqqqqqqhqq",
        "6-7 v: wvvvvzvv",
        "2-7 j: rbtzvjhnvfznhfbskcp",
        "7-8 n: nnnnncbnn",
        "14-15 j: jjjjjjjgjjjjjjbjj",
        "8-12 g: gggdgggghgqggggjgg",
        "9-10 b: rlcbfbbbbbb",
        "11-12 k: kkkkkkkkkktkk",
        "11-12 k: kmkkkkkkkkqk",
        "11-14 c: ccccfccccccccncccq",
        "13-16 k: pkkkkkjkkkbkrkkkkzk",
        "7-17 d: dhdvddzjddrwdchdd",
        "1-8 z: zqlzzzzzz",
        "3-9 p: ppfppsppwppbp",
        "7-9 r: rrnrrrbrrrrrhrrr",
        "4-7 t: tfqxtgl",
        "4-5 l: ljmglll",
        "5-6 d: tdvffd",
        "4-13 r: rwrqbdbtbrtmrmlrgrr",
        "15-19 g: gggggggggggggggggggg",
        "7-8 r: lrkrfxtrrj",
        "5-6 z: zkdlwzwv",
        "9-10 r: rzrrrrrrfr",
        "3-5 r: rrbcgrbrqrrd",
        "15-16 t: cqnkntxktjwtttctt",
        "9-12 p: ppppppppdpppp",
        "8-10 d: tgddddgdck",
        "6-9 n: nnnnncnxgpnnnnnn",
        "1-4 w: wpwxtjwlwt",
        "3-11 d: dclddmdkqdmf",
        "5-7 w: zgwkpnwkqctwxj",
        "4-6 x: phmxxhvlf",
        "1-2 g: gjrvfg",
        "11-13 t: ttbtptlzltttvttqttj",
        "2-6 p: nmqsppqcqxntchq",
        "10-13 z: dszzzzzgzzplf",
        "5-7 j: kjwbjjqjjgd",
        "12-16 r: rrrrrrjrrrrbrrrlrr",
        "4-6 p: dppppx",
        "11-20 w: wkswvxjwwzhxwwwqgqwn",
        "8-9 f: qffwwmfwjf",
        "3-4 g: pbgdtg",
        "3-8 q: dqqqqqql",
        "16-17 d: djddpdddddddddddsd",
        "4-7 q: qqqqqqmq",
        "13-14 r: rrlrsrrrrrrrrsrr",
        "10-12 k: kkkkkkkkkhkkk",
        "1-3 z: hdqwpdtmzgnpnffrh",
        "2-4 g: gtgrg",
        "5-7 l: lllltlllq",
        "6-8 s: sqcvnsgs",
        "1-3 q: qdqbqq",
        "1-4 j: jknh",
        "4-5 z: zzxlz",
        "2-5 c: ccccjc",
        "7-8 l: lllqfltlwll",
        "8-9 p: pplpppppt",
        "4-19 j: vkmrhblhpvfjlbwjlxjm",
        "4-5 l: rlllnl",
        "3-13 j: hgqcvcnjwnrnjp",
        "3-5 c: cpcbbjp",
        "4-5 z: jtpjzdrwcbrzhvmzz",
        "4-8 s: bqsbvtrs",
        "13-14 d: ddjddddddddpqddd",
        "5-6 l: lllllll",
        "7-9 z: zzzzzzzzpz",
        "3-4 f: rpsgm",
        "9-13 l: dllldlllllllb",
        "16-17 h: thvclfldkhxzcqwvhp",
        "8-9 z: zzzzzzzzz",
        "9-10 d: dtddddddtdk",
        "3-6 k: sxbvkk",
        "4-14 x: xffvxhtxxnmjcxm",
        "6-11 h: kthhhhfqhwf",
        "3-5 z: pzzzq",
        "3-4 x: xxxwxx",
        "2-4 w: sqdwjg",
        "5-6 t: mmgjhkqxts",
        "7-9 w: wwwwwwwwkww",
        "13-15 c: cccccccccccjccj",
        "2-4 j: qjmhjpzbwfj",
        "9-10 h: hhhjhhhhhzpwh",
        "3-6 x: xxtxxlgx",
        "11-14 q: qqqqqqqqqqqqqsq",
        "2-6 n: kqnxcnnx",
        "9-10 m: mmmmmpmmbm",
        "9-12 p: pphpspzpppdp",
        "4-5 j: jmndj",
        "11-13 w: wgdmhwgcwlwqbwpxwkw",
        "5-12 b: bdzqgjbfbbbbbqwb",
        "9-10 v: kvqpggvvcvpv",
        "9-12 s: lsbxnblrxfggt",
        "9-12 r: rrrrrrrrrrrqrrr",
        "9-10 s: nshmnkcdsz",
        "1-2 z: kzzz",
        "4-6 v: vsslvv",
        "8-9 j: jjjjjjjjk",
        "5-6 n: nnvnnn",
        "3-5 k: nnjsljhcwtckqjs",
        "2-5 b: cbhjxd",
        "4-10 c: ccxzqktkqjsggfcckccc",
        "13-16 f: ffffffffffffzffvf",
        "1-4 l: slllll",
        "7-9 v: vxvvvdrvnvwwklvv",
        "3-9 r: rrtrrrrrrr",
        "2-3 n: bnnsnf",
        "9-12 b: txbxbvbtkbbbbbbbgq",
        "6-17 c: ccccrcccccgcccccccc",
        "3-7 d: qcddhpm",
        "15-18 t: ttttttttttttttztttt",
        "2-5 v: mvxcfbgzzfgrqts",
        "3-4 q: vqwqqqmgk",
        "3-8 k: kknkkkkkkk",
        "4-6 z: skclpzmrlgzzzvzsl",
        "3-4 h: hhsh",
        "8-11 z: zzzzzzzzzzzznxzzzz",
        "1-5 p: ppppc",
        "2-6 f: nffwwj",
        "5-7 g: gwtwfqmdfcgtth",
        "1-17 q: wqfvwgcwcpwtgvtwf",
        "6-16 c: ccccccccccccccccxccc",
        "8-10 p: ppppprpppc",
        "4-6 k: kqkkkkkkkkkk",
        "3-10 q: cvqgjxqcrj",
        "3-6 l: hcbgpfjhscfbrsfkzk",
        "2-5 s: mskwssbdstsbssr",
        "2-8 m: fzmfccddffc",
        "6-8 v: vfmvltvv",
        "3-4 w: wwwk",
        "5-6 x: kxwxlxjxx",
        "14-16 f: ffffffhffffffffk",
        "7-8 q: lmqqlnqqpv",
        "6-8 s: mzssssds",
        "7-8 l: fgwtpwrltlvs",
        "3-4 n: nmncnnkhnclmhdkbsv",
        "8-9 f: fffffffftf",
        "2-11 x: xcxfxxpxxdj",
        "10-16 f: fffffffffffffffh",
        "2-6 m: mjmmmm",
        "5-7 g: gfgshggrhg",
        "9-13 q: qqqqqqqqqqqqjq",
        "11-14 x: xxxxxxxxxxxxxgxx",
        "5-8 m: mmmmkkmmfx",
        "3-4 n: nwdnnnn",
        "1-7 x: tksssgxnlvkphdxzcsx",
        "1-3 r: rrmn",
        "2-5 m: mnmmmxmmmm",
        "3-4 j: rcjcrjf",
        "6-14 z: wnchnnzkptzfwb",
        "5-16 j: jjljcjkjjmzjjjnxfjj",
        "8-14 k: kkkkjkknkkkkkk",
        "5-8 d: lcfpxdpdsrxhcgb",
        "1-3 d: ddscmtdh",
        "6-7 d: lxdddkd",
        "1-3 s: wssss",
        "4-8 x: xgxjflxzdxhxxcx",
        "3-6 w: nxwgww",
        "12-16 l: ljjglfblllllpblk",
        "9-10 n: nxnnnnnnnmn",
        "3-4 m: mmlgfmc",
        "5-13 k: lmvxkxkbqkbxmmsxkqfc",
        "10-11 c: cccccccccch",
        "13-16 v: vvvvvvvvvvvvvvvvvv",
        "1-2 h: hkhdbhjcf",
        "16-17 l: llllllllllllllllcl",
        "1-2 c: zcccccp",
        "2-10 x: jsgxvxxdrxwtsftx",
        "17-18 r: rrrrrrrrrxrrrrrrrh",
        "4-5 w: wwwpwwwwwwwww",
        "1-3 f: fzffff",
        "3-5 q: rqsqq",
        "4-7 b: blkxxbbrkkbjzqqd",
        "5-6 s: slsssssss",
        "13-15 g: xgggggggggggcrxg",
        "3-4 d: ddsddddddddddddddddd",
        "3-8 r: cklssrprdmgggk",
        "5-11 s: ndxsssddflsbsptdzfmh",
        "5-20 g: sgpdgmnsgxgghmlmgqgz",
        "13-14 p: kpppmppppzbppgpp",
        "10-11 f: ffffffffffvff",
        "4-8 g: gpgpggcnjggqg",
        "8-10 n: npxntcdndpnrq",
        "9-13 q: qqqqqqqqqqqqtq",
        "5-8 n: nnnnwnpnnv",
        "6-9 w: wwwwwwwkcwwwwwwww",
        "6-7 g: gxgghmg",
        "7-8 z: spgkbchz",
        "6-14 v: jdpxvvdvtvpsqm",
        "2-7 d: vjbktzd",
        "2-3 s: ssxv",
        "2-4 j: jtch",
        "3-5 r: hghhr",
        "17-19 k: kkkkkkkkkknkkkkkrkq",
        "4-13 v: vvvcvvvvvvvqvwvv",
        "5-8 k: dkpkvkkckkwbk",
        "8-15 h: qdqhqglpgbsjgnhc",
        "8-11 d: fdpddxmcdkdh",
        "3-5 c: ccwcccccpc",
        "10-11 z: wjnzrzkzwzfgrzzl",
        "12-14 m: mmmwmmmmmmmmmmmm",
        "2-4 q: kqcvqxxtdbtjrjmrgr",
        "12-14 f: wjsfpfhxbfvffh",
        "14-17 d: dddddddddddddddddddd",
        "1-5 s: dsssssdssswqns",
        "6-8 h: hnlbffhh",
        "4-5 k: bhssktq",
        "8-19 p: pppppppfpppppppppppp",
        "18-19 q: qsxllxqlzkqqmkqshqd",
        "3-6 p: qbpkpm",
        "1-3 z: zxnlclfzbp",
        "18-19 f: ffxmfdxfdfffffhrfjf",
        "2-5 w: wvpwwwf",
        "13-17 l: llplltllllllmlllp",
        "6-8 t: jzwtpjkcktrpqp",
        "3-4 f: wfhffdnfffffvfsfffz",
        "5-7 c: csctcxrcq",
        "5-8 f: fffftffff",
        "10-12 h: hjhmhwhzckhg",
        "7-9 r: rrrrrrgrrr",
        "11-13 z: zzzkzgfzzrzzrzz",
        "14-18 q: hpsqffzbhqqldqrtcz",
        "8-12 t: jtjttjtzvttvttptttlt",
        "1-3 w: wwrwwwwwpwvwwcnf",
        "2-6 d: clmpddfddhdd",
        "6-8 l: lllllpfll",
        "5-6 z: fhrdnz",
        "3-7 n: gxjntsp",
        "4-8 t: nddttttgnvt",
        "2-4 l: lbxlhlllllllllllfl",
        "2-5 l: llkdlbx",
        "3-6 q: wqbbhnmcwplxlm",
        "4-9 c: wkcccjcqrlclcgcccrc",
        "3-4 h: thjh",
        "2-6 s: hsbnjgjqj",
        "7-10 s: sbshrhsvhnqtb",
        "10-15 j: jjjjjjjjjcjjjjjjj",
        "9-12 k: kkkkkkqkkkknkkkkkkkk",
        "5-7 p: zrzpplx",
        "7-12 v: vvvvvvvvvvvjvv",
        "14-15 s: sssvssspssssswss",
        "11-13 z: nzzzzzzzzfzzrzz",
        "6-8 b: pwvlqbhlswggnhbwthjl",
        "8-12 t: tcxpmxcbswgt",
        "4-5 m: mmmxf",
        "11-12 f: ffpffffwfwfk",
        "3-8 r: rrrrrgrdxrr",
        "6-7 f: fsfbffjfb",
        "8-9 h: hhhhhhhhhhhhh",
        "11-12 w: lwwwttwwkvpwwwwwc",
        "1-14 s: pshssssssssssrsjs",
        "4-6 x: xxxxxxxx",
        "2-4 l: lldt",
        "9-10 x: xxdxxxxxczxbxd",
        "1-7 t: tgtttdst",
        "8-9 z: zzzzqzzzd",
        "11-12 g: ggggggggggdxg",
        "10-13 p: ppppgppppppprppgp",
        "2-16 g: qggkwxfvpcffplwg",
        "17-18 n: nnnnnnnnnnnnnnnnnz",
        "3-7 q: qqqqqbdqqq",
        "4-8 h: hjmhrmhbvhj",
        "12-15 d: bdzddppfddhpzjd",
        "5-7 z: zzzzzzzzzz",
        "3-4 v: vvvf",
        "3-5 g: dcnwgqrvggfqbllvfgk",
        "5-9 b: lsmsrwlvb",
        "7-8 w: wwwwwwwsw",
        "12-13 n: hrbpwjqrkhtxnnqqn",
        "5-6 m: mvxqmmd",
        "3-20 v: hmhjxjffzczvbwqfnngv",
        "15-16 d: ddddddddddddhddd",
        "4-7 x: jxxbwnxpbqrkx",
        "5-7 z: zrzzzzmzt",
        "16-20 c: hcdccfkcjcsxrcnccbpv",
        "8-9 j: jjjqjjjjk",
        "4-12 c: kszcxlswkcbvmxjsbdt",
        "1-5 r: rrhnr",
        "12-13 x: xxxxxbxxxxxxx",
        "2-4 r: rrnl",
        "1-8 m: rmmmmmvmmmmmm",
        "10-13 k: kkkkkkkkkkkkbk",
        "5-15 t: qtrtttxvtlxhtlpttwt",
        "9-10 k: kkkkkkkkzh",
        "17-18 q: qqqqqbqqqqqqqqqqwqq",
        "1-2 k: jkftk",
        "5-7 t: ltttktvtwtt",
        "4-6 b: rbbtwb",
        "1-5 w: gwwwwwmwww",
        "6-8 x: txqxxtxtx",
        "9-14 t: ttnttttnttttttw",
        "3-6 s: spszzskl",
        "4-9 x: xxxrxvxxkxx",
        "11-14 r: rrrfhrbrjkrrgh",
        "1-9 q: wqqqqqqqqqq",
        "5-14 f: gfdjfdmfmmcgfrffsp",
        "2-4 b: bvcb",
        "9-12 c: ccccnccrcccccccc",
        "9-14 s: ssssssssbgssssss",
        "15-18 d: ddddddcdkdsddddddw",
        "17-18 v: vvvvvvvvvvvvzvvvvbvv",
        "3-7 k: mfkbzqnwhkgkk",
        "2-5 p: cpqpp",
        "7-10 r: frrrrrrrrnrrrrrqrw",
        "5-6 h: fhhhxschhdpbh",
        "1-2 k: kcrkkk",
        "1-5 s: kssssssszsnssssssr",
        "3-4 t: ztct",
        "1-2 g: grkgc",
        "8-9 t: tttctttcttqc",
        "4-9 d: rddvdddddddddd",
        "5-7 j: jjjjjjnj",
        "4-5 f: ffrjd",
        "14-16 x: jqvkmxlxfxbwplhxl",
        "2-13 j: hkqphvkprvmjdfm",
        "1-12 b: bbbbbbbbvbbjvbbbbbs",
        "4-6 m: mmmnmm",
        "16-18 q: rvfzkvqqmddvqfrrpq",
        "7-14 c: czcccccccdpccgsc",
        "1-8 z: zgzfbtzzqp",
        "7-18 n: tzfktnkcpncxcsvxzv",
        "2-3 x: xmxz",
        "4-10 q: qqldqqqqqqql",
        "2-6 f: ffvfpqfftg",
        "12-15 c: cccccccccscxlccc",
        "3-17 d: ddlzddghdddgwdddddd",
        "11-13 g: lglgmqgcmlggbftgggd",
        "12-13 q: ldjqqvqqhbfqn",
        "4-5 h: hhhhp",
        "9-11 f: tfkcfzcqpzfdggbpw",
        "10-13 f: fffffffffkffffff",
        "2-3 l: swln",
        "6-10 j: kqsjnjtjmd",
        "8-12 c: ccccctclccscmcc",
        "10-18 r: rrrrrrrrrjrrjrrrrr",
        "4-7 t: btcxktc",
        "8-19 m: mmmmmmmmmmmmmmmmmmhm",
        "10-12 j: jjjjjjrjjjlj",
        "1-11 x: xlxxxxxxxxxxz",
        "11-17 w: fblmwxqwbfdwlcqww",
        "4-6 c: qchcclc",
        "3-4 k: kkkkkkkbkkr",
        "4-5 s: sssfssssssssw",
        "4-6 m: bvmmvjrlvmzmmtsm",
        "6-14 f: qftmhffffcccffsz",
        "3-4 t: rzthtt",
        "6-19 s: qshpkhjcsssqmzspxss",
        "3-4 p: vshp",
        "3-4 w: zwff",
        "5-7 s: tngctss",
        "11-14 g: bhshgkpgxrgkqwpprwv",
        "3-7 t: ttptttt",
        "1-3 q: qqzqqqvb",
        "7-12 r: kvzjrmrhvxxs",
        "11-12 j: jjjkjjjjjjjz",
        "10-11 p: pppppppppxppp",
        "1-10 s: rsssssssshsss",
        "4-5 x: xxxdcx",
        "3-4 n: fnlnfn",
        "8-9 l: llfflqlfll",
        "3-4 z: hzrz",
        "15-16 f: tdzxfwjvdgsxczff",
        "12-16 z: zzzzzzzvpzzzzzzzzz",
        "4-8 f: fwfwffffqmfkff",
        "7-8 z: zjvzzkzzzxmzz",
        "7-8 r: drrcsrrg",
        "7-8 z: znzzzzbzzz",
        "7-8 l: llllllml",
        "10-11 b: jbbbbbjbbbs",
        "5-12 c: cccccccccccpc",
        "9-13 n: nnnnnnnnpnnnn",
        "7-8 q: rvqqqqhnrqqqjqq",
        "4-8 d: xzqwgncdgqtd",
        "6-7 r: rrrrrmr",
        "1-2 q: rbql",
        "13-14 f: ffkfjffjzqfcnfhshw",
        "6-9 m: mmmhmmkmbmrlrwwmtfl",
        "5-12 s: ssfsmsbwsrzssstsssss",
        "6-14 m: bmpbqmjmqxmnrt",
        "5-7 g: hgwgdntgwgsp",
        "10-15 q: qzqqrbqmqqqqsntqqq",
        "7-10 w: wwwwwwnwwwhz",
        "5-7 v: vvvvrvv",
        "2-3 z: zzzlz",
        "5-9 r: rxqbhdrmr",
        "10-15 c: bchccbcqqcrtcrj",
        "8-10 g: hplggnlnbfpgfxmkgb",
        "14-18 p: pwpqppdpfpppppqpph",
        "8-9 t: ttlgpzmtdkzdrcstztf",
        "13-19 l: slzsflllllwlllllllg",
        "5-6 z: zqcnzz",
        "11-14 c: ccctcccccwckch",
        "3-5 v: vhblvzmvfv",
        "18-19 v: vvvvvvvvbvvvvvvvvmv",
        "4-5 g: gjjwgtlgjtggg",
        "4-5 l: rlnvlvxllg",
        "2-11 z: qzclmjsrfrsbrjtd",
        "9-11 w: gwwwwwwwwsbfrbw",
        "1-2 v: vzvvvvvvvvvvvvvvvvv",
        "4-8 b: bzbpbpbjdfbdbqfz",
        "10-16 m: wxcvdmldmzxvnxmmqb",
        "4-11 c: cljrsmxwvbcx",
        "12-17 n: lwzncwfjpwxbnnhnnpx",
        "9-10 k: vfbbmbxwkd",
        "17-19 c: ccccccccgcccccccccd",
        "5-9 h: vhrhmhvsh",
        "6-10 g: ggfggsggngwgpcgfvz",
        "4-17 g: ccvgdkxzggzbsjvzqq",
        "8-10 g: qggggggzggg",
        "3-4 w: wwvh",
        "4-5 r: gjrfrb",
        "1-5 r: qrrrrrrr",
        "11-15 k: kckkkkpjqmnkcwkkkkq",
        "3-6 q: qwswqq",
        "4-5 s: lsssss",
        "12-13 d: mdwgdddtdfdvr",
        "1-14 k: vkkkkkkkdkkkkkkk",
        "1-6 t: zrgnxtt",
        "8-9 d: kdqdfgndd",
        "6-7 z: pfnzzzzzwjzxg",
        "2-9 d: ddddddddddddkddd",
        "5-7 x: krnfpjx",
        "4-12 j: ksgkjzbqprvjqjxbp",
        "10-11 m: hmzmmmnmmmpmmg",
        "4-8 k: fxmkmppqnlckglvm",
        "8-11 x: xxxxxxtrxxxdxxx",
        "1-10 r: rwcrfrlhrsrhr",
        "5-7 m: mmmmvmm",
        "7-16 m: mmmmmjmhmmqmwmmqvfm",
        "8-13 v: mdmrvvsvvmhcxknc",
        "5-6 w: bwwxnw",
        "11-12 g: ggggggggggwgggg",
        "9-16 t: tgtttgtnttttdtttw",
        "4-7 j: jjjjjvvnjj",
        "12-17 p: vhbprgsmljmpzzhzn",
        "4-13 v: vrcntbdvkvtdvzxnptvb",
        "1-7 j: jjjjjjdjj",
        "11-14 c: cjhcrkhcccccdc",
        "17-18 j: jhjjjjjjjjjqjjjjjvj",
        "8-9 m: jmmkmmmmdmwvmgz",
        "9-12 x: dkxxxhqxxzzpcvcxgkx",
        "16-17 m: mmmmmqmmmmmtmmmfmjm",
        "3-13 b: mbbtrmxlzdpbt",
        "1-5 g: bggggggg",
        "11-19 m: dlgmdsscksdrtmmdpjrq",
        "1-4 g: skgg",
        "12-20 l: jcgnljnllllqftjrvnhl",
        "2-3 d: drbdcl",
        "2-3 t: mtwtt",
        "8-9 f: fxstfnfsfffft",
        "14-16 z: zzzzzzzzzzzzzzzrz",
        "5-8 x: xxxxpxxxxxxt",
        "10-11 t: ttttttttttv",
        "5-16 r: xrwrrrqhhlqkhdlzr",
        "5-7 n: nnnnnnhnnnnnnnnnnnnn",
        "4-6 v: dzwfvxz",
        "2-6 c: cccccccc",
        "5-6 j: jjjpjkjhj",
        "3-11 j: jjhnjjgjjghjjj",
        "14-20 s: zskwnsmpsswctgwshxsm",
        "1-3 j: tjfjj",
        "2-8 d: dsdrssdwqq",
        "9-10 j: rjjjjjjjqjjqjjj",
        "8-10 b: clmwbzzjkb",
        "4-9 k: klkdkkkkvkkkkkk",
        "8-16 v: xvvsvvqqvvjvvvvv",
        "4-5 d: qdrtd",
        "3-4 p: pppt",
        "4-5 p: ppvmp",
        "5-13 g: sggmnggwtggmqggkg",
        "9-17 g: ggcgggntdgjmgtgxg",
        "12-13 q: jqqqqqqqqzqqqqq",
        "2-3 v: tbvzcmzvn",
        "1-5 m: vnmwtww",
        "5-6 r: rrhkrs",
        "14-15 c: cccchccccccccct",
        "15-17 m: mmmmmmzmmmmmmmhmm",
        "5-10 k: kqkkkkkkkkkkkkkkkkk",
        "17-19 z: gxpjhhktxrlwwgqzlxzx",
        "3-7 s: wsbssvscgss",
        "3-5 k: kkkkf",
        "4-7 m: jmmmgmbmmmv",
        "2-7 p: vpchdqpxxwjpwdgr",
        "3-6 w: wwwxwwdww",
        "11-17 h: hhhhhhhhhhhhhhhhb",
        "1-5 w: zwwwwwwwwwwwwwgw",
        "2-5 m: mlmmmmk",
        "10-11 p: nptppbpplpd",
        "2-8 m: xcmsbqms",
        "7-13 w: wjvwrwwwlxnsnw",
        "3-9 r: rrrrrrrrv",
        "2-4 t: nttw",
        "17-18 q: ptqqpqrcrgqqqlqqmq",
        "6-7 h: jhxhkchjhhrhh",
        "8-11 n: nnbnnnnnnppgnnlnhbq",
        "15-18 g: gggqggrsggzggggdggg",
        "5-9 n: ptcnnjrnfnn",
        "9-15 h: hshwhbhhxhzhhhhhw",
        "1-3 x: fxxxx",
        "3-6 z: zzpzqzzzqzzzzzznzzwz",
        "10-11 p: lllpqpqvpppprppppp",
        "13-15 n: mnnnnnnntqcpdnn",
        "14-19 g: ggggggggggggggggggrg",
        "3-11 w: qtlpkwswvwwww",
        "3-8 g: lwgzgzml",
        "1-4 w: wwsw",
        "14-19 q: zcxqpjgxqfqqqqvjmklq",
        "4-5 h: hrxkh",
        "9-12 p: ppppgpppppppppp",
        "8-11 t: pwgtdxrtwtbskjnq",
        "2-4 m: pcdmkmlpwwxqw",
        "1-4 r: rrrxrr",
        "8-9 l: lllllwlbl",
        "8-9 p: pppppppfp",
        "4-5 x: jdxxs",
        "3-5 h: hvwth",
        "7-14 z: xkqhzztwjzzsgz",
        "7-11 b: bbwbbbcbbbbb",
        "2-4 f: fflf",
        "4-9 m: cmmmqlmmlxmgmmmmtmpm",
        "3-18 j: bnjznmljlhpfhcmnpcj",
        "7-9 z: zzxzvcbzzzz",
        "14-15 s: ssmssssssslssszss",
        "12-19 v: vgvvvvnvsvvrzvrvvvv",
        "8-10 q: qqqqcpqqqnqqqk",
        "11-14 p: zzvpwltfptcszpv",
        "2-9 h: khpnvdcvdh",
        "8-12 q: dqqqqqqlqqqqqq",
        "2-17 f: fffffffffgfgjfffxtt",
        "1-8 n: jnnnnrnn",
        "7-8 k: kkkhmkkdkkkk",
        "10-11 j: xjwmjcjjqrpjvlbjj",
        "3-4 v: vrvc",
        "4-9 r: dprrlbbrsrgqzvkc",
        "4-8 w: wrcwgsqghwwjw",
        "11-12 r: rrzrrrrrrrrrf",
        "4-5 c: rzccmcc",
        "1-8 w: wqwvwcwtwww",
        "7-9 f: hjfpvgffllfsfsft",
        "1-3 d: nddd",
        "12-13 c: ccccbccsccbbcccpcc",
        "1-11 d: xdhddwddjdg",
        "3-5 c: ccvcc",
        "2-6 d: djddddfd",
        "4-5 z: kzqszztpzz",
        "2-5 s: ssssz",
        "2-6 g: gdvjggrx",
        "9-11 k: qzklmckckkmkkk",
        "5-6 g: grjpwv",
        "8-12 n: nnnnnnngnnnnnnn",
        "15-19 z: zzzzzlmmczdzztgmzvzp",
        "4-5 p: pprvpmpdwppbqpmpw",
        "9-15 q: lfqrxjvdqnlqqtqgnqn",
        "1-4 w: pwwww",
        "1-9 q: qvqqqgqqfqnq",
        "2-4 k: rwlkkcqxcrwd",
        "2-12 q: qqdlpwqqftgjb",
        "5-7 q: qqqtlfq",
        "8-9 g: ggggggrgvtgg",
        "6-13 l: llllllllllllklll",
        "1-4 h: dhhhn",
        "5-6 f: ffjjsfft",
        "11-12 x: xxcxxxxxvxxxxx",
        "5-7 z: zzzwzfh",
        "12-14 d: vpblrhxdwrgdvkg",
        "1-3 n: xwrjrjdj",
        "8-9 z: zzzzdpzzghbzzzzwz",
        "15-16 p: pppppppppppppppw",
        "4-12 r: rrrvrrrrrrrrr",
        "8-9 d: dddddddndd",
        "4-10 l: llsfllllllll",
        "2-13 z: kzkctzprbpkkd",
        "2-4 c: qvcccd",
        "6-7 q: qqqrqqfqdq",
        "11-13 f: ffhffggfffgbf",
        "9-19 r: wncrtrxrrrjhjcvtvsbt",
        "9-11 h: hhhhhhhhlhh",
        "8-10 s: ggwhksdslsp",
        "3-13 h: hhhhhhhhhhhhfhhh",
        "6-7 h: hwghlnh",
        "7-10 r: rrrrrrjrrrr",
        "9-10 f: fbjfbfffff",
        "16-17 b: bbbbbbbbfbbbbbbjbbb",
        "13-16 t: tttttttttttttttz",
        "2-3 s: ssrsssjssss",
        "1-3 b: sbbbbk",
        "1-5 q: xcdbqqqdjxs",
        "10-12 p: jppvpstpjjpp",
        "8-15 b: bglbkmhbqbgfzfh",
        "1-6 x: xxxxxfx",
        "14-18 w: mwwwhtwjshwbvwrjrn",
        "1-2 w: wtww",
        "2-7 h: pxhntbg",
        "2-7 c: ccccccs",
        "2-5 z: zzzzczz",
        "3-5 n: tggvn",
        "13-14 h: hfhhhmthvhhwhdhhhhh",
        "11-16 n: nnnnznnpnnlknnnn",
        "2-6 d: dkdddddd",
        "12-15 g: gxbgggggggbnggg",
        "1-3 c: scxccc",
        "5-6 w: wwwwwfwww",
        "12-13 z: zzzzzzzzzzzczz",
        "6-9 j: psjjmjrxjnrwxzjjnnf",
        "15-16 h: hhhhhhhthhvhhhkhh",
        "4-5 n: nnnnj",
        "11-12 g: gqggdgggggggrgggm",
        "2-5 n: mqnlthrtjnr",
        "11-13 n: nnnnnnnnvnnnx",
        "2-4 b: xbbr",
        "2-7 j: rzjshrj",
        "18-20 b: gzcxrqfqmlbqmvrttbbp",
        "10-13 h: hhhhkhhhhrhhh",
        "5-6 n: mkjnnnqbdpznlndnnd",
        "5-6 w: wwwdqww",
        "15-19 v: vvvvvxvvvvvvvvzvvvv",
        "2-3 b: wlbhwdjwtncwpkbxvhc",
        "4-5 f: ffrrf",
        "15-16 v: vhvpvpvvvmckvbkvvvv",
        "2-6 m: mmbnrsq",
        "6-7 r: rlrqgrj",
        "7-12 c: ccccccsccccmcc",
        "1-9 g: kggggggglg",
        "4-9 f: frfffffnfzfcfff",
        "2-3 g: gcspt",
        "8-14 v: dvkpxqztcqttvv",
        "5-11 r: rrrrtrrrrrrrrrrrr",
        "4-15 c: jdrnrvqrzckbrxmzsgl",
        "3-5 t: dkttc",
        "4-11 j: ppjjwjjtdjt",
        "7-9 k: kkkkkkkkm",
        "1-4 p: lcpp",
        "3-6 w: wwwwwvw",
        "3-13 h: ghhlhhhhhbhgt",
        "5-6 f: ffzvft",
        "1-2 p: bppp",
        "6-12 m: lnnfbnmdrngmpt",
        "7-8 n: blfngnngnnnn",
        "5-8 s: ssssssss",
        "2-8 d: ddddddds",
        "6-11 x: gxnxbfxcrkjxxxhx",
        "9-10 m: wzfmmmvmtfrmm",
        "5-11 n: xgkpshnxwnn",
        "14-18 z: ztzczzzznzzzzzzzvz",
        "5-7 k: kgkkkks",
        "10-11 z: zvglzzxvqzpdj",
        "1-16 z: rdhsbnvmpfqpzmrzw",
        "13-14 m: mmmmmmmmmmmmmxm",
        "12-16 n: fsmnddnmkmjnkncn",
        "2-5 l: mqlllmgb",
        "2-4 k: xgmthfprbsk",
        "2-4 q: qssq",
        "12-13 r: rrrrrrrrrrrrq",
        "1-19 v: vzdtngrnnvmnpzvbmwvg",
        "7-11 m: lrjmsvmmmsjjmw",
        "4-7 r: rxrbfzrtvrhdq",
        "1-4 t: ltttttttt",
        "5-16 k: cktfdzrxppmkjfhk",
        "5-9 f: qfsfcfbtfl",
        "2-7 p: pvtkpppmppppd",
        "2-7 v: jkrmnjv",
        "2-4 n: qnjhnnnnfnnk",
        "2-3 n: ndvp",
        "1-3 g: ggdgg",
        "16-17 r: rrrrrbtrqrrrrrrvrrr",
        "8-10 r: rbkprrrmvr",
        "6-12 j: hqjwbjxjwjmjjqjhn",
        "16-17 r: rzrrrrrkrrrrrrrvrrr",
        "5-8 d: ddddldhdhd",
        "9-20 g: ggggghgggggcggggglgq",
        "7-9 c: rgmkcfctjpdccdwvtfcc",
        "2-12 w: wwwswtswhwhbdww",
        "4-10 t: ttdttttftt",
        "2-9 g: svtkbzggg",
        "4-7 r: rqrwrcwrqrr",
        "9-15 s: dsssrzsksmswwsljbbs",
        "5-13 d: dddhvdxhdrhddrd",
        "11-19 s: zdssvpmlqxqjbsssckp",
        "2-6 w: swcnwz",
        "17-18 n: nnnnnnnnnnnsnnnnnlcn",
        "6-7 t: tzzrcht",
        "4-10 k: bdckkrckkzk",
        "3-9 h: nrdrdvhhhh",
        "1-4 s: gssss",
        "3-15 h: jhhhtzmpvbhhjhck",
        "3-5 g: xlgzfg",
        "7-11 j: pmwflgjwjjrkl",
        "6-18 j: jjjjjjjjjjjjjjjjjt",
        "6-11 k: kkkkkskkkkkkvk",
        "4-15 l: llllllllldlllllllll",
        "5-6 w: rwtwfwg",
        "1-11 x: xxxgxxxxxxx",
        "2-7 v: vsgvvcvvv",
        "12-14 r: rrgrrrrrprrjrrr",
        "9-10 f: mclwdbqffzcsxqr",
        "6-7 v: vvkvvvv",
        "2-4 c: cczch",
        "6-10 c: hcqccmccscccrck",
        "1-4 v: vvvh",
        "6-12 s: nwfjghlpqsks",
        "4-5 p: hpcpt",
        "6-11 d: rdbgvdggzsj",
        "2-4 d: dddmdxx",
        "1-14 n: gnnnsnpfnnnbvnnj",
        "2-4 q: ckhqmjqqq",
        "2-7 w: vwpmwnp",
        "3-5 w: pnwwb",
        "8-12 k: kckkkkkknkkkkkp",
        "5-8 c: cccctcccccfcc",
        "13-14 n: qnnnnmlnnwhnnx",
        "3-4 f: qffb",
        "11-12 w: rwwwwwwwwwjpw",
        "6-11 b: pxgbbbcbbzbfbb",
        "12-14 m: mmmmmmmmmmmvmmmmm",
        "5-13 p: ppppxppppppppqpphh",
        "5-10 l: llllwllllgll",
        "1-8 c: ccfzcczscvjcc",
        "2-4 m: cmlrm",
        "4-7 j: szjkflj",
        "6-9 p: cjnppqpsppppjpdbhpf",
        "11-12 k: kkkjkkkkdkrkg",
        "4-15 j: jjjqjjjjjjjjdxcjjjj",
        "4-5 d: dddddd",
        "4-5 l: hjlllqdwvl",
        "12-14 r: rrmrrrrqrlrxrr",
        "7-8 c: nbzxczcn",
        "8-9 l: pgmlwccjlrg",
        "6-8 z: zzzzzzbzzj",
        "1-8 r: mrrrrrrrrrf",
        "1-3 w: wwww",
        "5-6 d: qldxdsdk",
        "3-14 h: hfdhhhhhhhhhhshhh",
        "9-10 q: lbvqvwfvnkmth",
        "7-10 v: lvdvwnvvnvt",
        "2-3 q: qqspq",
        "10-13 m: mmmmmmmlmmmmcmmmm",
        "4-6 w: wwwwwlw",
        "1-9 h: jkkhqlhhd",
        "6-8 x: xxxxxxxx",
        "13-16 b: bbbbbbbbbbbbbbbs",
        "17-18 m: lmmmspmmkmzmdmstvms",
        "1-5 g: ncgnggglggrgggrh",
        "2-6 t: sxjbjt",
        "5-7 k: ktkkkkvk",
        "7-8 k: kxkzkkkkks",
        "7-11 p: jvrfhrjpspt",
        "5-7 k: kkkkkkpk",
        "3-6 h: qhthhhshhhxhhhh",
        "1-6 l: llllzq",
        "2-4 k: bhqk",
        "2-3 m: tkfm",
        "4-14 m: mlzmmxjjphrcmcn",
        "15-19 w: wqwwwwwwwwwwwwfwwwlw",
        "11-13 k: wzskknsplzkkpnkmk",
        "3-4 g: gdgn",
        "1-4 b: psdbbbbkntx",
        "12-13 d: wdddddlddddfdd",
        "4-5 d: dddjwdf",
        "14-18 n: ntnsnnnnknrnnxnqnn",
        "1-8 v: vvvvvvvtvvv",
        "13-14 s: shwnbsssjwssss",
        "3-6 g: dgglgfzzg",
        "2-5 f: ffffmfff",
        "8-15 t: dtxltcttwtttttt",
        "9-14 z: zzzzzzzzzzzzznzz",
        "6-10 w: swwwwwwwwbww",
        "4-10 j: tqxwnppjxjbzrjppm",
        "3-4 b: slbdt",
        "3-7 d: bdsmhtr",
        "10-19 z: gznrnzzzzzvkztzznzbl",
        "8-12 t: whtztgtwtttt",
        "3-9 r: rfccdfmnrpj",
        "6-7 m: qhmmmpm",
        "12-15 q: zrnwqblrdqlqjrlptg",
        "6-10 c: ccccckccccccc",
        "15-17 r: rrrrrrrrrrrrrrrrhr",
        "3-11 g: ggggggggkgjgg",
        "6-7 b: mxjzshbgsdjcwsbjchgk",
        "10-11 h: hchghhhhhgh",
        "4-6 d: qvdddctdd",
        "2-5 b: qqpzfl",
        "7-8 z: zztgzzzh",
        "6-8 h: zmschbhh",
        "7-15 b: wbbbjpbcbjbbrshp",
        "1-10 n: ccnnznfnjd",
        "7-8 s: snjqvsss",
        "8-16 d: ddddddddddkddddcddd",
        "8-17 t: tjtrttlttrtptctjr",
        "5-7 d: ddddkddd",
        "5-6 k: kkgkks",
        "6-14 c: szcskczcftcctk",
        "2-3 b: zblpbt",
        "12-13 f: grpkpffxfftsf",
        "6-14 s: sssssssssssssws",
        "13-16 t: ttmmfcthmtcmttpn",
        "1-4 h: zrhhs",
        "2-3 k: krgk",
        "3-4 l: lllv",
        "8-9 z: zdbzzzzrzzz",
        "7-8 g: njlrzggqjgg",
        "4-5 c: dpcqc",
        "4-6 n: nnnhnnn",
        "3-4 l: lllq",
        "5-7 v: jbvvvphvwnhkmjrbhcsn",
        "12-14 d: kvbwfkjzdcpcjd",
        "9-12 h: hrhhhhhhxhhhhhh",
        "5-7 f: tffvfngffkhfff",
        "2-7 v: vvvvvvhvvvvvvvvv",
        "7-9 v: rcdhfnlhmwsgrzqz",
        "9-11 j: pfgzjbrbmjj",
        "6-7 j: ldxbbjnrjj",
        "8-14 d: kdddsdqdvddqcckr",
        "3-12 l: fxknndgbgdllkpzx",
        "2-9 f: xffbmfnfffpqf",
        "9-12 t: lttzwtgtktttkjct",
        "2-11 s: zsxrcxtqwmv",
        "5-8 s: ssdsspscsb",
        "4-13 r: shdxtlrmzqlrrwtdnpwx",
        "12-14 j: jjjlzzcvjjjvjjz",
        "2-4 s: ssvsc",
        "4-6 q: qkqkqqqqkq",
        "9-11 z: czzmzwzbbhz",
        "11-12 x: xxxxxxlxxxxpxxx",
        "7-13 x: xxxxhxxxxxxxxxx",
        "2-4 g: wggq",
        "2-6 v: zvxpnqvqwpmbfwnrl",
        "5-6 m: mmmmmc",
        "1-4 v: vvccm",
        "14-16 n: nngdnnnnnntxnwnn",
        "6-14 q: qfdqszrcvfwcqj",
        "13-14 z: zznzzszzzzzzdfz",
        "2-4 p: cppp",
        "3-7 q: qftqqdvh",
        "4-6 t: rrqsftttxs",
        "1-2 z: zzlzsz",
        "3-4 g: vsbg",
        "5-6 j: tzljjh",
        "1-2 v: vptfbrwgvztwp",
        "2-6 w: hqtnzw",
        "8-9 f: ffdffhcxf",
        "1-2 d: dbddddd",
        "12-14 f: fffffffffsfffpff",
        "12-13 t: ftwtgttttxxtbtwttt",
        "6-8 k: qvpkkmkkk",
        "2-9 b: tvjntzdbgmdbbbljwbmb",
        "1-4 v: vfvvvc",
        "8-12 v: vvvvvvvvvvvbv",
        "2-4 n: wncnxnmch",
        "10-12 t: tttqxrtttttvjrc",
        "11-14 f: fffffffffffffpf",
        "16-20 h: hhhjthhhtphchpkhmhhh",
        "4-5 m: zzmml",
        "6-7 c: ccccdcnclkccccck",
        "15-17 c: sxzcbfcntlgccwckcd",
        "9-17 t: mkfttrtvtwdsxxttf",
        "2-3 t: dtzt",
        "4-5 z: zzzbz",
        "12-17 f: zcfftrnfwvfhnvfffsdf",
        "7-16 f: ffdfxxtwffvdffjff",
        "3-5 r: rrxrr",
        "13-14 b: kbwbbbvbbmbwbb",
        "10-15 f: zflnbhfqmhfsqnf",
        "3-5 p: fwpptwzppkbhp",
        "13-14 k: kkkkkkkkbkkkjkk",
        "7-11 b: hqdbtbbhpht",
        "1-6 f: tffffff",
        "1-2 d: bddsrd",
        "13-18 m: mmmmmmmmmmmmxmmmmlmm",
        "3-4 x: xxkx",
        "4-13 z: hzmwzszzzzlhzzxknb",
        "10-15 h: hhhhhchhhbhbhhhhhk",
        "3-4 h: vhhhb",
        "10-11 z: zxwzzzzzllvzzzzmz",
        "7-10 h: phdkrrhkmhh",
        "2-11 j: jjjjjjjjjjtzjjjjg",
        "3-4 g: gxfgggg",
        "1-3 f: fwtdf",
        "8-9 d: bjqxpvzdddx",
        "7-15 c: vcccccccccccccccc",
        "1-7 n: tvnpzhn",
        "1-2 v: ktvv",
        "2-3 g: gpggg",
        "7-13 d: fddcdfgvbmpdd",
        "4-5 s: rsssw",
        "2-14 c: jckbwnnlkcmvnwtj"
    ]
}

method Main() {
    var puzzle_input := PuzzleInput();
    var entries: array<Entry> := new Entry[|puzzle_input|];

    var i := 0;
    while i < |puzzle_input|
        invariant 0 <= i <= |puzzle_input|
    {
        entries[i] := Parse(puzzle_input[i]);
        i := i + 1;
    }

    var result := CountValidEntries(entries);
    print result;
}
