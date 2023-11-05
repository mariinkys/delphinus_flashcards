package main

import (
	"net/http"

	"github.com/go-chi/chi/v5"
	"github.com/go-chi/chi/v5/middleware"
	"github.com/mariinkys/delphinus-backend/internal/config"
	"github.com/mariinkys/delphinus-backend/internal/handlers"
)

func routes(app *config.AppConfig) http.Handler {
	mux := chi.NewRouter()

	mux.Use(middleware.RealIP)
	mux.Use(middleware.Recoverer)
	mux.Use(middleware.Logger) //TODO: Temporal Logger
	mux.Use(SessionLoad)
	mux.Use(CorsMiddleware)

	mux.Post("/searchdictionary", handlers.Repo.SearchDictionary)
	mux.Post("/generateflashcards", handlers.Repo.GenerateFlashcards)

	fileServer := http.FileServer(http.Dir("./static"))
	mux.Handle("/*", fileServer)

	return mux
}
