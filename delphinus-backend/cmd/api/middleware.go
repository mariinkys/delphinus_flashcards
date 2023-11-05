package main

import (
	"net/http"

	"github.com/go-chi/cors"
)

// SessionLoad loads and saves session data for current request
func SessionLoad(next http.Handler) http.Handler {
	return session.LoadAndSave(next)
}

// CorsMiddleware handlers cors for the api
func CorsMiddleware(next http.Handler) http.Handler {
	corsMiddleware := cors.New(cors.Options{
		AllowedOrigins:   []string{"http://localhost:5173", "https://delphinus.mariinkys.dev/", "http://delphinus.mariinkys.dev/", "104.248.241.221"},
		AllowedMethods:   []string{"GET", "POST", "PUT", "DELETE", "OPTIONS", "PATCH", "DELETE"},
		AllowedHeaders:   []string{"Accept", "Authorization", "Content-Type", "X-CSRF-Token", "X-Requested-With", "Origin"},
		AllowCredentials: true,
		MaxAge:           300,
	})
	return corsMiddleware.Handler(next)
}
