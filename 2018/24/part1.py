import re
import textwrap

class Group:
    def __init__(self, team_name, group_number, units, hp, weaknesses, immunities, attack_damage, attack_type, initiative):
        self.team_name = team_name
        self.group_number = group_number
        self.units = units
        self.hp = hp
        self.weaknesses = weaknesses
        self.immunities = immunities
        self.attack_damage = attack_damage
        self.attack_type = attack_type
        self.initiative = initiative
        self.is_targeted = False
        self.target = None

    def effective_power(self):
        return self.units * self.attack_damage

    def target_selection_priority(self):
        return (-self.effective_power(), -self.initiative)

    def target_quality(self, target):
        return (attack_damage(self, target), target.effective_power(), target.initiative)

    def choose_target(self, enemy_groups):
        target = max([g for g in enemy_groups if not g.is_targeted],
                     default=None,
                     key=self.target_quality)
        if target is None or attack_damage(self, target) == 0:
            self.target = None
        else:
            self.target = target
            target.is_targeted = True

    def receive_damage(self, amount):
        killed = amount // self.hp
        self.units -= killed
        if self.units < 0:
            self.units = 0


def attack_damage(g1, g2):
    """The amount of damage attacking group g1 would deal to defending group g2 in
    a fight, after accounting for weaknesses and immunities, but not accounting
    for whether the defending group has enough units to actually receive all of
    that damage."""
    dmg = g1.effective_power()
    if g1.attack_type in g2.immunities:
        dmg = 0
    if g1.attack_type in g2.weaknesses:
        dmg *= 2
    return dmg

def fight(army1, army2):
    log = ''

    # target selection phase
    for unit in army1 + army2:
        unit.is_targeted = False
        unit.target = None
    for attackers, defenders in [(army1, army2), (army2, army1)]:
        for unit in sorted(attackers, key=Group.target_selection_priority):
            unit.choose_target(defenders)

    # attacking phase
    for group in sorted(army1 + army2, key=lambda g: -g.initiative):
        if group.units > 0:
            target = group.target
            if target is not None:
                dmg = attack_damage(group, target)
                units_before = target.units
                target.receive_damage(dmg)
                kills = units_before - target.units
                log += '{} group {} attacks defending group {}, killing {} unit{}\n'.format(
                    group.team_name,
                    group.group_number,
                    target.group_number,
                    kills,
                    's' if kills != 1 else '')

    # cleanup
    army1[:] = [g for g in army1 if g.units]
    army2[:] = [g for g in army2 if g.units]

    return log

def army_desc(army_name, army):
    log = '{}:\n'.format(army_name)
    if army:
        for group in army:
            log += 'Group {} contains {} unit{}\n'.format(
                group.group_number,
                group.units,
                's' if group.units != 1 else '')
    else:
        log += 'No groups remain.\n'
    return log

def combat(army1, army2):
    a1name = army1[0].team_name
    a2name = army2[0].team_name
    log = ''
    while True:
        log += army_desc(a1name, army1)
        log += army_desc(a2name, army2)
        if not (army1 and army2):
            break
        log += '\n'
        log += fight(army1, army2)
        log += '\n'
    return log

def simulate(state):
    green = state['Immune System']
    red = state['Infection']
    log = combat(green, red)
    answer = sum(group.units for group in red + green)
    return log, answer


# Parsing

group_re = re.compile(r'''(?x)
   (\d+) \s units \s each \s with \s 
   (\d+) \s hit \s points \s
   (?: \( ( [^)]* ) \) \s )?
   with \s an \s attack \s that \s does \s
   (\d+) \s (\w+) \s damage \s
   at \s initiative \s (\d+)
''')

def parse_etc(etc):
    weaknesses = set()
    immunities = set()
    if etc is not None:
        for section in etc.split(";"):
            kind, delim, types = section.strip().partition(' to ')
            types = set(types.split(", "))
            if kind == 'weak':
                weaknesses |= types
            else:
                assert kind == 'immune'
                immunities |= types
    return weaknesses, immunities

def parse_input(text):
    teams = {}
    for line in text.splitlines():
        line = line.strip()
        if line.endswith(':'):
            team_name = line.rstrip(':')
            teams[team_name] = []
        elif line:
            match = re.match(group_re, line)
            units = int(match.group(1))
            hp = int(match.group(2))
            weaknesses, immunities = parse_etc(match.group(3))
            attack_damage = int(match.group(4))
            attack_type = match.group(5)
            initiative = int(match.group(6))
            n = len(teams[team_name]) + 1
            teams[team_name].append(Group(team_name, n, units, hp, weaknesses, immunities, attack_damage, attack_type, initiative))
    return teams

def test():
    sample_input = textwrap.dedent('''\
    Immune System:
    17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2
    989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3

    Infection:
    801 units each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1
    4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4
    ''')

    sample_output = textwrap.dedent('''\
    Immune System:
    Group 1 contains 17 units
    Group 2 contains 989 units
    Infection:
    Group 1 contains 801 units
    Group 2 contains 4485 units

    Infection group 2 attacks defending group 2, killing 84 units
    Immune System group 2 attacks defending group 1, killing 4 units
    Immune System group 1 attacks defending group 2, killing 51 units
    Infection group 1 attacks defending group 1, killing 17 units

    Immune System:
    Group 2 contains 905 units
    Infection:
    Group 1 contains 797 units
    Group 2 contains 4434 units

    Immune System group 2 attacks defending group 1, killing 4 units
    Infection group 1 attacks defending group 2, killing 144 units

    Immune System:
    Group 2 contains 761 units
    Infection:
    Group 1 contains 793 units
    Group 2 contains 4434 units

    Immune System group 2 attacks defending group 1, killing 4 units
    Infection group 1 attacks defending group 2, killing 143 units

    Immune System:
    Group 2 contains 618 units
    Infection:
    Group 1 contains 789 units
    Group 2 contains 4434 units

    Immune System group 2 attacks defending group 1, killing 3 units
    Infection group 1 attacks defending group 2, killing 143 units

    Immune System:
    Group 2 contains 475 units
    Infection:
    Group 1 contains 786 units
    Group 2 contains 4434 units

    Immune System group 2 attacks defending group 1, killing 2 units
    Infection group 1 attacks defending group 2, killing 142 units

    Immune System:
    Group 2 contains 333 units
    Infection:
    Group 1 contains 784 units
    Group 2 contains 4434 units

    Immune System group 2 attacks defending group 1, killing 1 unit
    Infection group 1 attacks defending group 2, killing 142 units

    Immune System:
    Group 2 contains 191 units
    Infection:
    Group 1 contains 783 units
    Group 2 contains 4434 units

    Immune System group 2 attacks defending group 1, killing 1 unit
    Infection group 1 attacks defending group 2, killing 142 units

    Immune System:
    Group 2 contains 49 units
    Infection:
    Group 1 contains 782 units
    Group 2 contains 4434 units

    Immune System group 2 attacks defending group 1, killing 0 units
    Infection group 1 attacks defending group 2, killing 49 units

    Immune System:
    No groups remain.
    Infection:
    Group 1 contains 782 units
    Group 2 contains 4434 units
    ''').strip()

    state = parse_input(sample_input)
    log, answer = simulate(state)
    assert log.strip() == sample_output.strip()
    assert answer == 5216

test()


with open('puzzle-input.txt') as f:
    state = parse_input(f.read())

log, answer = simulate(state)
print(answer)
