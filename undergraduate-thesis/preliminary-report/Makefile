all:
	cp ufsc-thesis-rn46-2019/logo-ufsc.pdf target/pdf/main-logo.pdf
	latexmk -outdir=target/pdf -bibtex -pdf -latexoption=-shell-escape -cd main.tex

view:
	latexmk -outdir=target/pdf -bibtex -pdf -pvc -latexoption=-shell-escape main.tex

clean:
	latexmk -CA main.tex

