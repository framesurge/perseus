-- ----------------------------
-- Table structure for users
-- ----------------------------
CREATE TABLE "public"."users" (
  "user_id" SERIAL PRIMARY KEY,
  "email" varchar COLLATE "pg_catalog"."default" NOT NULL UNIQUE,
  "hashed_password" varchar COLLATE "pg_catalog"."default" NOT NULL,
  "reset_password_selector" varchar COLLATE "pg_catalog"."default",
  "reset_password_sent_at" timestamp(6),
  "reset_password_validator_hash" varchar COLLATE "pg_catalog"."default",
  "created_at" timestamp(6) NOT NULL DEFAULT now(),
  "updated_at" timestamp(6) NOT NULL DEFAULT now()
);


INSERT INTO users(email, hashed_password) VALUES('test1@test1.com', 'chou1979');
INSERT INTO users(email, hashed_password) VALUES('test2@test1.com', 'chou1979');
INSERT INTO users(email, hashed_password) VALUES('test3@test1.com', 'chou1979');
INSERT INTO users(email, hashed_password) VALUES('test4@test1.com', 'chou1979');
INSERT INTO users(email, hashed_password) VALUES('test5@test1.com', 'chou1979');
INSERT INTO users(email, hashed_password) VALUES('test6@test1.com', 'chou1979');
INSERT INTO users(email, hashed_password) VALUES('test7@test1.com', 'chou1979');
INSERT INTO users(email, hashed_password) VALUES('test8@test1.com', 'chou1979');
INSERT INTO users(email, hashed_password) VALUES('test9@test1.com', 'chou1979');
INSERT INTO users(email, hashed_password) VALUES('test10@test1.com', 'chou1979');
INSERT INTO users(email, hashed_password) VALUES('test11@test1.com', 'chou1979');
INSERT INTO users(email, hashed_password) VALUES('test12@test1.com', 'chou1979');
INSERT INTO users(email, hashed_password) VALUES('test13@test1.com', 'chou1979');

-- ----------------------------
-- Table structure for post_types
-- ----------------------------

CREATE TABLE "public"."post_types" (
  "type_id" SERIAL PRIMARY KEY,
  "name" varchar(50) NOT NULL UNIQUE,
  "slug" varchar(50) NOT NULL UNIQUE
);

-- ----------------------------
-- Table structure for categories
-- ----------------------------
-- UPDATED: Now includes type_id to link it to a specific Post Type

CREATE TABLE "public"."categories" (
  "category_id" SERIAL PRIMARY KEY,
  "type_id" int4 REFERENCES "public"."post_types"("type_id") ON DELETE CASCADE,
  "title" varchar(100) NOT NULL,
  "description" text,
  "created_at" timestamp(6) DEFAULT now()
);

-- ----------------------------
-- Table structure for posts
-- ----------------------------

CREATE TABLE "public"."posts" (
  "post_id" SERIAL PRIMARY KEY,
  "user_id" int4 REFERENCES "public"."users"("user_id"),
  "category_id" int4 REFERENCES "public"."categories"("category_id"),
  "title" varchar(255) NOT NULL,
  "content" text,
  "published_at" timestamp(6) DEFAULT now()
);

-- ----------------------------
-- Records of post_types
-- ----------------------------
INSERT INTO "public"."post_types" ("type_id", "name", "slug") VALUES
(1, 'Blog', 'blog'),
(2, 'Internal News', 'internal-news'),
(3, 'Inventory', 'inventory'),
(4, 'Fleet Logs', 'fleet-logs'),
(5, 'Support Tickets', 'support');

-- ----------------------------
-- Records of categories (Linked to specific Post Types)
-- ----------------------------
INSERT INTO "public"."categories" ("type_id", "title", "description") VALUES
(1, 'Tech Tips', 'General IT blog posts'),          -- Linked to Blog
(1, 'Industry Trends', 'Global haulage trends'),    -- Linked to Blog
(2, 'HR Updates', 'Company internal news'),         -- Linked to Internal News
(3, 'Truck Parts', 'Heavy machinery inventory'),    -- Linked to Inventory
(4, 'Fuel Receipts', 'Daily fuel consumption logs'); -- Linked to Fleet Logs

-- ----------------------------
-- Records of posts
-- ----------------------------
INSERT INTO "public"."posts" ("user_id", "category_id", "title", "content") VALUES
(1, 1, 'Optimizing Postgres', 'How to tune your DB...'),
(2, 2, 'The 2024 Logistics Outlook', 'Global shipping is changing...'),
(1, 3, 'New Policy on Leave', 'Please read the updated HR handbook...'),
(3, 4, 'Brake Pads - Volvo FH', 'Restocked 20 units of ceramic pads...'),
(4, 5, 'Fuel Log - Truck #402', 'Filled 200L at Shell station...');
