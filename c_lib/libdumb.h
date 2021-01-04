typedef void (*rustcb)(int32_t);

void printsomething();
void printthis(const char *s);
void changefirstfive(int *ints);
int32_t register_cb(rustcb cb);
void call_cb();