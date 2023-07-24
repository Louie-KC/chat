import 'package:flutter/material.dart';

import 'package:chat_app/service/chat_api_service.dart';

class ApiTestView extends StatefulWidget {
  const ApiTestView({super.key});

  @override
  State<StatefulWidget> createState() => _ApiTestView();
}

class _ApiTestView extends State<ApiTestView> {
  String lastResponse = "";

  ChatApi api = ChatApi();

  final _usernameController = TextEditingController();
  final _passwordController = TextEditingController();

  void healthCheck() async {
    lastResponse = await api.healthCheck();
    debugPrint(lastResponse);
    setState(() {});
  }

  void createAccount(String username, String password) async {
    lastResponse = await api.createAccount(username, password);
    debugPrint(lastResponse);
    setState(() {});
  }

  void login(String username, String password) async {
    lastResponse = await api.login(username, password);
    debugPrint(lastResponse);
    setState(() {});
    debugPrint("API has token: ${api.hasToken()}");
  }

  TextField _textField(
    String label,
    bool obscure,
    TextEditingController controller,
  ) =>
      TextField(
        controller: controller,
        cursorColor: Colors.black,
        obscureText: obscure,
        autocorrect: false,
        enableSuggestions: false,
        decoration: InputDecoration(labelText: label),
      );

  ElevatedButton _button(
    String text,
    void Function() function,
  ) =>
      ElevatedButton(onPressed: function, child: Text(text));

  @override
  Scaffold build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text("Api Test Page"),
      ),
      body: Padding(
        padding: const EdgeInsets.all(8),
        child: Center(
          child: Column(
            mainAxisAlignment: MainAxisAlignment.start,
            crossAxisAlignment: CrossAxisAlignment.center,
            children: [
              Text("last response: $lastResponse"),
              ElevatedButton(
                onPressed: healthCheck,
                child: const Text("health check"),
              ),
              _textField("username", false, _usernameController),
              _textField("password", true, _passwordController),
              _button(
                "create account",
                () => createAccount(
                  _usernameController.text,
                  _passwordController.text,
                ),
              ),
              _button(
                "login",
                () => login(
                  _usernameController.text,
                  _passwordController.text,
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}
