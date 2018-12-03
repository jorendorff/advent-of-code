import sys, subprocess

def run_test(script, input_nums, expected):
    p = subprocess.Popen(["python", script],
                         executable=sys.executable,
                         stdin=subprocess.PIPE,
                         stdout=subprocess.PIPE)
    for n in input_nums:
        if n != 0:
            p.stdin.write("{:+d}\n".format(n))
    p.stdin.close()
    result = p.stdout.read()
    if result != expected:
        raise ValueError("wrong answer: expected {!r}, got {!r}".format(expected, result))

def run_test1(input_nums, expected):
    assert expected == "{}\n".format(sum(input_nums))
    run_test("frequency.py", input_nums, expected)

run_test1([1, 1, 1], "3\n")
run_test1([1, 1, -2], "0\n")
run_test1([-1, -2, -3], "-6\n")
run_test1([3, 2, -1, -1, 6000, 80, -3], "6080\n")

def run_test2(input_nums, expected):
    run_test("frequency2.py", input_nums, expected)

run_test2([1, -2, 3, 1], "2\n")
run_test2([1, -1], "0\n")
run_test2([3, 3, 4, -2, -4], "10\n")
run_test2([-6, 3, 8, 5, -6], "5\n")
run_test2([7, 7, -2, -7, -4], "14\n")
run_test2([100, -99], "100\n")
