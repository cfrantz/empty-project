#ifndef EMPTY_PROJECT_RULES_PYTHON_SUPPORT_H
#define EMPTY_PROJECT_RULES_PYTHON_SUPPORT_H
#include "pybind11/cast.h"
#include "google/protobuf/message.h"
#include "python/google/protobuf/proto_api.h"
#include "absl/log/log.h"

namespace internal {
extern const google::protobuf::python::PyProto_API *api;
void init();
void remember_descriptor(google::protobuf::Message* m);
}


template<typename T>
class PyProto {
  public:
    PyProto(): message_(nullptr) {}
    PyProto(T *m): message_(m) {}

    bool validate() {
        return message_->GetTypeName() == T::descriptor()->full_name();
    }
    T* get() {
        if (validate()) {
            return reinterpret_cast<T*>(message_);
        } else {
            LOG(ERROR) << "Expected type " << T::descriptor()->full_name() << ", but got " << message_->GetTypeName();
            return nullptr;
        }
    }
    T* operator->() {
        return get();
    }

    google::protobuf::Message *message_;
  private:
    friend pybind11::detail::type_caster<PyProto<T>>;
};

namespace PYBIND11_NAMESPACE {
namespace detail {

template <typename T> struct type_caster<PyProto<T>> {
  public:
    PYBIND11_TYPE_CASTER(PyProto<T>, const_name("PyProto"));

    bool load(handle src, bool) {
        PyObject *source = src.ptr();
        internal::init();
        value.message_ = internal::api->GetMutableMessagePointer(source);

        LOG(INFO) << "Load name" << value.message_->GetTypeName();
        return true;
    }

    static handle cast(PyProto<T> src, return_value_policy, handle) {
        internal::init();
        internal::remember_descriptor(src.message_);
        LOG(INFO) << "Cast name" << src.message_->GetTypeName();
        return internal::api->NewMessageOwnedExternally(src.message_, nullptr);
    }
};

}  // namespace detail
}  // namespace

#endif // EMPTY_PROJECT_RULES_PYTHON_SUPPORT_H
