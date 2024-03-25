-- This file should undo anything in `up.sql`
CREATE TABLE "_sqlx_migrations"(
	"version" INT8 NOT NULL PRIMARY KEY,
	"description" TEXT NOT NULL,
	"installed_on" TIMESTAMPTZ NOT NULL,
	"success" BOOL NOT NULL,
	"checksum" BYTEA NOT NULL,
	"execution_time" INT8 NOT NULL
);

CREATE TABLE "posts"(
	"id" INT4 NOT NULL PRIMARY KEY,
	"title" VARCHAR NOT NULL,
	"body" TEXT NOT NULL,
	"published" BOOL NOT NULL
);

CREATE TABLE "todos"(
	"id" INT8 NOT NULL PRIMARY KEY,
	"description" TEXT NOT NULL,
	"done" BOOL NOT NULL
);

DROP TABLE IF EXISTS "users";
