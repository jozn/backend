package x

import "ms/sun/shared/helper"


type RoomMessageLocation_Flat struct {
    lat float64
    lon float64
}
//ToPB
func(m *RoomMessageLocation)ToFlat() *RoomMessageLocation_Flat {
r := &RoomMessageLocation_Flat{
    lat:  float64(m.lat) ,
    lon:  float64(m.lon) ,
}
return r
}
//ToPB
func(m *RoomMessageLocation_Flat)ToPB() *RoomMessageLocation {
r := &RoomMessageLocation{
    lat:  m.lat ,
    lon:  m.lon ,
}
return r
}
//folding
var RoomMessageLocation__FOlD = &RoomMessageLocation{
    lat:  0.0 ,
    lon:  0.0 ,
}


type RoomMessageLog_Flat struct {
    type Type
    extra_type ExtraType
    target_user TargetUser
}
//ToPB
func(m *RoomMessageLog)ToFlat() *RoomMessageLog_Flat {
r := &RoomMessageLog_Flat{
    
    
    
}
return r
}
//ToPB
func(m *RoomMessageLog_Flat)ToPB() *RoomMessageLog {
r := &RoomMessageLog{
    
    
    
}
return r
}
//folding
var RoomMessageLog__FOlD = &RoomMessageLog{
    
    
    
}


type RoomMessageContact_Flat struct {
    first_name string
    last_name string
    nickname string
    phone []string
    email []string
}
//ToPB
func(m *RoomMessageContact)ToFlat() *RoomMessageContact_Flat {
r := &RoomMessageContact_Flat{
    first_name:  m.first_name ,
    last_name:  m.last_name ,
    nickname:  m.nickname ,
    phone:  m.phone ,
    email:  m.email ,
}
return r
}
//ToPB
func(m *RoomMessageContact_Flat)ToPB() *RoomMessageContact {
r := &RoomMessageContact{
    first_name:  m.first_name ,
    last_name:  m.last_name ,
    nickname:  m.nickname ,
    phone:  m.phone ,
    email:  m.email ,
}
return r
}
//folding
var RoomMessageContact__FOlD = &RoomMessageContact{
    first_name:  "" ,
    last_name:  "" ,
    nickname:  "" ,
    
    
}


type RoomMessageWallet_Flat struct {
    type Type
    money_transfer MoneyTransfer
}
//ToPB
func(m *RoomMessageWallet)ToFlat() *RoomMessageWallet_Flat {
r := &RoomMessageWallet_Flat{
    
    
}
return r
}
//ToPB
func(m *RoomMessageWallet_Flat)ToPB() *RoomMessageWallet {
r := &RoomMessageWallet{
    
    
}
return r
}
//folding
var RoomMessageWallet__FOlD = &RoomMessageWallet{
    
    
}


type RoomMessageForwardFrom_Flat struct {
    room_id int
    message_id int
}
//ToPB
func(m *RoomMessageForwardFrom)ToFlat() *RoomMessageForwardFrom_Flat {
r := &RoomMessageForwardFrom_Flat{
    room_id:  int(m.room_id) ,
    message_id:  int(m.message_id) ,
}
return r
}
//ToPB
func(m *RoomMessageForwardFrom_Flat)ToPB() *RoomMessageForwardFrom {
r := &RoomMessageForwardFrom{
    room_id:  uint64(m.room_id) ,
    message_id:  uint64(m.message_id) ,
}
return r
}
//folding
var RoomMessageForwardFrom__FOlD = &RoomMessageForwardFrom{
    room_id:  0 ,
    message_id:  0 ,
}


type RegisteredUser_Flat struct {
    id int
    username string
    phone int
    first_name string
    last_name string
    display_name string
    initials string
    color string
    status Status
    last_seen int
    avatar_count int
    avatar Avatar
    mutual bool
    deleted bool
    cache_id string
    bio string
    verified bool
    bot bool
}
//ToPB
func(m *RegisteredUser)ToFlat() *RegisteredUser_Flat {
r := &RegisteredUser_Flat{
    id:  int(m.id) ,
    username:  m.username ,
    phone:  int(m.phone) ,
    first_name:  m.first_name ,
    last_name:  m.last_name ,
    display_name:  m.display_name ,
    initials:  m.initials ,
    color:  m.color ,
    
    last_seen:  int(m.last_seen) ,
    avatar_count:  int(m.avatar_count) ,
    
    mutual:  m.mutual ,
    deleted:  m.deleted ,
    cache_id:  m.cache_id ,
    bio:  m.bio ,
    verified:  m.verified ,
    bot:  m.bot ,
}
return r
}
//ToPB
func(m *RegisteredUser_Flat)ToPB() *RegisteredUser {
r := &RegisteredUser{
    id:  uint64(m.id) ,
    username:  m.username ,
    phone:  uint64(m.phone) ,
    first_name:  m.first_name ,
    last_name:  m.last_name ,
    display_name:  m.display_name ,
    initials:  m.initials ,
    color:  m.color ,
    
    last_seen:  uint32(m.last_seen) ,
    avatar_count:  uint32(m.avatar_count) ,
    
    mutual:  m.mutual ,
    deleted:  m.deleted ,
    cache_id:  m.cache_id ,
    bio:  m.bio ,
    verified:  m.verified ,
    bot:  m.bot ,
}
return r
}
//folding
var RegisteredUser__FOlD = &RegisteredUser{
    id:  0 ,
    username:  "" ,
    phone:  0 ,
    first_name:  "" ,
    last_name:  "" ,
    display_name:  "" ,
    initials:  "" ,
    color:  "" ,
    
    last_seen:  0 ,
    avatar_count:  0 ,
    
    mutual:  false ,
    deleted:  false ,
    cache_id:  "" ,
    bio:  "" ,
    verified:  false ,
    bot:  false ,
}


type Avatar_Flat struct {
    id int
    file File
}
//ToPB
func(m *Avatar)ToFlat() *Avatar_Flat {
r := &Avatar_Flat{
    id:  int(m.id) ,
    
}
return r
}
//ToPB
func(m *Avatar_Flat)ToPB() *Avatar {
r := &Avatar{
    id:  uint64(m.id) ,
    
}
return r
}
//folding
var Avatar__FOlD = &Avatar{
    id:  0 ,
    
}


type RoomMessage_Flat struct {
    message_id int
    message_version int
    status RoomMessageStatus
    status_version int
    message_type RoomMessageType
    message string
    attachment File
    author Author
    location RoomMessageLocation
    log RoomMessageLog
    contact RoomMessageContact
    wallet RoomMessageWallet
    edited bool
    create_time int
    update_time int
    deleted bool
    forward_from RoomMessage
    reply_to RoomMessage
    previous_message_id int
    random_id int
    additional_type int
    additional_data string
    extra_type ExtraType
    channel_extra ChannelExtra
}
//ToPB
func(m *RoomMessage)ToFlat() *RoomMessage_Flat {
r := &RoomMessage_Flat{
    message_id:  int(m.message_id) ,
    message_version:  int(m.message_version) ,
    
    status_version:  int(m.status_version) ,
    
    message:  m.message ,
    
    
    
    
    
    
    edited:  m.edited ,
    create_time:  int(m.create_time) ,
    update_time:  int(m.update_time) ,
    deleted:  m.deleted ,
    
    
    previous_message_id:  int(m.previous_message_id) ,
    random_id:  int(m.random_id) ,
    additional_type:  int(m.additional_type) ,
    additional_data:  m.additional_data ,
    
    
}
return r
}
//ToPB
func(m *RoomMessage_Flat)ToPB() *RoomMessage {
r := &RoomMessage{
    message_id:  uint64(m.message_id) ,
    message_version:  uint64(m.message_version) ,
    
    status_version:  uint64(m.status_version) ,
    
    message:  m.message ,
    
    
    
    
    
    
    edited:  m.edited ,
    create_time:  uint32(m.create_time) ,
    update_time:  uint32(m.update_time) ,
    deleted:  m.deleted ,
    
    
    previous_message_id:  uint64(m.previous_message_id) ,
    random_id:  uint64(m.random_id) ,
    additional_type:  uint32(m.additional_type) ,
    additional_data:  m.additional_data ,
    
    
}
return r
}
//folding
var RoomMessage__FOlD = &RoomMessage{
    message_id:  0 ,
    message_version:  0 ,
    
    status_version:  0 ,
    
    message:  "" ,
    
    
    
    
    
    
    edited:  false ,
    create_time:  0 ,
    update_time:  0 ,
    deleted:  false ,
    
    
    previous_message_id:  0 ,
    random_id:  0 ,
    additional_type:  0 ,
    additional_data:  "" ,
    
    
}


type RoomDraft_Flat struct {
    message string
    reply_to int
}
//ToPB
func(m *RoomDraft)ToFlat() *RoomDraft_Flat {
r := &RoomDraft_Flat{
    message:  m.message ,
    reply_to:  int(m.reply_to) ,
}
return r
}
//ToPB
func(m *RoomDraft_Flat)ToPB() *RoomDraft {
r := &RoomDraft{
    message:  m.message ,
    reply_to:  uint64(m.reply_to) ,
}
return r
}
//folding
var RoomDraft__FOlD = &RoomDraft{
    message:  "" ,
    reply_to:  0 ,
}


type Room_Flat struct {
    id int
    type Type
    title string
    initials string
    color string
    unread_count int
    last_message RoomMessage
    read_only bool
    is_participant bool
    draft RoomDraft
    first_unread_message RoomMessage
    room_mute RoomMute
    pin_id int
    pinned_message RoomMessage
    priority int
    chat_room_extra ChatRoom
    group_room_extra GroupRoom
    channel_room_extra ChannelRoom
}
//ToPB
func(m *Room)ToFlat() *Room_Flat {
r := &Room_Flat{
    id:  int(m.id) ,
    
    title:  m.title ,
    initials:  m.initials ,
    color:  m.color ,
    unread_count:  int(m.unread_count) ,
    
    read_only:  m.read_only ,
    is_participant:  m.is_participant ,
    
    
    
    pin_id:  int(m.pin_id) ,
    
    priority:  int(m.priority) ,
    
    
    
}
return r
}
//ToPB
func(m *Room_Flat)ToPB() *Room {
r := &Room{
    id:  uint64(m.id) ,
    
    title:  m.title ,
    initials:  m.initials ,
    color:  m.color ,
    unread_count:  uint32(m.unread_count) ,
    
    read_only:  m.read_only ,
    is_participant:  m.is_participant ,
    
    
    
    pin_id:  uint64(m.pin_id) ,
    
    priority:  uint32(m.priority) ,
    
    
    
}
return r
}
//folding
var Room__FOlD = &Room{
    id:  0 ,
    
    title:  "" ,
    initials:  "" ,
    color:  "" ,
    unread_count:  0 ,
    
    read_only:  false ,
    is_participant:  false ,
    
    
    
    pin_id:  0 ,
    
    priority:  0 ,
    
    
    
}


type ChatRoom_Flat struct {
    peer RegisteredUser
}
//ToPB
func(m *ChatRoom)ToFlat() *ChatRoom_Flat {
r := &ChatRoom_Flat{
    
}
return r
}
//ToPB
func(m *ChatRoom_Flat)ToPB() *ChatRoom {
r := &ChatRoom{
    
}
return r
}
//folding
var ChatRoom__FOlD = &ChatRoom{
    
}


type GroupRoom_Flat struct {
    type Type
    role Role
    participants_count int
    participants_count_label string
    participants_count_limit int
    participants_count_limit_label string
    description string
    avatar_count int
    avatar Avatar
    private_extra PrivateExtra
    public_extra PublicExtra
}
//ToPB
func(m *GroupRoom)ToFlat() *GroupRoom_Flat {
r := &GroupRoom_Flat{
    
    
    participants_count:  int(m.participants_count) ,
    participants_count_label:  m.participants_count_label ,
    participants_count_limit:  int(m.participants_count_limit) ,
    participants_count_limit_label:  m.participants_count_limit_label ,
    description:  m.description ,
    avatar_count:  int(m.avatar_count) ,
    
    
    
}
return r
}
//ToPB
func(m *GroupRoom_Flat)ToPB() *GroupRoom {
r := &GroupRoom{
    
    
    participants_count:  uint32(m.participants_count) ,
    participants_count_label:  m.participants_count_label ,
    participants_count_limit:  uint32(m.participants_count_limit) ,
    participants_count_limit_label:  m.participants_count_limit_label ,
    description:  m.description ,
    avatar_count:  uint32(m.avatar_count) ,
    
    
    
}
return r
}
//folding
var GroupRoom__FOlD = &GroupRoom{
    
    
    participants_count:  0 ,
    participants_count_label:  "" ,
    participants_count_limit:  0 ,
    participants_count_limit_label:  "" ,
    description:  "" ,
    avatar_count:  0 ,
    
    
    
}


type ChannelRoom_Flat struct {
    type Type
    role Role
    participants_count int
    participants_count_label string
    description string
    avatar_count int
    avatar Avatar
    private_extra PrivateExtra
    public_extra PublicExtra
    signature bool
    seen_id int
    verified bool
    reaction_status bool
}
//ToPB
func(m *ChannelRoom)ToFlat() *ChannelRoom_Flat {
r := &ChannelRoom_Flat{
    
    
    participants_count:  int(m.participants_count) ,
    participants_count_label:  m.participants_count_label ,
    description:  m.description ,
    avatar_count:  int(m.avatar_count) ,
    
    
    
    signature:  m.signature ,
    seen_id:  int(m.seen_id) ,
    verified:  m.verified ,
    reaction_status:  m.reaction_status ,
}
return r
}
//ToPB
func(m *ChannelRoom_Flat)ToPB() *ChannelRoom {
r := &ChannelRoom{
    
    
    participants_count:  uint32(m.participants_count) ,
    participants_count_label:  m.participants_count_label ,
    description:  m.description ,
    avatar_count:  uint32(m.avatar_count) ,
    
    
    
    signature:  m.signature ,
    seen_id:  uint64(m.seen_id) ,
    verified:  m.verified ,
    reaction_status:  m.reaction_status ,
}
return r
}
//folding
var ChannelRoom__FOlD = &ChannelRoom{
    
    
    participants_count:  0 ,
    participants_count_label:  "" ,
    description:  "" ,
    avatar_count:  0 ,
    
    
    
    signature:  false ,
    seen_id:  0 ,
    verified:  false ,
    reaction_status:  false ,
}


type Thumbnail_Flat struct {
    size int
    width int
    height int
    cache_id string
    name string
    mime string
}
//ToPB
func(m *Thumbnail)ToFlat() *Thumbnail_Flat {
r := &Thumbnail_Flat{
    size:  int(m.size) ,
    width:  int(m.width) ,
    height:  int(m.height) ,
    cache_id:  m.cache_id ,
    name:  m.name ,
    mime:  m.mime ,
}
return r
}
//ToPB
func(m *Thumbnail_Flat)ToPB() *Thumbnail {
r := &Thumbnail{
    size:  int64(m.size) ,
    width:  int32(m.width) ,
    height:  int32(m.height) ,
    cache_id:  m.cache_id ,
    name:  m.name ,
    mime:  m.mime ,
}
return r
}
//folding
var Thumbnail__FOlD = &Thumbnail{
    size:  0 ,
    width:  0 ,
    height:  0 ,
    cache_id:  "" ,
    name:  "" ,
    mime:  "" ,
}


type File_Flat struct {
    token string
    name string
    size int
    large_thumbnail Thumbnail
    small_thumbnail Thumbnail
    waveform_thumbnail Thumbnail
    width int
    height int
    duration float64
    cache_id string
    mime string
    public_url string
}
//ToPB
func(m *File)ToFlat() *File_Flat {
r := &File_Flat{
    token:  m.token ,
    name:  m.name ,
    size:  int(m.size) ,
    
    
    
    width:  int(m.width) ,
    height:  int(m.height) ,
    duration:  float64(m.duration) ,
    cache_id:  m.cache_id ,
    mime:  m.mime ,
    public_url:  m.public_url ,
}
return r
}
//ToPB
func(m *File_Flat)ToPB() *File {
r := &File{
    token:  m.token ,
    name:  m.name ,
    size:  int64(m.size) ,
    
    
    
    width:  int32(m.width) ,
    height:  int32(m.height) ,
    duration:  m.duration ,
    cache_id:  m.cache_id ,
    mime:  m.mime ,
    public_url:  m.public_url ,
}
return r
}
//folding
var File__FOlD = &File{
    token:  "" ,
    name:  "" ,
    size:  0 ,
    
    
    
    width:  0 ,
    height:  0 ,
    duration:  0.0 ,
    cache_id:  "" ,
    mime:  "" ,
    public_url:  "" ,
}


type Wallpaper_Flat struct {
    file File
    color string
}
//ToPB
func(m *Wallpaper)ToFlat() *Wallpaper_Flat {
r := &Wallpaper_Flat{
    
    color:  m.color ,
}
return r
}
//ToPB
func(m *Wallpaper_Flat)ToPB() *Wallpaper {
r := &Wallpaper{
    
    color:  m.color ,
}
return r
}
//folding
var Wallpaper__FOlD = &Wallpaper{
    
    color:  "" ,
}


type Pagination_Flat struct {
    offset int
    limit int
}
//ToPB
func(m *Pagination)ToFlat() *Pagination_Flat {
r := &Pagination_Flat{
    offset:  int(m.offset) ,
    limit:  int(m.limit) ,
}
return r
}
//ToPB
func(m *Pagination_Flat)ToPB() *Pagination {
r := &Pagination{
    offset:  uint32(m.offset) ,
    limit:  uint32(m.limit) ,
}
return r
}
//folding
var Pagination__FOlD = &Pagination{
    offset:  0 ,
    limit:  0 ,
}



/*
///// to_flat ///

func(m *RoomMessageLocation)ToFlat() *RoomMessageLocation_Flat {
r := &RoomMessageLocation_Flat{
    lat:  float64(m.lat) ,
    lon:  float64(m.lon) ,
}
return r
}

func(m *RoomMessageLog)ToFlat() *RoomMessageLog_Flat {
r := &RoomMessageLog_Flat{
    
    
    
}
return r
}

func(m *RoomMessageContact)ToFlat() *RoomMessageContact_Flat {
r := &RoomMessageContact_Flat{
    first_name:  m.first_name ,
    last_name:  m.last_name ,
    nickname:  m.nickname ,
    phone:  m.phone ,
    email:  m.email ,
}
return r
}

func(m *RoomMessageWallet)ToFlat() *RoomMessageWallet_Flat {
r := &RoomMessageWallet_Flat{
    
    
}
return r
}

func(m *RoomMessageForwardFrom)ToFlat() *RoomMessageForwardFrom_Flat {
r := &RoomMessageForwardFrom_Flat{
    room_id:  int(m.room_id) ,
    message_id:  int(m.message_id) ,
}
return r
}

func(m *RegisteredUser)ToFlat() *RegisteredUser_Flat {
r := &RegisteredUser_Flat{
    id:  int(m.id) ,
    username:  m.username ,
    phone:  int(m.phone) ,
    first_name:  m.first_name ,
    last_name:  m.last_name ,
    display_name:  m.display_name ,
    initials:  m.initials ,
    color:  m.color ,
    
    last_seen:  int(m.last_seen) ,
    avatar_count:  int(m.avatar_count) ,
    
    mutual:  m.mutual ,
    deleted:  m.deleted ,
    cache_id:  m.cache_id ,
    bio:  m.bio ,
    verified:  m.verified ,
    bot:  m.bot ,
}
return r
}

func(m *Avatar)ToFlat() *Avatar_Flat {
r := &Avatar_Flat{
    id:  int(m.id) ,
    
}
return r
}

func(m *RoomMessage)ToFlat() *RoomMessage_Flat {
r := &RoomMessage_Flat{
    message_id:  int(m.message_id) ,
    message_version:  int(m.message_version) ,
    
    status_version:  int(m.status_version) ,
    
    message:  m.message ,
    
    
    
    
    
    
    edited:  m.edited ,
    create_time:  int(m.create_time) ,
    update_time:  int(m.update_time) ,
    deleted:  m.deleted ,
    
    
    previous_message_id:  int(m.previous_message_id) ,
    random_id:  int(m.random_id) ,
    additional_type:  int(m.additional_type) ,
    additional_data:  m.additional_data ,
    
    
}
return r
}

func(m *RoomDraft)ToFlat() *RoomDraft_Flat {
r := &RoomDraft_Flat{
    message:  m.message ,
    reply_to:  int(m.reply_to) ,
}
return r
}

func(m *Room)ToFlat() *Room_Flat {
r := &Room_Flat{
    id:  int(m.id) ,
    
    title:  m.title ,
    initials:  m.initials ,
    color:  m.color ,
    unread_count:  int(m.unread_count) ,
    
    read_only:  m.read_only ,
    is_participant:  m.is_participant ,
    
    
    
    pin_id:  int(m.pin_id) ,
    
    priority:  int(m.priority) ,
    
    
    
}
return r
}

func(m *ChatRoom)ToFlat() *ChatRoom_Flat {
r := &ChatRoom_Flat{
    
}
return r
}

func(m *GroupRoom)ToFlat() *GroupRoom_Flat {
r := &GroupRoom_Flat{
    
    
    participants_count:  int(m.participants_count) ,
    participants_count_label:  m.participants_count_label ,
    participants_count_limit:  int(m.participants_count_limit) ,
    participants_count_limit_label:  m.participants_count_limit_label ,
    description:  m.description ,
    avatar_count:  int(m.avatar_count) ,
    
    
    
}
return r
}

func(m *ChannelRoom)ToFlat() *ChannelRoom_Flat {
r := &ChannelRoom_Flat{
    
    
    participants_count:  int(m.participants_count) ,
    participants_count_label:  m.participants_count_label ,
    description:  m.description ,
    avatar_count:  int(m.avatar_count) ,
    
    
    
    signature:  m.signature ,
    seen_id:  int(m.seen_id) ,
    verified:  m.verified ,
    reaction_status:  m.reaction_status ,
}
return r
}

func(m *Thumbnail)ToFlat() *Thumbnail_Flat {
r := &Thumbnail_Flat{
    size:  int(m.size) ,
    width:  int(m.width) ,
    height:  int(m.height) ,
    cache_id:  m.cache_id ,
    name:  m.name ,
    mime:  m.mime ,
}
return r
}

func(m *File)ToFlat() *File_Flat {
r := &File_Flat{
    token:  m.token ,
    name:  m.name ,
    size:  int(m.size) ,
    
    
    
    width:  int(m.width) ,
    height:  int(m.height) ,
    duration:  float64(m.duration) ,
    cache_id:  m.cache_id ,
    mime:  m.mime ,
    public_url:  m.public_url ,
}
return r
}

func(m *Wallpaper)ToFlat() *Wallpaper_Flat {
r := &Wallpaper_Flat{
    
    color:  m.color ,
}
return r
}

func(m *Pagination)ToFlat() *Pagination_Flat {
r := &Pagination_Flat{
    offset:  int(m.offset) ,
    limit:  int(m.limit) ,
}
return r
}



///// from_flat ///

func(m *RoomMessageLocation_Flat)ToPB() *RoomMessageLocation {
r := &RoomMessageLocation{
    lat:  m.lat ,
    lon:  m.lon ,
}
return r
}

func(m *RoomMessageLog_Flat)ToPB() *RoomMessageLog {
r := &RoomMessageLog{
    
    
    
}
return r
}

func(m *RoomMessageContact_Flat)ToPB() *RoomMessageContact {
r := &RoomMessageContact{
    first_name:  m.first_name ,
    last_name:  m.last_name ,
    nickname:  m.nickname ,
    phone:  m.phone ,
    email:  m.email ,
}
return r
}

func(m *RoomMessageWallet_Flat)ToPB() *RoomMessageWallet {
r := &RoomMessageWallet{
    
    
}
return r
}

func(m *RoomMessageForwardFrom_Flat)ToPB() *RoomMessageForwardFrom {
r := &RoomMessageForwardFrom{
    room_id:  uint64(m.room_id) ,
    message_id:  uint64(m.message_id) ,
}
return r
}

func(m *RegisteredUser_Flat)ToPB() *RegisteredUser {
r := &RegisteredUser{
    id:  uint64(m.id) ,
    username:  m.username ,
    phone:  uint64(m.phone) ,
    first_name:  m.first_name ,
    last_name:  m.last_name ,
    display_name:  m.display_name ,
    initials:  m.initials ,
    color:  m.color ,
    
    last_seen:  uint32(m.last_seen) ,
    avatar_count:  uint32(m.avatar_count) ,
    
    mutual:  m.mutual ,
    deleted:  m.deleted ,
    cache_id:  m.cache_id ,
    bio:  m.bio ,
    verified:  m.verified ,
    bot:  m.bot ,
}
return r
}

func(m *Avatar_Flat)ToPB() *Avatar {
r := &Avatar{
    id:  uint64(m.id) ,
    
}
return r
}

func(m *RoomMessage_Flat)ToPB() *RoomMessage {
r := &RoomMessage{
    message_id:  uint64(m.message_id) ,
    message_version:  uint64(m.message_version) ,
    
    status_version:  uint64(m.status_version) ,
    
    message:  m.message ,
    
    
    
    
    
    
    edited:  m.edited ,
    create_time:  uint32(m.create_time) ,
    update_time:  uint32(m.update_time) ,
    deleted:  m.deleted ,
    
    
    previous_message_id:  uint64(m.previous_message_id) ,
    random_id:  uint64(m.random_id) ,
    additional_type:  uint32(m.additional_type) ,
    additional_data:  m.additional_data ,
    
    
}
return r
}

func(m *RoomDraft_Flat)ToPB() *RoomDraft {
r := &RoomDraft{
    message:  m.message ,
    reply_to:  uint64(m.reply_to) ,
}
return r
}

func(m *Room_Flat)ToPB() *Room {
r := &Room{
    id:  uint64(m.id) ,
    
    title:  m.title ,
    initials:  m.initials ,
    color:  m.color ,
    unread_count:  uint32(m.unread_count) ,
    
    read_only:  m.read_only ,
    is_participant:  m.is_participant ,
    
    
    
    pin_id:  uint64(m.pin_id) ,
    
    priority:  uint32(m.priority) ,
    
    
    
}
return r
}

func(m *ChatRoom_Flat)ToPB() *ChatRoom {
r := &ChatRoom{
    
}
return r
}

func(m *GroupRoom_Flat)ToPB() *GroupRoom {
r := &GroupRoom{
    
    
    participants_count:  uint32(m.participants_count) ,
    participants_count_label:  m.participants_count_label ,
    participants_count_limit:  uint32(m.participants_count_limit) ,
    participants_count_limit_label:  m.participants_count_limit_label ,
    description:  m.description ,
    avatar_count:  uint32(m.avatar_count) ,
    
    
    
}
return r
}

func(m *ChannelRoom_Flat)ToPB() *ChannelRoom {
r := &ChannelRoom{
    
    
    participants_count:  uint32(m.participants_count) ,
    participants_count_label:  m.participants_count_label ,
    description:  m.description ,
    avatar_count:  uint32(m.avatar_count) ,
    
    
    
    signature:  m.signature ,
    seen_id:  uint64(m.seen_id) ,
    verified:  m.verified ,
    reaction_status:  m.reaction_status ,
}
return r
}

func(m *Thumbnail_Flat)ToPB() *Thumbnail {
r := &Thumbnail{
    size:  int64(m.size) ,
    width:  int32(m.width) ,
    height:  int32(m.height) ,
    cache_id:  m.cache_id ,
    name:  m.name ,
    mime:  m.mime ,
}
return r
}

func(m *File_Flat)ToPB() *File {
r := &File{
    token:  m.token ,
    name:  m.name ,
    size:  int64(m.size) ,
    
    
    
    width:  int32(m.width) ,
    height:  int32(m.height) ,
    duration:  m.duration ,
    cache_id:  m.cache_id ,
    mime:  m.mime ,
    public_url:  m.public_url ,
}
return r
}

func(m *Wallpaper_Flat)ToPB() *Wallpaper {
r := &Wallpaper{
    
    color:  m.color ,
}
return r
}

func(m *Pagination_Flat)ToPB() *Pagination {
r := &Pagination{
    offset:  uint32(m.offset) ,
    limit:  uint32(m.limit) ,
}
return r
}



///// folding ///

var RoomMessageLocation__FOlD = &RoomMessageLocation{
        lat:  0.0 ,
        lon:  0.0 ,
}


var RoomMessageLog__FOlD = &RoomMessageLog{
        
        
        
}


var RoomMessageContact__FOlD = &RoomMessageContact{
        first_name:  "" ,
        last_name:  "" ,
        nickname:  "" ,
        phone:  "" ,
        email:  "" ,
}


var RoomMessageWallet__FOlD = &RoomMessageWallet{
        
        
}


var RoomMessageForwardFrom__FOlD = &RoomMessageForwardFrom{
        room_id:  0 ,
        message_id:  0 ,
}


var RegisteredUser__FOlD = &RegisteredUser{
        id:  0 ,
        username:  "" ,
        phone:  0 ,
        first_name:  "" ,
        last_name:  "" ,
        display_name:  "" ,
        initials:  "" ,
        color:  "" ,
        
        last_seen:  0 ,
        avatar_count:  0 ,
        
        mutual:  false ,
        deleted:  false ,
        cache_id:  "" ,
        bio:  "" ,
        verified:  false ,
        bot:  false ,
}


var Avatar__FOlD = &Avatar{
        id:  0 ,
        
}


var RoomMessage__FOlD = &RoomMessage{
        message_id:  0 ,
        message_version:  0 ,
        
        status_version:  0 ,
        
        message:  "" ,
        
        
        
        
        
        
        edited:  false ,
        create_time:  0 ,
        update_time:  0 ,
        deleted:  false ,
        
        
        previous_message_id:  0 ,
        random_id:  0 ,
        additional_type:  0 ,
        additional_data:  "" ,
        
        
}


var RoomDraft__FOlD = &RoomDraft{
        message:  "" ,
        reply_to:  0 ,
}


var Room__FOlD = &Room{
        id:  0 ,
        
        title:  "" ,
        initials:  "" ,
        color:  "" ,
        unread_count:  0 ,
        
        read_only:  false ,
        is_participant:  false ,
        
        
        
        pin_id:  0 ,
        
        priority:  0 ,
        
        
        
}


var ChatRoom__FOlD = &ChatRoom{
        
}


var GroupRoom__FOlD = &GroupRoom{
        
        
        participants_count:  0 ,
        participants_count_label:  "" ,
        participants_count_limit:  0 ,
        participants_count_limit_label:  "" ,
        description:  "" ,
        avatar_count:  0 ,
        
        
        
}


var ChannelRoom__FOlD = &ChannelRoom{
        
        
        participants_count:  0 ,
        participants_count_label:  "" ,
        description:  "" ,
        avatar_count:  0 ,
        
        
        
        signature:  false ,
        seen_id:  0 ,
        verified:  false ,
        reaction_status:  false ,
}


var Thumbnail__FOlD = &Thumbnail{
        size:  0 ,
        width:  0 ,
        height:  0 ,
        cache_id:  "" ,
        name:  "" ,
        mime:  "" ,
}


var File__FOlD = &File{
        token:  "" ,
        name:  "" ,
        size:  0 ,
        
        
        
        width:  0 ,
        height:  0 ,
        duration:  0.0 ,
        cache_id:  "" ,
        mime:  "" ,
        public_url:  "" ,
}


var Wallpaper__FOlD = &Wallpaper{
        
        color:  "" ,
}


var Pagination__FOlD = &Pagination{
        offset:  0 ,
        limit:  0 ,
}



*/