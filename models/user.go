package models

import (
	"fmt"
	"time"
)

type User struct {
	Id            int    `json:"id"`
	Username      string `json:"username"`
	Password      string `json:"password"`
	Email         string `json:"email"`
	Phone         string `json:"phone"`
	Firstname     string `json:"firstname"`
	Lastname      string `json:"lastname"`
	EmailVerified bool   `json:"email_verified"`
	DateCreated   time   `json:"date_created"`
	Status        string `json:"status"`
	LastLogin     time   `json:"last_login"`
}
