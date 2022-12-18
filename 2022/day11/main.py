import sys
from pprint import pprint

f = sys.argv[1] if len(sys.argv) > 1  else "in"
lines = [line.strip() for line in open(f)]

pprint(lines)
