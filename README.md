# toy-crawler

## How to use

```sh
docker build . -t toy-crawler
docker run -it --workdir /mnt --volume "$(pwd)/out:/mnt" toy-crawler https://www.google.com https://blog.epigno.jp
```

Note:
- Updating the crates.io index and building may take some time, please be
  patient.
- We use a docker volume to access the HTML sources of the retrieved files in
  the `out/` directory.
