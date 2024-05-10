package main

import (
	"bufio"
	"encoding/xml"
	"fmt"
	"io"
	"log"
	"os"
	"regexp"
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

func main() {
	cleanOriginalXML("JMdict.xml")
	dictionary := loadJapDictionary()
	parsedSlice := searchJapDictionary(dictionary)
	writeNewDict(parsedSlice)
}

func loadJapDictionary() jMDict {
	xmlFile, err := os.Open("clean_jmdict.xml")
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

func searchJapDictionary(dict jMDict) []result {
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

		resArray = append(resArray, result{Kanji: kanji, Lecture: lecture, Definition: definition})
	}

	return resArray
}

func writeNewDict(res []result) {
	// Open a file for writing. Create it if it doesn't exist, truncate it if it does.
	file, err := os.OpenFile("new_jmdict.txt", os.O_WRONLY|os.O_CREATE|os.O_TRUNC, 0644)
	if err != nil {
		fmt.Println("Error opening file:", err)
		return
	}
	defer file.Close()

	// Create a buffered writer from the file.
	writer := bufio.NewWriter(file)

	//㷸 烨 [ye4] /variant of 燁|烨[ye4]/
	for _, line := range res {
		newString := line.Kanji + " [" + line.Lecture + "] " + line.Definition + "\n"
		if line.Kanji != "" {
			_, err := writer.WriteString(newString)
			if err != nil {
				fmt.Println("Error writing to file:", err)
				return
			}
		}
	}

	// Flush the buffer to ensure all data is written to the file.
	err = writer.Flush()
	if err != nil {
		fmt.Println("Error flushing buffer:", err)
		return
	}

	fmt.Println("Data written to file successfully.")
}

func cleanOriginalXML(path string) {
	file, err := os.Open(path)
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	tempFile, err := os.Create("clean_jmdict.xml")
	if err != nil {
		fmt.Println("Error creating clean_jmdict file:", err)
		return
	}
	defer tempFile.Close()

	pattern := `&[^&<>]*;`

	regex := regexp.MustCompile(pattern)

	scanner := bufio.NewScanner(file)
	// optionally, resize scanner's capacity for lines over 64K, see next example
	for scanner.Scan() {
		if !regex.MatchString(scanner.Text()) {
			_, err := tempFile.WriteString(scanner.Text() + "\n")
			if err != nil {
				fmt.Println("Error writing to clean_jmdict file:", err)
				return
			}
		}
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}
}
