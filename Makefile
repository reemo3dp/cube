OPENSCAD ?= openscad

.PHONY: all clean
ROOT_DIR := $(dir $(realpath $(lastword $(MAKEFILE_LIST))))


all: clean $(patsubst cubes/%.scad, output/%.3mf, $(wildcard cubes/*.scad)) \

clean:
	rm -rf output/
	mkdir output/

output/%.3mf: cubes/%.scad
	$(eval TEMP_FILE:=$(shell mktemp --suffix=.scad))
	echo "include<$(realpath $<)>;" > $(TEMP_FILE)
	echo "include<$(ROOT_DIR)/main.scad>;" >> $(TEMP_FILE)
	cp $< $(patsubst %.3mf,%.scad,$@)
	$(OPENSCAD) \
		--enable lazy-union \
		--backend Manifold \
		-o $@ \
		-o $(patsubst %.3mf,%.png,$@) \
		--render \
		--debug all \
		$(TEMP_FILE)
