///
//  Generated code. Do not modify.
//  source: rpc_auth.proto
//
// @dart = 2.3
// ignore_for_file: camel_case_types,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type

// ignore_for_file: UNDEFINED_SHOWN_NAME,UNUSED_SHOWN_NAME
import 'dart:core' as $core;
import 'package:protobuf/protobuf.dart' as $pb;

class SendConfirmCodeTypeEnum extends $pb.ProtobufEnum {
  static const SendConfirmCodeTypeEnum SEND_CODE_OK = SendConfirmCodeTypeEnum._(0, 'SEND_CODE_OK');
  static const SendConfirmCodeTypeEnum SEND_CODE_EMAIL = SendConfirmCodeTypeEnum._(1, 'SEND_CODE_EMAIL');

  static const $core.List<SendConfirmCodeTypeEnum> values = <SendConfirmCodeTypeEnum> [
    SEND_CODE_OK,
    SEND_CODE_EMAIL,
  ];

  static final $core.Map<$core.int, SendConfirmCodeTypeEnum> _byValue = $pb.ProtobufEnum.initByValue(values);
  static SendConfirmCodeTypeEnum valueOf($core.int value) => _byValue[value];

  const SendConfirmCodeTypeEnum._($core.int v, $core.String n) : super(v, n);
}

