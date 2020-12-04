Docs
	= head:Doc tail:("\n\n" Doc)* "\n"* {
		return [head].concat(tail.map(a => a[1])).filter(v => v).length;
	}

Doc
	= head:Pair tail:([ \n] Pair)* {
    	const required = {
			"byr": v => v.length == 4 && Number(v) >= 1920 && Number(v) <= 2002,
			"iyr": v => v.length == 4 && Number(v) >= 2010 && Number(v) <= 2020,
			"eyr": v => v.length == 4 && Number(v) >= 2020 && Number(v) <= 2030,
			"hgt": v => {
				if (v.endsWith("cm")) {
					let x = Number(v.slice(0, -2));
					return x >= 150 && x <= 193;
				} else if (v.endsWith("in")) {
					let x = Number(v.slice(0, -2));
					return x >= 59 && x <= 76;
				} else {
					return false;
				}
			},
			"hcl": v => /^#[a-f0-9]{6}$/.test(v),
			"ecl": v => {
				return [
					"amb", "blu", "brn", "gry", "grn", "hzl", "oth"
				].includes(v);
			},
			"pid": v => /^[0-9]{9}$/.test(v),
			"cid": v => true,
		};

    	const x = [head].concat(tail.map(a => a[1]));/*.filter(kv => {
			let x = required[kv[0]](kv[1]);
			return x;
		});*/

		for (let key in required) {
			if (key == "cid") {continue;}
			const val = x.filter(kv => kv[0] == key)[0];
			if (!val || !required[key](val[1])) {
				return false;
			}
		}
        return true;
      }

Pair
	= key:([a-z]+) ":" value:([^ \n]+) { return [key.join(""), value.join("")] }

