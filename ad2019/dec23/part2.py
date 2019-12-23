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
        self.idle = None

    def receive(self, x, y):
        self.queue.append(x)
        self.queue.append(y)

    def input(self):
        if self.queue:
            return self.queue.popleft()
        self.idle = True
        time.sleep(0.1)
        self.idle = False
        return -1

    def output(self, value):
        self.outgoing.append(value)
        if len(self.outgoing) == 3:
            self.network.send(*self.outgoing)
            del self.outgoing[:]

    def run(self):
        self.idle = False
        self.vm.run_some()
        print(f"computer {id} halted")
        assert self.vm.state == "halt"


class Nat(threading.Thread):
    def __init__(self, network):
        threading.Thread.__init__(self)
        self.network = network
        self.last_received = None

    def receive(self, x, y):
        self.last_received = x, y
        self.last_y_value_sent = None

    def run(self):
        while True:
            if all(c.idle for c in self.network.computers):
                x, y = self.last_received
                if y == self.last_y_value_sent:
                    print("sent again:", y)
                    return
                print("kicked network")
                self.network.send(0, x, y)
                self.last_y_value_sent = y


COMPUTER_COUNT = 50
NAT_ADDR = 255


class Network:
    def __init__(self, program):
        self.computers = [Computer(program, self, addr) for addr in range(COMPUTER_COUNT)]

    def start(self):
        for computer in self.computers:
            computer.start()
        

    def send(self, addr, x, y):
        if 0 <= addr < COMPUTER_COUNT:
            self.computers[addr].receive(x, y)
        elif addr == NAT_ADDR:
            
        

def main():
    program = parse(puzzle_input())

    network = []
    for addr in range(COMPUTER_COUNT):
        network.append(Computer(program, network, addr))

    for computer in network:
        computer.start()


if __name__ == '__main__':
    main()
