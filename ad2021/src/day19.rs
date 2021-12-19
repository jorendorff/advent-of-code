use std::collections::{HashMap, HashSet};

use aoc_runner_derive::*;
use nalgebra::{matrix, vector, Matrix4, Vector4};

type Point = Vector4<i32>;
type Transform = Matrix4<i32>; // 4x4 matrix

fn parse_point(s: &str) -> anyhow::Result<Point> {
    let fields: Vec<&str> = s.split(',').collect();
    anyhow::ensure!(fields.len() == 3);
    let x: i32 = fields[0].parse()?;
    let y: i32 = fields[1].parse()?;
    let z: i32 = fields[2].parse()?;
    Ok([x, y, z, 1].into())
}

#[aoc_generator(day19, part1, jorendorff)]
#[aoc_generator(day19, part2, jorendorff)]
fn parse_input(text: &str) -> anyhow::Result<Vec<Vec<Point>>> {
    text.split("\n\n")
        .enumerate()
        .map(|(i, section)| -> anyhow::Result<Vec<Point>> {
            let mut lines = section.lines();
            let header = lines.next().unwrap();
            assert_eq!(header, format!("--- scanner {} ---", i));
            lines.map(parse_point).collect()
        })
        .collect()
}

fn rotations() -> impl Iterator<Item = Transform> {
    let rotate_xyz: Transform = matrix![
        0, 1, 0, 0;
        0, 0, 1, 0;
        1, 0, 0, 0;
        0, 0, 0, 1
    ];

    let rotate_z_half_turn: Transform = matrix![
        -1, 0, 0, 0;
        0, -1, 0, 0;
        0, 0, 1, 0;
        0, 0, 0, 1
    ];

    let rotate_x_quarter_turn: Transform = matrix![
        1, 0, 0, 0;
        0, 0, -1, 0;
        0, 1, 0, 0;
        0, 0, 0, 1
    ];

    let mut current = Transform::identity();

    (0..24).map(move |i| {
        current *= rotate_x_quarter_turn;
        if i % 4 == 0 {
            current *= rotate_z_half_turn;
            if i % 8 == 0 {
                current *= rotate_xyz;
            }
        }
        current
    })
}

fn translation(v: Point) -> Transform {
    matrix![
        1, 0, 0, *v.index(0);
        0, 1, 0, *v.index(1);
        0, 0, 1, *v.index(2);
        0, 0, 0, 1;
    ]
}

fn try_placement(
    min_overlap: usize,
    b_scan: &[Point],
    c_scan: &[Point],
    (b0, b1): (usize, usize),
    (c0, c1): (usize, usize),
) -> Option<Transform> {
    let from_c = translation(-c_scan[c0]);
    let to_b = translation(b_scan[b0]);
    assert_eq!(to_b * from_c * c_scan[c0], b_scan[b0]);

    let b_vec = b_scan[b1] - b_scan[b0];
    let c_vec = c_scan[c1] - c_scan[c0];
    for rotation in rotations() {
        if rotation * c_vec == b_vec {
            let solution = to_b * rotation * from_c;
            assert_eq!(solution * c_scan[c0], b_scan[b0]);
            assert_eq!(solution * c_scan[c1], b_scan[b1]);
            let b_points = b_scan.iter().copied().collect::<HashSet<Point>>();
            let shared_points = c_scan
                .iter()
                .copied()
                .map(|point| solution * point)
                .filter(|c_point| b_points.contains(c_point))
                .count();
            assert!(shared_points >= 2);
            if shared_points >= min_overlap {
                return Some(solution);
            } else {
                println!("found {} shared points", shared_points);
                return None;
            }
        }
    }

    // The vectors can't be rotated into agreement; they are mirror images of one another.
    None
}

type Summary = HashMap<[i32; 3], Vec<(usize, usize)>>;

fn try_place(
    min_overlap: usize,
    b_scan: &[Point],
    b_summary: &Summary,
    c_scan: &[Point],
    c_summary: &Summary,
) -> Option<Transform> {
    // The summaries describe all possible matching point-pairs in these two scans.
    let mut b_summary_deterministic = b_summary.iter().collect::<Vec<_>>();
    b_summary_deterministic.sort();
    for (key, b_pairs) in b_summary_deterministic {
        if let Some(c_pairs) = c_summary.get(key) {
            // Perhaps a point-pair in b_pairs coincides with one in c_pairs.
            for &b_pair in b_pairs {
                for &c_pair in c_pairs {
                    // Perhaps this particular point-pair.
                    if let Some(transform) =
                        try_placement(min_overlap, b_scan, c_scan, b_pair, c_pair)
                    {
                        return Some(transform);
                    }
                    // Also try the other direction.
                    let c_pair_rev = (c_pair.1, c_pair.0);
                    if let Some(transform) =
                        try_placement(min_overlap, b_scan, c_scan, b_pair, c_pair_rev)
                    {
                        return Some(transform);
                    }
                }
            }
        }
    }
    None
}

// Returns the positions of beacons and the positions of scanners, relative to scanner 0.
// The scanner positions come out in the order we find them, not sorted by scanner id.
fn make_map(scans: &[Vec<Point>], min_overlap: usize) -> (HashSet<Point>, Vec<Point>) {
    println!("----\nmake_map in, {} scans", scans.len());

    let origin: Point = vector![0, 0, 0, 1];

    let mut scanners = vec![origin];
    let mut out = scans[0].iter().copied().collect::<HashSet<Point>>();

    // Build a summary of each scan. The summary is the set of all vectors from
    // one point in that scan to another, stripped of sign and axis
    // information, so that e.g. the vector <8, -3, 1> is represented as the
    // coordinate-set {1, 3, 8}.
    let summary: Vec<Summary> = scans
        .iter()
        .map(|scan| {
            let mut acc: Summary = HashMap::new();
            for (j, b) in scan.iter().copied().enumerate() {
                for (i, a) in scan[..j].iter().copied().enumerate() {
                    let v = b - a;
                    let mut coords = [v.index(0).abs(), v.index(1).abs(), v.index(2).abs()];
                    coords.sort_unstable();
                    acc.entry(coords).or_default().push((i, j));
                }
            }
            acc
        })
        .collect();

    // Number of vectors two summaries must share, if they share `min_overlap` points.
    let threshold = min_overlap * (min_overlap - 1) / 2;

    let n = scans.len();
    let mut placement: Vec<Option<Transform>> = vec![None; n];
    placement[0] = Some(Matrix4::identity());
    let mut queue = vec![0]; // includes already done elements
    let mut done = 0; // index of front of queue
    while done < queue.len() {
        let b = queue[done]; // base scan
        assert!(placement[b].is_some());
        for c in 0..n {
            // candidate scan
            if placement[c].is_none() {
                let num_common = summary[b]
                    .iter()
                    .map(|(v, b_pairs)| {
                        summary[c]
                            .get(v)
                            .map(|c_pairs| b_pairs.len().min(c_pairs.len()))
                            .unwrap_or(0)
                    })
                    .sum::<usize>();
                if num_common >= threshold {
                    // This is a candidate. Try really placing it.
                    if let Some(transform) =
                        try_place(min_overlap, &scans[b], &summary[b], &scans[c], &summary[c])
                    {
                        // It fits!
                        let c_to_0 = placement[b].unwrap() * transform;
                        scanners.push(c_to_0 * origin);
                        placement[c] = Some(c_to_0);
                        queue.push(c);
                        for point in &scans[c] {
                            out.insert(c_to_0 * point);
                        }
                    }
                }
            }
        }
        done += 1;
    }

    assert_eq!(done, scans.len());
    assert!(placement.iter().all(Option::is_some));
    assert_eq!(scanners.len(), scans.len());
    (out, scanners)
}

#[aoc(day19, part1, jorendorff)]
fn part_1(scans: &Vec<Vec<Point>>) -> usize {
    make_map(scans, 12).0.len()
}

fn manhattan_distance(a: &Point, b: &Point) -> i32 {
    let v = *b - *a;
    v.index(0).abs() + v.index(1).abs() + v.index(2).abs()
}

fn solve_part_2(scans: &[Vec<Point>], min_overlap: usize) -> i32 {
    let scanners = make_map(scans, min_overlap).1;
    (0..scanners.len())
        .flat_map(|i| {
            (0..i).map({
                let scanners = &scanners;
                move |j| manhattan_distance(&scanners[i], &scanners[j])
            })
        })
        .max()
        .unwrap()
}

#[aoc(day19, part2, jorendorff)]
fn part_2(scans: &Vec<Vec<Point>>) -> i32 {
    solve_part_2(scans, 12)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14
";

    const EXAMPLE_BEACONS: &str = "\
-892,524,684
-876,649,763
-838,591,734
-789,900,-551
-739,-1745,668
-706,-3180,-659
-697,-3072,-689
-689,845,-530
-687,-1600,576
-661,-816,-575
-654,-3158,-753
-635,-1737,486
-631,-672,1502
-624,-1620,1868
-620,-3212,371
-618,-824,-621
-612,-1695,1788
-601,-1648,-643
-584,868,-557
-537,-823,-458
-532,-1715,1894
-518,-1681,-600
-499,-1607,-770
-485,-357,347
-470,-3283,303
-456,-621,1527
-447,-329,318
-430,-3130,366
-413,-627,1469
-345,-311,381
-36,-1284,1171
-27,-1108,-65
7,-33,-71
12,-2351,-103
26,-1119,1091
346,-2985,342
366,-3059,397
377,-2827,367
390,-675,-793
396,-1931,-563
404,-588,-901
408,-1815,803
423,-701,434
432,-2009,850
443,580,662
455,729,728
456,-540,1869
459,-707,401
465,-695,1988
474,580,667
496,-1584,1900
497,-1838,-617
527,-524,1933
528,-643,409
534,-1912,768
544,-627,-890
553,345,-567
564,392,-477
568,-2007,-577
605,-1665,1952
612,-1593,1893
630,319,-379
686,-3108,-505
776,-3184,-501
846,-3110,-434
1135,-1161,1235
1243,-1093,1063
1660,-552,429
1693,-557,386
1735,-437,1738
1749,-1800,1813
1772,-405,1572
1776,-675,371
1779,-442,1789
1780,-1548,337
1786,-1538,337
1847,-1591,415
1889,-1729,1762
1994,-1805,1792
";

    fn parse_points(s: &str) -> anyhow::Result<Vec<Point>> {
        s.lines().map(parse_point).collect()
    }

    #[test]
    fn test_part_1() {
        let scans = parse_input(EXAMPLE).unwrap();

        assert_eq!(
            make_map(&scans, 3).0,
            parse_points(EXAMPLE_BEACONS)
                .unwrap()
                .into_iter()
                .collect::<HashSet<Point>>()
        );
        assert_eq!(make_map(&scans, 3).0.len(), 79);
    }

    #[test]
    fn test_part_2() {
        let scans = parse_input(EXAMPLE).unwrap();
        assert_eq!(solve_part_2(&scans, 12), 3621);
    }
}
