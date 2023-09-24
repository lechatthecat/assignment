-- Create restaurant tables table
DROP TABLE IF EXISTS orders;
DROP TABLE IF EXISTS restaurant_tables;
CREATE TABLE restaurant_tables (
  id INTEGER primary key generated always as identity,
  table_number INTEGER not null,
  note VARCHAR(500) DEFAULT null,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX index_table on restaurant_tables (table_number);

-- Adds 10 tables
INSERT INTO restaurant_tables (table_number, note) VALUES (1, null);
INSERT INTO restaurant_tables (table_number, note) VALUES (2, null);
INSERT INTO restaurant_tables (table_number, note) VALUES (3, null);
INSERT INTO restaurant_tables (table_number, note) VALUES (4, null);
INSERT INTO restaurant_tables (table_number, note) VALUES (5, null);
INSERT INTO restaurant_tables (table_number, note) VALUES (6, null);
INSERT INTO restaurant_tables (table_number, note) VALUES (7, null);
INSERT INTO restaurant_tables (table_number, note) VALUES (8, null);
INSERT INTO restaurant_tables (table_number, note) VALUES (9, null);
INSERT INTO restaurant_tables (table_number, note) VALUES (10, null);

-- Create users table
DROP TABLE IF EXISTS users;
CREATE TABLE users (
  id INTEGER primary key generated always as identity,
  name VARCHAR(50) NOT NULL,
  password VARCHAR(61) NOT null,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  UNIQUE(name)
);
CREATE UNIQUE INDEX unique_user on users (name);

-- TODO user generation
-- add users
INSERT INTO users ("name","password") VALUES
	 ('test_user1','$2b$04$BuM27R11fuD0hubq.Nykd.aw.WDI8F2/lYCPabzfLdGG1GHvYqR/i'),
	 ('test_user2','$2b$04$BuM27R11fuD0hubq.Nykd.aw.WDI8F2/lYCPabzfLdGG1GHvYqR/i'),
	 ('test_user3','$2b$04$BuM27R11fuD0hubq.Nykd.aw.WDI8F2/lYCPabzfLdGG1GHvYqR/i'),
	 ('test_user4','$2b$04$BuM27R11fuD0hubq.Nykd.aw.WDI8F2/lYCPabzfLdGG1GHvYqR/i'),
	 ('test_user5','$2b$04$BuM27R11fuD0hubq.Nykd.aw.WDI8F2/lYCPabzfLdGG1GHvYqR/i');

-- Create menus table
DROP TABLE IF EXISTS menus;
CREATE TABLE menus (
  id INTEGER primary key generated always as identity,
  name VARCHAR(50) NOT NULL,
  cook_time_seconds INTEGER NOT null,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE UNIQUE INDEX index_menu on menus (name, cook_time_seconds);

-- TODO menu item generation
-- add menus
INSERT INTO menus ("name","cook_time_seconds") VALUES
	 ('Hamburger',300), -- 5 minutes
   ('Curry rice',300), -- 5 minutes
	 ('Special Hamburger',600), -- 10 minutes
   ('Special soup',900), -- 15 minutes
   ('Sushi',900), -- 15 minutes
   ('Potato',60), -- 1 minute
   ('Cheese Hamburger',300), -- 5 minutes
   ('Rice',300), -- 5 minutes
	 ('Churros',600), -- 10 minutes
   ('Ice cream',300), -- 5 minutes
   ('Roll cake',900), -- 15 minutes
   ('Small cake',60), -- 1 minute
   ('Bread',300), -- 5 minutes
   ('Cola',60), -- 1 minute
   ('Strawberry cake',600), -- 10 minutes
   ('Tea',60), -- 1 minute
   ('Cheese Cake',900), -- 15 minutes
   ('Potato soup',300), -- 5 minute
   ('Hamburger and cola set',900), -- 15 minutes
   ('Curry rice and cola set',900), -- 15 minutes
	 ('Special Hamburger and cola set',900), -- 15 minutes
   ('Sushi and cola set',900), -- 15 minutes
   ('Lemon cake',900); -- 15 minutes;

-- Create orders table
CREATE TABLE orders (
  restaurant_table_id INTEGER NOT null REFERENCES restaurant_tables (id),
  menu_id INTEGER NOT null REFERENCES menus (id),
  expected_cook_finish_time TIMESTAMP NOT null,
  is_checked_by_staff BOOLEAN NOT NULL,
  checked_by_user_id INTEGER DEFAULT null REFERENCES users (id),
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  UNIQUE(restaurant_table_id, menu_id)
);
CREATE UNIQUE INDEX unique_order on orders (restaurant_table_id, menu_id);