CC=gcc
CARGS=

run: build
	./oblivious

build:
	$(CC) $(CARGS) -o oblivious src/main.c src/level0.c

clean:
	rm ./oblivious
