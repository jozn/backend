package ir.ms.pb;

import java.util.Map;
import java.util.concurrent.ConcurrentHashMap;
import com.google.protobuf.ByteString;

import ir.ms.pb.PB_SyncParam_GetGeneralUpdates;

/**
 * Created by Hamid on 9/30/2017.
 */

public class RpcNameToResponseMapper {
	public abstract static class RpcHelper2{
		public Class<? extends com.google.protobuf.GeneratedMessageLite> clazz;
		//public Parser parser;

		public abstract com.google.protobuf.GeneratedMessageLite parseData(ByteString byteString) throws com.google.protobuf.InvalidProtocolBufferException;
	}

	private static Map<String, RpcHelper2> mp = new ConcurrentHashMap<>();

	public static Map<String, RpcHelper2> getMap(){
		if(mp.size() < 1){
			fill();
		}
		return mp;
	}

	private static synchronized void fill(){
	 	
	 	// Service RPC_Auth
			
		mp.put("RPC_Auth.SendConfirmCode",  new RpcHelper2(){
				@Override
				public com.google.protobuf.GeneratedMessageLite parseData(ByteString byteString) throws com.google.protobuf.InvalidProtocolBufferException{
					return SendConfirmCodeResponse.parseFrom(byteString);
				}
             });
		mp.put("RPC_Auth.ConfirmCode",  new RpcHelper2(){
				@Override
				public com.google.protobuf.GeneratedMessageLite parseData(ByteString byteString) throws com.google.protobuf.InvalidProtocolBufferException{
					return ConfirmCodeResponse.parseFrom(byteString);
				}
             });
		mp.put("RPC_Auth.SingUp",  new RpcHelper2(){
				@Override
				public com.google.protobuf.GeneratedMessageLite parseData(ByteString byteString) throws com.google.protobuf.InvalidProtocolBufferException{
					return SingUpResponse.parseFrom(byteString);
				}
             });
		mp.put("RPC_Auth.SingIn",  new RpcHelper2(){
				@Override
				public com.google.protobuf.GeneratedMessageLite parseData(ByteString byteString) throws com.google.protobuf.InvalidProtocolBufferException{
					return SingInResponse.parseFrom(byteString);
				}
             });
		mp.put("RPC_Auth.LogOut",  new RpcHelper2(){
				@Override
				public com.google.protobuf.GeneratedMessageLite parseData(ByteString byteString) throws com.google.protobuf.InvalidProtocolBufferException{
					return LogOutResponse.parseFrom(byteString);
				}
             });

	   	

		//mp.put("Rpc_Msg.GetFull", PB_SyncParam_GetGeneralUpdates.class.getName());
	}
}
