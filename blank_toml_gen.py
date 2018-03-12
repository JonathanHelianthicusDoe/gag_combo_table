#!/usr/bin/env python3

GAGS = ["throw", "squirt", "drop", "sound"]

s = ""
for lvl in range(1, 17):
    v2 = False
    if lvl > 12:
        v2 = True
        lvl -= 4
    for lured in [False, True]:
        s += "["
        head = "level_"
        head += str(lvl)
        if v2:
            head += "_v2"
        if lured:
            head += "_lured"
        s += head
        s += "]\n"
        for org in [False, True]:
            s += "  ["
            s += head
            s += "."
            if not org:
                s += "non"
            s += "org]\n"
            for gagtype in GAGS:
                s += "    "
                s += gagtype.ljust(max(len(g) for g in GAGS))
                s += " = "
                s += str([[0] * tooncount for tooncount in range(1, 5)])
                s += "\n"

with open("combos.toml", "w", encoding="utf8") as f:
    f.write(s)
