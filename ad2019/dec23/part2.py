"""

"""

from lib.advent import *
from ad2019.intcode.interpreter import IntcodeVM, parse
import threading, time


class Computer(threading.Thread):
    def __init__(self, program, network, addr):
        threading.Thread.__init__(self)
        self.addr = addr
        self.vm = IntcodeVM(program, input=self.input, output=self.output)
        self.network = network
        self.queue = deque([addr])
        self.outgoing = []
        self.idle_count = 0
        self.idle = None

    def receive(self, x, y):
        self.queue.append(x)
        self.queue.append(y)
        if self.idle:
            self.idle = False
            print(f"computer {self.addr} received a packet")
        self.idle_count = 0

    def input(self):
        if self.queue:
            return self.queue.popleft()
        self.idle_count += 1
        if self.idle_count > 1:
            if not self.idle:
                print(f"computer {self.addr} is idle")
                self.idle = True
            time.sleep(0.1)
        return -1

    def output(self, value):
        if self.idle:
            print(f"computer {self.addr} sent a value")
            self.idle = False
        self.idle_count = 0
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
        self.last_y_value_sent = None

    def receive(self, x, y):
        was_unstarted = self.last_received is None
        self.last_received = x, y
        if was_unstarted:
            self.start()

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

                # wait for network to wake up again
                while all(c.idle for c in self.network.computers):
                    time.sleep(0.1)


COMPUTER_COUNT = 50
NAT_ADDR = 255


class Network:
    def __init__(self, program):
        self.computers = [Computer(program, self, addr) for addr in range(COMPUTER_COUNT)]
        self.nat = Nat(self)

    def start(self):
        for computer in self.computers:
            computer.start()

    def send(self, addr, x, y):
        if 0 <= addr < COMPUTER_COUNT:
            self.computers[addr].receive(x, y)
        elif addr == NAT_ADDR:
            self.nat.receive(x, y)


def main():
    program = parse(puzzle_input())

    network = Network(program)
    network.start()


if __name__ == '__main__':
    main()
