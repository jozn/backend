package ir.ms.pb;

public class PBFlatTypes {

	public class SendConfirmCodeParam {
	   public String Hash;
	   public String Phone;
	   public String CountryCode;
	   public boolean Resend;
	}
	/*
	folding
	PBFlatTypes.SendConfirmCodeParam t = new PBFlatTypes.SendConfirmCodeParam();
    t.setHash();
    t.setPhone();
    t.setCountryCode();
    t.setResend();
	*/

	/*
	PBFlatTypes.SendConfirmCodeParam t = new PBFlatTypes.SendConfirmCodeParam();
	t.Hash = ;
	t.Phone = ;
	t.CountryCode = ;
	t.Resend = ;
	*/

	/*
	SendConfirmCodeParam t = new SendConfirmCodeParam();
	t.Hash = m.getHash() ;
	t.Phone = m.getPhone() ;
	t.CountryCode = m.getCountryCode() ;
	t.Resend = m.getResend() ;
	*/

	public class SendConfirmCodeResponse {
	   public boolean Done;
	   public String ErrorMessage;
	   public boolean JustEmailRegister;
	   public String SmsNumbers;
	   public boolean IsLogin;
	}
	/*
	folding
	PBFlatTypes.SendConfirmCodeResponse t = new PBFlatTypes.SendConfirmCodeResponse();
    t.setDone();
    t.setErrorMessage();
    t.setJustEmailRegister();
    t.setSmsNumbers();
    t.setIsLogin();
	*/

	/*
	PBFlatTypes.SendConfirmCodeResponse t = new PBFlatTypes.SendConfirmCodeResponse();
	t.Done = ;
	t.ErrorMessage = ;
	t.JustEmailRegister = ;
	t.SmsNumbers = ;
	t.IsLogin = ;
	*/

	/*
	SendConfirmCodeResponse t = new SendConfirmCodeResponse();
	t.Done = m.getDone() ;
	t.ErrorMessage = m.getErrorMessage() ;
	t.JustEmailRegister = m.getJustEmailRegister() ;
	t.SmsNumbers = m.getSmsNumbers() ;
	t.IsLogin = m.getIsLogin() ;
	*/

	public class ConfirmCodeParam {
	   public String Hash;
	   public String Phone;
	   public int Code;
	}
	/*
	folding
	PBFlatTypes.ConfirmCodeParam t = new PBFlatTypes.ConfirmCodeParam();
    t.setHash();
    t.setPhone();
    t.setCode();
	*/

	/*
	PBFlatTypes.ConfirmCodeParam t = new PBFlatTypes.ConfirmCodeParam();
	t.Hash = ;
	t.Phone = ;
	t.Code = ;
	*/

	/*
	ConfirmCodeParam t = new ConfirmCodeParam();
	t.Hash = m.getHash() ;
	t.Phone = m.getPhone() ;
	t.Code = m.getCode() ;
	*/

	public class ConfirmCodeResponse {
	   public boolean Done;
	   public String ErrorMessage;
	   public SelfUserView SelfUserView;
	}
	/*
	folding
	PBFlatTypes.ConfirmCodeResponse t = new PBFlatTypes.ConfirmCodeResponse();
    t.setDone();
    t.setErrorMessage();
    t.setSelfUserView();
	*/

	/*
	PBFlatTypes.ConfirmCodeResponse t = new PBFlatTypes.ConfirmCodeResponse();
	t.Done = ;
	t.ErrorMessage = ;
	t.SelfUserView = ;
	*/

	/*
	ConfirmCodeResponse t = new ConfirmCodeResponse();
	t.Done = m.getDone() ;
	t.ErrorMessage = m.getErrorMessage() ;
	t.SelfUserView = m.getSelfUserView() ;
	*/

	public class SingUpParam {
	   public String Hash;
	   public String FirstName;
	   public String LastName;
	   public String UserName;
	   public String Phone;
	   public String Email;
	}
	/*
	folding
	PBFlatTypes.SingUpParam t = new PBFlatTypes.SingUpParam();
    t.setHash();
    t.setFirstName();
    t.setLastName();
    t.setUserName();
    t.setPhone();
    t.setEmail();
	*/

	/*
	PBFlatTypes.SingUpParam t = new PBFlatTypes.SingUpParam();
	t.Hash = ;
	t.FirstName = ;
	t.LastName = ;
	t.UserName = ;
	t.Phone = ;
	t.Email = ;
	*/

	/*
	SingUpParam t = new SingUpParam();
	t.Hash = m.getHash() ;
	t.FirstName = m.getFirstName() ;
	t.LastName = m.getLastName() ;
	t.UserName = m.getUserName() ;
	t.Phone = m.getPhone() ;
	t.Email = m.getEmail() ;
	*/

	public class SingUpResponse {
	   public boolean Done;
	   public String ErrorMessage;
	   public SelfUserView SelfUserView;
	}
	/*
	folding
	PBFlatTypes.SingUpResponse t = new PBFlatTypes.SingUpResponse();
    t.setDone();
    t.setErrorMessage();
    t.setSelfUserView();
	*/

	/*
	PBFlatTypes.SingUpResponse t = new PBFlatTypes.SingUpResponse();
	t.Done = ;
	t.ErrorMessage = ;
	t.SelfUserView = ;
	*/

	/*
	SingUpResponse t = new SingUpResponse();
	t.Done = m.getDone() ;
	t.ErrorMessage = m.getErrorMessage() ;
	t.SelfUserView = m.getSelfUserView() ;
	*/

	public class SingInParam {
	   public String UserNamePhoneEmail;
	   public String Password;
	}
	/*
	folding
	PBFlatTypes.SingInParam t = new PBFlatTypes.SingInParam();
    t.setUserNamePhoneEmail();
    t.setPassword();
	*/

	/*
	PBFlatTypes.SingInParam t = new PBFlatTypes.SingInParam();
	t.UserNamePhoneEmail = ;
	t.Password = ;
	*/

	/*
	SingInParam t = new SingInParam();
	t.UserNamePhoneEmail = m.getUserNamePhoneEmail() ;
	t.Password = m.getPassword() ;
	*/

	public class SingInResponse {
	   public boolean Done;
	   public String ErrorMessage;
	   public SelfUserView SelfUserView;
	}
	/*
	folding
	PBFlatTypes.SingInResponse t = new PBFlatTypes.SingInResponse();
    t.setDone();
    t.setErrorMessage();
    t.setSelfUserView();
	*/

	/*
	PBFlatTypes.SingInResponse t = new PBFlatTypes.SingInResponse();
	t.Done = ;
	t.ErrorMessage = ;
	t.SelfUserView = ;
	*/

	/*
	SingInResponse t = new SingInResponse();
	t.Done = m.getDone() ;
	t.ErrorMessage = m.getErrorMessage() ;
	t.SelfUserView = m.getSelfUserView() ;
	*/

	public class LogOutParam {
	}
	/*
	folding
	PBFlatTypes.LogOutParam t = new PBFlatTypes.LogOutParam();
	*/

	/*
	PBFlatTypes.LogOutParam t = new PBFlatTypes.LogOutParam();
	*/

	/*
	LogOutParam t = new LogOutParam();
	*/

	public class LogOutResponse {
	   public boolean Done;
	   public String ErrorMessage;
	}
	/*
	folding
	PBFlatTypes.LogOutResponse t = new PBFlatTypes.LogOutResponse();
    t.setDone();
    t.setErrorMessage();
	*/

	/*
	PBFlatTypes.LogOutResponse t = new PBFlatTypes.LogOutResponse();
	t.Done = ;
	t.ErrorMessage = ;
	*/

	/*
	LogOutResponse t = new LogOutResponse();
	t.Done = m.getDone() ;
	t.ErrorMessage = m.getErrorMessage() ;
	*/

	public class MediaView {
	   public long FileRefId;
	   public long UserId;
	   public String Name;
	   public int Width;
	   public int Height;
	   public int Duration;
	   public String Extension;
	   public String UrlSource;
	}
	/*
	folding
	PBFlatTypes.MediaView t = new PBFlatTypes.MediaView();
    t.setFileRefId();
    t.setUserId();
    t.setName();
    t.setWidth();
    t.setHeight();
    t.setDuration();
    t.setExtension();
    t.setUrlSource();
	*/

	/*
	PBFlatTypes.MediaView t = new PBFlatTypes.MediaView();
	t.FileRefId = ;
	t.UserId = ;
	t.Name = ;
	t.Width = ;
	t.Height = ;
	t.Duration = ;
	t.Extension = ;
	t.UrlSource = ;
	*/

	/*
	MediaView t = new MediaView();
	t.FileRefId = m.getFileRefId() ;
	t.UserId = m.getUserId() ;
	t.Name = m.getName() ;
	t.Width = m.getWidth() ;
	t.Height = m.getHeight() ;
	t.Duration = m.getDuration() ;
	t.Extension = m.getExtension() ;
	t.UrlSource = m.getUrlSource() ;
	*/

	public class ActionView {
	   public long ActionId;
	   public int ActorUserId;
	   public int ActionTypeEnum;
	   public int PeerUserId;
	   public long PostId;
	   public long CommentId;
	   public long Murmur64Hash;
	   public int CreatedTime;
	   public UserView ActorUserView;
	   public PostView PostView;
	   public CommentView CommentView;
	   public UserView FollowedUserView;
	   public UserView ContentOwenerUserView;
	}
	/*
	folding
	PBFlatTypes.ActionView t = new PBFlatTypes.ActionView();
    t.setActionId();
    t.setActorUserId();
    t.setActionTypeEnum();
    t.setPeerUserId();
    t.setPostId();
    t.setCommentId();
    t.setMurmur64Hash();
    t.setCreatedTime();
    t.setActorUserView();
    t.setPostView();
    t.setCommentView();
    t.setFollowedUserView();
    t.setContentOwenerUserView();
	*/

	/*
	PBFlatTypes.ActionView t = new PBFlatTypes.ActionView();
	t.ActionId = ;
	t.ActorUserId = ;
	t.ActionTypeEnum = ;
	t.PeerUserId = ;
	t.PostId = ;
	t.CommentId = ;
	t.Murmur64Hash = ;
	t.CreatedTime = ;
	t.ActorUserView = ;
	t.PostView = ;
	t.CommentView = ;
	t.FollowedUserView = ;
	t.ContentOwenerUserView = ;
	*/

	/*
	ActionView t = new ActionView();
	t.ActionId = m.getActionId() ;
	t.ActorUserId = m.getActorUserId() ;
	t.ActionTypeEnum = m.getActionTypeEnum() ;
	t.PeerUserId = m.getPeerUserId() ;
	t.PostId = m.getPostId() ;
	t.CommentId = m.getCommentId() ;
	t.Murmur64Hash = m.getMurmur64Hash() ;
	t.CreatedTime = m.getCreatedTime() ;
	t.ActorUserView = m.getActorUserView() ;
	t.PostView = m.getPostView() ;
	t.CommentView = m.getCommentView() ;
	t.FollowedUserView = m.getFollowedUserView() ;
	t.ContentOwenerUserView = m.getContentOwenerUserView() ;
	*/

	public class CommentView {
	   public long CommentId;
	   public int UserId;
	   public long PostId;
	   public String Text;
	   public int LikesCount;
	   public int CreatedTime;
	   public UserView SenderUserView;
	}
	/*
	folding
	PBFlatTypes.CommentView t = new PBFlatTypes.CommentView();
    t.setCommentId();
    t.setUserId();
    t.setPostId();
    t.setText();
    t.setLikesCount();
    t.setCreatedTime();
    t.setSenderUserView();
	*/

	/*
	PBFlatTypes.CommentView t = new PBFlatTypes.CommentView();
	t.CommentId = ;
	t.UserId = ;
	t.PostId = ;
	t.Text = ;
	t.LikesCount = ;
	t.CreatedTime = ;
	t.SenderUserView = ;
	*/

	/*
	CommentView t = new CommentView();
	t.CommentId = m.getCommentId() ;
	t.UserId = m.getUserId() ;
	t.PostId = m.getPostId() ;
	t.Text = m.getText() ;
	t.LikesCount = m.getLikesCount() ;
	t.CreatedTime = m.getCreatedTime() ;
	t.SenderUserView = m.getSenderUserView() ;
	*/

	public class PostView {
	   public long PostId;
	   public int UserId;
	   public PostTypeEnum PostTypeEnum;
	   public String Text;
	   public String RichText;
	   public int MediaCount;
	   public int SharedTo;
	   public int DisableComment;
	   public int HasTag;
	   public int CommentsCount;
	   public int LikesCount;
	   public int ViewsCount;
	   public int EditedTime;
	   public int CreatedTime;
	   public long ReSharedPostId;
	   public boolean DidILiked;
	   public boolean DidIReShared;
	   public UserView SenderUserView;
	   public UserView ReSharedUserView;
	   public MediaView MediaView;
	   public MediaView MediaViewList;
	}
	/*
	folding
	PBFlatTypes.PostView t = new PBFlatTypes.PostView();
    t.setPostId();
    t.setUserId();
    t.setPostTypeEnum();
    t.setText();
    t.setRichText();
    t.setMediaCount();
    t.setSharedTo();
    t.setDisableComment();
    t.setHasTag();
    t.setCommentsCount();
    t.setLikesCount();
    t.setViewsCount();
    t.setEditedTime();
    t.setCreatedTime();
    t.setReSharedPostId();
    t.setDidILiked();
    t.setDidIReShared();
    t.setSenderUserView();
    t.setReSharedUserView();
    t.setMediaView();
    t.setMediaViewList();
	*/

	/*
	PBFlatTypes.PostView t = new PBFlatTypes.PostView();
	t.PostId = ;
	t.UserId = ;
	t.PostTypeEnum = ;
	t.Text = ;
	t.RichText = ;
	t.MediaCount = ;
	t.SharedTo = ;
	t.DisableComment = ;
	t.HasTag = ;
	t.CommentsCount = ;
	t.LikesCount = ;
	t.ViewsCount = ;
	t.EditedTime = ;
	t.CreatedTime = ;
	t.ReSharedPostId = ;
	t.DidILiked = ;
	t.DidIReShared = ;
	t.SenderUserView = ;
	t.ReSharedUserView = ;
	t.MediaView = ;
	t.MediaViewList = ;
	*/

	/*
	PostView t = new PostView();
	t.PostId = m.getPostId() ;
	t.UserId = m.getUserId() ;
	t.PostTypeEnum = m.getPostTypeEnum() ;
	t.Text = m.getText() ;
	t.RichText = m.getRichText() ;
	t.MediaCount = m.getMediaCount() ;
	t.SharedTo = m.getSharedTo() ;
	t.DisableComment = m.getDisableComment() ;
	t.HasTag = m.getHasTag() ;
	t.CommentsCount = m.getCommentsCount() ;
	t.LikesCount = m.getLikesCount() ;
	t.ViewsCount = m.getViewsCount() ;
	t.EditedTime = m.getEditedTime() ;
	t.CreatedTime = m.getCreatedTime() ;
	t.ReSharedPostId = m.getReSharedPostId() ;
	t.DidILiked = m.getDidILiked() ;
	t.DidIReShared = m.getDidIReShared() ;
	t.SenderUserView = m.getSenderUserView() ;
	t.ReSharedUserView = m.getReSharedUserView() ;
	t.MediaView = m.getMediaView() ;
	t.MediaViewList = m.getMediaViewList() ;
	*/

	public class UserView {
	   public int UserId;
	   public String UserName;
	   public String FirstName;
	   public String LastName;
	   public long AvatarRefId;
	   public int ProfilePrivacy;
	   public long Phone;
	   public String About;
	   public int FollowersCount;
	   public int FollowingCount;
	   public int PostsCount;
	   public int MediaCount;
	   public UserOnlineStatusEnum UserOnlineStatusEnum;
	   public int LastActiveTime;
	   public String LastActiveTimeShow;
	   public FollowingEnum MyFollwing;
	}
	/*
	folding
	PBFlatTypes.UserView t = new PBFlatTypes.UserView();
    t.setUserId();
    t.setUserName();
    t.setFirstName();
    t.setLastName();
    t.setAvatarRefId();
    t.setProfilePrivacy();
    t.setPhone();
    t.setAbout();
    t.setFollowersCount();
    t.setFollowingCount();
    t.setPostsCount();
    t.setMediaCount();
    t.setUserOnlineStatusEnum();
    t.setLastActiveTime();
    t.setLastActiveTimeShow();
    t.setMyFollwing();
	*/

	/*
	PBFlatTypes.UserView t = new PBFlatTypes.UserView();
	t.UserId = ;
	t.UserName = ;
	t.FirstName = ;
	t.LastName = ;
	t.AvatarRefId = ;
	t.ProfilePrivacy = ;
	t.Phone = ;
	t.About = ;
	t.FollowersCount = ;
	t.FollowingCount = ;
	t.PostsCount = ;
	t.MediaCount = ;
	t.UserOnlineStatusEnum = ;
	t.LastActiveTime = ;
	t.LastActiveTimeShow = ;
	t.MyFollwing = ;
	*/

	/*
	UserView t = new UserView();
	t.UserId = m.getUserId() ;
	t.UserName = m.getUserName() ;
	t.FirstName = m.getFirstName() ;
	t.LastName = m.getLastName() ;
	t.AvatarRefId = m.getAvatarRefId() ;
	t.ProfilePrivacy = m.getProfilePrivacy() ;
	t.Phone = m.getPhone() ;
	t.About = m.getAbout() ;
	t.FollowersCount = m.getFollowersCount() ;
	t.FollowingCount = m.getFollowingCount() ;
	t.PostsCount = m.getPostsCount() ;
	t.MediaCount = m.getMediaCount() ;
	t.UserOnlineStatusEnum = m.getUserOnlineStatusEnum() ;
	t.LastActiveTime = m.getLastActiveTime() ;
	t.LastActiveTimeShow = m.getLastActiveTimeShow() ;
	t.MyFollwing = m.getMyFollwing() ;
	*/

	public class SelfUserView {
	   public UserView UserView;
	   public int ProfilePrivacy;
	   public int OnlinePrivacy;
	   public int CallPrivacy;
	   public int AddToGroupPrivacy;
	   public int SeenMessagePrivacy;
	}
	/*
	folding
	PBFlatTypes.SelfUserView t = new PBFlatTypes.SelfUserView();
    t.setUserView();
    t.setProfilePrivacy();
    t.setOnlinePrivacy();
    t.setCallPrivacy();
    t.setAddToGroupPrivacy();
    t.setSeenMessagePrivacy();
	*/

	/*
	PBFlatTypes.SelfUserView t = new PBFlatTypes.SelfUserView();
	t.UserView = ;
	t.ProfilePrivacy = ;
	t.OnlinePrivacy = ;
	t.CallPrivacy = ;
	t.AddToGroupPrivacy = ;
	t.SeenMessagePrivacy = ;
	*/

	/*
	SelfUserView t = new SelfUserView();
	t.UserView = m.getUserView() ;
	t.ProfilePrivacy = m.getProfilePrivacy() ;
	t.OnlinePrivacy = m.getOnlinePrivacy() ;
	t.CallPrivacy = m.getCallPrivacy() ;
	t.AddToGroupPrivacy = m.getAddToGroupPrivacy() ;
	t.SeenMessagePrivacy = m.getSeenMessagePrivacy() ;
	*/

	public class Error {
	   public ServerErrors Error;
	   public boolean ShowError;
	   public String ErrorMessage;
	}
	/*
	folding
	PBFlatTypes.Error t = new PBFlatTypes.Error();
    t.setError();
    t.setShowError();
    t.setErrorMessage();
	*/

	/*
	PBFlatTypes.Error t = new PBFlatTypes.Error();
	t.Error = ;
	t.ShowError = ;
	t.ErrorMessage = ;
	*/

	/*
	Error t = new Error();
	t.Error = m.getError() ;
	t.ShowError = m.getShowError() ;
	t.ErrorMessage = m.getErrorMessage() ;
	*/

	
}

/*

RPC_HANDLERS.RPC_Auth RPC_Auth_Handeler = null;
	
*/