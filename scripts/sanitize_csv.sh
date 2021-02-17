#!/bin/bash

sed -e $'s/"/""/g;s/[^\t]*/"&"/g' data/tatoeba_sentences.csv > data/tatoeba_sentences.csv.tmp && mv data/tatoeba_sentences.csv.tmp data/tatoeba_sentences.csv
