package handlers

import (
	"encoding/json"
	"net/http"

	"github.com/mariinkys/delphinus-backend/internal/config"
	"github.com/mariinkys/delphinus-backend/internal/forms"
	"github.com/mariinkys/delphinus-backend/internal/helpers"
	"github.com/mariinkys/delphinus-backend/internal/models"
)

// Repo the repository used by the handlers
var Repo *Repository

// Repository is the repository type
type Repository struct {
	App *config.AppConfig
}

// NewRepo creates a new repository
func NewRepo(a *config.AppConfig) *Repository {
	return &Repository{
		App: a,
	}
}

// NewHandlers sets the repository for the handlers
func NewHandlers(r *Repository) {
	Repo = r
}

// SearchDictionary is the handler for the result page
func (m *Repository) SearchDictionary(w http.ResponseWriter, r *http.Request) {
	err := r.ParseForm()
	if err != nil {
		helpers.ServerError(w, err)
		return
	}

	characterString := r.Form.Get("characters")
	language := r.Form.Get("language")

	form := forms.New(r.PostForm)
	form.Required("characters", "language")

	if !form.Valid() {
		helpers.ClientError(w, http.StatusBadRequest)
		return
	}

	w.Header().Set("Content-Type", "application/json")
	var flashcards []models.Flashcard
	switch language {
	case "jp":
		res, notFound := helpers.RunJap(characterString)
		w.WriteHeader(http.StatusCreated)

		//TMP
		for _, s := range res {
			flashcards = append(flashcards, models.Flashcard{Front: s.Kanji, Back: s.Lecture + "|" + s.Definition})
		}

		if len(notFound) > 0 {
			for _, s := range notFound {
				flashcards = append(flashcards, models.Flashcard{Front: s, Back: "NOT FOUND"})
			}
		}

		json.NewEncoder(w).Encode(flashcards)
	case "ch":
		res, notFound := helpers.RunCh(characterString)
		w.WriteHeader(http.StatusCreated)

		//TMP
		for _, s := range res {
			flashcards = append(flashcards, models.Flashcard{Front: s.Kanji, Back: s.Lecture + " | " + s.Definition})
		}

		if len(notFound) > 0 {
			for _, s := range notFound {
				flashcards = append(flashcards, models.Flashcard{Front: s, Back: "NOT FOUND"})
			}
		}

		json.NewEncoder(w).Encode(flashcards)
	default:
		w.WriteHeader(http.StatusBadRequest)
		json.NewEncoder(w).Encode("Error")
	}
}

// GenerateFlashcards is the handler for the GenerateFlashcards endpoint
func (m *Repository) GenerateFlashcards(w http.ResponseWriter, r *http.Request) {
	w.Header().Set("Content-Type", "application/json")
	var flashcards []models.Flashcard

	decoder := json.NewDecoder(r.Body)
	if err := decoder.Decode(&flashcards); err != nil {
		helpers.ServerError(w, err)
		return
	}

	var res = helpers.GenerateFlashcardsOutput(flashcards)
	json.NewEncoder(w).Encode(res)
}
