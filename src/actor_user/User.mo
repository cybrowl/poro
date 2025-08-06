persistent actor HelloWorld {
   var greeting : Text = "Hello, ";

  public func setGreeting(prefix : Text) : async () {
    greeting := prefix;
  };

  public query func greet(name : Text) : async Text {
    return greeting # name # "!";
  };

  public shared query (msg) func getCaller() : async Principal {
    msg.caller
  };
};
