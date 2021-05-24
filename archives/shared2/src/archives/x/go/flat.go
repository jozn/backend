package x

import "ms/sun/shared/helper"

type SendConfirmCodeParam_Flat struct {
	Hash        string
	Phone       string
	CountryCode string
	Resend      bool
}

//ToPB
func (m *SendConfirmCodeParam) ToFlat() *SendConfirmCodeParam_Flat {
	r := &SendConfirmCodeParam_Flat{
		Hash:        m.Hash,
		Phone:       m.Phone,
		CountryCode: m.CountryCode,
		Resend:      m.Resend,
	}
	return r
}

//ToPB
func (m *SendConfirmCodeParam_Flat) ToPB() *SendConfirmCodeParam {
	r := &SendConfirmCodeParam{
		Hash:        m.Hash,
		Phone:       m.Phone,
		CountryCode: m.CountryCode,
		Resend:      m.Resend,
	}
	return r
}

//folding
var SendConfirmCodeParam__FOlD = &SendConfirmCodeParam{
	Hash:        "",
	Phone:       "",
	CountryCode: "",
	Resend:      false,
}

type SendConfirmCodeResponse_Flat struct {
	Done              bool
	ErrorMessage      string
	JustEmailRegister bool
	SmsNumbers        []string
	IsLogin           bool
}

//ToPB
func (m *SendConfirmCodeResponse) ToFlat() *SendConfirmCodeResponse_Flat {
	r := &SendConfirmCodeResponse_Flat{
		Done:              m.Done,
		ErrorMessage:      m.ErrorMessage,
		JustEmailRegister: m.JustEmailRegister,
		SmsNumbers:        m.SmsNumbers,
		IsLogin:           m.IsLogin,
	}
	return r
}

//ToPB
func (m *SendConfirmCodeResponse_Flat) ToPB() *SendConfirmCodeResponse {
	r := &SendConfirmCodeResponse{
		Done:              m.Done,
		ErrorMessage:      m.ErrorMessage,
		JustEmailRegister: m.JustEmailRegister,
		SmsNumbers:        m.SmsNumbers,
		IsLogin:           m.IsLogin,
	}
	return r
}

//folding
var SendConfirmCodeResponse__FOlD = &SendConfirmCodeResponse{
	Done:              false,
	ErrorMessage:      "",
	JustEmailRegister: false,

	IsLogin: false,
}

type ConfirmCodeParam_Flat struct {
	Hash  string
	Phone string
	Code  int
}

//ToPB
func (m *ConfirmCodeParam) ToFlat() *ConfirmCodeParam_Flat {
	r := &ConfirmCodeParam_Flat{
		Hash:  m.Hash,
		Phone: m.Phone,
		Code:  int(m.Code),
	}
	return r
}

//ToPB
func (m *ConfirmCodeParam_Flat) ToPB() *ConfirmCodeParam {
	r := &ConfirmCodeParam{
		Hash:  m.Hash,
		Phone: m.Phone,
		Code:  int32(m.Code),
	}
	return r
}

//folding
var ConfirmCodeParam__FOlD = &ConfirmCodeParam{
	Hash:  "",
	Phone: "",
	Code:  0,
}

type ConfirmCodeResponse_Flat struct {
	Done         bool
	ErrorMessage string
	SelfUserView SelfUserView
}

//ToPB
func (m *ConfirmCodeResponse) ToFlat() *ConfirmCodeResponse_Flat {
	r := &ConfirmCodeResponse_Flat{
		Done:         m.Done,
		ErrorMessage: m.ErrorMessage,
	}
	return r
}

//ToPB
func (m *ConfirmCodeResponse_Flat) ToPB() *ConfirmCodeResponse {
	r := &ConfirmCodeResponse{
		Done:         m.Done,
		ErrorMessage: m.ErrorMessage,
	}
	return r
}

//folding
var ConfirmCodeResponse__FOlD = &ConfirmCodeResponse{
	Done:         false,
	ErrorMessage: "",
}

type SingUpParam_Flat struct {
	Hash      string
	FirstName string
	LastName  string
	UserName  string
	Phone     string
	Email     string
}

//ToPB
func (m *SingUpParam) ToFlat() *SingUpParam_Flat {
	r := &SingUpParam_Flat{
		Hash:      m.Hash,
		FirstName: m.FirstName,
		LastName:  m.LastName,
		UserName:  m.UserName,
		Phone:     m.Phone,
		Email:     m.Email,
	}
	return r
}

//ToPB
func (m *SingUpParam_Flat) ToPB() *SingUpParam {
	r := &SingUpParam{
		Hash:      m.Hash,
		FirstName: m.FirstName,
		LastName:  m.LastName,
		UserName:  m.UserName,
		Phone:     m.Phone,
		Email:     m.Email,
	}
	return r
}

//folding
var SingUpParam__FOlD = &SingUpParam{
	Hash:      "",
	FirstName: "",
	LastName:  "",
	UserName:  "",
	Phone:     "",
	Email:     "",
}

type SingUpResponse_Flat struct {
	Done         bool
	ErrorMessage string
	SelfUserView SelfUserView
}

//ToPB
func (m *SingUpResponse) ToFlat() *SingUpResponse_Flat {
	r := &SingUpResponse_Flat{
		Done:         m.Done,
		ErrorMessage: m.ErrorMessage,
	}
	return r
}

//ToPB
func (m *SingUpResponse_Flat) ToPB() *SingUpResponse {
	r := &SingUpResponse{
		Done:         m.Done,
		ErrorMessage: m.ErrorMessage,
	}
	return r
}

//folding
var SingUpResponse__FOlD = &SingUpResponse{
	Done:         false,
	ErrorMessage: "",
}

type SingInParam_Flat struct {
	UserNamePhoneEmail string
	Password           string
}

//ToPB
func (m *SingInParam) ToFlat() *SingInParam_Flat {
	r := &SingInParam_Flat{
		UserNamePhoneEmail: m.UserNamePhoneEmail,
		Password:           m.Password,
	}
	return r
}

//ToPB
func (m *SingInParam_Flat) ToPB() *SingInParam {
	r := &SingInParam{
		UserNamePhoneEmail: m.UserNamePhoneEmail,
		Password:           m.Password,
	}
	return r
}

//folding
var SingInParam__FOlD = &SingInParam{
	UserNamePhoneEmail: "",
	Password:           "",
}

type SingInResponse_Flat struct {
	Done         bool
	ErrorMessage string
	SelfUserView SelfUserView
}

//ToPB
func (m *SingInResponse) ToFlat() *SingInResponse_Flat {
	r := &SingInResponse_Flat{
		Done:         m.Done,
		ErrorMessage: m.ErrorMessage,
	}
	return r
}

//ToPB
func (m *SingInResponse_Flat) ToPB() *SingInResponse {
	r := &SingInResponse{
		Done:         m.Done,
		ErrorMessage: m.ErrorMessage,
	}
	return r
}

//folding
var SingInResponse__FOlD = &SingInResponse{
	Done:         false,
	ErrorMessage: "",
}

type LogOutParam_Flat struct {
}

//ToPB
func (m *LogOutParam) ToFlat() *LogOutParam_Flat {
	r := &LogOutParam_Flat{}
	return r
}

//ToPB
func (m *LogOutParam_Flat) ToPB() *LogOutParam {
	r := &LogOutParam{}
	return r
}

//folding
var LogOutParam__FOlD = &LogOutParam{}

type LogOutResponse_Flat struct {
	Done         bool
	ErrorMessage string
}

//ToPB
func (m *LogOutResponse) ToFlat() *LogOutResponse_Flat {
	r := &LogOutResponse_Flat{
		Done:         m.Done,
		ErrorMessage: m.ErrorMessage,
	}
	return r
}

//ToPB
func (m *LogOutResponse_Flat) ToPB() *LogOutResponse {
	r := &LogOutResponse{
		Done:         m.Done,
		ErrorMessage: m.ErrorMessage,
	}
	return r
}

//folding
var LogOutResponse__FOlD = &LogOutResponse{
	Done:         false,
	ErrorMessage: "",
}

type MediaView_Flat struct {
	FileRefId int
	UserId    int
	Name      string
	Width     int
	Height    int
	Duration  int
	Extension string
	UrlSource string
}

//ToPB
func (m *MediaView) ToFlat() *MediaView_Flat {
	r := &MediaView_Flat{
		FileRefId: int(m.FileRefId),
		UserId:    int(m.UserId),
		Name:      m.Name,
		Width:     int(m.Width),
		Height:    int(m.Height),
		Duration:  int(m.Duration),
		Extension: m.Extension,
		UrlSource: m.UrlSource,
	}
	return r
}

//ToPB
func (m *MediaView_Flat) ToPB() *MediaView {
	r := &MediaView{
		FileRefId: int64(m.FileRefId),
		UserId:    int64(m.UserId),
		Name:      m.Name,
		Width:     int32(m.Width),
		Height:    int32(m.Height),
		Duration:  int32(m.Duration),
		Extension: m.Extension,
		UrlSource: m.UrlSource,
	}
	return r
}

//folding
var MediaView__FOlD = &MediaView{
	FileRefId: 0,
	UserId:    0,
	Name:      "",
	Width:     0,
	Height:    0,
	Duration:  0,
	Extension: "",
	UrlSource: "",
}

type ActionView_Flat struct {
	ActionId              int
	ActorUserId           int
	ActionTypeEnum        int
	PeerUserId            int
	PostId                int
	CommentId             int
	Murmur64Hash          int
	CreatedTime           int
	ActorUserView         UserView
	PostView              PostView
	CommentView           CommentView
	FollowedUserView      UserView
	ContentOwenerUserView UserView
}

//ToPB
func (m *ActionView) ToFlat() *ActionView_Flat {
	r := &ActionView_Flat{
		ActionId:       int(m.ActionId),
		ActorUserId:    int(m.ActorUserId),
		ActionTypeEnum: int(m.ActionTypeEnum),
		PeerUserId:     int(m.PeerUserId),
		PostId:         int(m.PostId),
		CommentId:      int(m.CommentId),
		Murmur64Hash:   int(m.Murmur64Hash),
		CreatedTime:    int(m.CreatedTime),
	}
	return r
}

//ToPB
func (m *ActionView_Flat) ToPB() *ActionView {
	r := &ActionView{
		ActionId:       int64(m.ActionId),
		ActorUserId:    int32(m.ActorUserId),
		ActionTypeEnum: int32(m.ActionTypeEnum),
		PeerUserId:     int32(m.PeerUserId),
		PostId:         int64(m.PostId),
		CommentId:      int64(m.CommentId),
		Murmur64Hash:   int64(m.Murmur64Hash),
		CreatedTime:    int32(m.CreatedTime),
	}
	return r
}

//folding
var ActionView__FOlD = &ActionView{
	ActionId:       0,
	ActorUserId:    0,
	ActionTypeEnum: 0,
	PeerUserId:     0,
	PostId:         0,
	CommentId:      0,
	Murmur64Hash:   0,
	CreatedTime:    0,
}

type CommentView_Flat struct {
	CommentId      int
	UserId         int
	PostId         int
	Text           string
	LikesCount     int
	CreatedTime    int
	SenderUserView UserView
}

//ToPB
func (m *CommentView) ToFlat() *CommentView_Flat {
	r := &CommentView_Flat{
		CommentId:   int(m.CommentId),
		UserId:      int(m.UserId),
		PostId:      int(m.PostId),
		Text:        m.Text,
		LikesCount:  int(m.LikesCount),
		CreatedTime: int(m.CreatedTime),
	}
	return r
}

//ToPB
func (m *CommentView_Flat) ToPB() *CommentView {
	r := &CommentView{
		CommentId:   int64(m.CommentId),
		UserId:      int32(m.UserId),
		PostId:      int64(m.PostId),
		Text:        m.Text,
		LikesCount:  int32(m.LikesCount),
		CreatedTime: int32(m.CreatedTime),
	}
	return r
}

//folding
var CommentView__FOlD = &CommentView{
	CommentId:   0,
	UserId:      0,
	PostId:      0,
	Text:        "",
	LikesCount:  0,
	CreatedTime: 0,
}

type PostView_Flat struct {
	PostId           int
	UserId           int
	PostTypeEnum     PostTypeEnum
	Text             string
	RichText         string
	MediaCount       int
	SharedTo         int
	DisableComment   int
	HasTag           int
	CommentsCount    int
	LikesCount       int
	ViewsCount       int
	EditedTime       int
	CreatedTime      int
	ReSharedPostId   int
	DidILiked        bool
	DidIReShared     bool
	SenderUserView   UserView
	ReSharedUserView UserView
	MediaView        MediaView
	MediaViewList    []MediaView
}

//ToPB
func (m *PostView) ToFlat() *PostView_Flat {
	r := &PostView_Flat{
		PostId: int(m.PostId),
		UserId: int(m.UserId),

		Text:           m.Text,
		RichText:       m.RichText,
		MediaCount:     int(m.MediaCount),
		SharedTo:       int(m.SharedTo),
		DisableComment: int(m.DisableComment),
		HasTag:         int(m.HasTag),
		CommentsCount:  int(m.CommentsCount),
		LikesCount:     int(m.LikesCount),
		ViewsCount:     int(m.ViewsCount),
		EditedTime:     int(m.EditedTime),
		CreatedTime:    int(m.CreatedTime),
		ReSharedPostId: int(m.ReSharedPostId),
		DidILiked:      m.DidILiked,
		DidIReShared:   m.DidIReShared,
	}
	return r
}

//ToPB
func (m *PostView_Flat) ToPB() *PostView {
	r := &PostView{
		PostId: int64(m.PostId),
		UserId: int32(m.UserId),

		Text:           m.Text,
		RichText:       m.RichText,
		MediaCount:     int32(m.MediaCount),
		SharedTo:       int32(m.SharedTo),
		DisableComment: int32(m.DisableComment),
		HasTag:         int32(m.HasTag),
		CommentsCount:  int32(m.CommentsCount),
		LikesCount:     int32(m.LikesCount),
		ViewsCount:     int32(m.ViewsCount),
		EditedTime:     int32(m.EditedTime),
		CreatedTime:    int32(m.CreatedTime),
		ReSharedPostId: int64(m.ReSharedPostId),
		DidILiked:      m.DidILiked,
		DidIReShared:   m.DidIReShared,
	}
	return r
}

//folding
var PostView__FOlD = &PostView{
	PostId: 0,
	UserId: 0,

	Text:           "",
	RichText:       "",
	MediaCount:     0,
	SharedTo:       0,
	DisableComment: 0,
	HasTag:         0,
	CommentsCount:  0,
	LikesCount:     0,
	ViewsCount:     0,
	EditedTime:     0,
	CreatedTime:    0,
	ReSharedPostId: 0,
	DidILiked:      false,
	DidIReShared:   false,
}

type UserView_Flat struct {
	UserId               int
	UserName             string
	FirstName            string
	LastName             string
	AvatarRefId          int
	ProfilePrivacy       int
	Phone                int
	About                string
	FollowersCount       int
	FollowingCount       int
	PostsCount           int
	MediaCount           int
	UserOnlineStatusEnum UserOnlineStatusEnum
	LastActiveTime       int
	LastActiveTimeShow   string
	MyFollwing           FollowingEnum
}

//ToPB
func (m *UserView) ToFlat() *UserView_Flat {
	r := &UserView_Flat{
		UserId:         int(m.UserId),
		UserName:       m.UserName,
		FirstName:      m.FirstName,
		LastName:       m.LastName,
		AvatarRefId:    int(m.AvatarRefId),
		ProfilePrivacy: int(m.ProfilePrivacy),
		Phone:          int(m.Phone),
		About:          m.About,
		FollowersCount: int(m.FollowersCount),
		FollowingCount: int(m.FollowingCount),
		PostsCount:     int(m.PostsCount),
		MediaCount:     int(m.MediaCount),

		LastActiveTime:     int(m.LastActiveTime),
		LastActiveTimeShow: m.LastActiveTimeShow,
	}
	return r
}

//ToPB
func (m *UserView_Flat) ToPB() *UserView {
	r := &UserView{
		UserId:         int32(m.UserId),
		UserName:       m.UserName,
		FirstName:      m.FirstName,
		LastName:       m.LastName,
		AvatarRefId:    int64(m.AvatarRefId),
		ProfilePrivacy: int32(m.ProfilePrivacy),
		Phone:          int64(m.Phone),
		About:          m.About,
		FollowersCount: int32(m.FollowersCount),
		FollowingCount: int32(m.FollowingCount),
		PostsCount:     int32(m.PostsCount),
		MediaCount:     int32(m.MediaCount),

		LastActiveTime:     int32(m.LastActiveTime),
		LastActiveTimeShow: m.LastActiveTimeShow,
	}
	return r
}

//folding
var UserView__FOlD = &UserView{
	UserId:         0,
	UserName:       "",
	FirstName:      "",
	LastName:       "",
	AvatarRefId:    0,
	ProfilePrivacy: 0,
	Phone:          0,
	About:          "",
	FollowersCount: 0,
	FollowingCount: 0,
	PostsCount:     0,
	MediaCount:     0,

	LastActiveTime:     0,
	LastActiveTimeShow: "",
}

type SelfUserView_Flat struct {
	UserView           UserView
	ProfilePrivacy     int
	OnlinePrivacy      int
	CallPrivacy        int
	AddToGroupPrivacy  int
	SeenMessagePrivacy int
}

//ToPB
func (m *SelfUserView) ToFlat() *SelfUserView_Flat {
	r := &SelfUserView_Flat{

		ProfilePrivacy:     int(m.ProfilePrivacy),
		OnlinePrivacy:      int(m.OnlinePrivacy),
		CallPrivacy:        int(m.CallPrivacy),
		AddToGroupPrivacy:  int(m.AddToGroupPrivacy),
		SeenMessagePrivacy: int(m.SeenMessagePrivacy),
	}
	return r
}

//ToPB
func (m *SelfUserView_Flat) ToPB() *SelfUserView {
	r := &SelfUserView{

		ProfilePrivacy:     int32(m.ProfilePrivacy),
		OnlinePrivacy:      int32(m.OnlinePrivacy),
		CallPrivacy:        int32(m.CallPrivacy),
		AddToGroupPrivacy:  int32(m.AddToGroupPrivacy),
		SeenMessagePrivacy: int32(m.SeenMessagePrivacy),
	}
	return r
}

//folding
var SelfUserView__FOlD = &SelfUserView{

	ProfilePrivacy:     0,
	OnlinePrivacy:      0,
	CallPrivacy:        0,
	AddToGroupPrivacy:  0,
	SeenMessagePrivacy: 0,
}

type Error_Flat struct {
	Error        ServerErrors
	ShowError    bool
	ErrorMessage string
}

//ToPB
func (m *Error) ToFlat() *Error_Flat {
	r := &Error_Flat{

		ShowError:    m.ShowError,
		ErrorMessage: m.ErrorMessage,
	}
	return r
}

//ToPB
func (m *Error_Flat) ToPB() *Error {
	r := &Error{

		ShowError:    m.ShowError,
		ErrorMessage: m.ErrorMessage,
	}
	return r
}

//folding
var Error__FOlD = &Error{

	ShowError:    false,
	ErrorMessage: "",
}

/*
///// to_flat ///

func(m *SendConfirmCodeParam)ToFlat() *SendConfirmCodeParam_Flat {
r := &SendConfirmCodeParam_Flat{
    Hash:  m.Hash ,
    Phone:  m.Phone ,
    CountryCode:  m.CountryCode ,
    Resend:  m.Resend ,
}
return r
}

func(m *SendConfirmCodeResponse)ToFlat() *SendConfirmCodeResponse_Flat {
r := &SendConfirmCodeResponse_Flat{
    Done:  m.Done ,
    ErrorMessage:  m.ErrorMessage ,
    JustEmailRegister:  m.JustEmailRegister ,
    SmsNumbers:  m.SmsNumbers ,
    IsLogin:  m.IsLogin ,
}
return r
}

func(m *ConfirmCodeParam)ToFlat() *ConfirmCodeParam_Flat {
r := &ConfirmCodeParam_Flat{
    Hash:  m.Hash ,
    Phone:  m.Phone ,
    Code:  int(m.Code) ,
}
return r
}

func(m *ConfirmCodeResponse)ToFlat() *ConfirmCodeResponse_Flat {
r := &ConfirmCodeResponse_Flat{
    Done:  m.Done ,
    ErrorMessage:  m.ErrorMessage ,

}
return r
}

func(m *SingUpParam)ToFlat() *SingUpParam_Flat {
r := &SingUpParam_Flat{
    Hash:  m.Hash ,
    FirstName:  m.FirstName ,
    LastName:  m.LastName ,
    UserName:  m.UserName ,
    Phone:  m.Phone ,
    Email:  m.Email ,
}
return r
}

func(m *SingUpResponse)ToFlat() *SingUpResponse_Flat {
r := &SingUpResponse_Flat{
    Done:  m.Done ,
    ErrorMessage:  m.ErrorMessage ,

}
return r
}

func(m *SingInParam)ToFlat() *SingInParam_Flat {
r := &SingInParam_Flat{
    UserNamePhoneEmail:  m.UserNamePhoneEmail ,
    Password:  m.Password ,
}
return r
}

func(m *SingInResponse)ToFlat() *SingInResponse_Flat {
r := &SingInResponse_Flat{
    Done:  m.Done ,
    ErrorMessage:  m.ErrorMessage ,

}
return r
}

func(m *LogOutParam)ToFlat() *LogOutParam_Flat {
r := &LogOutParam_Flat{
}
return r
}

func(m *LogOutResponse)ToFlat() *LogOutResponse_Flat {
r := &LogOutResponse_Flat{
    Done:  m.Done ,
    ErrorMessage:  m.ErrorMessage ,
}
return r
}

func(m *MediaView)ToFlat() *MediaView_Flat {
r := &MediaView_Flat{
    FileRefId:  int(m.FileRefId) ,
    UserId:  int(m.UserId) ,
    Name:  m.Name ,
    Width:  int(m.Width) ,
    Height:  int(m.Height) ,
    Duration:  int(m.Duration) ,
    Extension:  m.Extension ,
    UrlSource:  m.UrlSource ,
}
return r
}

func(m *ActionView)ToFlat() *ActionView_Flat {
r := &ActionView_Flat{
    ActionId:  int(m.ActionId) ,
    ActorUserId:  int(m.ActorUserId) ,
    ActionTypeEnum:  int(m.ActionTypeEnum) ,
    PeerUserId:  int(m.PeerUserId) ,
    PostId:  int(m.PostId) ,
    CommentId:  int(m.CommentId) ,
    Murmur64Hash:  int(m.Murmur64Hash) ,
    CreatedTime:  int(m.CreatedTime) ,





}
return r
}

func(m *CommentView)ToFlat() *CommentView_Flat {
r := &CommentView_Flat{
    CommentId:  int(m.CommentId) ,
    UserId:  int(m.UserId) ,
    PostId:  int(m.PostId) ,
    Text:  m.Text ,
    LikesCount:  int(m.LikesCount) ,
    CreatedTime:  int(m.CreatedTime) ,

}
return r
}

func(m *PostView)ToFlat() *PostView_Flat {
r := &PostView_Flat{
    PostId:  int(m.PostId) ,
    UserId:  int(m.UserId) ,

    Text:  m.Text ,
    RichText:  m.RichText ,
    MediaCount:  int(m.MediaCount) ,
    SharedTo:  int(m.SharedTo) ,
    DisableComment:  int(m.DisableComment) ,
    HasTag:  int(m.HasTag) ,
    CommentsCount:  int(m.CommentsCount) ,
    LikesCount:  int(m.LikesCount) ,
    ViewsCount:  int(m.ViewsCount) ,
    EditedTime:  int(m.EditedTime) ,
    CreatedTime:  int(m.CreatedTime) ,
    ReSharedPostId:  int(m.ReSharedPostId) ,
    DidILiked:  m.DidILiked ,
    DidIReShared:  m.DidIReShared ,




}
return r
}

func(m *UserView)ToFlat() *UserView_Flat {
r := &UserView_Flat{
    UserId:  int(m.UserId) ,
    UserName:  m.UserName ,
    FirstName:  m.FirstName ,
    LastName:  m.LastName ,
    AvatarRefId:  int(m.AvatarRefId) ,
    ProfilePrivacy:  int(m.ProfilePrivacy) ,
    Phone:  int(m.Phone) ,
    About:  m.About ,
    FollowersCount:  int(m.FollowersCount) ,
    FollowingCount:  int(m.FollowingCount) ,
    PostsCount:  int(m.PostsCount) ,
    MediaCount:  int(m.MediaCount) ,

    LastActiveTime:  int(m.LastActiveTime) ,
    LastActiveTimeShow:  m.LastActiveTimeShow ,

}
return r
}

func(m *SelfUserView)ToFlat() *SelfUserView_Flat {
r := &SelfUserView_Flat{

    ProfilePrivacy:  int(m.ProfilePrivacy) ,
    OnlinePrivacy:  int(m.OnlinePrivacy) ,
    CallPrivacy:  int(m.CallPrivacy) ,
    AddToGroupPrivacy:  int(m.AddToGroupPrivacy) ,
    SeenMessagePrivacy:  int(m.SeenMessagePrivacy) ,
}
return r
}

func(m *Error)ToFlat() *Error_Flat {
r := &Error_Flat{

    ShowError:  m.ShowError ,
    ErrorMessage:  m.ErrorMessage ,
}
return r
}



///// from_flat ///

func(m *SendConfirmCodeParam_Flat)ToPB() *SendConfirmCodeParam {
r := &SendConfirmCodeParam{
    Hash:  m.Hash ,
    Phone:  m.Phone ,
    CountryCode:  m.CountryCode ,
    Resend:  m.Resend ,
}
return r
}

func(m *SendConfirmCodeResponse_Flat)ToPB() *SendConfirmCodeResponse {
r := &SendConfirmCodeResponse{
    Done:  m.Done ,
    ErrorMessage:  m.ErrorMessage ,
    JustEmailRegister:  m.JustEmailRegister ,
    SmsNumbers:  m.SmsNumbers ,
    IsLogin:  m.IsLogin ,
}
return r
}

func(m *ConfirmCodeParam_Flat)ToPB() *ConfirmCodeParam {
r := &ConfirmCodeParam{
    Hash:  m.Hash ,
    Phone:  m.Phone ,
    Code:  int32(m.Code) ,
}
return r
}

func(m *ConfirmCodeResponse_Flat)ToPB() *ConfirmCodeResponse {
r := &ConfirmCodeResponse{
    Done:  m.Done ,
    ErrorMessage:  m.ErrorMessage ,

}
return r
}

func(m *SingUpParam_Flat)ToPB() *SingUpParam {
r := &SingUpParam{
    Hash:  m.Hash ,
    FirstName:  m.FirstName ,
    LastName:  m.LastName ,
    UserName:  m.UserName ,
    Phone:  m.Phone ,
    Email:  m.Email ,
}
return r
}

func(m *SingUpResponse_Flat)ToPB() *SingUpResponse {
r := &SingUpResponse{
    Done:  m.Done ,
    ErrorMessage:  m.ErrorMessage ,

}
return r
}

func(m *SingInParam_Flat)ToPB() *SingInParam {
r := &SingInParam{
    UserNamePhoneEmail:  m.UserNamePhoneEmail ,
    Password:  m.Password ,
}
return r
}

func(m *SingInResponse_Flat)ToPB() *SingInResponse {
r := &SingInResponse{
    Done:  m.Done ,
    ErrorMessage:  m.ErrorMessage ,

}
return r
}

func(m *LogOutParam_Flat)ToPB() *LogOutParam {
r := &LogOutParam{
}
return r
}

func(m *LogOutResponse_Flat)ToPB() *LogOutResponse {
r := &LogOutResponse{
    Done:  m.Done ,
    ErrorMessage:  m.ErrorMessage ,
}
return r
}

func(m *MediaView_Flat)ToPB() *MediaView {
r := &MediaView{
    FileRefId:  int64(m.FileRefId) ,
    UserId:  int64(m.UserId) ,
    Name:  m.Name ,
    Width:  int32(m.Width) ,
    Height:  int32(m.Height) ,
    Duration:  int32(m.Duration) ,
    Extension:  m.Extension ,
    UrlSource:  m.UrlSource ,
}
return r
}

func(m *ActionView_Flat)ToPB() *ActionView {
r := &ActionView{
    ActionId:  int64(m.ActionId) ,
    ActorUserId:  int32(m.ActorUserId) ,
    ActionTypeEnum:  int32(m.ActionTypeEnum) ,
    PeerUserId:  int32(m.PeerUserId) ,
    PostId:  int64(m.PostId) ,
    CommentId:  int64(m.CommentId) ,
    Murmur64Hash:  int64(m.Murmur64Hash) ,
    CreatedTime:  int32(m.CreatedTime) ,





}
return r
}

func(m *CommentView_Flat)ToPB() *CommentView {
r := &CommentView{
    CommentId:  int64(m.CommentId) ,
    UserId:  int32(m.UserId) ,
    PostId:  int64(m.PostId) ,
    Text:  m.Text ,
    LikesCount:  int32(m.LikesCount) ,
    CreatedTime:  int32(m.CreatedTime) ,

}
return r
}

func(m *PostView_Flat)ToPB() *PostView {
r := &PostView{
    PostId:  int64(m.PostId) ,
    UserId:  int32(m.UserId) ,

    Text:  m.Text ,
    RichText:  m.RichText ,
    MediaCount:  int32(m.MediaCount) ,
    SharedTo:  int32(m.SharedTo) ,
    DisableComment:  int32(m.DisableComment) ,
    HasTag:  int32(m.HasTag) ,
    CommentsCount:  int32(m.CommentsCount) ,
    LikesCount:  int32(m.LikesCount) ,
    ViewsCount:  int32(m.ViewsCount) ,
    EditedTime:  int32(m.EditedTime) ,
    CreatedTime:  int32(m.CreatedTime) ,
    ReSharedPostId:  int64(m.ReSharedPostId) ,
    DidILiked:  m.DidILiked ,
    DidIReShared:  m.DidIReShared ,




}
return r
}

func(m *UserView_Flat)ToPB() *UserView {
r := &UserView{
    UserId:  int32(m.UserId) ,
    UserName:  m.UserName ,
    FirstName:  m.FirstName ,
    LastName:  m.LastName ,
    AvatarRefId:  int64(m.AvatarRefId) ,
    ProfilePrivacy:  int32(m.ProfilePrivacy) ,
    Phone:  int64(m.Phone) ,
    About:  m.About ,
    FollowersCount:  int32(m.FollowersCount) ,
    FollowingCount:  int32(m.FollowingCount) ,
    PostsCount:  int32(m.PostsCount) ,
    MediaCount:  int32(m.MediaCount) ,

    LastActiveTime:  int32(m.LastActiveTime) ,
    LastActiveTimeShow:  m.LastActiveTimeShow ,

}
return r
}

func(m *SelfUserView_Flat)ToPB() *SelfUserView {
r := &SelfUserView{

    ProfilePrivacy:  int32(m.ProfilePrivacy) ,
    OnlinePrivacy:  int32(m.OnlinePrivacy) ,
    CallPrivacy:  int32(m.CallPrivacy) ,
    AddToGroupPrivacy:  int32(m.AddToGroupPrivacy) ,
    SeenMessagePrivacy:  int32(m.SeenMessagePrivacy) ,
}
return r
}

func(m *Error_Flat)ToPB() *Error {
r := &Error{

    ShowError:  m.ShowError ,
    ErrorMessage:  m.ErrorMessage ,
}
return r
}



///// folding ///

var SendConfirmCodeParam__FOlD = &SendConfirmCodeParam{
        Hash:  "" ,
        Phone:  "" ,
        CountryCode:  "" ,
        Resend:  false ,
}


var SendConfirmCodeResponse__FOlD = &SendConfirmCodeResponse{
        Done:  false ,
        ErrorMessage:  "" ,
        JustEmailRegister:  false ,
        SmsNumbers:  "" ,
        IsLogin:  false ,
}


var ConfirmCodeParam__FOlD = &ConfirmCodeParam{
        Hash:  "" ,
        Phone:  "" ,
        Code:  0 ,
}


var ConfirmCodeResponse__FOlD = &ConfirmCodeResponse{
        Done:  false ,
        ErrorMessage:  "" ,

}


var SingUpParam__FOlD = &SingUpParam{
        Hash:  "" ,
        FirstName:  "" ,
        LastName:  "" ,
        UserName:  "" ,
        Phone:  "" ,
        Email:  "" ,
}


var SingUpResponse__FOlD = &SingUpResponse{
        Done:  false ,
        ErrorMessage:  "" ,

}


var SingInParam__FOlD = &SingInParam{
        UserNamePhoneEmail:  "" ,
        Password:  "" ,
}


var SingInResponse__FOlD = &SingInResponse{
        Done:  false ,
        ErrorMessage:  "" ,

}


var LogOutParam__FOlD = &LogOutParam{
}


var LogOutResponse__FOlD = &LogOutResponse{
        Done:  false ,
        ErrorMessage:  "" ,
}


var MediaView__FOlD = &MediaView{
        FileRefId:  0 ,
        UserId:  0 ,
        Name:  "" ,
        Width:  0 ,
        Height:  0 ,
        Duration:  0 ,
        Extension:  "" ,
        UrlSource:  "" ,
}


var ActionView__FOlD = &ActionView{
        ActionId:  0 ,
        ActorUserId:  0 ,
        ActionTypeEnum:  0 ,
        PeerUserId:  0 ,
        PostId:  0 ,
        CommentId:  0 ,
        Murmur64Hash:  0 ,
        CreatedTime:  0 ,





}


var CommentView__FOlD = &CommentView{
        CommentId:  0 ,
        UserId:  0 ,
        PostId:  0 ,
        Text:  "" ,
        LikesCount:  0 ,
        CreatedTime:  0 ,

}


var PostView__FOlD = &PostView{
        PostId:  0 ,
        UserId:  0 ,

        Text:  "" ,
        RichText:  "" ,
        MediaCount:  0 ,
        SharedTo:  0 ,
        DisableComment:  0 ,
        HasTag:  0 ,
        CommentsCount:  0 ,
        LikesCount:  0 ,
        ViewsCount:  0 ,
        EditedTime:  0 ,
        CreatedTime:  0 ,
        ReSharedPostId:  0 ,
        DidILiked:  false ,
        DidIReShared:  false ,




}


var UserView__FOlD = &UserView{
        UserId:  0 ,
        UserName:  "" ,
        FirstName:  "" ,
        LastName:  "" ,
        AvatarRefId:  0 ,
        ProfilePrivacy:  0 ,
        Phone:  0 ,
        About:  "" ,
        FollowersCount:  0 ,
        FollowingCount:  0 ,
        PostsCount:  0 ,
        MediaCount:  0 ,

        LastActiveTime:  0 ,
        LastActiveTimeShow:  "" ,

}


var SelfUserView__FOlD = &SelfUserView{

        ProfilePrivacy:  0 ,
        OnlinePrivacy:  0 ,
        CallPrivacy:  0 ,
        AddToGroupPrivacy:  0 ,
        SeenMessagePrivacy:  0 ,
}


var Error__FOlD = &Error{

        ShowError:  false ,
        ErrorMessage:  "" ,
}



*/
