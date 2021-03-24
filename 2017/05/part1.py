
def run(program):
    steps = 0
    pc = 0
    while 0 <= pc < len(program):
        i = program[pc]
        program[pc] = i + 1
        pc += i
        steps += 1
    return steps

assert run([0, 3, 0, -1, -3]) == 5

if __name__ == '__main__':
    with open("puzzle-input.txt") as f:
        program = [int(line.strip()) for line in f]
    print(run(program))


    

