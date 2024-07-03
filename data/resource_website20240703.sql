/*
 Navicat Premium Data Transfer

 Source Server         : 笔记本
 Source Server Type    : MySQL
 Source Server Version : 80300
 Source Host           : 192.168.1.53:3306
 Source Schema         : resource_website

 Target Server Type    : MySQL
 Target Server Version : 80300
 File Encoding         : 65001

 Date: 03/07/2024 11:21:07
*/

SET NAMES utf8mb4;
SET FOREIGN_KEY_CHECKS = 0;

-- ----------------------------
-- Table structure for count_data
-- ----------------------------
DROP TABLE IF EXISTS `count_data`;
CREATE TABLE `count_data`  (
  `id` int UNSIGNED NOT NULL,
  `count_deal` bigint UNSIGNED NOT NULL COMMENT '交易次数',
  `count_recharge` bigint UNSIGNED NOT NULL COMMENT '充值金额',
  `count_withdraw` bigint UNSIGNED NOT NULL COMMENT '取款金额',
  `count_custom` bigint UNSIGNED NOT NULL COMMENT '用户量',
  PRIMARY KEY (`id`) USING BTREE
) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci ROW_FORMAT = Dynamic;

-- ----------------------------
-- Records of count_data
-- ----------------------------

-- ----------------------------
-- Table structure for custom_orders
-- ----------------------------
DROP TABLE IF EXISTS `custom_orders`;
CREATE TABLE `custom_orders`  (
  `order_uuid` varchar(36) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '订单唯一标识',
  `user_uuid` varchar(36) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '客户ID, 外键关联到users_custom表的user_id',
  `resource_uuid` varchar(36) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '资源ID, 外键关联到resources表的resource_id',
  `resource_name` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '资源名称',
  `resource_category` varchar(10) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '资源类型',
  `resource_language` varchar(10) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '资源使用的开发语言',
  `download_link` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '下载链接',
  `order_resource_price` bigint UNSIGNED NOT NULL COMMENT '订单所花金额',
  `creation_date` datetime NOT NULL COMMENT '订单创建时间',
  PRIMARY KEY (`order_uuid`) USING BTREE,
  INDEX `user_uuid`(`user_uuid` ASC) USING BTREE,
  INDEX `resource_uuid`(`resource_uuid` ASC) USING BTREE
) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci COMMENT = '订单表' ROW_FORMAT = DYNAMIC;

-- ----------------------------
-- Records of custom_orders
-- ----------------------------

-- ----------------------------
-- Table structure for custom_recharge
-- ----------------------------
DROP TABLE IF EXISTS `custom_recharge`;
CREATE TABLE `custom_recharge`  (
  `record_uuid` varchar(36) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '充值记录唯一标识',
  `user_uuid` varchar(36) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '客户ID, 外键关联到users_custom表的user_id',
  `recharge_amount` bigint UNSIGNED NOT NULL COMMENT '充值金额（以USDT为单位）',
  `payment_channel` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '支付渠道（如：支付宝余额、某银行）',
  `recharge_status` int UNSIGNED NOT NULL COMMENT '充值状态（如：待处理0、处理中1、成功2、失败3、已取消4）',
  `recharge_date` datetime NOT NULL COMMENT '充值时间',
  `transaction_id` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '充值交易ID（用于追踪USDT充值交易）',
  `remark` text CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL COMMENT '备注信息',
  PRIMARY KEY (`record_uuid`) USING BTREE,
  INDEX `user_uuid`(`user_uuid` ASC) USING BTREE,
  INDEX `idx_recharge_date`(`recharge_date` ASC) USING BTREE COMMENT '充值时间索引',
  INDEX `idx_recharge_status`(`recharge_status` ASC) USING BTREE COMMENT '充值状态索引',
  CONSTRAINT `custom_recharge_ibfk_1` FOREIGN KEY (`user_uuid`) REFERENCES `custom_user` (`user_uuid`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci COMMENT = '充值记录表' ROW_FORMAT = DYNAMIC;

-- ----------------------------
-- Records of custom_recharge
-- ----------------------------

-- ----------------------------
-- Table structure for custom_user
-- ----------------------------
DROP TABLE IF EXISTS `custom_user`;
CREATE TABLE `custom_user`  (
  `user_uuid` varchar(36) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '账号唯一标识',
  `nick_name` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '昵称',
  `user_name` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '用户名',
  `user_pwd` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '密码哈希值',
  `email` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL COMMENT '电子邮箱',
  `user_status` int UNSIGNED NOT NULL COMMENT '可用：0，禁用：1',
  `balance_usdt` bigint UNSIGNED NOT NULL COMMENT '账户余额（以USDT为单位）',
  `registration_date` datetime NOT NULL COMMENT '注册时间',
  `avatar_path` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '头像图片',
  PRIMARY KEY (`user_uuid`) USING BTREE,
  INDEX `user_uuid`(`user_uuid` ASC) USING BTREE
) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci COMMENT = '用户自定义信息表' ROW_FORMAT = DYNAMIC;

-- ----------------------------
-- Records of custom_user
-- ----------------------------

-- ----------------------------
-- Table structure for sys_carousel
-- ----------------------------
DROP TABLE IF EXISTS `sys_carousel`;
CREATE TABLE `sys_carousel`  (
  `id` int(10) UNSIGNED ZEROFILL NOT NULL AUTO_INCREMENT,
  `image_uuid` varchar(36) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '对应sys_image表',
  PRIMARY KEY (`id`) USING BTREE
) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci ROW_FORMAT = Dynamic;

-- ----------------------------
-- Records of sys_carousel
-- ----------------------------

-- ----------------------------
-- Table structure for sys_image
-- ----------------------------
DROP TABLE IF EXISTS `sys_image`;
CREATE TABLE `sys_image`  (
  `image_uuid` varchar(36) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '图片唯一标识',
  `image_name` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '图片名称',
  `image_path` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '图片存储路径或URL',
  `image_to` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL COMMENT '跳转链接',
  `description` text CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL COMMENT '图片描述',
  `usage_location` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL COMMENT '图片使用位置或功能标识',
  PRIMARY KEY (`image_uuid`) USING BTREE,
  INDEX `idx_usage_location`(`usage_location` ASC) USING BTREE
) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci COMMENT = '系统图片表' ROW_FORMAT = DYNAMIC;

-- ----------------------------
-- Records of sys_image
-- ----------------------------

-- ----------------------------
-- Table structure for sys_menus
-- ----------------------------
DROP TABLE IF EXISTS `sys_menus`;
CREATE TABLE `sys_menus`  (
  `user_role` int UNSIGNED NOT NULL COMMENT '用户需要的权限等级',
  `f_role` int NULL DEFAULT NULL COMMENT '父权限等级',
  `menu_name` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '菜单项名称',
  `menu_url` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '菜单项路径',
  `menu_des` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL COMMENT '菜单项描述'
) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci ROW_FORMAT = Dynamic;

-- ----------------------------
-- Records of sys_menus
-- ----------------------------
INSERT INTO `sys_menus` VALUES (0, NULL, '管理侧管理', '/admin', '需要超级管理员权限');
INSERT INTO `sys_menus` VALUES (0, NULL, '查看所有管理员', '/admin/all', '需要超级管理员权限');
INSERT INTO `sys_menus` VALUES (0, NULL, '查看所有取款申请', '/unprocessed', '需要超级管理员权限');
INSERT INTO `sys_menus` VALUES (1, 0, '用户侧管理', '/custom', '需要管理员权限');
INSERT INTO `sys_menus` VALUES (1, 0, '为用户充值', '/custom/recharge', '需要管理员权限');
INSERT INTO `sys_menus` VALUES (1, 0, '查看所有用户', '/custom/all', '需要管理员权限');
INSERT INTO `sys_menus` VALUES (1, 0, '网站管理', '/website', '需要管理员权限');
INSERT INTO `sys_menus` VALUES (1, 0, '网站信息', '/website/detail', '需要管理员权限');
INSERT INTO `sys_menus` VALUES (1, 0, '轮播图', '/website/carousels', '需要管理员权限');
INSERT INTO `sys_menus` VALUES (2, 0, '查看信息', '/user/detail', '需要普通权限');
INSERT INTO `sys_menus` VALUES (2, 0, '修改密码', '/user/updatePwd', '需要普通权限');
INSERT INTO `sys_menus` VALUES (2, 0, '修改信息', '/user/updateInfo', '需要普通权限');
INSERT INTO `sys_menus` VALUES (2, 0, '提取', '/user/withdrawl', '需要普通权限');
INSERT INTO `sys_menus` VALUES (2, 0, '提取记录', '/user/withdrawls', '需要普通权限');
INSERT INTO `sys_menus` VALUES (2, 0, '资源管理', '/resources', '需要普通权限');
INSERT INTO `sys_menus` VALUES (2, 0, '查看资源', '/resource/query', '需要普通权限');
INSERT INTO `sys_menus` VALUES (2, 0, '图片', '/resource/image/all', NULL);
INSERT INTO `sys_menus` VALUES (2, 0, '查看语言', '/resource/languagies', NULL);
INSERT INTO `sys_menus` VALUES (2, 0, '查看分类', '/resource/categories', NULL);

-- ----------------------------
-- Table structure for sys_resource_category
-- ----------------------------
DROP TABLE IF EXISTS `sys_resource_category`;
CREATE TABLE `sys_resource_category`  (
  `category_id` int UNSIGNED NOT NULL AUTO_INCREMENT COMMENT '主键',
  `category_name` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '资源类型名称',
  `crate_user_name` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '创建人名称',
  PRIMARY KEY (`category_id`) USING BTREE
) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci ROW_FORMAT = Dynamic;

-- ----------------------------
-- Records of sys_resource_category
-- ----------------------------

-- ----------------------------
-- Table structure for sys_resource_images
-- ----------------------------
DROP TABLE IF EXISTS `sys_resource_images`;
CREATE TABLE `sys_resource_images`  (
  `id` int UNSIGNED NOT NULL AUTO_INCREMENT COMMENT '关联唯一标识',
  `resource_uuid` varchar(36) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '资源ID',
  `image_uuid` varchar(36) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '图片ID',
  PRIMARY KEY (`id`) USING BTREE,
  INDEX `resource_uuid`(`resource_uuid` ASC) USING BTREE,
  INDEX `image_uuid`(`image_uuid` ASC) USING BTREE
) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci COMMENT = '资源图片关联表' ROW_FORMAT = DYNAMIC;

-- ----------------------------
-- Records of sys_resource_images
-- ----------------------------

-- ----------------------------
-- Table structure for sys_resource_language
-- ----------------------------
DROP TABLE IF EXISTS `sys_resource_language`;
CREATE TABLE `sys_resource_language`  (
  `language_id` int UNSIGNED NOT NULL AUTO_INCREMENT COMMENT '主键',
  `language_name` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '开发语言：Java、PHP',
  `create_user_name` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '创建人名称',
  PRIMARY KEY (`language_id`) USING BTREE
) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci ROW_FORMAT = Dynamic;

-- ----------------------------
-- Records of sys_resource_language
-- ----------------------------

-- ----------------------------
-- Table structure for sys_resources
-- ----------------------------
DROP TABLE IF EXISTS `sys_resources`;
CREATE TABLE `sys_resources`  (
  `resource_uuid` varchar(36) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '资源唯一标识',
  `resource_name` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '资源名称',
  `description` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL COMMENT '资源描述',
  `description_file_path` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL COMMENT '资源描述文件',
  `resource_price` bigint UNSIGNED NOT NULL COMMENT '资源价格',
  `category` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '资源类型',
  `language` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '开发语言',
  `resource_link` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '资源文件存储路径或URL',
  `create_date` datetime NOT NULL COMMENT '创建日期',
  `create_user_name` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '创建人',
  PRIMARY KEY (`resource_uuid`) USING BTREE
) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci COMMENT = '资源信息表' ROW_FORMAT = DYNAMIC;

-- ----------------------------
-- Records of sys_resources
-- ----------------------------

-- ----------------------------
-- Table structure for sys_user
-- ----------------------------
DROP TABLE IF EXISTS `sys_user`;
CREATE TABLE `sys_user`  (
  `user_uuid` varchar(36) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '管理员唯一标识',
  `nick_name` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '昵称',
  `user_name` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '管理员用户名',
  `user_pwd` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '密码哈希值',
  `balance` bigint UNSIGNED NOT NULL COMMENT '收到的钱',
  `liaison` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '联系地址：Tg',
  `user_status` int UNSIGNED NOT NULL COMMENT '可用：0， 禁用：1',
  `role` int UNSIGNED NOT NULL COMMENT '管理员角色（0：超级管理员、1：普通管理员、2：个商）',
  `avatar_path` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '头像图片',
  UNIQUE INDEX `idx_unique_username`(`user_name` ASC) USING BTREE
) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci COMMENT = '管理员系统用户表' ROW_FORMAT = DYNAMIC;

-- ----------------------------
-- Records of sys_user
-- ----------------------------
INSERT INTO `sys_user` VALUES ('fc42232a-0714-47e4-a127-0089990f2cdb', '超级管理员', 'superadmin', '$argon2id$v=19$m=19456,t=2,p=1$CX3yvhubEl2+7cITw/QmkQ$T4tq8tbwNQQoslTEictuWEYFaEf73mRlmzK8mFR8oXc', 0, '/t.me/bitpieok', 0, 0, '../assets/avatar/default.png');

-- ----------------------------
-- Table structure for sys_website_info
-- ----------------------------
DROP TABLE IF EXISTS `sys_website_info`;
CREATE TABLE `sys_website_info`  (
  `id` int(10) UNSIGNED ZEROFILL NOT NULL AUTO_INCREMENT,
  `name` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '网站名称',
  `version` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL COMMENT '程序版本',
  `public_record` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL COMMENT '公网备案',
  `website_record` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL COMMENT '网站备案',
  `sys_kefu` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL COMMENT '客服链接',
  `website_icon` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL COMMENT '网站图标id（外键，关联sys_image表）',
  `custom_login_img` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL COMMENT '客户登录页图片id（外键，关联sys_image表）',
  `admin_login_img` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL COMMENT '管理员登录页图片id（外键，关联sys_image表）',
  PRIMARY KEY (`id`) USING BTREE
) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci COMMENT = '网站信息表' ROW_FORMAT = DYNAMIC;

-- ----------------------------
-- Records of sys_website_info
-- ----------------------------

-- ----------------------------
-- Table structure for withdrawals
-- ----------------------------
DROP TABLE IF EXISTS `withdrawals`;
CREATE TABLE `withdrawals`  (
  `uuid` varchar(36) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL,
  `user_uuid` varchar(36) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '挂售取款记录',
  `quantities` bigint UNSIGNED NOT NULL COMMENT '取款金额',
  `arrive` bigint UNSIGNED NOT NULL COMMENT '到账金额',
  `create_date` datetime NOT NULL COMMENT '取款时间',
  `tariff` bigint UNSIGNED NOT NULL COMMENT '手续费',
  `status` int UNSIGNED NOT NULL COMMENT '状态码:(0: 完成，1未完成，2取消/关闭)',
  `succes_date` datetime NOT NULL COMMENT '完成时间',
  PRIMARY KEY (`uuid`) USING BTREE
) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci ROW_FORMAT = Dynamic;

-- ----------------------------
-- Records of withdrawals
-- ----------------------------

SET FOREIGN_KEY_CHECKS = 1;
