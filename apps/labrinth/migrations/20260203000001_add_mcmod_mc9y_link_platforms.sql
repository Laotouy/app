-- 添加 MC百科 和 九域资源社区 链接平台

-- 添加 MC百科 (mcmod)
INSERT INTO link_platforms (name, donation)
VALUES ('mcmod', true)
ON CONFLICT (name) DO UPDATE SET donation = EXCLUDED.donation;

-- 添加 九域资源社区 (mc9y)
INSERT INTO link_platforms (name, donation)
VALUES ('mc9y', true)
ON CONFLICT (name) DO UPDATE SET donation = EXCLUDED.donation;

SELECT setval('link_platforms_id_seq', COALESCE((SELECT MAX(id) FROM link_platforms), 1));
SELECT setval('mods_links_id_seq', COALESCE((SELECT MAX(id) FROM mods_links), 1));