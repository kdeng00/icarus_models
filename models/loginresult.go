package models

type LoginResult struct {
	Id         int    `json:"id"`
	Username   string `json:"username"`
	Token      string `json:"token"`
	TokenType  string `json:"token_type"`
	Expiration int    `json:"expiration"`
}
