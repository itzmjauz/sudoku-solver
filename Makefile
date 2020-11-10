sudoku-solve: sudoku-solve.c
	gcc $(CFLAGS) -o sudoku-solve sudoku-solve.c -lm

clean:
	rm -f *.o sudoku-solve *~ *core
