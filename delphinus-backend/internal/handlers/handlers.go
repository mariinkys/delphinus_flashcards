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
		res := helpers.NewRunJap(characterString)
		w.WriteHeader(http.StatusCreated)

		//TMP
		for _, s := range res {
			flashcards = append(flashcards, models.Flashcard{Front: s.Kanji, Back: s.Lecture + "|" + s.Definition})
		}

		json.NewEncoder(w).Encode(flashcards)
	case "ch":
		res := helpers.NewRunCh(characterString)
		w.WriteHeader(http.StatusCreated)

		//TMP
		for _, s := range res {
			flashcards = append(flashcards, models.Flashcard{Front: s.Kanji, Back: s.Lecture + " | " + s.Definition})
		}

		json.NewEncoder(w).Encode(flashcards)
	default:
		w.WriteHeader(http.StatusBadRequest)
		json.NewEncoder(w).Encode("Error")
	}
}
