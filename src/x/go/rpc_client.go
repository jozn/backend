package x

import "github.com/golang/protobuf/proto"

//type RPCClientHandler func(cmd string, pb interface{}) interface{}
type RPCClientHandler func(cmdSre string, pbIn,pbOut proto.Message) ( error)
// all clients struc
var RPC_AllClinetsPlay = struct {
}{
}

// client types defs


/////////////// methods ////////////////

