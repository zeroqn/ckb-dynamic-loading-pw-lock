# ckb-dynamic-loading-pw-lock

Build contracts:

``` sh
git submodule update --init --recursive
cd pw-lock && make all-via-docker
cd .. && capsule build
```

Run tests:

``` sh
capsule test
```
