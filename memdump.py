#!/usr/bin/python3

import os
import re
import sys


def find_the_secret(pid: int):
    map_file = f"/proc/{pid}/maps"
    mem_file = f"/proc/{pid}/mem"

    if not (os.path.exists(map_file) and os.path.exists(mem_file)):
        print("The PID value of {} is incorrect, exiting.".format(pid), file=sys.stderr)
        sys.exit(1)

    with open(map_file, 'r') as map_f, open(mem_file, 'rb', 0) as mem_f:
        uuid_regex = re.compile(b'([0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12})', re.I)
        for line in map_f.readlines():
            m = re.match(r'([0-9A-Fa-f]+)-([0-9A-Fa-f]+) ([-r])', line)
            start = int(m.group(1), 16)
            end = int(m.group(2), 16)
            try:
                mem_f.seek(start)  # seek to region start
                chunk = mem_f.read(end - start)  # read region contents
                found = uuid_regex.findall(chunk)
                if found:
                    print("UUID found at memory range {}:{}:".format(hex(start), hex(end)))
                    for uuid in found:
                        print("\t{}".format(uuid.decode("utf-8")))
            except Exception:
                print(hex(start), '-', hex(end), '[error,skipped]', file=sys.stderr)
                continue


if __name__ == '__main__':
    if len(sys.argv) != 2:
        print("Usage: {} PID".format(sys.argv[0]))
        sys.exit(1)

    try:
        pid = int(sys.argv[1])
        find_the_secret(pid)
    except ValueError:
        print("Invalid pid: {}".format(sys.argv[1]))
        sys.exit(1)

# vim: tabstop=8 expandtab shiftwidth=4 softtabstop=4
