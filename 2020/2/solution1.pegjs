Result
	= rows:Row* { return rows.filter(el => el).length }

Row
	= lo:Num "-" hi:Num _ letter:[a-z] ":" _ pass:[a-z]* [\n]? {
		const n = pass.reduce((acc, el) => el == letter ? acc + 1 : acc, 0);
		return n >= lo && n <= hi
	}

Num
	= [0-9]+ { return parseInt(text(), 10); }

_
	= [ \t\n\r]*
