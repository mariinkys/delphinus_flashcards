package helpers

import (
	"bufio"
	"log"
	"os"
	"strings"
)

type dictionary struct {
	Entries []dictionaryEntry
}

type dictionaryEntry struct {
	Kanji      string
	Lecture    string
	Definition string
}

func RunCh(input string) ([]dictionaryEntry, []string) {
	parsedInput := parseChInput(input)
	dict := loadChDictionary()

	foundRes := searchChDictionary(dict, parsedInput)

	notFoundRes := notFoundInChDictionary(foundRes, parsedInput)
	var nfSlice []string
	for i := 0; i < len(notFoundRes); i++ {
		nfSlice = append(nfSlice, notFoundRes[i].Kanji)
	}

	return foundRes, nfSlice
}

func parseChInput(input string) []string {
	var charsArray []string
	if strings.Contains(input, ",") {
		charsArray = strings.Split(input, ",")
	} else if strings.Contains(input, "，") {
		charsArray = strings.Split(input, "，")
	} else if len(input) == 0 {
		log.Fatal("No hay caracteres!")
	} else {
		charsArray = append(charsArray, input)
	}
	return charsArray
}

func loadChDictionary() dictionary {
	dictionaryFile, err := os.Open("internal/dictionaries/ch/cedict_ts.u8")
	if err != nil {
		log.Fatal(err)
	}
	if err != nil {
		log.Fatal(err)
	}
	defer dictionaryFile.Close()

	var dict dictionary

	scanner := bufio.NewScanner(dictionaryFile)
	for scanner.Scan() {
		line := TrimLeftString(scanner.Text(), " ")

		lecture := TrimLeftString(line, "[")
		lecture = TrimRightString(lecture, "]")
		lecture = strings.TrimPrefix(lecture, "[")

		hanzi := TrimRightString(line, "[")

		definitions := TrimLeftString(line, "]")
		definitions = strings.TrimPrefix(definitions, "]")

		dict.Entries = append(dict.Entries, dictionaryEntry{Kanji: strings.TrimSpace(hanzi), Lecture: strings.TrimSpace(lecture), Definition: strings.TrimSpace(definitions)})
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	return dict
}

func searchChDictionary(dict dictionary, charsArray []string) []dictionaryEntry {
	var resArray []dictionaryEntry

	//For each entry in the dictionary
	for i := 0; i < len(dict.Entries); i++ {
		for j := 0; j < len(charsArray); j++ {
			if charsArray[j] == dict.Entries[i].Kanji {
				resArray = append(resArray, dict.Entries[i])
			}
		}
	}

	return resArray
}

func notFoundInChDictionary(resArray []dictionaryEntry, charsArray []string) []dictionaryEntry {
	var notFoundArray []dictionaryEntry

	if len(resArray) < len(charsArray) {
		for _, c := range charsArray {
			var found = false
			for _, r := range resArray {
				if c == r.Kanji {
					found = true
				}
			}
			if !found {
				notFoundArray = append(notFoundArray, dictionaryEntry{Kanji: c, Lecture: "", Definition: ""})
			}
		}
	}

	return notFoundArray
}
