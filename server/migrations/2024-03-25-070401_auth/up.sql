-- Your SQL goes here
DROP TABLE IF EXISTS "_sqlx_migrations";
DROP TABLE IF EXISTS "posts";
DROP TABLE IF EXISTS "todos";
CREATE TABLE "users"(
	"id" INT4 NOT NULL PRIMARY KEY,
	"username" VARCHAR NOT NULL,
	"full_name" TEXT NOT NULL,
	"email" TEXT NOT NULL,
	"password" TEXT NOT NULL,
	"created_at" TEXT NOT NULL,
	"updated_at" TEXT NOT NULL
);

