# Rrep - A Rust implementation of the Unix grep command

Part of my Rust learning attempts, this command uses buffers instead of direct memory where possible, hopefully resulting in better performance on large files.

Use in the same way as the native Unix `grep` command to search the content of a file and output to the standard output stream.

`rrep <search pattern> ./file/path/file.txt`
