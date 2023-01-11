USE RUST_API;

DROP TABLE IF EXISTS `m_country`;
CREATE TABLE `m_country` (
  `id` INT UNSIGNED NOT NULL AUTO_INCREMENT COMMENT 'ID',
  `code` CHAR(3) NOT NULL COMMENT '国コード',
  `value` VARCHAR(255) NOT NULL COMMENT '国名',
  `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT 'データ登録日',
  `created_user_code` VARCHAR(8) NOT NULL DEFAULT 'SYSTEM' COMMENT 'データ登録者コード',
  `updated_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT 'データ更新日',
  `updated_user_code` VARCHAR(8) NOT NULL DEFAULT 'SYSTEM' COMMENT 'データ更新者コード',
  PRIMARY KEY (`id`),
  UNIQUE KEY `m_country_idx_01` (`code`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 ROW_FORMAT=DYNAMIC COMMENT='国マスター';

INSERT INTO `m_country` (`code`, `value`)
VALUES
    ('001', "アメリカ"),
    ('002', "日本"),
    ('003', "中国"),
    ('004', "韓国");
