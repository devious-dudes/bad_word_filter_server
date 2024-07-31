# bad_word_svr

Bad Word Server pulls words from a MongoDB connection the schema for these documents should be anything like:

``` json
{ word: 'badword' }
```

All words are loaded into memory into a trie structure for fast searching.  Two endpoints are exposed:

``` bash
# Requests that bad_word_svr reload the word list and it does so atomically
curl -X POST http://localhost:8080/reload
```

``` bash
# Validate a message to see if it contains any words or phrases that violate one's TOS
curl -X POST http://localhost:8080/check -H "Content-Type: application/json" -d '{"content":"This is a test message."}'
# => Will return "ok"

# Example bad content
curl -X POST http://localhost:8080/check -H "Content-Type: application/json" -d '{"content":"This is a test rape message."}'
# Will return "not ok"
```

This server only binds to localhost (127.0.0.1) on port 8080 and is not configurable.  An HTTP proxy via nginx or Apache can be configured to require HTTP basic auth and SSL/TLS.  This process is kept very simple.

