CREATE TABLE "Clients" (
"id"	INTEGER UNIQUE,
"surname"	TEXT,
"name"	TEXT,
"telephone_number"	TEXT UNIQUE,
"city_of_residence"	TEXT,
"datetime_entrance"	TEXT,
"datetime_exit"	TEXT,
"hours_parked" INTEGER,
"pariah"	INTEGER,
PRIMARY KEY("ID" AUTOINCREMENT)
) STRICT;
