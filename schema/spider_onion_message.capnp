@0xf25f36c02ae13d9d;

struct SpiderOnionMessage {
    messageType :union {
        crawlRequest @0 :List(Text);
        frontierSizeRequest @1 :Void;
    }
}
