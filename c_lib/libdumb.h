typedef void (*rustcb)(int32_t);
typedef int32_t (*rustobjcb)(void *, int32_t);
typedef void (*rustnpocb)(void *, int32_t);

void printsomething();
void printthis(const char *s);
void changefirstfive(int *ints);
int32_t register_cb(rustcb cb);
void call_cb();
void register_objcb(void * objptr, rustobjcb cb);
void call_objcb();
int variadictest(int count, ...);
void register_npocb(rustnpocb cb);
void call_npocb();