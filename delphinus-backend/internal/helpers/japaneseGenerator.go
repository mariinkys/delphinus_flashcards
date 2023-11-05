package helpers

import (
	"encoding/xml"
	"io"
	"log"
	"os"
	"strings"
)

type jMDict struct {
	XMLName xml.Name `xml:"JMdict"`
	Entries []entry  `xml:"entry"`
}

type entry struct {
	XMLName             xml.Name              `xml:"entry"`
	KanjiContainer      []kanjiContainer      `xml:"k_ele"`
	LectureContainer    []lectureContainer    `xml:"r_ele"`
	DefinitionContainer []definitionContainer `xml:"sense"`
}

type kanjiContainer struct {
	XMLName xml.Name `xml:"k_ele"`
	Kanji   string   `xml:"keb"`
}

type lectureContainer struct {
	XMLName xml.Name `xml:"r_ele"`
	Lecture string   `xml:"reb"`
}

type definitionContainer struct {
	XMLName    xml.Name `xml:"sense"`
	Definition []string `xml:"gloss"`
}

type result struct {
	Kanji      string
	Lecture    string
	Definition string
}

func RunJap(input string) ([]result, []string) {
	parsedInput := parseJapInput(input)
	dict := loadJapDictionary()

	cleanInput := RemoveWhiteSpaces(parsedInput)

	foundRes := searchJapDictionary(dict, cleanInput)

	notFoundRes := notFoundInJapDictionary(foundRes, cleanInput)
	var nfSlice []string
	for i := 0; i < len(notFoundRes); i++ {
		nfSlice = append(nfSlice, notFoundRes[i].Kanji)
	}

	return foundRes, nfSlice
}

func parseJapInput(input string) []string {
	var charsArray []string
	if strings.Contains(input, ",") {
		charsArray = strings.Split(input, ",")
	} else if strings.Contains(input, "、") {
		charsArray = strings.Split(input, "、")
	} else if len(input) == 0 {
		//TODO: Handle error
		log.Fatal("No hay caracteres")
	} else {
		charsArray = append(charsArray, input)
	}
	return charsArray
}

func loadJapDictionary() jMDict {
	xmlFile, err := os.Open("internal/dictionaries/jap/JMdict.xml")
	if err != nil {
		log.Fatal(err)
	}
	defer xmlFile.Close()

	byteValue, err := io.ReadAll(xmlFile)
	if err != nil {
		log.Fatal(err)
	}

	var dict jMDict
	err = xml.Unmarshal(byteValue, &dict)
	if err != nil {
		log.Fatal(err)
	}
	return dict
}

func searchJapDictionary(dict jMDict, charsArray []string) []result {
	var resArray []result

	//For each entry in the dictionary
	for i := 0; i < len(dict.Entries); i++ {
		kanji, lecture, definition := "", "", ""

		//If the entry has a KanjiContainer (Containing a kanji)
		if len(dict.Entries[i].KanjiContainer) > 0 {
			kanji = dict.Entries[i].KanjiContainer[0].Kanji

			//If the Kanji has a lecture
			if len(dict.Entries[i].LectureContainer) > 0 {
				lecture = dict.Entries[i].LectureContainer[0].Lecture
			}

			//If the kanji has a definition
			if len(dict.Entries[i].DefinitionContainer) > 0 {
				//For each of the definitions of the kanji
				if len(dict.Entries[i].DefinitionContainer[0].Definition) > 0 {
					definition = ""
					for k, s := range dict.Entries[i].DefinitionContainer[0].Definition {
						definition += s
						if k != (len(dict.Entries[i].DefinitionContainer[0].Definition) - 1) {
							definition += " | "
						}
					}
				}
			}
		}

		for _, s := range charsArray {
			if kanji == s {
				resArray = append(resArray, result{Kanji: kanji, Lecture: lecture, Definition: definition})
			}
		}
	}

	return resArray
}

func notFoundInJapDictionary(resArray []result, charsArray []string) []result {
	var notFoundArray []result

	foundMap := make(map[string]bool)

	for _, r := range resArray {
		foundMap[r.Kanji] = true
	}

	for _, c := range charsArray {
		if !foundMap[c] {
			notFoundArray = append(notFoundArray, result{Kanji: c, Lecture: "", Definition: ""})
		}
	}

	return notFoundArray
}
