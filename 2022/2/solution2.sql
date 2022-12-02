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
	('A', 'X', 0 + 3),
	('B', 'X', 0 + 1),
	('C', 'X', 0 + 2),

	('A', 'Y', 3 + 1),
	('B', 'Y', 3 + 2),
	('C', 'Y', 3 + 3),

	('A', 'Z', 6 + 2),
	('B', 'Z', 6 + 3),
	('C', 'Z', 6 + 1)
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

