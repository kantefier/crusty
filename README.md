# Crusty
*Unofficial* [WavesEnterprise](https://wavesenterprise.com) blockchain crawler. The name is quite unoriginal, I know.

## Goal
A replacement for [my Python crawling script](https://github.com/kantefier/QuickPayout) I've been using for quite a while.
More recent versions of WavesEnterprise blockchain node support public gRPC API with a method that streams [Blockchain Events](https://docs.wavesenterprise.com/en/latest/usage/grpc/grpc-events.html).
That gives the opportunities to implement services that would subscribe to these events and perform their tasks in event-driven fashion.

And well, it's just fun to write some useful stuff using some tech I don't really know yet. What a great challenge to learn, isn't it?

### TODOs:
- [x] "Hello world!" compiles and runs (yay!)
- [x] Reads the first command line argument as a path to the config file
- [x] Has a defined Config structure
- [x] Parses TOML config file to a structure
- [x] Enable ENV overriding some config settings (would be handy for passwords)
- [x] Build Rust sources from WE proto-files
- [ ] Connect to MongoDB
- [ ] Establishes a gRPC connection to the node
- [ ] Subscribe to Blockchain Events stream
- [ ] Decide on MongoDB structure
- [ ] ...
- [ ] Pack it all up in a docker image
- [ ] ...
- [ ] Support REST API
- [ ] ...
- [ ] Implement an algorithm to calculate leaser payouts
- [ ] ...
- [ ] Automate payouts
- [ ] ...
- [ ] API for fancy statistics and other cool stuff
- [ ] ...
- [ ] Who knows where the road will lead us? Further researches, etc.