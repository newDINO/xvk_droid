CC = clang

CFLAGS = -Wall -Werror
TARGET_DIR = target

SYM_LINKS = libvulkan.so libvulkan.so.1

create_link:
	ln -s target/release/libvulkan.so libvulkan.so
	ln -s target/release/libvulkan.so libvulkan.so.1

TEST_SRC = c_test/main.c

test: $(TEST_SRC)
	mkdir -p $(TARGET_DIR)
	$(CC) $(TEST_SRC) $(CFLAGS) -lvulkan -lglfw -o $(TARGET_DIR)/test_bin

run_test: $(SYM_LINKS)
	LD_LIBRARY_PATH=$(PWD):$(LD_LIBRARY_PATH) ./$(TARGET_DIR)/test_bin

vkmark:
	LD_LIBRARY_PATH=$(PWD):$(LD_LIBRARY_PATH) vkmark

vkcube:
	LD_LIBRARY_PATH=$(PWD):$(LD_LIBRARY_PATH) vkcube

SAMPLE_SRC = c_test/sample.c

sample: $(SMAPLE_SRC)
	mkdir -p $(TARGET_DIR)
	$(CC) $(SAMPLE_SRC) $(CFLAGS) -lvulkan -lglfw -o $(TARGET_DIR)/sample

run_sample: $(SYM_LINKS)
	LD_LIBRARY_PATH=$(PWD):$(LD_LIBRARY_PATH) ./$(TARGET_DIR)/sample

XV_SRC = c_test/xv.c

xv: $(XV_SRC)
	mkdir -p $(TARGET_DIR)
	$(CC) $(XV_SRC) $(CFLAGS) -lvulkan -lxcb -o $(TARGET_DIR)/xv

run_xv: $(SYM_LINKS)
	LD_LIBRARY_PATH=$(PWD):$(LD_LIBRARY_PATH) ./$(TARGET_DIR)/xv