# rust-screen-record

I have a couple ideas that require screen capturing functionality to work. First things first, I'm focusing on making a fast performing and low profile library to record the screen. I'm hoping to use DXGI to do it, in Rust. I was initially prototyping using `GetFrontBufferData` but this looks interesting, and much easier: http://www.virtualdub.org/blog/pivot/entry.php?id=356
