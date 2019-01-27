SHELL = /bin/sh

targets = $(patsubst %.uml, bin/%.png, $(wildcard *.uml))
targets += $(patsubst %.dot, bin/%.png, $(wildcard *.dot))
targets += $(patsubst %.md, bin/%.pdf, $(wildcard *.md))

.PHONY: all
all: installdirs main

.PHONY: installdirs
installdirs: 
	@ mkdir -p bin 

.PHONY: clean
clean:
	rm -f bin/*

main: $(targets)

bin/%.pdf: %.md
	pandoc --to latex --from markdown_github+raw_tex+superscript -o $@ $<

bin/%.png: %.uml
	cat $< | docker run --rm -i think/plantuml -tpng > $@ 

bin/%.png: %.dot
	dot -Tpng -o $@ $<
