
fn partial_sums(arr: &[i64]) -> Vec<i64> {
    let mut out = Vec::with_capacity(arr.len() + 1);
    out.push(0);
    let mut total = 0;
    for &v in arr {
        total += v;
        out.push(total);
    }
    out
}

fn phase(arr: &[i64]) -> Vec<i64> {
    let n = arr.len();
    let psums = partial_sums(arr);

    let sumrange = |start, mut stop| {
        if stop > n {
            stop = n;
        }
        psums[stop] - psums[start]
    };

    (1..n + 1)
        .map(|wavelength: usize| -> i64 {
            let mut start = wavelength - 1;
            let mut polarity = 1;
            let mut total = 0;
            while start < n {
                total += polarity * sumrange(start, start + wavelength);
                start += 2 * wavelength;
                polarity = -polarity;
            }
            total.abs() % 10
        })
        .collect()
}

fn decode(s: &str) -> Vec<i64> {
    let mut arr = Vec::new();
    for c in s.chars() {
        assert!(c.is_ascii_digit());
        arr.push(c as i64 - '0' as i64);
    }
    arr
}

fn encode(results: &[i64]) -> String {
    results[..8]
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<String>>()
        .join("")
}

#[test]
fn test_phase() {
    let fft1 = |s| encode(&phase(&decode(s)));

    let fft_n = |n, s| {
        let mut arr = decode(s);
        for _ in 0..n {
            arr = phase(&arr);
        }
        encode(&arr)
    };

    assert_eq!(fft1("12345678"), "48226158");
    assert_eq!(fft1("48226158"), "34040438");
    assert_eq!(fft_n(4, "12345678"), "01029498");
}




const PUZZLE_INPUT: &str = "59762677844514231707968927042008409969419694517232887554478298452757853493727797530143429199414189647594938168529426160403829916419900898120019486915163598950118118387983556244981478390010010743564233546700525501778401179747909200321695327752871489034092824259484940223162317182126390835917821347817200000199661513570119976546997597685636902767647085231978254788567397815205001371476581059051537484714721063777591837624734411735276719972310946108792993253386343825492200871313132544437143522345753202438698221420628103468831032567529341170616724533679890049900700498413379538865945324121019550366835772552195421407346881595591791012185841146868209045";


fn main() {
    let mut arr = Vec::new();
    for c in PUZZLE_INPUT.chars() {
        assert!(c.is_ascii_digit());
        arr.push(c as i64 - '0' as i64);
    }

    // repeat the list 10000 times
    arr = (0..10000).flat_map(|_| &arr).cloned().collect();

    for i in 0..100 {
        println!("{}", i);
        arr = phase(&arr);
    }

    let mut acc = 0;
    for &digit in &arr[5976267..5976267 + 8] {
        acc = acc * 10 + digit;
    }
    println!("{}", acc);
}
