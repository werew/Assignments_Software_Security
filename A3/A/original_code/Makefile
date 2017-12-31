CFLAGS=-std=c99 -Wall -Werror -pedantic -Wextra -g -O0

OBJ=sortedcontainer.o \
    main.o \
	test.o

OBJ_AFL_64=$(patsubst %.o, %_afl_64.o, $(OBJ))
OBJ_AFL_32_ASAN=$(patsubst %.o, %_afl_32_asan.o, $(OBJ))

OBJOK=../../teacher/assignment1/sortedcontainer_ok.o \
    ../../teacher/assignment1/main_ok.o \
	test.o

CC=gcc

SRC=$(OBJ:.o=.c)

all: sint

clean:
	rm -f sint
	rm -f sint_afl
	rm -f sint_afl_asan
	rm -f *.o
	rm -f .depend

depend: .depend

.depend: $(SRC)
	rm -f ./.depend
	$(CC) $(CFLAGS) -MM $^ > ./.depend;

include .depend

$(OBJ): %.o : %.c
	$(CC) -c $(CFLAGS) $< -o $@

$(OBJ_AFL_64): %_afl_64.o : %.c
	afl-gcc -c $(CFLAGS) $< -o $@

$(OBJ_AFL_32_ASAN): %_afl_32_asan.o : %.c
	afl-gcc -c -m32 -fsanitize=address $(CFLAGS) $< -o $@



sint: $(OBJ)
	$(CC) $^ $(CFLAGS) -o $@ -lm

ok: $(OBJOK)
	$(CC) $^ $(CFLAGS) -o sint_ok -lm

sint_afl: $(OBJ_AFL_64)
	afl-gcc $^ -o sint_afl -lm

sint_afl_asan: $(OBJ_AFL_32_ASAN)
	afl-gcc $^ -m32 -fsanitize=address -o sint_afl_asan -lm


fuzz: sint_afl
	afl-fuzz -i ../fuzzing/input/ -o output/ -- ./sint_afl

fuzz_asan: sint_afl_asan
	afl-fuzz -i ../fuzzing/input/ -o output/ -m800 -- ./sint_afl_asan

fuzz_valgrind: sint_afl
	afl-fuzz -i ../fuzzing/input/ -o output/ -n -- valgrind.bin ./sint_afl
