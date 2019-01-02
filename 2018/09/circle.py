class CircularList:
    class Node:
        def __init__(self, list, value):
            self.list = list
            self.value = value
            self.prev = None
            self.next = None

    class Cursor:
        def __init__(self, node):
            self.pos = node

        def get(self):
            return self.pos.value

        def next(self):
            v = self.pos.value
            self.pos = self.pos.next
            return v

        def move_back(self, n=1):
            for i in range(n):
                self.pos = self.pos.prev

        def move_forward(self, n=1):
            for i in range(n):
                self.pos = self.pos.next

        def insert_after(self, value):
            list = self.pos.list
            n = list.Node(list, value)
            n.prev = self.pos
            n.next = self.pos.next
            n.prev.next = n
            n.next.prev = n
            list.count += 1

        def insert_and_move_forward(self, value):
            self.insert_after(value)
            self.pos = self.pos.next

        def insert_before(self, value):
            list = self.pos.list
            n = list.Node(list, value)
            n.prev = self.pos.prev
            n.next = self.pos
            n.prev.next = n
            n.next.prev = n
            list.count += 1

        def remove_and_move_forward(self):
            n = self.pos
            list = n.list
            n.prev.next = n.next
            n.next.prev = n.prev
            self.pos = n.next
            list.count -= 1
            return n.value

        def clone(self):
            return self.__class__(self.pos)

    def __init__(self, data=()):
        self.count = 0
        self._first = None
        cursor = None
        for v in data:
            if self._first is None:
                n = self.Node(self, v)
                n.prev = n
                n.next = n
                self._first = n
                self.count = 1
                cursor = self.Cursor(n)
            else:
                cursor.insert_before(v)

    def __iter__(self):
        if self.count > 0:
            node = self._first
            yield node.value
            node = node.next
            while node is not self._first:
                yield node.value
                node = node.next

    def cursor(self):
        return self.Cursor(self._first)

def test():
    a = CircularList()
    assert a.count == 0

    a = CircularList([1, 2, 3])
    assert a.count == 3
    assert list(a) == [1, 2, 3]

    a = CircularList([0])
    c = a.cursor()
    for i in [1, 2, 3, 4]:
        c.insert_after(i)
    assert list(a) == [0, 4, 3, 2, 1]

    a = CircularList([0])
    c = a.cursor()
    for i in [1, 2, 3, 4]:
        c.insert_and_move_forward(i)
        c.insert_after(i)
    assert list(a) == [0, 1, 2, 3, 4, 4, 3, 2, 1]
    assert c.get() == 4

if __name__ == '__main__':
    test()
