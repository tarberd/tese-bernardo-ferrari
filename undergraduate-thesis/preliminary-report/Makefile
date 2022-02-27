SRC_FILES = thesis.tex

LATEXMK_ARGS = -xelatex \
	-synctex=1\
	-file-line-error\
	-interaction=nonstopmode\
	-shell-escape\
	-pdfxe\
	-outdir=build

all: $(SRC_FILES:.tex=.pdf)

%.pdf: %.tex
	latexmk $(LATEXMK_ARGS) $<

# view: $(SRC_FILES) ufsc-thesis-rn46-2019.cls logo-ufsc.pdf
# 	latexmk -interaction=nonstopmode -shell-escape -pdf -use-make -cd -pvc thesis.tex

# clean:
# 	latexmk -cd -CA $(SRC_FILES)
# 	cd "$(1)" && latexmk -cd -CA
# 	cd "$(1)" && rm -f *.dvi *.bbl *.nav *.ilg *.idx *.64 logo-ufsc.pdf ufsc-thesis-rn46-2019.cls
# 	cd "$(1)" && rm -fr auto _minted-*

# ufsc-thesis-rn46-2019.cls:
# 	ln -s ufsc-thesis-rn46-2019/ufsc-thesis-rn46-2019.cls

# logo-ufsc.pdf:
# 	ln -s ufsc-thesis-rn46-2019/logo-ufsc.pdf
