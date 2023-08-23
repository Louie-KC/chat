import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'package:chat_app/controllers/chat_controller.dart';

// import 'package:chat_app/service/chat_api_service.dart';
// import 'package:chat_app/controllers/login_controller.dart';

class LoginPage extends StatelessWidget {
  late final TextEditingController _unameController;
  late final TextEditingController _pwordController;

  LoginPage({super.key}) {
    _unameController = TextEditingController();
    _pwordController = TextEditingController();
  }

  TextField _textField(
      String label, bool obscure, TextEditingController controller) {
    OutlineInputBorder outlineBorder = OutlineInputBorder(
      borderRadius: BorderRadius.circular(16),
      borderSide: const BorderSide(color: Colors.black),
    );

    return TextField(
      controller: controller,
      cursorColor: Colors.black,
      obscureText: obscure,
      autocorrect: false,
      enableSuggestions: false,
      decoration: InputDecoration(
        labelText: label,
        labelStyle: const TextStyle(color: Colors.black),
        border: outlineBorder,
        focusedBorder: outlineBorder,
      ),
    );
  }

  ElevatedButton _button(String text, void Function() function) =>
      ElevatedButton(onPressed: function, child: Text(text));

  @override
  Widget build(BuildContext context) {
    final ChatController controller = Provider.of<ChatController>(context);

    return Scaffold(
      appBar: AppBar(
        elevation: 0,
        backgroundColor: Colors.white,
      ),
      body: Column(
        mainAxisAlignment: MainAxisAlignment.start,
        crossAxisAlignment: CrossAxisAlignment.center,
        children: [
          Container(
            // placeholder circle
            width: 175,
            height: 175,
            color: Colors.white,
            foregroundDecoration: const BoxDecoration(
              color: Colors.grey,
              shape: BoxShape.circle,
            ),
          ),
          const Text("Enter login details"),
          Padding(
            padding: const EdgeInsets.all(8.0),
            child: _textField("username", false, _unameController),
          ),
          Padding(
            padding: const EdgeInsets.all(8.0),
            child: _textField("password", true, _pwordController),
          ),
          _button("login", () async {
            debugPrint("login button pushed");
            await controller.login(
                _unameController.text, _pwordController.text);
          }),
        ],
      ),
    );
  }
}
