NAME = picipaint
SRC_DIR = src
BIN_DIR = bin
CFLAGS = -Wall -Wextra


$(BIN_DIR)/$(NAME): $(wildcard $(SRC_DIR)/*.c) | $(BIN_DIR)
	$(CC) $? $(CFLAGS) -o $(BIN_DIR)/$(NAME)

$(BIN_DIR):
	mkdir $(BIN_DIR)


.PHONY: run clean

run: $(BIN_DIR)/$(NAME)
	./$(BIN_DIR)/$(NAME)

clear:
	rm -rf $(BIN_DIR)
