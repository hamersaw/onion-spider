@0xf25f36c02ae13d9d;

struct PolzatMessage {
    messageType :union {
        crawlRequest @0 :List(Text);
        statsRequest @1 :Void;
        statsReply :group {
            frontierSize @2 :UInt64;
        }
    }
}
