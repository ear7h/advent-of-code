Result
	= rows:Row* { return rows.filter(el => el).length }

Row
	= lo:Num "-" hi:Num _ letter:[a-z] ":" _ pass:[a-z]* [\n]? {
		return (pass[lo-1] == letter) != (pass[hi-1] == letter);
	}

Num
	= [0-9]+ { return parseInt(text(), 10); }

_
	= [ \t\n\r]*
