SVD file fetched from: https://packs.download.microchip.com/

To generate:
```sh
$ svd2rust -i ATSAML10E16A.svd
$ rm -rf src
$ form -i lib.rs -o src/ && rm lib.rs
$ cargo fmt
```
