package ir.ms.pb;

import java.util.HashMap;
import java.util.concurrent.ConcurrentHashMap;
import java.util.Map;

public class RPC_HANDLERS {

public interface RPC_Auth {
    void SendConfirmCode( SendConfirmCodeResponse pb, boolean handled);
    void ConfirmCode( ConfirmCodeResponse pb, boolean handled);
    void SingUp( SingUpResponse pb, boolean handled);
    void SingIn( SingInResponse pb, boolean handled);
    void LogOut( LogOutResponse pb, boolean handled);
}


  public static class RPC_Auth_Empty implements RPC_Auth{
  
  	@Override
    public void SendConfirmCode( SendConfirmCodeResponse pb, boolean handled){
    	Log.d("RPC", " default empty handler for RPC 'RPC_Auth.SendConfirmCode' ");
    }
  	@Override
    public void ConfirmCode( ConfirmCodeResponse pb, boolean handled){
    	Log.d("RPC", " default empty handler for RPC 'RPC_Auth.ConfirmCode' ");
    }
  	@Override
    public void SingUp( SingUpResponse pb, boolean handled){
    	Log.d("RPC", " default empty handler for RPC 'RPC_Auth.SingUp' ");
    }
  	@Override
    public void SingIn( SingInResponse pb, boolean handled){
    	Log.d("RPC", " default empty handler for RPC 'RPC_Auth.SingIn' ");
    }
  	@Override
    public void LogOut( LogOutResponse pb, boolean handled){
    	Log.d("RPC", " default empty handler for RPC 'RPC_Auth.LogOut' ");
    }
  }

	/////////////////////////////////// Maper of Handling methods /////////////////////////////////
	public static interface HandleRowRpcResponse {
		void handle(Object pb,boolean handled);
	}


	
	public static RPC_HANDLERS.RPC_Auth RPC_Auth_Default_Handler = new RPC_HANDLERS.RPC_Auth_Empty();


	public static Map<String,HandleRowRpcResponse > router = new HashMap<>();

	public static Map<String,HandleRowRpcResponse > getRouter(){
		if(router.size() ==0 ){
			initMap();
		}
		return router;
	}

	private synchronized static void initMap(){
	
	  
			router.put("RPC_Auth.SendConfirmCode", (pb, handled)->{
				if(pb instanceof SendConfirmCodeResponse){
					RPC_Auth_Default_Handler.SendConfirmCode((SendConfirmCodeResponse) pb, handled);
				}else{
					Log.d("RPC", " can not convert response object to SendConfirmCodeResponse in rpc: .SendConfirmCode -- class: " + pb );//.getClass().getName());
				}
			});
	  
			router.put("RPC_Auth.ConfirmCode", (pb, handled)->{
				if(pb instanceof ConfirmCodeResponse){
					RPC_Auth_Default_Handler.ConfirmCode((ConfirmCodeResponse) pb, handled);
				}else{
					Log.d("RPC", " can not convert response object to ConfirmCodeResponse in rpc: .ConfirmCode -- class: " + pb );//.getClass().getName());
				}
			});
	  
			router.put("RPC_Auth.SingUp", (pb, handled)->{
				if(pb instanceof SingUpResponse){
					RPC_Auth_Default_Handler.SingUp((SingUpResponse) pb, handled);
				}else{
					Log.d("RPC", " can not convert response object to SingUpResponse in rpc: .SingUp -- class: " + pb );//.getClass().getName());
				}
			});
	  
			router.put("RPC_Auth.SingIn", (pb, handled)->{
				if(pb instanceof SingInResponse){
					RPC_Auth_Default_Handler.SingIn((SingInResponse) pb, handled);
				}else{
					Log.d("RPC", " can not convert response object to SingInResponse in rpc: .SingIn -- class: " + pb );//.getClass().getName());
				}
			});
	  
			router.put("RPC_Auth.LogOut", (pb, handled)->{
				if(pb instanceof LogOutResponse){
					RPC_Auth_Default_Handler.LogOut((LogOutResponse) pb, handled);
				}else{
					Log.d("RPC", " can not convert response object to LogOutResponse in rpc: .LogOut -- class: " + pb );//.getClass().getName());
				}
			});
	  
	}
	
}
/*

RPC_HANDLERS.RPC_Auth RPC_Auth_Default_Handler = new RPC_HANDLERS.RPC_Auth RPC_Auth_Empty();
	
*/