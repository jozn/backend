///
//  Generated code. Do not modify.
//  source: global.proto
//
// @dart = 2.3
// ignore_for_file: camel_case_types,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

class Param extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('Param', createEmptyInstance: create)
    ..aOS(1, 'Text', protoName: 'Text')
    ..hasRequiredFields = false
  ;

  Param._() : super();
  factory Param() => create();
  factory Param.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Param.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  Param clone() => Param()..mergeFromMessage(this);
  Param copyWith(void Function(Param) updates) => super.copyWith((message) => updates(message as Param));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static Param create() => Param._();
  Param createEmptyInstance() => create();
  static $pb.PbList<Param> createRepeated() => $pb.PbList<Param>();
  @$core.pragma('dart2js:noInline')
  static Param getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Param>(create);
  static Param _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get text => $_getSZ(0);
  @$pb.TagNumber(1)
  set text($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasText() => $_has(0);
  @$pb.TagNumber(1)
  void clearText() => clearField(1);
}

