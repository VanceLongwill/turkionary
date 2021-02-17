# Turkionary

Next gen Turkish-English English-Turkish dictionary and learning tool

### TODO:

- TDK module
- Tureng module
- Separate executable crate for both
  + accept list of query words
  + store responses in db primary key as word
- separate tables for raw data from TDK & tureng to avoid battering their api

### Sources

#### TDK

- API

  - https://sozluk.gov.tr/gts?ara=abera
  - https://sozluk.gov.tr/bati?ara=abera
  - https://sozluk.gov.tr/tarama?ara=abera
  - https://sozluk.gov.tr/derleme?ara=abera
  - https://sozluk.gov.tr/atasozu?ara=abera
  - https://sozluk.gov.tr/kilavuz?prm=ysk&ara=abera
  - https://sozluk.gov.tr/terim?eser_ad=t%C3%BCm%C3%BC&ara=abera
  - https://sozluk.gov.tr/hemsirelik?ara=abera
  - https://sozluk.gov.tr/eczacilik?ara=abera
  - https://sozluk.gov.tr/metroloji?ara=abera

- Autocomplete JSON
  - https://sozluk.gov.tr/autocomplete.json

#### Tatoeba

- [https://tatoeba.org/eng/downloads](https://tatoeba.org/eng/downloads)

- Sentences
  - http://downloads.tatoeba.org/exports/sentences.tar.bz2
    > Fields and structure
    >
    > 1. Sentence id [tab] Lang [tab] Text
    > 2. Sentence id [tab] Lang [tab] Text [tab] Username [tab] Date added [tab] Date last modified
    >    File description
    >    Contains all the sentences. Each sentence is associated with a unique id and an ISO 639-3 language code.
- Links between sentences
  - http://downloads.tatoeba.org/exports/links.tar.bz2
    > Fields and structure
    > Sentence id [tab] Translation id
    > File description
    > Contains the links between the sentences. 1 [tab] 77 means that sentence #77 is the translation of sentence #1. The reciprocal link is also present, so the file will also contain a line that says 77 [tab] 1 .

## Usage

### DB Admin

- open a `psql` shell in the container (this doesn't require a local postgres installation)
  ```sh
  docker exec --tty --interactive \
  docker_db_1 psql \
  -h localhost -U postgres -d postgres
  ```
