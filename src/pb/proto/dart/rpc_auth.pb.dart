///
//  Generated code. Do not modify.
//  source: rpc_auth.proto
//
// @dart = 2.3
// ignore_for_file: camel_case_types,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type

import 'dart:async' as $async;
import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

import 'views.pb.dart' as $2;

export 'rpc_auth.pbenum.dart';

class SendConfirmCodeParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('SendConfirmCodeParam', createEmptyInstance: create)
    ..aOS(1, 'Hash', protoName: 'Hash')
    ..aOS(2, 'Phone', protoName: 'Phone')
    ..aOS(3, 'CountryCode', protoName: 'CountryCode')
    ..aOB(4, 'Resend', protoName: 'Resend')
    ..hasRequiredFields = false
  ;

  SendConfirmCodeParam._() : super();
  factory SendConfirmCodeParam() => create();
  factory SendConfirmCodeParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory SendConfirmCodeParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  SendConfirmCodeParam clone() => SendConfirmCodeParam()..mergeFromMessage(this);
  SendConfirmCodeParam copyWith(void Function(SendConfirmCodeParam) updates) => super.copyWith((message) => updates(message as SendConfirmCodeParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static SendConfirmCodeParam create() => SendConfirmCodeParam._();
  SendConfirmCodeParam createEmptyInstance() => create();
  static $pb.PbList<SendConfirmCodeParam> createRepeated() => $pb.PbList<SendConfirmCodeParam>();
  @$core.pragma('dart2js:noInline')
  static SendConfirmCodeParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<SendConfirmCodeParam>(create);
  static SendConfirmCodeParam _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get hash => $_getSZ(0);
  @$pb.TagNumber(1)
  set hash($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasHash() => $_has(0);
  @$pb.TagNumber(1)
  void clearHash() => clearField(1);

  @$pb.TagNumber(2)
  $core.String get phone => $_getSZ(1);
  @$pb.TagNumber(2)
  set phone($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasPhone() => $_has(1);
  @$pb.TagNumber(2)
  void clearPhone() => clearField(2);

  @$pb.TagNumber(3)
  $core.String get countryCode => $_getSZ(2);
  @$pb.TagNumber(3)
  set countryCode($core.String v) { $_setString(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasCountryCode() => $_has(2);
  @$pb.TagNumber(3)
  void clearCountryCode() => clearField(3);

  @$pb.TagNumber(4)
  $core.bool get resend => $_getBF(3);
  @$pb.TagNumber(4)
  set resend($core.bool v) { $_setBool(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasResend() => $_has(3);
  @$pb.TagNumber(4)
  void clearResend() => clearField(4);
}

class SendConfirmCodeResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('SendConfirmCodeResponse', createEmptyInstance: create)
    ..aOB(1, 'Done', protoName: 'Done')
    ..aOS(2, 'ErrorMessage', protoName: 'ErrorMessage')
    ..aOB(3, 'JustEmailRegister', protoName: 'JustEmailRegister')
    ..pPS(4, 'SmsNumbers', protoName: 'SmsNumbers')
    ..aOB(5, 'IsLogin', protoName: 'IsLogin')
    ..hasRequiredFields = false
  ;

  SendConfirmCodeResponse._() : super();
  factory SendConfirmCodeResponse() => create();
  factory SendConfirmCodeResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory SendConfirmCodeResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  SendConfirmCodeResponse clone() => SendConfirmCodeResponse()..mergeFromMessage(this);
  SendConfirmCodeResponse copyWith(void Function(SendConfirmCodeResponse) updates) => super.copyWith((message) => updates(message as SendConfirmCodeResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static SendConfirmCodeResponse create() => SendConfirmCodeResponse._();
  SendConfirmCodeResponse createEmptyInstance() => create();
  static $pb.PbList<SendConfirmCodeResponse> createRepeated() => $pb.PbList<SendConfirmCodeResponse>();
  @$core.pragma('dart2js:noInline')
  static SendConfirmCodeResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<SendConfirmCodeResponse>(create);
  static SendConfirmCodeResponse _defaultInstance;

  @$pb.TagNumber(1)
  $core.bool get done => $_getBF(0);
  @$pb.TagNumber(1)
  set done($core.bool v) { $_setBool(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasDone() => $_has(0);
  @$pb.TagNumber(1)
  void clearDone() => clearField(1);

  @$pb.TagNumber(2)
  $core.String get errorMessage => $_getSZ(1);
  @$pb.TagNumber(2)
  set errorMessage($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasErrorMessage() => $_has(1);
  @$pb.TagNumber(2)
  void clearErrorMessage() => clearField(2);

  @$pb.TagNumber(3)
  $core.bool get justEmailRegister => $_getBF(2);
  @$pb.TagNumber(3)
  set justEmailRegister($core.bool v) { $_setBool(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasJustEmailRegister() => $_has(2);
  @$pb.TagNumber(3)
  void clearJustEmailRegister() => clearField(3);

  @$pb.TagNumber(4)
  $core.List<$core.String> get smsNumbers => $_getList(3);

  @$pb.TagNumber(5)
  $core.bool get isLogin => $_getBF(4);
  @$pb.TagNumber(5)
  set isLogin($core.bool v) { $_setBool(4, v); }
  @$pb.TagNumber(5)
  $core.bool hasIsLogin() => $_has(4);
  @$pb.TagNumber(5)
  void clearIsLogin() => clearField(5);
}

class ConfirmCodeParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ConfirmCodeParam', createEmptyInstance: create)
    ..aOS(1, 'Hash', protoName: 'Hash')
    ..aOS(2, 'Phone', protoName: 'Phone')
    ..a<$core.int>(3, 'Code', $pb.PbFieldType.O3, protoName: 'Code')
    ..hasRequiredFields = false
  ;

  ConfirmCodeParam._() : super();
  factory ConfirmCodeParam() => create();
  factory ConfirmCodeParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ConfirmCodeParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ConfirmCodeParam clone() => ConfirmCodeParam()..mergeFromMessage(this);
  ConfirmCodeParam copyWith(void Function(ConfirmCodeParam) updates) => super.copyWith((message) => updates(message as ConfirmCodeParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ConfirmCodeParam create() => ConfirmCodeParam._();
  ConfirmCodeParam createEmptyInstance() => create();
  static $pb.PbList<ConfirmCodeParam> createRepeated() => $pb.PbList<ConfirmCodeParam>();
  @$core.pragma('dart2js:noInline')
  static ConfirmCodeParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ConfirmCodeParam>(create);
  static ConfirmCodeParam _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get hash => $_getSZ(0);
  @$pb.TagNumber(1)
  set hash($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasHash() => $_has(0);
  @$pb.TagNumber(1)
  void clearHash() => clearField(1);

  @$pb.TagNumber(2)
  $core.String get phone => $_getSZ(1);
  @$pb.TagNumber(2)
  set phone($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasPhone() => $_has(1);
  @$pb.TagNumber(2)
  void clearPhone() => clearField(2);

  @$pb.TagNumber(3)
  $core.int get code => $_getIZ(2);
  @$pb.TagNumber(3)
  set code($core.int v) { $_setSignedInt32(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasCode() => $_has(2);
  @$pb.TagNumber(3)
  void clearCode() => clearField(3);
}

class ConfirmCodeResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('ConfirmCodeResponse', createEmptyInstance: create)
    ..aOB(1, 'Done', protoName: 'Done')
    ..aOS(2, 'ErrorMessage', protoName: 'ErrorMessage')
    ..aOM<$2.SelfUserView>(3, 'SelfUserView', protoName: 'SelfUserView', subBuilder: $2.SelfUserView.create)
    ..hasRequiredFields = false
  ;

  ConfirmCodeResponse._() : super();
  factory ConfirmCodeResponse() => create();
  factory ConfirmCodeResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ConfirmCodeResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  ConfirmCodeResponse clone() => ConfirmCodeResponse()..mergeFromMessage(this);
  ConfirmCodeResponse copyWith(void Function(ConfirmCodeResponse) updates) => super.copyWith((message) => updates(message as ConfirmCodeResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ConfirmCodeResponse create() => ConfirmCodeResponse._();
  ConfirmCodeResponse createEmptyInstance() => create();
  static $pb.PbList<ConfirmCodeResponse> createRepeated() => $pb.PbList<ConfirmCodeResponse>();
  @$core.pragma('dart2js:noInline')
  static ConfirmCodeResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ConfirmCodeResponse>(create);
  static ConfirmCodeResponse _defaultInstance;

  @$pb.TagNumber(1)
  $core.bool get done => $_getBF(0);
  @$pb.TagNumber(1)
  set done($core.bool v) { $_setBool(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasDone() => $_has(0);
  @$pb.TagNumber(1)
  void clearDone() => clearField(1);

  @$pb.TagNumber(2)
  $core.String get errorMessage => $_getSZ(1);
  @$pb.TagNumber(2)
  set errorMessage($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasErrorMessage() => $_has(1);
  @$pb.TagNumber(2)
  void clearErrorMessage() => clearField(2);

  @$pb.TagNumber(3)
  $2.SelfUserView get selfUserView => $_getN(2);
  @$pb.TagNumber(3)
  set selfUserView($2.SelfUserView v) { setField(3, v); }
  @$pb.TagNumber(3)
  $core.bool hasSelfUserView() => $_has(2);
  @$pb.TagNumber(3)
  void clearSelfUserView() => clearField(3);
  @$pb.TagNumber(3)
  $2.SelfUserView ensureSelfUserView() => $_ensure(2);
}

class SingUpParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('SingUpParam', createEmptyInstance: create)
    ..aOS(1, 'Hash', protoName: 'Hash')
    ..aOS(2, 'FirstName', protoName: 'FirstName')
    ..aOS(3, 'LastName', protoName: 'LastName')
    ..aOS(4, 'UserName', protoName: 'UserName')
    ..aOS(5, 'Phone', protoName: 'Phone')
    ..aOS(6, 'Email', protoName: 'Email')
    ..hasRequiredFields = false
  ;

  SingUpParam._() : super();
  factory SingUpParam() => create();
  factory SingUpParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory SingUpParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  SingUpParam clone() => SingUpParam()..mergeFromMessage(this);
  SingUpParam copyWith(void Function(SingUpParam) updates) => super.copyWith((message) => updates(message as SingUpParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static SingUpParam create() => SingUpParam._();
  SingUpParam createEmptyInstance() => create();
  static $pb.PbList<SingUpParam> createRepeated() => $pb.PbList<SingUpParam>();
  @$core.pragma('dart2js:noInline')
  static SingUpParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<SingUpParam>(create);
  static SingUpParam _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get hash => $_getSZ(0);
  @$pb.TagNumber(1)
  set hash($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasHash() => $_has(0);
  @$pb.TagNumber(1)
  void clearHash() => clearField(1);

  @$pb.TagNumber(2)
  $core.String get firstName => $_getSZ(1);
  @$pb.TagNumber(2)
  set firstName($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasFirstName() => $_has(1);
  @$pb.TagNumber(2)
  void clearFirstName() => clearField(2);

  @$pb.TagNumber(3)
  $core.String get lastName => $_getSZ(2);
  @$pb.TagNumber(3)
  set lastName($core.String v) { $_setString(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasLastName() => $_has(2);
  @$pb.TagNumber(3)
  void clearLastName() => clearField(3);

  @$pb.TagNumber(4)
  $core.String get userName => $_getSZ(3);
  @$pb.TagNumber(4)
  set userName($core.String v) { $_setString(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasUserName() => $_has(3);
  @$pb.TagNumber(4)
  void clearUserName() => clearField(4);

  @$pb.TagNumber(5)
  $core.String get phone => $_getSZ(4);
  @$pb.TagNumber(5)
  set phone($core.String v) { $_setString(4, v); }
  @$pb.TagNumber(5)
  $core.bool hasPhone() => $_has(4);
  @$pb.TagNumber(5)
  void clearPhone() => clearField(5);

  @$pb.TagNumber(6)
  $core.String get email => $_getSZ(5);
  @$pb.TagNumber(6)
  set email($core.String v) { $_setString(5, v); }
  @$pb.TagNumber(6)
  $core.bool hasEmail() => $_has(5);
  @$pb.TagNumber(6)
  void clearEmail() => clearField(6);
}

class SingUpResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('SingUpResponse', createEmptyInstance: create)
    ..aOB(1, 'Done', protoName: 'Done')
    ..aOS(2, 'ErrorMessage', protoName: 'ErrorMessage')
    ..aOM<$2.SelfUserView>(3, 'SelfUserView', protoName: 'SelfUserView', subBuilder: $2.SelfUserView.create)
    ..hasRequiredFields = false
  ;

  SingUpResponse._() : super();
  factory SingUpResponse() => create();
  factory SingUpResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory SingUpResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  SingUpResponse clone() => SingUpResponse()..mergeFromMessage(this);
  SingUpResponse copyWith(void Function(SingUpResponse) updates) => super.copyWith((message) => updates(message as SingUpResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static SingUpResponse create() => SingUpResponse._();
  SingUpResponse createEmptyInstance() => create();
  static $pb.PbList<SingUpResponse> createRepeated() => $pb.PbList<SingUpResponse>();
  @$core.pragma('dart2js:noInline')
  static SingUpResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<SingUpResponse>(create);
  static SingUpResponse _defaultInstance;

  @$pb.TagNumber(1)
  $core.bool get done => $_getBF(0);
  @$pb.TagNumber(1)
  set done($core.bool v) { $_setBool(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasDone() => $_has(0);
  @$pb.TagNumber(1)
  void clearDone() => clearField(1);

  @$pb.TagNumber(2)
  $core.String get errorMessage => $_getSZ(1);
  @$pb.TagNumber(2)
  set errorMessage($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasErrorMessage() => $_has(1);
  @$pb.TagNumber(2)
  void clearErrorMessage() => clearField(2);

  @$pb.TagNumber(3)
  $2.SelfUserView get selfUserView => $_getN(2);
  @$pb.TagNumber(3)
  set selfUserView($2.SelfUserView v) { setField(3, v); }
  @$pb.TagNumber(3)
  $core.bool hasSelfUserView() => $_has(2);
  @$pb.TagNumber(3)
  void clearSelfUserView() => clearField(3);
  @$pb.TagNumber(3)
  $2.SelfUserView ensureSelfUserView() => $_ensure(2);
}

class SingInParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('SingInParam', createEmptyInstance: create)
    ..aOS(4, 'UserNamePhoneEmail', protoName: 'UserNamePhoneEmail')
    ..aOS(5, 'Password', protoName: 'Password')
    ..hasRequiredFields = false
  ;

  SingInParam._() : super();
  factory SingInParam() => create();
  factory SingInParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory SingInParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  SingInParam clone() => SingInParam()..mergeFromMessage(this);
  SingInParam copyWith(void Function(SingInParam) updates) => super.copyWith((message) => updates(message as SingInParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static SingInParam create() => SingInParam._();
  SingInParam createEmptyInstance() => create();
  static $pb.PbList<SingInParam> createRepeated() => $pb.PbList<SingInParam>();
  @$core.pragma('dart2js:noInline')
  static SingInParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<SingInParam>(create);
  static SingInParam _defaultInstance;

  @$pb.TagNumber(4)
  $core.String get userNamePhoneEmail => $_getSZ(0);
  @$pb.TagNumber(4)
  set userNamePhoneEmail($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(4)
  $core.bool hasUserNamePhoneEmail() => $_has(0);
  @$pb.TagNumber(4)
  void clearUserNamePhoneEmail() => clearField(4);

  @$pb.TagNumber(5)
  $core.String get password => $_getSZ(1);
  @$pb.TagNumber(5)
  set password($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(5)
  $core.bool hasPassword() => $_has(1);
  @$pb.TagNumber(5)
  void clearPassword() => clearField(5);
}

class SingInResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('SingInResponse', createEmptyInstance: create)
    ..aOB(1, 'Done', protoName: 'Done')
    ..aOS(2, 'ErrorMessage', protoName: 'ErrorMessage')
    ..aOM<$2.SelfUserView>(3, 'SelfUserView', protoName: 'SelfUserView', subBuilder: $2.SelfUserView.create)
    ..hasRequiredFields = false
  ;

  SingInResponse._() : super();
  factory SingInResponse() => create();
  factory SingInResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory SingInResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  SingInResponse clone() => SingInResponse()..mergeFromMessage(this);
  SingInResponse copyWith(void Function(SingInResponse) updates) => super.copyWith((message) => updates(message as SingInResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static SingInResponse create() => SingInResponse._();
  SingInResponse createEmptyInstance() => create();
  static $pb.PbList<SingInResponse> createRepeated() => $pb.PbList<SingInResponse>();
  @$core.pragma('dart2js:noInline')
  static SingInResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<SingInResponse>(create);
  static SingInResponse _defaultInstance;

  @$pb.TagNumber(1)
  $core.bool get done => $_getBF(0);
  @$pb.TagNumber(1)
  set done($core.bool v) { $_setBool(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasDone() => $_has(0);
  @$pb.TagNumber(1)
  void clearDone() => clearField(1);

  @$pb.TagNumber(2)
  $core.String get errorMessage => $_getSZ(1);
  @$pb.TagNumber(2)
  set errorMessage($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasErrorMessage() => $_has(1);
  @$pb.TagNumber(2)
  void clearErrorMessage() => clearField(2);

  @$pb.TagNumber(3)
  $2.SelfUserView get selfUserView => $_getN(2);
  @$pb.TagNumber(3)
  set selfUserView($2.SelfUserView v) { setField(3, v); }
  @$pb.TagNumber(3)
  $core.bool hasSelfUserView() => $_has(2);
  @$pb.TagNumber(3)
  void clearSelfUserView() => clearField(3);
  @$pb.TagNumber(3)
  $2.SelfUserView ensureSelfUserView() => $_ensure(2);
}

class LogOutParam extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('LogOutParam', createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  LogOutParam._() : super();
  factory LogOutParam() => create();
  factory LogOutParam.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory LogOutParam.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  LogOutParam clone() => LogOutParam()..mergeFromMessage(this);
  LogOutParam copyWith(void Function(LogOutParam) updates) => super.copyWith((message) => updates(message as LogOutParam));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static LogOutParam create() => LogOutParam._();
  LogOutParam createEmptyInstance() => create();
  static $pb.PbList<LogOutParam> createRepeated() => $pb.PbList<LogOutParam>();
  @$core.pragma('dart2js:noInline')
  static LogOutParam getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<LogOutParam>(create);
  static LogOutParam _defaultInstance;
}

class LogOutResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo('LogOutResponse', createEmptyInstance: create)
    ..aOB(1, 'Done', protoName: 'Done')
    ..aOS(2, 'ErrorMessage', protoName: 'ErrorMessage')
    ..hasRequiredFields = false
  ;

  LogOutResponse._() : super();
  factory LogOutResponse() => create();
  factory LogOutResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory LogOutResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  LogOutResponse clone() => LogOutResponse()..mergeFromMessage(this);
  LogOutResponse copyWith(void Function(LogOutResponse) updates) => super.copyWith((message) => updates(message as LogOutResponse));
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static LogOutResponse create() => LogOutResponse._();
  LogOutResponse createEmptyInstance() => create();
  static $pb.PbList<LogOutResponse> createRepeated() => $pb.PbList<LogOutResponse>();
  @$core.pragma('dart2js:noInline')
  static LogOutResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<LogOutResponse>(create);
  static LogOutResponse _defaultInstance;

  @$pb.TagNumber(1)
  $core.bool get done => $_getBF(0);
  @$pb.TagNumber(1)
  set done($core.bool v) { $_setBool(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasDone() => $_has(0);
  @$pb.TagNumber(1)
  void clearDone() => clearField(1);

  @$pb.TagNumber(2)
  $core.String get errorMessage => $_getSZ(1);
  @$pb.TagNumber(2)
  set errorMessage($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasErrorMessage() => $_has(1);
  @$pb.TagNumber(2)
  void clearErrorMessage() => clearField(2);
}

class RPC_AuthApi {
  $pb.RpcClient _client;
  RPC_AuthApi(this._client);

  $async.Future<SendConfirmCodeResponse> sendConfirmCode($pb.ClientContext ctx, SendConfirmCodeParam request) {
    var emptyResponse = SendConfirmCodeResponse();
    return _client.invoke<SendConfirmCodeResponse>(ctx, 'RPC_Auth', 'SendConfirmCode', request, emptyResponse);
  }
  $async.Future<ConfirmCodeResponse> confirmCode($pb.ClientContext ctx, ConfirmCodeParam request) {
    var emptyResponse = ConfirmCodeResponse();
    return _client.invoke<ConfirmCodeResponse>(ctx, 'RPC_Auth', 'ConfirmCode', request, emptyResponse);
  }
  $async.Future<SingUpResponse> singUp($pb.ClientContext ctx, SingUpParam request) {
    var emptyResponse = SingUpResponse();
    return _client.invoke<SingUpResponse>(ctx, 'RPC_Auth', 'SingUp', request, emptyResponse);
  }
  $async.Future<SingInResponse> singIn($pb.ClientContext ctx, SingInParam request) {
    var emptyResponse = SingInResponse();
    return _client.invoke<SingInResponse>(ctx, 'RPC_Auth', 'SingIn', request, emptyResponse);
  }
  $async.Future<LogOutResponse> logOut($pb.ClientContext ctx, LogOutParam request) {
    var emptyResponse = LogOutResponse();
    return _client.invoke<LogOutResponse>(ctx, 'RPC_Auth', 'LogOut', request, emptyResponse);
  }
}

