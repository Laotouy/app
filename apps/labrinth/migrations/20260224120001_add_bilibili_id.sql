ALTER TABLE users ADD COLUMN bilibili_id text;
CREATE INDEX users_bilibili_id ON users (bilibili_id);
