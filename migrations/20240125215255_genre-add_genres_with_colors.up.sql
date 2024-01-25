DELETE FROM "Genre";

ALTER TABLE "Genre" ADD COLUMN color text NOT NULL default '';

INSERT INTO "Genre" (name, color) VALUES ('Biography', '#9ACD32');
INSERT INTO "Genre" (name, color) VALUES ('SciFi', '#6495ED');
INSERT INTO "Genre" (name, color) VALUES ('Mystery', '#483D8B');
INSERT INTO "Genre" (name, color) VALUES ('Fantasy', '#800080');
INSERT INTO "Genre" (name, color) VALUES ('Crime', '#FF4500');
INSERT INTO "Genre" (name, color) VALUES ('Horror', '#8B0000');
INSERT INTO "Genre" (name, color) VALUES ('Thriller', '#FFD700');
INSERT INTO "Genre" (name, color) VALUES ('Dystopian', '#2E8B57');
INSERT INTO "Genre" (name, color) VALUES ('Magic Realism', '#FFA500');
INSERT INTO "Genre" (name, color) VALUES ('Educational', '#87CEEB');
INSERT INTO "Genre" (name, color) VALUES ('Romance', '#FF69B4');
INSERT INTO "Genre" (name, color) VALUES ('Business and Economics', '#4169E1');
INSERT INTO "Genre" (name, color) VALUES ('Kids', '#00BFFF');
INSERT INTO "Genre" (name, color) VALUES ('Cooking', '#CD853F');
INSERT INTO "Genre" (name, color) VALUES ('Fairy Tales', '#FF6347');
INSERT INTO "Genre" (name, color) VALUES ('Novels', '#008080');
INSERT INTO "Genre" (name, color) VALUES ('History', '#8B4513');
INSERT INTO "Genre" (name, color) VALUES ('Adventure', '#228B22');
INSERT INTO "Genre" (name, color) VALUES ('Sports', '#FF8C00');
INSERT INTO "Genre" (name, color) VALUES ('Entertainment', '#FFD700');
INSERT INTO "Genre" (name, color) VALUES ('Travel', '#32CD32');
INSERT INTO "Genre" (name, color) VALUES ('Politics', '#800000');
INSERT INTO "Genre" (name, color) VALUES ('Motorsport', '#FF0000');
INSERT INTO "Genre" (name, color) VALUES ('Computers', '#00CED1');
INSERT INTO "Genre" (name, color) VALUES ('Art', '#FFD700');
INSERT INTO "Genre" (name, color) VALUES ('Fiction', '#9400D3');