create table input (
	ruck int,
	comp int,
	item char
);

--real .import input.csv      input --csv
--test .import input-test.csv input --csv

WITH i1 AS (
	SELECT
		i.ruck,
		i.item
	FROM input i
	WHERE
		i.comp = 0
	GROUP BY i.ruck, i.item
), vals AS (
	SELECT
		-- i1.item,

		CASE WHEN unicode(i1.item) >= unicode('a')
		THEN unicode(i1.item) - unicode('a') + 1
		ELSE unicode(i1.item) - unicode('A') + 27
		END AS vals
	FROM
		i1
	WHERE EXISTS (
		SELECT *
		FROM input i2
		WHERE
			i1.ruck = i2.ruck
		AND
			i1.item = i2.item
		AND
			i2.comp = 1
	)
)
SELECT sum(vals)
FROM vals
;

