
def run(program):
    steps = 0
    pc = 0
    while 0 <= pc < len(program):
        i = program[pc]
        if i >= 3:
            program[pc] -= 1
        else:
            program[pc] += 1
        pc += i
        steps += 1
    return steps

test_prog = [0, 3, 0, 1, -3]
assert run(test_prog) == 10
assert test_prog == [2, 3, 2, 3, -1]

if __name__ == '__main__':
    with open("puzzle-input.txt") as f:
        program = [int(line.strip()) for line in f]
    print(run(program))
    

