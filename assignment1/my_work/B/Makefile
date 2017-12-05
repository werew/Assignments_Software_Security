CFLAGS=-std=c99 -Wall -Werror -pedantic -Wextra -g -O0
OBJ=sortedcontainer.o \
    main.o \
	test.o
OBJOK=../../teacher/assignment1/sortedcontainer_ok.o \
    ../../teacher/assignment1/main_ok.o \
	test.o

CC=gcc

SRC=$(OBJ:.o=.c)

all: sint

clean:
	rm -f sint
	rm -f sint_afl
	rm -f *.o
	rm -f .depend

depend: .depend

.depend: $(SRC)
	rm -f ./.depend
	$(CC) $(CFLAGS) -MM $^ > ./.depend;

include .depend

%.o: %.c
	$(CC) -c $(CFLAGS) $< -o $@

sint: $(OBJ)
	$(CC) $^ $(CFLAGS) -o $@ -lm

ok: $(OBJOK)
	$(CC) $^ $(CFLAGS) -o sint_ok -lm

sint_afl: $(OBJ)
	afl-gcc $^ $(CFLAGS) -o sint_afl -lm
