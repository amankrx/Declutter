-- Your SQL goes here
CREATE TABLE "habit_entry" (
	"id"	INTEGER NOT NULL UNIQUE,
	"user_id"	INTEGER NOT NULL,
	"habit_id"	INTEGER NOT NULL,
	"datetime"	TEXT NOT NULL,
	"note"	TEXT,
	"value"	INTEGER NOT NULL,
	PRIMARY KEY("id" AUTOINCREMENT)
    FOREIGN KEY("user_id") REFERENCES "user"("id")
    FOREIGN KEY("habit_id") REFERENCES "habit"("id")
);