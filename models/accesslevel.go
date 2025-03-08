package models

type AccessLevel struct {
	Id     int    `json:"id"`
	Level  string `json:"level"`
	SongId int    `json:"song_id"`
}
