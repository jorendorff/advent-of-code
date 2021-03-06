"""

"""

from lib.advent import *
from ad2019.intcode.interpreter import IntcodeVM, parse
import threading


class Computer(threading.Thread):
    def __init__(self, program, network, addr):
        threading.Thread.__init__(self)
        self.addr = addr
        self.vm = IntcodeVM(program, input=self.input, output=self.output)
        self.network = network
        self.queue = deque([addr])
        self.outgoing = []

    def receive(self, x, y):
        self.queue.append(x)
        self.queue.append(y)

    def input(self):
        if self.queue:
            return self.queue.popleft()
        return -1

    def output(self, value):
        self.outgoing.append(value)
        if len(self.outgoing) == 3:
            addr, x, y = self.outgoing
            if 0 <= addr < len(self.network):
                self.network[addr].receive(x, y)
            elif addr == 255:
                print(f"\n\n\nPACKET SENT TO 255: x={x} y={y} ***\n\n\n")
            else:
                print("packet dropped:", self.outgoing)
            del self.outgoing[:]

    def run(self):
        self.vm.run_some()
        print(f"computer {id} halted")
        assert self.vm.state == "halt"


COMPUTER_COUNT = 50


def main():
    program = parse(puzzle_input())

    network = []
    for addr in range(COMPUTER_COUNT):
        network.append(Computer(program, network, addr))

    for computer in network:
        computer.start()


if __name__ == '__main__':
    main()
