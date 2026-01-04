---
title: Migrating Home Assistant from SQLite to MySQL
date: 2026-01-04
---

I wanted to change to MySQL as [recorder](https://www.home-assistant.io/integrations/recorder) backend for my Home Assistant instance, but it was not as easy as I hoped it would be. Home Assistant does not provide any migrationg tools, so I had to fiddle around to move my data from SQLite to MySQL.

## Useful commands

When working with MySQL, you probably want to disable `foreign_key_checks`, so you can delete or insert data without caring about the order of the tables.

```bash
# interactive shell
mysql -h 127.0.0.1 -u homeassistant -p homeassistant --init-command="set session foreign_key_checks=0"

# truncate a table, incase you need to import again
mysql -h 127.0.0.1 -u homeassistant -p homeassistant --init-command="set session foreign_key_checks=0" -e "truncate table states"
```

The following two commands can be used to inspect the database schemas in both SQLite and MySQL. It will come in handy, when you need to verify that the order of the columns are correct.

```bash
cd /path/to/homeassistant/config;
sqlite3 home-assistant_v2.db ".schema statistics_meta"
mysql -h 127.0.0.1 -u homeassistant -p homeassistant -e "show create table statistics_meta"
```

Note that the commands need to be updated accordingly to you setup:

* `-h 127.0.0.1` - The MySQL IP / hostname
* `-u homeassistant` - The MySQL username
* `-p` - Can be changed to `-pMYSECRETPASSWORD` if you don't want to enter your password every time.
* `homeassistant` - Database name

## Preparing MySQL

The installation process for MySQL will not be covered here. Only the setup of the database.

```sql
-- IMPORTANT: Change MYSECRETPASSWORD to something secret
create database homeassistant;
create user 'homeassistant'@'127.0.0.1' identified by 'mysecretpassword';
grant all privileges on homeassistant.* to 'homeassistant'@'127.0.0.1';
flush privileges;
```

## Fixing MySQL database schemas

When I first started migrating the data, I got a lot of `ERROR 1406 Data too long for column ...` messages. This took me down a rabbit hole where I thought columns with [char(0)](https://dev.mysql.com/doc/refman/8.4/en/char.html) was causing the issues, but the real reason was simply that the columns in the table definitions was in a different order in MySQL, which again resulted in the data from the `INSERT INTO ...` statements to be (not) inserted into the wrong column.q

I decided to create a fresh schema, instead of trying to guess how they should look from the sqlite dump. To create the schemas I ran:


```bash
sudo systemctl stop home-assistant.service;
cd /path/to/homeassistant/;
rsync -va config/ backup/;
mkdir config;
cat - > config/configuration.yaml;
recorder:
  db_url: mysql://homeassistant:SECRETPASSWORD@127.0.0.1/homeassistant?charset=utf8mb4
  purge_keep_days: 31
^C

sudo systemctl start home-assistant.service;
# wait for the tables to be created...
sudo systemctl stop home-assistant.service;
```

After comparing the SQLite and MySQL tables, I found that I needed to make these changes:

```sql
alter table states modify column last_reported_ts double default null after metadata_id;
alter table statistics modify column `mean_weight` double default null after sum;
alter table statistics_short_term modify column `mean_weight` double default null after sum;
alter table statistics_meta modify column unit_class varchar(255) default null after mean_type;
```

## Migrating from SQLite

After the schemas was fixed, I ran the commands below to dump SQLite and then import the data back into MySQL. Note that this can take several minutes, depending on how much data there is!

```bash
cd backup;
# dump sqlite into one file per table
sqlite3 home-assistant_v2.db .dump \
  | perl -ne 'BEGIN{open $FH{_},">","misc.sql"||die}if(/^INSERT INTO `?(\w+)/){open $FH{$1},">","ha_$1.sql"||die unless $FH{$1};print {$FH{$1}} $_}else{print {$FH{_}} $_}';

# This table does not exist in MySQL:
mv ha_sqlite_stat1.sql sqlite_stat1.sql

# import the files back into mysql
for i in ha_*.sql; do echo "# $i"; mysql -h 127.0.0.1 -u homeassistant -p --init-command="set session foreign_key_checks=0" homeassistant < "$i" && mv "$i" "done_$i" || break; done
```

If the import fails, then it's probably because the order of the columns are wrong. If that's the case, then alter the table that fails and run the `for`-loop again. It should eventually work.

After the migration is done, I could move back my config, update the `recorder` setting and start Home Assistant again:

```bash
rsync -va backup/ config/;
rm config/home-assistant_v2.db;
# Add the "recorder" config
$EDITOR config/configuration.yaml;
sudo systemctl start home-assistant.service;
```

## Conclusion

Migrating from SQLite to MySQL is not easy, but it was worth the trouble, since I didn't want to loose out on my power and glucose data.

## Appendix

### Truncate tables

```sql
set foreign_key_checks = 0;
truncate table event_data;
truncate table event_types;
truncate table events;
truncate table migration_changes;
truncate table recorder_runs;
truncate table schema_changes;
truncate table state_attributes;
truncate table states;
truncate table states_meta;
truncate table statistics;
truncate table statistics_meta;
truncate table statistics_runs;
truncate table statistics_short_term;
```

### MySQL database schema

```sql
-- mysqldump --compact --no-data -h 127.0.0.1 -u homeassistant -p homeassistant
/*M!999999\- enable the sandbox mode */
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `event_data` (
  `data_id` bigint(20) NOT NULL AUTO_INCREMENT,
  `hash` int(10) unsigned DEFAULT NULL,
  `shared_data` longtext DEFAULT NULL,
  PRIMARY KEY (`data_id`),
  KEY `ix_event_data_hash` (`hash`)
) ENGINE=InnoDB AUTO_INCREMENT=3012261 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin;
/*!40101 SET character_set_client = @saved_cs_client */;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `event_types` (
  `event_type_id` bigint(20) NOT NULL AUTO_INCREMENT,
  `event_type` varchar(64) DEFAULT NULL,
  PRIMARY KEY (`event_type_id`),
  UNIQUE KEY `ix_event_types_event_type` (`event_type`)
) ENGINE=InnoDB AUTO_INCREMENT=216 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin;
/*!40101 SET character_set_client = @saved_cs_client */;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `events` (
  `event_id` bigint(20) NOT NULL AUTO_INCREMENT,
  `event_type` char(0) DEFAULT NULL,
  `event_data` char(0) DEFAULT NULL,
  `origin` char(0) DEFAULT NULL,
  `origin_idx` smallint(6) DEFAULT NULL,
  `time_fired` char(0) DEFAULT NULL,
  `time_fired_ts` double DEFAULT NULL,
  `context_id` char(0) DEFAULT NULL,
  `context_user_id` char(0) DEFAULT NULL,
  `context_parent_id` char(0) DEFAULT NULL,
  `data_id` bigint(20) DEFAULT NULL,
  `context_id_bin` tinyblob DEFAULT NULL,
  `context_user_id_bin` tinyblob DEFAULT NULL,
  `context_parent_id_bin` tinyblob DEFAULT NULL,
  `event_type_id` bigint(20) DEFAULT NULL,
  PRIMARY KEY (`event_id`),
  KEY `ix_events_time_fired_ts` (`time_fired_ts`),
  KEY `ix_events_context_id_bin` (`context_id_bin`(16)),
  KEY `ix_events_event_type_id_time_fired_ts` (`event_type_id`,`time_fired_ts`),
  KEY `ix_events_data_id` (`data_id`),
  CONSTRAINT `events_ibfk_1` FOREIGN KEY (`data_id`) REFERENCES `event_data` (`data_id`),
  CONSTRAINT `events_ibfk_2` FOREIGN KEY (`event_type_id`) REFERENCES `event_types` (`event_type_id`)
) ENGINE=InnoDB AUTO_INCREMENT=8981745 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin;
/*!40101 SET character_set_client = @saved_cs_client */;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `migration_changes` (
  `migration_id` varchar(255) NOT NULL,
  `version` smallint(6) NOT NULL,
  PRIMARY KEY (`migration_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin;
/*!40101 SET character_set_client = @saved_cs_client */;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `recorder_runs` (
  `run_id` bigint(20) NOT NULL AUTO_INCREMENT,
  `start` datetime(6) NOT NULL,
  `end` datetime(6) DEFAULT NULL,
  `closed_incorrect` tinyint(1) NOT NULL,
  `created` datetime(6) NOT NULL,
  PRIMARY KEY (`run_id`),
  KEY `ix_recorder_runs_start_end` (`start`,`end`)
) ENGINE=InnoDB AUTO_INCREMENT=548 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin;
/*!40101 SET character_set_client = @saved_cs_client */;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `schema_changes` (
  `change_id` bigint(20) NOT NULL AUTO_INCREMENT,
  `schema_version` int(11) DEFAULT NULL,
  `changed` datetime(6) NOT NULL,
  PRIMARY KEY (`change_id`)
) ENGINE=InnoDB AUTO_INCREMENT=14 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin;
/*!40101 SET character_set_client = @saved_cs_client */;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `state_attributes` (
  `attributes_id` bigint(20) NOT NULL AUTO_INCREMENT,
  `hash` int(10) unsigned DEFAULT NULL,
  `shared_attrs` longtext DEFAULT NULL,
  PRIMARY KEY (`attributes_id`),
  KEY `ix_state_attributes_hash` (`hash`)
) ENGINE=InnoDB AUTO_INCREMENT=2993969 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin;
/*!40101 SET character_set_client = @saved_cs_client */;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `states` (
  `state_id` bigint(20) NOT NULL AUTO_INCREMENT,
  `entity_id` char(0) DEFAULT NULL,
  `state` varchar(255) DEFAULT NULL,
  `attributes` char(0) DEFAULT NULL,
  `event_id` smallint(6) DEFAULT NULL,
  `last_changed` char(0) DEFAULT NULL,
  `last_changed_ts` double DEFAULT NULL,
  `last_updated` char(0) DEFAULT NULL,
  `last_updated_ts` double DEFAULT NULL,
  `old_state_id` bigint(20) DEFAULT NULL,
  `attributes_id` bigint(20) DEFAULT NULL,
  `context_id` char(0) DEFAULT NULL,
  `context_user_id` char(0) DEFAULT NULL,
  `context_parent_id` char(0) DEFAULT NULL,
  `origin_idx` smallint(6) DEFAULT NULL,
  `context_id_bin` tinyblob DEFAULT NULL,
  `context_user_id_bin` tinyblob DEFAULT NULL,
  `context_parent_id_bin` tinyblob DEFAULT NULL,
  `metadata_id` bigint(20) DEFAULT NULL,
  `last_reported_ts` double DEFAULT NULL,
  PRIMARY KEY (`state_id`),
  KEY `ix_states_last_updated_ts` (`last_updated_ts`),
  KEY `ix_states_old_state_id` (`old_state_id`),
  KEY `ix_states_metadata_id_last_updated_ts` (`metadata_id`,`last_updated_ts`),
  KEY `ix_states_attributes_id` (`attributes_id`),
  KEY `ix_states_context_id_bin` (`context_id_bin`(16)),
  CONSTRAINT `states_ibfk_1` FOREIGN KEY (`old_state_id`) REFERENCES `states` (`state_id`),
  CONSTRAINT `states_ibfk_2` FOREIGN KEY (`attributes_id`) REFERENCES `state_attributes` (`attributes_id`),
  CONSTRAINT `states_ibfk_3` FOREIGN KEY (`metadata_id`) REFERENCES `states_meta` (`metadata_id`)
) ENGINE=InnoDB AUTO_INCREMENT=133774223 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin;
/*!40101 SET character_set_client = @saved_cs_client */;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `states_meta` (
  `metadata_id` bigint(20) NOT NULL AUTO_INCREMENT,
  `entity_id` varchar(255) DEFAULT NULL,
  PRIMARY KEY (`metadata_id`),
  UNIQUE KEY `ix_states_meta_entity_id` (`entity_id`)
) ENGINE=InnoDB AUTO_INCREMENT=4566 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin;
/*!40101 SET character_set_client = @saved_cs_client */;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `statistics` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT,
  `created` char(0) DEFAULT NULL,
  `created_ts` double DEFAULT NULL,
  `metadata_id` bigint(20) DEFAULT NULL,
  `start` char(0) DEFAULT NULL,
  `start_ts` double DEFAULT NULL,
  `mean` double DEFAULT NULL,
  `min` double DEFAULT NULL,
  `max` double DEFAULT NULL,
  `last_reset` char(0) DEFAULT NULL,
  `last_reset_ts` double DEFAULT NULL,
  `state` double DEFAULT NULL,
  `sum` double DEFAULT NULL,
  `mean_weight` double DEFAULT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `ix_statistics_statistic_id_start_ts` (`metadata_id`,`start_ts`),
  KEY `ix_statistics_start_ts` (`start_ts`),
  CONSTRAINT `statistics_ibfk_1` FOREIGN KEY (`metadata_id`) REFERENCES `statistics_meta` (`id`) ON DELETE CASCADE
) ENGINE=InnoDB AUTO_INCREMENT=1424122 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin;
/*!40101 SET character_set_client = @saved_cs_client */;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `statistics_meta` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT,
  `statistic_id` varchar(255) DEFAULT NULL,
  `source` varchar(32) DEFAULT NULL,
  `unit_of_measurement` varchar(255) DEFAULT NULL,
  `has_mean` tinyint(1) DEFAULT NULL,
  `has_sum` tinyint(1) DEFAULT NULL,
  `name` varchar(255) DEFAULT NULL,
  `mean_type` smallint(6) NOT NULL,
  `unit_class` varchar(255) DEFAULT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `ix_statistics_meta_statistic_id` (`statistic_id`)
) ENGINE=InnoDB AUTO_INCREMENT=419 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin;
/*!40101 SET character_set_client = @saved_cs_client */;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `statistics_runs` (
  `run_id` bigint(20) NOT NULL AUTO_INCREMENT,
  `start` datetime(6) NOT NULL,
  PRIMARY KEY (`run_id`),
  KEY `ix_statistics_runs_start` (`start`)
) ENGINE=InnoDB AUTO_INCREMENT=239350 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin;
/*!40101 SET character_set_client = @saved_cs_client */;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `statistics_short_term` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT,
  `created` char(0) DEFAULT NULL,
  `created_ts` double DEFAULT NULL,
  `metadata_id` bigint(20) DEFAULT NULL,
  `start` char(0) DEFAULT NULL,
  `start_ts` double DEFAULT NULL,
  `mean` double DEFAULT NULL,
  `min` double DEFAULT NULL,
  `max` double DEFAULT NULL,
  `last_reset` char(0) DEFAULT NULL,
  `last_reset_ts` double DEFAULT NULL,
  `state` double DEFAULT NULL,
  `sum` double DEFAULT NULL,
  `mean_weight` double DEFAULT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `ix_statistics_short_term_statistic_id_start_ts` (`metadata_id`,`start_ts`),
  KEY `ix_statistics_short_term_start_ts` (`start_ts`),
  CONSTRAINT `statistics_short_term_ibfk_1` FOREIGN KEY (`metadata_id`) REFERENCES `statistics_meta` (`id`) ON DELETE CASCADE
) ENGINE=InnoDB AUTO_INCREMENT=17038233 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin;
/*!40101 SET character_set_client = @saved_cs_client */;
```
