-- Your SQL goes here
CREATE TABLE "habit" (
	"id"	INTEGER NOT NULL UNIQUE,
	"user_id"	INTEGER NOT NULL,
	"name"	TEXT NOT NULL,
	"description"	TEXT,
	"categories"	TEXT,
	"icon"	TEXT,
	"frequency"	TEXT NOT NULL,
	"created_at"	TEXT NOT NULL,
	"updated_at"	TEXT,
	"reminder_times"	TEXT,
	"note"	TEXT,
	"archived"	INTEGER NOT NULL,
	"archived_date"	TEXT,
	"archived_reason"	TEXT,
	PRIMARY KEY("id" AUTOINCREMENT)
    FOREIGN KEY("user_id") REFERENCES "user"("id")
);