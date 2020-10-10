///
//  Generated code. Do not modify.
//  source: rpc_auth.proto
//
// @dart = 2.3
// ignore_for_file: camel_case_types,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type

import 'views.pbjson.dart' as $2;

const SendConfirmCodeTypeEnum$json = const {
  '1': 'SendConfirmCodeTypeEnum',
  '2': const [
    const {'1': 'SEND_CODE_OK', '2': 0},
    const {'1': 'SEND_CODE_EMAIL', '2': 1},
  ],
};

const SendConfirmCodeParam$json = const {
  '1': 'SendConfirmCodeParam',
  '2': const [
    const {'1': 'Hash', '3': 1, '4': 1, '5': 9, '10': 'Hash'},
    const {'1': 'Phone', '3': 2, '4': 1, '5': 9, '10': 'Phone'},
    const {'1': 'CountryCode', '3': 3, '4': 1, '5': 9, '10': 'CountryCode'},
    const {'1': 'Resend', '3': 4, '4': 1, '5': 8, '10': 'Resend'},
  ],
};

const SendConfirmCodeResponse$json = const {
  '1': 'SendConfirmCodeResponse',
  '2': const [
    const {'1': 'Done', '3': 1, '4': 1, '5': 8, '10': 'Done'},
    const {'1': 'ErrorMessage', '3': 2, '4': 1, '5': 9, '10': 'ErrorMessage'},
    const {'1': 'JustEmailRegister', '3': 3, '4': 1, '5': 8, '10': 'JustEmailRegister'},
    const {'1': 'SmsNumbers', '3': 4, '4': 3, '5': 9, '10': 'SmsNumbers'},
    const {'1': 'IsLogin', '3': 5, '4': 1, '5': 8, '10': 'IsLogin'},
  ],
};

const ConfirmCodeParam$json = const {
  '1': 'ConfirmCodeParam',
  '2': const [
    const {'1': 'Hash', '3': 1, '4': 1, '5': 9, '10': 'Hash'},
    const {'1': 'Phone', '3': 2, '4': 1, '5': 9, '10': 'Phone'},
    const {'1': 'Code', '3': 3, '4': 1, '5': 5, '10': 'Code'},
  ],
};

const ConfirmCodeResponse$json = const {
  '1': 'ConfirmCodeResponse',
  '2': const [
    const {'1': 'Done', '3': 1, '4': 1, '5': 8, '10': 'Done'},
    const {'1': 'ErrorMessage', '3': 2, '4': 1, '5': 9, '10': 'ErrorMessage'},
    const {'1': 'SelfUserView', '3': 3, '4': 1, '5': 11, '6': '.SelfUserView', '10': 'SelfUserView'},
  ],
};

const SingUpParam$json = const {
  '1': 'SingUpParam',
  '2': const [
    const {'1': 'Hash', '3': 1, '4': 1, '5': 9, '10': 'Hash'},
    const {'1': 'FirstName', '3': 2, '4': 1, '5': 9, '10': 'FirstName'},
    const {'1': 'LastName', '3': 3, '4': 1, '5': 9, '10': 'LastName'},
    const {'1': 'UserName', '3': 4, '4': 1, '5': 9, '10': 'UserName'},
    const {'1': 'Phone', '3': 5, '4': 1, '5': 9, '10': 'Phone'},
    const {'1': 'Email', '3': 6, '4': 1, '5': 9, '10': 'Email'},
  ],
};

const SingUpResponse$json = const {
  '1': 'SingUpResponse',
  '2': const [
    const {'1': 'Done', '3': 1, '4': 1, '5': 8, '10': 'Done'},
    const {'1': 'ErrorMessage', '3': 2, '4': 1, '5': 9, '10': 'ErrorMessage'},
    const {'1': 'SelfUserView', '3': 3, '4': 1, '5': 11, '6': '.SelfUserView', '10': 'SelfUserView'},
  ],
};

const SingInParam$json = const {
  '1': 'SingInParam',
  '2': const [
    const {'1': 'UserNamePhoneEmail', '3': 4, '4': 1, '5': 9, '10': 'UserNamePhoneEmail'},
    const {'1': 'Password', '3': 5, '4': 1, '5': 9, '10': 'Password'},
  ],
};

const SingInResponse$json = const {
  '1': 'SingInResponse',
  '2': const [
    const {'1': 'Done', '3': 1, '4': 1, '5': 8, '10': 'Done'},
    const {'1': 'ErrorMessage', '3': 2, '4': 1, '5': 9, '10': 'ErrorMessage'},
    const {'1': 'SelfUserView', '3': 3, '4': 1, '5': 11, '6': '.SelfUserView', '10': 'SelfUserView'},
  ],
};

const LogOutParam$json = const {
  '1': 'LogOutParam',
};

const LogOutResponse$json = const {
  '1': 'LogOutResponse',
  '2': const [
    const {'1': 'Done', '3': 1, '4': 1, '5': 8, '10': 'Done'},
    const {'1': 'ErrorMessage', '3': 2, '4': 1, '5': 9, '10': 'ErrorMessage'},
  ],
};

const RPC_AuthServiceBase$json = const {
  '1': 'RPC_Auth',
  '2': const [
    const {'1': 'SendConfirmCode', '2': '.SendConfirmCodeParam', '3': '.SendConfirmCodeResponse'},
    const {'1': 'ConfirmCode', '2': '.ConfirmCodeParam', '3': '.ConfirmCodeResponse'},
    const {'1': 'SingUp', '2': '.SingUpParam', '3': '.SingUpResponse'},
    const {'1': 'SingIn', '2': '.SingInParam', '3': '.SingInResponse'},
    const {'1': 'LogOut', '2': '.LogOutParam', '3': '.LogOutResponse'},
  ],
};

const RPC_AuthServiceBase$messageJson = const {
  '.SendConfirmCodeParam': SendConfirmCodeParam$json,
  '.SendConfirmCodeResponse': SendConfirmCodeResponse$json,
  '.ConfirmCodeParam': ConfirmCodeParam$json,
  '.ConfirmCodeResponse': ConfirmCodeResponse$json,
  '.SelfUserView': $2.SelfUserView$json,
  '.UserView': $2.UserView$json,
  '.SettingNotificationView': $2.SettingNotificationView$json,
  '.SingUpParam': SingUpParam$json,
  '.SingUpResponse': SingUpResponse$json,
  '.SingInParam': SingInParam$json,
  '.SingInResponse': SingInResponse$json,
  '.LogOutParam': LogOutParam$json,
  '.LogOutResponse': LogOutResponse$json,
};

