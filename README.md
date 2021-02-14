# yaww

Yet Another Winapi Wrapper, for thread-safe, async wingui programming. `yaww` distinguishes itself from other 
Winapi crates in three ways:

1). All primitives are thread safe (`Send + Sync`). 

2). Priority is given to async operations. Most operations return a `Task` object that can either be used as a 
    future or just used to block sync code.
    
3). The API is designed to suit GUI programming, as other facets of the Windows API are largely covered by either
    the standard library or other crates.

## Implementation

`yaww` accomplishes these things by spawning two threads: the GUI thread and the processing thread. All functions
in reality send "directives" down a channel to the processing thread. The processing thread converts the
directives into messages, which are pushed onto the Win32 message queue. Meanwhile, the GUI thread constantly
polls the message queue for new messages to process. All GUI primitives are created on the GUI thread, and
the objects pointing to them are actually keys that correspond to the primitives.

Note that, during event processing, the roles of the two threads are actually inverted. The processing thread
runs the closure that handles events, and the GUI thread handles directives while the event handler runs.
    
## License

MIT/Apache2 License
