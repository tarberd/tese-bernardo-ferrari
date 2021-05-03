SRC_FILES = thesis.tex

all: ufsc-thesis-rn46-2019.cls logo-ufsc.pdf $(SRC_FILES:.tex=.pdf)

%.pdf: %.tex
	latexmk -interaction=nonstopmode -shell-escape -pdf -use-make -cd $<

view: $(SRC_FILES) ufsc-thesis-rn46-2019.cls logo-ufsc.pdf
	latexmk -interaction=nonstopmode -shell-escape -pdf -use-make -cd -pvc thesis.tex

clean:
	latexmk -cd -CA $(SRC_FILES)
	cd "$(1)" && latexmk -cd -CA
	cd "$(1)" && rm -f *.dvi *.bbl *.nav *.ilg *.idx *.64 logo-ufsc.pdf ufsc-thesis-rn46-2019.cls
	cd "$(1)" && rm -fr auto _minted-*

ufsc-thesis-rn46-2019.cls:
	ln -s ufsc-thesis-rn46-2019/ufsc-thesis-rn46-2019.cls

logo-ufsc.pdf:
	ln -s ufsc-thesis-rn46-2019/logo-ufsc.pdf

