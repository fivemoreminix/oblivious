CC=gcc
CARGS=

run: build
	./oblivious

build:
	$(CC) $(CARGS) -o oblivious src/main.c

clean:
	rm ./oblivious
