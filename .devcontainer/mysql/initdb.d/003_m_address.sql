USE RUST_API;


DROP TABLE IF EXISTS `m_address`;
CREATE TABLE `m_address` (
  `id` INT UNSIGNED NOT NULL AUTO_INCREMENT COMMENT 'ID',
  `country_code` CHAR(3) NOT NULL COMMENT '国コード',
  `state` VARCHAR(255) NOT NULL COMMENT '都道府県',
  `city` VARCHAR(255) DEFAULT 'Unapproved' COMMENT '市区町村',
  `address_line1` VARCHAR(255) DEFAULT NULL COMMENT '大字・町名',
  `address_line2` VARCHAR(255) DEFAULT NULL COMMENT 'ビル名・階数',
  `create_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT 'データ登録日',
  `create_user_code` VARCHAR(8) NOT NULL DEFAULT 'SYSTEM' COMMENT 'データ登録者コード',
  `update_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT 'データ更新日',
  `update_user_code` VARCHAR(8) NOT NULL DEFAULT 'SYSTEM' COMMENT 'データ更新者コード',
  `delete_flag` BIT(1) DEFAULT b'0' COMMENT '削除フラグ',
  PRIMARY KEY (`id`),
  KEY `m_address_idx_01` (`country_code`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 ROW_FORMAT=DYNAMIC COMMENT='住所マスター';
