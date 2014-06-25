Igneous
=======

Currently houses a crude attempt at an Azure Blob Storage client.


You'll need to get a copy of rust-http and rust-openssl into the third_party/ directory. This will do it:

```shell
make prepare
```


To run tests cleanly:

```shell
make clean
make deps
make runtest
```


To run the example app cleanly:

```shell
make clean
make deps
make run
```