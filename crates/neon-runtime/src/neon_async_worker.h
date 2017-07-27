#ifndef NEON_ASYNC_WORKER_H_
#define NEON_ASYNC_WORKER_H_

#include "neon.h"

namespace neon {

class AsyncWorkerHolder : public Nan::AsyncWorker {
public:
    explicit AsyncWorkerHolder(
        Nan::Callback *callback_, 
        Neon_AsyncExecuteCallback execCallback,
        void *execArg,
        Neon_AsyncResultCallback resultCallback,
        void *resultArg,
        Neon_DropCallback dropCallback) : 
        Nan::AsyncWorker(callback_), 
        execCallback(execCallback),
        execArg(execArg),
        resultCallback(resultCallback),
        resultArg(resultArg),
        dropCallback(dropCallback) { /* constructor */ }

    ~AsyncWorkerHolder() {
         /* destructor */ 
         this->dropCallback(this->state);
    }

    void SetState(void *state)
    {
        this->state = state;
    }

    void Execute()
    {
        // execution
        this->execCallback(this->execArg, &this->state);
    }

    void HandleOkCallback () {
        Nan::HandleScope scope;

        v8::Local<v8::Value> error;
        v8::Local<v8::Value> value;

        this->resultCallback(&error, &value, this->resultArg, this->state);

        v8::Local<v8::Value> argv[] = { error, value };

        callback->Call(2, argv);
    }
private:
    Neon_AsyncExecuteCallback execCallback;
    void *execArg;
    Neon_AsyncResultCallback resultCallback;
    void *resultArg;
    Neon_DropCallback dropCallback;
    void *state;
};

};

#endif