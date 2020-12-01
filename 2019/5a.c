// switched to c bc haskell is not the language for this kinda thing
#include <stdlib.h>
#include <assert.h>

#define OP_ADD 1
#define OP_MUL 2
#define OP_MAX 3
#define OP_HLT 99


#define MEM_REL(S,O) (S->mem[S->pc+O])
#define MEM_ABS(S,I) (S->mem[I])

typedef struct interp {
	unsigned int pc;
	unsigned int len;
	int mem[];
} interp;

void opAdd(interp * state) {
	int src1 = MEM_REL(state, 1);
	int src2 = MEM_REL(state, 2);
	int dst = MEM_REL(state, 3);
	int x = MEM_ABS(state, MEM_REL(state, 1));
	int y = MEM_ABS(state, MEM_REL(state, 2));
	*(&MEM_ABS(state, MEM_REL(state, 3))) = x + y;
}

void opMul(interp * state) {
	int src1 = MEM_REL(state, 1);
	int src2 = MEM_REL(state, 2);
	int dst = MEM_REL(state, 3);
	int x = MEM_ABS(state, MEM_REL(state, 1));
	int y = MEM_ABS(state, MEM_REL(state, 2));
	*(&MEM_ABS(state, MEM_REL(state, 3))) = x * y;
}

void (*opFns[])(interp *) = {
	&opAdd,
	&opMul,
};

void interp_start(interp * state) {
	while (1) {
		unsigned int pc = state->pc;
		assert(pc < state->len);
		int op = state->mem[pc];
		if (op == 99) {
			return;
		}

		assert(op && op < OP_MAX);

		opFns[op-1](state);
	}
}

int main() {
	return 0;
}
