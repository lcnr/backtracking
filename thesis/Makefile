TEX = thesis.tex

DVI = $(TEX:%.tex=%.dvi)
PS  = $(TEX:%.tex=%.ps)
PDF = $(TEX:%.tex=%.pdf)

all: *.tex
	pdflatex -shell-escape thesis.tex

.PHONY: clean all
clean:
	@echo " ** Deletion of all backup and automatically generated files.";\
	rm -f *.out *.bbl *.blg *.tpt *.toc *.idx *.log *.aux *.backup *~ $(DVI) $(PS) $(PDF);
