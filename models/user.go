package models

import "time"

type User struct {
	Id            int       `json:"id"`
	Username      string    `json:"username"`
	Password      string    `json:"password"`
	Email         string    `json:"email"`
	Phone         string    `json:"phone"`
	Firstname     string    `json:"firstname"`
	Lastname      string    `json:"lastname"`
	EmailVerified bool      `json:"email_verified"`
	DateCreated   time.Time `json:"date_created"`
	Status        string    `json:"status"`
	LastLogin     time.Time `json:"last_login"`
}
