#embedded-mp3
This is a Rust wrapper over the minimp3 C library. The main difference to other wrappers is that it allows streaming. The output of the decode function is placed in the mut buffer ``pcm``. 

Because DecoderData (mp3dec_init) a signifficant size has, this is required as input to Decoder::new, therefore allowing it's placement in a specific memory location.
