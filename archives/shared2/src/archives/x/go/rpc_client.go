package x

import "github.com/golang/protobuf/proto"

//type RPCClientHandler func(cmd string, pb interface{}) interface{}
type RPCClientHandler func(cmdSre string, pbIn, pbOut proto.Message) error

// all clients struc
var RPC_AllClinetsPlay = struct {
	RPC_Auth RPC_Auth_Client
}{
	RPC_Auth: RPC_Auth_Client(0),
}

// client types defs
type RPC_Auth_Client int

/////////////// methods ////////////////

// service: RPC_Auth

func (RPC_Auth_Client) SendConfirmCode(param *SendConfirmCodeParam, fn RPCClientHandler) (*SendConfirmCodeResponse, error) {
	out := &SendConfirmCodeResponse{}
	err := fn("RPC_Auth.SendConfirmCode", param, out)
	if err == nil {
		return out, nil
	}
	return nil, err
}

func (RPC_Auth_Client) ConfirmCode(param *ConfirmCodeParam, fn RPCClientHandler) (*ConfirmCodeResponse, error) {
	out := &ConfirmCodeResponse{}
	err := fn("RPC_Auth.ConfirmCode", param, out)
	if err == nil {
		return out, nil
	}
	return nil, err
}

func (RPC_Auth_Client) SingUp(param *SingUpParam, fn RPCClientHandler) (*SingUpResponse, error) {
	out := &SingUpResponse{}
	err := fn("RPC_Auth.SingUp", param, out)
	if err == nil {
		return out, nil
	}
	return nil, err
}

func (RPC_Auth_Client) SingIn(param *SingInParam, fn RPCClientHandler) (*SingInResponse, error) {
	out := &SingInResponse{}
	err := fn("RPC_Auth.SingIn", param, out)
	if err == nil {
		return out, nil
	}
	return nil, err
}

func (RPC_Auth_Client) LogOut(param *LogOutParam, fn RPCClientHandler) (*LogOutResponse, error) {
	out := &LogOutResponse{}
	err := fn("RPC_Auth.LogOut", param, out)
	if err == nil {
		return out, nil
	}
	return nil, err
}
