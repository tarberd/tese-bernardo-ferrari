SRC_FILES = article.tex

OUTDIR = build

LATEXMK_ARGS = -synctex=1\
	-file-line-error\
	-logfilewarninglist\
	-recorder\
	-interaction=nonstopmode\
	-shell-escape\
	-pdf\
	-outdir=$(OUTDIR)

all: $(SRC_FILES:.tex=.pdf)

%.pdf: %.tex
	latexmk $(LATEXMK_ARGS) $<

clean:
	rm -r $(OUTDIR)
