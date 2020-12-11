#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <stdbool.h>
#include <assert.h>

#define IS_TAKEN(x) ('#' == (x))
#define IS_EMPTY(x) ('L' == (x))

size_t los(char * grid, size_t rows, size_t cols, size_t row, size_t col) {
	size_t taken = 0;

	for (size_t i = row; i > 0; i--) {
		if (IS_EMPTY(grid[(i-1) * cols + col])) {
			break;
		}

		if (IS_TAKEN(grid[(i-1) * cols + col])) {
			taken++;
			break;
		}
	}

	for (size_t i = row+1; i < rows; i++) {
		if (IS_EMPTY(grid[i * cols + col])) {
			break;
		}

		if (IS_TAKEN(grid[i * cols + col])) {
			taken++;
			break;
		}
	}

	for (size_t i = col; i > 0; i--) {
		if (IS_EMPTY(grid[row * cols + i - 1])) {
			break;
		}

		if (IS_TAKEN(grid[row * cols + i - 1])) {
			taken++;
			break;
		}
	}

	for (size_t i = col+1; i < cols; i++) {
		if (IS_EMPTY(grid[row * cols + i])) {
			break;
		}

		if (IS_TAKEN(grid[row * cols + i])) {
			taken++;
			break;
		}
	}

	size_t r = row;
	size_t c = col;

	while (r < rows-1 && c < cols-1) {
		if (IS_EMPTY(grid[++r * cols + ++c])) {
			break;
		}

		if (IS_TAKEN(grid[r * cols + c])) {
			taken++;
			break;
		}
	}

	r = row;
	c = col;
	while (r > 0 && c > 0) {
		if (IS_EMPTY(grid[--r * cols + --c])) {
			break;
		}

		if (IS_TAKEN(grid[r * cols + c])) {
			taken++;
			break;
		}
	}

	r = row;
	c = col;
	while (r < rows-1 && c > 0) {
		if (IS_EMPTY(grid[++r * cols + --c])) {
			break;
		}

		if (IS_TAKEN(grid[r * cols + c])) {
			taken++;
			break;
		}
	}

	r = row;
	c = col;
	while (r > 0 && c < cols-1) {
		if (IS_EMPTY(grid[--r * cols + ++c])) {
			break;
		}

		if (IS_TAKEN(grid[r * cols + c])) {
			taken++;
			break;
		}
	}

	return taken;
}

size_t step(char * dst, char * grid, size_t rows, size_t cols) {
	size_t changed = 0;

	for (size_t i = 1; i < rows - 1; i++) {
		for (size_t j = 1; j < cols - 1; j++) {
			size_t idx = i * cols + j;
			uint8_t ch = grid[idx];
			switch (ch) {
			case 'L':
				if (!los(grid, rows, cols, i, j)) {
					ch = '#';
					changed++;
				}
				break;
			case '#':
				if (los(grid, rows, cols, i, j) >= 5) {
					ch = 'L';
					changed++;
				}
				break;
			}

			dst[idx] = ch;
		}
	}

	return changed;
}

int main() {
	char * line = NULL;
	size_t siz = 0;

	size_t rows  = 0;
	size_t cols  = 0;
	char * grid = NULL;
	char * grid1 = NULL;

	while (getline(&line, &siz, stdin) >= 0) {
		if (!cols) {
			char * cols1 = line;
			while (*cols1++) {}
			cols = cols1-line+1;
			rows++;
			grid = realloc(grid, rows * cols);
			for (size_t i = 0; i < cols; i++) {
				grid[i] = '.';
			}
			grid[cols-1] = '\0';
		}
		rows++;

		grid = realloc(grid, rows * cols);
		assert(grid);

		strcpy(&grid[(rows-1) * cols + 1], line);
		grid[(rows-1) * cols] = '.';
		grid[rows * cols - 2] = '.';
		grid[rows * cols - 1] = '\0';
	}

	rows++;
	grid = realloc(grid, rows * cols);
	assert(grid);
	for (size_t i = 0; i < cols; i++) {
		grid[(rows-1) * cols + i] = '.';
	}
	grid[rows * cols - 1] = '\0';


	grid1 = malloc(rows * cols);
	assert(grid1);
	for (size_t i = 0; i < cols; i++) {
		grid1[i] = '.';
		grid1[(rows-1) * cols + i] = '.';
	}

	for (size_t i = 0; i < rows; i++) {
		grid1[i * cols] = '.';
		grid1[(i+1) * cols - 2] = '.';
		grid1[(i+1) * cols - 1] = '\0';
	}


	size_t i = 0;
	while(step(grid1, grid, rows, cols)) {
		char * tmp = grid;
		grid = grid1;
		grid1 = tmp;
		i++;
	}

	size_t count = 0;
	for (size_t i = 0; i < rows * cols; i++) {
		if (grid[i] == '#') {
			count++;
		}
	}
	printf("%ld\n", count);

	return 0;
}
