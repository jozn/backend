package x

/////////////// Empty Sample RPC - mainly for mocking ////////////////

/////////////////// RPC_Auth  -  EmptyRPC_RPC_Auth /////////////////////
type EmptyRPC_RPC_Auth int

var Empty_RPC_RPC_Auth_Sample = EmptyRPC_RPC_Auth(0)

func (EmptyRPC_RPC_Auth) SendConfirmCode(i *SendConfirmCodeParam, p RPC_UserParam) (*SendConfirmCodeResponse, error) {
	return nil, nil
}
func (EmptyRPC_RPC_Auth) ConfirmCode(i *ConfirmCodeParam, p RPC_UserParam) (*ConfirmCodeResponse, error) {
	return nil, nil
}
func (EmptyRPC_RPC_Auth) SingUp(i *SingUpParam, p RPC_UserParam) (*SingUpResponse, error) {
	return nil, nil
}
func (EmptyRPC_RPC_Auth) SingIn(i *SingInParam, p RPC_UserParam) (*SingInResponse, error) {
	return nil, nil
}
func (EmptyRPC_RPC_Auth) LogOut(i *LogOutParam, p RPC_UserParam) (*LogOutResponse, error) {
	return nil, nil
}
