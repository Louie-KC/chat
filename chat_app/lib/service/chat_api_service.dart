import 'dart:convert';

import 'package:http/http.dart' as http;
import 'package:flutter/material.dart';

class ChatApi {
  static final ChatApi _instance = ChatApi._();
  static const String _baseUrl = "http://192.168.1.8:8080/api";
  static final _jsonHeader = {"Content-Type": "application/json"};
  String _token = "";

  ChatApi._();
  factory ChatApi() => _instance;

  bool hasToken() => _token.isNotEmpty;

  void clearToken() => _token = "";

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
      _token = jsonDecode(response.body)["token"];
      debugPrint("token: $_token");
      return "Success";
    } else {
      return "Incorrect login credentials";
    }
  }

  // TODO: post /message
  // TODO: get /message
  // TODO: get /conversation
}
