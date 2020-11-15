rm -rf `find . -type f | perl -lne 'print if -B'`
