package x

import (
	"errors"
	"github.com/golang/protobuf/proto"
	"log"
	"ms/sun/shared/config"
	"strings"
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
	RPC_Auth RPC_Auth
}

/////////////// Interfaces ////////////////

type RPC_Auth interface {
	SendConfirmCode(param *SendConfirmCodeParam, userParam RPC_UserParam) (res SendConfirmCodeResponse, errRes error)
	ConfirmCode(param *ConfirmCodeParam, userParam RPC_UserParam) (res ConfirmCodeResponse, errRes error)
	SingUp(param *SingUpParam, userParam RPC_UserParam) (res SingUpResponse, errRes error)
	SingIn(param *SingInParam, userParam RPC_UserParam) (res SingInResponse, errRes error)
	LogOut(param *LogOutParam, userParam RPC_UserParam) (res LogOutResponse, errRes error)
}

func noDevErr(err error) {
	if config.IS_DEBUG && err != nil {
		log.Panic(err)
	}
}

type rpcParamHandler struct {
	cmd             PB_CommandToServer
	params          RPC_UserParam
	rpcHandler      RPC_AllHandlersInteract
	responseHandler RPC_ResponseHandlerInterface
}

//todo: this method can be outputed from x package
////////////// map of rpc methods to all
func HandleRpcs(cmd PB_CommandToServer, params RPC_UserParam, rpcHandler RPC_AllHandlersInteract, responseHandler RPC_ResponseHandlerInterface) {

	fn, ok := mpRpcMethods[cmd.Command]
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

	// rpc: RPC_Auth

	"RPC_Auth.SendConfirmCode": func(p rpcParamHandler) {
		if p.rpcHandler.RPC_Auth == nil {
			noDevErr(errors.New("rpc service is null for: p.rpcHandler.RPC_Auth.SendConfirmCode"))
			return
		}
		load := &SendConfirmCodeParam{}
		err := proto.Unmarshal(p.cmd.Data, load)
		if err == nil {
			res, err := p.rpcHandler.RPC_Auth.SendConfirmCode(load, p.params)
			if err == nil {
				out := RpcResponseOutput{
					RpcName:         "RPC_Auth.SendConfirmCode",
					UserParam:       p.params,
					CommandToServer: p.cmd,
					PBClassName:     "SendConfirmCodeResponse",
					ResponseData:    &res,
					RpcParamPassed:  load,
				}
				p.responseHandler.HandleOfflineResult(out)
			} else {
				p.responseHandler.HandelError(err)
			}
		} else {
			p.responseHandler.HandelError(err)
		}
	},
	"RPC_Auth.ConfirmCode": func(p rpcParamHandler) {
		if p.rpcHandler.RPC_Auth == nil {
			noDevErr(errors.New("rpc service is null for: p.rpcHandler.RPC_Auth.ConfirmCode"))
			return
		}
		load := &ConfirmCodeParam{}
		err := proto.Unmarshal(p.cmd.Data, load)
		if err == nil {
			res, err := p.rpcHandler.RPC_Auth.ConfirmCode(load, p.params)
			if err == nil {
				out := RpcResponseOutput{
					RpcName:         "RPC_Auth.ConfirmCode",
					UserParam:       p.params,
					CommandToServer: p.cmd,
					PBClassName:     "ConfirmCodeResponse",
					ResponseData:    &res,
					RpcParamPassed:  load,
				}
				p.responseHandler.HandleOfflineResult(out)
			} else {
				p.responseHandler.HandelError(err)
			}
		} else {
			p.responseHandler.HandelError(err)
		}
	},
	"RPC_Auth.SingUp": func(p rpcParamHandler) {
		if p.rpcHandler.RPC_Auth == nil {
			noDevErr(errors.New("rpc service is null for: p.rpcHandler.RPC_Auth.SingUp"))
			return
		}
		load := &SingUpParam{}
		err := proto.Unmarshal(p.cmd.Data, load)
		if err == nil {
			res, err := p.rpcHandler.RPC_Auth.SingUp(load, p.params)
			if err == nil {
				out := RpcResponseOutput{
					RpcName:         "RPC_Auth.SingUp",
					UserParam:       p.params,
					CommandToServer: p.cmd,
					PBClassName:     "SingUpResponse",
					ResponseData:    &res,
					RpcParamPassed:  load,
				}
				p.responseHandler.HandleOfflineResult(out)
			} else {
				p.responseHandler.HandelError(err)
			}
		} else {
			p.responseHandler.HandelError(err)
		}
	},
	"RPC_Auth.SingIn": func(p rpcParamHandler) {
		if p.rpcHandler.RPC_Auth == nil {
			noDevErr(errors.New("rpc service is null for: p.rpcHandler.RPC_Auth.SingIn"))
			return
		}
		load := &SingInParam{}
		err := proto.Unmarshal(p.cmd.Data, load)
		if err == nil {
			res, err := p.rpcHandler.RPC_Auth.SingIn(load, p.params)
			if err == nil {
				out := RpcResponseOutput{
					RpcName:         "RPC_Auth.SingIn",
					UserParam:       p.params,
					CommandToServer: p.cmd,
					PBClassName:     "SingInResponse",
					ResponseData:    &res,
					RpcParamPassed:  load,
				}
				p.responseHandler.HandleOfflineResult(out)
			} else {
				p.responseHandler.HandelError(err)
			}
		} else {
			p.responseHandler.HandelError(err)
		}
	},
	"RPC_Auth.LogOut": func(p rpcParamHandler) {
		if p.rpcHandler.RPC_Auth == nil {
			noDevErr(errors.New("rpc service is null for: p.rpcHandler.RPC_Auth.LogOut"))
			return
		}
		load := &LogOutParam{}
		err := proto.Unmarshal(p.cmd.Data, load)
		if err == nil {
			res, err := p.rpcHandler.RPC_Auth.LogOut(load, p.params)
			if err == nil {
				out := RpcResponseOutput{
					RpcName:         "RPC_Auth.LogOut",
					UserParam:       p.params,
					CommandToServer: p.cmd,
					PBClassName:     "LogOutResponse",
					ResponseData:    &res,
					RpcParamPassed:  load,
				}
				p.responseHandler.HandleOfflineResult(out)
			} else {
				p.responseHandler.HandelError(err)
			}
		} else {
			p.responseHandler.HandelError(err)
		}
	},
}

/////////////// Direct in PB_CommandToClient /////////////
/*


 RPC_Auth.SendConfirmCode
 RPC_Auth.ConfirmCode
 RPC_Auth.SingUp
 RPC_Auth.SingIn
 RPC_Auth.LogOut


*/
