ifdef TEST
	IMPORT_PARSE=sed -e 's/--test //'
else
	IMPORT_PARSE=sed -e 's/--real //'
endif

.PHONY: solution2
solution2: input.csv input-test.csv
	cat solution2.sql | $(IMPORT_PARSE) | sqlite3

.PHONY: solution1
solution1: input.csv input-test.csv
	cat solution1.sql | $(IMPORT_PARSE) | sqlite3

input.csv: input.txt
	cat input.txt | ./parse.awk > input.csv

input-test.csv: input-test.txt
	cat input-test.txt | ./parse.awk > input-test.csv

.PHONY: clean
clean:
	rm *.csv
