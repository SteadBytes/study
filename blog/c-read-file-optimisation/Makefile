TARGET=read_file
CC = gcc
CFLAGS = -g -Wall

.PHONY: default all clean

default: $(TARGET)
all: default

$(TARGET):
	$(CC) main.c $(CFLAGS) -o $@

clean:
	-rm -f $(TARGET)