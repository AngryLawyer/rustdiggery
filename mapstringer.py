#!/usr/bin/env python3
import sys

output = []
for line in sys.stdin:
    output.append(line)

print(''.join(output).replace('\n', ''))
