# Blocking Threads in Tokio

If you spawn a long-running task on the default Tokio thread pool, you might starve other tasks.
