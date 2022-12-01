create table input (
	elf int,
	calories int
);

--real .import input.csv      input --csv
--test .import input-test.csv input --csv

WITH top3 AS (
	SELECT
		sum(i.calories) AS calories
	FROM
		input i
	GROUP BY
		i.elf
	ORDER BY
		sum(i.calories) DESC
	LIMIT 3
)
SELECT sum(t3.calories)
FROM top3 t3;
;

