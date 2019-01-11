# Front

Flubber is a general specification for a messaging client. The idea is to unify
multiple chat services into a single client framework, while keeping it abstract
enough to allow the same service plugins to work for multiple clients.

## Architecture

Flubber uses the following architecture:

```
+-----------------+            +-----------------+
| chat service #1 |            | chat service #2 |
+-----+-----------+            +-----------+-----+
      |                                    |
+-----+-----+   +----------------+   +-----+-----+
| plugin #1 +---+ flubber daemon +---+ plugin #2 |
+-----------+   +--------+-------+   +-----------+
                         |
                +--------+-------+
                | flubber client |
                +----------------+
```

The various plugins will update the daemon about the state of messages to and
from that service, which it then relays to and from the client(or clients).

- The protocol used for communication between the flubber daemon and the flubber
  client can be found in the section [Client Protocol][1].
- The protocol used for communication between the flubber daemon and the
  various clients can found in the section [Plugin Protocol][2].

### View

In order to do this, we have a unified view of messages and channels as follows:

- **Division:** a collection of either buffers or subdivisions. This is used for
  organizing buffers in the buffer list.
- **Buffer:** a constantly updating list of messages.

[1]: /protocol.html#client-protocol
[2]: /protocol.html#plugin-protocol
