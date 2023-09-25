BUILDDIR := build
CC ?= clang
CFLAGS := -Wall -Wextra
CFILES := $(wildcard *.c)
OFILES := $(patsubst %.c,$(BUILDDIR)/%.o,$(CFILES))

$(info $(CFILES))
$(info $(OFILES))

$(shell mkdir -p $(BUILDDIR))

seL4-sample-converter: $(OFILES)
	$(CC) $^ -o $(BUILDDIR)/seL4-sample-converter

$(OFILES): $(CFILES)
	$(CC) $(CFLAGS) -c $< -o $@

clean:
	rm -r $(BUILDDIR)