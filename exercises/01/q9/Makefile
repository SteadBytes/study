CC = /usr/bin/gcc 
DEPS =

all: clean replace

replace:
	$(CC) $(DEBUG) -o replace.out replace.c $(DEPS) 

debug: DEBUG = -g
debug: clean replace

clean:
	rm -f *out