#! /bin/sh

export TEXINPUTS=.:./themefau:

pdflatex -shell-escape presentation.tex
biber presentation
pdflatex -shell-escape presentation.tex

rm *.aux *.bbl *.bcf *.blg *.nav *.out *.snm *.log *.toc *.vrb
