package ir.ms.pb;

import java.util.HashMap;
import java.util.concurrent.ConcurrentHashMap;
import java.util.Map;

public class RPC_HANDLERS {




	/////////////////////////////////// Maper of Handling methods /////////////////////////////////
	public static interface HandleRowRpcResponse {
		void handle(Object pb,boolean handled);
	}


	


	public static Map<String,HandleRowRpcResponse > router = new HashMap<>();

	public static Map<String,HandleRowRpcResponse > getRouter(){
		if(router.size() ==0 ){
			initMap();
		}
		return router;
	}

	private synchronized static void initMap(){
	
	}
	
}
/*

	
*/