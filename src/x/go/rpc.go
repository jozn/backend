package x

import (
    "strings"
    "github.com/golang/protobuf/proto"
    "errors"
    "ms/sun/shared/config"
    "log"
)

type RPC_UserParam interface {
    GetUserId() int
    IsUser() bool
    FailRequest(error PB_Error)
}

type RPC_ResponseHandlerInterface interface {
    //HandleOfflineResult(dataPB interface{},PBClass string,RpcName string,cmd PB_CommandToServer,p RPC_UserParam, paramParsed interface{})
    HandleOfflineResult(resOut RpcResponseOutput)
    IsUserOnlineResult(interface{}, error)
    HandelError(error)
}

type RpcResponseOutput struct {
    UserParam       RPC_UserParam
    ResponseData    interface{}
    PBClassName     string
    RpcName         string
    CommandToServer PB_CommandToServer
    RpcParamPassed  interface{}
}

var RPC_ResponseHandler RPC_ResponseHandlerInterface

//note: rpc methods cant have equal name they must be different even in different rpc services
type RPC_AllHandlersInteract struct {

}

/////////////// Interfaces ////////////////


func noDevErr(err error)  {
    if config.IS_DEBUG && err != nil {
        log.Panic(err)
    }
}

type rpcParamHandler struct {
    cmd PB_CommandToServer
    params RPC_UserParam
    rpcHandler RPC_AllHandlersInteract
    responseHandler RPC_ResponseHandlerInterface
}

//todo: this method can be outputed from x package
////////////// map of rpc methods to all
func HandleRpcs(cmd PB_CommandToServer, params RPC_UserParam, rpcHandler RPC_AllHandlersInteract,responseHandler RPC_ResponseHandlerInterface) {

    fn,ok := mpRpcMethods[cmd.Command]
    if !ok {
        if config.IS_DEBUG {
            log.Panic("HandleRpcs:  command not registerd for ", cmd.Command)
        }
    }
    
    p := rpcParamHandler{
		cmd:             cmd,
		params:          params,
		rpcHandler:      rpcHandler,
		responseHandler: responseHandler,
	}
	fn(p)
}

var mpRpcMethods = map[string]func(p rpcParamHandler){
    

}


/////////////// Direct in PB_CommandToClient /////////////
/*

*/