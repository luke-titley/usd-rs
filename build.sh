docker run -v $PWD:$PWD --workdir $PWD luketitley/vfxrs_env_usd env USD_ROOT=/usr/local/USD LD_LIBRARY_PATH=/usr/local/USD/lib  cargo test
