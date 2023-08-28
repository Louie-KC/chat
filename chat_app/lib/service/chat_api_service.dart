import 'dart:convert';

import 'package:http/http.dart' as http;
import 'package:flutter/material.dart';

import '../models/message.dart';
import '../models/chat.dart';

class ChatApi {
  static final ChatApi _instance = ChatApi._();
  static const String _baseUrl = "http://192.168.1.8:8080/api";
  static final _jsonHeader = {"Content-Type": "application/json"};
  static final _tokenJsonHeader = {
    "Authorization": "",
    "Content-Type": "application/json"
  };
  // String _token = "";

  ChatApi._();
  factory ChatApi() => _instance;

  // bool hasToken() => _token.isNotEmpty;

  // void clearToken() => _token = "";

  Future<String> healthCheck() async {
    debugPrint("Health check called");

    final Uri url = Uri.parse("$_baseUrl/health");
    final response = await http.get(url);

    debugPrint("Health check response code: ${response.statusCode}");
    return jsonDecode(response.body)["status"];
  }

  Future<String> createAccount(String username, String password) async {
    debugPrint("Create Account called");

    final Uri url = Uri.parse("$_baseUrl/register");
    Map<String, String> body = {"username": username, "password": password};
    final response =
        await http.post(url, headers: _jsonHeader, body: jsonEncode(body));

    debugPrint(
        "Create account response status: ${response.statusCode}, body: ${response.body}");

    if (response.statusCode == 201) {
      debugPrint("Account creation successful");
      return jsonDecode(response.body)["status"];
    } else {
      debugPrint("Account creation failed");
      return jsonDecode(response.body)["reason"];
    }
  }

  Future<String> login(String username, String password) async {
    debugPrint("login called");

    final Uri url = Uri.parse("$_baseUrl/authenticate");
    Map<String, String> body = {"username": username, "password": password};
    final response =
        await http.post(url, headers: _jsonHeader, body: jsonEncode(body));

    if (response.statusCode == 200) {
      String token = jsonDecode(response.body)["token"];
      debugPrint("token: $token");
      return token;
    } else {
      return "";
    }
  }

  Future<int> sendMessage(
    String chatID,
    String messageContent,
    String token,
  ) async {
    debugPrint("sendMessage called");

    final Uri url = Uri.parse("$_baseUrl/message/$chatID");
    _tokenJsonHeader["Authorization"] = "Bearer $token";
    Map<String, String> body = {"content": messageContent};
    final response =
        await http.post(url, headers: _tokenJsonHeader, body: jsonEncode(body));

    if (response.statusCode == 200) {
      debugPrint("sendMessage success");
    }
    return response.statusCode;
  }

  Future<List<Message>> getChatMessages(
    String chatID,
    DateTime since,
    String token,
  ) async {
    debugPrint("getMessages called");

    final Uri url = Uri.parse("$_baseUrl/message/$chatID");
    _tokenJsonHeader["Authorization"] = "Bearer $token";
    Map<String, String> body = {
      "from_time": "${since.toIso8601String().split(".")[0]}Z"
    };
    var request = http.Request("GET", url);
    request.headers.addAll(_tokenJsonHeader);
    request.body = jsonEncode(body);
    final response = await request.send();

    if (response.statusCode == 200) {
      debugPrint("getChatMessages Success");
      Iterable messages = json.decode(await response.stream.bytesToString());
      List<Message> result =
          List.from(messages.map((msg) => Message.fromJson(msg)));
      for (var msg in result) {
        debugPrint("$msg");
      }
      return result;
    } else {
      debugPrint("getChatMessages failed");
      debugPrint("${response.statusCode}");
      return List.empty(growable: false);
    }
  }

  // TODO: get /message
  Future<List<Chat>> getChatOverview(DateTime since, String token) async {
    debugPrint("getChatOverview called");

    final Uri url = Uri.parse("$_baseUrl/message");
    _tokenJsonHeader["Authorization"] = "Bearer $token";
    Map<String, String> body = {
      "from_time": "${since.toIso8601String().split(".")[0]}Z"
    };
    var request = http.Request("GET", url);
    request.headers.addAll(_tokenJsonHeader);
    request.body = jsonEncode(body);
    final response = await request.send();

    if (response.statusCode == 200) {
      // We will receive messages newer than the `from` time,
      // and only the top message from each Chat. Blindly make each
      // chat from each message
      Iterable messages = json.decode(await response.stream.bytesToString());
      return List.from(
        messages.map((msg) => Chat.previewOne(msg)),
        growable: false,
      );
    } else {
      return List.empty(growable: false);
    }
  }
}
