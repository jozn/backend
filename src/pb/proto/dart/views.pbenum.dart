///
//  Generated code. Do not modify.
//  source: views.proto
//
// @dart = 2.3
// ignore_for_file: camel_case_types,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type

// ignore_for_file: UNDEFINED_SHOWN_NAME,UNUSED_SHOWN_NAME
import 'dart:core' as $core;
import 'package:protobuf/protobuf.dart' as $pb;

class ServerErrors extends $pb.ProtobufEnum {
  static const ServerErrors UNKNOWN_ERR = ServerErrors._(0, 'UNKNOWN_ERR');
  static const ServerErrors ERR_CLIENT_IS_DEPRECATED = ServerErrors._(1, 'ERR_CLIENT_IS_DEPRECATED');
  static const ServerErrors ERR_SERVER_UPGRADING = ServerErrors._(2, 'ERR_SERVER_UPGRADING');

  static const $core.List<ServerErrors> values = <ServerErrors> [
    UNKNOWN_ERR,
    ERR_CLIENT_IS_DEPRECATED,
    ERR_SERVER_UPGRADING,
  ];

  static final $core.Map<$core.int, ServerErrors> _byValue = $pb.ProtobufEnum.initByValue(values);
  static ServerErrors valueOf($core.int value) => _byValue[value];

  const ServerErrors._($core.int v, $core.String n) : super(v, n);
}

