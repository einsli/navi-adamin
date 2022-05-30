/*
 Navicat Premium Data Transfer

 Source Server         : 1.13.156.154
 Source Server Type    : MySQL
 Source Server Version : 80028
 Source Host           : 1.13.156.154:23106
 Source Schema         : navigation

 Target Server Type    : MySQL
 Target Server Version : 80028
 File Encoding         : 65001

 Date: 29/05/2022 14:50:35
*/

SET NAMES utf8mb4;
SET FOREIGN_KEY_CHECKS = 0;

-- ----------------------------
-- Table structure for navi_groups
-- ----------------------------
DROP TABLE IF EXISTS `navi_groups`;
CREATE TABLE `navi_groups` (
  `id` int NOT NULL AUTO_INCREMENT,
  `name` varchar(25) DEFAULT NULL,
  `is_deleted` tinyint(1) unsigned zerofill DEFAULT '0',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=18 DEFAULT CHARSET=utf8mb3;

-- ----------------------------
-- Table structure for navi_sites
-- ----------------------------
DROP TABLE IF EXISTS `navi_sites`;
CREATE TABLE `navi_sites` (
  `id` int NOT NULL AUTO_INCREMENT,
  `name` varchar(50) DEFAULT NULL,
  `site_description` varchar(255) DEFAULT NULL,
  `site_group_id` int DEFAULT NULL,
  `is_deleted` int DEFAULT '0',
  `site_img_path` varchar(255) DEFAULT NULL,
  `site_url` varchar(255) DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=17 DEFAULT CHARSET=utf8mb3;

SET FOREIGN_KEY_CHECKS = 1;
