/*
SQLyog Community v13.1.7 (64 bit)
MySQL - 8.0.24 : Database - flip
*********************************************************************
*/

/*!40101 SET NAMES utf8 */;

/*!40101 SET SQL_MODE=''*/;

/*!40014 SET @OLD_UNIQUE_CHECKS=@@UNIQUE_CHECKS, UNIQUE_CHECKS=0 */;
/*!40014 SET @OLD_FOREIGN_KEY_CHECKS=@@FOREIGN_KEY_CHECKS, FOREIGN_KEY_CHECKS=0 */;
/*!40101 SET @OLD_SQL_MODE=@@SQL_MODE, SQL_MODE='NO_AUTO_VALUE_ON_ZERO' */;
/*!40111 SET @OLD_SQL_NOTES=@@SQL_NOTES, SQL_NOTES=0 */;
CREATE DATABASE /*!32312 IF NOT EXISTS*/`flip_my` /*!40100 DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci */ /*!80016 DEFAULT ENCRYPTION='N' */;

USE `flip_my`;

/*Table structure for table `channel` */

DROP TABLE IF EXISTS `channel`;

CREATE TABLE `channel` (
  `channel_cid` bigint unsigned NOT NULL,
  `pb_data` longblob,
  PRIMARY KEY (`channel_cid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

/*Table structure for table `channel_follower` */

DROP TABLE IF EXISTS `channel_follower`;

CREATE TABLE `channel_follower` (
  `channel_cid` bigint unsigned NOT NULL,
  `profile_cid` bigint unsigned NOT NULL,
  `follow_gid` bigint unsigned DEFAULT NULL,
  PRIMARY KEY (`channel_cid`,`profile_cid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

/*Table structure for table `channel_msg` */

DROP TABLE IF EXISTS `channel_msg`;

CREATE TABLE `channel_msg` (
  `channel_cid` bigint unsigned NOT NULL,
  `msg_gid` bigint unsigned NOT NULL,
  `pb_data` longblob,
  PRIMARY KEY (`channel_cid`,`msg_gid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

/*Table structure for table `channel_msg_media` */

DROP TABLE IF EXISTS `channel_msg_media`;

CREATE TABLE `channel_msg_media` (
  `channel_cid` bigint unsigned NOT NULL,
  `media_type_id` tinyint unsigned NOT NULL,
  `msg_gid` bigint unsigned NOT NULL,
  PRIMARY KEY (`channel_cid`,`media_type_id`,`msg_gid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

/*Table structure for table `channel_subscriber` */

DROP TABLE IF EXISTS `channel_subscriber`;

CREATE TABLE `channel_subscriber` (
  `channel_cid` bigint unsigned NOT NULL,
  `profile_cid` bigint unsigned NOT NULL,
  `subscriber_gid` bigint unsigned NOT NULL,
  PRIMARY KEY (`channel_cid`,`profile_cid`,`subscriber_gid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

/*Table structure for table `chat` */

DROP TABLE IF EXISTS `chat`;

CREATE TABLE `chat` (
  `profile_cid` bigint unsigned NOT NULL,
  `chat_gid` bigint unsigned NOT NULL,
  `pb_data` longblob,
  PRIMARY KEY (`profile_cid`,`chat_gid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

/*Table structure for table `chat_msg` */

DROP TABLE IF EXISTS `chat_msg`;

CREATE TABLE `chat_msg` (
  `profile_cid` bigint unsigned NOT NULL,
  `chat_gid` bigint unsigned NOT NULL,
  `msg_gid` bigint unsigned NOT NULL,
  `pb_data` longblob,
  PRIMARY KEY (`profile_cid`,`chat_gid`,`msg_gid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

/*Table structure for table `direct` */

DROP TABLE IF EXISTS `direct`;

CREATE TABLE `direct` (
  `profile_cid` bigint unsigned NOT NULL,
  `direct_gid` bigint unsigned NOT NULL,
  `msg_gid` bigint unsigned NOT NULL,
  `pb_data` longblob,
  PRIMARY KEY (`profile_cid`,`direct_gid`,`msg_gid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

/*Table structure for table `file` */

DROP TABLE IF EXISTS `file`;

CREATE TABLE `file` (
  `file_gid` bigint unsigned NOT NULL,
  `pb_data` longblob,
  PRIMARY KEY (`file_gid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

/*Table structure for table `file_ref` */

DROP TABLE IF EXISTS `file_ref`;

CREATE TABLE `file_ref` (
  `file_gid` bigint unsigned NOT NULL,
  `ref_id` bigint unsigned NOT NULL,
  PRIMARY KEY (`file_gid`,`ref_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

/*Table structure for table `msg_comment` */

DROP TABLE IF EXISTS `msg_comment`;

CREATE TABLE `msg_comment` (
  `message_gid` bigint unsigned NOT NULL,
  `comment_gid` bigint unsigned NOT NULL,
  `pb_data` longblob,
  PRIMARY KEY (`message_gid`,`comment_gid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

/*Table structure for table `msg_like` */

DROP TABLE IF EXISTS `msg_like`;

CREATE TABLE `msg_like` (
  `message_gid` bigint unsigned NOT NULL,
  `profile_cid` bigint unsigned NOT NULL,
  PRIMARY KEY (`message_gid`,`profile_cid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

/*Table structure for table `msg_reshare` */

DROP TABLE IF EXISTS `msg_reshare`;

CREATE TABLE `msg_reshare` (
  `message_gid` bigint unsigned NOT NULL,
  `profile_cid` bigint unsigned NOT NULL,
  PRIMARY KEY (`message_gid`,`profile_cid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

/*Table structure for table `profile` */

DROP TABLE IF EXISTS `profile`;

CREATE TABLE `profile` (
  `profile_cid` bigint unsigned NOT NULL,
  `pb_data` longblob,
  PRIMARY KEY (`profile_cid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

/*Table structure for table `profile_follower` */

DROP TABLE IF EXISTS `profile_follower`;

CREATE TABLE `profile_follower` (
  `profile_cid` bigint unsigned NOT NULL,
  `channel_cid` bigint unsigned NOT NULL,
  `follow_gid` bigint unsigned NOT NULL,
  PRIMARY KEY (`profile_cid`,`channel_cid`,`follow_gid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

/*Table structure for table `profile_subscriber` */

DROP TABLE IF EXISTS `profile_subscriber`;

CREATE TABLE `profile_subscriber` (
  `profile_cid` bigint unsigned NOT NULL,
  `channel_cid` bigint unsigned NOT NULL,
  `subscriber_gid` bigint unsigned NOT NULL,
  PRIMARY KEY (`profile_cid`,`channel_cid`,`subscriber_gid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

/*Table structure for table `tweet` */

DROP TABLE IF EXISTS `tweet`;

CREATE TABLE `tweet` (
  `tweet_id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `created_time` bigint DEFAULT '0',
  `text_body` text,
  PRIMARY KEY (`tweet_id`)
) ENGINE=InnoDB DEFAULT CHARSET=latin1;

/*Table structure for table `user` */

DROP TABLE IF EXISTS `user`;

CREATE TABLE `user` (
  `user_cid` bigint unsigned NOT NULL,
  `pb_data` longblob,
  PRIMARY KEY (`user_cid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

/*!40101 SET SQL_MODE=@OLD_SQL_MODE */;
/*!40014 SET FOREIGN_KEY_CHECKS=@OLD_FOREIGN_KEY_CHECKS */;
/*!40014 SET UNIQUE_CHECKS=@OLD_UNIQUE_CHECKS */;
/*!40111 SET SQL_NOTES=@OLD_SQL_NOTES */;
