CREATE TABLE "new_tiles" (
	"id"	INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
	"title"	TEXT NOT NULL,
	"command"	TEXT NOT NULL,
	"day"	INTEGER NOT NULL,
	"start_time"	TEXT NOT NULL,
	"duration"	REAL NOT NULL,
	"color"	INTEGER NOT NULL
);

INSERT INTO "new_tiles" SELECT "id", "title", "mpv", "day", "time", "duration", "color" FROM "tiles";
DROP TABLE "tiles";
ALTER TABLE "new_tiles" RENAME TO "tiles"
