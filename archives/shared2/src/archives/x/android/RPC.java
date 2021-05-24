package ir.ms.pb;

import com.mardomsara.social.pipe.*;
import android.util.Log;

public class RPC {

public static class RPC_Auth {
	
        public static interface SendConfirmCode_ResultHandler{
    		void onResult(SendConfirmCodeResponse res);
        }
        public static interface ConfirmCode_ResultHandler{
    		void onResult(ConfirmCodeResponse res);
        }
        public static interface SingUp_ResultHandler{
    		void onResult(SingUpResponse res);
        }
        public static interface SingIn_ResultHandler{
    		void onResult(SingInResponse res);
        }
        public static interface LogOut_ResultHandler{
    		void onResult(LogOutResponse res);
        }
    public static void SendConfirmCode( SendConfirmCodeParam param ,SendConfirmCode_ResultHandler resultHandler, ErrorCallback errorCallback){
		SendConfirmCodeImpl(param,resultHandler, errorCallback ,false,"");
    }

    public static void SendConfirmCode_Offline(String offlineKey, SendConfirmCodeParam param ,SendConfirmCode_ResultHandler resultHandler, ErrorCallback errorCallback){
    		SendConfirmCodeImpl(param,resultHandler, errorCallback ,true ,offlineKey);
    }

    private static void SendConfirmCodeImpl( SendConfirmCodeParam param ,SendConfirmCode_ResultHandler resultHandler, ErrorCallback errorCallback , Boolean offline,String offlineKey){
    		SuccessCallback callback = null;
    		if(resultHandler != null){
    			callback = new SuccessCallback() {
    				@Override
    				public Object handle(byte[] data) {
    					Log.i("RPC ws", "handling rpc respnse for: SendConfirmCode with respose class SendConfirmCodeResponse");
    					SendConfirmCodeResponse d = null;
    					try {
    						 d =SendConfirmCodeResponse.parseFrom(data);
    						resultHandler.onResult(d);
    					}catch (com.google.protobuf.InvalidProtocolBufferException e){
    						Log.d("RPC ws", "parsing protocol buffer is faild: SendConfirmCodeResponse");
    					}
    					return d;
    				}
    			};
    		}

    		if(offline){
    			Pipe.sendOffline(offlineKey,"RPC_Auth.SendConfirmCode", param, callback ,errorCallback);
    		}else{
    		  Pipe.send("RPC_Auth.SendConfirmCode", param, callback ,errorCallback);
    		}
        }
    public static void ConfirmCode( ConfirmCodeParam param ,ConfirmCode_ResultHandler resultHandler, ErrorCallback errorCallback){
		ConfirmCodeImpl(param,resultHandler, errorCallback ,false,"");
    }

    public static void ConfirmCode_Offline(String offlineKey, ConfirmCodeParam param ,ConfirmCode_ResultHandler resultHandler, ErrorCallback errorCallback){
    		ConfirmCodeImpl(param,resultHandler, errorCallback ,true ,offlineKey);
    }

    private static void ConfirmCodeImpl( ConfirmCodeParam param ,ConfirmCode_ResultHandler resultHandler, ErrorCallback errorCallback , Boolean offline,String offlineKey){
    		SuccessCallback callback = null;
    		if(resultHandler != null){
    			callback = new SuccessCallback() {
    				@Override
    				public Object handle(byte[] data) {
    					Log.i("RPC ws", "handling rpc respnse for: ConfirmCode with respose class ConfirmCodeResponse");
    					ConfirmCodeResponse d = null;
    					try {
    						 d =ConfirmCodeResponse.parseFrom(data);
    						resultHandler.onResult(d);
    					}catch (com.google.protobuf.InvalidProtocolBufferException e){
    						Log.d("RPC ws", "parsing protocol buffer is faild: ConfirmCodeResponse");
    					}
    					return d;
    				}
    			};
    		}

    		if(offline){
    			Pipe.sendOffline(offlineKey,"RPC_Auth.ConfirmCode", param, callback ,errorCallback);
    		}else{
    		  Pipe.send("RPC_Auth.ConfirmCode", param, callback ,errorCallback);
    		}
        }
    public static void SingUp( SingUpParam param ,SingUp_ResultHandler resultHandler, ErrorCallback errorCallback){
		SingUpImpl(param,resultHandler, errorCallback ,false,"");
    }

    public static void SingUp_Offline(String offlineKey, SingUpParam param ,SingUp_ResultHandler resultHandler, ErrorCallback errorCallback){
    		SingUpImpl(param,resultHandler, errorCallback ,true ,offlineKey);
    }

    private static void SingUpImpl( SingUpParam param ,SingUp_ResultHandler resultHandler, ErrorCallback errorCallback , Boolean offline,String offlineKey){
    		SuccessCallback callback = null;
    		if(resultHandler != null){
    			callback = new SuccessCallback() {
    				@Override
    				public Object handle(byte[] data) {
    					Log.i("RPC ws", "handling rpc respnse for: SingUp with respose class SingUpResponse");
    					SingUpResponse d = null;
    					try {
    						 d =SingUpResponse.parseFrom(data);
    						resultHandler.onResult(d);
    					}catch (com.google.protobuf.InvalidProtocolBufferException e){
    						Log.d("RPC ws", "parsing protocol buffer is faild: SingUpResponse");
    					}
    					return d;
    				}
    			};
    		}

    		if(offline){
    			Pipe.sendOffline(offlineKey,"RPC_Auth.SingUp", param, callback ,errorCallback);
    		}else{
    		  Pipe.send("RPC_Auth.SingUp", param, callback ,errorCallback);
    		}
        }
    public static void SingIn( SingInParam param ,SingIn_ResultHandler resultHandler, ErrorCallback errorCallback){
		SingInImpl(param,resultHandler, errorCallback ,false,"");
    }

    public static void SingIn_Offline(String offlineKey, SingInParam param ,SingIn_ResultHandler resultHandler, ErrorCallback errorCallback){
    		SingInImpl(param,resultHandler, errorCallback ,true ,offlineKey);
    }

    private static void SingInImpl( SingInParam param ,SingIn_ResultHandler resultHandler, ErrorCallback errorCallback , Boolean offline,String offlineKey){
    		SuccessCallback callback = null;
    		if(resultHandler != null){
    			callback = new SuccessCallback() {
    				@Override
    				public Object handle(byte[] data) {
    					Log.i("RPC ws", "handling rpc respnse for: SingIn with respose class SingInResponse");
    					SingInResponse d = null;
    					try {
    						 d =SingInResponse.parseFrom(data);
    						resultHandler.onResult(d);
    					}catch (com.google.protobuf.InvalidProtocolBufferException e){
    						Log.d("RPC ws", "parsing protocol buffer is faild: SingInResponse");
    					}
    					return d;
    				}
    			};
    		}

    		if(offline){
    			Pipe.sendOffline(offlineKey,"RPC_Auth.SingIn", param, callback ,errorCallback);
    		}else{
    		  Pipe.send("RPC_Auth.SingIn", param, callback ,errorCallback);
    		}
        }
    public static void LogOut( LogOutParam param ,LogOut_ResultHandler resultHandler, ErrorCallback errorCallback){
		LogOutImpl(param,resultHandler, errorCallback ,false,"");
    }

    public static void LogOut_Offline(String offlineKey, LogOutParam param ,LogOut_ResultHandler resultHandler, ErrorCallback errorCallback){
    		LogOutImpl(param,resultHandler, errorCallback ,true ,offlineKey);
    }

    private static void LogOutImpl( LogOutParam param ,LogOut_ResultHandler resultHandler, ErrorCallback errorCallback , Boolean offline,String offlineKey){
    		SuccessCallback callback = null;
    		if(resultHandler != null){
    			callback = new SuccessCallback() {
    				@Override
    				public Object handle(byte[] data) {
    					Log.i("RPC ws", "handling rpc respnse for: LogOut with respose class LogOutResponse");
    					LogOutResponse d = null;
    					try {
    						 d =LogOutResponse.parseFrom(data);
    						resultHandler.onResult(d);
    					}catch (com.google.protobuf.InvalidProtocolBufferException e){
    						Log.d("RPC ws", "parsing protocol buffer is faild: LogOutResponse");
    					}
    					return d;
    				}
    			};
    		}

    		if(offline){
    			Pipe.sendOffline(offlineKey,"RPC_Auth.LogOut", param, callback ,errorCallback);
    		}else{
    		  Pipe.send("RPC_Auth.LogOut", param, callback ,errorCallback);
    		}
        }}
	
}
/*

RPC_INTERFACES.RPC_Auth RPC_Auth_Handeler = null;
	
*/