#!/usr/bin/env -S awk -f

BEGIN {
	elf = 1
	print "elf,calories"
}
/^$/ {
	elf += 1
}
/./ {
	print elf "," $0
}

