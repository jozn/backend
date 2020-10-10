///
//  Generated code. Do not modify.
//  source: rpc_upload.proto
//
// @dart = 2.3
// ignore_for_file: camel_case_types,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type

const UploadFileParam$json = const {
  '1': 'UploadFileParam',
};

const UploadFileResponse$json = const {
  '1': 'UploadFileResponse',
  '2': const [
    const {'1': 'done', '3': 1, '4': 1, '5': 8, '10': 'done'},
    const {'1': 'text', '3': 2, '4': 1, '5': 9, '10': 'text'},
  ],
};

const RPC_UploadServiceBase$json = const {
  '1': 'RPC_Upload',
  '2': const [
    const {'1': 'UploadFile', '2': '.UploadFileParam', '3': '.UploadFileResponse'},
  ],
};

const RPC_UploadServiceBase$messageJson = const {
  '.UploadFileParam': UploadFileParam$json,
  '.UploadFileResponse': UploadFileResponse$json,
};

