//##### New Dump > Changed Manulay

DROP KEYSPACE flip;

CREATE KEYSPACE IF NOT EXISTS flip WITH REPLICATION = { 'class' : 'org.apache.cassandra.locator.SimpleStrategy', 'replication_factor': '1' } AND DURABLE_WRITES = true;

//### Channel
CREATE TABLE IF NOT EXISTS flip.channel (
    channel_cid bigint,
    pb_data blob,
    PRIMARY KEY (channel_cid)
); 

CREATE TABLE IF NOT EXISTS flip.channel_follower (
    channel_cid bigint,
    profile_cid bigint,
    follow_gid bigint,
    PRIMARY KEY (channel_cid, profile_cid, follow_gid)
);

CREATE TABLE IF NOT EXISTS flip.channel_follower_sort (
    channel_cid bigint,
    follow_gid bigint,
    profile_cid bigint,
    PRIMARY KEY (channel_cid,follow_gid, profile_cid)
);

CREATE TABLE IF NOT EXISTS flip.channel_msg (
    channel_cid bigint,
    msg_gid bigint,
    pb_data blob,
    PRIMARY KEY (channel_cid, msg_gid)
);

CREATE TABLE IF NOT EXISTS flip.channel_msg_media (
    channel_cid bigint,
    media_type_id tinyint,
    msg_gid bigint,
    PRIMARY KEY ((channel_cid), media_type_id, msg_gid)
); 

CREATE TABLE IF NOT EXISTS flip.channel_subscriber_bk (
    channel_cid bigint,
    profile_cid bigint,
    PRIMARY KEY (channel_cid, profile_cid)
); 

CREATE TABLE IF NOT EXISTS flip.channel_subscriber (
    channel_cid bigint,
    profile_cid bigint,
    subscriber_gid bigint,
    PRIMARY KEY (channel_cid, profile_cid, subscriber_gid)
);

CREATE TABLE IF NOT EXISTS flip.channel_subscriber_sort (
    channel_cid bigint,
    subscriber_gid bigint,
    profile_cid bigint,
    PRIMARY KEY (channel_cid,subscriber_gid, profile_cid)
);

//### Chat
CREATE TABLE IF NOT EXISTS flip.chat (
    profile_cid bigint,
    chat_gid bigint,
    pb_data blob,
    PRIMARY KEY (profile_cid, chat_gid)
);

CREATE TABLE IF NOT EXISTS flip.chat_msg (
    profile_cid bigint,
    chat_gid bigint,
    msg_gid bigint,
    pb_data blob,
    PRIMARY KEY ((profile_cid,chat_gid), msg_gid)
);

//### Direct
CREATE TABLE IF NOT EXISTS flip.direct (
    profile_cid bigint,
    direct_gid bigint,
    pb_data blob,
    PRIMARY KEY (profile_cid, direct_gid)
);

CREATE TABLE IF NOT EXISTS flip.chat_msg (
    profile_cid bigint,
    direct_gid bigint,
    msg_gid bigint,
    pb_data blob,
    PRIMARY KEY ((profile_cid,direct_gid), msg_gid)
);

//### File
CREATE TABLE IF NOT EXISTS flip.file (
    file_gid bigint,
    pb_data blob,
    PRIMARY KEY (file_gid)
);

CREATE TABLE IF NOT EXISTS flip.file_ref (
    file_gid bigint,
    ref_id bigint,
    PRIMARY KEY (file_gid, ref_id)
);

//### Channel Comment/Like/ReShare
CREATE TABLE IF NOT EXISTS flip.msg_comment (
    message_gid bigint,
    comment_gid bigint,
    pb_data blob,
    PRIMARY KEY (message_gid, comment_gid)
);

CREATE TABLE IF NOT EXISTS flip.msg_like (
    message_gid bigint,
    profile_cid bigint,
    PRIMARY KEY (message_gid, profile_cid)
);

CREATE TABLE IF NOT EXISTS flip.msg_reshare (
    message_gid bigint,
    profile_cid bigint,
    PRIMARY KEY (message_gid, profile_cid)
);

//### Profile
CREATE TABLE IF NOT EXISTS flip.profile (
    profile_cid bigint,
    pb_data blob,
    PRIMARY KEY (profile_cid)
);

CREATE TABLE IF NOT EXISTS flip.profile_follower (
    profile_cid bigint,
    channel_cid bigint,
    follow_gid bigint,
    PRIMARY KEY (profile_cid , channel_cid, follow_gid)
);

CREATE TABLE IF NOT EXISTS flip.profile_follower_sort (
    profile_cid bigint,
    follow_gid bigint,
    channel_cid bigint,
    PRIMARY KEY (profile_cid,follow_gid, channel_cid)
);

CREATE TABLE IF NOT EXISTS flip.profile_subscriber(
    profile_cid bigint,
    channel_cid bigint,
    subscriber_gid bigint,
    PRIMARY KEY (profile_cid , channel_cid, subscriber_gid)
);

CREATE TABLE IF NOT EXISTS flip.profile_subscriber_sort (
    profile_cid bigint,
    subscriber_gid bigint,
    channel_cid bigint,
    PRIMARY KEY (profile_cid,subscriber_gid, channel_cid)
);

//### User
CREATE TABLE IF NOT EXISTS flip.user (
    user_cid bigint,
    pb_data blob,
    PRIMARY KEY (user_cid)
);


//############################################
//################### End of Dump ############
//############################################


//################### Manula #################
//##== Channnels 
CREATE TABLE IF NOT EXISTS msgs.channel_follower (
    channel_cid bigint,
    profile_cid bigint,
    PRIMARY KEY (channel_cid, profile_cid)
);

CREATE TABLE IF NOT EXISTS msgs.channel_subscriber(
    channel_cid bigint,
    profile_cid bigint,
    PRIMARY KEY (channel_cid, profile_cid)
);


CREATE TABLE IF NOT EXISTS msgs.msg_like(
    message_gid bigint,
    profile_cid bigint,
    PRIMARY KEY (message_gid, profile_cid)
);

CREATE TABLE IF NOT EXISTS msgs.msg_reshare(
    message_gid bigint,
    profile_cid bigint,
    PRIMARY KEY (message_gid, pro   file_cid)
);

CREATE TABLE IF NOT EXISTS msgs.msg_comment(
    message_gid bigint,
    comment_gid bigint,
    pb_data blob,
    PRIMARY KEY (message_gid, comment_gid)
);


//##== Profiles 
CREATE TABLE IF NOT EXISTS msgs.profile_follow (
    profile_cid bigint,
    channel_cid bigint,
    PRIMARY KEY (profile_cid,channel_cid)
);

CREATE TABLE IF NOT EXISTS msgs.profile_subscribe (
    profile_cid bigint,
    channel_cid bigint,
    PRIMARY KEY (profile_cid,channel_cid)
);




//################### Output From GUI ################

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
