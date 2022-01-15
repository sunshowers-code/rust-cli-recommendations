# Write operations

In general, writing data to disk is inherently fraught: there are many unexpected situations that may arise, from rogue processes accessing user data as you, to disk corruption, to power outages. If your tool really must write data, there are many things you can do to make it "safer, but not safe".

## Atomic writes

If you open a file and write data to it, the default is to do something like:

TODO

Your application *should* write data to disk atomically.

TODO
