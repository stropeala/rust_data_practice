CREATE TABLE "Clients" (
"id"	INTEGER UNIQUE,
"surname"	TEXT,
"name"	TEXT,
"telephone_number"	TEXT UNIQUE,
"city_of_residence"	TEXT,
"datetime_entrance"	TEXT,
"datetime_exit"	TEXT,
"hours_Parked" TEXT,
"pariah"	TEXT,
PRIMARY KEY("ID" AUTOINCREMENT)
) STRICT;
