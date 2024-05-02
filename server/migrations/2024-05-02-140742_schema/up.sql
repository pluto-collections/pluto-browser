-- Your SQL goes here
CREATE TABLE "user"(
	"id" INTEGER NOT NULL PRIMARY KEY,
	"username" TEXT NOT NULL,
	"full_name" TEXT NOT NULL,
	"email" TEXT NOT NULL,
	"password" TEXT NOT NULL,
	"profile_pic" TEXT NOT NULL,
	"created_at" TIMESTAMP NOT NULL,
	"updated_at" TIMESTAMP NOT NULL
);

CREATE TABLE "persona"(
	"id" INTEGER NOT NULL PRIMARY KEY,
	"name" TEXT NOT NULL,
	"profile_pic" TEXT NOT NULL,
	"color" TEXT NOT NULL,
	"created_at" TIMESTAMP NOT NULL,
	"updated_at" TIMESTAMP NOT NULL,
	"user_id" INTEGER NOT NULL,
	FOREIGN KEY ("user_id") REFERENCES "user"("id")
);

CREATE TABLE "workspace"(
	"id" INTEGER NOT NULL PRIMARY KEY,
	"persona_id" INTEGER NOT NULL,
	"icon" TEXT NOT NULL,
	"color" TEXT NOT NULL,
	"created_at" TIMESTAMP NOT NULL,
	"updated_at" TIMESTAMP NOT NULL,
	FOREIGN KEY ("persona_id") REFERENCES "persona"("id")
);

CREATE TABLE "layout"(
	"id" INTEGER NOT NULL PRIMARY KEY,
	"workspace_id" INTEGER NOT NULL,
	"position_x" INTEGER NOT NULL,
	"position_y" INTEGER NOT NULL,
	FOREIGN KEY ("workspace_id") REFERENCES "workspace"("id")
);

CREATE TABLE "tab"(
	"id" INTEGER NOT NULL PRIMARY KEY,
	"url" TEXT NOT NULL,
	"layout_id" INTEGER NOT NULL,
	FOREIGN KEY ("layout_id") REFERENCES "layout"("id")
);

CREATE TABLE "tag"(
	"id" INTEGER NOT NULL PRIMARY KEY,
	"tagname" TEXT NOT NULL
);

CREATE TABLE "bookmark"(
	"id" INTEGER NOT NULL PRIMARY KEY,
	"name" TEXT NOT NULL,
	"url" TEXT NOT NULL,
	"created_at" TIMESTAMP NOT NULL,
	"updated_at" TIMESTAMP NOT NULL,
	"persona_id" INTEGER NOT NULL,
	FOREIGN KEY ("persona_id") REFERENCES "persona"("id")
);


CREATE TABLE "bookmark_tag_relation"(
	"bookmark_id" INTEGER NOT NULL,
	"tag_id" INTEGER NOT NULL,
	PRIMARY KEY("bookmark_id", "tag_id")
);

CREATE TABLE "preference"(
	"id" INTEGER NOT NULL PRIMARY KEY,
	"persona_id" INTEGER NOT NULL,
	FOREIGN KEY ("persona_id") REFERENCES "persona"("id")
);

CREATE TABLE "history"(
	"id" INTEGER NOT NULL PRIMARY KEY,
	"title" TEXT NOT NULL,
	"favicon" TEXT NOT NULL,
	"url" TEXT NOT NULL,
	"date" TIMESTAMP NOT NULL,
	"persona_id" INTEGER NOT NULL,
	FOREIGN KEY ("persona_id") REFERENCES "persona"("id")
);

CREATE TABLE "tab_layout_relation"(
	"tab_id" INTEGER NOT NULL,
	"layout_id" INTEGER NOT NULL,
	PRIMARY KEY("tab_id", "layout_id")
);

