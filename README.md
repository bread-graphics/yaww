# yaww

Yet Another Winapi Wrapper, for thread-safe, async wingui programming. `yaww` distinguishes itself from other Winapi crates
in three ways:

1). All primitives are thread safe (`Send + Sync`). 

2). Priority is given to async operations. Most operations return a `Task` object that can either be used as a future or just
    used to block sync code.
    
3). The API is designed to suit GUI programming, as other facets of the Windows API are largely covered by either the
    standard library or other crates.
    
## License

MIT/Apache2 License
