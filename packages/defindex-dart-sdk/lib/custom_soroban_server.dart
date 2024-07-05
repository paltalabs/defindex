import 'dart:convert';
import 'dart:developer';
import 'package:stellar_flutter_sdk/stellar_flutter_sdk.dart';
import 'package:dio/dio.dart' as dio;

class CustomSorobanServer extends SorobanServer {
  final String _serverUrl;
  final dio.Dio _dio = dio.Dio();
  late Map<String, String> _headers;

  CustomSorobanServer(this._serverUrl) : super(_serverUrl) {
    _headers = {...RequestBuilder.headers};
    _headers.putIfAbsent("Content-Type", () => "application/json");
  }

  Future<SendTransactionResponse> sendRawTransaction(
      String transactionEnvelopeXdr) async {
    JsonRpcMethod getAccount = JsonRpcMethod("sendTransaction",
        args: {'transaction': transactionEnvelopeXdr});
    dio.Response response = await _dio.post(_serverUrl,
        data: json.encode(getAccount), options: dio.Options(headers: _headers));
    if (enableLogging) {
      log("sendTransaction response: $response");
    }
    return SendTransactionResponse.fromJson(response.data);
  }
}
