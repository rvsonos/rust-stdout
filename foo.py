#!/usr/bin/env python3
import sys
import random
import time

i = 1000
lines = open('file.txt','r').readlines()

while i != 0:
    print("this is a random string from file.txt")
    print(random.choice(lines), flush=True)
    # every 10th stdout, write something to stderr
    if i % 10 == 0:
        print("Error: fake error", file=sys.stderr, flush=True)
    time.sleep(0.1)
    i = i - 1
