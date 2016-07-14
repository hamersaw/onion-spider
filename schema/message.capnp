@0xf25f36c02ae13d9d;

struct OnionSpiderMessage {
    messageType :union {
        crawlRequest @0 :List(Text);
        statsRequest @1 :Void;
    }
}
