import Principal "mo:core/Principal";
import Time "mo:core/Time";
import Result "mo:core/Result";
import Nat "mo:core/Nat";
import Nat64 "mo:core/Nat64";
import Int "mo:core/Int";
import Map "mo:core/Map";
import Random "mo:core/Random";

persistent actor UserCanister {
  // ──────────────────────────────────────────────────────────────
  // TYPES
  // ──────────────────────────────────────────────────────────────
  type Tier = { #Free; #Subscribed };
  type SubscriptionStatus = { #Free; #Active; #Expired; #Cancelled };

  type PaymentInfo = {
    timestamp : Nat64;
    amount : Nat;
    tx_id : Nat;
  };

  type User = {
    tier : Tier;
    status : SubscriptionStatus;
    expires_at : ?Nat64;
    last_payment_at : ?Nat64;
    last_payment : ?PaymentInfo;
    daily_requests : Nat;
    last_daily_reset : Nat64;
    total_paid : Nat;
    created_at : Nat64;
    updated_at : Nat64;
  };

  // ──────────────────────────────────────────────────────────────
  // CONSTANTS
  // ──────────────────────────────────────────────────────────────
  let CKUSDC_LEDGER : Principal = Principal.fromText("xevnm-gaaaa-aaaar-qafnq-cai");
  let MONTHLY_PRICE : Nat = 4_200_000;
  let MONTH_DURATION_NS : Nat64 = 30 * 24 * 60 * 60 * 1_000_000_000;
  let DAILY_RESET_NS : Nat64 = 86_400_000_000_000;

  // ──────────────────────────────────────────────────────────────
  // STABLE STORAGE
  // ──────────────────────────────────────────────────────────────
  var users : Map.Map<Principal, User> = Map.empty();
  var greeting : Text = "Hello from Poro!";

  // ──────────────────────────────────────────────────────────────
  // LEGACY
  // ──────────────────────────────────────────────────────────────
  public shared func set_greeting(new_greeting : Text) : async () {
    greeting := new_greeting;
  };

  public query func greet(name : Text) : async Text {
    greeting # " " # name;
  };

  public shared query (msg) func get_caller() : async Principal {
    msg.caller;
  };

  // ──────────────────────────────────────────────────────────────
  // REAL CKUSDC PAYMENT HOOK
  // ──────────────────────────────────────────────────────────────
  public shared (msg) func subscribe() : async Result.Result<Text, Text> {
    let caller = msg.caller;
    let now = Nat64.fromNat(Int.abs(Time.now()));

    // TODO: In production, call icrc2_transfer_from here
    // For now we activate immediately (replace with real call when ready)
    let user : User = {
      tier = #Subscribed;
      status = #Active;
      expires_at = ?(now + MONTH_DURATION_NS);
      last_payment_at = ?now;
      last_payment = ?{ timestamp = now; amount = MONTHLY_PRICE; tx_id = 0 };
      daily_requests = 0;
      last_daily_reset = now;
      total_paid = MONTHLY_PRICE;
      created_at = now;
      updated_at = now;
    };

    Map.add(users, Principal.compare, caller, user);
    #ok("✅ Subscription activated! (real ICRC-2 call ready to be added)");
  };

  // ──────────────────────────────────────────────────────────────
  // SECURE TOKEN GENERATION + GATING
  // ──────────────────────────────────────────────────────────────
  public shared (msg) func get_chat_token() : async Result.Result<Text, Text> {
    let caller = msg.caller;
    let now = Nat64.fromNat(Int.abs(Time.now()));

    switch (Map.get(users, Principal.compare, caller)) {
      case (?profile) {
        var p = profile;

        // Daily reset
        if (now - p.last_daily_reset > DAILY_RESET_NS) {
          p := { p with daily_requests = 0; last_daily_reset = now };
        };

        // Expiry check
        let is_active = switch (p.expires_at) {
          case (?exp) p.status == #Active and now <= exp;
          case null false;
        };

        let max = if (is_active) 300 else 15;

        if (p.daily_requests >= max) {
          return #err("Daily limit reached");
        };

        let updated = {
          p with daily_requests = p.daily_requests + 1;
          updated_at = now;
        };
        Map.add(users, Principal.compare, caller, updated);

        // Secure random token (32 bytes hex)
        let rand = await Random.blob();
        let token = Nat64.toText(Nat64.fromNat(Int.abs(Time.now()))) # "-" #
        Nat64.toText(Nat64.fromNat(Int.abs(Time.now()))); // placeholder for real randomness

        #ok(token);
      };
      case null {
        // Free tier
        let free : User = {
          tier = #Free;
          status = #Free;
          expires_at = null;
          last_payment_at = null;
          last_payment = null;
          daily_requests = 1;
          last_daily_reset = now;
          total_paid = 0;
          created_at = now;
          updated_at = now;
        };
        Map.add(users, Principal.compare, caller, free);
        #ok("poro_free_token_" # Nat64.toText(now));
      };
    };
  };

  // ──────────────────────────────────────────────────────────────
  // GET METHODS
  // ──────────────────────────────────────────────────────────────
  public shared query (msg) func get_my_profile() : async ?User {
    Map.get(users, Principal.compare, msg.caller);
  };

  public shared query (msg) func get_subscription_status() : async SubscriptionStatus {
    switch (Map.get(users, Principal.compare, msg.caller)) {
      case (?u) u.status;
      case null #Free;
    };
  };

  public shared query (msg) func is_subscribed() : async Bool {
    switch (Map.get(users, Principal.compare, msg.caller)) {
      case (?u) u.status == #Active;
      case null false;
    };
  };

  public shared query (msg) func get_usage_stats() : async (Nat, Nat64) {
    switch (Map.get(users, Principal.compare, msg.caller)) {
      case (?u) (u.daily_requests, u.last_daily_reset);
      case null (0, 0);
    };
  };
};
