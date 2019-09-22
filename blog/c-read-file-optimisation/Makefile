TARGET=read_file
CC = gcc
CFLAGS = -g -Wall
TEST_DATA_DIR=test-data
.PHONY: default all clean

default: $(TARGET)
all: default

$(TARGET):
	$(CC) main.c $(CFLAGS) -o $@

benchmark: clean $(TARGET)
	./benchmark.sh $(TEST_DATA_DIR)

clean:
	rm -rf $(TEST_DATA_DIR)
	rm -f $(TARGET)