
def run(grid):
    grid = [line.strip('\n') for line in grid]
    height = len(grid)
    width = len(grid[0])
    y = 0
    x = grid[y].index('|')
    assert grid[y] == ' ' * x + '|' + ' ' * (width - x - 1)
    assert all(len(row) == width for row in grid)
    assert all(row[0] == ' ' and row[-1] == ' ' for row in grid)
    assert grid[-1] == ' ' * width
    dx, dy = 0, +1

    log = ''
    while True:
        x += dx
        y += dy
        c = grid[y][x]
        if c == ' ':
            return log
        elif c.isalpha():
            log += c
        elif c == '+':
            assert grid[y + dy][x + dx] == ' '
            if grid[y + dx][x + dy] != ' ':
                assert grid[y - dx][x - dy] == ' '
                dx, dy = dy, dx
            else:
                assert grid[y - dx][x - dy] != ' '
                dx, dy = -dy, -dx
        else:
            assert c == '-' or c == '|'

sample_path = '''\
     |          
     |  +--+    
     A  |  C    
 F---|----E|--+ 
     |  |  |  D 
     +B-+  +--+ 
                
'''

assert run(sample_path.splitlines()) == 'ABCDEF'

with open('puzzle-input.txt') as f:
    grid = f.readlines()
print(run(grid))
