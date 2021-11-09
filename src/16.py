#!/usr/bin/env python3

def list_map(f, s):
    return list(map(f, s))

def addr(registers, a, b, c):
    result = registers[::]
    result[c] = result[a] + result[b]
    return result

def addi(registers, a, b, c):
    result = registers[::]
    result[c] = result[a] + b
    return result

def mulr(registers, a, b, c):
    result = registers[::]
    result[c] = result[a] * result[b]
    return result

def muli(registers, a, b, c):
    result = registers[::]
    result[c] = result[a] * b
    return result

def banr(registers, a, b, c):
    result = registers[::]
    result[c] = result[a] & result[b]
    return result

def bani(registers, a, b, c):
    result = registers[::]
    result[c] = result[a] & b
    return result

def borr(registers, a, b, c):
    result = registers[::]
    result[c] = result[a] | result[b]
    return result

def bori(registers, a, b, c):
    result = registers[::]
    result[c] = result[a] | b
    return result

def setr(registers, a, b, c):
    result = registers[::]
    result[c] = result[a]
    return result

def seti(registers, a, b, c):
    result = registers[::]
    result[c] = a
    return result

def gtir(registers, a, b, c):
    result = registers[::]
    result[c] = bool(a > result[b])
    return result

def gtri(registers, a, b, c):
    result = registers[::]
    result[c] = bool(result[a] > b)
    return result

def gtrr(registers, a, b, c):
    result = registers[::]
    result[c] = bool(result[a] > result[b])
    return result

def eqir(registers, a, b, c):
    result = registers[::]
    result[c] = bool(a == result[b])
    return result

def eqri(registers, a, b, c):
    result = registers[::]
    result[c] = bool(result[a] == b)
    return result

def eqrr(registers, a, b, c):
    result = registers[::]
    result[c] = bool(result[a] == result[b])
    return result

OPERATIONS = [
    addr, addi,
    mulr, muli,
    banr, bani,
    borr, bori,
    setr, seti,
    gtir, gtri, gtrr,
    eqir, eqri, eqrr
]

def possible_operations(instruction, before, after):
    result = set()
    for operation in OPERATIONS:
        op_result = operation(before, *instruction[1:])
        if op_result == after:
            result.add(operation)
    return result

def problem1(LINES):
    i = 0
    experiments = []
    while LINES[i].strip():
        before, instruction, after = LINES[i:i+3]
        i += 4
        experiments.append((
            list_map(int, instruction.split(' ')),
            eval(before[8:]),
            eval(after[8:])
        ))
    return len([experiment for experiment in experiments if len(possible_operations(*experiment)) >= 3])

def parse_input_file(fname):
    s = open(fname).read()
    if s and s[-1] == '\n':
        s = s[:-1]
    return s.splitlines()

def main():
    l = parse_input_file('data/day_16.txt')
    print(problem1(l))

if __name__ == '__main__':
    main()#!/usr/bin/env python3