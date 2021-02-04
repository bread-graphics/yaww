# yaww

`yaww` is short for "Yet Another Winapi Wrapper". However, `yaww` differentiates itself from the competition in
three major ways:

1. All of `yaww`'s major types are thread safe; they are both `Send` and `Sync`.
2. `yaww` provides async capabilities for its users.
3. `yaww` specializes itself towards GUI programming, as most other facets of the Windows API are covered by
    the standard library or other libraries.

## Implementation

`yaww` provides an object called the `GuiThread` that serves as the centerpiece of the program. As the name
implies, creating the `GuiThread` object spawns two threads: one that listens for Win32 messages, and one that
listens for user requests (although it just pushes these user requests onto the Win32 message queue, so the
former is the thread that really does all the work). All actual GUI function calls are done on the message
queue thread. This has two primary advantages:

1. Since the primitives are isolated to one thread, and since they never leave that thread (only keys that 
   uniquely identify that primitives do), the whole construct is thread safe.
2. Since all of the blocking logic happens on that thread, async code can just wait until it gets back a
   response from that thread, pending until then.
