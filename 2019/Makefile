test: $(DAY).out
	cat $(shell echo $(DAY) |  head -c 1).txt | ./$(DAY).out


%.out : %.hs
	ghc $< -o $@


.PHONY: clean
clean:
	rm *.hi *.o *.out
