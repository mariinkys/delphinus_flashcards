package main

import (
	"net/http"

	"github.com/go-chi/chi/v5"
	"github.com/go-chi/chi/v5/middleware"
	"github.com/mariinkys/delphinus_flashcards/internal/config"
	"github.com/mariinkys/delphinus_flashcards/internal/handlers"
)

func routes(app *config.AppConfig) http.Handler {
	mux := chi.NewRouter()

	mux.Use(middleware.Recoverer)
	mux.Use(SessionLoad)

	mux.Get("/", handlers.Repo.Home)
	mux.Get("/generator", handlers.Repo.Generator)
	mux.Post("/generator", handlers.Repo.PostGenerator)
	mux.Get("/result", handlers.Repo.Result)
	mux.Get("/faq", handlers.Repo.Faq)

	fileServer := http.FileServer(http.Dir("./static"))
	mux.Handle("/*", fileServer)

	return mux
}
