#include <stdio.h>
#include <stdbool.h>


int main() {
	char * line = NULL;
	size_t siz = 0;
	int sum = 0;
	int answers[26] = {0};
	int group_size = 0;
	while (getline(&line, &siz, stdin) >= 0) {
		if (*line == '\n') {
			int group_sum = 0;
			for (size_t i = 0; i < 26; i++) {
				if (answers[i] == group_size) {
					group_sum++;
				}
				answers[i] = 0;
			}
			group_size = 0;
			sum += group_sum;
		} else {
			group_size++;
			for (size_t i = 0; line[i] != '\n'; i++) {
				answers[line[i] - 'a']++;
			}
		}
	}

	{
		int group_sum = 0;
		for (size_t i = 0; i < 26; i++) {
			if (answers[i] == group_size) {
				group_sum++;
			}
			answers[i] = 0;
		}
		group_size = 0;
		sum += group_sum;
	}

	printf("%d\n", sum);

	return 0;
}
