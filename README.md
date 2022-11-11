# Overview

I wanted to try my hand at LAN peer-to-peer networking, so I made this fork of JackMcGinty's **Corrosion Dice** to give it the ability to connect to other Corrosion Dice clients via their IP addresses. By default, these connections are made to port 4000.

When run, the user will first be asked if they want to connect to another remote client. The user can respond by inputing the IP address of their choice, and an attempt to connect will be made. The user can also simply press enter or anything other than an IP address to remove networking. **Corrosion Dice** then supplies the user with a minimal command-line-type interface that they can use to roll dice for the role-playing game of their choice, using commands like "roll 20" or "roll 3". Although minimal, this is only the beginning, and I will definitely be coming back to add more functionality.

[Demo Video](https://youtu.be/oGOPDVKSByw)

# Development Environment

* Pop!_OS 22.04
* VS-Code V.1.73.1
* Rust V.1.63.0

# Useful Websites

* ["The book" (official rust guide)](https://doc.rust-lang.org/book)
* [Rust's TcpStream documentation](https://doc.rust-lang.org/std/net/struct.TcpStream.html)
* [Rust's Ipv4Addr documentation](https://doc.rust-lang.org/std/net/struct.Ipv4Addr.html)
* [thepacketgeek's codec tutorial.](https://github.com/thepacketgeek/rust-tcpstream-demo/tree/master/lines)

# Future Work

* Make server recieving initial connection make connections back to form a two-way peer-to-peer connection.
* Support for multiple peers.
* Automate LAN input (search LAN for corrosion clients).
