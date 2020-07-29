package ir.ms.pb;

public class PBFlatTypes {

	public class RoomMessageLocation {
	   public double lat;
	   public double lon;
	}
	/*
	folding
	PBFlatTypes.RoomMessageLocation t = new PBFlatTypes.RoomMessageLocation();
    t.setlat();
    t.setlon();
	*/

	/*
	PBFlatTypes.RoomMessageLocation t = new PBFlatTypes.RoomMessageLocation();
	t.lat = ;
	t.lon = ;
	*/

	/*
	RoomMessageLocation t = new RoomMessageLocation();
	t.lat = m.getlat() ;
	t.lon = m.getlon() ;
	*/

	public class RoomMessageLog {
	   public Type type;
	   public ExtraType extra_type;
	   public TargetUser target_user;
	}
	/*
	folding
	PBFlatTypes.RoomMessageLog t = new PBFlatTypes.RoomMessageLog();
    t.settype();
    t.setextra_type();
    t.settarget_user();
	*/

	/*
	PBFlatTypes.RoomMessageLog t = new PBFlatTypes.RoomMessageLog();
	t.type = ;
	t.extra_type = ;
	t.target_user = ;
	*/

	/*
	RoomMessageLog t = new RoomMessageLog();
	t.type = m.gettype() ;
	t.extra_type = m.getextra_type() ;
	t.target_user = m.gettarget_user() ;
	*/

	public class RoomMessageContact {
	   public String first_name;
	   public String last_name;
	   public String nickname;
	   public String phone;
	   public String email;
	}
	/*
	folding
	PBFlatTypes.RoomMessageContact t = new PBFlatTypes.RoomMessageContact();
    t.setfirst_name();
    t.setlast_name();
    t.setnickname();
    t.setphone();
    t.setemail();
	*/

	/*
	PBFlatTypes.RoomMessageContact t = new PBFlatTypes.RoomMessageContact();
	t.first_name = ;
	t.last_name = ;
	t.nickname = ;
	t.phone = ;
	t.email = ;
	*/

	/*
	RoomMessageContact t = new RoomMessageContact();
	t.first_name = m.getfirst_name() ;
	t.last_name = m.getlast_name() ;
	t.nickname = m.getnickname() ;
	t.phone = m.getphone() ;
	t.email = m.getemail() ;
	*/

	public class RoomMessageWallet {
	   public Type type;
	   public MoneyTransfer money_transfer;
	}
	/*
	folding
	PBFlatTypes.RoomMessageWallet t = new PBFlatTypes.RoomMessageWallet();
    t.settype();
    t.setmoney_transfer();
	*/

	/*
	PBFlatTypes.RoomMessageWallet t = new PBFlatTypes.RoomMessageWallet();
	t.type = ;
	t.money_transfer = ;
	*/

	/*
	RoomMessageWallet t = new RoomMessageWallet();
	t.type = m.gettype() ;
	t.money_transfer = m.getmoney_transfer() ;
	*/

	public class RoomMessageForwardFrom {
	   public long room_id;
	   public long message_id;
	}
	/*
	folding
	PBFlatTypes.RoomMessageForwardFrom t = new PBFlatTypes.RoomMessageForwardFrom();
    t.setroom_id();
    t.setmessage_id();
	*/

	/*
	PBFlatTypes.RoomMessageForwardFrom t = new PBFlatTypes.RoomMessageForwardFrom();
	t.room_id = ;
	t.message_id = ;
	*/

	/*
	RoomMessageForwardFrom t = new RoomMessageForwardFrom();
	t.room_id = m.getroom_id() ;
	t.message_id = m.getmessage_id() ;
	*/

	public class RegisteredUser {
	   public long id;
	   public String username;
	   public long phone;
	   public String first_name;
	   public String last_name;
	   public String display_name;
	   public String initials;
	   public String color;
	   public Status status;
	   public int last_seen;
	   public int avatar_count;
	   public Avatar avatar;
	   public boolean mutual;
	   public boolean deleted;
	   public String cache_id;
	   public String bio;
	   public boolean verified;
	   public boolean bot;
	}
	/*
	folding
	PBFlatTypes.RegisteredUser t = new PBFlatTypes.RegisteredUser();
    t.setid();
    t.setusername();
    t.setphone();
    t.setfirst_name();
    t.setlast_name();
    t.setdisplay_name();
    t.setinitials();
    t.setcolor();
    t.setstatus();
    t.setlast_seen();
    t.setavatar_count();
    t.setavatar();
    t.setmutual();
    t.setdeleted();
    t.setcache_id();
    t.setbio();
    t.setverified();
    t.setbot();
	*/

	/*
	PBFlatTypes.RegisteredUser t = new PBFlatTypes.RegisteredUser();
	t.id = ;
	t.username = ;
	t.phone = ;
	t.first_name = ;
	t.last_name = ;
	t.display_name = ;
	t.initials = ;
	t.color = ;
	t.status = ;
	t.last_seen = ;
	t.avatar_count = ;
	t.avatar = ;
	t.mutual = ;
	t.deleted = ;
	t.cache_id = ;
	t.bio = ;
	t.verified = ;
	t.bot = ;
	*/

	/*
	RegisteredUser t = new RegisteredUser();
	t.id = m.getid() ;
	t.username = m.getusername() ;
	t.phone = m.getphone() ;
	t.first_name = m.getfirst_name() ;
	t.last_name = m.getlast_name() ;
	t.display_name = m.getdisplay_name() ;
	t.initials = m.getinitials() ;
	t.color = m.getcolor() ;
	t.status = m.getstatus() ;
	t.last_seen = m.getlast_seen() ;
	t.avatar_count = m.getavatar_count() ;
	t.avatar = m.getavatar() ;
	t.mutual = m.getmutual() ;
	t.deleted = m.getdeleted() ;
	t.cache_id = m.getcache_id() ;
	t.bio = m.getbio() ;
	t.verified = m.getverified() ;
	t.bot = m.getbot() ;
	*/

	public class Avatar {
	   public long id;
	   public File file;
	}
	/*
	folding
	PBFlatTypes.Avatar t = new PBFlatTypes.Avatar();
    t.setid();
    t.setfile();
	*/

	/*
	PBFlatTypes.Avatar t = new PBFlatTypes.Avatar();
	t.id = ;
	t.file = ;
	*/

	/*
	Avatar t = new Avatar();
	t.id = m.getid() ;
	t.file = m.getfile() ;
	*/

	public class RoomMessage {
	   public long message_id;
	   public long message_version;
	   public RoomMessageStatus status;
	   public long status_version;
	   public RoomMessageType message_type;
	   public String message;
	   public File attachment;
	   public Author author;
	   public RoomMessageLocation location;
	   public RoomMessageLog log;
	   public RoomMessageContact contact;
	   public RoomMessageWallet wallet;
	   public boolean edited;
	   public int create_time;
	   public int update_time;
	   public boolean deleted;
	   public RoomMessage forward_from;
	   public RoomMessage reply_to;
	   public long previous_message_id;
	   public long random_id;
	   public int additional_type;
	   public String additional_data;
	   public ExtraType extra_type;
	   public ChannelExtra channel_extra;
	}
	/*
	folding
	PBFlatTypes.RoomMessage t = new PBFlatTypes.RoomMessage();
    t.setmessage_id();
    t.setmessage_version();
    t.setstatus();
    t.setstatus_version();
    t.setmessage_type();
    t.setmessage();
    t.setattachment();
    t.setauthor();
    t.setlocation();
    t.setlog();
    t.setcontact();
    t.setwallet();
    t.setedited();
    t.setcreate_time();
    t.setupdate_time();
    t.setdeleted();
    t.setforward_from();
    t.setreply_to();
    t.setprevious_message_id();
    t.setrandom_id();
    t.setadditional_type();
    t.setadditional_data();
    t.setextra_type();
    t.setchannel_extra();
	*/

	/*
	PBFlatTypes.RoomMessage t = new PBFlatTypes.RoomMessage();
	t.message_id = ;
	t.message_version = ;
	t.status = ;
	t.status_version = ;
	t.message_type = ;
	t.message = ;
	t.attachment = ;
	t.author = ;
	t.location = ;
	t.log = ;
	t.contact = ;
	t.wallet = ;
	t.edited = ;
	t.create_time = ;
	t.update_time = ;
	t.deleted = ;
	t.forward_from = ;
	t.reply_to = ;
	t.previous_message_id = ;
	t.random_id = ;
	t.additional_type = ;
	t.additional_data = ;
	t.extra_type = ;
	t.channel_extra = ;
	*/

	/*
	RoomMessage t = new RoomMessage();
	t.message_id = m.getmessage_id() ;
	t.message_version = m.getmessage_version() ;
	t.status = m.getstatus() ;
	t.status_version = m.getstatus_version() ;
	t.message_type = m.getmessage_type() ;
	t.message = m.getmessage() ;
	t.attachment = m.getattachment() ;
	t.author = m.getauthor() ;
	t.location = m.getlocation() ;
	t.log = m.getlog() ;
	t.contact = m.getcontact() ;
	t.wallet = m.getwallet() ;
	t.edited = m.getedited() ;
	t.create_time = m.getcreate_time() ;
	t.update_time = m.getupdate_time() ;
	t.deleted = m.getdeleted() ;
	t.forward_from = m.getforward_from() ;
	t.reply_to = m.getreply_to() ;
	t.previous_message_id = m.getprevious_message_id() ;
	t.random_id = m.getrandom_id() ;
	t.additional_type = m.getadditional_type() ;
	t.additional_data = m.getadditional_data() ;
	t.extra_type = m.getextra_type() ;
	t.channel_extra = m.getchannel_extra() ;
	*/

	public class RoomDraft {
	   public String message;
	   public long reply_to;
	}
	/*
	folding
	PBFlatTypes.RoomDraft t = new PBFlatTypes.RoomDraft();
    t.setmessage();
    t.setreply_to();
	*/

	/*
	PBFlatTypes.RoomDraft t = new PBFlatTypes.RoomDraft();
	t.message = ;
	t.reply_to = ;
	*/

	/*
	RoomDraft t = new RoomDraft();
	t.message = m.getmessage() ;
	t.reply_to = m.getreply_to() ;
	*/

	public class Room {
	   public long id;
	   public Type type;
	   public String title;
	   public String initials;
	   public String color;
	   public int unread_count;
	   public RoomMessage last_message;
	   public boolean read_only;
	   public boolean is_participant;
	   public RoomDraft draft;
	   public RoomMessage first_unread_message;
	   public RoomMute room_mute;
	   public long pin_id;
	   public RoomMessage pinned_message;
	   public int priority;
	   public ChatRoom chat_room_extra;
	   public GroupRoom group_room_extra;
	   public ChannelRoom channel_room_extra;
	}
	/*
	folding
	PBFlatTypes.Room t = new PBFlatTypes.Room();
    t.setid();
    t.settype();
    t.settitle();
    t.setinitials();
    t.setcolor();
    t.setunread_count();
    t.setlast_message();
    t.setread_only();
    t.setis_participant();
    t.setdraft();
    t.setfirst_unread_message();
    t.setroom_mute();
    t.setpin_id();
    t.setpinned_message();
    t.setpriority();
    t.setchat_room_extra();
    t.setgroup_room_extra();
    t.setchannel_room_extra();
	*/

	/*
	PBFlatTypes.Room t = new PBFlatTypes.Room();
	t.id = ;
	t.type = ;
	t.title = ;
	t.initials = ;
	t.color = ;
	t.unread_count = ;
	t.last_message = ;
	t.read_only = ;
	t.is_participant = ;
	t.draft = ;
	t.first_unread_message = ;
	t.room_mute = ;
	t.pin_id = ;
	t.pinned_message = ;
	t.priority = ;
	t.chat_room_extra = ;
	t.group_room_extra = ;
	t.channel_room_extra = ;
	*/

	/*
	Room t = new Room();
	t.id = m.getid() ;
	t.type = m.gettype() ;
	t.title = m.gettitle() ;
	t.initials = m.getinitials() ;
	t.color = m.getcolor() ;
	t.unread_count = m.getunread_count() ;
	t.last_message = m.getlast_message() ;
	t.read_only = m.getread_only() ;
	t.is_participant = m.getis_participant() ;
	t.draft = m.getdraft() ;
	t.first_unread_message = m.getfirst_unread_message() ;
	t.room_mute = m.getroom_mute() ;
	t.pin_id = m.getpin_id() ;
	t.pinned_message = m.getpinned_message() ;
	t.priority = m.getpriority() ;
	t.chat_room_extra = m.getchat_room_extra() ;
	t.group_room_extra = m.getgroup_room_extra() ;
	t.channel_room_extra = m.getchannel_room_extra() ;
	*/

	public class ChatRoom {
	   public RegisteredUser peer;
	}
	/*
	folding
	PBFlatTypes.ChatRoom t = new PBFlatTypes.ChatRoom();
    t.setpeer();
	*/

	/*
	PBFlatTypes.ChatRoom t = new PBFlatTypes.ChatRoom();
	t.peer = ;
	*/

	/*
	ChatRoom t = new ChatRoom();
	t.peer = m.getpeer() ;
	*/

	public class GroupRoom {
	   public Type type;
	   public Role role;
	   public int participants_count;
	   public String participants_count_label;
	   public int participants_count_limit;
	   public String participants_count_limit_label;
	   public String description;
	   public int avatar_count;
	   public Avatar avatar;
	   public PrivateExtra private_extra;
	   public PublicExtra public_extra;
	}
	/*
	folding
	PBFlatTypes.GroupRoom t = new PBFlatTypes.GroupRoom();
    t.settype();
    t.setrole();
    t.setparticipants_count();
    t.setparticipants_count_label();
    t.setparticipants_count_limit();
    t.setparticipants_count_limit_label();
    t.setdescription();
    t.setavatar_count();
    t.setavatar();
    t.setprivate_extra();
    t.setpublic_extra();
	*/

	/*
	PBFlatTypes.GroupRoom t = new PBFlatTypes.GroupRoom();
	t.type = ;
	t.role = ;
	t.participants_count = ;
	t.participants_count_label = ;
	t.participants_count_limit = ;
	t.participants_count_limit_label = ;
	t.description = ;
	t.avatar_count = ;
	t.avatar = ;
	t.private_extra = ;
	t.public_extra = ;
	*/

	/*
	GroupRoom t = new GroupRoom();
	t.type = m.gettype() ;
	t.role = m.getrole() ;
	t.participants_count = m.getparticipants_count() ;
	t.participants_count_label = m.getparticipants_count_label() ;
	t.participants_count_limit = m.getparticipants_count_limit() ;
	t.participants_count_limit_label = m.getparticipants_count_limit_label() ;
	t.description = m.getdescription() ;
	t.avatar_count = m.getavatar_count() ;
	t.avatar = m.getavatar() ;
	t.private_extra = m.getprivate_extra() ;
	t.public_extra = m.getpublic_extra() ;
	*/

	public class ChannelRoom {
	   public Type type;
	   public Role role;
	   public int participants_count;
	   public String participants_count_label;
	   public String description;
	   public int avatar_count;
	   public Avatar avatar;
	   public PrivateExtra private_extra;
	   public PublicExtra public_extra;
	   public boolean signature;
	   public long seen_id;
	   public boolean verified;
	   public boolean reaction_status;
	}
	/*
	folding
	PBFlatTypes.ChannelRoom t = new PBFlatTypes.ChannelRoom();
    t.settype();
    t.setrole();
    t.setparticipants_count();
    t.setparticipants_count_label();
    t.setdescription();
    t.setavatar_count();
    t.setavatar();
    t.setprivate_extra();
    t.setpublic_extra();
    t.setsignature();
    t.setseen_id();
    t.setverified();
    t.setreaction_status();
	*/

	/*
	PBFlatTypes.ChannelRoom t = new PBFlatTypes.ChannelRoom();
	t.type = ;
	t.role = ;
	t.participants_count = ;
	t.participants_count_label = ;
	t.description = ;
	t.avatar_count = ;
	t.avatar = ;
	t.private_extra = ;
	t.public_extra = ;
	t.signature = ;
	t.seen_id = ;
	t.verified = ;
	t.reaction_status = ;
	*/

	/*
	ChannelRoom t = new ChannelRoom();
	t.type = m.gettype() ;
	t.role = m.getrole() ;
	t.participants_count = m.getparticipants_count() ;
	t.participants_count_label = m.getparticipants_count_label() ;
	t.description = m.getdescription() ;
	t.avatar_count = m.getavatar_count() ;
	t.avatar = m.getavatar() ;
	t.private_extra = m.getprivate_extra() ;
	t.public_extra = m.getpublic_extra() ;
	t.signature = m.getsignature() ;
	t.seen_id = m.getseen_id() ;
	t.verified = m.getverified() ;
	t.reaction_status = m.getreaction_status() ;
	*/

	public class Thumbnail {
	   public long size;
	   public int width;
	   public int height;
	   public String cache_id;
	   public String name;
	   public String mime;
	}
	/*
	folding
	PBFlatTypes.Thumbnail t = new PBFlatTypes.Thumbnail();
    t.setsize();
    t.setwidth();
    t.setheight();
    t.setcache_id();
    t.setname();
    t.setmime();
	*/

	/*
	PBFlatTypes.Thumbnail t = new PBFlatTypes.Thumbnail();
	t.size = ;
	t.width = ;
	t.height = ;
	t.cache_id = ;
	t.name = ;
	t.mime = ;
	*/

	/*
	Thumbnail t = new Thumbnail();
	t.size = m.getsize() ;
	t.width = m.getwidth() ;
	t.height = m.getheight() ;
	t.cache_id = m.getcache_id() ;
	t.name = m.getname() ;
	t.mime = m.getmime() ;
	*/

	public class File {
	   public String token;
	   public String name;
	   public long size;
	   public Thumbnail large_thumbnail;
	   public Thumbnail small_thumbnail;
	   public Thumbnail waveform_thumbnail;
	   public int width;
	   public int height;
	   public double duration;
	   public String cache_id;
	   public String mime;
	   public String public_url;
	}
	/*
	folding
	PBFlatTypes.File t = new PBFlatTypes.File();
    t.settoken();
    t.setname();
    t.setsize();
    t.setlarge_thumbnail();
    t.setsmall_thumbnail();
    t.setwaveform_thumbnail();
    t.setwidth();
    t.setheight();
    t.setduration();
    t.setcache_id();
    t.setmime();
    t.setpublic_url();
	*/

	/*
	PBFlatTypes.File t = new PBFlatTypes.File();
	t.token = ;
	t.name = ;
	t.size = ;
	t.large_thumbnail = ;
	t.small_thumbnail = ;
	t.waveform_thumbnail = ;
	t.width = ;
	t.height = ;
	t.duration = ;
	t.cache_id = ;
	t.mime = ;
	t.public_url = ;
	*/

	/*
	File t = new File();
	t.token = m.gettoken() ;
	t.name = m.getname() ;
	t.size = m.getsize() ;
	t.large_thumbnail = m.getlarge_thumbnail() ;
	t.small_thumbnail = m.getsmall_thumbnail() ;
	t.waveform_thumbnail = m.getwaveform_thumbnail() ;
	t.width = m.getwidth() ;
	t.height = m.getheight() ;
	t.duration = m.getduration() ;
	t.cache_id = m.getcache_id() ;
	t.mime = m.getmime() ;
	t.public_url = m.getpublic_url() ;
	*/

	public class Wallpaper {
	   public File file;
	   public String color;
	}
	/*
	folding
	PBFlatTypes.Wallpaper t = new PBFlatTypes.Wallpaper();
    t.setfile();
    t.setcolor();
	*/

	/*
	PBFlatTypes.Wallpaper t = new PBFlatTypes.Wallpaper();
	t.file = ;
	t.color = ;
	*/

	/*
	Wallpaper t = new Wallpaper();
	t.file = m.getfile() ;
	t.color = m.getcolor() ;
	*/

	public class Pagination {
	   public int offset;
	   public int limit;
	}
	/*
	folding
	PBFlatTypes.Pagination t = new PBFlatTypes.Pagination();
    t.setoffset();
    t.setlimit();
	*/

	/*
	PBFlatTypes.Pagination t = new PBFlatTypes.Pagination();
	t.offset = ;
	t.limit = ;
	*/

	/*
	Pagination t = new Pagination();
	t.offset = m.getoffset() ;
	t.limit = m.getlimit() ;
	*/

	
}

/*

	
*/