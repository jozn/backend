################### Manula #################

CREATE TABLE msgs.channel_follower (
    channel_cid bigint,
    profile_cid bigint,
    PRIMARY KEY (channel_cid, profile_cid)
);

CREATE TABLE msgs.channel_subscriber(
    channel_cid bigint,
    profile_cid bigint,
    PRIMARY KEY (channel_cid, profile_cid)
);


CREATE TABLE msgs.msg_like(
    message_gid bigint,
    profile_cid bigint,
    PRIMARY KEY (message_gid, profile_cid)
);

CREATE TABLE msgs.msg_reshare(
    message_gid bigint,
    profile_cid bigint,
    PRIMARY KEY (message_gid, profile_cid)
);

CREATE TABLE msgs.msg_comment(
    message_gid bigint,
    comment_gid bigint,
    pb_data blob,
    PRIMARY KEY (message_gid, comment_gid)
);

################### Output From GUI ################

CREATE KEYSPACE msgs WITH REPLICATION = { 'class' : 'org.apache.cassandra.locator.SimpleStrategy', 'replication_factor': '1' } AND DURABLE_WRITES = true;

CREATE TABLE msgs.channel (
    channel_id bigint,
    pb_data blob,
    PRIMARY KEY (channel_id)
) WITH read_repair_chance = 0.0
    AND dclocal_read_repair_chance = 0.0
    AND gc_grace_seconds = 864000
    AND bloom_filter_fp_chance = 0.01
    AND caching = { 'keys' : 'ALL', 'rows_per_partition' : 'ALL' }
    AND comment = ''
    AND compaction = { 'class' : 'SizeTieredCompactionStrategy' }
    AND compression = { 'sstable_compression' : 'org.apache.cassandra.io.compress.LZ4Compressor' }
    AND default_time_to_live = 0
    AND speculative_retry = '99.0PERCENTILE'
    AND min_index_interval = 128
    AND max_index_interval = 2048
    AND crc_check_chance = 1.0;

CREATE TABLE msgs.channel_msg (
    channel_id bigint,
    msg_id bigint,
    pb_data blob,
    PRIMARY KEY (channel_id, msg_id)
) WITH CLUSTERING ORDER BY (msg_id ASC)
    AND read_repair_chance = 0.0
    AND dclocal_read_repair_chance = 0.0
    AND gc_grace_seconds = 864000
    AND bloom_filter_fp_chance = 0.01
    AND caching = { 'keys' : 'ALL', 'rows_per_partition' : 'ALL' }
    AND comment = ''
    AND compaction = { 'class' : 'SizeTieredCompactionStrategy' }
    AND compression = { 'sstable_compression' : 'org.apache.cassandra.io.compress.LZ4Compressor' }
    AND default_time_to_live = 0
    AND speculative_retry = '99.0PERCENTILE'
    AND min_index_interval = 128
    AND max_index_interval = 2048
    AND crc_check_chance = 1.0;

CREATE TABLE msgs.chat (
    profile_cid bigint,
    chat_id bigint,
    pb_data blob,
    PRIMARY KEY (profile_cid, chat_id)
) WITH CLUSTERING ORDER BY (chat_id ASC)
    AND read_repair_chance = 0.0
    AND dclocal_read_repair_chance = 0.0
    AND gc_grace_seconds = 864000
    AND bloom_filter_fp_chance = 0.01
    AND caching = { 'keys' : 'ALL', 'rows_per_partition' : 'ALL' }
    AND comment = ''
    AND compaction = { 'class' : 'SizeTieredCompactionStrategy' }
    AND compression = { 'sstable_compression' : 'org.apache.cassandra.io.compress.LZ4Compressor' }
    AND default_time_to_live = 0
    AND speculative_retry = '99.0PERCENTILE'
    AND min_index_interval = 128
    AND max_index_interval = 2048
    AND crc_check_chance = 1.0;

CREATE TABLE msgs.chat_msg (
    chat_id bigint,
    msg_id bigint,
    pb_data blob,
    PRIMARY KEY (chat_id, msg_id)
) WITH CLUSTERING ORDER BY (msg_id ASC)
    AND read_repair_chance = 0.0
    AND dclocal_read_repair_chance = 0.0
    AND gc_grace_seconds = 864000
    AND bloom_filter_fp_chance = 0.01
    AND caching = { 'keys' : 'ALL', 'rows_per_partition' : 'ALL' }
    AND comment = ''
    AND compaction = { 'class' : 'SizeTieredCompactionStrategy' }
    AND compression = { 'sstable_compression' : 'org.apache.cassandra.io.compress.LZ4Compressor' }
    AND default_time_to_live = 0
    AND speculative_retry = '99.0PERCENTILE'
    AND min_index_interval = 128
    AND max_index_interval = 2048
    AND crc_check_chance = 1.0;

CREATE TABLE msgs.file (
    file_gid bigint,
    pb_data blob,
    PRIMARY KEY (file_gid)
) WITH read_repair_chance = 0.0
    AND dclocal_read_repair_chance = 0.0
    AND gc_grace_seconds = 864000
    AND bloom_filter_fp_chance = 0.01
    AND caching = { 'keys' : 'ALL', 'rows_per_partition' : 'ALL' }
    AND comment = ''
    AND compaction = { 'class' : 'SizeTieredCompactionStrategy' }
    AND compression = { 'sstable_compression' : 'org.apache.cassandra.io.compress.LZ4Compressor' }
    AND default_time_to_live = 0
    AND speculative_retry = '99.0PERCENTILE'
    AND min_index_interval = 128
    AND max_index_interval = 2048
    AND crc_check_chance = 1.0;

CREATE TABLE msgs.file_ref (
    file_gid bigint,
    ref_id bigint,
    PRIMARY KEY (file_gid, ref_id)
) WITH CLUSTERING ORDER BY (ref_id ASC)
    AND read_repair_chance = 0.0
    AND dclocal_read_repair_chance = 0.0
    AND gc_grace_seconds = 864000
    AND bloom_filter_fp_chance = 0.01
    AND caching = { 'keys' : 'ALL', 'rows_per_partition' : 'ALL' }
    AND comment = ''
    AND compaction = { 'class' : 'SizeTieredCompactionStrategy' }
    AND compression = { 'sstable_compression' : 'org.apache.cassandra.io.compress.LZ4Compressor' }
    AND default_time_to_live = 0
    AND speculative_retry = '99.0PERCENTILE'
    AND min_index_interval = 128
    AND max_index_interval = 2048
    AND crc_check_chance = 1.0;

CREATE TABLE msgs.messages (
    chat_id bigint,
    data blob,
    PRIMARY KEY (chat_id)
) WITH read_repair_chance = 0.0
    AND dclocal_read_repair_chance = 0.0
    AND gc_grace_seconds = 864000
    AND bloom_filter_fp_chance = 0.01
    AND caching = { 'keys' : 'ALL', 'rows_per_partition' : 'ALL' }
    AND comment = ''
    AND compaction = { 'class' : 'SizeTieredCompactionStrategy' }
    AND compression = { 'sstable_compression' : 'org.apache.cassandra.io.compress.LZ4Compressor' }
    AND default_time_to_live = 0
    AND speculative_retry = '99.0PERCENTILE'
    AND min_index_interval = 128
    AND max_index_interval = 2048
    AND crc_check_chance = 1.0;

CREATE TABLE msgs.messages12 (
    chat_id bigint,
    data blob,
    PRIMARY KEY (chat_id)
) WITH read_repair_chance = 0.0
    AND dclocal_read_repair_chance = 0.0
    AND gc_grace_seconds = 864000
    AND bloom_filter_fp_chance = 0.01
    AND caching = { 'keys' : 'ALL', 'rows_per_partition' : 'ALL' }
    AND comment = ''
    AND compaction = { 'class' : 'SizeTieredCompactionStrategy' }
    AND compression = { 'sstable_compression' : 'org.apache.cassandra.io.compress.LZ4Compressor' }
    AND default_time_to_live = 0
    AND speculative_retry = '99.0PERCENTILE'
    AND min_index_interval = 128
    AND max_index_interval = 2048
    AND crc_check_chance = 1.0;

CREATE TABLE msgs.profile (
    profile_cid bigint,
    pb_data blob,
    PRIMARY KEY (profile_cid)
) WITH read_repair_chance = 0.0
    AND dclocal_read_repair_chance = 0.0
    AND gc_grace_seconds = 864000
    AND bloom_filter_fp_chance = 0.01
    AND caching = { 'keys' : 'ALL', 'rows_per_partition' : 'ALL' }
    AND comment = ''
    AND compaction = { 'class' : 'SizeTieredCompactionStrategy' }
    AND compression = { 'sstable_compression' : 'org.apache.cassandra.io.compress.LZ4Compressor' }
    AND default_time_to_live = 0
    AND speculative_retry = '99.0PERCENTILE'
    AND min_index_interval = 128
    AND max_index_interval = 2048
    AND crc_check_chance = 1.0;

CREATE TABLE msgs.user (
    user_cid bigint,
    pb_data blob,
    PRIMARY KEY (user_cid)
) WITH read_repair_chance = 0.0
    AND dclocal_read_repair_chance = 0.0
    AND gc_grace_seconds = 864000
    AND bloom_filter_fp_chance = 0.01
    AND caching = { 'keys' : 'ALL', 'rows_per_partition' : 'ALL' }
    AND comment = ''
    AND compaction = { 'class' : 'SizeTieredCompactionStrategy' }
    AND compression = { 'sstable_compression' : 'org.apache.cassandra.io.compress.LZ4Compressor' }
    AND default_time_to_live = 0
    AND speculative_retry = '99.0PERCENTILE'
    AND min_index_interval = 128
    AND max_index_interval = 2048
    AND crc_check_chance = 1.0;
