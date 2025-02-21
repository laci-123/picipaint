NAME		?= picipaint
SRC_DIR		?= ./src
BIN_DIR		?= ./bin
CC			?= gcc
CFLAGS	     = -Wall -Wextra -std=c2x -pedantic
LINKFLAGS	 = -lm
RAYLIB	     = ./raylib/src
NFD          = ./nativefiledialog-extended
NFD_FLAGS    = -DCMAKE_BUILD_TYPE=Release -DNFD_BUILD_TESTS=OFF
INCLUDE      = -I./raylib/src -I./nativefiledialog-extended/src/include
C_FILES      = $(wildcard $(SRC_DIR)/*.c)
H_FILES      = $(wildcard $(SRC_DIR)/*.h)

ifeq ($(OS),Windows_NT)
	NAME := $(NAME)-windows-x86-64.exe
	 #raylib
	LINKFLAGS += -lgdi32 -lwinmm
	 #nfd
	LINKFLAGS += -lole32 -luuid -lshell32
	 #use Windows subsystem
	LINKFLAGS += -mwindows
	 #CMake should use MinGW instead of default Windows toolchain
	NFD_FLAGS += -G "MinGW Makefiles"
else
	NAME := $(NAME)-linux-x86-64
	 #raylib
	LINKFLAGS += `pkg-config --libs gtk+-3.0`
endif


$(BIN_DIR)/$(NAME): $(C_FILES) $(H_FILES) | $(RAYLIB)/libraylib.a $(NFD)/build/src/libnfd.a $(BIN_DIR)
	$(CC) $(C_FILES) $(RAYLIB)/libraylib.a $(NFD)/build/src/libnfd.a $(CFLAGS) $(INCLUDE) $(LINKFLAGS) -o $(BIN_DIR)/$(NAME)

$(RAYLIB)/libraylib.a: 
	cd $(RAYLIB) && $(MAKE)

$(NFD)/build/src/libnfd.a:
	cd $(NFD) && \
	mkdir build && \
	cd build && \
	cmake $(NFD_FLAGS) .. && \
	cmake --build .

$(BIN_DIR):
	mkdir -p $(BIN_DIR)


.PHONY: run clean clean_all

run: $(BIN_DIR)/$(NAME)
	./$(BIN_DIR)/$(NAME)

clean:
	rm -rf $(BIN_DIR)

clean_all: clean
	cd $(RAYLIB) && $(MAKE) clean
	rm -rf $(NFD)/build
