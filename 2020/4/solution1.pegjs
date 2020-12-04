Docs
	= head:Doc tail:("\n\n" Doc)* "\n"* {
		return [head].concat(tail.map(a => a[1])).filter(v => v).length;
	}

Doc
	= head:Pair tail:([ \n] Pair)* {
    	const required = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];// "cid"];
    	const x = [head].concat(tail.map(a => a[1]));
        const valid = required.every(v => x.includes(v));
        return valid;
      }

Pair
	= key:([a-z]+) ":" val:([^ \n]+) { return key.join("") }

