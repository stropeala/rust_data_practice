CREATE TABLE IF NOT EXISTS "Clients" (
    "id"                INTEGER,
    "surname"           TEXT,
    "name"              TEXT,
    "telephone_number"  TEXT UNIQUE,
    "city_of_residence" TEXT,
    "datetime_entrance" TEXT,
    "datetime_exit"     TEXT,
    "hours_parked"      INTEGER,
    "pariah"            INTEGER,
    PRIMARY KEY("id" AUTOINCREMENT)
) STRICT;
