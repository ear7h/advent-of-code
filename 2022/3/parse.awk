#!/usr/bin/env -S awk -f

{
	mid = length($0) / 2

	for (i = 1; i <= length($0); i += 1) {
		print NR "," (i > mid) "," substr($0, i, 1)
	}
}

