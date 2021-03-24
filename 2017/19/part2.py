
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

    nsteps = 1
    while True:
        x += dx
        y += dy
        nsteps += 1
        c = grid[y][x]
        if c == ' ':
            return nsteps - 1 # don't count final step into air
        elif c == '+':
            assert grid[y + dy][x + dx] == ' '
            if grid[y + dx][x + dy] != ' ':
                assert grid[y - dx][x - dy] == ' '
                dx, dy = dy, dx
            else:
                assert grid[y - dx][x - dy] != ' '
                dx, dy = -dy, -dx
        else:
            assert c == '-' or c == '|' or c.isalpha()

sample_path = '''\
     |          
     |  +--+    
     A  |  C    
 F---|----E|--+ 
     |  |  |  D 
     +B-+  +--+ 
                
'''

print( run(sample_path.splitlines()) )

with open('puzzle-input.txt') as f:
    grid = f.readlines()
print(run(grid))
