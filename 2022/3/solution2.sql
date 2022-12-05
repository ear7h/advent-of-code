create table input (
	ruck int,
	comp int,
	item char
);

--real .import input.csv      input --csv
--test .import input-test.csv input --csv

WITH groups AS (
	-- group id
	SELECT
		(ruck - 1) / 3 AS gid,
		(ruck - 1) % 3 AS gel,
		item
	FROM
		input
), badges AS (
	SELECT
		-- g1.item AS item

		CASE WHEN unicode(g1.item) >= unicode('a')
		THEN unicode(g1.item) - unicode('a') + 1
		ELSE unicode(g1.item) - unicode('A') + 27
		END AS vals
	FROM
		groups g1,
		groups g2,
		groups g3
	WHERE
		g1.gid = g2.gid
	AND
		g2.gid = g3.gid
	AND
		g1.gel = 0
	AND
		g2.gel = 1
	AND
		g3.gel = 2
	AND
		g1.item = g2.item
	AND
		g2.item = g3.item
	GROUP BY g1.gid
)
SELECT sum(vals)
FROM badges
;

