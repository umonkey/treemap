-- SQLite database schema for the tree map.
-- Use `make sqlite-schema` to apply.

CREATE TABLE IF NOT EXISTS trees (
    `id` INT NOT NULL,
    `lat` REAL NOT NULL,
    `lon` REAL NOT NULL,
    `species` TEXT NOT NULL, -- latin name
    `notes` TEXT NULL,
    `height` REAL NULL,
    `circumference` REAL NULL,
    `diameter` REAL NULL,
    `state` TEXT NOT NULL DEFAULT "healthy",
    `added_at` INT NOT NULL,
    `updated_at` INT NOT NULL,
    `added_by` INT NOT NULL,
    `thumbnail_id` INT NULL,
    PRIMARY KEY(`id`)
);

CREATE INDEX IF NOT EXISTS trees_lat ON trees (lat);
CREATE INDEX IF NOT EXISTS trees_lon ON trees (lon);
CREATE INDEX IF NOT EXISTS trees_state ON trees (state);


-- Tree attributes.
-- This contains random properties, like year of planting, height, etc.
--
-- We keep all records ever added, we then aggregate them to use the latest
-- added value for each attribute to show on the tree page.
--
-- The code that reads or updates this table needs to be aware of the data
-- type, since the values are stored as strings.
CREATE TABLE IF NOT EXISTS trees_props (
    `id` INT NOT NULL,
    `tree_id` INT NOT NULL,
    `added_at` INT NOT NULL,
    `name` TEXT NOT NULL,
    `value` TEXT NOT NULL,
    PRIMARY KEY(`id`)
);

CREATE INDEX IF NOT EXISTS trees_props_tree_id ON trees_props (tree_id);


-- Tree images.
CREATE TABLE IF NOT EXISTS files (
    `id` INT NOT NULL,
    `tree_id` INT NOT NULL,
    `added_at` INT NOT NULL,
    `added_by` INT NOT NULL,
    `small_id` INT NOT NULL,
    `large_id` INT NOT NULL,
    PRIMARY KEY(`id`)
);

CREATE INDEX IF NOT EXISTS files_tree_id ON files (tree_id);


-- Upload tickets.
-- Ticket id is also the file name.
CREATE TABLE IF NOT EXISTS upload_tickets (
    `id` INT NOT NULL,
    `created_at` INT NOT NULL,
    `created_by` INT NOT NULL,
    `upload_url` TEXT NOT NULL,
    PRIMARY KEY(`id`)
);


-- User accounts.
CREATE TABLE IF NOT EXISTS users (
    `id` INT NOT NULL,
    `email` TEXT NOT NULL,
    `name` TEXT NOT NULL,
    `picture` TEXT NOT NULL,
    PRIMARY KEY(`id`)
);
CREATE UNIQUE INDEX IF NOT EXISTS users_email ON users (email);


CREATE TABLE IF NOT EXISTS comments (
    `id` INT NOT NULL,
    `tree_id` INT NOT NULL,
    `added_at` INT NOT NULL,
    `added_by` INT NOT NULL,
    `message` TEXT NOT NULL,
    PRIMARY KEY(`id`)
);
CREATE INDEX IF NOT EXISTS comments_tree_id ON comments (tree_id);
CREATE INDEX IF NOT EXISTS comments_added_at ON comments (added_at);


CREATE TABLE IF NOT EXISTS queue_messages (
    `id` INT NOT NULL,
    `added_at` INT NOT NULL,
    `available_at` INT NOT NULL,
    `payload` TEXT NOT NULL,
    PRIMARY KEY(`id`)
);
CREATE INDEX IF NOT EXISTS queue_messages_added_at ON queue_messages (added_at);
CREATE INDEX IF NOT EXISTS queue_messages_available_at ON queue_messages (available_at);


CREATE TABLE IF NOT EXISTS species (
    `name` TEXT NOT NULL,
    `local` TEXT NOT NULL,
    `keywords` TEXT NULL,
    PRIMARY KEY(`name`)
);

DELETE FROM species;
INSERT INTO species (name, local, keywords) VALUES ('Acer negundo', 'Box Elder', 'клён;ясенелистный;boxelder;maple;ashleaf;manitoba');
INSERT INTO species (name, local, keywords) VALUES ('Acer pseudoplatanus', 'Sycamore maple', 'клён белый;явор;немецкий;maple;sycamore');
INSERT INTO species (name, local, keywords) VALUES ('Acer', 'Maple', 'клён');
INSERT INTO species (name, local, keywords) VALUES ('Aesculus hippocastanum', 'Horse chestnut', 'каштан;конский;buckeye;conker');
INSERT INTO species (name, local, keywords) VALUES ('Betula', 'Birch', 'берёза;береза');
INSERT INTO species (name, local, keywords) VALUES ('Catalpa', 'Catalpa', 'катальпа');
INSERT INTO species (name, local, keywords) VALUES ('Cercis siliquastrum', 'Judas tree', 'багряник европейский');
INSERT INTO species (name, local, keywords) VALUES ('Fraxinus', 'Ash', 'ясень');
INSERT INTO species (name, local, keywords) VALUES ('Juglans', 'Walnut', 'орех');
INSERT INTO species (name, local, keywords) VALUES ('Morus', 'Mulberry', 'шелковица;silkworm');
INSERT INTO species (name, local, keywords) VALUES ('Paulwnia', 'Foxglove', 'пауловния;павловния');
INSERT INTO species (name, local, keywords) VALUES ('Picea', 'Spruce', 'ель;ёлка');
INSERT INTO species (name, local, keywords) VALUES ('Pinus', 'Pine', 'сосна');
INSERT INTO species (name, local, keywords) VALUES ('Platanus orientalis', 'Oriental plane', 'sycamore;платан;чинар');
INSERT INTO species (name, local, keywords) VALUES ('Populus alba', 'White poplar', 'тополь;белый');
INSERT INTO species (name, local, keywords) VALUES ('Populus', 'Poplar', 'тополь');
INSERT INTO species (name, local, keywords) VALUES ('Quercus robur', 'Pedunculate oak', 'дуб;черешчатый');
INSERT INTO species (name, local, keywords) VALUES ('Quercus rubra', 'Northern red oak', 'дуб;красный');
INSERT INTO species (name, local, keywords) VALUES ('Quercus', 'Oak', 'дуб');
INSERT INTO species (name, local, keywords) VALUES ('Salix', 'Willow', 'ива');
INSERT INTO species (name, local, keywords) VALUES ('Thuja', 'Cedar', 'туя;биота;cedar');
INSERT INTO species (name, local, keywords) VALUES ('Tilia', 'Linden', 'липа');
INSERT INTO species (name, local, keywords) VALUES ('Ulmus glabra', 'Wych elm', 'вяз;шершавый;scots');
INSERT INTO species (name, local, keywords) VALUES ('Ulmus parvifolia', 'Lacebark elm', 'вяз;мелколистный;карагач;chinese;sieboldii');
INSERT INTO species (name, local, keywords) VALUES ('Ulmus', 'Elm', 'вяз;карагач');
