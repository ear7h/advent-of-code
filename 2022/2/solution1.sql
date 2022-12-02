create table input (
	opp char,
	self char
);

--real .import input.csv      input --csv
--test .import input-test.csv input --csv

create table lut (
	opp char,
	self char,
	score int
);

insert into lut (opp, self, score)
values
	('A', 'X', 1 + 3),
	('B', 'X', 1 + 0),
	('C', 'X', 1 + 6),

	('A', 'Y', 2 + 6),
	('B', 'Y', 2 + 3),
	('C', 'Y', 2 + 0),

	('A', 'Z', 3 + 0),
	('B', 'Z', 3 + 6),
	('C', 'Z', 3 + 3)
;

SELECT
	sum(lut.score)
FROM
	input i
LEFT JOIN
	lut
ON
	i.opp  = lut.opp
AND
	i.self = lut.self
;

