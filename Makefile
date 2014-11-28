linenoise_example: linenoise.h linenoise.c

linenoise_example: linenoise.c
	$(CC) -Wall -W -Os -g -c linenoise.c

clean:
	rm -f linenoise_example
