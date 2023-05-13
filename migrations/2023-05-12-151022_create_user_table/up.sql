-- Your SQL goes here
CREATE TABLE "users" (
	"id"	INTEGER NOT NULL UNIQUE,
	"name"	TEXT NOT NULL,
	"date_of_birth"	TEXT NOT NULL,
	"created_at"	TEXT NOT NULL,
	PRIMARY KEY("id" AUTOINCREMENT)
);