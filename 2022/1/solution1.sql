create table input (
	elf int,
	calories int
);

--real .import input.csv      input --csv
--test .import input-test.csv input --csv

SELECT
	sum(i.calories)
FROM
	input i
GROUP BY
	i.elf
ORDER BY
	sum(i.calories) DESC
LIMIT 1
;

